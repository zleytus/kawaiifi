use std::fmt::Display;

use deku::{DekuContainerWrite, DekuError, DekuRead, DekuWrite};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

use super::{IeId, write_bits_lsb0};
use crate::{BitRange, ChannelWidth, Field};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct HtCapabilities {
    pub ht_capability_information: HtCapabilityInformation,
    pub ampdu_parameters: AmpduParameters,
    pub supported_mcs_set: SupportedMcsSet,
    pub ht_extended_capabilities: HtExtendedCapabilities,
    pub transmit_beamforming_capabilities: TransmitBeamformingCapabilities,
    pub asel_capabilities: AselCapabilities,
}

impl HtCapabilities {
    pub const NAME: &'static str = "HT Capabilities";
    pub const ID: u8 = 45;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const MIN_LENGTH: usize = 26;

    /// Calculate HT (802.11n) data rate in Mbps
    pub fn max_rate(&self, channel_width: ChannelWidth) -> f64 {
        let data_subcarriers = match channel_width {
            ChannelWidth::FortyMhz => 108.0,
            _ => 52.0,
        };

        let short_gi = self.supports_short_gi_for_width(channel_width);
        let symbol_duration_us = if short_gi { 3.6 } else { 4.0 };

        // Bits per symbol depends on MCS (modulation + coding rate)
        let bits_per_symbol = match self.max_mcs() {
            0 => 0.5, // BPSK 1/2
            1 => 1.0, // QPSK 1/2
            2 => 1.5, // QPSK 3/4
            3 => 2.0, // 16-QAM 1/2
            4 => 3.0, // 16-QAM 3/4
            5 => 4.0, // 64-QAM 2/3
            6 => 4.5, // 64-QAM 3/4
            7 => 5.0, // 64-QAM 5/6
            _ => return 0.0,
        };

        // Calculate data rate
        // Rate = (subcarriers × bits/symbol × streams) / symbol_time
        (data_subcarriers
            * bits_per_symbol
            * f64::from(self.supported_mcs_set.max_spatial_streams()))
            / symbol_duration_us
    }

    pub(crate) fn max_mcs(&self) -> u8 {
        let max_streams = self.supported_mcs_set.max_spatial_streams();
        self.supported_mcs_set
            .max_mcs_for_stream(max_streams)
            .unwrap_or(0)
    }

    pub(crate) fn supports_short_gi_for_width(&self, width: ChannelWidth) -> bool {
        match width {
            ChannelWidth::TwentyMhz => self.ht_capability_information.short_gi_for_twenty_mhz,
            ChannelWidth::FortyMhz => self.ht_capability_information.short_gi_for_forty_mhz,
            _ => false,
        }
    }

    pub fn summary(&self) -> String {
        let max_spatial_streams = self.supported_mcs_set.max_spatial_streams();
        if max_spatial_streams == 1 {
            format!(
                "{} MHz, 1 Spatial Stream",
                self.ht_capability_information.supported_channel_width_set
            )
        } else {
            format!(
                "{} MHz, {} Spatial Streams",
                self.ht_capability_information.supported_channel_width_set, max_spatial_streams
            )
        }
    }

    pub fn fields(&self) -> Vec<Field> {
        vec![
            self.ht_capability_information.to_field(),
            self.ampdu_parameters.to_field(),
            self.supported_mcs_set.to_field("Supported MCS Set"),
            self.ht_extended_capabilities.to_field(),
            self.transmit_beamforming_capabilities.to_field(),
            self.asel_capabilities.to_field(),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct HtCapabilityInformation {
    #[deku(bits = 1)]
    pub ldpc_coding_capability: bool,
    #[deku(
        bits = 1,
        map = "|value: u8| SupportedChannelWidthSet::try_from(value).map_err(|_| DekuError::Parse(\"Invalid SupportedChannelWidthSet\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*supported_channel_width_set), 1)"
    )]
    pub supported_channel_width_set: SupportedChannelWidthSet,
    #[deku(
        bits = 2,
        map = "|value: u8| SmPowerSave::try_from(value).map_err(|_| deku::DekuError::Parse(\"Invalid SmPowerSave\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*sm_power_save), 2)"
    )]
    pub sm_power_save: SmPowerSave,
    #[deku(bits = 1)]
    pub ht_greenfield: bool,
    #[deku(bits = 1)]
    pub short_gi_for_twenty_mhz: bool,
    #[deku(bits = 1)]
    pub short_gi_for_forty_mhz: bool,
    #[deku(bits = 1)]
    pub tx_stbc: bool,
    #[deku(
        bits = 2,
        map = "|value: u8| RxStbc::try_from(value).map_err(|_| deku::DekuError::Parse(\"Invalid RxStbc\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*rx_stbc), 2)"
    )]
    pub rx_stbc: RxStbc,
    #[deku(bits = 1)]
    reserved_1: bool,
    #[deku(
        bits = 1,
        map = "|value: bool| -> Result<u16, DekuError> { if value { Ok(7935u16) } else { Ok(3839u16) } }",
        writer = "write_bits_lsb0(deku::writer, (*maximum_amsdu_length == 7935u16) as u8, 1)"
    )]
    pub maximum_amsdu_length: u16,
    #[deku(bits = 1)]
    pub dsss_cck_mode_in_forty_mhz: bool,
    #[deku(bits = 1)]
    reserved_2: bool,
    #[deku(bits = 1)]
    pub forty_mhz_intolerant: bool,
    #[deku(bits = 1)]
    reserved_3: bool,
}

impl HtCapabilityInformation {
    pub fn to_field(&self) -> Field {
        let bytes = self.to_bytes().unwrap_or_default();

        Field::builder()
            .title("HT Capability Information")
            .value("")
            .subfields([
                Field::builder()
                    .title("LDPC Coding Capability")
                    .value(self.ldpc_coding_capability)
                    .bits(BitRange::new(&bytes, 0, 1))
                    .build(),
                Field::builder()
                    .title("Supported Channel Width Set")
                    .value(self.supported_channel_width_set)
                    .units("MHz")
                    .bits(BitRange::new(&bytes, 1, 1))
                    .build(),
                Field::builder()
                    .title("SM Power Save")
                    .value(self.sm_power_save)
                    .bits(BitRange::new(&bytes, 2, 2))
                    .build(),
                Field::builder()
                    .title("HT-Greenfield")
                    .value(self.ht_greenfield)
                    .bits(BitRange::new(&bytes, 4, 1))
                    .build(),
                Field::builder()
                    .title("Short GI for 20 MHz")
                    .value(self.short_gi_for_twenty_mhz)
                    .bits(BitRange::new(&bytes, 5, 1))
                    .build(),
                Field::builder()
                    .title("Short GI for 40 MHz")
                    .value(self.short_gi_for_forty_mhz)
                    .bits(BitRange::new(&bytes, 6, 1))
                    .build(),
                Field::builder()
                    .title("Tx STBC")
                    .value(self.tx_stbc)
                    .bits(BitRange::new(&bytes, 7, 1))
                    .build(),
                Field::builder()
                    .title("Rx STBC")
                    .value(self.rx_stbc)
                    .bits(BitRange::new(&bytes, 8, 2))
                    .build(),
                Field::reserved(BitRange::new(&bytes, 10, 1)),
                Field::builder()
                    .title("Maximum A-MSDU Length")
                    .value(self.maximum_amsdu_length)
                    .units("bytes")
                    .bits(BitRange::new(&bytes, 11, 1))
                    .build(),
                Field::builder()
                    .title("DSSS/CCK Mode in 40 MHz")
                    .value(self.dsss_cck_mode_in_forty_mhz)
                    .bits(BitRange::new(&bytes, 12, 1))
                    .build(),
                Field::reserved(BitRange::new(&bytes, 13, 1)),
                Field::builder()
                    .title("40 MHz Intolerant")
                    .value(self.forty_mhz_intolerant)
                    .bits(BitRange::new(&bytes, 14, 1))
                    .build(),
                Field::reserved(BitRange::new(&bytes, 15, 1)),
            ])
            .bytes(bytes.clone())
            .build()
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum SupportedChannelWidthSet {
    TwentyMhz = 0,
    TwentyOrFortyMhz = 1,
}

impl Display for SupportedChannelWidthSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TwentyMhz => write!(f, "20"),
            Self::TwentyOrFortyMhz => write!(f, "20/40"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct AmpduParameters {
    #[deku(bits = 2)]
    pub maximum_ampdu_length_exponent: u8,
    #[deku(
        bits = 3,
        map = "|value: u8| MpduStartSpacing::try_from(value).map_err(|_| deku::DekuError::Parse(\"Invalid MpduStartSpacing\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*minimum_mpdu_start_spacing), 3)"
    )]
    pub minimum_mpdu_start_spacing: MpduStartSpacing,
    #[deku(bits = 3)]
    reserved: u8,
}

impl AmpduParameters {
    pub fn max_ampdu_length(&self) -> u32 {
        2u32.pow(13 + u32::from(self.maximum_ampdu_length_exponent)) - 1
    }

    pub fn to_field(&self) -> Field {
        let byte = self
            .to_bytes()
            .unwrap_or_default()
            .first()
            .cloned()
            .unwrap_or_default();
        Field::builder()
            .title("A-MPDU Parameters")
            .value("")
            .subfields([
                Field::builder()
                    .title("Maximum A-MPDU Length Exponent")
                    .value(self.maximum_ampdu_length_exponent)
                    .bits(BitRange::from_byte(byte, 0, 2))
                    .build(),
                Field::builder()
                    .title("Minimum MPDU Start Spacing")
                    .value(self.minimum_mpdu_start_spacing)
                    .bits(BitRange::from_byte(byte, 2, 3))
                    .build(),
                Field::reserved(BitRange::from_byte(byte, 5, 3)),
            ])
            .byte(byte)
            .build()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct SupportedMcsSet {
    #[deku(bits = 77)]
    pub rx_mcs_bitmask: u128,
    #[deku(bits = 3)]
    reserved_1: u8,
    #[deku(bits = 10)]
    pub rx_highest_supported_data_rate: u16,
    #[deku(bits = 6)]
    reserved_2: u8,
    #[deku(bits = 1)]
    pub tx_mcs_set_defined: bool,
    #[deku(bits = 1)]
    pub tx_rx_mcs_set_not_equal: bool,
    #[deku(bits = 2)]
    pub tx_maximum_number_spatial_streams_supported: u8,
    #[deku(bits = 1)]
    pub tx_unequal_modulation_supported: bool,
    #[deku(bits = 27)]
    reserved_3: u32,
}

impl SupportedMcsSet {
    /// Check if a specific MCS index is supported.
    pub fn is_mcs_supported(&self, mcs: u8) -> bool {
        if mcs > 76 {
            return false;
        }
        (self.rx_mcs_bitmask & (1u128 << mcs)) != 0
    }

    /// Returns the maximum number of spatial streams supported.
    /// MCS 0-7: 1 stream, 8-15: 2 streams, 16-23: 3 streams, 24-31: 4 streams
    pub fn max_spatial_streams(&self) -> u8 {
        for stream in (1..=4u8).rev() {
            let start_mcs = (stream - 1) * 8;
            for mcs in start_mcs..(start_mcs + 8) {
                if self.is_mcs_supported(mcs) {
                    return stream;
                }
            }
        }
        1 // At least 1 stream
    }

    /// Returns the maximum MCS index (0-7) supported for the given spatial stream.
    pub fn max_mcs_for_stream(&self, stream: u8) -> Option<u8> {
        if stream == 0 || stream > 4 {
            return None;
        }
        let base_mcs = (stream - 1) * 8;
        for mcs_index in (0..8).rev() {
            if self.is_mcs_supported(base_mcs + mcs_index) {
                return Some(mcs_index);
            }
        }
        None
    }

    pub fn to_field(&self, title: &str) -> Field {
        let bytes = self.to_bytes().unwrap_or_default();
        let bitmask_le_bytes = self.rx_mcs_bitmask.to_le_bytes();
        Field::builder()
            .title(title)
            .value("")
            .subfields([
                Field::builder()
                    .title("Rx MCS Set")
                    .value("")
                    .subfields([
                        Self::spatial_stream_field(1, bitmask_le_bytes[0]),
                        Self::spatial_stream_field(2, bitmask_le_bytes[1]),
                        Self::spatial_stream_field(3, bitmask_le_bytes[2]),
                        Self::spatial_stream_field(4, bitmask_le_bytes[3]),
                        Field::builder()
                            .title("Rx MCS Index 32")
                            .value(if self.is_mcs_supported(32) {
                                "Supported"
                            } else {
                                "Not Supported"
                            })
                            .bits(BitRange::new(&bitmask_le_bytes[4..10], 0, 1))
                            .build(),
                        Field::builder()
                            .title("Rx MCS Indices 33-76")
                            .value("")
                            .bits(BitRange::new(&bitmask_le_bytes[4..10], 1, 43))
                            .build(),
                    ])
                    .bytes(bitmask_le_bytes.to_vec())
                    .build(),
                Field::builder()
                    .title("Rx Highest Supported Data Rate")
                    .value(self.rx_highest_supported_data_rate)
                    .bits(BitRange::new(&bytes[10..12], 0, 10))
                    .units("Mb/s")
                    .build(),
                Field::reserved(BitRange::new(&bytes[11..], 2, 6)),
                Field::builder()
                    .title("Tx MCS Set Defined")
                    .value(self.tx_mcs_set_defined)
                    .bits(BitRange::new(&bytes[11..], 8, 1))
                    .build(),
                Field::builder()
                    .title("Tx Rx MCS Set Not Equal")
                    .value(self.tx_rx_mcs_set_not_equal)
                    .bits(BitRange::new(&bytes[11..], 9, 1))
                    .build(),
                Field::builder()
                    .title("Tx Maximum Number Spatial Streams Supported")
                    .value(self.tx_maximum_number_spatial_streams_supported + 1)
                    .units(format!(
                        "({})",
                        self.tx_maximum_number_spatial_streams_supported
                    ))
                    .bits(BitRange::new(&bytes[11..], 10, 2))
                    .build(),
                Field::builder()
                    .title("Tx Unequal Modulation Supported")
                    .value(self.tx_unequal_modulation_supported)
                    .bits(BitRange::new(&bytes[11..], 12, 1))
                    .build(),
                Field::reserved(BitRange::new(&bytes[11..], 13, 27)),
            ])
            .bytes(bytes)
            .build()
    }

    /// Creates a field for a spatial stream's MCS indices.
    /// `stream` is 1-4, `byte` contains the 8 MCS index bits for that stream.
    fn spatial_stream_field(stream: u8, byte: u8) -> Field {
        let title = if stream == 1 {
            "1 Spatial Stream".to_string()
        } else {
            format!("{} Spatial Streams", stream)
        };
        let base_mcs = (stream - 1) * 8;

        Field::builder()
            .title(title)
            .value("")
            .byte(byte)
            .subfields(
                (0u8..8)
                    .map(|i| {
                        let supported = (byte >> i) & 1 == 1;
                        Field::builder()
                            .title(format!("MCS Index {}", base_mcs + i))
                            .value(if supported {
                                "Supported"
                            } else {
                                "Not Supported"
                            })
                            .units(match i {
                                0 => "(BPSK 1/2)",
                                1 => "(QPSK 1/2)",
                                2 => "(QPSK 3/4)",
                                3 => "(16-QAM 1/2)",
                                4 => "(16-QAM 3/4)",
                                5 => "(64-QAM 2/3)",
                                6 => "(64-QAM 3/4)",
                                7 => "(64-QAM 5/6)",
                                _ => "",
                            })
                            .bits(BitRange::from_byte(byte, usize::from(i), 1))
                            .build()
                    })
                    .collect::<Vec<_>>(),
            )
            .build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct HtExtendedCapabilities {
    #[deku(bits = 8)]
    reserved_1: u8,
    #[deku(
        bits = 2,
        map = "|value: u8| McsFeedback::try_from(value).map_err(|_| deku::DekuError::Parse(\"Invalid McsFeedback\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*mcs_feedback), 2)"
    )]
    pub mcs_feedback: McsFeedback,
    #[deku(bits = 1)]
    pub htc_ht_support: bool,
    #[deku(bits = 1)]
    pub rd_responder: bool,
    #[deku(bits = 4)]
    reserved_2: u8,
}

impl HtExtendedCapabilities {
    pub fn to_field(&self) -> Field {
        let bytes = self.to_bytes().unwrap_or_default();
        Field::builder()
            .title("HT Extended Capabilities")
            .value("")
            .subfields([
                Field::reserved(BitRange::new(&bytes, 0, 8)),
                Field::builder()
                    .title("MCS Feedback")
                    .value(self.mcs_feedback)
                    .bits(BitRange::new(&bytes, 8, 2))
                    .build(),
                Field::builder()
                    .title("+HTC-HT Support")
                    .value(self.htc_ht_support)
                    .bits(BitRange::new(&bytes, 10, 1))
                    .build(),
                Field::builder()
                    .title("RD Responder")
                    .value(self.rd_responder)
                    .bits(BitRange::new(&bytes, 11, 1))
                    .build(),
                Field::reserved(BitRange::new(&bytes, 12, 4)),
            ])
            .bytes(bytes)
            .build()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct TransmitBeamformingCapabilities {
    #[deku(bits = 1)]
    pub implicit_transmit_beamforming_receiving_capable: bool,
    #[deku(bits = 1)]
    pub receive_staggered_sounding_capable: bool,
    #[deku(bits = 1)]
    pub transmit_staggered_sounding_capable: bool,
    #[deku(bits = 1)]
    pub receive_ndp_capable: bool,
    #[deku(bits = 1)]
    pub transmit_ndp_capable: bool,
    #[deku(bits = 1)]
    pub implicit_transmit_beamforming_capable: bool,
    #[deku(
        bits = 2,
        map = "|value: u8| Calibration::try_from(value).map_err(|_| deku::DekuError::Parse(\"Invalid Calibration\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*calibration), 2)"
    )]
    pub calibration: Calibration,
    #[deku(bits = 1)]
    pub explicit_csi_transmit_beamforming_capable: bool,
    #[deku(bits = 1)]
    pub explicit_noncompressed_steering_capable: bool,
    #[deku(bits = 1)]
    pub explicit_compressed_steering_capable: bool,
    #[deku(
        bits = 2,
        map = "|value: u8| BeamformingFeedback::try_from(value).map_err(|_| deku::DekuError::Parse(\"Invalid BeamformingFeedback\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*explicit_transmit_beamforming_csi_feedback), 2)"
    )]
    pub explicit_transmit_beamforming_csi_feedback: BeamformingFeedback,
    #[deku(
        bits = 2,
        map = "|value: u8| BeamformingFeedback::try_from(value).map_err(|_| deku::DekuError::Parse(\"Invalid BeamformingFeedback\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*explicit_noncompressed_beamforming_feedback_capable), 2)"
    )]
    pub explicit_noncompressed_beamforming_feedback_capable: BeamformingFeedback,
    #[deku(
        bits = 2,
        map = "|value: u8| BeamformingFeedback::try_from(value).map_err(|_| deku::DekuError::Parse(\"Invalid BeamformingFeedback\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*explicit_compressed_beamforming_feedback_capable), 2)"
    )]
    pub explicit_compressed_beamforming_feedback_capable: BeamformingFeedback,
    #[deku(
        bits = 2,
        map = "|value: u8| MinimalGrouping::try_from(value).map_err(|_| deku::DekuError::Parse(\"Invalid MinimalGrouping\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*minimal_grouping), 2)"
    )]
    pub minimal_grouping: MinimalGrouping,
    #[deku(bits = 2)]
    pub csi_number_of_beamformer_antennas_supported: u8,
    #[deku(bits = 2)]
    pub noncompressed_steering_number_of_beamformer_antennas_supported: u8,
    #[deku(bits = 2)]
    pub compressed_steering_number_of_beamformer_antennas_supported: u8,
    #[deku(bits = 2)]
    pub csi_max_number_of_rows_beamformer_supported: u8,
    #[deku(bits = 2)]
    pub channel_estimation_capability: u8,
    #[deku(bits = 3)]
    reserved: u8,
}

impl TransmitBeamformingCapabilities {
    pub fn to_field(&self) -> Field {
        let bytes = self.to_bytes().unwrap_or_default();
        Field::builder()
            .title("Transmit Beamforming Capabilities")
            .value("")
            .subfields([
                Field::builder()
                    .title("Implicit Transmit Beamforming Receiving Capable")
                    .value(self.implicit_transmit_beamforming_receiving_capable)
                    .bits(BitRange::new(&bytes, 0, 1))
                    .build(),
                Field::builder()
                    .title("Receive Staggered Sounding Capable")
                    .value(self.receive_staggered_sounding_capable)
                    .bits(BitRange::new(&bytes, 1, 1))
                    .build(),
                Field::builder()
                    .title("Transmit Staggered Sounding Capable")
                    .value(self.transmit_staggered_sounding_capable)
                    .bits(BitRange::new(&bytes, 2, 1))
                    .build(),
                Field::builder()
                    .title("Receive NDP Capable")
                    .value(self.receive_ndp_capable)
                    .bits(BitRange::new(&bytes, 3, 1))
                    .build(),
                Field::builder()
                    .title("Transmit NDP Capable")
                    .value(self.transmit_ndp_capable)
                    .bits(BitRange::new(&bytes, 4, 1))
                    .build(),
                Field::builder()
                    .title("Implicit Transmit Beamforming Capable")
                    .value(self.implicit_transmit_beamforming_capable)
                    .bits(BitRange::new(&bytes, 5, 1))
                    .build(),
                Field::builder()
                    .title("Calibration")
                    .value(self.calibration)
                    .bits(BitRange::new(&bytes, 6, 2))
                    .build(),
                Field::builder()
                    .title("Explicit CSI Transmit Beamforming Capable")
                    .value(self.explicit_csi_transmit_beamforming_capable)
                    .bits(BitRange::new(&bytes, 8, 1))
                    .build(),
                Field::builder()
                    .title("Explicit Noncompressed Steering Capable")
                    .value(self.explicit_noncompressed_steering_capable)
                    .bits(BitRange::new(&bytes, 9, 1))
                    .build(),
                Field::builder()
                    .title("Explicit Compressed Steering Capable")
                    .value(self.explicit_compressed_steering_capable)
                    .bits(BitRange::new(&bytes, 10, 1))
                    .build(),
                Field::builder()
                    .title("Explicit Transmit Beamforming CSI Feedback")
                    .value(self.explicit_transmit_beamforming_csi_feedback)
                    .bits(BitRange::new(&bytes, 11, 2))
                    .build(),
                Field::builder()
                    .title("Explicit Noncompressed Beamforming Feedback Capable")
                    .value(self.explicit_noncompressed_beamforming_feedback_capable)
                    .bits(BitRange::new(&bytes, 13, 2))
                    .build(),
                Field::builder()
                    .title("Explicit Compressed Beamforming Feedback Capable")
                    .value(self.explicit_compressed_beamforming_feedback_capable)
                    .bits(BitRange::new(&bytes, 15, 2))
                    .build(),
                Field::builder()
                    .title("Minimal Grouping")
                    .value(self.minimal_grouping)
                    .bits(BitRange::new(&bytes, 17, 2))
                    .build(),
                Field::builder()
                    .title("CSI Number of Beamformer Antennas Supported")
                    .value(self.csi_number_of_beamformer_antennas_supported + 1)
                    .units("Tx antenna sounding")
                    .bits(BitRange::new(&bytes, 19, 2))
                    .build(),
                Field::builder()
                    .title("Noncompressed Steering Number of Beamformer Antennas Supported")
                    .value(self.noncompressed_steering_number_of_beamformer_antennas_supported + 1)
                    .units("Tx antenna sounding")
                    .bits(BitRange::new(&bytes, 21, 2))
                    .build(),
                Field::builder()
                    .title("Compressed Steering Number of Beamformer Antennas Supported")
                    .value(self.compressed_steering_number_of_beamformer_antennas_supported + 1)
                    .units("Tx antenna sounding")
                    .bits(BitRange::new(&bytes, 23, 2))
                    .build(),
                Field::builder()
                    .title("CSI Max Number of Rows Beamformer Supported")
                    .value(self.csi_max_number_of_rows_beamformer_supported + 1)
                    .units(if self.csi_max_number_of_rows_beamformer_supported == 0 {
                        "row of CSI"
                    } else {
                        "rows of CSI"
                    })
                    .bits(BitRange::new(&bytes, 25, 2))
                    .build(),
                Field::builder()
                    .title("Channel Estimation Capability")
                    .value(self.channel_estimation_capability + 1)
                    .units(if self.channel_estimation_capability == 0 {
                        "space-time stream"
                    } else {
                        "space-time streams"
                    })
                    .bits(BitRange::new(&bytes, 27, 2))
                    .build(),
                Field::reserved(BitRange::new(&bytes, 29, 3)),
            ])
            .bytes(bytes)
            .build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct AselCapabilities {
    #[deku(bits = 1)]
    pub antenna_selection_capable: bool,
    #[deku(bits = 1)]
    pub explicit_csi_feedback_based_transmit_asel_capable: bool,
    #[deku(bits = 1)]
    pub antenna_indices_feedback_based_transmit_asel_capable: bool,
    #[deku(bits = 1)]
    pub explicit_csi_feedback_capable: bool,
    #[deku(bits = 1)]
    pub antenna_indices_feedback_capable: bool,
    #[deku(bits = 1)]
    pub receive_asel_capable: bool,
    #[deku(bits = 1)]
    pub transmit_sounding_ppdus_capable: bool,
    #[deku(bits = 1)]
    reserved: bool,
}

impl AselCapabilities {
    pub fn to_field(&self) -> Field {
        let byte = self
            .to_bytes()
            .unwrap_or_default()
            .first()
            .cloned()
            .unwrap_or_default();

        Field::builder()
            .title("ASEL Capabilities")
            .value("")
            .subfields([
                Field::builder()
                    .title("Antenna Selection Capable")
                    .value(self.antenna_selection_capable)
                    .bits(BitRange::from_byte(byte, 0, 1))
                    .build(),
                Field::builder()
                    .title("Explicit CSI Feedback Based Transmit ASEL Capable")
                    .value(self.explicit_csi_feedback_based_transmit_asel_capable)
                    .bits(BitRange::from_byte(byte, 1, 1))
                    .build(),
                Field::builder()
                    .title("Antenna Indices Feedback Based Transmit ASEL Capable")
                    .value(self.antenna_indices_feedback_based_transmit_asel_capable)
                    .bits(BitRange::from_byte(byte, 2, 1))
                    .build(),
                Field::builder()
                    .title("Explicit CSI Feedback Capable")
                    .value(self.explicit_csi_feedback_capable)
                    .bits(BitRange::from_byte(byte, 3, 1))
                    .build(),
                Field::builder()
                    .title("Antenna Indices Feedback Capable")
                    .value(self.antenna_indices_feedback_capable)
                    .bits(BitRange::from_byte(byte, 4, 1))
                    .build(),
                Field::builder()
                    .title("Receive ASEL Capable")
                    .value(self.receive_asel_capable)
                    .bits(BitRange::from_byte(byte, 5, 1))
                    .build(),
                Field::builder()
                    .title("Transmit Sounding PPDUs Capable")
                    .value(self.transmit_sounding_ppdus_capable)
                    .bits(BitRange::from_byte(byte, 6, 1))
                    .build(),
                Field::reserved(BitRange::from_byte(byte, 7, 1)),
            ])
            .byte(byte)
            .build()
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum SmPowerSave {
    Static = 0,
    Dynamic = 1,
    None = 3,
}

impl Display for SmPowerSave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SmPowerSave::Static => write!(f, "Static"),
            SmPowerSave::Dynamic => write!(f, "Dynamic"),
            SmPowerSave::None => write!(f, "None"),
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum RxStbc {
    NotSupported = 0,
    OneSpatialStream,
    OneAndTwoSpatialStreams,
    OneTwoAndThreeSpatialStreams,
}

impl Display for RxStbc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RxStbc::NotSupported => write!(f, "Not Supported"),
            RxStbc::OneSpatialStream => write!(f, "One Spatial Stream"),
            RxStbc::OneAndTwoSpatialStreams => write!(f, "One and Two Spatial Streams"),
            RxStbc::OneTwoAndThreeSpatialStreams => {
                write!(f, "One, Two, and Three Spatial Streams")
            }
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum MpduStartSpacing {
    NoRestriction = 0,
    QuarterMicrosecond,
    HalfMicrosecond,
    OneMicrosecond,
    TwoMicroseconds,
    FourMicroseconds,
    EightMicroseconds,
    SixteenMicroseconds,
}

impl Display for MpduStartSpacing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MpduStartSpacing::NoRestriction => write!(f, "No Restriction"),
            MpduStartSpacing::QuarterMicrosecond => write!(f, "1/4 μs"),
            MpduStartSpacing::HalfMicrosecond => write!(f, "1/2 μs"),
            MpduStartSpacing::OneMicrosecond => write!(f, "1 μs"),
            MpduStartSpacing::TwoMicroseconds => write!(f, "2 μs"),
            MpduStartSpacing::FourMicroseconds => write!(f, "4 μs"),
            MpduStartSpacing::EightMicroseconds => write!(f, "8 μs"),
            MpduStartSpacing::SixteenMicroseconds => write!(f, "16 μs"),
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum McsFeedback {
    NoMfb = 0,
    UnsolicitedMfb = 2,
    ResponseOrUnsolicitedMfb = 3,
}

impl Display for McsFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            McsFeedback::NoMfb => write!(f, "No MFB"),
            McsFeedback::UnsolicitedMfb => write!(f, "Unsolicited MFB"),
            McsFeedback::ResponseOrUnsolicitedMfb => {
                write!(f, "Response (Delayed/Immediate) or Unsolicited MFB")
            }
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum Calibration {
    NotSupported = 0,
    Respond = 1,
    InitiateAndRespond = 3,
}

impl Display for Calibration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Calibration::NotSupported => write!(f, "Not Supported"),
            Calibration::Respond => write!(f, "Respond to Calibration Request"),
            Calibration::InitiateAndRespond => {
                write!(f, "Initiate and Respond to Calibration Request")
            }
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum BeamformingFeedback {
    NotSupported = 0,
    Delayed,
    Immediate,
    DelayedAndImmediate,
}

impl Display for BeamformingFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BeamformingFeedback::NotSupported => write!(f, "Not Supported"),
            BeamformingFeedback::Delayed => write!(f, "Delayed"),
            BeamformingFeedback::Immediate => write!(f, "Immediate"),
            BeamformingFeedback::DelayedAndImmediate => write!(f, "Delayed and Immediate"),
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum MinimalGrouping {
    One = 0,
    OneOrTwo,
    OneOrFour,
    OneOrTwoOrFour,
}

impl Display for MinimalGrouping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MinimalGrouping::One => write!(f, "1"),
            MinimalGrouping::OneOrTwo => write!(f, "1 or 2"),
            MinimalGrouping::OneOrFour => write!(f, "1 or 4"),
            MinimalGrouping::OneOrTwoOrFour => write!(f, "1, 2, or 4"),
        }
    }
}
