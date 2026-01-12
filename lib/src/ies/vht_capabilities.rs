use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::{HtCapabilities, IeId, write_bits_lsb0};
use crate::ChannelWidth;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct VhtCapabilities {
    pub vht_capabilities_info: VhtCapabilitiesInfo,
    #[deku(count = "8")]
    pub supported_vht_mcs_and_nss_set: Vec<u8>,
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
        (data_subcarriers * bits_per_symbol * f64::from(self.max_spatial_streams()))
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

    pub(crate) fn max_spatial_streams(&self) -> u8 {
        // MCS map: 2 bits per stream, 8 streams total (16 bits)
        // 0 = MCS 0-7, 1 = MCS 0-8, 2 = MCS 0-9, 3 = not supported

        let rx_mcs_map = [
            self.supported_vht_mcs_and_nss_set[0],
            self.supported_vht_mcs_and_nss_set[1],
        ];
        let rx_mcs_map = u16::from_le_bytes(rx_mcs_map);
        for stream in (1..=8).rev() {
            let shift = (stream - 1) * 2;
            let mcs_support = (rx_mcs_map >> shift) & 0b11;

            if mcs_support != 0b11 {
                return stream;
            }
        }

        1 // At least 1 stream
    }

    pub(crate) fn max_mcs_for_stream(&self, stream: u8) -> Option<u8> {
        if stream < 1 || stream > 8 {
            return None;
        }

        let rx_mcs_map = [
            self.supported_vht_mcs_and_nss_set[0],
            self.supported_vht_mcs_and_nss_set[1],
        ];
        let rx_mcs_map = u16::from_le_bytes(rx_mcs_map);
        let shift = (stream - 1) * 2;
        let mcs_support = (rx_mcs_map >> shift) & 0b11;

        match mcs_support {
            0b00 => Some(7), // MCS 0-7 supported
            0b01 => Some(8), // MCS 0-8 supported
            0b10 => Some(9), // MCS 0-9 supported
            0b11 => None,    // Not supported
            _ => None,
        }
    }

    pub(crate) fn max_mcs(&self) -> u8 {
        let max_streams = self.max_spatial_streams();
        self.max_mcs_for_stream(max_streams).unwrap_or(0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
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
