use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "len: usize")]
pub struct RsnExtension {
    pub extended_rsn_capabilities: ExtendedRsnCapabilities,
}

impl RsnExtension {
    pub const NAME: &'static str = "RSN Extension";
    pub const ID: u8 = 244;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct ExtendedRsnCapabilities {
    #[deku(bits = 4)]
    pub field_length: u8,
    #[deku(bits = 1)]
    pub protected_twt_operation_support: bool,
    #[deku(bits = 1)]
    pub sae_hash_to_element: bool,
    #[deku(bits = "8 * usize::from(field_length + 1) - 6")]
    reserved: u128,
}
