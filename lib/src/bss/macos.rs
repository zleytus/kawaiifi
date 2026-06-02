use objc2_core_wlan::{CWChannelBand, CWNetwork};

use crate::{Bss, ies, interface::parse_bssid};

impl Bss {
    /// The noise measurement in dBm.
    pub fn noise_dbm(&self) -> i32 {
        self.noise_dbm
    }

    pub(crate) fn from_core_wlan_network(network: &CWNetwork) -> Option<Self> {
        let bssid = unsafe { network.bssid() }
            .and_then(|bssid| parse_bssid(&bssid.to_string()))
            .unwrap_or_default();
        let channel = unsafe { network.wlanChannel() }?;
        let information_element_data = unsafe { network.informationElementData() };
        let ies = information_element_data
            .as_ref()
            .map(|data| ies::from_bytes(unsafe { data.as_bytes_unchecked() }))
            .unwrap_or_default();

        let mut bss = Bss {
            bssid,
            frequency_mhz: channel_to_frequency_mhz(
                unsafe { channel.channelNumber() } as u32,
                unsafe { channel.channelBand() },
            ),
            signal_dbm: unsafe { network.rssiValue() } as i32,
            beacon_interval_tu: beacon_interval_ms_to_tu(unsafe { network.beaconInterval() }),
            ies,
            noise_dbm: unsafe { network.noiseMeasurement() } as i32,
        };
        bss.resolve_ie_dependencies();

        Some(bss)
    }
}

fn beacon_interval_ms_to_tu(beacon_interval_ms: isize) -> u16 {
    if beacon_interval_ms <= 0 {
        return 0;
    }

    ((beacon_interval_ms as f64) / 1.024)
        .round()
        .clamp(0.0, u16::MAX as f64) as u16
}

fn channel_to_frequency_mhz(channel: u32, band: CWChannelBand) -> u32 {
    match band {
        CWChannelBand::Band2GHz => match channel {
            14 => 2484,
            _ => 2407 + channel * 5,
        },
        CWChannelBand::Band6GHz => 5950 + channel * 5,
        _ => 5000 + channel * 5,
    }
}
