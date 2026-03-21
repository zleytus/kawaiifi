use std::collections::HashMap;

use chrono::Utc;
use neli::{attr::Attribute, genl::Genlmsghdr};

use super::Bss;
use crate::{
    ies::{self, Ie},
    nl80211::{Attr, Bss as Nl80211Bss, BssScanWidth, BssStatus, Cmd, ParseError},
};

impl Bss {
    pub fn status(&self) -> Option<BssStatus> {
        self.status
    }

    pub fn is_from_probe_response(&self) -> bool {
        self.is_from_probe_response
    }

    pub fn parent_bssid(&self) -> Option<[u8; 6]> {
        self.parent_bssid
    }

    pub fn parent_tsf(&self) -> Option<u64> {
        self.parent_tsf
    }

    pub fn beacon_tsf(&self) -> Option<u64> {
        self.beacon_tsf
    }

    pub fn frequency_offset_khz(&self) -> Option<u32> {
        self.frequency_offset_khz
    }

    pub fn signal_percent(&self) -> Option<u8> {
        self.signal_percent
    }

    pub fn beacon_ies(&self) -> Option<&[Ie]> {
        self.beacon_ies.as_deref()
    }

    pub fn scan_width(&self) -> Option<BssScanWidth> {
        self.scan_width
    }

    pub fn last_seen_boottime(&self) -> Option<u64> {
        self.last_seen_boottime
    }

    pub fn seen_ms_ago(&self) -> Option<u32> {
        self.seen_ms_ago
    }

    pub fn mlo_link_id(&self) -> Option<u8> {
        self.mlo_link_id
    }

    pub fn mld_address(&self) -> Option<&[u8; 6]> {
        self.mld_address.as_ref()
    }
}

fn current_boottime_ns() -> u64 {
    let mut ts = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    unsafe { libc::clock_gettime(libc::CLOCK_BOOTTIME, &mut ts) };
    (ts.tv_sec as u64) * 1_000_000_000 + (ts.tv_nsec as u64)
}

impl TryFrom<&Genlmsghdr<Cmd, Attr>> for Bss {
    type Error = ParseError;

    fn try_from(msghdr: &Genlmsghdr<Cmd, Attr>) -> Result<Self, Self::Error> {
        if *msghdr.cmd() != Cmd::NewScanResults {
            return Err(ParseError::UnexpectedCommand {
                expected: Cmd::NewScanResults,
                got: *msghdr.cmd(),
            });
        }

        let attr_handle = msghdr.attrs().get_attr_handle();
        let bss_attrs = attr_handle.get_nested_attributes(Attr::Bss)?;

        let bss_attrs: HashMap<_, _> = bss_attrs
            .iter()
            .map(|attr| (attr.nla_type().nla_type(), attr))
            .collect();

        let mut bss = Bss {
            bssid: bss_attrs
                .get(&Nl80211Bss::Bssid)
                .ok_or(ParseError::MissingAttribute("Nl80211Bss::Bssid"))?
                .get_payload_as()?,
            frequency_mhz: bss_attrs
                .get(&Nl80211Bss::Frequency)
                .ok_or(ParseError::MissingAttribute("Nl80211Bss::Frequency"))?
                .get_payload_as()?,
            signal_dbm: bss_attrs
                .get(&Nl80211Bss::SignalMbm)
                .ok_or(ParseError::MissingAttribute("Nl80211Bss::SignalMbm"))?
                .get_payload_as::<i32>()?
                / 100,
            beacon_interval_tu: bss_attrs
                .get(&Nl80211Bss::BeaconInterval)
                .ok_or(ParseError::MissingAttribute("Nl80211Bss::BeaconInterval"))?
                .get_payload_as()?,
            capability_info: bss_attrs
                .get(&Nl80211Bss::Capability)
                .ok_or(ParseError::MissingAttribute("Nl80211Bss::Capability"))?
                .get_payload_as::<[u8; 2]>()?
                .as_ref()
                .try_into()?,
            status: bss_attrs
                .get(&Nl80211Bss::Status)
                .and_then(|attr| attr.get_payload_as::<u32>().ok())
                .and_then(|val| BssStatus::try_from(val).ok()),
            ies: bss_attrs
                .get(&Nl80211Bss::InformationElements)
                .map(|attr| ies::from_bytes(attr.payload().as_ref()))
                .unwrap_or_default(),
            is_from_probe_response: bss_attrs.contains_key(&Nl80211Bss::PrespData),
            parent_bssid: bss_attrs
                .get(&Nl80211Bss::ParentBssid)
                .and_then(|attr| attr.payload().as_ref().try_into().ok()),
            parent_tsf: bss_attrs
                .get(&Nl80211Bss::ParentTsf)
                .and_then(|attr| attr.get_payload_as().ok()),
            tsf: bss_attrs
                .get(&Nl80211Bss::Tsf)
                .ok_or(ParseError::MissingAttribute("Nl80211Bss::Tsf"))?
                .get_payload_as()?,
            beacon_tsf: bss_attrs
                .get(&Nl80211Bss::BeaconTsf)
                .and_then(|attr| attr.get_payload_as().ok()),
            frequency_offset_khz: bss_attrs
                .get(&Nl80211Bss::FrequencyOffset)
                .and_then(|attr| attr.get_payload_as().ok()),
            signal_percent: bss_attrs
                .get(&Nl80211Bss::SignalUnspec)
                .and_then(|attr| attr.get_payload_as().ok()),
            beacon_ies: bss_attrs
                .get(&Nl80211Bss::BeaconIes)
                .map(|attr| ies::from_bytes(attr.payload().as_ref())),
            scan_width: bss_attrs.get(&Nl80211Bss::ChanWidth).and_then(|attr| {
                BssScanWidth::try_from(attr.get_payload_as::<u32>().unwrap_or_default()).ok()
            }),
            last_seen_boottime: bss_attrs
                .get(&Nl80211Bss::LastSeenBoottime)
                .and_then(|attr| attr.get_payload_as().ok()),
            seen_ms_ago: bss_attrs
                .get(&Nl80211Bss::SeenMsAgo)
                .and_then(|attr| attr.get_payload_as().ok()),
            last_seen_utc: {
                let last_seen_boottime: Option<u64> = bss_attrs
                    .get(&Nl80211Bss::LastSeenBoottime)
                    .and_then(|attr| attr.get_payload_as().ok());
                let seen_ms_ago: Option<u32> = bss_attrs
                    .get(&Nl80211Bss::SeenMsAgo)
                    .and_then(|attr| attr.get_payload_as().ok());
                if let Some(boottime_ns) = last_seen_boottime {
                    let ago_ns = current_boottime_ns().saturating_sub(boottime_ns);
                    Utc::now().checked_sub_signed(chrono::Duration::nanoseconds(ago_ns as i64))
                } else {
                    seen_ms_ago.and_then(|ms| {
                        Utc::now().checked_sub_signed(chrono::Duration::milliseconds(ms as i64))
                    })
                }
            },
            mlo_link_id: bss_attrs
                .get(&Nl80211Bss::MloLinkId)
                .and_then(|attr| attr.get_payload_as().ok()),
            mld_address: bss_attrs
                .get(&Nl80211Bss::MldAddr)
                .and_then(|attr| attr.payload().as_ref().try_into().ok()),
        };
        bss.resolve_ie_dependencies();

        Ok(bss)
    }
}
