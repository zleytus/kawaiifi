use kawaiifi::Bss;

use crate::common::{self, CapabilityInfo};

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

/// Returns the link quality of the BSS as a value from 0 to 100, or 0 if `bss` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_link_quality(bss: Option<&Bss>) -> u8 {
    bss.map(Bss::link_quality).unwrap_or_default()
}
