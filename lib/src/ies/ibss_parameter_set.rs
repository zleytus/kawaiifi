use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct IbssParameterSet {
    #[deku(bytes = 2)]
    pub atim_window_tu: u16,
}

impl IbssParameterSet {
    pub const NAME: &'static str = "IBSS Parameter Set";
    pub const ID: u8 = 6;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const LENGTH: usize = 2;
}
