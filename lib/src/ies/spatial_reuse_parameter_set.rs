use deku::{DekuRead, DekuWrite};

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
pub struct SpatialReuseParameterSet {
    pub sr_control: SrControl,
    #[deku(cond = "sr_control.non_srg_offset_present", bytes = "1")]
    pub non_srg_obss_pd_max_offset: Option<u8>,
    #[deku(cond = "sr_control.srg_information_present", bytes = "1")]
    pub srg_obss_pd_min_offset: Option<u8>,
    #[deku(cond = "sr_control.srg_information_present", bytes = "1")]
    pub srg_obss_pd_max_offset: Option<u8>,
    #[deku(cond = "sr_control.srg_information_present", bytes = "8")]
    pub srg_bss_color_bitmap: Option<[u8; 8]>,
    #[deku(cond = "sr_control.srg_information_present", bytes = "8")]
    pub srg_partial_bssid_bitmap: Option<[u8; 8]>,
}

impl SpatialReuseParameterSet {
    pub const NAME: &'static str = "Spatial Reuse Parameter Set";
    pub const ID: u8 = 255;
    pub const ID_EXT: Option<u8> = Some(39);
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(bit_order = "lsb")]
pub struct SrControl {
    #[deku(bits = 1)]
    pub pssr_disallowed: bool,
    #[deku(bits = 1)]
    pub non_srg_obss_pd_sr_disallowed: bool,
    #[deku(bits = 1)]
    pub non_srg_offset_present: bool,
    #[deku(bits = 1)]
    pub srg_information_present: bool,
    #[deku(bits = 1)]
    pub hesiga_spatial_reuse_value15_allowed: bool,
    #[deku(bits = 3)]
    reserved: u8,
}
