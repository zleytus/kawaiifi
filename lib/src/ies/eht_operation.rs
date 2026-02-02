use std::fmt::Display;

use deku::{DekuContainerWrite, DekuError, DekuRead, DekuWrite};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

use super::{IeId, write_bits_lsb0};
use crate::{BitRange, ChannelWidth, Field};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct EhtOperation {
    pub eht_operation_parameters: EhtOperationParameters,
    pub basic_eht_mcs_and_nss_set: BasicEhtMcsAndNssSet,
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

    pub fn summary(&self) -> String {
        let max_spatial_streams = self.basic_eht_mcs_and_nss_set.max_spatial_streams();
        if max_spatial_streams == 1 {
            "1 spatial stream".to_string()
        } else {
            format!("{} spatial streams", max_spatial_streams)
        }
    }

    pub fn fields(&self) -> Vec<Field> {
        let mut fields = vec![
            self.eht_operation_parameters.to_field(),
            self.basic_eht_mcs_and_nss_set.to_field(),
        ];

        if let Some(eht_operation_information) = self.eht_operation_information {
            fields.push(eht_operation_information.to_field());
        }

        fields
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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

impl EhtOperationParameters {
    pub fn to_field(&self) -> Field {
        let byte = self
            .to_bytes()
            .unwrap_or_default()
            .first()
            .cloned()
            .unwrap_or_default();
        Field::builder()
            .title("EHT Operation Parameters")
            .value("")
            .subfields([
                Field::builder()
                    .title("EHT Operation Information Present")
                    .value(self.eht_operation_information_present)
                    .bits(BitRange::from_byte(byte, 0, 1))
                    .build(),
                Field::builder()
                    .title("Disabled Subchannel Bitmap Present")
                    .value(self.disabled_subchannel_bitmap_present)
                    .bits(BitRange::from_byte(byte, 1, 1))
                    .build(),
                Field::builder()
                    .title("EHT Default PE Duration")
                    .value(self.eht_default_pe_duration)
                    .bits(BitRange::from_byte(byte, 2, 1))
                    .build(),
                Field::builder()
                    .title("Group Addressed BU Indication Limit")
                    .value(self.group_addressed_bu_indication_limit)
                    .bits(BitRange::from_byte(byte, 3, 1))
                    .build(),
                Field::builder()
                    .title("Group Addressed BU Indication Exponent")
                    .value(self.group_addressed_bu_indication_exponent)
                    .bits(BitRange::from_byte(byte, 4, 2))
                    .build(),
                Field::builder()
                    .title("MCS15 Disable")
                    .value(self.mcs15_disable)
                    .bits(BitRange::from_byte(byte, 6, 1))
                    .build(),
                Field::reserved(BitRange::from_byte(byte, 7, 1)),
            ])
            .byte(byte)
            .build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct BasicEhtMcsAndNssSet {
    #[deku(bits = 4)]
    pub rx_max_nss_that_supports_eht_mcs_zero_through_seven: u8,
    #[deku(bits = 4)]
    pub tx_max_nss_that_supports_eht_mcs_zero_through_seven: u8,
    #[deku(bits = 4)]
    pub rx_max_nss_that_supports_eht_mcs_eight_through_nine: u8,
    #[deku(bits = 4)]
    pub tx_max_nss_that_supports_eht_mcs_eight_through_nine: u8,
    #[deku(bits = 4)]
    pub rx_max_nss_that_supports_eht_mcs_ten_through_eleven: u8,
    #[deku(bits = 4)]
    pub tx_max_nss_that_supports_eht_mcs_ten_through_eleven: u8,
    #[deku(bits = 4)]
    pub rx_max_nss_that_supports_eht_mcs_twelve_through_thirteen: u8,
    #[deku(bits = 4)]
    pub tx_max_nss_that_supports_eht_mcs_twelve_through_thirteen: u8,
}

impl BasicEhtMcsAndNssSet {
    pub fn max_spatial_streams(&self) -> u8 {
        self.rx_max_nss_that_supports_eht_mcs_zero_through_seven
            .max(self.rx_max_nss_that_supports_eht_mcs_eight_through_nine)
            .max(self.rx_max_nss_that_supports_eht_mcs_ten_through_eleven)
            .max(self.rx_max_nss_that_supports_eht_mcs_twelve_through_thirteen)
            .max(1)
    }

    pub fn to_field(&self) -> Field {
        let bytes = self.to_bytes().unwrap_or_default();

        fn spatial_streams_value(nss: u8) -> String {
            if nss == 0 {
                "Not Supported".to_string()
            } else {
                nss.to_string()
            }
        }

        fn spatial_streams_units(nss: u8) -> &'static str {
            match nss {
                1 => "spatial stream",
                2..=u8::MAX => "spatial streams",
                _ => "",
            }
        }

        Field::builder()
            .title("Basic EHT-MCS and NSS Set")
            .value("")
            .subfields([
                Field::builder()
                    .title("Rx Max NSS That Supports EHT-MCS 0-7")
                    .value(spatial_streams_value(
                        self.rx_max_nss_that_supports_eht_mcs_zero_through_seven,
                    ))
                    .bits(BitRange::new(&bytes, 0, 4))
                    .units(spatial_streams_units(
                        self.rx_max_nss_that_supports_eht_mcs_zero_through_seven,
                    ))
                    .build(),
                Field::builder()
                    .title("Tx Max NSS That Supports EHT-MCS 0-7")
                    .value(spatial_streams_value(
                        self.tx_max_nss_that_supports_eht_mcs_zero_through_seven,
                    ))
                    .bits(BitRange::new(&bytes, 4, 4))
                    .units(spatial_streams_units(
                        self.tx_max_nss_that_supports_eht_mcs_zero_through_seven,
                    ))
                    .build(),
                Field::builder()
                    .title("Rx Max NSS That Supports EHT-MCS 8-9")
                    .value(spatial_streams_value(
                        self.rx_max_nss_that_supports_eht_mcs_eight_through_nine,
                    ))
                    .bits(BitRange::new(&bytes, 8, 4))
                    .units(spatial_streams_units(
                        self.rx_max_nss_that_supports_eht_mcs_eight_through_nine,
                    ))
                    .build(),
                Field::builder()
                    .title("Tx Max NSS That Supports EHT-MCS 8-9")
                    .value(spatial_streams_value(
                        self.tx_max_nss_that_supports_eht_mcs_eight_through_nine,
                    ))
                    .bits(BitRange::new(&bytes, 12, 4))
                    .units(spatial_streams_units(
                        self.tx_max_nss_that_supports_eht_mcs_eight_through_nine,
                    ))
                    .build(),
                Field::builder()
                    .title("Rx Max NSS That Supports EHT-MCS 10-11")
                    .value(spatial_streams_value(
                        self.rx_max_nss_that_supports_eht_mcs_ten_through_eleven,
                    ))
                    .bits(BitRange::new(&bytes, 16, 4))
                    .units(spatial_streams_units(
                        self.rx_max_nss_that_supports_eht_mcs_ten_through_eleven,
                    ))
                    .build(),
                Field::builder()
                    .title("Tx Max NSS That Supports EHT-MCS 10-11")
                    .value(spatial_streams_value(
                        self.tx_max_nss_that_supports_eht_mcs_ten_through_eleven,
                    ))
                    .bits(BitRange::new(&bytes, 20, 4))
                    .units(spatial_streams_units(
                        self.tx_max_nss_that_supports_eht_mcs_ten_through_eleven,
                    ))
                    .build(),
                Field::builder()
                    .title("Rx Max NSS That Supports EHT-MCS 12-13")
                    .value(spatial_streams_value(
                        self.rx_max_nss_that_supports_eht_mcs_twelve_through_thirteen,
                    ))
                    .bits(BitRange::new(&bytes, 24, 4))
                    .units(spatial_streams_units(
                        self.rx_max_nss_that_supports_eht_mcs_twelve_through_thirteen,
                    ))
                    .build(),
                Field::builder()
                    .title("Tx Max NSS That Supports EHT-MCS 12-13")
                    .value(spatial_streams_value(
                        self.tx_max_nss_that_supports_eht_mcs_twelve_through_thirteen,
                    ))
                    .bits(BitRange::new(&bytes, 28, 4))
                    .units(spatial_streams_units(
                        self.tx_max_nss_that_supports_eht_mcs_twelve_through_thirteen,
                    ))
                    .build(),
            ])
            .bytes(bytes)
            .build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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

impl EhtOperationInformation {
    pub fn to_field(&self) -> Field {
        let mut subfields = vec![
            self.control.to_field(),
            Field::builder()
                .title("CCFS0")
                .value(self.ccfs0)
                .byte(self.ccfs0)
                .build(),
            Field::builder()
                .title("CCFS1")
                .value(self.ccfs1)
                .byte(self.ccfs1)
                .build(),
        ];

        if let Some(disabled_subchannel_bitmap) = self.disabled_subchannel_bitmap {
            let bytes = disabled_subchannel_bitmap.to_le_bytes();
            subfields.push(
                Field::builder()
                    .title("Disabled Subchannel Bitmap")
                    .value(disabled_subchannel_bitmap)
                    .bits(BitRange::new(&bytes, 0, 16))
                    .build(),
            )
        }

        Field::builder()
            .title("EHT Operation Information")
            .value("")
            .subfields(subfields)
            .build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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

impl Control {
    pub fn to_field(&self) -> Field {
        let byte = self
            .to_bytes()
            .unwrap_or_default()
            .first()
            .cloned()
            .unwrap_or_default();
        Field::builder()
            .title("Control")
            .value("")
            .subfields([
                Field::builder()
                    .title("Channel Width")
                    .value(self.channel_width)
                    .units("MHz")
                    .bits(BitRange::from_byte(byte, 0, 3))
                    .build(),
                Field::reserved(BitRange::from_byte(byte, 3, 5)),
            ])
            .byte(byte)
            .build()
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum EhtChannelWidth {
    TwentyMhz,
    FortyMhz,
    EightyMhz,
    OneHundredSixtyMhz,
    ThreeHundredTwentyMhz,
}

impl Display for EhtChannelWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TwentyMhz => write!(f, "20"),
            Self::FortyMhz => write!(f, "40"),
            Self::EightyMhz => write!(f, "80"),
            Self::OneHundredSixtyMhz => write!(f, "160"),
            Self::ThreeHundredTwentyMhz => write!(f, "320"),
        }
    }
}
