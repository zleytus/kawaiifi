use deku::{DekuContainerWrite, DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use crate::ies::{BitRange, Field, IeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "len: usize")]
pub struct RsnExtension {
    pub extended_rsn_capabilities: ExtendedRsnCapabilities,
}

impl RsnExtension {
    pub const NAME: &'static str = "RSNXE";
    pub const ID: u8 = 244;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);

    pub fn summary(&self) -> String {
        "".to_string()
    }

    pub fn fields(&self) -> Vec<Field> {
        vec![self.extended_rsn_capabilities.to_field()]
    }
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

impl ExtendedRsnCapabilities {
    pub fn to_field(&self) -> Field {
        let bytes = self.to_bytes().unwrap_or_default();

        Field::builder()
            .title("Extended RSN Capabilities")
            .value("")
            .subfields([
                Field::builder()
                    .title("Field Length")
                    .value(self.field_length)
                    .units(if self.field_length == 1 {
                        "byte"
                    } else {
                        "bytes"
                    })
                    .bits(BitRange::new(&bytes, 0, 4))
                    .build(),
                Field::builder()
                    .title("Protected TWT Operation Support")
                    .value(self.protected_twt_operation_support)
                    .bits(BitRange::new(&bytes, 4, 1))
                    .build(),
                Field::builder()
                    .title("SAE Hash-to-Element")
                    .value(self.sae_hash_to_element)
                    .bits(BitRange::new(&bytes, 5, 1))
                    .build(),
                Field::reserved(BitRange::new(&bytes, 6, bytes.len() * 8 - 6)),
            ])
            .bytes(bytes)
            .build()
    }
}
