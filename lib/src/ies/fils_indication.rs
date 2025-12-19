use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct FilsIndication {
    pub fils_information: FilsInformation,
    #[deku(cond = "fils_information.cache_identifier_included", bytes = 2)]
    pub cache_identifier: Option<[u8; 2]>,
    #[deku(cond = "fils_information.hessid_included", bytes = 6)]
    pub hessid: Option<[u8; 6]>,
    #[deku(count = "fils_information.number_of_realm_identifiers")]
    pub realm_identifiers: Vec<RealmIdentifier>,
    #[deku(count = "fils_information.number_of_public_key_identifiers")]
    pub public_key_identifiers: Vec<PublicKeyIdentifier>,
}

impl FilsIndication {
    pub const NAME: &'static str = "FILS Indication";
    pub const ID: u8 = 240;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct FilsInformation {
    #[deku(bits = 3)]
    pub number_of_public_key_identifiers: u8,
    #[deku(bits = 3)]
    pub number_of_realm_identifiers: u8,
    #[deku(bits = 1)]
    pub fils_ip_address_configuration: bool,
    #[deku(bits = 1)]
    pub cache_identifier_included: bool,
    #[deku(bits = 1)]
    pub hessid_included: bool,
    #[deku(bits = 1)]
    pub fils_shared_key_authentication_without_pfs_supported: bool,
    #[deku(bits = 1)]
    pub fils_shared_key_authentication_with_pfs_supported: bool,
    #[deku(bits = 1)]
    pub fils_public_key_authentication_supported: bool,
    #[deku(bits = 4)]
    reserved: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct RealmIdentifier {
    #[deku(bytes = 2)]
    pub hashed_realm: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct PublicKeyIdentifier {
    #[deku(bytes = 1)]
    pub key_type: u8,
    #[deku(bytes = 1)]
    pub length: u8,
    #[deku(count = "length")]
    pub public_key_indicator: Vec<u8>,
}
