use kawaiifi::Bss;

use crate::common::str_to_c;

pub struct BssList(pub Vec<Bss>);

/// Returns the number of BSSs in the list, or 0 if `list` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_list_count(list: Option<&BssList>) -> usize {
    list.map(|l| l.0.len()).unwrap_or(0)
}

/// Returns a borrowed pointer to the BSS at `index`, or null if out of bounds or `list` is null.
/// The pointer is valid for the lifetime of the list. Do NOT free it individually.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_list_get(list: Option<&BssList>, index: usize) -> *const Bss {
    list.and_then(|l| l.0.get(index))
        .map(|b| b as *const Bss)
        .unwrap_or(std::ptr::null())
}

/// Frees a BSS list returned by `kawaiifi_interface_cached_bss_list`. Does nothing if `list` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_bss_list_free(list: Option<Box<BssList>>) {
    drop(list);
}

/// FFI-safe equivalent of kawaiifi::Band.
#[repr(C)]
pub enum Band {
    TwoPointFourGhz,
    FiveGhz,
    SixGhz,
    Unknown,
}

/// FFI-safe equivalent of kawaiifi::ChannelWidth.
#[repr(C)]
pub enum ChannelWidth {
    TwentyMhz,
    FortyMhz,
    EightyMhz,
    EightyPlusEightyMhz,
    OneSixtyMhz,
    ThreeHundredTwentyMhz,
    Unknown,
}

/// FFI-safe equivalent of kawaiifi::CapabilityInfo.
#[repr(C)]
pub struct CapabilityInfo {
    pub ess: bool,
    pub ibss: bool,
    pub privacy: bool,
    pub short_preamble: bool,
    pub critical_update_flag: bool,
    pub nontransmitted_bssids_critical_update_flag: bool,
    pub spectrum_management: bool,
    pub qos: bool,
    pub short_slot_time: bool,
    pub apsd: bool,
    pub radio_measurement: bool,
    pub epd: bool,
}

/// Returns a borrowed pointer to the BSS's 6-byte BSSID (MAC address), or null if `bss` is null.
/// The pointer is valid for the lifetime of the BSS. Do NOT free it.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_bssid(bss: Option<&Bss>) -> *const u8 {
    bss.map(|b| b.bssid().as_ptr()).unwrap_or(std::ptr::null())
}

/// Returns the SSID as a C string, or null if not present.
/// The caller must free the returned string with `kawaiifi_string_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_ssid(bss: Option<&Bss>) -> *mut std::ffi::c_char {
    bss.and_then(Bss::ssid)
        .map(str_to_c)
        .unwrap_or(std::ptr::null_mut())
}

/// Returns the operating frequency of the BSS in MHz, or 0 if `bss` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_frequency_mhz(bss: Option<&Bss>) -> u32 {
    bss.map(Bss::frequency_mhz).unwrap_or_default()
}

/// Returns the band the BSS operates on (2.4 GHz, 5 GHz, or 6 GHz), or `BAND_UNKNOWN` if `bss` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_band(bss: Option<&Bss>) -> Band {
    match bss.map(Bss::band) {
        Some(kawaiifi::Band::TwoPointFourGhz) => Band::TwoPointFourGhz,
        Some(kawaiifi::Band::FiveGhz) => Band::FiveGhz,
        Some(kawaiifi::Band::SixGhz) => Band::SixGhz,
        None => Band::Unknown,
    }
}

/// Returns the channel width of the BSS, or `CHANNEL_WIDTH_UNKNOWN` if `bss` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_channel_width(bss: Option<&Bss>) -> ChannelWidth {
    match bss.map(Bss::channel_width) {
        Some(kawaiifi::ChannelWidth::TwentyMhz) => ChannelWidth::TwentyMhz,
        Some(kawaiifi::ChannelWidth::FortyMhz) => ChannelWidth::FortyMhz,
        Some(kawaiifi::ChannelWidth::EightyMhz) => ChannelWidth::EightyMhz,
        Some(kawaiifi::ChannelWidth::EightyPlusEightyMhz) => ChannelWidth::EightyPlusEightyMhz,
        Some(kawaiifi::ChannelWidth::OneSixtyMhz) => ChannelWidth::OneSixtyMhz,
        Some(kawaiifi::ChannelWidth::ThreeHundredTwentyMhz) => ChannelWidth::ThreeHundredTwentyMhz,
        None => ChannelWidth::Unknown,
    }
}

/// Returns the center frequency of the BSS in MHz, or 0 if `bss` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_center_frequency_mhz(bss: Option<&Bss>) -> u32 {
    bss.map(Bss::center_frequency_mhz).unwrap_or_default()
}

/// Returns the channel number of the BSS, or 0 if `bss` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_channel_number(bss: Option<&Bss>) -> u8 {
    bss.map(Bss::channel_number).unwrap_or_default()
}

/// Returns the signal strength of the BSS in dBm, or 0 if `bss` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_signal_dbm(bss: Option<&Bss>) -> i32 {
    bss.map(Bss::signal_dbm).unwrap_or_default()
}

/// Returns the beacon interval of the BSS in time units (TUs, 1 TU = 1024 µs), or 0 if `bss` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_beacon_interval_tu(bss: Option<&Bss>) -> u16 {
    bss.map(Bss::beacon_interval_tu).unwrap_or_default()
}

/// Returns the beacon interval of the BSS in milliseconds, or 0.0 if `bss` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_beacon_interval_ms(bss: Option<&Bss>) -> f64 {
    bss.map(Bss::beacon_interval_ms).unwrap_or_default()
}

/// Returns the 802.11 capability information flags for the BSS.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_capability_info(bss: Option<&Bss>) -> CapabilityInfo {
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

/// Returns the timing synchronization function (TSF) timer value of the BSS, or 0 if `bss` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_tsf(bss: Option<&Bss>) -> u64 {
    bss.map(Bss::tsf).unwrap_or_default()
}

/// Writes the Unix timestamp (milliseconds) of when the BSS was last seen into `out`.
/// Returns false if the timestamp is unavailable or `bss` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_last_seen_utc_ms(
    bss: Option<&Bss>,
    out: Option<&mut i64>,
) -> bool {
    match bss.and_then(Bss::last_seen_utc) {
        Some(dt) => {
            if let Some(out) = out {
                *out = dt.timestamp_millis();
            }
            true
        }
        None => false,
    }
}

/// Returns the security protocols as a bitmask (WEP=1, WPA=2, WPA2=4, WPA3=8).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_security_protocols(bss: Option<&Bss>) -> u8 {
    bss.map(|b| b.security_protocols().bits())
        .unwrap_or_default()
}

/// Returns the Wi-Fi protocols as a bitmask (a=1, b=2, g=4, n=8, ac=16, ax=32, be=64).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_wifi_protocols(bss: Option<&Bss>) -> u16 {
    bss.map(|b| b.wifi_protocols().bits()).unwrap_or_default()
}

/// Returns the maximum supported data rate of the BSS in Mbps, or 0.0 if `bss` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_max_rate_mbps(bss: Option<&Bss>) -> f64 {
    bss.map(Bss::max_rate_mbps).unwrap_or_default()
}

/// Returns the number of information elements in the BSS, or 0 if `bss` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_ie_count(bss: Option<&Bss>) -> usize {
    bss.map(|b| b.ies().len()).unwrap_or(0)
}

/// Returns a borrowed pointer to the information element at `index`, or null if out of bounds or `bss` is null.
/// The pointer is valid for the lifetime of the BSS. Do NOT free it.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_ie_get(
    bss: Option<&Bss>,
    index: usize,
) -> *const kawaiifi::Ie {
    bss.and_then(|b| b.ies().get(index))
        .map(|ie| ie as *const kawaiifi::Ie)
        .unwrap_or(std::ptr::null())
}
