use std::ffi::c_char;

use kawaiifi::Ie;

use crate::common::string_to_c;

/// Returns the element ID of the information element, or 0 if `ie` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_ie_id(ie: Option<&Ie>) -> u8 {
    ie.map(|ie| ie.id).unwrap_or_default()
}

/// Returns the length in bytes of the information element's data, or 0 if `ie` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_ie_len(ie: Option<&Ie>) -> u8 {
    ie.map(|ie| ie.len).unwrap_or_default()
}

/// Writes the extended element ID into `out`. Returns false if the IE has no extension ID or `ie` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_ie_id_ext(ie: Option<&Ie>, out: Option<&mut u8>) -> bool {
    match ie.and_then(|ie| ie.id_ext) {
        Some(id_ext) => {
            if let Some(out) = out {
                *out = id_ext;
            }
            true
        }
        None => false,
    }
}

/// Returns the IE's name as a null-terminated C string, or null if `ie` is null.
/// The caller must free the returned string with `kawaiifi_string_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_ie_name(ie: Option<&Ie>) -> *mut c_char {
    ie.map(|ie| string_to_c(ie.name().to_owned()))
        .unwrap_or(std::ptr::null_mut())
}

/// Returns the raw bytes of the IE. The caller must free with `kawaiifi_bytes_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_ie_bytes(ie: Option<&Ie>, out_count: *mut usize) -> *mut u8 {
    match ie {
        Some(ie) => {
            let mut bytes = ie.bytes();
            let count = bytes.len();
            bytes.shrink_to_fit();
            let ptr = bytes.as_mut_ptr();
            std::mem::forget(bytes);
            if !out_count.is_null() {
                unsafe { *out_count = count };
            }
            ptr
        }
        None => {
            if !out_count.is_null() {
                unsafe { *out_count = 0 };
            }
            std::ptr::null_mut()
        }
    }
}

/// Returns the IE's summary as a C string. The caller must free with `kawaiifi_string_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_ie_summary(ie: Option<&Ie>) -> *mut c_char {
    ie.map(|ie| string_to_c(ie.summary()))
        .unwrap_or(std::ptr::null_mut())
}

/// Returns the IE's fields as an opaque FieldList. The caller must free with `kawaiifi_field_list_free`.
/// Returns null if the IE is null or has no fields.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_ie_fields(ie: Option<&Ie>) -> *mut crate::field::FieldList {
    match ie {
        Some(ie) => {
            let fields = ie.fields();
            if fields.is_empty() {
                return std::ptr::null_mut();
            }
            Box::into_raw(Box::new(crate::field::FieldList::new(fields)))
        }
        None => std::ptr::null_mut(),
    }
}
