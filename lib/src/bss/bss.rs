use std::{collections::HashMap, fmt::Display, hash::Hash, time::Duration};

use chrono::{DateTime, Utc};
use neli::{attr::Attribute, genl::Genlmsghdr};
use serde::{Deserialize, Serialize};

use crate::{
    Band, CapabilityInfo, ChannelWidth, SecurityProtocols, WifiProtocols,
    ies::{self, Ie, IeData},
    nl80211::{Attr, Bss as Nl80211Bss, BssScanWidth, BssStatus, Cmd, ParseError},
};

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
pub struct Bss {
    bssid: [u8; 6],
    frequency_mhz: u32,
    signal_dbm: i32,
    beacon_interval_tu: u16,
    #[serde(with = "crate::ies::serde_raw::capability_info_as_u16")]
    capability_info: CapabilityInfo,
    status: Option<BssStatus>,
    #[serde(with = "crate::ies::serde_raw::ies_as_base64")]
    ies: Vec<Ie>,
    is_from_probe_response: bool,
    parent_bssid: Option<[u8; 6]>,
    parent_tsf: Option<u64>,
    tsf: u64,
    beacon_tsf: Option<u64>,
    frequency_offset_khz: Option<u32>,
    signal_percent: Option<u8>,
    #[serde(with = "crate::ies::serde_raw::option_ies_as_base64")]
    beacon_ies: Option<Vec<Ie>>,
    scan_width: Option<BssScanWidth>,
    last_seen_boottime: Option<u64>,
    seen_ms_ago: Option<u32>,
    last_seen_utc: Option<DateTime<Utc>>,
    mlo_link_id: Option<u8>,
    mld_address: Option<[u8; 6]>,
}

impl Bss {
    pub fn bssid(&self) -> &[u8; 6] {
        &self.bssid
    }

    pub fn frequency_mhz(&self) -> u32 {
        self.frequency_mhz
    }

    pub fn band(&self) -> Band {
        Band::from_freq_mhz(self.frequency_mhz)
    }

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
        if let Some(eht_op) = eht_op {
            if let Some(info) = eht_op.eht_operation_information {
                if info.ccfs1 != 0 {
                    return chan_to_freq(info.ccfs1);
                }
                if info.ccfs0 != 0 {
                    return chan_to_freq(info.ccfs0);
                }
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

    pub fn channel_number(&self) -> u8 {
        self.ies
            .iter()
            .find_map(|ie| match &ie.data {
                IeData::DsParameterSet(ds_parameter_set) => Some(ds_parameter_set.current_channel),
                IeData::HtOperation(ht_operation) => Some(ht_operation.primary_channel),
                _ => None,
            })
            .unwrap_or_else(|| match self.band() {
                Band::TwoPointFourGhz => {
                    if self.frequency_mhz() == 2484 {
                        14
                    } else {
                        ((self.frequency_mhz() - 2407) / 5) as u8
                    }
                }
                Band::FiveGhz => ((self.frequency_mhz() - 5000) / 5) as u8,
                Band::SixGhz => ((self.frequency_mhz() - 5950) / 5) as u8,
            })
    }

    pub fn signal_dbm(&self) -> i32 {
        self.signal_dbm
    }

    pub fn beacon_interval_tu(&self) -> u16 {
        self.beacon_interval_tu
    }

    pub fn beacon_interval_ms(&self) -> f64 {
        f64::from(self.beacon_interval_tu) * 1.024
    }

    pub fn capability_info(&self) -> &CapabilityInfo {
        &self.capability_info
    }

    pub fn status(&self) -> Option<BssStatus> {
        self.status
    }

    pub fn ies(&self) -> &[Ie] {
        &self.ies
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

    pub fn tsf(&self) -> u64 {
        self.tsf
    }

    pub fn uptime(&self) -> Duration {
        Duration::from_micros(self.tsf)
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

    pub fn last_seen_utc(&self) -> Option<DateTime<Utc>> {
        self.last_seen_utc
    }

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

    pub fn security_protocols(&self) -> SecurityProtocols {
        SecurityProtocols::from(self.ies.as_slice())
    }

    pub fn wifi_protocols(&self) -> WifiProtocols {
        WifiProtocols::from(self.ies.as_slice())
    }

    /// The max data rate of the BSS in Mbps
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

    pub fn mlo_link_id(&self) -> Option<u8> {
        self.mlo_link_id
    }

    pub fn mld_address(&self) -> Option<&[u8; 6]> {
        self.mld_address.as_ref()
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
            _ = eht_capabilities.parse_with_he_capabilities(&he_capabilities);
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

fn current_boottime_ns() -> u64 {
    let mut ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
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

        let _b = writeln!(f, "{}", self.capability_info);

        Ok(())
    }
}
