use std::ffi::{CString, c_char};

#[cfg(any(target_os = "linux", target_os = "windows"))]
use kawaiifi::Bss;

pub fn to_c_string(s: impl Into<Vec<u8>>) -> *mut c_char {
    CString::new(s)
        .map(CString::into_raw)
        .unwrap_or(std::ptr::null_mut())
}

/// Frees a string returned by any kawaiifi function.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_string_free(s: *mut c_char) {
    if !s.is_null() {
        drop(unsafe { CString::from_raw(s) });
    }
}

/// Frees a byte buffer returned by any kawaiifi function.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bytes_free(ptr: *mut u8, count: usize) {
    if !ptr.is_null() {
        drop(unsafe { Box::from_raw(std::ptr::slice_from_raw_parts_mut(ptr, count)) });
    }
}

/// The 802.11 capability information flags advertised in beacon and probe response frames.
#[allow(dead_code)]
#[repr(C)]
pub struct CapabilityInfo {
    /// Set by an AP (1) or cleared by an IBSS or mesh STA (0).
    pub ess: bool,
    /// Set by an IBSS STA (1) or cleared by an AP or mesh STA (0).
    pub ibss: bool,
    /// Indicates data confidentiality is required for all Data frames exchanged within the BSS.
    pub privacy: bool,
    /// Indicates use of the short preamble is allowed within the BSS.
    pub short_preamble: bool,
    /// Set by an AP affiliated with an AP MLD to signal a critical update is pending. Reserved in other contexts.
    pub critical_update_flag: bool,
    /// Set by a transmitted-BSSID AP affiliated with an AP MLD if any nontransmitted BSS in its multiple BSSID set has a critical update pending. Reserved in other contexts.
    pub nontransmitted_bssids_critical_update_flag: bool,
    /// Indicates the STA implements spectrum management.
    pub spectrum_management: bool,
    /// Indicates the STA implements QoS.
    pub qos: bool,
    /// Indicates the BSS is currently using the short slot time. Always 0 for IBSS and mesh.
    pub short_slot_time: bool,
    /// Set by an AP to indicate Automatic Power Save Delivery (APSD) support. Always 0 for non-AP STAs.
    pub apsd: bool,
    /// Indicates the STA supports radio measurement.
    pub radio_measurement: bool,
    /// Indicates the STA implements EPD.
    pub epd: bool,
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
pub(crate) fn bss_capability_info(bss: Option<&Bss>) -> CapabilityInfo {
    bss.map(|b| {
        let ci = b.capability_info();
        CapabilityInfo {
            ess: ci.ess,
            ibss: ci.ibss,
            privacy: ci.privacy,
            short_preamble: ci.short_preamble,
            critical_update_flag: ci.critical_update_flag,
            nontransmitted_bssids_critical_update_flag: ci
                .nontransmitted_bssids_critical_update_flag,
            spectrum_management: ci.spectrum_management,
            qos: ci.qos,
            short_slot_time: ci.short_slot_time,
            apsd: ci.apsd,
            radio_measurement: ci.radio_measurement,
            epd: ci.epd,
        }
    })
    .unwrap_or(CapabilityInfo {
        ess: false,
        ibss: false,
        privacy: false,
        short_preamble: false,
        critical_update_flag: false,
        nontransmitted_bssids_critical_update_flag: false,
        spectrum_management: false,
        qos: false,
        short_slot_time: false,
        apsd: false,
        radio_measurement: false,
        epd: false,
    })
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
pub(crate) fn bss_tsf(bss: Option<&Bss>) -> u64 {
    bss.map(Bss::tsf).unwrap_or_default()
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
pub(crate) fn bss_last_seen_utc_ms(bss: Option<&Bss>) -> Option<i64> {
    bss.and_then(Bss::last_seen_utc)
        .map(|last_seen_utc| last_seen_utc.timestamp_millis())
}
