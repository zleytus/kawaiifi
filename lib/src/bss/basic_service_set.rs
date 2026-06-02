#[cfg(any(target_os = "linux", target_os = "windows"))]
use std::time::Duration;
use std::{fmt::Display, hash::Hash};

#[cfg(any(target_os = "linux", target_os = "windows"))]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(any(target_os = "linux", target_os = "windows"))]
use crate::CapabilityInfo;
#[cfg(target_os = "linux")]
use crate::nl80211::{BssScanWidth, BssStatus};
use crate::{
    Band, ChannelWidth, SecurityProtocols, WifiAmendments, WifiProtocols,
    ies::{Ie, IeData},
};

/// A basic service set (BSS).
#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
pub struct Bss {
    // Cross-platform
    pub(super) bssid: [u8; 6],
    pub(super) frequency_mhz: u32,
    pub(super) signal_dbm: i32,
    pub(super) beacon_interval_tu: u16,
    #[serde(with = "crate::ies::serde_raw::ies_as_base64")]
    pub(super) ies: Vec<Ie>,

    // Linux and Windows
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    pub(super) tsf: u64,
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    #[serde(with = "crate::ies::serde_raw::capability_info_as_u16")]
    pub(super) capability_info: CapabilityInfo,
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    pub(super) last_seen_utc: Option<DateTime<Utc>>,

    // Linux-only
    #[cfg(target_os = "linux")]
    pub(super) status: Option<BssStatus>,
    #[cfg(target_os = "linux")]
    pub(super) is_from_probe_response: bool,
    #[cfg(target_os = "linux")]
    pub(super) parent_bssid: Option<[u8; 6]>,
    #[cfg(target_os = "linux")]
    pub(super) parent_tsf: Option<u64>,
    #[cfg(target_os = "linux")]
    pub(super) beacon_tsf: Option<u64>,
    #[cfg(target_os = "linux")]
    pub(super) frequency_offset_khz: Option<u32>,
    #[cfg(target_os = "linux")]
    pub(super) signal_percent: Option<u8>,
    #[cfg(target_os = "linux")]
    #[serde(with = "crate::ies::serde_raw::option_ies_as_base64")]
    pub(super) beacon_ies: Option<Vec<Ie>>,
    #[cfg(target_os = "linux")]
    pub(super) scan_width: Option<BssScanWidth>,
    #[cfg(target_os = "linux")]
    pub(super) last_seen_boottime: Option<u64>,
    #[cfg(target_os = "linux")]
    pub(super) seen_ms_ago: Option<u32>,
    #[cfg(target_os = "linux")]
    pub(super) mlo_link_id: Option<u8>,
    #[cfg(target_os = "linux")]
    pub(super) mld_address: Option<[u8; 6]>,

    // Windows-only
    #[cfg(target_os = "windows")]
    pub(super) link_quality: u8,

    // macOS-only
    #[cfg(target_os = "macos")]
    pub(super) noise_dbm: i32,
}

impl Bss {
    /// The 6-byte BSSID (MAC address) of the BSS.
    pub fn bssid(&self) -> &[u8; 6] {
        &self.bssid
    }

    /// The operating frequency in MHz.
    pub fn frequency_mhz(&self) -> u32 {
        self.frequency_mhz
    }

    /// The frequency band the BSS operates on.
    pub fn band(&self) -> Band {
        Band::from_freq_mhz(self.frequency_mhz)
    }

    /// The channel width used by the BSS.
    pub fn channel_width(&self) -> ChannelWidth {
        ChannelWidth::from(self.ies())
    }

    /// The center frequency of the full channel in MHz.
    ///
    /// For 20 MHz channels this equals `frequency_mhz`. For wider channels,
    /// the primary channel frequency reported by nl80211 is offset from the
    /// true center, so this reads the center channel frequency segments from
    /// the operation IEs (EHT > HE > VHT > HT priority).
    pub fn center_frequency_mhz(&self) -> u32 {
        use crate::ies::ht_operation::SecondaryChannelOffset;

        let primary = self.frequency_mhz;
        let band = self.band();

        let chan_to_freq = |chan: u8| -> u32 {
            match band {
                Band::FiveGhz => 5000 + (chan as u32 * 5),
                Band::SixGhz => 5950 + (chan as u32 * 5),
                Band::TwoPointFourGhz => primary,
            }
        };

        let (mut eht_op, mut he_op, mut vht_op, mut ht_op) = (None, None, None, None);
        for ie in &self.ies {
            match &ie.data {
                IeData::EhtOperation(op) => eht_op = Some(op),
                IeData::HeOperation(op) => he_op = Some(op),
                IeData::VhtOperation(op) => vht_op = Some(op),
                IeData::HtOperation(op) => ht_op = Some(op),
                _ => {}
            }
        }

        // EHT Operation:
        //   CCFS1 — center of the full 160 or 320 MHz channel (when present)
        //   CCFS0 — center for 20/40/80 MHz, or primary 80 MHz for 160 MHz,
        //           or primary 160 MHz for 320 MHz
        if let Some(eht_op) = eht_op
            && let Some(info) = eht_op.eht_operation_information
        {
            if info.ccfs1 != 0 {
                return chan_to_freq(info.ccfs1);
            }
            if info.ccfs0 != 0 {
                return chan_to_freq(info.ccfs0);
            }
        }

        // HE Operation: 6 GHz has its own info; 5 GHz embeds VHT operation info.
        // CCFS1 (when 8 channels/40 MHz from CCFS0) is the 160 MHz full center.
        if let Some(he_op) = he_op {
            if let Some(six_ghz) = &he_op.six_ghz_operation_information {
                let (ccfs0, ccfs1) = (
                    six_ghz.channel_center_frequency_segment_0,
                    six_ghz.channel_center_frequency_segment_1,
                );
                if ccfs1 != 0 && ccfs1.abs_diff(ccfs0) == 8 {
                    return chan_to_freq(ccfs1);
                }
                if ccfs0 != 0 {
                    return chan_to_freq(ccfs0);
                }
            }
            if let Some(vht_info) = &he_op.vht_operation_information {
                let (ccfs0, ccfs1) = (
                    vht_info.channel_center_frequency_segment_0,
                    vht_info.channel_center_frequency_segment_1,
                );
                if ccfs1 != 0 && ccfs1.abs_diff(ccfs0) == 8 {
                    return chan_to_freq(ccfs1);
                }
                if ccfs0 != 0 {
                    return chan_to_freq(ccfs0);
                }
            }
        }

        // VHT Operation (5 GHz): CCFS1 for 160 MHz full center, CCFS0 for 80 MHz
        if let Some(vht_op) = vht_op {
            let info = &vht_op.vht_operation_information;
            let (ccfs0, ccfs1) = (
                info.channel_center_frequency_segment_0,
                info.channel_center_frequency_segment_1,
            );
            if ccfs1 != 0 && ccfs1.abs_diff(ccfs0) == 8 {
                return chan_to_freq(ccfs1);
            }
            if ccfs0 != 0 {
                return chan_to_freq(ccfs0);
            }
        }

        // HT Operation: 40 MHz secondary channel is above or below the primary
        if let Some(ht_op) = ht_op {
            match ht_op.ht_operation_information.secondary_channel_offset {
                SecondaryChannelOffset::AbovePrimary => return primary + 10,
                SecondaryChannelOffset::BelowPrimary => return primary - 10,
                SecondaryChannelOffset::NoSecondary => {}
            }
        }

        primary
    }

    /// The 802.11 channel number.
    pub fn channel_number(&self) -> u8 {
        self.ies
            .iter()
            .find_map(|ie| match &ie.data {
                IeData::DsParameterSet(ds_parameter_set) => Some(ds_parameter_set.current_channel),
                IeData::HtOperation(ht_operation) => Some(ht_operation.primary_channel),
                _ => None,
            })
            .unwrap_or_else(|| {
                let freq = self.frequency_mhz();
                if Band::TwoPointFourGhz.range_mhz().contains(&freq) {
                    if freq == 2484 {
                        14
                    } else {
                        ((freq - 2407) / 5) as u8
                    }
                } else if Band::FiveGhz.range_mhz().contains(&freq) {
                    ((freq - 5000) / 5) as u8
                } else if Band::SixGhz.range_mhz().contains(&freq) {
                    ((freq - 5950) / 5) as u8
                } else {
                    0
                }
            })
    }

    /// The received signal strength in dBm.
    pub fn signal_dbm(&self) -> i32 {
        self.signal_dbm
    }

    /// The beacon interval in time units (1 TU = 1024 µs).
    pub fn beacon_interval_tu(&self) -> u16 {
        self.beacon_interval_tu
    }

    /// The beacon interval in milliseconds.
    pub fn beacon_interval_ms(&self) -> f64 {
        f64::from(self.beacon_interval_tu) * 1.024
    }

    /// The 802.11 capability information flags advertised by the BSS.
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    pub fn capability_info(&self) -> &CapabilityInfo {
        &self.capability_info
    }

    /// The information elements (IEs) included in the BSS's beacon or probe response.
    pub fn ies(&self) -> &[Ie] {
        &self.ies
    }

    /// The timing synchronization function (TSF) timer value.
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    pub fn tsf(&self) -> u64 {
        self.tsf
    }

    /// The estimated time the BSS has been running, derived from its TSF timer.
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    pub fn uptime(&self) -> Duration {
        Duration::from_micros(self.tsf)
    }

    /// The UTC date and time when the BSS was last seen, or `None` if unavailable.
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    pub fn last_seen_utc(&self) -> Option<DateTime<Utc>> {
        self.last_seen_utc
    }

    /// The SSID (network name), or `None` for hidden networks.
    pub fn ssid(&self) -> Option<&str> {
        self.ies.iter().find_map(|ie| {
            if let IeData::Ssid(ssid) = &ie.data
                && !ssid.is_empty()
            {
                ssid.as_str().ok()
            } else {
                None
            }
        })
    }

    /// The security protocols supported by the BSS.
    pub fn security_protocols(&self) -> SecurityProtocols {
        #[cfg(any(target_os = "linux", target_os = "windows"))]
        {
            let mut protocols = SecurityProtocols::from(self.ies.as_slice());
            if self.capability_info.privacy && protocols.is_empty() {
                protocols |= SecurityProtocols::from(enumflags2::BitFlags::from(
                    crate::SecurityProtocol::WEP,
                ));
            }
            protocols
        }

        #[cfg(target_os = "macos")]
        {
            SecurityProtocols::from(self.ies.as_slice())
        }
    }

    /// The Wi-Fi protocols supported by the BSS.
    pub fn wifi_protocols(&self) -> WifiProtocols {
        WifiProtocols::from(self.ies.as_slice())
    }

    /// The Wi-Fi amendments supported by the BSS.
    pub fn wifi_amendments(&self) -> WifiAmendments {
        let amendments = WifiAmendments::from(self.ies.as_slice());
        #[cfg(any(target_os = "linux", target_os = "windows"))]
        let amendments = amendments | WifiAmendments::from(&self.capability_info);

        amendments
    }

    /// The maximum supported data rate in Mbps.
    pub fn max_rate_mbps(&self) -> f64 {
        // Iterate once through the IEs and find all of the following IEs that can be used
        // to figure out the max rate
        let (mut eht_caps, mut he_caps, mut vht_caps, mut ht_caps) = (None, None, None, None);
        let (mut supported_rates, mut extended_supported_rates) = (None, None);

        for ie in self.ies.iter() {
            match &ie.data {
                IeData::EhtCapabilities(ie_data) => eht_caps = Some(ie_data),
                IeData::HeCapabilities(ie_data) => he_caps = Some(ie_data),
                IeData::VhtCapabilities(ie_data) => vht_caps = Some(ie_data),
                IeData::HtCapabilities(ie_data) => ht_caps = Some(ie_data),
                IeData::SupportedRates(ie_data) => supported_rates = Some(ie_data),
                IeData::ExtendedSupportedRates(ie_data) => extended_supported_rates = Some(ie_data),
                _ => continue,
            }
        }

        // Find the max rate reported by an EHT/HE/VHT/HT Capability element
        let channel_width = self.channel_width();
        let rates = [
            eht_caps.map(|c| c.max_rate(channel_width)),
            he_caps.map(|c| c.max_rate(channel_width)),
            vht_caps.map(|c| c.max_rate(channel_width, ht_caps.map(|h| h.as_ref()))),
            ht_caps.map(|c| c.max_rate(channel_width)),
        ];

        if let Some(max_rate) = rates.into_iter().flatten().max_by(|a, b| a.total_cmp(b)) {
            return max_rate;
        }

        // If we don't have an EHT/HE/VHT/HT Capability element then use the extended supported
        // rates or supported rates element

        let mut max_rate = 0.0f64;
        if let Some(supported_rates) = supported_rates {
            max_rate = max_rate.max(
                supported_rates
                    .rates()
                    .iter()
                    .map(|rate| rate.value())
                    .max_by(|r1, r2| r1.total_cmp(r2))
                    .unwrap_or_default(),
            );
        }

        if let Some(extended_supported_rates) = extended_supported_rates {
            max_rate = max_rate.max(
                extended_supported_rates
                    .rates()
                    .iter()
                    .map(|rate| rate.value())
                    .max_by(|r1, r2| r1.total_cmp(r2))
                    .unwrap_or_default(),
            );
        }

        max_rate
    }

    /// The fraction of time the BSS's channel is busy, as a value from 0 to 255, where 255 represents 100%, or `None` if unavailable.
    pub fn channel_utilization(&self) -> Option<u8> {
        self.ies.iter().find_map(|ie| {
            if let IeData::BssLoad(bss_load) = &ie.data {
                Some(bss_load.channel_utilization)
            } else {
                None
            }
        })
    }

    /// The number of devices associated with the BSS, or `None` if unavailable.
    pub fn station_count(&self) -> Option<u16> {
        self.ies.iter().find_map(|ie| {
            if let IeData::BssLoad(bss_load) = &ie.data {
                Some(bss_load.station_count)
            } else {
                None
            }
        })
    }

    pub(crate) fn resolve_ie_dependencies(&mut self) {
        // Handle EHT + HE dependency
        let mut eht_capabilities = None;
        let mut he_capabilities = None;
        for ie in self.ies.iter_mut() {
            match &mut ie.data {
                IeData::EhtCapabilities(eht_caps) => eht_capabilities = Some(eht_caps),
                IeData::HeCapabilities(he_caps) => he_capabilities = Some(he_caps),
                _ => continue,
            }
        }

        if let Some(eht_capabilities) = eht_capabilities
            && let Some(he_capabilities) = he_capabilities
        {
            _ = eht_capabilities.parse_with_he_capabilities(he_capabilities);
        }
    }
}

impl Hash for Bss {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bssid.hash(state);
    }
}

impl PartialEq for Bss {
    fn eq(&self, other: &Self) -> bool {
        self.bssid == other.bssid
    }
}

impl Display for Bss {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _r = writeln!(
            f,
            "BSSID: {:?}\nSSID: {}\nRSSI: {} dBm\nChannel Number: {}\nChannel Width: {}\nWi-Fi Protocols: {}",
            self.bssid,
            self.ssid().unwrap_or_default(),
            self.signal_dbm,
            self.channel_number(),
            self.channel_width(),
            self.wifi_protocols()
        );

        #[cfg(any(target_os = "linux", target_os = "windows"))]
        let _b = writeln!(f, "{}", self.capability_info);

        Ok(())
    }
}
