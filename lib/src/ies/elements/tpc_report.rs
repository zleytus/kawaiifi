use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use crate::{Field, ies::IeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct TpcReport {
    #[deku(bytes = 1)]
    pub transmit_power_dbm: i8,
    #[deku(bytes = 1)]
    pub link_margin_db: i8,
}

impl TpcReport {
    pub const NAME: &'static str = "TPC Report";
    pub const ID: u8 = 35;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);

    pub fn summary(&self) -> String {
        format!("Transmit Power: {} dBm", self.transmit_power_dbm)
    }

    pub fn fields(&self) -> Vec<Field> {
        vec![
            Field::builder()
                .title("Transmit Power")
                .value(self.transmit_power_dbm)
                .units("dBm")
                .byte(self.transmit_power_dbm as u8)
                .build(),
            Field::builder()
                .title("Link Margin")
                .value(self.link_margin_db)
                .units("dB")
                .byte(self.link_margin_db as u8)
                .build(),
        ]
    }
}
