use std::{collections::HashSet, fmt};

use chrono::Utc;
use objc2_core_wlan::{
    CWChannel, CWChannelBand, CWChannelWidth, CWInterface, CWInterfaceMode, CWPHYMode, CWSecurity,
    CWWiFiClient,
};

use crate::{Band, Bss, ChannelWidth, Scan, ScanError};

pub(super) fn interfaces() -> Vec<Interface> {
    let client = unsafe { CWWiFiClient::sharedWiFiClient() };
    let Some(interfaces) = (unsafe { client.interfaces() }) else {
        return Vec::new();
    };

    interfaces
        .iter()
        .map(|interface| Interface { interface })
        .collect()
}

/// A Wi-Fi interface obtained from CoreWLAN.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Interface {
    interface: objc2::rc::Retained<CWInterface>,
}

impl Interface {
    /// The BSD name of the Wi-Fi interface (e.g., `en0`).
    pub fn name(&self) -> Option<String> {
        unsafe { self.interface.interfaceName() }.map(|name| name.to_string())
    }

    /// The hardware MAC address of the Wi-Fi interface.
    pub fn hardware_address(&self) -> Option<String> {
        unsafe { self.interface.hardwareAddress() }.map(|address| address.to_string())
    }

    /// The SSID the interface is currently associated with, or `None` if not associated.
    pub fn ssid(&self) -> Option<String> {
        unsafe { self.interface.ssid() }.map(|ssid| ssid.to_string())
    }

    /// The current basic service set identifier (BSSID).
    pub fn bssid(&self) -> Option<[u8; 6]> {
        unsafe { self.interface.bssid() }.and_then(|bssid| parse_bssid(&bssid.to_string()))
    }

    /// Whether the Wi-Fi interface is powered on.
    pub fn power_on(&self) -> bool {
        unsafe { self.interface.powerOn() }
    }

    /// The channels supported by this interface as `(band, channel number, width)`.
    pub fn supported_wlan_channels(&self) -> HashSet<(Band, i32, ChannelWidth)> {
        let Some(channels) = (unsafe { self.interface.supportedWLANChannels() }) else {
            return HashSet::new();
        };

        channels
            .iter()
            .map(|channel| wlan_channel_tuple(&channel))
            .collect()
    }

    /// The current WLAN channel as `(band, channel number, width)`.
    pub fn wlan_channel(&self) -> Option<(Band, i32, ChannelWidth)> {
        unsafe { self.interface.wlanChannel() }
            .as_ref()
            .map(|channel| wlan_channel_tuple(channel))
    }

    /// The currently active PHY mode.
    pub fn active_phy_mode(&self) -> CWPHYMode {
        unsafe { self.interface.activePHYMode() }
    }

    /// The current security type.
    pub fn security(&self) -> CWSecurity {
        unsafe { self.interface.security() }
    }

    /// The current transmit rate in Mbps.
    pub fn transmit_rate_mbps(&self) -> f64 {
        unsafe { self.interface.transmitRate() }
    }

    /// The currently adopted country code.
    pub fn country_code(&self) -> Option<String> {
        unsafe { self.interface.countryCode() }.map(|country_code| country_code.to_string())
    }

    /// The current operating mode.
    pub fn interface_mode(&self) -> CWInterfaceMode {
        unsafe { self.interface.interfaceMode() }
    }

    /// The current transmit power in mW.
    pub fn transmit_power_mw(&self) -> i32 {
        (unsafe { self.interface.transmitPower() }) as i32
    }

    /// The current signal strength in dBm.
    pub fn signal_dbm(&self) -> i32 {
        (unsafe { self.interface.rssiValue() }) as i32
    }

    /// The current aggregate noise measurement in dBm.
    pub fn noise_dbm(&self) -> i32 {
        (unsafe { self.interface.noiseMeasurement() }) as i32
    }

    /// Whether the network service is active.
    pub fn service_active(&self) -> bool {
        unsafe { self.interface.serviceActive() }
    }

    /// Triggers a new scan and returns the results.
    ///
    /// CoreWLAN performs scans synchronously, so this async method blocks the
    /// current task while the scan is running.
    #[tracing::instrument(skip(self), fields(interface = ?self.name()))]
    pub async fn scan(&self) -> Result<Scan, ScanError> {
        self.scan_blocking()
    }

    /// Triggers a new scan and returns the results, blocking the current thread.
    #[tracing::instrument(skip(self), fields(interface = ?self.name()))]
    pub fn scan_blocking(&self) -> Result<Scan, ScanError> {
        let start_time = Utc::now();
        let networks = unsafe { self.interface.scanForNetworksWithSSID_error(None) }?;
        let bss_list = networks
            .iter()
            .filter_map(|network| Bss::from_core_wlan_network(&network))
            .collect();
        let end_time = Utc::now();

        Ok(Scan::new(bss_list, start_time, end_time))
    }

    /// Returns the most recently cached scan results without triggering a new scan.
    ///
    /// CoreWLAN exposes cached scan results synchronously, so this async method
    /// blocks the current task while reading them.
    pub async fn cached_scan_results(&self) -> Result<Vec<Bss>, ScanError> {
        self.cached_scan_results_blocking()
    }

    /// Returns the most recently cached scan results without triggering a new scan, blocking the current thread.
    pub fn cached_scan_results_blocking(&self) -> Result<Vec<Bss>, ScanError> {
        let Some(networks) = (unsafe { self.interface.cachedScanResults() }) else {
            return Ok(Vec::new());
        };

        Ok(networks
            .iter()
            .filter_map(|network| Bss::from_core_wlan_network(&network))
            .collect())
    }
}

impl fmt::Debug for Interface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Interface")
            .field("name", &self.name())
            .field("hardware_address", &self.hardware_address())
            .field("ssid", &self.ssid())
            .field("bssid", &self.bssid())
            .field("power_on", &self.power_on())
            .field("wlan_channel", &self.wlan_channel())
            .field("active_phy_mode", &self.active_phy_mode())
            .field("security", &self.security())
            .field("transmit_rate_mbps", &self.transmit_rate_mbps())
            .field("country_code", &self.country_code())
            .field("interface_mode", &self.interface_mode())
            .field("transmit_power_mw", &self.transmit_power_mw())
            .field("service_active", &self.service_active())
            .finish_non_exhaustive()
    }
}

fn wlan_channel_tuple(channel: &CWChannel) -> (Band, i32, ChannelWidth) {
    (
        band_from_core_wlan(unsafe { channel.channelBand() }),
        unsafe { channel.channelNumber() } as i32,
        channel_width_from_core_wlan(unsafe { channel.channelWidth() }),
    )
}

fn band_from_core_wlan(band: CWChannelBand) -> Band {
    match band {
        CWChannelBand::Band2GHz => Band::TwoPointFourGhz,
        CWChannelBand::Band5GHz => Band::FiveGhz,
        CWChannelBand::Band6GHz => Band::SixGhz,
        _ => Band::TwoPointFourGhz,
    }
}

fn channel_width_from_core_wlan(channel_width: CWChannelWidth) -> ChannelWidth {
    match channel_width {
        CWChannelWidth::Width20MHz => ChannelWidth::TwentyMhz,
        CWChannelWidth::Width40MHz => ChannelWidth::FortyMhz,
        CWChannelWidth::Width80MHz => ChannelWidth::EightyMhz,
        CWChannelWidth::Width160MHz => ChannelWidth::OneSixtyMhz,
        _ => ChannelWidth::TwentyMhz,
    }
}

pub(crate) fn parse_bssid(bssid: &str) -> Option<[u8; 6]> {
    let mut bytes = [0; 6];
    let mut parts = bssid.split(':');
    for byte in &mut bytes {
        *byte = u8::from_str_radix(parts.next()?, 16).ok()?;
    }
    parts.next().is_none().then_some(bytes)
}
