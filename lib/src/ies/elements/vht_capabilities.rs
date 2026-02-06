use std::fmt::Display;

use deku::{DekuContainerWrite, DekuError, DekuRead, DekuWrite};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

use super::HtCapabilities;
use crate::{
    BitRange, ChannelWidth, Field,
    ies::{IeId, write_bits_lsb0},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct VhtCapabilities {
    pub vht_capabilities_info: VhtCapabilitiesInfo,
    pub supported_vht_mcs_and_nss_set: SupportedVhtMcsAndNssSet,
}

impl VhtCapabilities {
    pub const NAME: &'static str = "VHT Capabilities";
    pub const ID: u8 = 191;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const LENGTH: usize = 12;

    /// Calculate VHT (802.11ac) data rate in Mbps
    pub fn max_rate(&self, channel_width: ChannelWidth, ht_caps: Option<&HtCapabilities>) -> f64 {
        let data_subcarriers = match channel_width {
            ChannelWidth::TwentyMhz => 52.0,
            ChannelWidth::FortyMhz => 108.0,
            ChannelWidth::EightyMhz => 234.0,
            ChannelWidth::EightyPlusEightyMhz | ChannelWidth::OneSixtyMhz => 468.0,
            _ => return 0.0,
        };

        let short_gi = self.supports_short_gi_for_width(channel_width)
            || ht_caps
                .map(|ht_caps| ht_caps.supports_short_gi_for_width(channel_width))
                .unwrap_or(false);
        // VHT symbol duration
        // 3.2 µs OFDM symbol + 0.8 µs long GI = 4.0 µs total
        // 3.2 µs OFDM symbol + 0.4 µs short GI = 3.6 µs total
        let symbol_duration_us = if short_gi { 3.6 } else { 4.0 };

        // Bits per symbol (modulation × coding rate)
        let bits_per_symbol = match self.max_mcs() {
            0 => 0.5,      // BPSK 1/2
            1 => 1.0,      // QPSK 1/2
            2 => 1.5,      // QPSK 3/4
            3 => 2.0,      // 16-QAM 1/2
            4 => 3.0,      // 16-QAM 3/4
            5 => 4.0,      // 64-QAM 2/3
            6 => 4.5,      // 64-QAM 3/4
            7 => 5.0,      // 64-QAM 5/6
            8 => 6.0,      // 256-QAM 3/4
            9 => 6.666667, // 256-QAM 5/6 (exactly 20/3)
            _ => return 0.0,
        };

        // Rate formula: (subcarriers × bits/symbol × streams) / symbol_duration
        (data_subcarriers
            * bits_per_symbol
            * f64::from(
                self.supported_vht_mcs_and_nss_set
                    .rx_vht_mcs_map
                    .max_spatial_streams(),
            ))
            / symbol_duration_us
    }

    pub(crate) fn supports_short_gi_for_width(&self, width: ChannelWidth) -> bool {
        match width {
            ChannelWidth::EightyMhz => self.vht_capabilities_info.short_gi_for_eighty_mhz,
            ChannelWidth::EightyPlusEightyMhz | ChannelWidth::OneSixtyMhz => {
                self.vht_capabilities_info
                    .short_gi_for_one_hundred_sixty_and_eighty_plus_eighty_mhz
            }
            _ => false, // 20/40 MHz use HT short GI
        }
    }

    pub(crate) fn max_mcs(&self) -> u8 {
        let max_streams = self
            .supported_vht_mcs_and_nss_set
            .rx_vht_mcs_map
            .max_spatial_streams();
        self.supported_vht_mcs_and_nss_set
            .rx_vht_mcs_map
            .max_mcs_for_stream(max_streams)
            .unwrap_or(0)
    }

    pub fn summary(&self) -> String {
        let mut summary = Vec::new();
        let max_spatial_streams = self
            .supported_vht_mcs_and_nss_set
            .rx_vht_mcs_map
            .max_spatial_streams();
        if max_spatial_streams == 1 {
            summary.push("1 Spatial Stream".to_string());
        } else {
            summary.push(format!("{} Spatial Streams", max_spatial_streams));
        }

        if self.vht_capabilities_info.short_gi_for_eighty_mhz {
            summary.push("Short GI for 80 MHz".to_string());
        }

        if self
            .vht_capabilities_info
            .short_gi_for_one_hundred_sixty_and_eighty_plus_eighty_mhz
        {
            summary.push("Short GI for 160 MHz".to_string());
        }

        summary.join(", ")
    }

    pub fn fields(&self) -> Vec<Field> {
        vec![
            self.vht_capabilities_info.to_field(),
            self.supported_vht_mcs_and_nss_set.to_fields(),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct VhtCapabilitiesInfo {
    #[deku(
        bits = 2,
        map = "|value: u8| match value {
        0 => Ok(3895),
        1 => Ok(7991),
        2 => Ok(11_454),
        _ => Err(deku::DekuError::Parse(\"Invalid Maximum MPDU Length\".into()))
        }",
        writer = "match maximum_mpdu_length {
            3895 => write_bits_lsb0(deku::writer, 0, 2),
            7991 => write_bits_lsb0(deku::writer, 1, 2),
            11_454 => write_bits_lsb0(deku::writer, 2, 2),
            _ => write_bits_lsb0(deku::writer, 0, 2)
        }"
    )]
    pub maximum_mpdu_length: u16,
    #[deku(bits = 2)]
    pub supported_channel_width_set: u8,
    #[deku(bits = 1)]
    pub rx_ldpc: bool,
    #[deku(bits = 1)]
    pub short_gi_for_eighty_mhz: bool,
    #[deku(bits = 1)]
    pub short_gi_for_one_hundred_sixty_and_eighty_plus_eighty_mhz: bool,
    #[deku(bits = 1)]
    pub tx_stbc: bool,
    #[deku(bits = 3)]
    pub rx_stbc: u8,
    #[deku(bits = 1)]
    pub su_beamformer_capable: bool,
    #[deku(bits = 1)]
    pub su_beamformee_capable: bool,
    #[deku(bits = 3)]
    pub beamformee_sts_capability: u8,
    #[deku(bits = 3)]
    pub number_of_sounding_dimensions: u8,
    #[deku(bits = 1)]
    pub mu_beamformer_capable: bool,
    #[deku(bits = 1)]
    pub mu_beamformee_capable: bool,
    #[deku(bits = 1)]
    pub txop_ps: bool,
    #[deku(bits = 1)]
    pub htc_vht_capable: bool,
    #[deku(bits = 3)]
    pub maximum_ampdu_length_exponent: u8,
    #[deku(bits = 2)]
    pub vht_link_adaptation_capable: u8,
    #[deku(bits = 1)]
    pub rx_antenna_pattern_consistency: bool,
    #[deku(bits = 1)]
    pub tx_antenna_pattern_consistency: bool,
    #[deku(bits = 2)]
    pub extended_nss_bw_support: u8,
}

impl VhtCapabilitiesInfo {
    pub fn to_field(&self) -> Field {
        let bytes = self.to_bytes().unwrap_or_default();
        Field::builder()
            .title("VHT Capabilities Info")
            .value("")
            .subfields([
                Field::builder()
                    .title("Maximum MPDU Length")
                    .value(self.maximum_mpdu_length)
                    .units("bytes")
                    .bits(BitRange::new(&bytes, 0, 2))
                    .build(),
                Field::builder()
                    .title("Supported Channel Width Set")
                    .value(self.supported_channel_width_set)
                    .bits(BitRange::new(&bytes, 2, 2))
                    .build(),
                Field::builder()
                    .title("Rx LDPC")
                    .value(self.rx_ldpc)
                    .bits(BitRange::new(&bytes, 4, 1))
                    .build(),
                Field::builder()
                    .title("Short GI for 80 MHz")
                    .value(self.short_gi_for_eighty_mhz)
                    .bits(BitRange::new(&bytes, 5, 1))
                    .build(),
                Field::builder()
                    .title("Short GI for 160 MHz and 80+80 MHz")
                    .value(self.short_gi_for_one_hundred_sixty_and_eighty_plus_eighty_mhz)
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
                    .bits(BitRange::new(&bytes, 8, 3))
                    .build(),
                Field::builder()
                    .title("SU Beamformer Capable")
                    .value(self.su_beamformer_capable)
                    .bits(BitRange::new(&bytes, 11, 1))
                    .build(),
                Field::builder()
                    .title("SU Beamformee Capable")
                    .value(self.su_beamformee_capable)
                    .bits(BitRange::new(&bytes, 12, 1))
                    .build(),
                Field::builder()
                    .title("Beamformee STS Capability")
                    .value(self.beamformee_sts_capability + 1)
                    .bits(BitRange::new(&bytes, 13, 3))
                    .build(),
                Field::builder()
                    .title("Number of Sounding Dimensions")
                    .value(self.number_of_sounding_dimensions + 1)
                    .bits(BitRange::new(&bytes, 16, 3))
                    .build(),
                Field::builder()
                    .title("MU Beamformer Capable")
                    .value(self.mu_beamformer_capable)
                    .bits(BitRange::new(&bytes, 19, 1))
                    .build(),
                Field::builder()
                    .title("MU Beamformee Capable")
                    .value(self.mu_beamformee_capable)
                    .bits(BitRange::new(&bytes, 20, 1))
                    .build(),
                Field::builder()
                    .title("TXOP PS")
                    .value(self.txop_ps)
                    .bits(BitRange::new(&bytes, 21, 1))
                    .build(),
                Field::builder()
                    .title("+HTC-VHT Capable")
                    .value(self.htc_vht_capable)
                    .bits(BitRange::new(&bytes, 22, 1))
                    .build(),
                Field::builder()
                    .title("Maximum A-MPDU Length Exponent")
                    .value(self.maximum_ampdu_length_exponent)
                    .bits(BitRange::new(&bytes, 23, 3))
                    .build(),
                Field::builder()
                    .title("VHT Link Adaptation Capable")
                    .value(self.vht_link_adaptation_capable)
                    .bits(BitRange::new(&bytes, 26, 2))
                    .build(),
                Field::builder()
                    .title("Rx Antenna Pattern Consistency")
                    .value(self.rx_antenna_pattern_consistency)
                    .bits(BitRange::new(&bytes, 28, 1))
                    .build(),
                Field::builder()
                    .title("Tx Antenna Pattern Consistency")
                    .value(self.tx_antenna_pattern_consistency)
                    .bits(BitRange::new(&bytes, 29, 1))
                    .build(),
                Field::builder()
                    .title("Extended NSS BW Support")
                    .value(self.extended_nss_bw_support)
                    .bits(BitRange::new(&bytes, 30, 2))
                    .build(),
            ])
            .bytes(bytes)
            .build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct SupportedVhtMcsAndNssSet {
    pub rx_vht_mcs_map: VhtMcsMap,
    pub rx_mcs_nss_info: RxMcsNssInfo,
    pub tx_vht_mcs_map: VhtMcsMap,
    pub tx_mcs_nss_info: TxMcsNssInfo,
}

impl SupportedVhtMcsAndNssSet {
    pub fn to_fields(&self) -> Field {
        let bytes = self.to_bytes().unwrap_or_default();

        Field::builder()
            .title("Supported VHT-MCS and NSS Set")
            .value("")
            .subfields([
                self.rx_vht_mcs_map.to_field("Rx VHT-MCS Map"),
                Field::builder()
                    .title("Rx Highest Supported Long GI Data Rate")
                    .value(self.rx_mcs_nss_info.rx_highest_supported_long_gi_data_rate)
                    .units("Mb/s")
                    .bits(BitRange::new(&bytes[2..4], 0, 13))
                    .build(),
                Field::builder()
                    .title("Maximum NSTS,total")
                    .value(self.rx_mcs_nss_info.maximum_nsts_total)
                    .bits(BitRange::new(&bytes[2..4], 13, 3))
                    .build(),
                self.tx_vht_mcs_map.to_field("Tx VHT-MCS Map"),
                Field::builder()
                    .title("Tx Highest Supported Long GI Data Rate")
                    .value(self.tx_mcs_nss_info.tx_highest_supported_long_gi_data_rate)
                    .units("Mb/s")
                    .bits(BitRange::new(&bytes[6..], 0, 13))
                    .build(),
                Field::builder()
                    .title("VHT Extended NSS BW Capable")
                    .value(self.tx_mcs_nss_info.vht_extended_nss_bw_capable)
                    .bits(BitRange::new(&bytes[6..], 13, 1))
                    .build(),
                Field::reserved(BitRange::new(&bytes[6..], 14, 2)),
            ])
            .bytes(bytes)
            .build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct RxMcsNssInfo {
    #[deku(bits = 13)]
    pub rx_highest_supported_long_gi_data_rate: u16,
    #[deku(bits = 3)]
    pub maximum_nsts_total: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct TxMcsNssInfo {
    #[deku(bits = 13)]
    pub tx_highest_supported_long_gi_data_rate: u16,
    #[deku(bits = 1)]
    pub vht_extended_nss_bw_capable: bool,
    #[deku(bits = 2)]
    pub reserved: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct VhtMcsMap {
    #[deku(
        bits = 2,
        map = "|value: u8| Support::try_from(value).map_err(|_| DekuError::Parse(\"Invalid Support\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*max_vht_mcs_for_one_ss), 2)"
    )]
    pub max_vht_mcs_for_one_ss: Support,
    #[deku(
        bits = 2,
        map = "|value: u8| Support::try_from(value).map_err(|_| DekuError::Parse(\"Invalid Support\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*max_vht_mcs_for_two_ss), 2)"
    )]
    pub max_vht_mcs_for_two_ss: Support,
    #[deku(
        bits = 2,
        map = "|value: u8| Support::try_from(value).map_err(|_| DekuError::Parse(\"Invalid Support\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*max_vht_mcs_for_three_ss), 2)"
    )]
    pub max_vht_mcs_for_three_ss: Support,
    #[deku(
        bits = 2,
        map = "|value: u8| Support::try_from(value).map_err(|_| DekuError::Parse(\"Invalid Support\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*max_vht_mcs_for_four_ss), 2)"
    )]
    pub max_vht_mcs_for_four_ss: Support,
    #[deku(
        bits = 2,
        map = "|value: u8| Support::try_from(value).map_err(|_| DekuError::Parse(\"Invalid Support\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*max_vht_mcs_for_five_ss), 2)"
    )]
    pub max_vht_mcs_for_five_ss: Support,
    #[deku(
        bits = 2,
        map = "|value: u8| Support::try_from(value).map_err(|_| DekuError::Parse(\"Invalid Support\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*max_vht_mcs_for_six_ss), 2)"
    )]
    pub max_vht_mcs_for_six_ss: Support,
    #[deku(
        bits = 2,
        map = "|value: u8| Support::try_from(value).map_err(|_| DekuError::Parse(\"Invalid Support\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*max_vht_mcs_for_seven_ss), 2)"
    )]
    pub max_vht_mcs_for_seven_ss: Support,
    #[deku(
        bits = 2,
        map = "|value: u8| Support::try_from(value).map_err(|_| DekuError::Parse(\"Invalid Support\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*max_vht_mcs_for_eight_ss), 2)"
    )]
    pub max_vht_mcs_for_eight_ss: Support,
}

impl VhtMcsMap {
    /// Returns the maximum number of spatial streams supported.
    pub fn max_spatial_streams(&self) -> u8 {
        for (support, stream) in [
            (self.max_vht_mcs_for_eight_ss, 8),
            (self.max_vht_mcs_for_seven_ss, 7),
            (self.max_vht_mcs_for_six_ss, 6),
            (self.max_vht_mcs_for_five_ss, 5),
            (self.max_vht_mcs_for_four_ss, 4),
            (self.max_vht_mcs_for_three_ss, 3),
            (self.max_vht_mcs_for_two_ss, 2),
            (self.max_vht_mcs_for_one_ss, 1),
        ] {
            if support != Support::NotSupported {
                return stream;
            }
        }
        1 // At least 1 stream
    }

    /// Returns the maximum MCS index supported for the given spatial stream.
    pub fn max_mcs_for_stream(&self, stream: u8) -> Option<u8> {
        let support = match stream {
            1 => self.max_vht_mcs_for_one_ss,
            2 => self.max_vht_mcs_for_two_ss,
            3 => self.max_vht_mcs_for_three_ss,
            4 => self.max_vht_mcs_for_four_ss,
            5 => self.max_vht_mcs_for_five_ss,
            6 => self.max_vht_mcs_for_six_ss,
            7 => self.max_vht_mcs_for_seven_ss,
            8 => self.max_vht_mcs_for_eight_ss,
            _ => Support::NotSupported,
        };

        match support {
            Support::VhtMcsZeroThroughSeven => Some(7),
            Support::VhtMcsZeroThroughEight => Some(8),
            Support::VhtMcsZeroThroughNine => Some(9),
            Support::NotSupported => None,
        }
    }

    pub fn to_field(&self, title: &str) -> Field {
        let bytes = self.to_bytes().unwrap_or_default();
        Field::builder()
            .title(title)
            .value("")
            .bytes(bytes.to_vec())
            .subfields([
                Field::builder()
                    .title("Max VHT-MCS for 1 SS")
                    .value(self.max_vht_mcs_for_one_ss)
                    .units(format!("({})", u8::from(self.max_vht_mcs_for_one_ss)))
                    .bits(BitRange::new(&bytes, 0, 2))
                    .build(),
                Field::builder()
                    .title("Max VHT-MCS for 2 SS")
                    .value(self.max_vht_mcs_for_two_ss)
                    .units(format!("({})", u8::from(self.max_vht_mcs_for_two_ss)))
                    .bits(BitRange::new(&bytes, 2, 2))
                    .build(),
                Field::builder()
                    .title("Max VHT-MCS for 3 SS")
                    .value(self.max_vht_mcs_for_three_ss)
                    .units(format!("({})", u8::from(self.max_vht_mcs_for_three_ss)))
                    .bits(BitRange::new(&bytes, 4, 2))
                    .build(),
                Field::builder()
                    .title("Max VHT-MCS for 4 SS")
                    .value(self.max_vht_mcs_for_four_ss)
                    .units(format!("({})", u8::from(self.max_vht_mcs_for_four_ss)))
                    .bits(BitRange::new(&bytes, 6, 2))
                    .build(),
                Field::builder()
                    .title("Max VHT-MCS for 5 SS")
                    .value(self.max_vht_mcs_for_five_ss)
                    .units(format!("({})", u8::from(self.max_vht_mcs_for_five_ss)))
                    .bits(BitRange::new(&bytes, 8, 2))
                    .build(),
                Field::builder()
                    .title("Max VHT-MCS for 6 SS")
                    .value(self.max_vht_mcs_for_six_ss)
                    .units(format!("({})", u8::from(self.max_vht_mcs_for_six_ss)))
                    .bits(BitRange::new(&bytes, 10, 2))
                    .build(),
                Field::builder()
                    .title("Max VHT-MCS for 7 SS")
                    .value(self.max_vht_mcs_for_seven_ss)
                    .units(format!("({})", u8::from(self.max_vht_mcs_for_seven_ss)))
                    .bits(BitRange::new(&bytes, 12, 2))
                    .build(),
                Field::builder()
                    .title("Max VHT-MCS for 8 SS")
                    .value(self.max_vht_mcs_for_eight_ss)
                    .units(format!("({})", u8::from(self.max_vht_mcs_for_eight_ss)))
                    .bits(BitRange::new(&bytes, 14, 2))
                    .build(),
            ])
            .build()
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum Support {
    VhtMcsZeroThroughSeven = 0,
    VhtMcsZeroThroughEight,
    VhtMcsZeroThroughNine,
    NotSupported,
}

impl Display for Support {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VhtMcsZeroThroughSeven => write!(f, "VHT-MCS 0-7"),
            Self::VhtMcsZeroThroughEight => write!(f, "VHT-MCS 0-8"),
            Self::VhtMcsZeroThroughNine => write!(f, "VHT-MCS 0-9"),
            Self::NotSupported => write!(f, "Not Supported"),
        }
    }
}
