use deku::{DekuContainerWrite, DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use crate::ies::{BitRange, Field, IeId};

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

    pub fn summary(&self) -> String {
        match self.number_of_anqp_ois {
            1 => "1 ANQP OI".to_string(),
            ois => format!("{} ANQP OIs", ois),
        }
    }

    pub fn fields(&self) -> Vec<Field> {
        let mut fields = vec![
            Field::builder()
                .title("Number of ANQP OIs")
                .value(self.number_of_anqp_ois)
                .byte(self.number_of_anqp_ois)
                .build(),
            self.oi_1_and_2_lengths.to_field(),
            Field::builder()
                .title("OI 1")
                .value(
                    self.oi_1
                        .iter()
                        .map(|byte| format!("{:02X}", byte))
                        .collect::<Vec<String>>()
                        .join(":"),
                )
                .bytes(self.oi_1.clone())
                .build(),
        ];

        if let Some(oi_2) = &self.oi_2 {
            fields.push(
                Field::builder()
                    .title("OI 2")
                    .value(
                        oi_2.iter()
                            .map(|byte| format!("{:02X}", byte))
                            .collect::<Vec<String>>()
                            .join(":"),
                    )
                    .bytes(oi_2.clone())
                    .build(),
            );
        }

        if let Some(oi_3) = &self.oi_3 {
            fields.push(
                Field::builder()
                    .title("OI 3")
                    .value(
                        oi_3.iter()
                            .map(|byte| format!("{:02X}", byte))
                            .collect::<Vec<String>>()
                            .join(":"),
                    )
                    .bytes(oi_3.clone())
                    .build(),
            );
        }

        fields
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct OiLengths {
    #[deku(bits = 4)]
    pub oi_1_length: u8,
    #[deku(bits = 4)]
    pub oi_2_length: u8,
}

impl OiLengths {
    pub fn to_field(&self) -> Field {
        let byte = self
            .to_bytes()
            .unwrap_or_default()
            .first()
            .cloned()
            .unwrap_or_default();

        Field::builder()
            .title("OI Lengths")
            .value("")
            .subfields([
                Field::builder()
                    .title("OI 1 Length")
                    .value(self.oi_1_length)
                    .bits(BitRange::from_byte(byte, 0, 4))
                    .build(),
                Field::builder()
                    .title("OI 2 Length")
                    .value(self.oi_2_length)
                    .bits(BitRange::from_byte(byte, 4, 4))
                    .build(),
            ])
            .byte(byte)
            .build()
    }
}
