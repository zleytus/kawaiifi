use deku::prelude::*;

use super::IeId;

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[deku(bit_order = "lsb")]
pub struct TwentyFortyBssCoexistence {
    #[deku(bits = 1)]
    pub information_request: bool,
    #[deku(bits = 1)]
    pub forty_mhz_intolerant: bool,
    #[deku(bits = 1)]
    pub twenty_mhz_bss_width_request: bool,
    #[deku(bits = 1)]
    pub obss_scanning_exemption_request: bool,
    #[deku(bits = 1)]
    pub obss_scanning_exemption_grant: bool,
    #[deku(bits = 3)]
    reserved: u8,
}

impl TwentyFortyBssCoexistence {
    pub const NAME: &'static str = "20/40 BSS Coexistence";
    pub const ID: u8 = 72;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
}
