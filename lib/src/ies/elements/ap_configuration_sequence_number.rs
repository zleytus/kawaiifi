use deku::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Field, ies::IeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct ApConfigurationSequenceNumber {
    #[deku(bytes = 1)]
    pub ap_csn: u8,
}

impl ApConfigurationSequenceNumber {
    pub const ID: u8 = 239;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const NAME: &'static str = "AP Configuration Sequence Number";

    pub fn summary(&self) -> String {
        self.ap_csn.to_string()
    }

    pub fn fields(&self) -> Vec<Field> {
        vec![
            Field::builder()
                .title("AP-CSN")
                .value(self.ap_csn)
                .byte(self.ap_csn)
                .build(),
        ]
    }
}
