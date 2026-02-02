use std::fmt::Display;

use deku::{
    DekuContainerWrite, DekuError, DekuRead, DekuWrite, DekuWriter,
    bitvec::{BitSlice, BitVec, BitView, Lsb0},
    writer::Writer,
};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

use super::{IeId, write_bits_lsb0};
use crate::{BitRange, ChannelWidth, Field};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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

        (data_subcarriers
            * bits_per_symbol
            * f64::from(
                self.supported_he_mcs_and_nss_set
                    .rx_he_mcs_map_less_than_or_equal_to_eighty_mhz
                    .max_spatial_streams(),
            ))
            / f64::from(symbol_duration_us)
    }

    /// Get the shortest supported guard interval in microseconds
    fn min_guard_interval_us(&self) -> f32 {
        0.8
    }

    /// Get the highest MCS supported across all streams
    fn max_mcs(&self) -> u8 {
        let mcs_map = self
            .supported_he_mcs_and_nss_set
            .rx_he_mcs_map_less_than_or_equal_to_eighty_mhz;
        let max_streams = mcs_map.max_spatial_streams();
        mcs_map.max_mcs_for_stream(max_streams).unwrap_or(0)
    }

    pub fn summary(&self) -> String {
        let max_spatial_streams = self
            .supported_he_mcs_and_nss_set
            .rx_he_mcs_map_less_than_or_equal_to_eighty_mhz
            .max_spatial_streams();
        if max_spatial_streams == 1 {
            "1 Spatial Stream".to_string()
        } else {
            format!("{} Spatial Streams", max_spatial_streams)
        }
    }

    pub fn fields(&self) -> Vec<Field> {
        let supported_width_set = self
            .he_phy_capabilities_information
            .supported_channel_width_set;

        let mut fields = vec![
            self.he_mac_capabilities_information.to_field(),
            self.he_phy_capabilities_information.to_field(),
            self.supported_he_mcs_and_nss_set
                .to_field(supported_width_set),
        ];

        if let Some(ppe_thresholds) = &self.ppe_thresholds {
            fields.push(ppe_thresholds.to_field());
        }

        fields
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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

impl HeMacCapabilitiesInformation {
    pub fn to_field(&self) -> Field {
        let bytes = self.to_bytes().unwrap_or_default();
        Field::builder()
            .title("HE MAC Capabilities Information")
            .value("")
            .bytes(bytes.clone())
            .subfields([
                Field::builder()
                    .title("+HTC HE Support")
                    .value(self.htc_he_support)
                    .bits(BitRange::new(&bytes, 0, 1))
                    .build(),
                Field::builder()
                    .title("TWT Requester Support")
                    .value(self.twt_requester_support)
                    .bits(BitRange::new(&bytes, 1, 1))
                    .build(),
                Field::builder()
                    .title("TWT Responder Support")
                    .value(self.twt_responder_support)
                    .bits(BitRange::new(&bytes, 2, 1))
                    .build(),
                Field::builder()
                    .title("Dynamic Fragmentation Support")
                    .value(self.dynamic_fragmentation_support)
                    .bits(BitRange::new(&bytes, 3, 2))
                    .build(),
                Field::builder()
                    .title("Maximum Number of Fragmented MSDUs/AMSDUs Exponent")
                    .value(self.maximum_number_of_fragmented_msdus_amsdus_exponent)
                    .bits(BitRange::new(&bytes, 5, 3))
                    .build(),
                Field::builder()
                    .title("Minimum Fragment Size")
                    .value(self.minimum_fragment_size)
                    .bits(BitRange::new(&bytes, 8, 2))
                    .build(),
                Field::builder()
                    .title("Trigger Frame MAC Padding Duration")
                    .value(self.trigger_frame_mac_padding_duration)
                    .bits(BitRange::new(&bytes, 10, 2))
                    .build(),
                Field::builder()
                    .title("Multi-TID Aggregation RX Support")
                    .value(self.multi_tid_aggregation_rx_support)
                    .bits(BitRange::new(&bytes, 12, 3))
                    .build(),
                Field::builder()
                    .title("HE Link Adaptation Support")
                    .value(self.he_link_adaptation_support)
                    .bits(BitRange::new(&bytes, 15, 2))
                    .build(),
                Field::builder()
                    .title("All Ack Support")
                    .value(self.all_ack_support)
                    .bits(BitRange::new(&bytes, 17, 1))
                    .build(),
                Field::builder()
                    .title("TRS Support")
                    .value(self.trs_support)
                    .bits(BitRange::new(&bytes, 18, 1))
                    .build(),
                Field::builder()
                    .title("BSR Support")
                    .value(self.bsr_support)
                    .bits(BitRange::new(&bytes, 19, 1))
                    .build(),
                Field::builder()
                    .title("Broadcast TWT Support")
                    .value(self.broadcast_twt_support)
                    .bits(BitRange::new(&bytes, 20, 1))
                    .build(),
                Field::builder()
                    .title("32-bit BA Bitmap Support")
                    .value(self.thirty_two_bit_ba_bitmap_support)
                    .bits(BitRange::new(&bytes, 21, 1))
                    .build(),
                Field::builder()
                    .title("MU Cascading Support")
                    .value(self.mu_cascading_support)
                    .bits(BitRange::new(&bytes, 22, 1))
                    .build(),
                Field::builder()
                    .title("Ack-Enabled Aggregation Support")
                    .value(self.ack_enabled_aggregation_support)
                    .bits(BitRange::new(&bytes, 23, 1))
                    .build(),
                Field::reserved(BitRange::new(&bytes, 24, 1)),
                Field::builder()
                    .title("OM Control Support")
                    .value(self.om_control_support)
                    .bits(BitRange::new(&bytes, 25, 1))
                    .build(),
                Field::builder()
                    .title("OFDMA RA Support")
                    .value(self.ofdma_ra_support)
                    .bits(BitRange::new(&bytes, 26, 1))
                    .build(),
                Field::builder()
                    .title("Maximum A-MPDU Length Exponent Extension")
                    .value(self.maximum_ampdu_length_exponent_extension)
                    .bits(BitRange::new(&bytes, 27, 2))
                    .build(),
                Field::builder()
                    .title("A-MSDU Fragmentation Support")
                    .value(self.amsdu_fragmentation_support)
                    .bits(BitRange::new(&bytes, 29, 1))
                    .build(),
                Field::builder()
                    .title("Flexible TWT Schedule Support")
                    .value(self.flexible_twt_schedule_support)
                    .bits(BitRange::new(&bytes, 30, 1))
                    .build(),
                Field::builder()
                    .title("Rx Control Frame to MultiBSS")
                    .value(self.rx_control_frame_to_multibss)
                    .bits(BitRange::new(&bytes, 31, 1))
                    .build(),
                Field::builder()
                    .title("BSRP BQRP A-MPDU Aggregation")
                    .value(self.bsrp_bqrp_ampdu_aggregation)
                    .bits(BitRange::new(&bytes, 32, 1))
                    .build(),
                Field::builder()
                    .title("QTP Support")
                    .value(self.qtp_support)
                    .bits(BitRange::new(&bytes, 33, 1))
                    .build(),
                Field::builder()
                    .title("BQR Support")
                    .value(self.bqr_support)
                    .bits(BitRange::new(&bytes, 34, 1))
                    .build(),
                Field::builder()
                    .title("PSR Responder")
                    .value(self.psr_responder)
                    .bits(BitRange::new(&bytes, 35, 1))
                    .build(),
                Field::builder()
                    .title("NDP Feedback Report Support")
                    .value(self.ndp_feedback_report_support)
                    .bits(BitRange::new(&bytes, 36, 1))
                    .build(),
                Field::builder()
                    .title("OPS Support")
                    .value(self.ops_support)
                    .bits(BitRange::new(&bytes, 37, 1))
                    .build(),
                Field::builder()
                    .title("A-MSDU Not Under BA in Ack-Enabled A-MPDU Support")
                    .value(self.amsdu_not_under_ba_in_ack_enabled_ampdu_support)
                    .bits(BitRange::new(&bytes, 38, 1))
                    .build(),
                Field::builder()
                    .title("Multi-TID Aggregation TX Support")
                    .value(self.multi_tid_aggregation_tx_support)
                    .bits(BitRange::new(&bytes, 39, 3))
                    .build(),
                Field::builder()
                    .title("HE Subchannel Selective Transmission Support")
                    .value(self.he_subchannel_selective_transmission_support)
                    .bits(BitRange::new(&bytes, 42, 1))
                    .build(),
                Field::builder()
                    .title("UL 2×996-tone RU Support")
                    .value(self.ul_2_by_996_tone_ru_support)
                    .bits(BitRange::new(&bytes, 43, 1))
                    .build(),
                Field::builder()
                    .title("OM Control UL MU Data Disable RX Support")
                    .value(self.om_control_ul_mu_data_disable_rx_support)
                    .bits(BitRange::new(&bytes, 44, 1))
                    .build(),
                Field::builder()
                    .title("HE Dynamic SM Power Save")
                    .value(self.he_dynamic_sm_power_save)
                    .bits(BitRange::new(&bytes, 45, 1))
                    .build(),
                Field::builder()
                    .title("Punctured Sounding Support")
                    .value(self.punctured_sounding_support)
                    .bits(BitRange::new(&bytes, 46, 1))
                    .build(),
                Field::builder()
                    .title("HT and VHT Trigger Frame RX Support")
                    .value(self.ht_and_vht_trigger_frame_rx_support)
                    .bits(BitRange::new(&bytes, 47, 1))
                    .build(),
            ])
            .build()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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
    pub triggered_su_beamforming_feedback: bool,
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
    pub power_boost_factor_support: bool,
    #[deku(bits = 1)]
    pub he_su_ppdu_and_he_mu_ppdu_with_4x_he_ltf_and_zero_point_eighty_microseconds_gi: bool,
    #[deku(bits = 3)]
    pub max_nc: u8,
    #[deku(bits = 1)]
    pub stbc_tx_greater_than_eighty_mhz: bool,
    #[deku(bits = 1)]
    pub stbc_rx_greater_than_eighty_mhz: bool,
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

impl HePhyCapabilitiesInformation {
    pub fn to_field(&self) -> Field {
        let bytes = self.to_bytes().unwrap_or_default();
        Field::builder()
            .title("HE PHY Capabilities Information")
            .value("")
            .bytes(bytes.clone())
            .subfields([
                Field::reserved(BitRange::new(&bytes, 0, 1)),
                Field::builder()
                    .title("Supported Channel Width Set")
                    .value(self.supported_channel_width_set)
                    .bits(BitRange::new(&bytes, 1, 7))
                    .build(),
                Field::builder()
                    .title("Punctured Preamble RX")
                    .value(self.punctured_preamble_rx)
                    .bits(BitRange::new(&bytes, 8, 4))
                    .build(),
                Field::builder()
                    .title("Device Class")
                    .value(self.device_class)
                    .bits(BitRange::new(&bytes, 12, 1))
                    .build(),
                Field::builder()
                    .title("LDPC Coding in Payload")
                    .value(self.ldpc_coding_in_payload)
                    .bits(BitRange::new(&bytes, 13, 1))
                    .build(),
                Field::builder()
                    .title("HE SU PPDU with 1x HE-LTF and 0.8 µs GI")
                    .value(self.he_ssu_ppdu_with_1x_he_ltf_and_zero_point_eight_microseconds_gi)
                    .bits(BitRange::new(&bytes, 14, 1))
                    .build(),
                Field::builder()
                    .title("Midamble Tx/Rx Max NSTS")
                    .value(self.midamble_tx_rx_max_nsts)
                    .bits(BitRange::new(&bytes, 15, 2))
                    .build(),
                Field::builder()
                    .title("NDP with 4x HE-LTF and 3.2µs GI")
                    .value(self.ndp_with_4x_he_ltf_and_three_point_two_microseconds_gi)
                    .bits(BitRange::new(&bytes, 17, 1))
                    .build(),
                Field::builder()
                    .title("STBC Tx ≤ 80 MHz")
                    .value(self.stbc_tx_less_than_or_equal_to_eighty_mhz)
                    .bits(BitRange::new(&bytes, 18, 1))
                    .build(),
                Field::builder()
                    .title("STBC Rx ≤ 80 MHz")
                    .value(self.stbc_rx_less_than_or_equal_to_eighty_mhz)
                    .bits(BitRange::new(&bytes, 19, 1))
                    .build(),
                Field::builder()
                    .title("Doppler Tx")
                    .value(self.doppler_tx)
                    .bits(BitRange::new(&bytes, 20, 1))
                    .build(),
                Field::builder()
                    .title("Doppler Rx")
                    .value(self.doppler_rx)
                    .bits(BitRange::new(&bytes, 21, 1))
                    .build(),
                Field::builder()
                    .title("Full Bandwidth UL MU-MIMO")
                    .value(self.full_bandwidth_ul_mu_mimo)
                    .bits(BitRange::new(&bytes, 22, 1))
                    .build(),
                Field::builder()
                    .title("Partial Bandwidth UL MU-MIMO")
                    .value(self.partial_bandwidth_ul_mu_mimo)
                    .bits(BitRange::new(&bytes, 23, 1))
                    .build(),
                Field::builder()
                    .title("DCM Max Constellation Tx")
                    .value(self.dcm_max_constellation_tx)
                    .bits(BitRange::new(&bytes, 24, 2))
                    .build(),
                Field::builder()
                    .title("DCM Max NSS Tx")
                    .value(self.dcm_max_nss_tx)
                    .bits(BitRange::new(&bytes, 26, 1))
                    .build(),
                Field::builder()
                    .title("DCM Max Constellation Rx")
                    .value(self.dcm_max_constellation_rx)
                    .bits(BitRange::new(&bytes, 27, 2))
                    .build(),
                Field::builder()
                    .title("DCM Max NSS Rx")
                    .value(self.dcm_max_nss_rx)
                    .bits(BitRange::new(&bytes, 29, 1))
                    .build(),
                Field::builder()
                    .title("Rx Partial BW SU in 20 MHz HE MU PPDU")
                    .value(self.rx_partial_bw_su_in_twenty_mhz_he_mu_ppdu)
                    .bits(BitRange::new(&bytes, 30, 1))
                    .build(),
                Field::builder()
                    .title("SU Beamformer")
                    .value(self.su_beamformer)
                    .bits(BitRange::new(&bytes, 31, 1))
                    .build(),
                Field::builder()
                    .title("SU Beamformee")
                    .value(self.su_beamformee)
                    .bits(BitRange::new(&bytes, 32, 1))
                    .build(),
                Field::builder()
                    .title("MU Beamformer")
                    .value(self.mu_beamformer)
                    .bits(BitRange::new(&bytes, 33, 1))
                    .build(),
                Field::builder()
                    .title("Beamformee STS ≤ 80 MHz")
                    .value(self.beamformee_sts_less_than_or_equal_to_eighty_mhz)
                    .bits(BitRange::new(&bytes, 34, 3))
                    .build(),
                Field::builder()
                    .title("Beamformee STS > 80 MHz")
                    .value(self.beamformee_sts_greater_than_eighty_mhz)
                    .bits(BitRange::new(&bytes, 37, 3))
                    .build(),
                Field::builder()
                    .title("Number of Sounding Dimensions ≤ 80 MHz")
                    .value(self.number_of_sounding_dimensions_less_than_or_equal_to_eighty_mhz)
                    .bits(BitRange::new(&bytes, 40, 3))
                    .build(),
                Field::builder()
                    .title("Number of Sounding Dimensions > 80 MHz")
                    .value(self.number_of_sounding_dimensions_greater_than_eighty_mhz)
                    .bits(BitRange::new(&bytes, 43, 3))
                    .build(),
                Field::builder()
                    .title("Ng = 16 SU Feedback")
                    .value(self.ng_equals_sixteen_su_feedback)
                    .bits(BitRange::new(&bytes, 46, 1))
                    .build(),
                Field::builder()
                    .title("Ng = 16 MU Feedback")
                    .value(self.ng_equals_sixteen_mu_feedback)
                    .bits(BitRange::new(&bytes, 47, 1))
                    .build(),
                Field::builder()
                    .title("Codebook Size (ϕ, ψ) = {4, 2} SU Feedback")
                    .value(self.codebook_size_su_feedback)
                    .bits(BitRange::new(&bytes, 48, 1))
                    .build(),
                Field::builder()
                    .title("Codebook Size (ϕ, ψ) = {7, 5} MU Feedback")
                    .value(self.codebook_size_mu_feedback)
                    .bits(BitRange::new(&bytes, 49, 1))
                    .build(),
                Field::builder()
                    .title("Triggered SU Beamforming Feedback")
                    .value(self.triggered_su_beamforming_feedback)
                    .bits(BitRange::new(&bytes, 50, 1))
                    .build(),
                Field::builder()
                    .title("Triggered MU Beamforming Partial BW Feedback")
                    .value(self.triggered_mu_beamforming_partial_bw_feedback)
                    .bits(BitRange::new(&bytes, 51, 1))
                    .build(),
                Field::builder()
                    .title("Triggered CQI Feedback")
                    .value(self.triggered_cqi_feedback)
                    .bits(BitRange::new(&bytes, 52, 1))
                    .build(),
                Field::builder()
                    .title("Partial Bandwidth Extended Range")
                    .value(self.partial_bandwidth_extended_range)
                    .bits(BitRange::new(&bytes, 53, 1))
                    .build(),
                Field::builder()
                    .title("Partial Bandwidth DL MU-MIMO")
                    .value(self.partial_bandwidth_dl_mu_mimo)
                    .bits(BitRange::new(&bytes, 54, 1))
                    .build(),
                Field::builder()
                    .title("PPE Thresholds Present")
                    .value(self.ppe_thresholds_present)
                    .bits(BitRange::new(&bytes, 55, 1))
                    .build(),
                Field::builder()
                    .title("PSR-based SR Support")
                    .value(self.psr_based_sr_support)
                    .bits(BitRange::new(&bytes, 56, 1))
                    .build(),
                Field::builder()
                    .title("Power Boost Factor Support")
                    .value(self.power_boost_factor_support)
                    .bits(BitRange::new(&bytes, 57, 1))
                    .build(),
                Field::builder()
                    .title("HE SU/MU PPDU with 4x HE-LTF and 0.8 µs GI")
                    .value(self.he_su_ppdu_and_he_mu_ppdu_with_4x_he_ltf_and_zero_point_eighty_microseconds_gi)
                    .bits(BitRange::new(&bytes, 58, 1))
                    .build(),
                Field::builder()
                    .title("Max Nc")
                    .value(self.max_nc)
                    .bits(BitRange::new(&bytes, 59, 3))
                    .build(),
                Field::builder()
                    .title("STBC Tx > 80 MHz")
                    .value(self.stbc_tx_greater_than_eighty_mhz)
                    .bits(BitRange::new(&bytes, 62, 1))
                    .build(),
                Field::builder()
                    .title("STBC Rx > 80 MHz")
                    .value(self.stbc_rx_greater_than_eighty_mhz)
                    .bits(BitRange::new(&bytes, 63, 1))
                    .build(),
                Field::builder()
                    .title("HE ER SU PPDU with 4x HE-LTF and 0.8 µs GI")
                    .value(self.he_er_su_ppdu_with_4x_he_ltf_and_zero_point_eight_microseconds_gi)
                    .bits(BitRange::new(&bytes, 64, 1))
                    .build(),
                Field::builder()
                    .title("20 MHz in 40 MHz HE PPDU in 2.4 GHz Band")
                    .value(self.twenty_mhz_in_forty_mhz_he_ppdu_in_two_point_four_ghz_band)
                    .bits(BitRange::new(&bytes, 65, 1))
                    .build(),
                Field::builder()
                    .title("20 MHz in 160 MHz HE PPDU")
                    .value(self.twenty_mhz_in_one_hundred_sixty_mhz_he_ppdu)
                    .bits(BitRange::new(&bytes, 66, 1))
                    .build(),
                Field::builder()
                    .title("80 MHz in 160 MHz HE PPDU")
                    .value(self.eighty_mhz_in_one_hundred_sixty_mhz_he_ppdu)
                    .bits(BitRange::new(&bytes, 67, 1))
                    .build(),
                Field::builder()
                    .title("HE ER SU PPDU with 1x HE-LTF and 0.8 µs GI")
                    .value(self.he_er_su_ppdu_with_1x_he_ltf_and_zero_point_eighty_microseconds_gi)
                    .bits(BitRange::new(&bytes, 68, 1))
                    .build(),
                Field::builder()
                    .title("Midamble Tx/Rx 2x and 1x HE-LTF")
                    .value(self.midamble_tx_rx_2x_and_1x_he_ltf)
                    .bits(BitRange::new(&bytes, 69, 1))
                    .build(),
                Field::builder()
                    .title("DCM Max RU")
                    .value(self.dcm_max_ru)
                    .bits(BitRange::new(&bytes, 70, 2))
                    .build(),
                Field::builder()
                    .title("Longer Than 16 HE-SIG-B OFDM Symbols Support")
                    .value(self.longer_than_sixteen_he_sig_b_ofdm_symbols_present)
                    .bits(BitRange::new(&bytes, 72, 1))
                    .build(),
                Field::builder()
                    .title("Non-Triggered CQI Feedback")
                    .value(self.non_triggered_cqi_feedback)
                    .bits(BitRange::new(&bytes, 73, 1))
                    .build(),
                Field::builder()
                    .title("Tx 1024-QAM < 242-tone RU Support")
                    .value(self.tx_one_thousand_twenty_four_qam_less_than_two_hundred_forty_two_tone_ru_support)
                    .bits(BitRange::new(&bytes, 74, 1))
                    .build(),
                Field::builder()
                    .title("Rx 1024-QAM < 242-tone RU Support")
                    .value(self.rx_one_thousand_twenty_four_qam_less_than_two_hundred_forty_two_tone_ru_support)
                    .bits(BitRange::new(&bytes, 75, 1))
                    .build(),
                Field::builder()
                    .title("Rx Full BW SU Using HE MU PPDU with Compressed HE-SIG-B")
                    .value(self.rx_full_bw_su_using_he_mu_ppdu_with_compressed_he_sig_b)
                    .bits(BitRange::new(&bytes, 76, 1))
                    .build(),
                Field::builder()
                    .title("Rx Full BW SU Using HE MU PPDU with Non-Compressed HE-SIG-B")
                    .value(self.rx_full_bw_su_using_he_mu_ppdu_with_non_compressed_he_sig_b)
                    .bits(BitRange::new(&bytes, 77, 1))
                    .build(),
                Field::builder()
                    .title("Nominal Packet Padding")
                    .value(self.nominal_packet_padding)
                    .bits(BitRange::new(&bytes, 78, 2))
                    .build(),
                Field::builder()
                    .title("HE MU PPDU With More Than One RU Rx Max N_HE-LTF")
                    .value(self.he_mu_ppdu_with_more_than_one_ru_rx_max_n_he_ltf)
                    .bits(BitRange::new(&bytes, 80, 1))
                    .build(),
                Field::reserved(BitRange::new(&bytes, 81, 7)),
            ])
            .build()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "supported_width_set: deku::bitvec::BitVec<u8, Lsb0>")]
pub struct SupportedHeMcsAndNssSet {
    pub rx_he_mcs_map_less_than_or_equal_to_eighty_mhz: HeMcsMap,
    pub tx_he_mcs_map_less_than_or_equal_to_eighty_mhz: HeMcsMap,
    #[deku(cond = "supported_width_set[2]")]
    pub rx_he_mcs_map_one_hundred_sixty_mhz: Option<HeMcsMap>,
    #[deku(cond = "supported_width_set[2]")]
    pub tx_he_mcs_map_one_hundred_sixty_mhz: Option<HeMcsMap>,
    #[deku(cond = "supported_width_set[3]")]
    pub rx_he_mcs_map_eighty_plus_eighty_mhz: Option<HeMcsMap>,
    #[deku(cond = "supported_width_set[3]")]
    pub tx_he_mcs_map_eighty_plus_eighty_mhz: Option<HeMcsMap>,
}

impl SupportedHeMcsAndNssSet {
    pub fn to_field(&self, supported_width_set: u8) -> Field {
        use std::io::Cursor;

        // Get bytes by writing with context
        let bytes = {
            let mut cursor = Cursor::new(Vec::new());
            let mut writer = Writer::new(&mut cursor);
            let ctx = BitVec::<u8, Lsb0>::from_element(supported_width_set);
            let _ = self.to_writer(&mut writer, ctx);
            cursor.into_inner()
        };

        let mut subfields = vec![
            self.rx_he_mcs_map_less_than_or_equal_to_eighty_mhz
                .to_field("Rx HE-MCS Map ≤ 80 MHz"),
            self.tx_he_mcs_map_less_than_or_equal_to_eighty_mhz
                .to_field("Tx HE-MCS Map ≤ 80 MHz"),
        ];

        if let Some(map) = self.rx_he_mcs_map_one_hundred_sixty_mhz {
            subfields.push(map.to_field("Rx HE-MCS Map 160 MHz"));
        }

        if let Some(map) = self.tx_he_mcs_map_one_hundred_sixty_mhz {
            subfields.push(map.to_field("Tx HE-MCS Map 160 MHz"));
        }

        if let Some(map) = self.rx_he_mcs_map_eighty_plus_eighty_mhz {
            subfields.push(map.to_field("Rx HE-MCS Map 80+80 MHz"));
        }

        if let Some(map) = self.tx_he_mcs_map_eighty_plus_eighty_mhz {
            subfields.push(map.to_field("Tx HE-MCS Map 80+80 MHz"));
        }

        Field::builder()
            .title("Supported HE-MCS And NSS Set")
            .value("")
            .bytes(bytes)
            .subfields(subfields)
            .build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct HeMcsMap {
    #[deku(
        bits = 2,
        map = "|value: u8| Support::try_from(value).map_err(|_| DekuError::Parse(\"Invalid Support\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*max_he_mcs_for_one_ss), 2)"
    )]
    pub max_he_mcs_for_one_ss: Support,
    #[deku(
        bits = 2,
        map = "|value: u8| Support::try_from(value).map_err(|_| DekuError::Parse(\"Invalid Support\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*max_he_mcs_for_two_ss), 2)"
    )]
    pub max_he_mcs_for_two_ss: Support,
    #[deku(
        bits = 2,
        map = "|value: u8| Support::try_from(value).map_err(|_| DekuError::Parse(\"Invalid Support\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*max_he_mcs_for_three_ss), 2)"
    )]
    pub max_he_mcs_for_three_ss: Support,
    #[deku(
        bits = 2,
        map = "|value: u8| Support::try_from(value).map_err(|_| DekuError::Parse(\"Invalid Support\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*max_he_mcs_for_four_ss), 2)"
    )]
    pub max_he_mcs_for_four_ss: Support,
    #[deku(
        bits = 2,
        map = "|value: u8| Support::try_from(value).map_err(|_| DekuError::Parse(\"Invalid Support\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*max_he_mcs_for_five_ss), 2)"
    )]
    pub max_he_mcs_for_five_ss: Support,
    #[deku(
        bits = 2,
        map = "|value: u8| Support::try_from(value).map_err(|_| DekuError::Parse(\"Invalid Support\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*max_he_mcs_for_six_ss), 2)"
    )]
    pub max_he_mcs_for_six_ss: Support,
    #[deku(
        bits = 2,
        map = "|value: u8| Support::try_from(value).map_err(|_| DekuError::Parse(\"Invalid Support\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*max_he_mcs_for_seven_ss), 2)"
    )]
    pub max_he_mcs_for_seven_ss: Support,
    #[deku(
        bits = 2,
        map = "|value: u8| Support::try_from(value).map_err(|_| DekuError::Parse(\"Invalid Support\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*max_he_mcs_for_eight_ss), 2)"
    )]
    pub max_he_mcs_for_eight_ss: Support,
}

impl HeMcsMap {
    /// Returns the maximum number of spatial streams supported.
    pub fn max_spatial_streams(&self) -> u8 {
        for (support, stream) in [
            (self.max_he_mcs_for_eight_ss, 8),
            (self.max_he_mcs_for_seven_ss, 7),
            (self.max_he_mcs_for_six_ss, 6),
            (self.max_he_mcs_for_five_ss, 5),
            (self.max_he_mcs_for_four_ss, 4),
            (self.max_he_mcs_for_three_ss, 3),
            (self.max_he_mcs_for_two_ss, 2),
            (self.max_he_mcs_for_one_ss, 1),
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
            1 => self.max_he_mcs_for_one_ss,
            2 => self.max_he_mcs_for_two_ss,
            3 => self.max_he_mcs_for_three_ss,
            4 => self.max_he_mcs_for_four_ss,
            5 => self.max_he_mcs_for_five_ss,
            6 => self.max_he_mcs_for_six_ss,
            7 => self.max_he_mcs_for_seven_ss,
            8 => self.max_he_mcs_for_eight_ss,
            _ => Support::NotSupported,
        };

        match support {
            Support::HeMcsZeroThroughSeven => Some(7),
            Support::HeMcsZeroThroughNine => Some(9),
            Support::HeMcsZeroThroughEleven => Some(11),
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
                    .title("Max HE-MCS for 1 SS")
                    .value(self.max_he_mcs_for_one_ss)
                    .units(format!("({})", u8::from(self.max_he_mcs_for_one_ss)))
                    .bits(BitRange::new(&bytes, 0, 2))
                    .build(),
                Field::builder()
                    .title("Max HE-MCS for 2 SS")
                    .value(self.max_he_mcs_for_two_ss)
                    .units(format!("({})", u8::from(self.max_he_mcs_for_two_ss)))
                    .bits(BitRange::new(&bytes, 2, 2))
                    .build(),
                Field::builder()
                    .title("Max HE-MCS for 3 SS")
                    .value(self.max_he_mcs_for_three_ss)
                    .units(format!("({})", u8::from(self.max_he_mcs_for_three_ss)))
                    .bits(BitRange::new(&bytes, 4, 2))
                    .build(),
                Field::builder()
                    .title("Max HE-MCS for 4 SS")
                    .value(self.max_he_mcs_for_four_ss)
                    .units(format!("({})", u8::from(self.max_he_mcs_for_four_ss)))
                    .bits(BitRange::new(&bytes, 6, 2))
                    .build(),
                Field::builder()
                    .title("Max HE-MCS for 5 SS")
                    .value(self.max_he_mcs_for_five_ss)
                    .units(format!("({})", u8::from(self.max_he_mcs_for_five_ss)))
                    .bits(BitRange::new(&bytes, 8, 2))
                    .build(),
                Field::builder()
                    .title("Max HE-MCS for 6 SS")
                    .value(self.max_he_mcs_for_six_ss)
                    .units(format!("({})", u8::from(self.max_he_mcs_for_six_ss)))
                    .bits(BitRange::new(&bytes, 10, 2))
                    .build(),
                Field::builder()
                    .title("Max HE-MCS for 7 SS")
                    .value(self.max_he_mcs_for_seven_ss)
                    .units(format!("({})", u8::from(self.max_he_mcs_for_seven_ss)))
                    .bits(BitRange::new(&bytes, 12, 2))
                    .build(),
                Field::builder()
                    .title("Max HE-MCS for 8 SS")
                    .value(self.max_he_mcs_for_eight_ss)
                    .units(format!("({})", u8::from(self.max_he_mcs_for_eight_ss)))
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
    HeMcsZeroThroughSeven = 0,
    HeMcsZeroThroughNine,
    HeMcsZeroThroughEleven,
    NotSupported,
}

impl Display for Support {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HeMcsZeroThroughSeven => write!(f, "HE-MCS 0-7"),
            Self::HeMcsZeroThroughNine => write!(f, "HE-MCS 0-9"),
            Self::HeMcsZeroThroughEleven => write!(f, "HE-MCS 0-11"),
            Self::NotSupported => write!(f, "Not Supported"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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

impl PpeThresholds {
    pub fn to_field(&self) -> Field {
        let bytes = self.to_bytes().unwrap_or_default();
        let mut subfields = vec![
            Field::builder()
                .title("NSTS")
                .value(self.nsts)
                .bits(BitRange::from_byte(*bytes.get(0).unwrap_or(&0), 0, 3))
                .build(),
            Field::builder()
                .title("RU Index Bitmask")
                .value(self.ru_index_bitmask)
                .bits(BitRange::from_byte(*bytes.get(0).unwrap_or(&0), 3, 4))
                .build(),
        ];
        for (i, ppe_threshold_info) in self.ppe_thresholds_info.iter().enumerate() {
            subfields
                .push(ppe_threshold_info.to_field(&format!("NSS {}", i), self.ru_index_bitmask));
        }
        Field::builder()
            .title("PPE Thresholds")
            .value("")
            .bytes(bytes)
            .subfields(subfields)
            .build()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(
    bit_order = "lsb",
    ctx = "bit_order: deku::ctx::Order, ru_index_bitmask: &BitSlice<u8, Lsb0>"
)]
pub struct PpeThresholdsInfo {
    #[deku(
        cond = "ru_index_bitmask[0]",
        bits = 3,
        map = "|value: u8| TransmissionConstellation::try_from(value).map(Some).map_err(|_| DekuError::Parse(\"Invalid TransmissionConstellation\".into()))",
        writer = "if let Some(val) = ppet_sixteen_nsts_ru_0 { write_bits_lsb0(deku::writer, u8::from(*val), 3) } else { Ok(()) }"
    )]
    pub ppet_sixteen_nsts_ru_0: Option<TransmissionConstellation>,
    #[deku(
        cond = "ru_index_bitmask[0]",
        bits = 3,
        map = "|value: u8| TransmissionConstellation::try_from(value).map(Some).map_err(|_| DekuError::Parse(\"Invalid TransmissionConstellation\".into()))",
        writer = "if let Some(val) = ppet_eight_nsts_ru_0 { write_bits_lsb0(deku::writer, u8::from(*val), 3) } else { Ok(()) }"
    )]
    pub ppet_eight_nsts_ru_0: Option<TransmissionConstellation>,
    #[deku(
        cond = "ru_index_bitmask[1]",
        bits = 3,
        map = "|value: u8| TransmissionConstellation::try_from(value).map(Some).map_err(|_| DekuError::Parse(\"Invalid TransmissionConstellation\".into()))",
        writer = "if let Some(val) = ppet_sixteen_nsts_ru_1 { write_bits_lsb0(deku::writer, u8::from(*val), 3) } else { Ok(()) }"
    )]
    pub ppet_sixteen_nsts_ru_1: Option<TransmissionConstellation>,
    #[deku(
        cond = "ru_index_bitmask[1]",
        bits = 3,
        map = "|value: u8| TransmissionConstellation::try_from(value).map(Some).map_err(|_| DekuError::Parse(\"Invalid TransmissionConstellation\".into()))",
        writer = "if let Some(val) = ppet_eight_nsts_ru_1 { write_bits_lsb0(deku::writer, u8::from(*val), 3) } else { Ok(()) }"
    )]
    pub ppet_eight_nsts_ru_1: Option<TransmissionConstellation>,
    #[deku(
        cond = "ru_index_bitmask[2]",
        bits = 3,
        map = "|value: u8| TransmissionConstellation::try_from(value).map(Some).map_err(|_| DekuError::Parse(\"Invalid TransmissionConstellation\".into()))",
        writer = "if let Some(val) = ppet_sixteen_nsts_ru_2 { write_bits_lsb0(deku::writer, u8::from(*val), 3) } else { Ok(()) }"
    )]
    pub ppet_sixteen_nsts_ru_2: Option<TransmissionConstellation>,
    #[deku(
        cond = "ru_index_bitmask[2]",
        bits = 3,
        map = "|value: u8| TransmissionConstellation::try_from(value).map(Some).map_err(|_| DekuError::Parse(\"Invalid TransmissionConstellation\".into()))",
        writer = "if let Some(val) = ppet_eight_nsts_ru_2 { write_bits_lsb0(deku::writer, u8::from(*val), 3) } else { Ok(()) }"
    )]
    pub ppet_eight_nsts_ru_2: Option<TransmissionConstellation>,
    #[deku(
        cond = "ru_index_bitmask[3]",
        bits = 3,
        map = "|value: u8| TransmissionConstellation::try_from(value).map(Some).map_err(|_| DekuError::Parse(\"Invalid TransmissionConstellation\".into()))",
        writer = "if let Some(val) = ppet_sixteen_nsts_ru_3 { write_bits_lsb0(deku::writer, u8::from(*val), 3) } else { Ok(()) }"
    )]
    pub ppet_sixteen_nsts_ru_3: Option<TransmissionConstellation>,
    #[deku(
        cond = "ru_index_bitmask[3]",
        bits = 3,
        map = "|value: u8| TransmissionConstellation::try_from(value).map(Some).map_err(|_| DekuError::Parse(\"Invalid TransmissionConstellation\".into()))",
        writer = "if let Some(val) = ppet_eight_nsts_ru_3 { write_bits_lsb0(deku::writer, u8::from(*val), 3) } else { Ok(()) }"
    )]
    pub ppet_eight_nsts_ru_3: Option<TransmissionConstellation>,
}

impl PpeThresholdsInfo {
    pub fn to_field(&self, title: &str, ru_index_bitmask: u8) -> Field {
        use std::io::Cursor;

        // Get bytes by writing with context
        let bytes = {
            let mut cursor = Cursor::new(Vec::new());
            let mut writer = Writer::new(&mut cursor);
            let ctx = (deku::ctx::Order::Lsb0, ru_index_bitmask.view_bits::<Lsb0>());
            let _ = self.to_writer(&mut writer, ctx);
            cursor.into_inner()
        };

        let mut bit_offset = 0;
        let mut subfields = Vec::new();
        if let Some(ppet_sixteen) = self.ppet_sixteen_nsts_ru_0 {
            subfields.push(
                Field::builder()
                    .title("RU 242 PPET16")
                    .value(ppet_sixteen)
                    .units(format!("({})", u8::from(ppet_sixteen)))
                    .bits(BitRange::new(&bytes, bit_offset, 3))
                    .build(),
            );
            bit_offset += 3;
        }
        if let Some(ppet_eight) = self.ppet_eight_nsts_ru_0 {
            subfields.push(
                Field::builder()
                    .title("RU 242 PPET8")
                    .value(ppet_eight)
                    .units(format!("({})", u8::from(ppet_eight)))
                    .bits(BitRange::new(&bytes, bit_offset, 3))
                    .build(),
            );
            bit_offset += 3;
        }
        if let Some(ppet_sixteen) = self.ppet_sixteen_nsts_ru_1 {
            subfields.push(
                Field::builder()
                    .title("RU 484 PPET16")
                    .value(ppet_sixteen)
                    .units(format!("({})", u8::from(ppet_sixteen)))
                    .bits(BitRange::new(&bytes, bit_offset, 3))
                    .build(),
            );
            bit_offset += 3;
        }
        if let Some(ppet_eight) = self.ppet_eight_nsts_ru_1 {
            subfields.push(
                Field::builder()
                    .title("RU 484 PPET8")
                    .value(ppet_eight)
                    .units(format!("({})", u8::from(ppet_eight)))
                    .bits(BitRange::new(&bytes, bit_offset, 3))
                    .build(),
            );
            bit_offset += 3;
        }
        if let Some(ppet_sixteen) = self.ppet_sixteen_nsts_ru_2 {
            subfields.push(
                Field::builder()
                    .title("RU 996 PPET16")
                    .value(ppet_sixteen)
                    .units(format!("({})", u8::from(ppet_sixteen)))
                    .bits(BitRange::new(&bytes, bit_offset, 3))
                    .build(),
            );
            bit_offset += 3;
        }
        if let Some(ppet_eight) = self.ppet_eight_nsts_ru_2 {
            subfields.push(
                Field::builder()
                    .title("RU 996 PPET8")
                    .value(ppet_eight)
                    .units(format!("({})", u8::from(ppet_eight)))
                    .bits(BitRange::new(&bytes, bit_offset, 3))
                    .build(),
            );
            bit_offset += 3;
        }
        if let Some(ppet_sixteen) = self.ppet_sixteen_nsts_ru_3 {
            subfields.push(
                Field::builder()
                    .title("RU 2x996 PPET16")
                    .value(ppet_sixteen)
                    .units(format!("({})", u8::from(ppet_sixteen)))
                    .bits(BitRange::new(&bytes, bit_offset, 3))
                    .build(),
            );
            bit_offset += 3;
        }
        if let Some(ppet_eight) = self.ppet_eight_nsts_ru_3 {
            subfields.push(
                Field::builder()
                    .title("RU 2x996 PPET8")
                    .value(ppet_eight)
                    .units(format!("({})", u8::from(ppet_eight)))
                    .bits(BitRange::new(&bytes, bit_offset, 3))
                    .build(),
            );
        }
        Field::builder()
            .title(title)
            .value("")
            .bytes(bytes)
            .subfields(subfields)
            .build()
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum TransmissionConstellation {
    Bpsk = 0,
    Qpsk = 1,
    SixteenQam = 2,
    SixtyFourQam = 3,
    TwoHundredFiftySixQam = 4,
    OneThousandTwentyFourQam = 5,
    Reserved = 6,
    None = 7,
}

impl Display for TransmissionConstellation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bpsk => write!(f, "BPSK"),
            Self::Qpsk => write!(f, "QPSK"),
            Self::SixteenQam => write!(f, "16-QAM"),
            Self::SixtyFourQam => write!(f, "64-QAM"),
            Self::TwoHundredFiftySixQam => write!(f, "256-QAM"),
            Self::OneThousandTwentyFourQam => write!(f, "1024-QAM"),
            Self::Reserved => write!(f, "Reserved"),
            Self::None => write!(f, "None"),
        }
    }
}
