use std::ffi::c_char;

use kawaiifi::Interface;
use windows_sys::core::GUID;

use crate::common::to_c_string;

/// Returns the GUID of the network interface, or a zeroed GUID if `interface` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_guid(interface: Option<&Interface>) -> GUID {
    interface.map(Interface::guid).unwrap_or(GUID::from_u128(0))
}

/// Returns the human-readable description of the network interface as a C string, or null if `interface` is null.
/// The caller must free the returned string with `kawaiifi_string_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_interface_description(
    interface: Option<&Interface>,
) -> *mut c_char {
    interface
        .map(|i| to_c_string(i.description()))
        .unwrap_or(std::ptr::null_mut())
}
