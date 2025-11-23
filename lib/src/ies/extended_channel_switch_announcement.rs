use deku::{DekuRead, DekuWrite};

use super::IeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite)]
pub struct ExtendedChannelSwitchAnnouncement {
    #[deku(bytes = 1)]
    pub channel_switch_mode: u8,
    #[deku(bytes = 1)]
    pub new_operating_class: u8,
    #[deku(bytes = 1)]
    pub new_channel_number: u8,
    #[deku(bytes = 1)]
    pub channel_switch_count: u8,
}

impl ExtendedChannelSwitchAnnouncement {
    pub const ID: u8 = 60;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const NAME: &'static str = "Extended Channel Switch Announcement";
}
