use chrono::DateTime;
use windows_sys::Win32::NetworkManagement::WiFi::WLAN_BSS_ENTRY;

use crate::{Bss, ies};

impl Bss {
    pub fn link_quality(&self) -> u8 {
        self.link_quality
    }

    pub(crate) unsafe fn from_wlan_entry(entry: *const WLAN_BSS_ENTRY) -> Self {
        let entry_ref = unsafe { &*entry };
        let ie_bytes = unsafe {
            std::slice::from_raw_parts(
                (entry as *const u8).add(entry_ref.ulIeOffset as usize),
                entry_ref.ulIeSize as usize,
            )
        };
        let ies = ies::from_bytes(ie_bytes);

        let mut bss = Bss {
            bssid: entry_ref.dot11Bssid,
            frequency_mhz: entry_ref.ulChCenterFrequency / 1_000,
            signal_dbm: entry_ref.lRssi,
            beacon_interval_tu: entry_ref.usBeaconPeriod,
            capability_info: entry_ref
                .usCapabilityInformation
                .to_le_bytes()
                .as_slice()
                .try_into()
                .unwrap(),
            last_seen_utc: {
                // 100-nanosecond intervals between 1601-01-01 (Windows FILETIME) and 1970-01-01 (Unix)
                const WINDOWS_TO_UNIX_EPOCH_100NS: u64 = 116_444_736_000_000_000;

                let unix_us = entry_ref
                    .ullHostTimestamp
                    .saturating_sub(WINDOWS_TO_UNIX_EPOCH_100NS)
                    / 10; // 100ns → µs
                DateTime::from_timestamp(
                    (unix_us / 1_000_000) as i64,
                    ((unix_us % 1_000_000) * 1_000) as u32, // remainder µs → ns
                )
            },
            tsf: entry_ref.ullTimestamp,
            link_quality: entry_ref.uLinkQuality.clamp(0, 100) as u8,
            ies,
        };
        bss.resolve_ie_dependencies();

        bss
    }
}
