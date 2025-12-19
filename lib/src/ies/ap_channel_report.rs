use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "len: usize")]
pub struct ApChannelReport {
    #[deku(bytes = 1)]
    pub operating_class: u8,
    #[deku(count = "len.checked_sub(1).unwrap_or_default()")]
    pub channel_list: Vec<u8>,
}

impl ApChannelReport {
    pub const NAME: &'static str = "AP Channel Report";
    pub const ID: u8 = 51;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
}
