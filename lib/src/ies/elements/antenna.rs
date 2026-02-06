use deku::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Field, ies::IeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct Antenna {
    #[deku(bytes = 1)]
    pub antenna_id: u8,
}

impl Antenna {
    pub const NAME: &'static str = "Antenna";
    pub const ID: u8 = 64;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const LENGTH: usize = 1;

    pub fn summary(&self) -> String {
        format!("ID: {}", self.antenna_id.to_string())
    }

    pub fn fields(&self) -> Vec<Field> {
        vec![
            Field::builder()
                .title("Antenna ID")
                .value(self.antenna_id)
                .byte(self.antenna_id)
                .build(),
        ]
    }
}
