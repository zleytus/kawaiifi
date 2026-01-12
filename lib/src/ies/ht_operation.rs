use deku::{DekuError, DekuRead, DekuWrite};
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};

use super::{IeId, write_bits_lsb0};
use crate::ChannelWidth;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct HtOperation {
    #[deku(bytes = 1)]
    pub primary_channel: u8,
    pub ht_operation_information: HtOperationInformation,
    #[deku(bytes = 16)]
    pub basic_ht_mcs_set: [u8; 16],
}

impl HtOperation {
    pub const NAME: &'static str = "HT Operation";
    pub const ID: u8 = 61;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const LENGTH: usize = 22;

    pub fn channel_width(&self) -> ChannelWidth {
        match self.ht_operation_information.sta_channel_width {
            SupportedChannelWidths::TwentyMhz => ChannelWidth::TwentyMhz,
            SupportedChannelWidths::Any => ChannelWidth::FortyMhz,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct HtOperationInformation {
    #[deku(
        bits = 2,
        map = "|value: u8| SecondaryChannelOffset::try_from(value).map_err(|_| DekuError::Parse(\"Invalid SecondaryChannelOffset\".into()))",
        writer = "write_bits_lsb0(deku::writer, *&self.secondary_channel_offset as u8, 2)"
    )]
    pub secondary_channel_offset: SecondaryChannelOffset,
    #[deku(
        bits = 1,
        map = "|value: u8| SupportedChannelWidths::try_from(value).map_err(|_| DekuError::Parse(\"SupportedChannelWidths\".into()))",
        writer = "write_bits_lsb0(deku::writer, *&self.sta_channel_width as u8, 1)"
    )]
    pub sta_channel_width: SupportedChannelWidths,
    #[deku(bits = 1)]
    pub rifs_mode: bool,
    #[deku(bits = 4)]
    reserved_1: u8,
    #[deku(
        bits = 2,
        map = "|value: u8| HtProtection::try_from(value).map_err(|_| DekuError::Parse(\"Invalid HT Protection\".into()))",
        writer = "write_bits_lsb0(deku::writer, *&self.ht_protection as u8, 2)"
    )]
    pub ht_protection: HtProtection,
    #[deku(bits = 1)]
    pub nongreenfield_ht_stas_present: bool,
    #[deku(bits = 1)]
    reserved_2: bool,
    #[deku(bits = 1)]
    pub obss_non_ht_stas_present: bool,
    #[deku(bits = 8)]
    pub channel_center_frequency_segment_two: u8,
    #[deku(bits = 3)]
    reserved_3: u8,
    #[deku(bits = 6)]
    reserved_4: u8,
    #[deku(bits = 1)]
    pub dual_beacon: bool,
    #[deku(bits = 1)]
    pub dual_cts_protection: bool,
    #[deku(bits = 1)]
    pub stbc_beacon: bool,
    #[deku(bits = 7)]
    reserved_5: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, Serialize, Deserialize)]
#[repr(u8)]
pub enum SecondaryChannelOffset {
    NoSecondary = 0,
    AbovePrimary = 1,
    BelowPrimary = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, Serialize, Deserialize)]
#[repr(u8)]
pub enum SupportedChannelWidths {
    TwentyMhz = 0,
    Any = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, Serialize, Deserialize)]
#[repr(u8)]
pub enum HtProtection {
    NoProtection = 0,
    NonmemberProtection,
    TwentyMhzProtection,
    NonHtMixed,
}
