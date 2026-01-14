use deku::prelude::*;
use serde::{Deserialize, Serialize};

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct Antenna {
    #[deku(bytes = 1)]
    pub antenna_id: u8,
}

impl Antenna {
    pub const NAME: &'static str = "Antenna";
    pub const ID: u8 = 64;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const LENGTH: usize = 1;
}
