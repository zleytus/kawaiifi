use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use crate::Field;

/// Represents an unrecognized or unsupported Information Element.
#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "len: usize")]
pub struct Unknown {
    #[deku(count = "len")]
    pub data: Vec<u8>,
}

/// Unlike other IE types, this doesn't have `ID` or `IE_ID` constants
/// since it matches any IE that doesn't have a specific parser.
impl Unknown {
    pub const NAME: &'static str = "Unknown";

    pub fn summary(&self) -> String {
        "".to_string()
    }

    pub fn fields(&self) -> Vec<Field> {
        vec![
            Field::builder()
                .title("Data")
                .value("---")
                .bytes(self.data.clone())
                .build(),
        ]
    }
}
