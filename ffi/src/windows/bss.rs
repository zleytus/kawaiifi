use kawaiifi::Bss;

/// Returns the link quality of the BSS as a value from 0 to 100, or 0 if `bss` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_link_quality(bss: Option<&Bss>) -> u8 {
    bss.map(Bss::link_quality).unwrap_or_default()
}
