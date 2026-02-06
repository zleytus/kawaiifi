use std::fmt::Display;

use deku::{DekuContainerWrite, DekuError, DekuRead, DekuWrite};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

use super::{he_capabilities::HeMcsMap, vht_operation::VhtOperationInformation};
use crate::ChannelWidth;
use crate::ies::{BitRange, Field, IeId, write_bits_lsb0};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct HeOperation {
    pub he_operation_parameters: HeOperationParameters,
    pub bss_color_information: BssColorInformation,
    pub basic_he_mcs_and_nss_set: HeMcsMap,
    #[deku(cond = "he_operation_parameters.vht_operation_information_present")]
    pub vht_operation_information: Option<VhtOperationInformation>,
    #[deku(cond = "he_operation_parameters.cohosted_bss")]
    pub max_cohosted_bssid_indicator: Option<u8>,
    #[deku(cond = "he_operation_parameters.six_ghz_operation_information_present")]
    pub six_ghz_operation_information: Option<SixGhzOperationInformation>,
}

impl HeOperation {
    pub const LENGTH: usize = 22;
    pub const NAME: &'static str = "HE Operation";
    pub const ID: u8 = 255;
    pub const ID_EXT: Option<u8> = Some(36);
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);

    pub fn channel_width(&self) -> Option<ChannelWidth> {
        // If 6 GHz information is present, use it to determine channel width
        if let Some(six_ghz_operation_information) = &self.six_ghz_operation_information {
            match six_ghz_operation_information.control.channel_width {
                SixGhzHeChannelWidth::TwentyMhz => return Some(ChannelWidth::TwentyMhz),
                SixGhzHeChannelWidth::FortyMhz => return Some(ChannelWidth::FortyMhz),
                SixGhzHeChannelWidth::EightyMhz => return Some(ChannelWidth::EightyMhz),
                SixGhzHeChannelWidth::EightyPlusEightyOrOneHundredSixtyMhz => {
                    if six_ghz_operation_information
                        .channel_center_frequency_segment_1
                        .abs_diff(six_ghz_operation_information.channel_center_frequency_segment_0)
                        == 8
                    {
                        return Some(ChannelWidth::OneSixtyMhz);
                    } else {
                        return Some(ChannelWidth::EightyPlusEightyMhz);
                    }
                }
            }
        }

        // If VHT information is present, use it to determine channel width
        if let Some(vht_operation_information) = &self.vht_operation_information {
            return vht_operation_information.channel_width();
        }

        // 6 GHz and VHT Information is not present so we can't determine
        // ChannelWidth from this element
        None
    }

    pub fn summary(&self) -> String {
        let max_spatial_streams = self.basic_he_mcs_and_nss_set.max_spatial_streams();
        if max_spatial_streams == 1 {
            format!(
                "1 Spatial Stream, BSS Color: {}",
                self.bss_color_information.bss_color
            )
        } else {
            format!(
                "{} Spatial Streams, BSS Color: {}",
                max_spatial_streams, self.bss_color_information.bss_color
            )
        }
    }

    pub fn fields(&self) -> Vec<Field> {
        let mut fields = vec![
            self.he_operation_parameters.to_field(),
            self.bss_color_information.to_field(),
            self.basic_he_mcs_and_nss_set
                .to_field("Basic HE-MCS and NSS Set"),
        ];

        if let Some(vht_operation_information) = self.vht_operation_information {
            fields.push(vht_operation_information.to_field());
        }

        if let Some(max_cohosted_bssid_indicator) = self.max_cohosted_bssid_indicator {
            fields.push(
                Field::builder()
                    .title("Max Co-Hosted BSSID Indicator")
                    .value(max_cohosted_bssid_indicator)
                    .byte(max_cohosted_bssid_indicator)
                    .build(),
            );
        }

        if let Some(six_ghz_operation_information) = self.six_ghz_operation_information {
            fields.push(six_ghz_operation_information.to_field());
        }

        fields
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct HeOperationParameters {
    #[deku(bits = 3)]
    pub default_pe_duration: u8,
    #[deku(bits = 1)]
    pub twt_required: bool,
    #[deku(bits = 10)]
    pub txop_duration_rts_threshold: u16,
    #[deku(bits = 1)]
    pub vht_operation_information_present: bool,
    #[deku(bits = 1)]
    pub cohosted_bss: bool,
    #[deku(bits = 1)]
    pub er_su_disable: bool,
    #[deku(bits = 1)]
    pub six_ghz_operation_information_present: bool,
    #[deku(bits = 6)]
    reserved: u8,
}

impl HeOperationParameters {
    pub fn to_field(&self) -> Field {
        let bytes = self.to_bytes().unwrap_or_default();
        Field::builder()
            .title("HE Operation Parameters")
            .value("")
            .bytes(bytes.clone())
            .subfields([
                Field::builder()
                    .title("Default PE Duration")
                    .value(self.default_pe_duration)
                    .bits(BitRange::new(&bytes, 0, 3))
                    .build(),
                Field::builder()
                    .title("TWT Required")
                    .value(self.twt_required)
                    .bits(BitRange::new(&bytes, 3, 1))
                    .build(),
                Field::builder()
                    .title("TXOP Duration RTS Threshold")
                    .value(self.txop_duration_rts_threshold)
                    .bits(BitRange::new(&bytes, 4, 10))
                    .build(),
                Field::builder()
                    .title("VHT Operation Information Present")
                    .value(self.vht_operation_information_present)
                    .bits(BitRange::new(&bytes, 14, 1))
                    .build(),
                Field::builder()
                    .title("Co-Hosted BSS")
                    .value(self.cohosted_bss)
                    .bits(BitRange::new(&bytes, 15, 1))
                    .build(),
                Field::builder()
                    .title("ER SU Disable")
                    .value(self.er_su_disable)
                    .bits(BitRange::new(&bytes, 16, 1))
                    .build(),
                Field::builder()
                    .title("6 GHz Operation Information Present")
                    .value(self.six_ghz_operation_information_present)
                    .bits(BitRange::new(&bytes, 17, 1))
                    .build(),
                Field::reserved(BitRange::new(&bytes, 18, 6)),
            ])
            .build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct BssColorInformation {
    #[deku(bits = 6)]
    pub bss_color: u8,
    #[deku(bits = 1)]
    pub partial_bss_color: bool,
    #[deku(bits = 1)]
    pub bss_color_disabled: bool,
}

impl BssColorInformation {
    pub fn to_field(&self) -> Field {
        let byte = self
            .to_bytes()
            .unwrap_or_default()
            .first()
            .cloned()
            .unwrap_or_default();
        Field::builder()
            .title("BSS Color Information")
            .value("")
            .byte(byte)
            .subfields([
                Field::builder()
                    .title("BSS Color")
                    .value(self.bss_color)
                    .bits(BitRange::new(&[byte], 0, 6))
                    .build(),
                Field::builder()
                    .title("Partial BSS Color")
                    .value(self.partial_bss_color)
                    .bits(BitRange::new(&[byte], 6, 1))
                    .build(),
                Field::builder()
                    .title("BSS Color Disabled")
                    .value(self.bss_color_disabled)
                    .bits(BitRange::new(&[byte], 7, 1))
                    .build(),
            ])
            .build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct SixGhzOperationInformation {
    #[deku(bytes = 1)]
    pub primary_channel: u8,
    pub control: Control,
    #[deku(bytes = 1)]
    pub channel_center_frequency_segment_0: u8,
    #[deku(bytes = 1)]
    pub channel_center_frequency_segment_1: u8,
    #[deku(bytes = 1)]
    pub minimum_rate: u8,
}

impl SixGhzOperationInformation {
    pub fn to_field(&self) -> Field {
        Field::builder()
            .title("6 GHz Operation Information")
            .value("")
            .bytes(self.to_bytes().unwrap_or_default())
            .subfields([
                Field::builder()
                    .title("Primary Channel")
                    .value(self.primary_channel)
                    .byte(self.primary_channel)
                    .build(),
                self.control.to_field(),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct Control {
    #[deku(
        bits = 2,
        map = "|value: u8| SixGhzHeChannelWidth::try_from(value).map_err(|_| DekuError::Parse(\"Invalid SixGhzChannelWidth\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*channel_width), 2)"
    )]
    pub channel_width: SixGhzHeChannelWidth,
    #[deku(bits = 1)]
    pub duplicate_beacon: bool,
    #[deku(bits = 3)]
    pub regulatory_info: u8,
    #[deku(bits = 2)]
    reserved: u8,
}

impl Control {
    pub fn to_field(&self) -> Field {
        Field::builder()
            .title("Control")
            .value("")
            .byte(
                self.to_bytes()
                    .ok()
                    .and_then(|bytes| bytes.get(0).cloned())
                    .unwrap_or_default(),
            )
            .subfields([
                Field::builder()
                    .title("Channel Width")
                    .value(self.channel_width)
                    .units("MHz")
                    .build(),
                Field::builder()
                    .title("Duplicate Beacon")
                    .value(self.duplicate_beacon)
                    .build(),
                Field::builder()
                    .title("Regulatory Info")
                    .value(self.regulatory_info)
                    .build(),
            ])
            .build()
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum SixGhzHeChannelWidth {
    TwentyMhz = 0,
    FortyMhz = 1,
    EightyMhz = 2,
    EightyPlusEightyOrOneHundredSixtyMhz = 3,
}

impl Display for SixGhzHeChannelWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TwentyMhz => write!(f, "20"),
            Self::FortyMhz => write!(f, "40"),
            Self::EightyMhz => write!(f, "80"),
            Self::EightyPlusEightyOrOneHundredSixtyMhz => write!(f, "80+80/160"),
        }
    }
}
