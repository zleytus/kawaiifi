use kawaiifi::Scan;

/// Returns the number of BSSes in the scan, or 0 if `scan` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_scan_bss_count(scan: Option<&Scan>) -> usize {
    scan.map(|s| s.bss_list().len()).unwrap_or(0)
}

/// Returns a borrowed pointer to the BSS at `index`, or null if out of bounds or `scan` is null.
/// The pointer is valid for the lifetime of the scan. Do NOT free it.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_scan_bss_get(
    scan: Option<&Scan>,
    index: usize,
) -> *const kawaiifi::Bss {
    scan.and_then(|s| s.bss_list().get(index))
        .map(|bss| bss as *const kawaiifi::Bss)
        .unwrap_or(std::ptr::null())
}

/// Returns the start time of the scan as a Unix timestamp in milliseconds, or 0 if `scan` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_scan_start_time_utc_ms(scan: Option<&Scan>) -> i64 {
    scan.map(|scan| scan.start_time().timestamp_millis())
        .unwrap_or_default()
}

/// Returns the end time of the scan as a Unix timestamp in milliseconds, or 0 if `scan` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_scan_end_time_utc_ms(scan: Option<&Scan>) -> i64 {
    scan.map(|scan| scan.end_time().timestamp_millis())
        .unwrap_or_default()
}

/// Frees a scan returned by `kawaiifi_interface_scan`. Does nothing if `scan` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_scan_free(scan: Option<&mut Scan>) {
    if let Some(scan) = scan {
        drop(unsafe { Box::from_raw(scan) });
    }
}
