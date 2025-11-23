use deku::{DekuRead, DekuWrite};

use super::{IeId, write_bits_lsb0};

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
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
}

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
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
    pub short_git_for_one_hundred_sixty_and_eighty_plus_eighty_mhz: bool,
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
