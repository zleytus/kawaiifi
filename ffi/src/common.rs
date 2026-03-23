use std::ffi::{CString, c_char};

pub fn string_to_c(s: String) -> *mut c_char {
    CString::new(s)
        .map(CString::into_raw)
        .unwrap_or(std::ptr::null_mut())
}

pub fn str_to_c(s: &str) -> *mut c_char {
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
        drop(unsafe { Vec::from_raw_parts(ptr, count, count) });
    }
}
