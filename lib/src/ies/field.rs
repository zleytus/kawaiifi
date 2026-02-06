use std::fmt::Display;

use deku::bitvec::{BitVec, Lsb0};
use typed_builder::TypedBuilder;

/// A parsed field from a WiFi information element.
///
/// Fields represent individual pieces of data extracted from IEs, such as
/// "Channel Width: 80 MHz" or "MCS Index: 9". They can optionally include:
/// - The raw byte(s) the field was parsed from
/// - A bit range showing which bits within a byte encode the value
/// - Units for the value (e.g., "MHz", "dBm")
/// - Nested subfields for compound data
#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
pub struct Field {
    #[builder(setter(transform = |title: impl Display| title.to_string()))]
    title: String,

    #[builder(setter(transform = |value: impl Display| value.to_string()))]
    value: String,

    #[builder(default, setter(transform = |byte: u8| Some(byte)))]
    byte: Option<u8>,

    #[builder(default, setter(transform = |bytes: Vec<u8>| Some(bytes)))]
    bytes: Option<Vec<u8>>,

    #[builder(default, setter(transform = |bit_range: BitRange| Some(bit_range)))]
    bits: Option<BitRange>,

    #[builder(default, setter(transform = |units: impl Display| Some(units.to_string())))]
    units: Option<String>,

    #[builder(default, setter(transform = |subfields: impl IntoIterator<Item = Field>| subfields.into_iter().collect()))]
    subfields: Vec<Field>,
}

impl Field {
    /// Creates a simple field with just a title and value.
    pub fn new(title: impl Display, value: impl Display) -> Field {
        Field::builder()
            .title(title.to_string())
            .value(value.to_string())
            .build()
    }

    /// Creates a reserved field whose data has no meaning in the 802.11 protocol.
    /// Reserved bits are included for completeness when displaying IE contents.
    pub fn reserved(bits: BitRange) -> Field {
        Field::builder()
            .title("Reserved")
            .value("---")
            .bits(bits)
            .build()
    }

    /// The field's display name (e.g., "Channel Width", "Signal Strength").
    pub fn title(&self) -> &str {
        &self.title
    }

    /// The field's parsed value as a human-readable string.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// The single byte this field was parsed from, if applicable.
    pub fn byte(&self) -> Option<u8> {
        self.byte
    }

    /// The raw bytes this field was parsed from, if applicable.
    pub fn bytes(&self) -> Option<&[u8]> {
        self.bytes.as_deref()
    }

    /// A formatted string showing which bits encode this field's value.
    ///
    /// The format uses `1`/`0` for bits in the field and `.` for bits outside it,
    /// grouped into nibbles. For example, `".001 0110"` shows bits 1-5 of a byte.
    pub fn bits(&self) -> Option<String> {
        self.bits.as_ref().map(|bits| bits.to_string())
    }

    /// Nested fields for compound data structures.
    pub fn subfields(&self) -> &[Field] {
        &self.subfields
    }

    /// The units for this field's value (e.g., "MHz", "dBm", "µs").
    pub fn units(&self) -> Option<&str> {
        self.units.as_deref()
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.subfields.is_empty() {
            let mut res = write!(f, "{}: {}", self.title, self.value);
            for field in &self.subfields {
                res = write!(f, "\n- {}: {}", field.title, field.value);
            }
            res
        } else {
            write!(f, "{}: {}", self.title, self.value)
        }
    }
}

/// A range of bits within one or more bytes, used to show which bits encode a field's value.
///
/// When displayed, bits inside the range show their actual values (`0` or `1`),
/// while bits outside the range are shown as `.` to indicate they belong to other fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitRange {
    bits: deku::bitvec::BitVec<u8, Lsb0>,
    start: usize,
    length: usize,
}

impl BitRange {
    /// Creates a bit range from raw bytes.
    ///
    /// # Arguments
    /// * `bytes` - The raw bytes containing the bits
    /// * `start` - The starting bit index (0-indexed from LSB)
    /// * `length` - The number of bits in the range
    pub fn new(bytes: &[u8], start: usize, length: usize) -> Self {
        let bits = BitVec::<u8, Lsb0>::from_slice(bytes);

        Self {
            bits,
            start,
            length,
        }
    }

    /// Creates a bit range from a single byte.
    pub fn from_byte(byte: u8, start: usize, length: usize) -> Self {
        Self::new(&[byte], start, length)
    }
}

impl Display for BitRange {
    /// Formats the bit range for display.
    ///
    /// Output format: bits are grouped into nibbles separated by spaces, with bytes
    /// separated by wider gaps. Bits inside the range show `0`/`1`, bits outside show `.`.
    ///
    /// Example: For a 2-byte value where bits 4-11 are in the range:
    /// ```text
    /// .... 1010 0011 ....
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        let end = self.start + self.length;

        let num_bytes = (self.bits.len() + 7) / 8;

        // Display bytes in reverse order (MSB first) so multi-byte fields read naturally
        for byte_idx in (0..num_bytes).rev() {
            if !result.is_empty() {
                result.push(' ');
            }

            let byte_start = byte_idx * 8;
            let byte_end = ((byte_idx + 1) * 8).min(self.bits.len());

            // Display each byte MSB-first (bits 7..0)
            for bit_idx in (byte_start..byte_end).rev() {
                if bit_idx >= self.start && bit_idx < end {
                    result.push(if self.bits[bit_idx] { '1' } else { '0' });
                } else {
                    result.push('.');
                }

                // Add space between nibbles within the byte
                if (byte_end - bit_idx) % 4 == 0 && bit_idx > byte_start {
                    result.push(' ');
                }
            }
        }

        write!(f, "{}", result)
    }
}
