use deku::{DekuContainerWrite, DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::IeId;
use crate::{BitRange, Field};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct FilsIndication {
    pub fils_information: FilsInformation,
    #[deku(cond = "fils_information.cache_identifier_included", bytes = 2)]
    pub cache_identifier: Option<u16>,
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

    pub fn summary(&self) -> String {
        "".to_string()
    }

    pub fn fields(&self) -> Vec<Field> {
        let mut fields = vec![self.fils_information.to_field()];

        if let Some(cache_identifier) = self.cache_identifier {
            fields.push(
                Field::builder()
                    .title("Cache Identifier")
                    .value(cache_identifier)
                    .bytes(cache_identifier.to_le_bytes().to_vec())
                    .build(),
            );
        }

        if let Some(hessid) = self.hessid {
            fields.push(
                Field::builder()
                    .title("HESSID")
                    .value(
                        hessid
                            .iter()
                            .map(|byte| format!("{:02X}", byte))
                            .collect::<Vec<String>>()
                            .join(":"),
                    )
                    .bytes(hessid.to_vec())
                    .build(),
            );
        }

        for realm_identifier in &self.realm_identifiers {
            fields.push(realm_identifier.to_field());
        }

        for public_key_identifier in &self.public_key_identifiers {
            fields.push(public_key_identifier.to_field());
        }

        fields
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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

impl FilsInformation {
    pub fn to_field(&self) -> Field {
        let bytes = self.to_bytes().unwrap_or_default();

        Field::builder()
            .title("FILS Information")
            .value("")
            .subfields([
                Field::builder()
                    .title("Number of Public Key Identifiers")
                    .value(self.number_of_public_key_identifiers)
                    .bits(BitRange::new(&bytes, 0, 3))
                    .build(),
                Field::builder()
                    .title("Number of Realm Identifiers")
                    .value(self.number_of_realm_identifiers)
                    .bits(BitRange::new(&bytes, 3, 3))
                    .build(),
                Field::builder()
                    .title("FILS IP Address Configuration")
                    .value(self.fils_ip_address_configuration)
                    .bits(BitRange::new(&bytes, 6, 1))
                    .build(),
                Field::builder()
                    .title("Cache Identifier Included")
                    .value(self.cache_identifier_included)
                    .bits(BitRange::new(&bytes, 7, 1))
                    .build(),
                Field::builder()
                    .title("HESSID Included")
                    .value(self.hessid_included)
                    .bits(BitRange::new(&bytes, 8, 1))
                    .build(),
                Field::builder()
                    .title("FILS Shared Key Authentication Without PFS Supported")
                    .value(self.fils_shared_key_authentication_without_pfs_supported)
                    .bits(BitRange::new(&bytes, 9, 1))
                    .build(),
                Field::builder()
                    .title("FILS Shared Key Authentication With PFS Supported")
                    .value(self.fils_shared_key_authentication_with_pfs_supported)
                    .bits(BitRange::new(&bytes, 10, 1))
                    .build(),
                Field::builder()
                    .title("FILS Public Key Authentication Supported")
                    .value(self.fils_public_key_authentication_supported)
                    .bits(BitRange::new(&bytes, 11, 1))
                    .build(),
                Field::reserved(BitRange::new(&bytes, 12, 4)),
            ])
            .bytes(bytes)
            .build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct RealmIdentifier {
    #[deku(bytes = 2)]
    pub hashed_realm: u16,
}

impl RealmIdentifier {
    pub fn to_field(&self) -> Field {
        let bytes = self.hashed_realm.to_le_bytes();

        Field::builder()
            .title("Realm Identifier")
            .value(self.hashed_realm)
            .subfields([Field::builder()
                .title("Hashed Realm")
                .value(self.hashed_realm)
                .bytes(bytes.to_vec())
                .build()])
            .bytes(bytes.to_vec())
            .build()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct PublicKeyIdentifier {
    #[deku(bytes = 1)]
    pub key_type: u8,
    #[deku(bytes = 1)]
    pub length: u8,
    #[deku(count = "length")]
    pub public_key_indicator: Vec<u8>,
}

impl PublicKeyIdentifier {
    pub fn to_field(&self) -> Field {
        Field::builder()
            .title("Public Key Identifier")
            .value("")
            .subfields([
                Field::builder()
                    .title("Key Type")
                    .value(self.key_type)
                    .byte(self.key_type)
                    .build(),
                Field::builder()
                    .title("Length")
                    .value(self.length)
                    .units(if self.length == 1 { "byte" } else { "bytes" })
                    .byte(self.length)
                    .build(),
                Field::builder()
                    .title("Public Key Indicator")
                    .value(format!("{:#?}", self.public_key_indicator))
                    .bytes(self.public_key_indicator.clone())
                    .build(),
            ])
            .bytes(self.to_bytes().unwrap_or_default())
            .build()
    }
}
