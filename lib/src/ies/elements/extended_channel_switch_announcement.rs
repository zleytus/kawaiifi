use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use crate::ies::{Field, IeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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

    pub fn summary(&self) -> String {
        format!(
            "Mode: {}, Operating Class: {}, New Channel: {}, Switch Count: {}",
            self.channel_switch_mode,
            self.new_operating_class,
            self.new_channel_number,
            self.channel_switch_count
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
                .title("New Operating Class")
                .value(self.new_operating_class)
                .byte(self.new_operating_class)
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
