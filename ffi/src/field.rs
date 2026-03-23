use std::ffi::c_char;

use kawaiifi::ies::Field;

use crate::common::string_to_c;

pub struct FieldList(Vec<Field>);

impl FieldList {
    pub fn new(fields: Vec<Field>) -> Self {
        Self(fields)
    }
}

/// Returns the number of fields in the list, or 0 if `list` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_field_list_count(list: Option<&FieldList>) -> usize {
    list.map(|l| l.0.len()).unwrap_or(0)
}

/// Returns a borrowed pointer to the field at `index`, or null if out of bounds or `list` is null.
/// The pointer is valid for the lifetime of the list. Do NOT free it.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_field_list_get(
    list: Option<&FieldList>,
    index: usize,
) -> *const Field {
    list.and_then(|l| l.0.get(index))
        .map(|f| f as *const Field)
        .unwrap_or(std::ptr::null())
}

/// Frees a field list returned by `kawaiifi_ie_fields`. Does nothing if `list` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_field_list_free(list: Option<Box<FieldList>>) {
    drop(list);
}

/// Returns the field's title as a C string. The caller must free with `kawaiifi_string_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_field_title(field: Option<&Field>) -> *mut c_char {
    field
        .map(|f| string_to_c(f.title().to_string()))
        .unwrap_or(std::ptr::null_mut())
}

/// Returns the field's value as a C string. The caller must free with `kawaiifi_string_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_field_value(field: Option<&Field>) -> *mut c_char {
    field
        .map(|f| string_to_c(f.value().to_string()))
        .unwrap_or(std::ptr::null_mut())
}

/// Writes the field's byte into `out`. Returns false if unavailable or `field` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_field_byte(field: Option<&Field>, out: Option<&mut u8>) -> bool {
    match field.and_then(|f| f.byte()) {
        Some(byte) => {
            if let Some(out) = out {
                *out = byte;
            }
            true
        }
        None => false,
    }
}

/// Returns the field's raw bytes as a borrowed pointer. Valid for the lifetime of the field.
/// Do NOT free this pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_field_bytes(
    field: Option<&Field>,
    out_count: *mut usize,
) -> *const u8 {
    match field.and_then(|f| f.bytes()) {
        Some(bytes) => {
            if !out_count.is_null() {
                unsafe { *out_count = bytes.len() };
            }
            bytes.as_ptr()
        }
        None => {
            if !out_count.is_null() {
                unsafe { *out_count = 0 };
            }
            std::ptr::null()
        }
    }
}

/// Returns a formatted bit range string. The caller must free with `kawaiifi_string_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_field_bits(field: Option<&Field>) -> *mut c_char {
    field
        .and_then(|f| f.bits())
        .map(string_to_c)
        .unwrap_or(std::ptr::null_mut())
}

/// Returns the field's units as a C string. The caller must free with `kawaiifi_string_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_field_units(field: Option<&Field>) -> *mut c_char {
    field
        .and_then(|f| f.units())
        .map(|u| string_to_c(u.to_string()))
        .unwrap_or(std::ptr::null_mut())
}

/// Returns the number of subfields in the field, or 0 if `field` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_field_subfield_count(field: Option<&Field>) -> usize {
    field.map(|f| f.subfields().len()).unwrap_or(0)
}

/// Returns a borrowed pointer to the subfield at `index`, or null if out of bounds or `field` is null.
/// The pointer is valid for the lifetime of the field. Do NOT free it.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kawaiifi_field_subfield_get(
    field: Option<&Field>,
    index: usize,
) -> *const Field {
    field
        .and_then(|f| f.subfields().get(index))
        .map(|f| f as *const Field)
        .unwrap_or(std::ptr::null())
}
