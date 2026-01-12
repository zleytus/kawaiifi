use deku::{
    DekuRead, DekuWrite,
    bitvec::{BitSlice, BitVec, BitView, Lsb0},
};
use serde::{Deserialize, Serialize};

use super::IeId;
use crate::ChannelWidth;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct HeCapabilities {
    pub he_mac_capabilities_information: HeMacCapabilitiesInformation,
    pub he_phy_capabilities_information: HePhyCapabilitiesInformation,
    #[deku(
        ctx = "BitVec::from_element(he_phy_capabilities_information.supported_channel_width_set)"
    )]
    pub supported_he_mcs_and_nss_set: SupportedHeMcsAndNssSet,
    #[deku(cond = "he_phy_capabilities_information.ppe_thresholds_present")]
    pub ppe_thresholds: Option<PpeThresholds>,
}

impl HeCapabilities {
    pub const NAME: &'static str = "HE Capabilities";
    pub const ID: u8 = 255;
    pub const ID_EXT: Option<u8> = Some(35);
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const LENGTH: usize = 22;

    /// Calculate HE (802.11ax) data rate in Mbps
    pub(crate) fn max_rate(&self, channel_width: ChannelWidth) -> f64 {
        let data_subcarriers = match channel_width {
            ChannelWidth::TwentyMhz => 234.0,
            ChannelWidth::FortyMhz => 468.0,
            ChannelWidth::EightyMhz => 980.0,
            ChannelWidth::EightyPlusEightyMhz | ChannelWidth::OneSixtyMhz => 1960.0,
            _ => return 0.0,
        };

        let symbol_duration_us = 12.8 + self.min_guard_interval_us();

        let bits_per_symbol = match self.max_mcs() {
            0 => 0.5,
            1 => 1.0,
            2 => 1.5,
            3 => 2.0,
            4 => 3.0,
            5 => 4.0,
            6 => 4.5,
            7 => 5.0,
            8 => 6.0,
            9 => 6.666667,
            10 => 8.0,
            11 => 8.333333,
            _ => return 0.0,
        };

        (data_subcarriers * bits_per_symbol * f64::from(self.max_spatial_streams()))
            / f64::from(symbol_duration_us)
    }

    /// Get the shortest supported guard interval in microseconds
    fn min_guard_interval_us(&self) -> f32 {
        0.8
    }

    /// Get maximum spatial streams from RX MCS map
    fn max_spatial_streams(&self) -> u8 {
        // Use the 80 MHz map as baseline
        self.max_spatial_streams_for_map(
            self.supported_he_mcs_and_nss_set
                .rx_he_mcs_map_less_than_or_equal_to_eighty_mhz,
        )
    }

    fn max_spatial_streams_for_map(&self, mcs_map: u16) -> u8 {
        // MCS map: 2 bits per stream, 8 streams total
        // 0 = MCS 0-7, 1 = MCS 0-9, 2 = MCS 0-11, 3 = not supported

        for stream in (1..=8).rev() {
            let shift = (stream - 1) * 2;
            let mcs_support = (mcs_map >> shift) & 0b11;

            if mcs_support != 0b11 {
                // 0b11 = not supported
                return stream;
            }
        }

        1 // At least 1 stream
    }

    /// Get maximum MCS for a given spatial stream
    fn max_mcs_for_stream(&self, stream: u8) -> Option<u8> {
        if stream < 1 || stream > 8 {
            return None;
        }

        let shift = (stream - 1) * 2;
        let mcs_support = (self
            .supported_he_mcs_and_nss_set
            .rx_he_mcs_map_less_than_or_equal_to_eighty_mhz
            >> shift)
            & 0b11;

        match mcs_support {
            0b00 => Some(7),  // MCS 0-7 supported
            0b01 => Some(9),  // MCS 0-9 supported
            0b10 => Some(11), // MCS 0-11 supported (1024-QAM)
            0b11 => None,     // Not supported
            _ => None,
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
pub struct HeMacCapabilitiesInformation {
    #[deku(bits = 1)]
    pub htc_he_support: bool,
    #[deku(bits = 1)]
    pub twt_requester_support: bool,
    #[deku(bits = 1)]
    pub twt_responder_support: bool,
    #[deku(bits = 2)]
    pub dynamic_fragmentation_support: u8,
    #[deku(bits = 3)]
    pub maximum_number_of_fragmented_msdus_amsdus_exponent: u8,
    #[deku(bits = 2)]
    pub minimum_fragment_size: u8,
    #[deku(bits = 2)]
    pub trigger_frame_mac_padding_duration: u8,
    #[deku(bits = 3)]
    pub multi_tid_aggregation_rx_support: u8,
    #[deku(bits = 2)]
    pub he_link_adaptation_support: u8,
    #[deku(bits = 1)]
    pub all_ack_support: bool,
    #[deku(bits = 1)]
    pub trs_support: bool,
    #[deku(bits = 1)]
    pub bsr_support: bool,
    #[deku(bits = 1)]
    pub broadcast_twt_support: bool,
    #[deku(bits = 1)]
    pub thirty_two_bit_ba_bitmap_support: bool,
    #[deku(bits = 1)]
    pub mu_cascading_support: bool,
    #[deku(bits = 1)]
    pub ack_enabled_aggregation_support: bool,
    #[deku(bits = 1)]
    reserved: bool,
    #[deku(bits = 1)]
    pub om_control_support: bool,
    #[deku(bits = 1)]
    pub ofdma_ra_support: bool,
    #[deku(bits = 2)]
    pub maximum_ampdu_length_exponent_extension: u8,
    #[deku(bits = 1)]
    pub amsdu_fragmentation_support: bool,
    #[deku(bits = 1)]
    pub flexible_twt_schedule_support: bool,
    #[deku(bits = 1)]
    pub rx_control_frame_to_multibss: bool,
    #[deku(bits = 1)]
    pub bsrp_bqrp_ampdu_aggregation: bool,
    #[deku(bits = 1)]
    pub qtp_support: bool,
    #[deku(bits = 1)]
    pub bqr_support: bool,
    #[deku(bits = 1)]
    pub psr_responder: bool,
    #[deku(bits = 1)]
    pub ndp_feedback_report_support: bool,
    #[deku(bits = 1)]
    pub ops_support: bool,
    #[deku(bits = 1)]
    pub amsdu_not_under_ba_in_ack_enabled_ampdu_support: bool,
    #[deku(bits = 3)]
    pub multi_tid_aggregation_tx_support: u8,
    #[deku(bits = 1)]
    pub he_subchannel_selective_transmission_support: bool,
    #[deku(bits = 1)]
    pub ul_2_by_996_tone_ru_support: bool,
    #[deku(bits = 1)]
    pub om_control_ul_mu_data_disable_rx_support: bool,
    #[deku(bits = 1)]
    pub he_dynamic_sm_power_save: bool,
    #[deku(bits = 1)]
    pub punctured_sounding_support: bool,
    #[deku(bits = 1)]
    pub ht_and_vht_trigger_frame_rx_support: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct HePhyCapabilitiesInformation {
    #[deku(bits = 1)]
    reserved_1: bool,
    #[deku(bits = 7)]
    pub supported_channel_width_set: u8,
    #[deku(bits = 4)]
    pub punctured_preamble_rx: u8,
    #[deku(bits = 1)]
    pub device_class: bool,
    #[deku(bits = 1)]
    pub ldpc_coding_in_payload: bool,
    #[deku(bits = 1)]
    pub he_ssu_ppdu_with_1x_he_ltf_and_zero_point_eight_microseconds_gi: bool,
    #[deku(bits = 2)]
    pub midamble_tx_rx_max_nsts: u8,
    #[deku(bits = 1)]
    pub ndp_with_4x_he_ltf_and_three_point_two_microseconds_gi: bool,
    #[deku(bits = 1)]
    pub stbc_tx_less_than_or_equal_to_eighty_mhz: bool,
    #[deku(bits = 1)]
    pub stbc_rx_less_than_or_equal_to_eighty_mhz: bool,
    #[deku(bits = 1)]
    pub doppler_tx: bool,
    #[deku(bits = 1)]
    pub doppler_rx: bool,
    #[deku(bits = 1)]
    pub full_bandwidth_ul_mu_mimo: bool,
    #[deku(bits = 1)]
    pub partial_bandwidth_ul_mu_mimo: bool,
    #[deku(bits = 2)]
    pub dcm_max_constellation_tx: u8,
    #[deku(bits = 1)]
    pub dcm_max_nss_tx: bool,
    #[deku(bits = 2)]
    pub dcm_max_constellation_rx: u8,
    #[deku(bits = 1)]
    pub dcm_max_nss_rx: bool,
    #[deku(bits = 1)]
    pub rx_partial_bw_su_in_twenty_mhz_he_mu_ppdu: bool,
    #[deku(bits = 1)]
    pub su_beamformer: bool,
    #[deku(bits = 1)]
    pub su_beamformee: bool,
    #[deku(bits = 1)]
    pub mu_beamformer: bool,
    #[deku(bits = 3)]
    pub beamformee_sts_less_than_or_equal_to_eighty_mhz: u8,
    #[deku(bits = 3)]
    pub beamformee_sts_greater_than_eighty_mhz: u8,
    #[deku(bits = 3)]
    pub number_of_sounding_dimensions_less_than_or_equal_to_eighty_mhz: u8,
    #[deku(bits = 3)]
    pub number_of_sounding_dimensions_greater_than_eighty_mhz: u8,
    #[deku(bits = 1)]
    pub ng_equals_sixteen_su_feedback: bool,
    #[deku(bits = 1)]
    pub ng_equals_sixteen_mu_feedback: bool,
    #[deku(bits = 1)]
    pub codebook_size_su_feedback: bool,
    #[deku(bits = 1)]
    pub codebook_size_mu_feedback: bool,
    #[deku(bits = 1)]
    pub triggered_su_beamfroming_feedback: bool,
    #[deku(bits = 1)]
    pub triggered_mu_beamforming_partial_bw_feedback: bool,
    #[deku(bits = 1)]
    pub triggered_cqi_feedback: bool,
    #[deku(bits = 1)]
    pub partial_bandwidth_extended_range: bool,
    #[deku(bits = 1)]
    pub partial_bandwidth_dl_mu_mimo: bool,
    #[deku(bits = 1)]
    pub ppe_thresholds_present: bool,
    #[deku(bits = 1)]
    pub psr_based_sr_support: bool,
    #[deku(bits = 1)]
    pub power_boosst_factor_support: bool,
    #[deku(bits = 1)]
    pub he_su_ppdu_and_he_mu_ppdu_with_4x_he_ltf_and_zero_point_eighty_microseconds_gi: bool,
    #[deku(bits = 3)]
    pub max_nc: u8,
    #[deku(bits = 1)]
    pub stbc_tx_greater_than_eighty_mhz: bool,
    #[deku(bits = 1)]
    pub stbc_rx_greater_than_eigthy_mhz: bool,
    #[deku(bits = 1)]
    pub he_er_su_ppdu_with_4x_he_ltf_and_zero_point_eight_microseconds_gi: bool,
    #[deku(bits = 1)]
    pub twenty_mhz_in_forty_mhz_he_ppdu_in_two_point_four_ghz_band: bool,
    #[deku(bits = 1)]
    pub twenty_mhz_in_one_hundred_sixty_mhz_he_ppdu: bool,
    #[deku(bits = 1)]
    pub eighty_mhz_in_one_hundred_sixty_mhz_he_ppdu: bool,
    #[deku(bits = 1)]
    pub he_er_su_ppdu_with_1x_he_ltf_and_zero_point_eighty_microseconds_gi: bool,
    #[deku(bits = 1)]
    pub midamble_tx_rx_2x_and_1x_he_ltf: bool,
    #[deku(bits = 2)]
    pub dcm_max_ru: u8,
    #[deku(bits = 1)]
    pub longer_than_sixteen_he_sig_b_ofdm_symbols_present: bool,
    #[deku(bits = 1)]
    pub non_triggered_cqi_feedback: bool,
    #[deku(bits = 1)]
    pub tx_one_thousand_twenty_four_qam_less_than_two_hundred_forty_two_tone_ru_support: bool,
    #[deku(bits = 1)]
    pub rx_one_thousand_twenty_four_qam_less_than_two_hundred_forty_two_tone_ru_support: bool,
    #[deku(bits = 1)]
    pub rx_full_bw_su_using_he_mu_ppdu_with_compressed_he_sig_b: bool,
    #[deku(bits = 1)]
    pub rx_full_bw_su_using_he_mu_ppdu_with_non_compressed_he_sig_b: bool,
    #[deku(bits = 2)]
    pub nominal_packet_padding: u8,
    #[deku(bits = 1)]
    pub he_mu_ppdu_with_more_than_one_ru_rx_max_n_he_ltf: bool,
    #[deku(bits = 7)]
    reserved_2: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "supported_width_set: deku::bitvec::BitVec<u8, Lsb0>")]
pub struct SupportedHeMcsAndNssSet {
    #[deku(bytes = 2)]
    pub rx_he_mcs_map_less_than_or_equal_to_eighty_mhz: u16,
    #[deku(bytes = 2)]
    pub tx_he_mcs_map_less_than_or_equal_to_eighty_mhz: u16,
    #[deku(cond = "supported_width_set[2]", bytes = 2)]
    pub rx_he_mcs_map_one_hudred_sixty_mhz: Option<u16>,
    #[deku(cond = "supported_width_set[2]", bytes = 2)]
    pub tx_he_mcs_map_one_hudred_sixty_mhz: Option<u16>,
    #[deku(cond = "supported_width_set[3]", bytes = 2)]
    pub rx_he_mcs_map_eighty_plus_eighty_mhz: Option<u16>,
    #[deku(cond = "supported_width_set[3]", bytes = 2)]
    pub tx_he_mcs_map_eighty_plus_eighty_mhz: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct PpeThresholds {
    #[deku(bits = 3, bit_order = "lsb")]
    pub nsts: u8,
    #[deku(bits = 4)]
    pub ru_index_bitmask: u8,
    #[deku(count = "nsts + 1", ctx = "ru_index_bitmask.view_bits()")]
    pub ppe_thresholds_info: Vec<PpeThresholdsInfo>,
    #[deku(
        bits = "(8 - (7 + 6 * ppe_thresholds_info.len() * ru_index_bitmask.view_bits::<Lsb0>().count_ones()) % 8) % 8"
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
    pub ppet_sixteen_nsts_ru_0: Option<u8>,
    #[deku(cond = "ru_index_bitmask[0]", bits = 3)]
    pub ppet_eight_nsts_ru_0: Option<u8>,
    #[deku(cond = "ru_index_bitmask[1]", bits = 3)]
    pub ppet_sixteen_nsts_ru_1: Option<u8>,
    #[deku(cond = "ru_index_bitmask[1]", bits = 3)]
    pub ppet_eight_nsts_ru_1: Option<u8>,
    #[deku(cond = "ru_index_bitmask[2]", bits = 3)]
    pub ppet_sixteen_nsts_ru_2: Option<u8>,
    #[deku(cond = "ru_index_bitmask[2]", bits = 3)]
    pub ppet_eight_nsts_ru_2: Option<u8>,
    #[deku(cond = "ru_index_bitmask[3]", bits = 3)]
    pub ppet_sixteen_nsts_ru_3: Option<u8>,
    #[deku(cond = "ru_index_bitmask[3]", bits = 3)]
    pub ppet_eight_nsts_ru_3: Option<u8>,
}
