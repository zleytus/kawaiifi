use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::IeId;
use crate::Field;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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

    pub fn summary(&self) -> String {
        format!(
            "Mode: {}, New Channel: {}, Switch Count: {}",
            self.channel_switch_mode, self.new_channel_number, self.channel_switch_count
        )
    }

    pub fn fields(&self) -> Vec<Field> {
        vec![
            Field::builder()
                .title("Channel Switch Mode")
                .value(self.channel_switch_mode)
                .byte(self.channel_switch_mode)
                .build(),
            Field::builder()
                .title("New Channel Number")
                .value(self.new_channel_number)
                .byte(self.new_channel_number)
                .build(),
            Field::builder()
                .title("Channel Switch Count")
                .value(self.channel_switch_count)
                .byte(self.channel_switch_count)
                .build(),
        ]
    }
}
