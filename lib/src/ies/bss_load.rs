use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct BssLoad {
    #[deku(bytes = 2)]
    pub station_count: u16,
    #[deku(bytes = 1)]
    pub channel_utilization: u8,
    #[deku(bytes = 2)]
    pub available_admission_capacity: u16,
}

impl BssLoad {
    pub const NAME: &'static str = "Bss Load";
    pub const ID: u8 = 11;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const LENGTH: usize = 5;
}
