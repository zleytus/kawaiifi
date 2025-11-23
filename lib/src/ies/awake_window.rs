use deku::prelude::*;

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
pub struct AwakeWindow {
    #[deku(bytes = 2)]
    pub awake_window_duration_micros: u8,
}

impl AwakeWindow {
    pub const ID: u8 = 157;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const NAME: &'static str = "Awake Window";
}
