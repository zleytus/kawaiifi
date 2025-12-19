use deku::{DekuError, DekuRead, DekuWrite};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

use super::IeId;
use crate::ChannelWidth;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct VhtOperation {
    pub vht_operation_information: VhtOperationInformation,
    #[deku(count = "2")]
    pub basic_vht_mcs_and_nss_set: Vec<u8>,
}

impl VhtOperation {
    pub const NAME: &'static str = "VHT Operation";
    pub const ID: u8 = 192;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const LENGTH: usize = 5;

    pub fn channel_width(&self) -> Option<ChannelWidth> {
        self.vht_operation_information.channel_width()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct VhtOperationInformation {
    #[deku(
        bytes = 1,
        map = "|value: u8| VhtChannelWidth::try_from(value).map_err(|_| DekuError::Parse(\"Invalid VhtChannelWidth\".into()))",
        writer = "u8::from(*channel_width).to_writer(deku::writer, ())"
    )]
    pub channel_width: VhtChannelWidth,
    #[deku(bytes = 1)]
    pub channel_center_frequency_segment_0: u8,
    #[deku(bytes = 1)]
    pub channel_center_frequency_segment_1: u8,
}

impl VhtOperationInformation {
    pub fn channel_width(&self) -> Option<ChannelWidth> {
        match self.channel_width {
            VhtChannelWidth::TwentyOrFortyMhz => return None,
            VhtChannelWidth::EightyOrOneHundredSixtyOrEightyPlusEightyMhz => {
                if self.channel_center_frequency_segment_1 == 0 {
                    return Some(ChannelWidth::EightyMhz);
                } else if self
                    .channel_center_frequency_segment_1
                    .abs_diff(self.channel_center_frequency_segment_0)
                    == 8
                {
                    return Some(ChannelWidth::OneSixtyMhz);
                } else {
                    return Some(ChannelWidth::EightyPlusEightyMhz);
                }
            }
            VhtChannelWidth::OneHundredSixtyMhz => return Some(ChannelWidth::OneSixtyMhz),
            VhtChannelWidth::EightyPlusEightyMhz => return Some(ChannelWidth::EightyPlusEightyMhz),
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum VhtChannelWidth {
    TwentyOrFortyMhz = 0,
    EightyOrOneHundredSixtyOrEightyPlusEightyMhz = 1,
    OneHundredSixtyMhz = 2,
    EightyPlusEightyMhz = 3,
}
