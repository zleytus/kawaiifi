use std::fmt::Display;
use std::io::Cursor;

use deku::{
    DekuContainerWrite, DekuError, DekuRead, DekuReader, DekuWrite, DekuWriter,
    bitvec::{BitSlice, BitView, Lsb0},
    reader::Reader,
    writer::Writer,
};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

use super::HeCapabilities;
use crate::ChannelWidth;
use crate::ies::{BitRange, Field, IeId, write_bits_lsb0};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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
            12 => 9.0,      // 4096-QAM 3/4 (EHT introduces 4096-QAM)
            13 => 10.0,     // 4096-QAM 5/6
            _ => return 0.0,
        };

        // Rate formula
        (data_subcarriers * bits_per_symbol * f64::from(self.max_spatial_streams()))
            / f64::from(symbol_duration_us)
    }

    fn max_spatial_streams(&self) -> u8 {
        self.supported_eht_mcs_and_nss_set
            .as_ref()
            .map(|mcs_nss_set| {
                mcs_nss_set
                    .eht_mcs_map_bw_lte_80_mhz_except_20_mhz_only_non_ap_sta
                    .max_spatial_streams()
            })
            .unwrap_or(1)
    }

    fn min_guard_interval_us(&self) -> f32 {
        0.8
    }

    /// Get maximum MCS for a given spatial stream
    fn max_mcs_for_stream(&self, stream: u8) -> Option<u8> {
        self.supported_eht_mcs_and_nss_set
            .as_ref()
            .and_then(|mcs_nss_set| {
                mcs_nss_set
                    .eht_mcs_map_bw_lte_80_mhz_except_20_mhz_only_non_ap_sta
                    .max_mcs_for_stream(stream)
            })
    }

    /// Get the highest MCS supported across all streams
    fn max_mcs(&self) -> u8 {
        let max_streams = self.max_spatial_streams();
        self.max_mcs_for_stream(max_streams).unwrap_or(0)
    }

    pub fn summary(&self) -> String {
        let max_spatial_streams = self.max_spatial_streams();
        if max_spatial_streams == 1 {
            "1 Spatial Stream".to_string()
        } else {
            format!("{} Spatial Streams", max_spatial_streams)
        }
    }

    pub fn fields(&self) -> Vec<Field> {
        let mut fields = vec![
            self.eht_mac_capabilities_information.to_field(),
            self.eht_phy_capabilities_information.to_field(),
        ];

        if let Some(supported_eht_mcs_and_nss_set) = &self.supported_eht_mcs_and_nss_set {
            fields.push(supported_eht_mcs_and_nss_set.to_field());
        }

        if let Some(eht_ppe_thresholds) = &self.eht_ppe_thresholds {
            fields.push(eht_ppe_thresholds.to_field());
        }

        fields
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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

impl EhtMacCapabilitiesInformation {
    pub fn to_field(&self) -> Field {
        let bytes = self.to_bytes().unwrap_or_default();

        let maximum_mpdu_length_value = match self.maximum_mpdu_length {
            0 => "3895",
            1 => "7991",
            2 => "11454",
            _ => "Reserved",
        };

        let eht_link_adaptation_support_value = match self.eht_link_adaptation_support {
            0 => "Not Supported",
            1 => "Reserved",
            2 => "Unsolicited",
            3 => "Solicited and Unsolicited",
            _ => "Unknown",
        };

        Field::builder()
            .title("EHT MAC Capabilities Information")
            .value("")
            .subfields([
                Field::builder()
                    .title("EPCS Priority Access Support")
                    .value(self.epcs_priority_access_support)
                    .bits(BitRange::new(&bytes, 0, 1))
                    .build(),
                Field::builder()
                    .title("EHT OM Control Support")
                    .value(self.eht_om_control_support)
                    .bits(BitRange::new(&bytes, 1, 1))
                    .build(),
                Field::builder()
                    .title("TXS Mode 1 Support")
                    .value(self.txs_mode_1_support)
                    .bits(BitRange::new(&bytes, 2, 1))
                    .build(),
                Field::builder()
                    .title("TXS Mode 2 Support")
                    .value(self.txs_mode_2_support)
                    .bits(BitRange::new(&bytes, 3, 1))
                    .build(),
                Field::builder()
                    .title("Restricted TWT Support")
                    .value(self.restricted_twt_support)
                    .bits(BitRange::new(&bytes, 4, 1))
                    .build(),
                Field::builder()
                    .title("SCS Traffic Description Support")
                    .value(self.scs_traffic_description_support)
                    .bits(BitRange::new(&bytes, 5, 1))
                    .build(),
                Field::builder()
                    .title("Maximum MPDU Length")
                    .value(maximum_mpdu_length_value)
                    .bits(BitRange::new(&bytes, 6, 2))
                    .build(),
                Field::builder()
                    .title("Maximum A-MPDU Length Exponent Extension")
                    .value(self.maximum_ampdu_length_exponent_extension)
                    .bits(BitRange::new(&bytes, 8, 1))
                    .build(),
                Field::builder()
                    .title("EHT TRS Support")
                    .value(self.eht_trs_support)
                    .bits(BitRange::new(&bytes, 9, 1))
                    .build(),
                Field::builder()
                    .title("TXOP Return Support in TXS Mode 2")
                    .value(self.txop_return_support_in_txs_mode_2)
                    .bits(BitRange::new(&bytes, 10, 1))
                    .build(),
                Field::builder()
                    .title("Two BQRs Support")
                    .value(self.two_bqrs_support)
                    .bits(BitRange::new(&bytes, 11, 1))
                    .build(),
                Field::builder()
                    .title("EHT Link Adaptation Support")
                    .value(eht_link_adaptation_support_value)
                    .bits(BitRange::new(&bytes, 12, 2))
                    .build(),
                Field::builder()
                    .title("Unsolicited EPCS Priority Access Parameter Update")
                    .value(self.unsolicited_epcs_priority_access_parameter_update)
                    .bits(BitRange::new(&bytes, 14, 1))
                    .build(),
                Field::reserved(BitRange::new(&bytes, 15, 1)),
            ])
            .build()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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

impl EhtPhyCapabilitiesInformation {
    pub fn to_field(&self) -> Field {
        let bytes = self.to_bytes().unwrap_or_default();

        let common_nominal_packet_padding_value = match self.common_nominal_packet_padding {
            0 => Some(0),
            1 => Some(8),
            2 => Some(16),
            3 => Some(20),
            _ => None,
        };

        Field::builder()
            .title("EHT PHY Capabilities Information")
            .value("")
            .subfields([
                Field::reserved(BitRange::new(&bytes, 0, 1)),
                Field::builder()
                    .title("Support for 320 MHz in 6 GHz")
                    .value(self.support_for_320_mhz_in_6_ghz)
                    .bits(BitRange::new(&bytes, 1, 1))
                    .build(),
                Field::builder()
                    .title("Support for 242-tone RU in BW Wider Than 20 MHz")
                    .value(self.support_for_two_hundred_forty_two_tone_ru_in_bw_wider_than_twenty_mhz)
                    .bits(BitRange::new(&bytes, 2, 1))
                    .build(),
                Field::builder()
                    .title("NDP With 4x EHT-LTF and 3.2 µs GI")
                    .value(self.ndp_with_4x_eht_ltf_and_three_point_two_microseconds_gi)
                    .bits(BitRange::new(&bytes, 3, 1))
                    .build(),
                Field::builder()
                    .title("Partial Bandwidth UL MU-MIMO")
                    .value(self.partial_bandwidth_ul_mu_mimo)
                    .bits(BitRange::new(&bytes, 4, 1))
                    .build(),
                Field::builder()
                    .title("SU Beamformer")
                    .value(self.su_beamformer)
                    .bits(BitRange::new(&bytes, 5, 1))
                    .build(),
                Field::builder()
                    .title("SU Beamformee")
                    .value(self.su_beamformee)
                    .bits(BitRange::new(&bytes, 6, 1))
                    .build(),
                Field::builder()
                    .title("Beamformee SS (≤ 80 MHz)")
                    .value(self.beamformee_ss_less_than_or_equal_to_eighty_mhz)
                    .bits(BitRange::new(&bytes, 7, 3))
                    .build(),
                Field::builder()
                    .title("Beamformee SS (= 160 MHz)")
                    .value(self.beamformee_ss_equal_to_one_hundred_sixty_mhz)
                    .bits(BitRange::new(&bytes, 10, 3))
                    .build(),
                Field::builder()
                    .title("Beamformee SS (= 320 MHz)")
                    .value(self.beamformee_ss_equal_to_three_hundred_twenty_mhz)
                    .bits(BitRange::new(&bytes, 13, 3))
                    .build(),
                Field::builder()
                    .title("Number of Sounding Dimensions (≤ 80 MHz)")
                    .value(self.number_of_sounding_dimensions_less_than_or_equal_to_eighty_mhz)
                    .bits(BitRange::new(&bytes, 16, 3))
                    .build(),
                Field::builder()
                    .title("Number of Sounding Dimensions (= 160 MHz)")
                    .value(self.number_of_sounding_dimensions_equal_to_one_hundred_sixty_mhz)
                    .bits(BitRange::new(&bytes, 19, 3))
                    .build(),
                Field::builder()
                    .title("Number of Sounding Dimensions (= 320 MHz)")
                    .value(self.number_of_sounding_dimensions_equal_to_three_hundred_twenty_mhz)
                    .bits(BitRange::new(&bytes, 22, 3))
                    .build(),
                Field::builder()
                    .title("Ng = 16 SU Feedback")
                    .value(self.ng_equals_sixteen_su_feedback)
                    .bits(BitRange::new(&bytes, 25, 1))
                    .build(),
                Field::builder()
                    .title("Ng = 16 MU Feedback")
                    .value(self.ng_equals_sixteen_mu_feedback)
                    .bits(BitRange::new(&bytes, 26, 1))
                    .build(),
                Field::builder()
                    .title("Codebook Size {4, 2} SU Feedback")
                    .value(self.codebook_size_su_feedback)
                    .bits(BitRange::new(&bytes, 27, 1))
                    .build(),
                Field::builder()
                    .title("Codebook Size {7, 5} MU Feedback")
                    .value(self.codebook_size_mu_feedback)
                    .bits(BitRange::new(&bytes, 28, 1))
                    .build(),
                Field::builder()
                    .title("Triggered SU Beamforming Feedback")
                    .value(self.triggered_su_beamforming_feedback)
                    .bits(BitRange::new(&bytes, 29, 1))
                    .build(),
                Field::builder()
                    .title("Triggered MU Beamforming Partial BW Feedback")
                    .value(self.triggered_mu_beamforming_partial_bw_feedback)
                    .bits(BitRange::new(&bytes, 30, 1))
                    .build(),
                Field::builder()
                    .title("Triggered CQI Feedback")
                    .value(self.triggered_cqi_feedback)
                    .bits(BitRange::new(&bytes, 31, 1))
                    .build(),
                Field::builder()
                    .title("Partial Bandwidth DL MU-MIMO")
                    .value(self.partial_bandwidth_dl_mu_mimo)
                    .bits(BitRange::new(&bytes, 32, 1))
                    .build(),
                Field::builder()
                    .title("EHT PSR-Based SR Support")
                    .value(self.eht_psr_based_sr_support)
                    .bits(BitRange::new(&bytes, 33, 1))
                    .build(),
                Field::builder()
                    .title("Power Boost Factor Support")
                    .value(self.power_boost_factor_support)
                    .bits(BitRange::new(&bytes, 34, 1))
                    .build(),
                Field::builder()
                    .title("EHT MU PPDU With 4x EHT-LTF and 0.8 µs GI")
                    .value(self.eht_mu_ppdu_with_4x_eht_ltf_and_zero_point_eight_microseconds_gi)
                    .bits(BitRange::new(&bytes, 35, 1))
                    .build(),
                Field::builder()
                    .title("Max Nc")
                    .value(self.max_nc)
                    .bits(BitRange::new(&bytes, 36, 4))
                    .build(),
                Field::builder()
                    .title("Non-Triggered CQI Feedback")
                    .value(self.non_triggered_cqi_feedback)
                    .bits(BitRange::new(&bytes, 40, 1))
                    .build(),
                Field::builder()
                    .title("Tx 1024-QAM and 4096-QAM < 242-tone RU Support")
                    .value(self.tx_1024_qam_and_4096_qam_less_than_242_tone_ru_support)
                    .bits(BitRange::new(&bytes, 41, 1))
                    .build(),
                Field::builder()
                    .title("Rx 1024-QAM and 4096-QAM < 242-tone RU Support")
                    .value(self.rx_1024_qam_and_4096_qam_less_than_242_tone_ru_support)
                    .bits(BitRange::new(&bytes, 42, 1))
                    .build(),
                Field::builder()
                    .title("PPE Thresholds Present")
                    .value(self.ppe_thresholds_present)
                    .bits(BitRange::new(&bytes, 43, 1))
                    .build(),
                Field::builder()
                    .title("Common Nominal Packet Padding")
                    .value(common_nominal_packet_padding_value.map(|padding| padding.to_string()).unwrap_or("Unknown".to_string()))
                    .units(if common_nominal_packet_padding_value.is_some() {"µs"} else {""})
                    .bits(BitRange::new(&bytes, 44, 2))
                    .build(),
                Field::builder()
                    .title("Maximum Number of Supported EHT-LTFs")
                    .value(self.maximum_number_of_supported_eht_ltfs)
                    .bits(BitRange::new(&bytes, 46, 5))
                    .build(),
                Field::builder()
                    .title("Support of EHT-MCS 15 in MRU")
                    .value(self.support_of_eht_mcs_15_in_mru)
                    .bits(BitRange::new(&bytes, 51, 4))
                    .build(),
                Field::builder()
                    .title("Support of EHT DUP in 6 GHz")
                    .value(self.support_of_eht_dup_in_6_ghz)
                    .bits(BitRange::new(&bytes, 55, 1))
                    .build(),
                Field::builder()
                    .title("Support for 20 MHz Operating STA Receiving NDP With Wider BW")
                    .value(self.support_for_20_mhz_operating_sta_receiving_ndp_with_wider_bandwidth)
                    .bits(BitRange::new(&bytes, 56, 1))
                    .build(),
                Field::builder()
                    .title("Non-OFDMA UL MU-MIMO (≤ 80 MHz)")
                    .value(self.non_ofdma_ul_mu_mimo_bw_less_than_or_equal_to_eighty_mhz)
                    .bits(BitRange::new(&bytes, 57, 1))
                    .build(),
                Field::builder()
                    .title("Non-OFDMA UL MU-MIMO (= 160 MHz)")
                    .value(self.non_ofdma_ul_mu_mimo_bw_equal_to_160_mhz)
                    .bits(BitRange::new(&bytes, 58, 1))
                    .build(),
                Field::builder()
                    .title("Non-OFDMA UL MU-MIMO (= 320 MHz)")
                    .value(self.non_ofdma_ul_mu_mimo_bw_equal_to_320_mhz)
                    .bits(BitRange::new(&bytes, 59, 1))
                    .build(),
                Field::builder()
                    .title("MU Beamformer (≤ 80 MHz)")
                    .value(self.mu_beamformer_bw_less_than_or_equal_to_80_mhz)
                    .bits(BitRange::new(&bytes, 60, 1))
                    .build(),
                Field::builder()
                    .title("MU Beamformer (= 160 MHz)")
                    .value(self.mu_beamformer_bw_equal_to_160_mhz)
                    .bits(BitRange::new(&bytes, 61, 1))
                    .build(),
                Field::builder()
                    .title("MU Beamformer (= 320 MHz)")
                    .value(self.mu_beamformer_equal_to_320_mhz)
                    .bits(BitRange::new(&bytes, 62, 1))
                    .build(),
                Field::builder()
                    .title("TB Sounding Feedback Rate Limit")
                    .value(self.tb_sounding_feedback_rate_limit)
                    .bits(BitRange::new(&bytes, 63, 1))
                    .build(),
                Field::builder()
                    .title("Rx 1024-QAM in Wider BW DL OFDMA Support")
                    .value(self.rx_1024_qam_in_wider_bandwidth_dl_ofdma_support)
                    .bits(BitRange::new(&bytes, 64, 1))
                    .build(),
                Field::builder()
                    .title("Rx 4096-QAM in Wider BW DL OFDMA Support")
                    .value(self.rx_4096_qam_in_wider_bandwidth_dl_ofdma_support)
                    .bits(BitRange::new(&bytes, 65, 1))
                    .build(),
                Field::builder()
                    .title("20 MHz-Only Limited Capabilities Support")
                    .value(self.twenty_mhz_only_limited_capabilities_support)
                    .bits(BitRange::new(&bytes, 66, 1))
                    .build(),
                Field::builder()
                    .title("20 MHz-Only Triggered MU Beamforming Full BW Feedback and DL MU-MIMO")
                    .value(self.twenty_mhz_only_triggered_mu_beamforming_full_bw_feedback_and_dl_mu_mimo)
                    .bits(BitRange::new(&bytes, 67, 1))
                    .build(),
                Field::builder()
                    .title("20 MHz-Only MRU Support")
                    .value(self.twenty_mhz_only_mru_support)
                    .bits(BitRange::new(&bytes, 68, 1))
                    .build(),
                Field::reserved(BitRange::new(&bytes, 69, 3)),
            ])
            .build()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "eht_mcs_map_bw_eq_160_mhz_present: bool, eht_mcs_map_bw_eq_320_mhz_present: bool")]
pub struct SupportedEhtMcsAndNssSet {
    // Note: 20 MHz-only map for non-AP STAs is not parsed, as this library
    // only processes beacons/probe responses from APs

    // Always present for APs
    pub eht_mcs_map_bw_lte_80_mhz_except_20_mhz_only_non_ap_sta: EhtMcsMap,

    // Relies on HE Capabilities to know whether or not it's present
    // Therefore we can't finish parsing EHT Capabilities without passing
    // a reference to HE Capabilities
    #[deku(cond = "eht_mcs_map_bw_eq_160_mhz_present")]
    pub eht_mcs_map_bw_eq_160_mhz: Option<EhtMcsMap>,

    #[deku(cond = "eht_mcs_map_bw_eq_320_mhz_present")]
    pub eht_mcs_map_bw_eq_320_mhz: Option<EhtMcsMap>,
}

impl SupportedEhtMcsAndNssSet {
    pub fn to_field(&self) -> Field {
        let mut subfields = vec![
            self.eht_mcs_map_bw_lte_80_mhz_except_20_mhz_only_non_ap_sta
                .to_field("EHT-MCS Map BW ≤ 80 MHz"),
        ];

        if let Some(eht_mcs_map) = self.eht_mcs_map_bw_eq_160_mhz {
            subfields.push(eht_mcs_map.to_field("EHT-MCS Map BW = 160 MHz"));
        }

        if let Some(eht_mcs_map) = self.eht_mcs_map_bw_eq_320_mhz {
            subfields.push(eht_mcs_map.to_field("EHT-MCS Map BW = 320 MHz"));
        }

        Field::builder()
            .title("Supported EHT-MCS and NSS Set")
            .value("")
            .subfields(subfields)
            .build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct EhtMcsMap {
    #[deku(bits = 4)]
    pub rx_max_nss_that_supports_eht_mcs_zero_through_nine: u8,
    #[deku(bits = 4)]
    pub tx_max_nss_that_supports_eht_mcs_zero_through_nine: u8,
    #[deku(bits = 4)]
    pub rx_max_nss_that_supports_eht_mcs_ten_through_eleven: u8,
    #[deku(bits = 4)]
    pub tx_max_nss_that_supports_eht_mcs_ten_through_eleven: u8,
    #[deku(bits = 4)]
    pub rx_max_nss_that_supports_eht_mcs_twelve_through_thirteen: u8,
    #[deku(bits = 4)]
    pub tx_max_nss_that_supports_eht_mcs_twelve_through_thirteen: u8,
}

impl EhtMcsMap {
    /// Get the maximum number of spatial streams supported
    pub fn max_spatial_streams(&self) -> u8 {
        // The max NSS is the highest of all the Rx fields
        // A value of 0 means not supported for that MCS range
        let max_nss_0_9 = self.rx_max_nss_that_supports_eht_mcs_zero_through_nine;
        let max_nss_10_11 = self.rx_max_nss_that_supports_eht_mcs_ten_through_eleven;
        let max_nss_12_13 = self.rx_max_nss_that_supports_eht_mcs_twelve_through_thirteen;

        max_nss_0_9.max(max_nss_10_11).max(max_nss_12_13).max(1)
    }

    /// Get the maximum MCS supported for a given spatial stream
    pub fn max_mcs_for_stream(&self, stream: u8) -> Option<u8> {
        if stream < 1 || stream > 8 {
            return None;
        }

        // Check if stream is supported at each MCS level (highest first)
        if self.rx_max_nss_that_supports_eht_mcs_twelve_through_thirteen >= stream {
            return Some(13);
        }
        if self.rx_max_nss_that_supports_eht_mcs_ten_through_eleven >= stream {
            return Some(11);
        }
        if self.rx_max_nss_that_supports_eht_mcs_zero_through_nine >= stream {
            return Some(9);
        }

        None
    }

    pub fn to_field(&self, title: &str) -> Field {
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
            .title(title)
            .value("")
            .subfields([
                Field::builder()
                    .title("Rx Max NSS That Supports EHT-MCS 0-9")
                    .value(spatial_streams_value(
                        self.rx_max_nss_that_supports_eht_mcs_zero_through_nine,
                    ))
                    .units(spatial_streams_units(
                        self.rx_max_nss_that_supports_eht_mcs_zero_through_nine,
                    ))
                    .bits(BitRange::new(&bytes, 0, 4))
                    .build(),
                Field::builder()
                    .title("Tx Max NSS That Supports EHT-MCS 0-9")
                    .value(spatial_streams_value(
                        self.tx_max_nss_that_supports_eht_mcs_zero_through_nine,
                    ))
                    .units(spatial_streams_units(
                        self.tx_max_nss_that_supports_eht_mcs_zero_through_nine,
                    ))
                    .bits(BitRange::new(&bytes, 4, 4))
                    .build(),
                Field::builder()
                    .title("Rx Max NSS That Supports EHT-MCS 10-11")
                    .value(spatial_streams_value(
                        self.rx_max_nss_that_supports_eht_mcs_ten_through_eleven,
                    ))
                    .units(spatial_streams_units(
                        self.rx_max_nss_that_supports_eht_mcs_ten_through_eleven,
                    ))
                    .bits(BitRange::new(&bytes, 8, 4))
                    .build(),
                Field::builder()
                    .title("Tx Max NSS That Supports EHT-MCS 10-11")
                    .value(spatial_streams_value(
                        self.tx_max_nss_that_supports_eht_mcs_ten_through_eleven,
                    ))
                    .units(spatial_streams_units(
                        self.tx_max_nss_that_supports_eht_mcs_ten_through_eleven,
                    ))
                    .bits(BitRange::new(&bytes, 12, 4))
                    .build(),
                Field::builder()
                    .title("Rx Max NSS That Supports EHT-MCS 12-13")
                    .value(spatial_streams_value(
                        self.rx_max_nss_that_supports_eht_mcs_twelve_through_thirteen,
                    ))
                    .units(spatial_streams_units(
                        self.rx_max_nss_that_supports_eht_mcs_twelve_through_thirteen,
                    ))
                    .bits(BitRange::new(&bytes, 16, 4))
                    .build(),
                Field::builder()
                    .title("Tx Max NSS That Supports EHT-MCS 12-13")
                    .value(spatial_streams_value(
                        self.tx_max_nss_that_supports_eht_mcs_twelve_through_thirteen,
                    ))
                    .units(spatial_streams_units(
                        self.tx_max_nss_that_supports_eht_mcs_twelve_through_thirteen,
                    ))
                    .bits(BitRange::new(&bytes, 20, 4))
                    .build(),
            ])
            .bytes(bytes)
            .build()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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

impl EhtPpeThresholds {
    pub fn to_field(&self) -> Field {
        let bytes = self.to_bytes().unwrap_or_default();
        let mut subfields = vec![
            Field::builder()
                .title("NSS_PE")
                .value(self.nss_pe)
                .bits(BitRange::new(&bytes, 0, 4))
                .build(),
            Field::builder()
                .title("RU Index Bitmask")
                .value(self.ru_index_bitmask)
                .bits(BitRange::new(&bytes, 4, 5))
                .build(),
        ];
        for (i, ppe_thresholds_info) in self.ppe_thresholds_info.iter().enumerate() {
            subfields
                .push(ppe_thresholds_info.to_field(&format!("NSS {}", i), self.ru_index_bitmask));
        }

        Field::builder()
            .title("EHT PPE Thresholds")
            .value("")
            .subfields(subfields)
            .bytes(bytes)
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
        writer = "if let Some(val) = ppet_max_nss_ru_0 { write_bits_lsb0(deku::writer, u8::from(*val), 3) } else { Ok(()) }"
    )]
    pub ppet_max_nss_ru_0: Option<TransmissionConstellation>,
    #[deku(
        cond = "ru_index_bitmask[0]",
        bits = 3,
        map = "|value: u8| TransmissionConstellation::try_from(value).map(Some).map_err(|_| DekuError::Parse(\"Invalid TransmissionConstellation\".into()))",
        writer = "if let Some(val) = ppet_eight_nss_ru_0 { write_bits_lsb0(deku::writer, u8::from(*val), 3) } else { Ok(()) }"
    )]
    pub ppet_eight_nss_ru_0: Option<TransmissionConstellation>,
    #[deku(
        cond = "ru_index_bitmask[1]",
        bits = 3,
        map = "|value: u8| TransmissionConstellation::try_from(value).map(Some).map_err(|_| DekuError::Parse(\"Invalid TransmissionConstellation\".into()))",
        writer = "if let Some(val) = ppet_max_nss_ru_1 { write_bits_lsb0(deku::writer, u8::from(*val), 3) } else { Ok(()) }"
    )]
    pub ppet_max_nss_ru_1: Option<TransmissionConstellation>,
    #[deku(
        cond = "ru_index_bitmask[1]",
        bits = 3,
        map = "|value: u8| TransmissionConstellation::try_from(value).map(Some).map_err(|_| DekuError::Parse(\"Invalid TransmissionConstellation\".into()))",
        writer = "if let Some(val) = ppet_eight_nss_ru_1 { write_bits_lsb0(deku::writer, u8::from(*val), 3) } else { Ok(()) }"
    )]
    pub ppet_eight_nss_ru_1: Option<TransmissionConstellation>,
    #[deku(
        cond = "ru_index_bitmask[2]",
        bits = 3,
        map = "|value: u8| TransmissionConstellation::try_from(value).map(Some).map_err(|_| DekuError::Parse(\"Invalid TransmissionConstellation\".into()))",
        writer = "if let Some(val) = ppet_max_nss_ru_2 { write_bits_lsb0(deku::writer, u8::from(*val), 3) } else { Ok(()) }"
    )]
    pub ppet_max_nss_ru_2: Option<TransmissionConstellation>,
    #[deku(
        cond = "ru_index_bitmask[2]",
        bits = 3,
        map = "|value: u8| TransmissionConstellation::try_from(value).map(Some).map_err(|_| DekuError::Parse(\"Invalid TransmissionConstellation\".into()))",
        writer = "if let Some(val) = ppet_eight_nss_ru_2 { write_bits_lsb0(deku::writer, u8::from(*val), 3) } else { Ok(()) }"
    )]
    pub ppet_eight_nss_ru_2: Option<TransmissionConstellation>,
    #[deku(
        cond = "ru_index_bitmask[3]",
        bits = 3,
        map = "|value: u8| TransmissionConstellation::try_from(value).map(Some).map_err(|_| DekuError::Parse(\"Invalid TransmissionConstellation\".into()))",
        writer = "if let Some(val) = ppet_max_nss_ru_3 { write_bits_lsb0(deku::writer, u8::from(*val), 3) } else { Ok(()) }"
    )]
    pub ppet_max_nss_ru_3: Option<TransmissionConstellation>,
    #[deku(
        cond = "ru_index_bitmask[3]",
        bits = 3,
        map = "|value: u8| TransmissionConstellation::try_from(value).map(Some).map_err(|_| DekuError::Parse(\"Invalid TransmissionConstellation\".into()))",
        writer = "if let Some(val) = ppet_eight_nss_ru_3 { write_bits_lsb0(deku::writer, u8::from(*val), 3) } else { Ok(()) }"
    )]
    pub ppet_eight_nss_ru_3: Option<TransmissionConstellation>,
    #[deku(
        cond = "ru_index_bitmask[4]",
        bits = 3,
        map = "|value: u8| TransmissionConstellation::try_from(value).map(Some).map_err(|_| DekuError::Parse(\"Invalid TransmissionConstellation\".into()))",
        writer = "if let Some(val) = ppet_max_nss_ru_4 { write_bits_lsb0(deku::writer, u8::from(*val), 3) } else { Ok(()) }"
    )]
    pub ppet_max_nss_ru_4: Option<TransmissionConstellation>,
    #[deku(
        cond = "ru_index_bitmask[4]",
        bits = 3,
        map = "|value: u8| TransmissionConstellation::try_from(value).map(Some).map_err(|_| DekuError::Parse(\"Invalid TransmissionConstellation\".into()))",
        writer = "if let Some(val) = ppet_eight_nss_ru_4 { write_bits_lsb0(deku::writer, u8::from(*val), 3) } else { Ok(()) }"
    )]
    pub ppet_eight_nss_ru_4: Option<TransmissionConstellation>,
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

        if let Some(ppet_max) = self.ppet_max_nss_ru_0 {
            subfields.push(
                Field::builder()
                    .title("PPETmax NSS RU 0")
                    .value(ppet_max)
                    .units(format!("({})", u8::from(ppet_max)))
                    .bits(BitRange::new(&bytes, bit_offset, 3))
                    .build(),
            );
            bit_offset += 3;
        }

        if let Some(ppet_eight) = self.ppet_eight_nss_ru_0 {
            subfields.push(
                Field::builder()
                    .title("PPET8 NSS RU 0")
                    .value(ppet_eight)
                    .units(format!("({})", u8::from(ppet_eight)))
                    .bits(BitRange::new(&bytes, bit_offset, 3))
                    .build(),
            );
            bit_offset += 3;
        }

        if let Some(ppet_max) = self.ppet_max_nss_ru_1 {
            subfields.push(
                Field::builder()
                    .title("PPETmax NSS RU 1")
                    .value(ppet_max)
                    .units(format!("({})", u8::from(ppet_max)))
                    .bits(BitRange::new(&bytes, bit_offset, 3))
                    .build(),
            );
            bit_offset += 3;
        }

        if let Some(ppet_eight) = self.ppet_eight_nss_ru_1 {
            subfields.push(
                Field::builder()
                    .title("PPET8 NSS RU 1")
                    .value(ppet_eight)
                    .units(format!("({})", u8::from(ppet_eight)))
                    .bits(BitRange::new(&bytes, bit_offset, 3))
                    .build(),
            );
            bit_offset += 3;
        }

        if let Some(ppet_max) = self.ppet_max_nss_ru_2 {
            subfields.push(
                Field::builder()
                    .title("PPETmax NSS RU 2")
                    .value(ppet_max)
                    .units(format!("({})", u8::from(ppet_max)))
                    .bits(BitRange::new(&bytes, bit_offset, 3))
                    .build(),
            );
            bit_offset += 3;
        }

        if let Some(ppet_eight) = self.ppet_eight_nss_ru_2 {
            subfields.push(
                Field::builder()
                    .title("PPET8 NSS RU 2")
                    .value(ppet_eight)
                    .units(format!("({})", u8::from(ppet_eight)))
                    .bits(BitRange::new(&bytes, bit_offset, 3))
                    .build(),
            );
            bit_offset += 3;
        }

        if let Some(ppet_max) = self.ppet_max_nss_ru_3 {
            subfields.push(
                Field::builder()
                    .title("PPETmax NSS RU 3")
                    .value(ppet_max)
                    .units(format!("({})", u8::from(ppet_max)))
                    .bits(BitRange::new(&bytes, bit_offset, 3))
                    .build(),
            );
            bit_offset += 3;
        }

        if let Some(ppet_eight) = self.ppet_eight_nss_ru_3 {
            subfields.push(
                Field::builder()
                    .title("PPET8 NSS RU 3")
                    .value(ppet_eight)
                    .units(format!("({})", u8::from(ppet_eight)))
                    .bits(BitRange::new(&bytes, bit_offset, 3))
                    .build(),
            );
            bit_offset += 3;
        }

        if let Some(ppet_max) = self.ppet_max_nss_ru_4 {
            subfields.push(
                Field::builder()
                    .title("PPETmax NSS RU 4")
                    .value(ppet_max)
                    .units(format!("({})", u8::from(ppet_max)))
                    .bits(BitRange::new(&bytes, bit_offset, 3))
                    .build(),
            );
            bit_offset += 3;
        }

        if let Some(ppet_eight) = self.ppet_eight_nss_ru_4 {
            subfields.push(
                Field::builder()
                    .title("PPET8 NSS RU 4")
                    .value(ppet_eight)
                    .units(format!("({})", u8::from(ppet_eight)))
                    .bits(BitRange::new(&bytes, bit_offset, 3))
                    .build(),
            );
        }

        Field::builder()
            .title(title)
            .value("")
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
    FourThousandNinetySixQam = 6,
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
            Self::FourThousandNinetySixQam => write!(f, "4096-QAM"),
            Self::None => write!(f, "None"),
        }
    }
}
