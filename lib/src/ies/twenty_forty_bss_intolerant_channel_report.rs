use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::IeId;
use crate::Field;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "len: usize")]
pub struct TwentyFortyBssIntolerantChannelReport {
    #[deku(bytes = 1)]
    pub operating_class: u8,
    #[deku(count = "len.checked_sub(1).unwrap_or_default()")]
    pub channel_list: Vec<u8>,
}

impl TwentyFortyBssIntolerantChannelReport {
    pub const NAME: &'static str = "20/40 BSS Intolerant Channel Report";
    pub const ID: u8 = 73;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);

    pub fn summary(&self) -> String {
        format!("Operating Class: {}", self.operating_class)
    }

    pub fn fields(&self) -> Vec<Field> {
        vec![
            Field::builder()
                .title("Operating Class")
                .value(self.operating_class)
                .byte(self.operating_class)
                .build(),
            Field::builder()
                .title("Channel List")
                .value(
                    self.channel_list
                        .iter()
                        .map(|channel| channel.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                )
                .build(),
        ]
    }
}
