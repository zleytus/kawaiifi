use deku::DekuContainerRead;
#[cfg(debug_assertions)]
use deku::DekuContainerWrite;

use super::Ie;

/// Parses a sequence of Information Elements from raw bytes.
///
/// Returns all successfully parsed IEs. If parsing fails partway through,
/// returns the IEs that were successfully parsed before the error.
/// Parse failures are logged at the `warn` level.
///
/// # Example
///
/// ```
/// # use kawaiifi::ies;
/// // Two IEs: SSID "Hello" + DS Parameter Set (channel 6)
/// let ie_bytes = &[
///     0x00, 0x05, 0x48, 0x65, 0x6c, 0x6c, 0x6f,  // SSID IE
///     0x03, 0x01, 0x06,                           // DS Parameter Set IE
/// ];
/// let ies = ies::from_bytes(ie_bytes);
/// assert_eq!(ies.len(), 2);
/// ```
pub fn from_bytes(bytes: &[u8]) -> Vec<Ie> {
    let mut ies = Vec::new();
    let mut input = bytes;

    while !input.is_empty() {
        let offset = bytes.len() - input.len();
        match Ie::from_bytes((input, 0)) {
            Ok(((rest, _), ie)) => {
                let bytes_read = input.len() - rest.len();
                let expected_bytes = usize::from(ie.len) + 2;
                if bytes_read != expected_bytes {
                    log::warn!(
                        "Incorrect number of bytes read for IE at offset {}: read {}, expected {}. IE: {:?}",
                        offset,
                        bytes_read,
                        expected_bytes,
                        &ie
                    );
                }
                #[cfg(debug_assertions)]
                if let Ok(serialized) = ie.to_bytes() {
                    if serialized.as_slice() != &input[..expected_bytes.min(input.len())] {
                        log::warn!(
                            "Mismatch between raw IE bytes and parsed Ie::to_bytes at offset {}. IE: {:?}",
                            offset,
                            &ie
                        );
                    }
                }
                ies.push(ie);
                input = &input[expected_bytes..];
            }
            Err(error) => {
                let failed_bytes = bytes
                    .get(offset..offset.saturating_add(20).min(bytes.len()))
                    .unwrap_or(&[]);
                log::warn!(
                    "Failed to parse IE at offset {} (parsed {} IEs successfully): {:?}. Failed bytes: {:02x?}",
                    offset,
                    ies.len(),
                    error,
                    failed_bytes
                );
                break;
            }
        }
    }

    ies
}
