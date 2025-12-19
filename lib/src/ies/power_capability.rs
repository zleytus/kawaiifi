use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::IeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct PowerCapability {
    #[deku(bytes = 1)]
    pub minimum_transmit_power_capability_dbm: i8,
    #[deku(bytes = 1)]
    pub maximum_transmit_power_capability_dbm: i8,
}

impl PowerCapability {
    pub const ID: u8 = 33;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const NAME: &'static str = "Power Capability";
}
