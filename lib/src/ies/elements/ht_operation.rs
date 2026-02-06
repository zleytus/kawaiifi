use std::fmt::Display;

use deku::{DekuContainerWrite, DekuError, DekuRead, DekuWrite};
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};

use crate::{
    BitRange, ChannelWidth, Field,
    ies::{IeId, ht_capabilities::SupportedMcsSet, write_bits_lsb0},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct HtOperation {
    #[deku(bytes = 1)]
    pub primary_channel: u8,
    pub ht_operation_information: HtOperationInformation,
    pub basic_ht_mcs_set: SupportedMcsSet,
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

    pub fn summary(&self) -> String {
        format!(
            "Primary Channel: {}, Secondary Channel: {}",
            self.primary_channel, self.ht_operation_information.secondary_channel_offset
        )
    }

    pub fn fields(&self) -> Vec<Field> {
        vec![
            Field::builder()
                .title("Primary Channel")
                .value(self.primary_channel)
                .byte(self.primary_channel)
                .build(),
            self.ht_operation_information.to_field(),
            self.basic_ht_mcs_set.to_field("Basic HT-MCS Set"),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct HtOperationInformation {
    #[deku(
        bits = 2,
        map = "|value: u8| SecondaryChannelOffset::try_from(value).map_err(|_| DekuError::Parse(\"Invalid SecondaryChannelOffset\".into()))",
        writer = "write_bits_lsb0(deku::writer, self.secondary_channel_offset as u8, 2)"
    )]
    pub secondary_channel_offset: SecondaryChannelOffset,
    #[deku(
        bits = 1,
        map = "|value: u8| SupportedChannelWidths::try_from(value).map_err(|_| DekuError::Parse(\"SupportedChannelWidths\".into()))",
        writer = "write_bits_lsb0(deku::writer, self.sta_channel_width as u8, 1)"
    )]
    pub sta_channel_width: SupportedChannelWidths,
    #[deku(bits = 1)]
    pub rifs_mode: bool,
    #[deku(bits = 4)]
    reserved_1: u8,
    #[deku(
        bits = 2,
        map = "|value: u8| HtProtection::try_from(value).map_err(|_| DekuError::Parse(\"Invalid HT Protection\".into()))",
        writer = "write_bits_lsb0(deku::writer, self.ht_protection as u8, 2)"
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

impl HtOperationInformation {
    pub fn to_field(&self) -> Field {
        let bytes = self.to_bytes().unwrap_or_default();
        Field::builder()
            .title("HT Operation Information")
            .value("")
            .subfields([
                Field::builder()
                    .title("Secondary Channel Offset")
                    .value(self.secondary_channel_offset)
                    .bits(BitRange::new(&bytes, 0, 2))
                    .build(),
                Field::builder()
                    .title("STA Channel Width")
                    .value(self.sta_channel_width)
                    .units("MHz")
                    .bits(BitRange::new(&bytes, 2, 1))
                    .build(),
                Field::builder()
                    .title("RIFS Mode")
                    .value(self.rifs_mode)
                    .bits(BitRange::new(&bytes, 3, 1))
                    .build(),
                Field::reserved(BitRange::new(&bytes, 4, 4)),
                Field::builder()
                    .title("HT Protection")
                    .value(self.ht_protection)
                    .bits(BitRange::new(&bytes, 8, 2))
                    .build(),
                Field::builder()
                    .title("Nongreenfield HT STAs Present")
                    .value(self.nongreenfield_ht_stas_present)
                    .bits(BitRange::new(&bytes, 10, 1))
                    .build(),
                Field::reserved(BitRange::new(&bytes, 11, 1)),
                Field::builder()
                    .title("OBSS Non-HT STAs Present")
                    .value(self.obss_non_ht_stas_present)
                    .bits(BitRange::new(&bytes, 12, 1))
                    .build(),
                Field::builder()
                    .title("Channel Center Frequency Segment 2")
                    .value(self.channel_center_frequency_segment_two)
                    .bits(BitRange::new(&bytes, 13, 8))
                    .build(),
                Field::reserved(BitRange::new(&bytes, 21, 3)),
                Field::reserved(BitRange::new(&bytes, 24, 6)),
                Field::builder()
                    .title("Dual Beacon")
                    .value(self.dual_beacon)
                    .bits(BitRange::new(&bytes, 30, 1))
                    .build(),
                Field::builder()
                    .title("Dual CTS Protection")
                    .value(self.dual_cts_protection)
                    .bits(BitRange::new(&bytes, 31, 1))
                    .build(),
                Field::builder()
                    .title("STBC Beacon")
                    .value(self.stbc_beacon)
                    .bits(BitRange::new(&bytes, 32, 1))
                    .build(),
                Field::reserved(BitRange::new(&bytes, 33, 7)),
            ])
            .bytes(bytes)
            .build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, Serialize, Deserialize)]
#[repr(u8)]
pub enum SecondaryChannelOffset {
    NoSecondary = 0,
    AbovePrimary = 1,
    BelowPrimary = 3,
}

impl Display for SecondaryChannelOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSecondary => write!(f, "None"),
            Self::AbovePrimary => write!(f, "Above Primary"),
            Self::BelowPrimary => write!(f, "Below Primary"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, Serialize, Deserialize)]
#[repr(u8)]
pub enum SupportedChannelWidths {
    TwentyMhz = 0,
    Any = 1,
}

impl Display for SupportedChannelWidths {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TwentyMhz => write!(f, "20"),
            Self::Any => write!(f, "20/40"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, Serialize, Deserialize)]
#[repr(u8)]
pub enum HtProtection {
    NoProtection = 0,
    NonmemberProtection,
    TwentyMhzProtection,
    NonHtMixed,
}

impl Display for HtProtection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoProtection => write!(f, "No Protection"),
            Self::NonmemberProtection => write!(f, "Nonmember Protection"),
            Self::TwentyMhzProtection => write!(f, "20 MHz Protection"),
            Self::NonHtMixed => write!(f, "Non-HT Mixed"),
        }
    }
}
