use deku::{DekuRead, DekuWrite};

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(ctx = "len: usize")]
pub struct Interworking {
    pub access_network_options: AccessNetworkOptions,
    #[deku(cond = "len == 3 || len == 9")]
    pub venue_info: Option<VenueInfo>,
    #[deku(cond = "len == 7 || len == 9")]
    pub hessid: Option<[u8; 6]>,
}

impl Interworking {
    pub const ID: u8 = 107;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const NAME: &'static str = "Interworking";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(bit_order = "lsb")]
pub struct AccessNetworkOptions {
    #[deku(bits = 4)]
    pub access_network_type: u8,
    #[deku(bits = 1)]
    pub internet: bool,
    #[deku(bits = 1)]
    pub asra: bool,
    #[deku(bits = 1)]
    pub esr: bool,
    #[deku(bits = 1)]
    pub uesa: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
pub struct VenueInfo {
    pub venue_group: VenueGroup,
    #[deku(bytes = 1)]
    pub venue_type: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite)]
#[repr(u8)]
#[deku(id_type = "u8")]
pub enum VenueGroup {
    Unspecified = 0,
    Assembly,
    Business,
    Educational,
    FactoryAndIndustrial,
    Institutional,
    Mercantile,
    Residential,
    Storage,
    UtilityAndMiscellaneous,
    Vehicular,
    Outdoor,
    #[deku(id_pat = "_")]
    Reserved,
}
