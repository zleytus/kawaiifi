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

/// Returns the Wi-Fi amendments as a bitmask (d=1, e=2, h=4, i=8, k=16, r=32, s=64, v=128, w=256).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_wifi_amendments(bss: Option<&Bss>) -> u16 {
    bss.map(|b| b.wifi_amendments().bits()).unwrap_or_default()
}

/// Returns the maximum supported data rate of the BSS in Mbps, or 0.0 if `bss` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_max_rate_mbps(bss: Option<&Bss>) -> f64 {
    bss.map(Bss::max_rate_mbps).unwrap_or_default()
}

/// Returns the fraction of time the BSS's channel is busy, as a value from 0 to 255, where 255 represents 100%.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_channel_utilization(
    bss: Option<&Bss>,
    channel_utilization: Option<&mut u8>,
) -> bool {
    let Some(bss) = bss else {
        return false;
    };

    if let Some(utilization) = bss.channel_utilization()
        && let Some(channel_utilization) = channel_utilization
    {
        *channel_utilization = utilization;
        true
    } else {
        false
    }
}

/// Returns the number of devices associated with the BSS.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_bss_station_count(
    bss: Option<&Bss>,
    station_count: Option<&mut u16>,
) -> bool {
    let Some(bss) = bss else {
        return false;
    };

    if let Some(count) = bss.station_count()
        && let Some(station_count) = station_count
    {
        *station_count = count;
        true
    } else {
        false
    }
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
