use kawaiifi::Scan;

/// FFI-safe equivalent of kawaiifi::scan::Backend.
#[repr(C)]
#[allow(dead_code)]
pub enum Backend {
    Nl80211,
    NetworkManager,
}

#[repr(C)]
pub struct Flags {
    /// The scan can be delayed or paused to allow normal data transmission
    /// or other higher priority operations to proceed.
    pub low_priority: bool,

    /// Flush cached scan results before starting a new scan.
    ///
    /// When set, the driver will discard previously cached BSS entries
    /// before reporting new scan results.
    pub flush: bool,

    /// Force a scan even if the interface is an AP.
    ///
    /// Indicates this scan was initiated by an AP, which may have
    /// different scanning behavior than client devices.
    pub ap: bool,

    /// Use a random MAC address for probe requests.
    ///
    /// Privacy feature that randomizes the device's MAC address during
    /// active scanning to prevent tracking across networks.
    pub random_addr: bool,

    /// Fill the dwell time in the FILS request parameters IE in the probe request
    pub fils_max_channel_time: bool,

    /// Accept broadcast probe responses.
    pub accept_bcast_probe_resp: bool,

    /// Send probe request frames at rate of at least 5.5M.
    pub oce_probe_req_high_tx_rate: bool,

    /// Allow probe request tx deferral and suppression.
    pub oce_probe_req_deferral_suppression: bool,

    /// Perform the scan with minimal time on each channel.
    pub low_span: bool,

    /// Perform the scan with lower power.
    pub low_power: bool,

    /// Perform the scan with highest accuracy to find all available networks.
    pub high_accuracy: bool,

    /// Use random sequence numbers in probe requests.
    pub random_sn: bool,

    /// Use minimal content in probe requests.
    pub min_preq_content: bool,

    /// Frequencies specified in kHz (not MHz).
    pub freq_khz: bool,

    /// Discover colocated 6 GHz APs through RNR.
    pub colocated_6ghz: bool,
}

/// Returns the wiphy index of the radio that performed the scan, or 0 if `scan` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_scan_wiphy(scan: Option<&Scan>) -> u32 {
    scan.map(Scan::wiphy).unwrap_or_default()
}

/// Returns the interface index (ifindex) that performed the scan, or 0 if `scan` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_scan_ifindex(scan: Option<&Scan>) -> u32 {
    scan.map(Scan::ifindex).unwrap_or_default()
}

/// Returns a borrowed pointer to the frequencies (in MHz) that were scanned and writes the count into `out_count`.
/// The pointer is valid for the lifetime of the scan. Do NOT free it.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_scan_freqs_mhz(
    scan: Option<&Scan>,
    out_count: *mut usize,
) -> *const u32 {
    match scan {
        Some(scan) => {
            let freqs = scan.freqs_mhz();
            if !out_count.is_null() {
                unsafe { *out_count = freqs.len() };
            }
            freqs.as_ptr()
        }
        None => {
            if !out_count.is_null() {
                unsafe { *out_count = 0 };
            }
            std::ptr::null()
        }
    }
}

/// Returns the number of information elements requested in the scan probe, or 0 if `scan` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_scan_ie_count(scan: Option<&Scan>) -> usize {
    scan.map(|s| s.ies().len()).unwrap_or(0)
}

/// Returns a borrowed pointer to the information element at `index`, or null if out of bounds or `scan` is null.
/// The pointer is valid for the lifetime of the scan. Do NOT free it.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_scan_ie_get(
    scan: Option<&Scan>,
    index: usize,
) -> *const kawaiifi::Ie {
    scan.and_then(|s| s.ies().get(index))
        .map(|ie| ie as *const kawaiifi::Ie)
        .unwrap_or(std::ptr::null())
}

/// Writes the scan flags into `out`. Returns false if unavailable or `scan` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_scan_flags(scan: Option<&Scan>, out: Option<&mut Flags>) -> bool {
    match scan.and_then(Scan::flags) {
        Some(val) => {
            if let Some(out) = out {
                *out = Flags {
                    low_priority: val.low_priority,
                    flush: val.flush,
                    ap: val.ap,
                    random_addr: val.random_addr,
                    fils_max_channel_time: val.fils_max_channel_time,
                    accept_bcast_probe_resp: val.accept_bcast_probe_resp,
                    oce_probe_req_high_tx_rate: val.oce_probe_req_high_tx_rate,
                    oce_probe_req_deferral_suppression: val.oce_probe_req_deferral_suppression,
                    low_span: val.low_span,
                    low_power: val.low_power,
                    high_accuracy: val.high_accuracy,
                    random_sn: val.random_sn,
                    min_preq_content: val.min_preq_content,
                    freq_khz: val.freq_khz,
                    colocated_6ghz: val.colocated_6ghz,
                };
            }
            true
        }
        None => false,
    }
}
