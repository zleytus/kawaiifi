/// Formats a 6-byte MAC address (or BSSID) as an uppercase, colon-separated
/// hex string (e.g. `AA:BB:CC:DD:EE:FF`).
pub fn format_mac(bytes: &[u8; 6]) -> String {
    let mut s = String::with_capacity(17); // "XX:XX:XX:XX:XX:XX"
    for (i, byte) in bytes.iter().enumerate() {
        if i > 0 {
            s.push(':');
        }
        s.push_str(&format!("{byte:02X}"));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_mac_uses_uppercase_colon_separated_hex() {
        assert_eq!(
            format_mac(&[0x00, 0x1a, 0x2b, 0xcc, 0xee, 0xff]),
            "00:1A:2B:CC:EE:FF"
        );
    }
}
