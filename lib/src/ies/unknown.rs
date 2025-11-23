use deku::{DekuRead, DekuWrite};

/// Represents an unrecognized or unsupported Information Element.
#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(ctx = "len: usize")]
pub struct Unknown {
    #[deku(count = "len")]
    pub data: Vec<u8>,
}

/// Unlike other IE types, this doesn't have `ID` or `IE_ID` constants
/// since it matches any IE that doesn't have a specific parser.
impl Unknown {
    pub const NAME: &'static str = "Unknown";
}
