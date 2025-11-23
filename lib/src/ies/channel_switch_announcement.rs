use deku::{DekuRead, DekuWrite};

use super::IeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite)]
pub struct ChannelSwitchAnnouncement {
    channel_switch_mode: u8,
    new_channel_number: u8,
    channel_switch_count: u8,
}

impl ChannelSwitchAnnouncement {
    pub const NAME: &'static str = "Channel Switch Announcement";
    pub const ID: u8 = 37;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
}
