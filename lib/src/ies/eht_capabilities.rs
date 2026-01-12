use std::io::Cursor;

use deku::{
    DekuError, DekuRead, DekuReader, DekuWrite,
    bitvec::{BitSlice, BitView, Lsb0},
    reader::Reader,
};
use serde::{Deserialize, Serialize};

use crate::ChannelWidth;

use super::{HeCapabilities, IeId};

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "len: usize")]
pub struct EhtCapabilities {
    pub eht_mac_capabilities_information: EhtMacCapabilitiesInformation,
    pub eht_phy_capabilities_information: EhtPhyCapabilitiesInformation,
    #[deku(count = "len.checked_sub(11).unwrap_or_default()")]
    rest: Vec<u8>,
    #[deku(skip)]
    pub supported_eht_mcs_and_nss_set: Option<SupportedEhtMcsAndNssSet>,
    #[deku(skip)]
    pub eht_ppe_thresholds: Option<EhtPpeThresholds>,
}

impl EhtCapabilities {
    pub const NAME: &'static str = "EHT Capabilities";
    pub const ID: u8 = 255;
    pub const ID_EXT: Option<u8> = Some(108);
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);

    pub(crate) fn parse_with_he_capabilities(
        &mut self,
        he_capabilities: &HeCapabilities,
    ) -> Result<(), DekuError> {
        let cursor = Cursor::new(&self.rest);
        let mut reader = Reader::new(cursor);
        let supported_eht_mcs_and_nss_set = SupportedEhtMcsAndNssSet::from_reader_with_ctx(
            &mut reader,
            (
                he_capabilities
                    .he_phy_capabilities_information
                    .supported_channel_width_set
                    .view_bits::<Lsb0>()[0],
                self.eht_phy_capabilities_information
                    .support_for_320_mhz_in_6_ghz,
            ),
        )?;
        self.supported_eht_mcs_and_nss_set = Some(supported_eht_mcs_and_nss_set);

        if self.eht_phy_capabilities_information.ppe_thresholds_present {
            let eht_ppe_thresholds = EhtPpeThresholds::from_reader_with_ctx(&mut reader, ())?;
            self.eht_ppe_thresholds = Some(eht_ppe_thresholds);
        }

        Ok(())
    }

    /// Calculate EHT (802.11be) data rate in Mbps
    pub(crate) fn max_rate(&self, channel_width: ChannelWidth) -> f64 {
        let data_subcarriers = match channel_width {
            ChannelWidth::TwentyMhz => 234.0,
            ChannelWidth::FortyMhz => 468.0,
            ChannelWidth::EightyMhz => 980.0,
            ChannelWidth::EightyPlusEightyMhz | ChannelWidth::OneSixtyMhz => 1960.0,
            ChannelWidth::ThreeHundredTwentyMhz => 3920.0,
        };

        // EHT uses same 12.8 µs OFDM symbol as HE
        let symbol_duration_us = 12.8 + self.min_guard_interval_us();

        // Bits per symbol (modulation × coding rate)
        let bits_per_symbol = match self.max_mcs() {
            0 => 0.5,       // BPSK 1/2
            1 => 1.0,       // QPSK 1/2
            2 => 1.5,       // QPSK 3/4
            3 => 2.0,       // 16-QAM 1/2
            4 => 3.0,       // 16-QAM 3/4
            5 => 4.0,       // 64-QAM 2/3
            6 => 4.5,       // 64-QAM 3/4
            7 => 5.0,       // 64-QAM 5/6
            8 => 6.0,       // 256-QAM 3/4
            9 => 6.666667,  // 256-QAM 5/6
            10 => 8.0,      // 1024-QAM 3/4
            11 => 8.333333, // 1024-QAM 5/6
            12 => 10.0,     // 4096-QAM 3/4 (EHT introduces 4096-QAM)
            13 => 10.0,     // 4096-QAM 5/6
            _ => return 0.0,
        };

        // Rate formula
        (data_subcarriers * bits_per_symbol * f64::from(self.max_spatial_streams()))
            / f64::from(symbol_duration_us)
    }

    fn max_spatial_streams(&self) -> u8 {
        let rx_mcs_map_80 = self
            .supported_eht_mcs_and_nss_set
            .as_ref()
            .map(|mcs_nss_set| {
                mcs_nss_set
                    .eht_mcs_map_bw_lte_80_mhz_except_20_mhz_only_non_ap_sta
                    .clone()
            })
            .unwrap_or_default();
        let rx_mcs_map_80 = [
            rx_mcs_map_80.get(0).cloned().unwrap_or_default(),
            rx_mcs_map_80.get(1).cloned().unwrap_or_default(),
            rx_mcs_map_80.get(2).cloned().unwrap_or_default(),
            0,
        ];
        self.max_spatial_streams_for_map(u32::from_le_bytes(rx_mcs_map_80))
    }

    fn max_spatial_streams_for_map(&self, mcs_map: u32) -> u8 {
        // EHT MCS map: 4 bits per stream, 8 streams total (32 bits)
        // 0000 = MCS 0-7
        // 0001 = MCS 0-9
        // 0010 = MCS 0-11
        // 0011 = MCS 0-13 (4096-QAM support!)
        // 1111 = Not supported

        for stream in (1..=8).rev() {
            let shift = (stream - 1) * 4;
            let mcs_support = (mcs_map >> shift) & 0b1111;

            if mcs_support != 0b1111 {
                // Not "not supported"
                return stream;
            }
        }

        1 // At least 1 stream
    }

    fn min_guard_interval_us(&self) -> f32 {
        0.8
    }

    /// Get maximum MCS for a given spatial stream
    fn max_mcs_for_stream(&self, stream: u8) -> Option<u8> {
        if stream < 1 || stream > 8 {
            return None;
        }

        let shift = (stream - 1) * 4;
        let rx_mcs_map_80 = self
            .supported_eht_mcs_and_nss_set
            .as_ref()
            .map(|mcs_nss_set| {
                mcs_nss_set
                    .eht_mcs_map_bw_lte_80_mhz_except_20_mhz_only_non_ap_sta
                    .clone()
            })
            .unwrap_or_default();
        let rx_mcs_map_80 = [
            rx_mcs_map_80.get(0).cloned().unwrap_or_default(),
            rx_mcs_map_80.get(1).cloned().unwrap_or_default(),
            rx_mcs_map_80.get(2).cloned().unwrap_or_default(),
            0,
        ];
        let mcs_support = (u32::from_le_bytes(rx_mcs_map_80) >> shift) & 0b1111;

        match mcs_support {
            0b0000 => Some(7),  // MCS 0-7
            0b0001 => Some(9),  // MCS 0-9
            0b0010 => Some(11), // MCS 0-11
            0b0011 => Some(13), // MCS 0-13 (4096-QAM!)
            0b1111 => None,     // Not supported
            _ => None,          // Reserved values
        }
    }

    /// Get the highest MCS supported across all streams
    fn max_mcs(&self) -> u8 {
        let max_streams = self.max_spatial_streams();
        self.max_mcs_for_stream(max_streams).unwrap_or(0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct EhtMacCapabilitiesInformation {
    #[deku(bits = 1)]
    pub epcs_priority_access_support: bool,
    #[deku(bits = 1)]
    pub eht_om_control_support: bool,
    #[deku(bits = 1)]
    pub txs_mode_1_support: bool,
    #[deku(bits = 1)]
    pub txs_mode_2_support: bool,
    #[deku(bits = 1)]
    pub restricted_twt_support: bool,
    #[deku(bits = 1)]
    pub scs_traffic_description_support: bool,
    #[deku(bits = 2)]
    pub maximum_mpdu_length: u8,
    #[deku(bits = 1)]
    pub maximum_ampdu_length_exponent_extension: bool,
    #[deku(bits = 1)]
    pub eht_trs_support: bool,
    #[deku(bits = 1)]
    pub txop_return_support_in_txs_mode_2: bool,
    #[deku(bits = 1)]
    pub two_bqrs_support: bool,
    #[deku(bits = 2)]
    pub eht_link_adaptation_support: u8,
    #[deku(bits = 1)]
    pub unsolicited_epcs_priority_access_parameter_update: bool,
    #[deku(bits = 1)]
    pub reserved: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct EhtPhyCapabilitiesInformation {
    #[deku(bits = 1)]
    reserved_1: bool,
    #[deku(bits = 1)]
    pub support_for_320_mhz_in_6_ghz: bool,
    #[deku(bits = 1)]
    pub support_for_two_hundred_forty_two_tone_ru_in_bw_wider_than_twenty_mhz: bool,
    #[deku(bits = 1)]
    pub ndp_with_4x_eht_ltf_and_three_point_two_microseconds_gi: bool,
    #[deku(bits = 1)]
    pub partial_bandwidth_ul_mu_mimo: bool,
    #[deku(bits = 1)]
    pub su_beamformer: bool,
    #[deku(bits = 1)]
    pub su_beamformee: bool,
    #[deku(bits = 3)]
    pub beamformee_ss_less_than_or_equal_to_eighty_mhz: u8,
    #[deku(bits = 3)]
    pub beamformee_ss_equal_to_one_hundred_sixty_mhz: u8,
    #[deku(bits = 3)]
    pub beamformee_ss_equal_to_three_hundred_twenty_mhz: u8,
    #[deku(bits = 3)]
    pub number_of_sounding_dimensions_less_than_or_equal_to_eighty_mhz: u8,
    #[deku(bits = 3)]
    pub number_of_sounding_dimensions_equal_to_one_hundred_sixty_mhz: u8,
    #[deku(bits = 3)]
    pub number_of_sounding_dimensions_equal_to_three_hundred_twenty_mhz: u8,
    #[deku(bits = 1)]
    pub ng_equals_sixteen_su_feedback: bool,
    #[deku(bits = 1)]
    pub ng_equals_sixteen_mu_feedback: bool,
    #[deku(bits = 1)]
    pub codebook_size_su_feedback: bool,
    #[deku(bits = 1)]
    pub codebook_size_mu_feedback: bool,
    #[deku(bits = 1)]
    pub triggered_su_beamforming_feedback: bool,
    #[deku(bits = 1)]
    pub triggered_mu_beamforming_partial_bw_feedback: bool,
    #[deku(bits = 1)]
    pub triggered_cqi_feedback: bool,
    #[deku(bits = 1)]
    pub partial_bandwidth_dl_mu_mimo: bool,
    #[deku(bits = 1)]
    pub eht_psr_based_sr_support: bool,
    #[deku(bits = 1)]
    pub power_boost_factor_support: bool,
    #[deku(bits = 1)]
    pub eht_mu_ppdu_with_4x_eht_ltf_and_zero_point_eight_microseconds_gi: bool,
    #[deku(bits = 4)]
    pub max_nc: u8,
    #[deku(bits = 1)]
    pub non_triggered_cqi_feedback: bool,
    #[deku(bits = 1)]
    pub tx_1024_qam_and_4096_qam_less_than_242_tone_ru_support: bool,
    #[deku(bits = 1)]
    pub rx_1024_qam_and_4096_qam_less_than_242_tone_ru_support: bool,
    #[deku(bits = 1)]
    pub ppe_thresholds_present: bool,
    #[deku(bits = 2)]
    pub common_nominal_packet_padding: u8,
    #[deku(bits = 5)]
    pub maximum_number_of_supported_eht_ltfs: u8,
    #[deku(bits = 4)]
    pub support_of_eht_mcs_15_in_mru: u8,
    #[deku(bits = 1)]
    pub support_of_eht_dup_in_6_ghz: bool,
    #[deku(bits = 1)]
    pub support_for_20_mhz_operating_sta_receiving_ndp_with_wider_bandwidth: bool,
    #[deku(bits = 1)]
    pub non_ofdma_ul_mu_mimo_bw_less_than_or_equal_to_eighty_mhz: bool,
    #[deku(bits = 1)]
    pub non_ofdma_ul_mu_mimo_bw_equal_to_160_mhz: bool,
    #[deku(bits = 1)]
    pub non_ofdma_ul_mu_mimo_bw_equal_to_320_mhz: bool,
    #[deku(bits = 1)]
    pub mu_beamformer_bw_less_than_or_equal_to_80_mhz: bool,
    #[deku(bits = 1)]
    pub mu_beamformer_bw_equal_to_160_mhz: bool,
    #[deku(bits = 1)]
    pub mu_beamformer_equal_to_320_mhz: bool,
    #[deku(bits = 1)]
    pub tb_sounding_feedback_rate_limit: bool,
    #[deku(bits = 1)]
    pub rx_1024_qam_in_wider_bandwidth_dl_ofdma_support: bool,
    #[deku(bits = 1)]
    pub rx_4096_qam_in_wider_bandwidth_dl_ofdma_support: bool,
    #[deku(bits = 1)]
    pub twenty_mhz_only_limited_capabilities_support: bool,
    #[deku(bits = 1)]
    pub twenty_mhz_only_triggered_mu_beamforming_full_bw_feedback_and_dl_mu_mimo: bool,
    #[deku(bits = 1)]
    pub twenty_mhz_only_mru_support: bool,
    #[deku(bits = 3)]
    reserved_2: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "eht_mcs_map_bw_eq_160_mhz_present: bool, eht_mcs_map_bw_eq_320_mhz_present: bool")]
pub struct SupportedEhtMcsAndNssSet {
    // Note: 20 MHz-only map for non-AP STAs is not parsed, as this library
    // only processes beacons/probe responses from APs

    // Always present for APs
    #[deku(count = "3")]
    pub eht_mcs_map_bw_lte_80_mhz_except_20_mhz_only_non_ap_sta: Vec<u8>,

    // Relies on HE Capabilities to know whether or not it's present
    // Therefore we can't finish parsing EHT Capabilities without passing
    // a reference to HE Capabilities
    #[deku(cond = "eht_mcs_map_bw_eq_160_mhz_present", count = "3")]
    pub eht_mcs_map_bw_eq_160_mhz: Option<Vec<u8>>,

    #[deku(cond = "eht_mcs_map_bw_eq_320_mhz_present", count = "3")]
    pub eht_mcs_map_bw_eq_320_mhz: Option<Vec<u8>>,
}

impl SupportedEhtMcsAndNssSet {}

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct EhtPpeThresholds {
    #[deku(bits = 4, bit_order = "lsb")]
    pub nss_pe: u8,
    #[deku(bits = 5)]
    pub ru_index_bitmask: u8,
    #[deku(count = "nss_pe + 1", ctx = "ru_index_bitmask.view_bits()")]
    pub ppe_thresholds_info: Vec<PpeThresholdsInfo>,
    #[deku(
        bits = "(8 - (9 + 6 * ppe_thresholds_info.len() * ru_index_bitmask.view_bits::<Lsb0>().count_ones()) % 8) % 8"
    )]
    ppe_pad: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(
    bit_order = "lsb",
    ctx = "bit_order: deku::ctx::Order, ru_index_bitmask: &BitSlice<u8, Lsb0>"
)]
pub struct PpeThresholdsInfo {
    #[deku(cond = "ru_index_bitmask[0]", bits = 3)]
    pub ppet_max_nss_ru_0: Option<u8>,
    #[deku(cond = "ru_index_bitmask[0]", bits = 3)]
    pub ppet_eight_nss_ru_0: Option<u8>,
    #[deku(cond = "ru_index_bitmask[1]", bits = 3)]
    pub ppet_max_nss_ru_1: Option<u8>,
    #[deku(cond = "ru_index_bitmask[1]", bits = 3)]
    pub ppet_eight_nss_ru_1: Option<u8>,
    #[deku(cond = "ru_index_bitmask[2]", bits = 3)]
    pub ppet_max_nss_ru_2: Option<u8>,
    #[deku(cond = "ru_index_bitmask[2]", bits = 3)]
    pub ppet_eight_nss_ru_2: Option<u8>,
    #[deku(cond = "ru_index_bitmask[3]", bits = 3)]
    pub ppet_max_nss_ru_3: Option<u8>,
    #[deku(cond = "ru_index_bitmask[3]", bits = 3)]
    pub ppet_eight_nss_ru_3: Option<u8>,
    #[deku(cond = "ru_index_bitmask[4]", bits = 3)]
    pub ppet_max_nss_ru_4: Option<u8>,
    #[deku(cond = "ru_index_bitmask[4]", bits = 3)]
    pub ppet_eight_nss_ru_4: Option<u8>,
}
