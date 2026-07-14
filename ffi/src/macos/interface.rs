use std::ffi::c_char;

use kawaiifi::Interface;
use objc2_core_wlan::{CWInterfaceMode, CWPHYMode, CWSecurity};

use crate::common::to_c_string;

/// FFI-safe equivalent of CoreWLAN's CWPHYMode.
#[derive(Debug)]
#[repr(i32)]
pub enum CwPhyMode {
    /// No active PHY mode.
    None = 0,
    /// 802.11a.
    A = 1,
    /// 802.11b.
    B = 2,
    /// 802.11g.
    G = 3,
    /// 802.11n.
    N = 4,
    /// 802.11ac.
    AC = 5,
    /// 802.11ax.
    AX = 6,
}

impl From<CWPHYMode> for CwPhyMode {
    fn from(value: CWPHYMode) -> Self {
        match value {
            CWPHYMode::Mode11a => Self::A,
            CWPHYMode::Mode11b => Self::B,
            CWPHYMode::Mode11g => Self::G,
            CWPHYMode::Mode11n => Self::N,
            CWPHYMode::Mode11ac => Self::AC,
            CWPHYMode::Mode11ax => Self::AX,
            _ => Self::None,
        }
    }
}

/// FFI-safe equivalent of CoreWLAN's CWSecurity.
#[derive(Debug)]
#[repr(i32)]
pub enum CwSecurity {
    /// No security.
    None = 0,
    /// WEP security.
    Wep = 1,
    /// WPA Personal security.
    WpaPersonal = 2,
    /// Mixed WPA Personal security.
    WpaPersonalMixed = 3,
    /// WPA2 Personal security.
    Wpa2Personal = 4,
    /// Personal security.
    Personal = 5,
    /// Dynamic WEP security.
    DynamicWep = 6,
    /// WPA Enterprise security.
    WpaEnterprise = 7,
    /// Mixed WPA Enterprise security.
    WpaEnterpriseMixed = 8,
    /// WPA2 Enterprise security.
    Wpa2Enterprise = 9,
    /// Enterprise security.
    Enterprise = 10,
    /// WPA3 Personal security.
    Wpa3Personal = 11,
    /// WPA3 Enterprise security.
    Wpa3Enterprise = 12,
    /// WPA3 transition security.
    Wpa3Transition = 13,
    /// Opportunistic Wireless Encryption security.
    Owe = 14,
    /// Opportunistic Wireless Encryption transition security.
    OweTransition = 15,
    /// Unknown security type.
    Unknown = -1,
}

impl From<CWSecurity> for CwSecurity {
    fn from(value: CWSecurity) -> Self {
        match value {
            CWSecurity::None => Self::None,
            CWSecurity::WEP => Self::Wep,
            CWSecurity::WPAPersonal => Self::WpaPersonal,
            CWSecurity::WPAPersonalMixed => Self::WpaPersonalMixed,
            CWSecurity::WPA2Personal => Self::Wpa2Personal,
            CWSecurity::Personal => Self::Personal,
            CWSecurity::DynamicWEP => Self::DynamicWep,
            CWSecurity::WPAEnterprise => Self::WpaEnterprise,
            CWSecurity::WPAEnterpriseMixed => Self::WpaEnterpriseMixed,
            CWSecurity::WPA2Enterprise => Self::Wpa2Enterprise,
            CWSecurity::Enterprise => Self::Enterprise,
            CWSecurity::WPA3Personal => Self::Wpa3Personal,
            CWSecurity::WPA3Enterprise => Self::Wpa3Enterprise,
            CWSecurity::WPA3Transition => Self::Wpa3Transition,
            CWSecurity::OWE => Self::Owe,
            CWSecurity::OWETransition => Self::OweTransition,
            _ => Self::Unknown,
        }
    }
}

/// FFI-safe equivalent of CoreWLAN's CWInterfaceMode.
#[derive(Debug)]
#[repr(i32)]
pub enum CwInterfaceMode {
    /// No interface mode.
    None = 0,
    /// Station/client mode.
    Station = 1,
    /// IBSS/ad-hoc mode.
    Ibss = 2,
    /// Host access point mode.
    HostAp = 3,
}

impl From<CWInterfaceMode> for CwInterfaceMode {
    fn from(value: CWInterfaceMode) -> Self {
        match value {
            CWInterfaceMode::Station => Self::Station,
            CWInterfaceMode::IBSS => Self::Ibss,
            CWInterfaceMode::HostAP => Self::HostAp,
            _ => Self::None,
        }
    }
}

/// Returns the BSD name of the Wi-Fi interface as a C string, or null if unavailable or `interface` is null.
/// The caller must free the returned string with `kawaiifi_string_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_name(interface: Option<&Interface>) -> *mut c_char {
    interface
        .and_then(Interface::name)
        .map(to_c_string)
        .unwrap_or(std::ptr::null_mut())
}

/// Returns the current security type.
/// Returns `Unknown` if `interface` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_security(interface: Option<&Interface>) -> CwSecurity {
    interface
        .map(|interface| CwSecurity::from(interface.security()))
        .unwrap_or(CwSecurity::Unknown)
}

/// Returns the currently active PHY mode.
/// Returns `None` if `interface` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_active_phy_mode(
    interface: Option<&Interface>,
) -> CwPhyMode {
    interface
        .map(|interface| CwPhyMode::from(interface.active_phy_mode()))
        .unwrap_or(CwPhyMode::None)
}

/// Returns the current operating mode.
/// Returns `None` if `interface` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_mode(interface: Option<&Interface>) -> CwInterfaceMode {
    interface
        .map(|interface| CwInterfaceMode::from(interface.interface_mode()))
        .unwrap_or(CwInterfaceMode::None)
}

/// Returns the hardware MAC address as a C string, or null if unavailable or `interface` is null.
/// The caller must free the returned string with `kawaiifi_string_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_hardware_address(
    interface: Option<&Interface>,
) -> *mut c_char {
    interface
        .and_then(Interface::hardware_address)
        .map(to_c_string)
        .unwrap_or(std::ptr::null_mut())
}

/// Returns the SSID as a C string, or null if unavailable, not associated, or `interface` is null.
/// The caller must free the returned string with `kawaiifi_string_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_ssid(interface: Option<&Interface>) -> *mut c_char {
    interface
        .and_then(Interface::ssid)
        .map(to_c_string)
        .unwrap_or(std::ptr::null_mut())
}

/// Writes the 6-byte current BSSID into `out`.
/// Returns false if unavailable, not associated, `interface` is null, or `out` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_bssid(
    interface: Option<&Interface>,
    out: *mut u8,
) -> bool {
    let Some(bssid) = interface.and_then(Interface::bssid) else {
        return false;
    };
    if out.is_null() {
        return false;
    }

    unsafe { std::ptr::copy_nonoverlapping(bssid.as_ptr(), out, 6) };
    true
}

/// Returns true if the Wi-Fi interface is powered on.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_power_on(interface: Option<&Interface>) -> bool {
    interface.map(Interface::power_on).unwrap_or_default()
}

/// Returns the current transmit rate in Mbps.
/// Returns 0 if `interface` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_transmit_rate_mbps(
    interface: Option<&Interface>,
) -> f64 {
    interface
        .map(|interface| interface.transmit_rate_mbps())
        .unwrap_or_default()
}

/// Returns the current transmit power in mW.
/// Returns 0 if `interface` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_transmit_power_mw(
    interface: Option<&Interface>,
) -> i32 {
    interface
        .map(|interface| interface.transmit_power_mw())
        .unwrap_or_default()
}

/// Returns the current signal strength in dBm.
/// Returns 0 if `interface` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_signal_dbm(interface: Option<&Interface>) -> i32 {
    interface
        .map(|interface| interface.signal_dbm())
        .unwrap_or_default()
}

/// Returns the current noise in dBm.
/// Returns 0 if `interface` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_noise_dbm(interface: Option<&Interface>) -> i32 {
    interface
        .map(|interface| interface.noise_dbm())
        .unwrap_or_default()
}

/// Returns the currently adopted country code as a C string, or null if unavailable or `interface` is null.
/// The caller must free the returned string with `kawaiifi_string_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_country_code(
    interface: Option<&Interface>,
) -> *mut c_char {
    interface
        .and_then(Interface::country_code)
        .map(to_c_string)
        .unwrap_or(std::ptr::null_mut())
}

/// Returns true if the network service is active.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_service_active(interface: Option<&Interface>) -> bool {
    interface.map(Interface::service_active).unwrap_or_default()
}
