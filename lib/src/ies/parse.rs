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
                    tracing::warn!(
                        offset,
                        bytes_read,
                        expected_bytes,
                        ie = ?ie,
                        "Incorrect number of bytes read for IE"
                    );
                }
                #[cfg(debug_assertions)]
                if let Ok(serialized) = ie.to_bytes()
                    && serialized.as_slice() != &input[..expected_bytes.min(input.len())]
                {
                    tracing::warn!(
                        offset,
                        ie = ?ie,
                        "Mismatch between raw IE bytes and serialized IE"
                    );
                }
                ies.push(ie);
                if expected_bytes > input.len() {
                    tracing::warn!(
                        offset,
                        expected_bytes,
                        remaining_bytes = input.len(),
                        "IE declares more bytes than remain"
                    );
                    break;
                }
                input = &input[expected_bytes..];
            }
            Err(error) => {
                let failed_bytes = bytes
                    .get(offset..offset.saturating_add(20).min(bytes.len()))
                    .unwrap_or(&[]);
                tracing::warn!(
                    offset,
                    parsed_count = ies.len(),
                    total_bytes = bytes.len(),
                    remaining_bytes = input.len(),
                    error = ?error,
                    failed_bytes = ?failed_bytes,
                    "Failed to parse IE"
                );
                tracing::debug!(
                    offset,
                    parsed_count = ies.len(),
                    total_bytes = bytes.len(),
                    full_bytes = ?bytes,
                    parsed_ies = ?ies,
                    "Full IE parse failure context"
                );
                break;
            }
        }
    }

    ies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_bytes_stops_on_truncated_ie() {
        let ies = from_bytes(&[0x00, 0x05, b'H', b'i']);

        assert!(ies.is_empty());
    }
}
