use kawaiifi::Bss;

/// Returns the noise measurement in dBm.
/// Returns 0 if bss is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_noise_dbm(bss: Option<&Bss>) -> i32 {
    bss.map(|bss| bss.noise_dbm()).unwrap_or_default()
}
