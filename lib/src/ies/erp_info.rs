use deku::prelude::*;
use serde::{Deserialize, Serialize};

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
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
}
