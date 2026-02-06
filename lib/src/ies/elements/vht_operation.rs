use std::fmt::Display;

use deku::{DekuContainerWrite, DekuError, DekuRead, DekuWrite};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

use crate::{
    ChannelWidth, Field,
    ies::{IeId, vht_capabilities::VhtMcsMap},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct VhtOperation {
    pub vht_operation_information: VhtOperationInformation,
    pub basic_vht_mcs_and_nss_set: VhtMcsMap,
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

    pub fn summary(&self) -> String {
        let mut summary = Vec::new();
        summary.push(format!(
            "{} MHz",
            self.vht_operation_information.channel_width
        ));

        if self
            .vht_operation_information
            .channel_center_frequency_segment_0
            > 0
        {
            summary.push(format!(
                "Channel Center Frequency Segment 0: {}",
                self.vht_operation_information
                    .channel_center_frequency_segment_0
            ));
        }

        if self
            .vht_operation_information
            .channel_center_frequency_segment_1
            > 0
        {
            summary.push(format!(
                "Channel Center Frequency Segment 1: {}",
                self.vht_operation_information
                    .channel_center_frequency_segment_1
            ));
        }

        summary.join(", ")
    }

    pub fn fields(&self) -> Vec<Field> {
        vec![
            self.vht_operation_information.to_field(),
            self.basic_vht_mcs_and_nss_set
                .to_field("Basic VHT-MCS and NSS Set"),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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

    pub fn to_field(&self) -> Field {
        Field::builder()
            .title("VHT Operation Information")
            .value("")
            .bytes(self.to_bytes().unwrap_or_default())
            .subfields([
                Field::builder()
                    .title("VHT Channel Width")
                    .value(self.channel_width)
                    .units("MHz")
                    .byte(self.channel_width.into())
                    .build(),
                Field::builder()
                    .title("Channel Center Frequency Segment 0")
                    .value(self.channel_center_frequency_segment_0)
                    .byte(self.channel_center_frequency_segment_0)
                    .build(),
                Field::builder()
                    .title("Channel Center Frequency Segment 1")
                    .value(self.channel_center_frequency_segment_1)
                    .byte(self.channel_center_frequency_segment_1)
                    .build(),
            ])
            .build()
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum VhtChannelWidth {
    TwentyOrFortyMhz = 0,
    EightyOrOneHundredSixtyOrEightyPlusEightyMhz = 1,
    OneHundredSixtyMhz = 2,
    EightyPlusEightyMhz = 3,
}

impl Display for VhtChannelWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TwentyOrFortyMhz => write!(f, "20/40"),
            Self::EightyOrOneHundredSixtyOrEightyPlusEightyMhz => write!(f, "80/160/80+80"),
            Self::OneHundredSixtyMhz => write!(f, "160"),
            Self::EightyPlusEightyMhz => write!(f, "80+80"),
        }
    }
}
