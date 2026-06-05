use kawaiifi::{Bss, BssStatus};

use crate::common::{self, CapabilityInfo};

/// Returns the authentication/association status of this device with the BSS.
/// Returns `Unknown` if not authenticated or associated, or if `bss` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_status(bss: Option<&Bss>) -> BssStatus {
    bss.and_then(Bss::status).unwrap_or(BssStatus::Unknown)
}

/// Returns true if the BSS information came from a probe response, or false if from a beacon or if `bss` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_is_from_probe_response(bss: Option<&Bss>) -> bool {
    bss.map(Bss::is_from_probe_response).unwrap_or_default()
}

/// Returns the 802.11 capability information flags for the BSS.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_capability_info(bss: Option<&Bss>) -> CapabilityInfo {
    common::bss_capability_info(bss)
}

/// Returns the timing synchronization function (TSF) timer value of the BSS, or 0 if `bss` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_tsf(bss: Option<&Bss>) -> u64 {
    common::bss_tsf(bss)
}

/// Writes the Unix timestamp (milliseconds) of when the BSS was last seen into `out`.
/// Returns false if the timestamp is unavailable or `bss` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_last_seen_utc_ms(
    bss: Option<&Bss>,
    out: Option<&mut i64>,
) -> bool {
    match common::bss_last_seen_utc_ms(bss) {
        Some(last_seen_utc_ms) => {
            if let Some(out) = out {
                *out = last_seen_utc_ms;
            }
            true
        }
        None => false,
    }
}

/// Returns a borrowed pointer to the 6-byte parent BSSID, or null if unavailable or `bss` is null.
/// The pointer is valid for the lifetime of the BSS. Do NOT free it.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_parent_bssid(bss: Option<&Bss>) -> *const u8 {
    bss.and_then(Bss::parent_bssid)
        .map(|addr| addr.as_ptr())
        .unwrap_or(std::ptr::null())
}

/// Writes the TSF timer value of the transmitting BSS into `out`. Returns false if unavailable or `bss` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_parent_tsf(bss: Option<&Bss>, out: Option<&mut u64>) -> bool {
    match bss.and_then(Bss::parent_tsf) {
        Some(val) => {
            if let Some(out) = out {
                *out = val;
            }
            true
        }
        None => false,
    }
}

/// Writes the TSF timer value from the last beacon into `out`. Returns false if unavailable or `bss` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_beacon_tsf(bss: Option<&Bss>, out: Option<&mut u64>) -> bool {
    match bss.and_then(Bss::beacon_tsf) {
        Some(val) => {
            if let Some(out) = out {
                *out = val;
            }
            true
        }
        None => false,
    }
}

/// Writes the frequency offset of the BSS in kHz into `out`. Returns false if unavailable or `bss` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_frequency_offset_khz(
    bss: Option<&Bss>,
    out: Option<&mut u32>,
) -> bool {
    match bss.and_then(Bss::frequency_offset_khz) {
        Some(val) => {
            if let Some(out) = out {
                *out = val;
            }
            true
        }
        None => false,
    }
}

/// Writes the signal strength as a percentage (0–100) into `out`. Returns false if unavailable or `bss` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_signal_percent(
    bss: Option<&Bss>,
    out: Option<&mut u8>,
) -> bool {
    match bss.and_then(Bss::signal_percent) {
        Some(val) => {
            if let Some(out) = out {
                *out = val;
            }
            true
        }
        None => false,
    }
}

/// Writes the time the BSS was last seen as nanoseconds since boot into `out`.
/// Returns false if unavailable or `bss` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_last_seen_boottime_ns(
    bss: Option<&Bss>,
    out: Option<&mut u64>,
) -> bool {
    match bss.and_then(Bss::last_seen_boottime) {
        Some(val) => {
            if let Some(out) = out {
                *out = val;
            }
            true
        }
        None => false,
    }
}

/// Writes the number of milliseconds since the BSS was last seen into `out`.
/// Returns false if unavailable or `bss` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_seen_ms_ago(
    bss: Option<&Bss>,
    out: Option<&mut u32>,
) -> bool {
    match bss.and_then(Bss::seen_ms_ago) {
        Some(val) => {
            if let Some(out) = out {
                *out = val;
            }
            true
        }
        None => false,
    }
}

/// Writes the Multi-Link Operation (MLO) link ID into `out`. Returns false if unavailable or `bss` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_mlo_link_id(bss: Option<&Bss>, out: Option<&mut u8>) -> bool {
    match bss.and_then(Bss::mlo_link_id) {
        Some(val) => {
            if let Some(out) = out {
                *out = val;
            }
            true
        }
        None => false,
    }
}

/// Returns a borrowed pointer to the 6-byte MLD address, or null if unavailable or `bss` is null.
/// The pointer is valid for the lifetime of the BSS. Do NOT free it.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_mld_address(bss: Option<&Bss>) -> *const u8 {
    bss.and_then(Bss::mld_address)
        .map(|addr| addr.as_ptr())
        .unwrap_or(std::ptr::null())
}
