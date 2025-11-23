use deku::{DekuRead, DekuWrite};

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
pub struct DsParameterSet {
    #[deku(bytes = 1)]
    pub current_channel: u8,
}

impl DsParameterSet {
    pub const NAME: &'static str = "DS Parameter Set";
    pub const ID: u8 = 3;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const LENGTH: usize = 1;
}
