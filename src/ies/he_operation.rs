use deku::{DekuError, DekuRead, DekuWrite};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use super::{vht_operation::VhtOperationInformation, write_bits_lsb0, IeId};
use crate::ChannelWidth;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
pub struct HeOperation {
    pub he_operation_parameters: HeOperationParameters,
    pub bss_color_information: BssColorInformation,
    #[deku(bytes = 2)]
    pub basic_he_mcs_and_nss_set: [u8; 2],
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(bit_order = "lsb")]
pub struct BssColorInformation {
    #[deku(bits = 6)]
    pub bss_color: u8,
    #[deku(bits = 1)]
    pub partial_bss_color: bool,
    #[deku(bits = 1)]
    pub bss_color_disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum SixGhzHeChannelWidth {
    TwentyMhz = 0,
    FortyMhz = 1,
    EightyMhz = 2,
    EightyPlusEightyOrOneHundredSixtyMhz = 3,
}
