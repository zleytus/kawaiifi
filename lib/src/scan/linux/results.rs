use std::{collections::HashSet, fmt::Debug, ops::Deref};

use chrono::{DateTime, Utc};
use deku::DekuContainerRead;
use neli::{attr::Attribute, genl::Genlmsghdr};
use serde::{Deserialize, Serialize};

use super::Flags;
use crate::{
    Bss,
    ies::{self, Ie},
    nl80211::{Attr, Cmd, ParseError},
};

/// Results of a WiFi scan operation.
///
/// A `Scan` contains all Basic Service Sets (BSSs) discovered during a scan, along with
/// metadata about how the scan was performed. The scan may have covered multiple frequency
/// bands and may represent multiple underlying sub-scans aggregated into a single result.
///
/// # Multiple Sub-Scans
///
/// Some scan backends (particularly NetworkManager) may split a single scan request into
/// multiple sub-scans, each covering different frequency bands (e.g., 2.4 GHz, 5 GHz, 6 GHz).
/// This type automatically aggregates all sub-scans into a unified result, combining:
/// - All discovered BSSs from each sub-scan
/// - All frequencies scanned across all sub-scans
/// - All IEs used across all sub-scans (deduplicated)
/// - Start time from the first sub-scan
/// - End time from the last sub-scan
///
/// # Example
///
/// ```no_run
/// use kawaiifi::scan::Backend;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let interface = kawaiifi::default_interface()
///     .expect("Expected to find a wireless interface");
/// let scan = interface.scan(Backend::NetworkManager).await?;
///
/// println!("Found {} BSSs", scan.bss_list().len());
/// println!("Scanned {} frequencies", scan.freqs_mhz().len());
/// println!("Scan took {:?}", scan.duration());
///
/// for bss in scan.bss_list() {
///     println!("{}: {} dBm", bss.ssid().unwrap_or(""), bss.signal_dbm());
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Scan {
    bss_list: Vec<Bss>,
    wiphy: u32,
    ifindex: u32,
    freqs_mhz: Vec<u32>,
    ies: Vec<Ie>,
    flags: Option<Flags>,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
}

impl Scan {
    pub(super) fn new(scans: Vec<ScanInternal>) -> Self {
        Self {
            bss_list: scans
                .iter()
                .flat_map(|scan| scan.bss_list.iter().cloned())
                .collect(),
            wiphy: scans.first().map(|scan| scan.wiphy()).unwrap_or_default(),
            ifindex: scans.first().map(|scan| scan.ifindex()).unwrap_or_default(),
            freqs_mhz: scans
                .iter()
                .flat_map(|scan| scan.freqs_mhz().iter().cloned())
                .collect(),
            ies: HashSet::<Ie>::from_iter(scans.iter().flat_map(|scan| scan.ies().iter().cloned()))
                .into_iter()
                .collect(),
            flags: scans.first().and_then(|scan| scan.flags()),
            start_time: scans
                .first()
                .map(|first_scan| first_scan.start_time())
                .unwrap_or_default(),
            end_time: scans
                .last()
                .map(|last_scan| last_scan.end_time())
                .unwrap_or_default(),
        }
    }

    /// Returns all BSSs (access points) discovered during the scan.
    ///
    /// If multiple sub-scans were performed, this list contains the combined results
    /// from all sub-scans.
    pub fn bss_list(&self) -> &[Bss] {
        &self.bss_list
    }

    /// The physical wireless device used to perform the scan.
    pub fn wiphy(&self) -> u32 {
        self.wiphy
    }

    /// The index of the wireless interface used to perform the scan.
    pub fn ifindex(&self) -> u32 {
        self.ifindex
    }

    /// The frequencies that were scanned for BSSs.
    pub fn freqs_mhz(&self) -> &[u32] {
        &self.freqs_mhz
    }

    /// The information elements that were sent with each probe request.
    pub fn ies(&self) -> &[Ie] {
        &self.ies
    }

    /// The flags/settings used to control the scan.
    pub fn flags(&self) -> Option<Flags> {
        self.flags
    }

    /// The time at which the scan started.
    pub fn start_time(&self) -> DateTime<Utc> {
        self.start_time
    }

    /// The time at which the scan ended.
    pub fn end_time(&self) -> DateTime<Utc> {
        self.end_time
    }

    /// The duration of the scan.
    pub fn duration(&self) -> std::time::Duration {
        (self.end_time() - self.start_time())
            .to_std()
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct ScanInternal {
    pub(super) bss_list: Vec<Bss>,
    pub(super) scan_triggered: ScanTriggered,
    pub(super) scan_completed: ScanCompleted,
}

impl ScanInternal {
    /// The physical wireless device used to perform the scan
    fn wiphy(&self) -> u32 {
        self.scan_completed.wiphy
    }

    /// The index of the wireless interface used to perform the scan
    fn ifindex(&self) -> u32 {
        self.scan_completed.ifindex
    }

    /// The frequencies at which probe requests were transmitted
    fn freqs_mhz(&self) -> &[u32] {
        &self.scan_completed.freqs_mhz
    }

    /// The information elements that were sent with each probe request
    fn ies(&self) -> &[Ie] {
        &self.scan_completed.ies
    }

    /// The flags/settings used to control the scan
    fn flags(&self) -> Option<Flags> {
        self.scan_completed.flags
    }

    /// The time at which the scan started
    fn start_time(&self) -> DateTime<Utc> {
        self.scan_triggered.timestamp
    }

    /// The time at which the scan ended
    fn end_time(&self) -> DateTime<Utc> {
        self.scan_completed.timestamp
    }
}

/// Message received on the SCAN multicast group when a scan is triggered
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct ScanTriggered {
    params: ScanParams,
    timestamp: DateTime<Utc>,
}

impl TryFrom<&Genlmsghdr<Cmd, Attr>> for ScanTriggered {
    type Error = ParseError;

    fn try_from(msghdr: &Genlmsghdr<Cmd, Attr>) -> Result<Self, Self::Error> {
        if *msghdr.cmd() != Cmd::TriggerScan {
            return Err(ParseError::UnexpectedCommand {
                expected: Cmd::TriggerScan,
                got: *msghdr.cmd(),
            });
        }

        let params = ScanParams::try_from(msghdr)?;

        Ok(Self {
            params,
            timestamp: Utc::now(),
        })
    }
}

impl Deref for ScanTriggered {
    type Target = ScanParams;

    fn deref(&self) -> &Self::Target {
        &self.params
    }
}

/// Message received on the SCAN multicast group when a scan is completed
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct ScanCompleted {
    params: ScanParams,
    timestamp: DateTime<Utc>,
}

impl TryFrom<&Genlmsghdr<Cmd, Attr>> for ScanCompleted {
    type Error = ParseError;

    fn try_from(msghdr: &Genlmsghdr<Cmd, Attr>) -> Result<Self, Self::Error> {
        if *msghdr.cmd() != Cmd::NewScanResults {
            return Err(ParseError::UnexpectedCommand {
                expected: Cmd::NewScanResults,
                got: *msghdr.cmd(),
            });
        }

        let params = ScanParams::try_from(msghdr)?;

        Ok(Self {
            params,
            timestamp: Utc::now(),
        })
    }
}

impl Deref for ScanCompleted {
    type Target = ScanParams;

    fn deref(&self) -> &Self::Target {
        &self.params
    }
}

/// The parameters/fields included in both ScanTriggered and ScanCompleted messages
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct ScanParams {
    pub(super) wiphy: u32,
    pub(super) ifindex: u32,
    pub(super) freqs_mhz: Vec<u32>,
    pub(super) ies: Vec<Ie>,
    pub(super) flags: Option<Flags>,
}

impl TryFrom<&Genlmsghdr<Cmd, Attr>> for ScanParams {
    type Error = ParseError;

    fn try_from(msghdr: &Genlmsghdr<Cmd, Attr>) -> Result<Self, Self::Error> {
        let attr_handle = msghdr.attrs().get_attr_handle();
        let wiphy = attr_handle.get_attr_payload_as::<u32>(Attr::Wiphy)?;
        let ifindex = attr_handle.get_attr_payload_as::<u32>(Attr::Ifindex)?;
        let scan_freqs_handle = attr_handle.get_nested_attributes::<u16>(Attr::ScanFrequencies)?;
        let freqs_mhz = scan_freqs_handle
            .iter()
            .map(|attr| attr.get_payload_as::<u32>().unwrap_or_default())
            .collect();
        let ies = attr_handle
            .get_attribute(Attr::Ie)
            .map(|attr| ies::from_bytes(attr.payload().as_ref()))
            .unwrap_or_default();
        let flags = {
            if let Ok(flags_payload) = attr_handle.get_attr_payload_as::<u32>(Attr::ScanFlags) {
                let flags_bytes = flags_payload.to_le_bytes();
                if let Ok((_, flags)) = Flags::from_bytes((&flags_bytes, 0)) {
                    Some(flags)
                } else {
                    None
                }
            } else {
                None
            }
        };

        Ok(Self {
            wiphy,
            ifindex,
            freqs_mhz,
            ies,
            flags,
        })
    }
}
