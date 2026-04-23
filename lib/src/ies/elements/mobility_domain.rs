use deku::{DekuContainerWrite, DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use crate::ies::{BitRange, Field, IeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct MobilityDomain {
    pub mdid: u16,
    pub ft_capability_and_policy: FtCapabilityAndPolicy,
}

impl MobilityDomain {
    pub const NAME: &'static str = "Mobility Domain";
    pub const ID: u8 = 54;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);

    pub fn summary(&self) -> String {
        format!("MDID: {}", self.mdid)
    }

    pub fn fields(&self) -> Vec<Field> {
        vec![
            Field::builder()
                .title("MDID")
                .value(self.mdid)
                .bytes(self.mdid.to_le_bytes().to_vec())
                .build(),
            self.ft_capability_and_policy.to_field(),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct FtCapabilityAndPolicy {
    #[deku(bits = "1")]
    fast_bss_transition_over_ds: bool,
    #[deku(bits = "1")]
    resource_request_protocol_capability: bool,
    #[deku(bits = "6")]
    reserved: u8,
}

impl FtCapabilityAndPolicy {
    fn to_field(&self) -> Field {
        let byte = self
            .to_bytes()
            .unwrap_or_default()
            .first()
            .cloned()
            .unwrap_or_default();

        Field::builder()
            .title("FT Capability and Policy")
            .value("")
            .subfields([
                Field::builder()
                    .title("Fast BSS Transition Over DS")
                    .value(self.fast_bss_transition_over_ds)
                    .bits(BitRange::from_byte(byte, 0, 1))
                    .build(),
                Field::builder()
                    .title("Resource Request Protocol Capability")
                    .value(self.resource_request_protocol_capability)
                    .bits(BitRange::from_byte(byte, 1, 1))
                    .build(),
                Field::reserved(BitRange::from_byte(byte, 2, 6)),
            ])
            .byte(byte)
            .build()
    }
}
