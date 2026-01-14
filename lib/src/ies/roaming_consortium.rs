use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "len: usize")]
pub struct RoamingConsortium {
    #[deku(bytes = 1)]
    pub number_of_anqp_ois: u8,
    pub oi_1_and_2_lengths: OiLengths,
    #[deku(count = "oi_1_and_2_lengths.oi_1_length")]
    pub oi_1: Vec<u8>,
    #[deku(
        cond = "oi_1_and_2_lengths.oi_2_length > 0",
        count = "oi_1_and_2_lengths.oi_2_length"
    )]
    pub oi_2: Option<Vec<u8>>,
    #[deku(
        cond = "len - 2 - usize::from(oi_1_and_2_lengths.oi_1_length) - usize::from(oi_1_and_2_lengths.oi_2_length) > 0",
        count = "len - 2 - usize::from(oi_1_and_2_lengths.oi_1_length) - usize::from(oi_1_and_2_lengths.oi_2_length)"
    )]
    pub oi_3: Option<Vec<u8>>,
}

impl RoamingConsortium {
    pub const NAME: &'static str = "Roaming Consortium";
    pub const ID: u8 = 111;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct OiLengths {
    #[deku(bits = 4)]
    pub oi_1_length: u8,
    #[deku(bits = 4)]
    pub oi_2_length: u8,
}
