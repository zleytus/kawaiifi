use deku::{DekuError, DekuRead, DekuWrite};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

use super::{IeId, write_bits_lsb0};
use crate::ChannelWidth;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct EhtOperation {
    pub eht_operation_parameters: EhtOperationParameters,
    #[deku(bytes = 4)]
    pub basic_eht_mcs_and_nss_set: [u8; 4],
    #[deku(
        cond = "eht_operation_parameters.eht_operation_information_present",
        ctx = "eht_operation_parameters.disabled_subchannel_bitmap_present"
    )]
    pub eht_operation_information: Option<EhtOperationInformation>,
}

impl EhtOperation {
    pub const NAME: &'static str = "EHT Operation";
    pub const ID: u8 = 255;
    pub const ID_EXT: Option<u8> = Some(106);
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);

    pub fn channel_width(&self) -> Option<ChannelWidth> {
        self.eht_operation_information
            .map(|eht_operation_information| {
                match eht_operation_information.control.channel_width {
                    EhtChannelWidth::TwentyMhz => ChannelWidth::TwentyMhz,
                    EhtChannelWidth::FortyMhz => ChannelWidth::FortyMhz,
                    EhtChannelWidth::EightyMhz => ChannelWidth::EightyMhz,
                    EhtChannelWidth::OneHundredSixtyMhz => ChannelWidth::OneSixtyMhz,
                    EhtChannelWidth::ThreeHundredTwentyMhz => ChannelWidth::ThreeHundredTwentyMhz,
                }
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct EhtOperationParameters {
    #[deku(bits = 1)]
    pub eht_operation_information_present: bool,
    #[deku(bits = 1)]
    pub disabled_subchannel_bitmap_present: bool,
    #[deku(bits = 1)]
    pub eht_default_pe_duration: bool,
    #[deku(bits = 1)]
    pub group_addressed_bu_indication_limit: bool,
    #[deku(bits = 2)]
    pub group_addressed_bu_indication_exponent: u8,
    #[deku(bits = 1)]
    pub mcs15_disable: bool,
    #[deku(bits = 1)]
    reserved: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "disabled_subchannel_bitmap_present: bool")]
pub struct EhtOperationInformation {
    pub control: Control,
    #[deku(bytes = 1)]
    pub ccfs0: u8,
    #[deku(bytes = 1)]
    pub ccfs1: u8,
    #[deku(cond = "disabled_subchannel_bitmap_present", bytes = 2)]
    pub disabled_subchannel_bitmap: Option<u16>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct Control {
    #[deku(
        bits = 3,
        map = "|value: u8| EhtChannelWidth::try_from(value).map_err(|_| DekuError::Parse(\"Invalid EhtChannelWidth\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*channel_width), 3)"
    )]
    pub channel_width: EhtChannelWidth,
    #[deku(bits = 5)]
    reserved: u8,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum EhtChannelWidth {
    TwentyMhz,
    FortyMhz,
    EightyMhz,
    OneHundredSixtyMhz,
    ThreeHundredTwentyMhz,
}
