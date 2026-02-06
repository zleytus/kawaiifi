use deku::prelude::*;
use serde::{Deserialize, Serialize};

use crate::ies::{BitRange, Field, IeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct ErpInfo {
    #[deku(bits = 1)]
    pub non_erp_present: bool,
    #[deku(bits = 1)]
    pub use_protection: bool,
    #[deku(bits = 1)]
    pub barker_preamble_mode: bool,
    #[deku(bits = 5)]
    reserved: u8,
}

impl ErpInfo {
    pub const NAME: &'static str = "ERP Info";
    pub const ID: u8 = 42;
    pub const ID_ALT: u8 = 47;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub(crate) const IE_ID_ALT: IeId = IeId::new(Self::ID_ALT, Self::ID_EXT);
    pub const LENGTH: usize = 1;

    pub fn summary(&self) -> String {
        "".to_string()
    }

    pub fn fields(&self) -> Vec<Field> {
        vec![
            Field::builder()
                .title("Non ERP Present")
                .value(self.non_erp_present)
                .bits(BitRange::new(
                    self.to_bytes().unwrap_or_default().as_slice(),
                    0,
                    1,
                ))
                .build(),
            Field::builder()
                .title("Use Protection")
                .value(self.use_protection)
                .bits(BitRange::new(
                    self.to_bytes().unwrap_or_default().as_slice(),
                    1,
                    1,
                ))
                .build(),
            Field::builder()
                .title("Barker Preamble Mode")
                .value(self.barker_preamble_mode)
                .bits(BitRange::new(
                    self.to_bytes().unwrap_or_default().as_slice(),
                    2,
                    1,
                ))
                .build(),
            Field::builder()
                .title("Reserved")
                .value("---")
                .bits(BitRange::new(
                    self.to_bytes().unwrap_or_default().as_slice(),
                    3,
                    5,
                ))
                .build(),
        ]
    }
}
