use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::IeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct TpcReport {
    #[deku(bytes = 1)]
    pub transmit_power_dbm: i8,
    #[deku(bytes = 1)]
    pub link_margin_db: i8,
}

impl TpcReport {
    pub const NAME: &'static str = "TPC Report";
    pub const ID: u8 = 35;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
}
