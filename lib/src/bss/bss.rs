use std::{collections::HashMap, fmt::Display, hash::Hash, time::Duration};

use neli::{attr::Attribute, genl::Nlattr, types::Buffer};
use serde::{Deserialize, Serialize};

use crate::{
    Band, CapabilityInfo, ChannelWidth, SecurityProtocols, WifiProtocols,
    ies::{self, Ie, IeData},
    nl80211::{Bss as Nl80211Bss, BssScanWidth, BssStatus},
};

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
pub struct Bss {
    bssid: [u8; 6],
    frequency_mhz: u32,
    signal_dbm: i32,
    beacon_interval_tu: u16,
    capability_info: CapabilityInfo,
    status: BssStatus,
    ies: Vec<Ie>,
    is_from_probe_response: bool,
    parent_bssid: Option<[u8; 6]>,
    parent_tsf: Option<u64>,
    tsf: u64,
    beacon_tsf: Option<u64>,
    frequency_offset_khz: Option<u32>,
    signal_percent: Option<u8>,
    beacon_ies: Option<Vec<Ie>>,
    scan_width: Option<BssScanWidth>,
    last_seen_boottime: Option<u64>,
    seen_ms_ago: Option<u32>,
    mlo_link_id: Option<u8>,
    mld_address: Option<[u8; 6]>,
}

impl Bss {
    pub(crate) fn from_attrs<'a, I>(bss_attrs: I) -> Result<Bss, ()>
    where
        I: IntoIterator<Item = &'a Nlattr<Nl80211Bss, Buffer>>,
    {
        let bss_attrs: HashMap<_, _> = bss_attrs
            .into_iter()
            .map(|attr| (attr.nla_type().nla_type(), attr))
            .collect();

        let mut bss = Bss {
            bssid: bss_attrs
                .get(&Nl80211Bss::Bssid)
                .and_then(|attr| attr.payload().as_ref().try_into().ok())
                .ok_or(())?,
            frequency_mhz: bss_attrs
                .get(&Nl80211Bss::Frequency)
                .and_then(|attr| attr.get_payload_as().ok())
                .ok_or(())?,
            signal_dbm: bss_attrs
                .get(&Nl80211Bss::SignalMbm)
                .and_then(|attr| attr.get_payload_as::<i32>().ok())
                .ok_or(())?
                / 100,
            beacon_interval_tu: bss_attrs
                .get(&Nl80211Bss::BeaconInterval)
                .and_then(|attr| attr.get_payload_as().ok())
                .ok_or(())?,
            capability_info: bss_attrs
                .get(&Nl80211Bss::Capability)
                .and_then(|attr| attr.payload().as_ref().try_into().ok())
                .and_then(|payload: &[u8]| CapabilityInfo::try_from(payload).ok())
                .ok_or(())?,
            status: bss_attrs
                .get(&Nl80211Bss::Status)
                .and_then(|attr| attr.get_payload_as::<u32>().ok())
                .and_then(|payload| BssStatus::try_from(payload).ok())
                .unwrap_or(BssStatus::NotAssociated),
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
                .and_then(|attr| attr.get_payload_as().ok())
                .ok_or(())?,
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
        self.beacon_interval_tu as f64 * 1.024
    }

    pub fn capability_info(&self) -> &CapabilityInfo {
        &self.capability_info
    }

    pub fn status(&self) -> BssStatus {
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

    pub fn max_rate_mbps(&self) -> f64 {
        let mut max_rate = 0.0;

        for ie in self.ies.iter() {
            match &ie.data {
                IeData::SupportedRates(supported_rates) => {
                    let data_rates = supported_rates.rates();
                    if max_rate < data_rates.iter().max().unwrap().value() {
                        max_rate = data_rates.iter().max().unwrap().value();
                    }
                }
                IeData::HtOperation(_) => continue,
                IeData::VhtOperation(_) => continue,
                _ => continue,
            }
        }
        0.0
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
