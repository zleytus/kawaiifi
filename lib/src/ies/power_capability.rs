use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::IeId;
use crate::Field;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct PowerCapability {
    #[deku(bytes = 1)]
    pub minimum_transmit_power_capability_dbm: i8,
    #[deku(bytes = 1)]
    pub maximum_transmit_power_capability_dbm: i8,
}

impl PowerCapability {
    pub const ID: u8 = 33;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const NAME: &'static str = "Power Capability";

    pub fn summary(&self) -> String {
        format!(
            "Min Tx: {} dBm, Max Tx: {} dBm",
            self.minimum_transmit_power_capability_dbm, self.maximum_transmit_power_capability_dbm
        )
    }

    pub fn fields(&self) -> Vec<Field> {
        vec![
            Field::builder()
                .title("Minimum Tx Power Capability")
                .value(self.minimum_transmit_power_capability_dbm)
                .units("dBm")
                .byte(self.minimum_transmit_power_capability_dbm as u8)
                .build(),
            Field::builder()
                .title("Maximum Tx Power Capability")
                .value(self.maximum_transmit_power_capability_dbm)
                .units("dBm")
                .byte(self.maximum_transmit_power_capability_dbm as u8)
                .build(),
        ]
    }
}
