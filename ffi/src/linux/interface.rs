use std::ffi::c_char;

use kawaiifi::{Interface, Scan};

use crate::bss::ChannelWidth;

use crate::common::{str_to_c, string_to_c};
use crate::linux::scan::Backend;

/// FFI-safe equivalent of kawaiifi::BusType.
#[repr(C)]
pub enum BusType {
    Pci,
    Usb,
    Sdio,
    Unknown,
}

/// Returns the interface name as a C string, or null if `interface` is null.
/// The caller must free the returned string with `kawaiifi_string_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_name(interface: Option<&Interface>) -> *mut c_char {
    interface
        .map(|i| str_to_c(i.name()))
        .unwrap_or(std::ptr::null_mut())
}

/// Returns the kernel interface index (ifindex), or 0 if `interface` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_index(interface: Option<&Interface>) -> u32 {
    interface.map(Interface::index).unwrap_or_default()
}

/// Returns the wiphy index of the physical radio this interface belongs to, or 0 if `interface` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_wiphy(interface: Option<&Interface>) -> u32 {
    interface.map(Interface::wiphy).unwrap_or_default()
}

/// Returns the wireless device identifier (wdev), or 0 if `interface` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_wdev(interface: Option<&Interface>) -> u64 {
    interface.map(Interface::wdev).unwrap_or_default()
}

/// Writes the 6-byte MAC address into `out`. Does nothing if either argument is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_mac_address(
    interface: Option<&Interface>,
    out: *mut u8,
) {
    if let Some(interface) = interface
        && !out.is_null()
    {
        unsafe { std::ptr::copy_nonoverlapping(interface.mac_address().as_ptr(), out, 6) };
    }
}

/// Returns the netlink generation counter for this interface, or 0 if `interface` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_generation(interface: Option<&Interface>) -> u32 {
    interface.map(Interface::generation).unwrap_or_default()
}

/// Returns true if the interface is operating in 4-address (WDS) mode, or false if `interface` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_four_address(interface: Option<&Interface>) -> bool {
    interface.map(Interface::four_address).unwrap_or_default()
}

/// Returns the SSID as a C string, or null if not associated or `interface` is null.
/// The caller must free the returned string with `kawaiifi_string_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_ssid(interface: Option<&Interface>) -> *mut c_char {
    interface
        .and_then(Interface::ssid)
        .map(str_to_c)
        .unwrap_or(std::ptr::null_mut())
}

/// Writes the current operating frequency of the radio in MHz into `out`.
/// Returns false if unavailable or `interface` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_wiphy_freq_mhz(
    interface: Option<&Interface>,
    out: Option<&mut u32>,
) -> bool {
    match interface.and_then(Interface::wiphy_freq_mhz) {
        Some(val) => {
            if let Some(out) = out {
                *out = val;
            }
            true
        }
        None => false,
    }
}

/// Writes the frequency offset of the radio in kHz into `out`.
/// Returns false if unavailable or `interface` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_wiphy_freq_offset_khz(
    interface: Option<&Interface>,
    out: Option<&mut u32>,
) -> bool {
    match interface.and_then(Interface::wiphy_freq_offset_khz) {
        Some(val) => {
            if let Some(out) = out {
                *out = val;
            }
            true
        }
        None => false,
    }
}

/// Writes the transmit power level in mBm (100 * dBm) into `out`.
/// Returns false if unavailable or `interface` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_wiphy_tx_power_level_mbm(
    interface: Option<&Interface>,
    out: Option<&mut u32>,
) -> bool {
    match interface.and_then(Interface::wiphy_tx_power_level_mbm) {
        Some(val) => {
            if let Some(out) = out {
                *out = val;
            }
            true
        }
        None => false,
    }
}

/// Writes the primary center frequency in MHz into `out`.
/// Returns false if unavailable or `interface` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_center_freq_1_mhz(
    interface: Option<&Interface>,
    out: Option<&mut u32>,
) -> bool {
    match interface.and_then(Interface::center_freq_1_mhz) {
        Some(val) => {
            if let Some(out) = out {
                *out = val;
            }
            true
        }
        None => false,
    }
}

/// Writes the secondary center frequency in MHz into `out` (used for 80+80 MHz channels).
/// Returns false if unavailable or `interface` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_center_freq_2_mhz(
    interface: Option<&Interface>,
    out: Option<&mut u32>,
) -> bool {
    match interface.and_then(Interface::center_freq_2_mhz) {
        Some(val) => {
            if let Some(out) = out {
                *out = val;
            }
            true
        }
        None => false,
    }
}

/// Writes the channel width into `out`. Returns false if unavailable or `interface` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_channel_width(
    interface: Option<&Interface>,
    out: Option<&mut ChannelWidth>,
) -> bool {
    match interface.and_then(Interface::channel_width) {
        Some(val) => {
            if let Some(out) = out {
                *out = match val {
                    kawaiifi::ChannelWidth::TwentyMhz => ChannelWidth::TwentyMhz,
                    kawaiifi::ChannelWidth::FortyMhz => ChannelWidth::FortyMhz,
                    kawaiifi::ChannelWidth::EightyMhz => ChannelWidth::EightyMhz,
                    kawaiifi::ChannelWidth::EightyPlusEightyMhz => ChannelWidth::EightyPlusEightyMhz,
                    kawaiifi::ChannelWidth::OneSixtyMhz => ChannelWidth::OneSixtyMhz,
                    kawaiifi::ChannelWidth::ThreeHundredTwentyMhz => ChannelWidth::ThreeHundredTwentyMhz,
                };
            }
            true
        }
        None => false,
    }
}

/// Writes the virtual interface radio mask into `out`. Returns false if unavailable or `interface` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_vif_radio_mask(
    interface: Option<&Interface>,
    out: Option<&mut u32>,
) -> bool {
    match interface.and_then(Interface::vif_radio_mask) {
        Some(val) => {
            if let Some(out) = out {
                *out = val;
            }
            true
        }
        None => false,
    }
}

/// Writes the PCI/USB vendor ID into `out`. Returns false if unavailable or `interface` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_vendor_id(
    interface: Option<&Interface>,
    out: Option<&mut u16>,
) -> bool {
    match interface.and_then(Interface::vendor_id) {
        Some(val) => {
            if let Some(out) = out {
                *out = val;
            }
            true
        }
        None => false,
    }
}

/// Writes the PCI/USB device ID into `out`. Returns false if unavailable or `interface` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_device_id(
    interface: Option<&Interface>,
    out: Option<&mut u16>,
) -> bool {
    match interface.and_then(Interface::device_id) {
        Some(val) => {
            if let Some(out) = out {
                *out = val;
            }
            true
        }
        None => false,
    }
}

/// Returns the vendor name as a C string, or null if unavailable.
/// The caller must free the returned string with `kawaiifi_string_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_vendor_name(
    interface: Option<&Interface>,
) -> *mut c_char {
    interface
        .and_then(Interface::vendor_name)
        .map(string_to_c)
        .unwrap_or(std::ptr::null_mut())
}

/// Returns the device name as a C string, or null if unavailable.
/// The caller must free the returned string with `kawaiifi_string_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_device_name(
    interface: Option<&Interface>,
) -> *mut c_char {
    interface
        .and_then(Interface::device_name)
        .map(string_to_c)
        .unwrap_or(std::ptr::null_mut())
}

/// Returns the driver name as a C string, or null if unavailable.
/// The caller must free the returned string with `kawaiifi_string_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_driver(interface: Option<&Interface>) -> *mut c_char {
    interface
        .and_then(Interface::driver)
        .map(string_to_c)
        .unwrap_or(std::ptr::null_mut())
}

/// Returns the bus type (PCI, USB, SDIO) the wireless adapter is connected via.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_bus_type(interface: Option<&Interface>) -> BusType {
    match interface.map(Interface::bus_type) {
        Some(kawaiifi::BusType::Pci) => BusType::Pci,
        Some(kawaiifi::BusType::Usb) => BusType::Usb,
        Some(kawaiifi::BusType::Sdio) => BusType::Sdio,
        _ => BusType::Unknown,
    }
}

/// Performs a blocking scan and returns the result, or null on error.
/// The caller must free the returned scan with `kawaiifi_scan_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_scan(
    interface: Option<&Interface>,
    backend: Backend,
) -> Option<Box<Scan>> {
    let interface = interface?;
    let backend = match backend {
        Backend::Nl80211 => kawaiifi::scan::Backend::Nl80211,
        Backend::NetworkManager => kawaiifi::scan::Backend::NetworkManager,
    };
    interface.scan_blocking(backend).ok().map(Box::new)
}
