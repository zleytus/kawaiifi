use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::IeId;
use crate::Field;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct BssLoad {
    #[deku(bytes = 2)]
    pub station_count: u16,
    #[deku(bytes = 1)]
    pub channel_utilization: u8,
    #[deku(bytes = 2)]
    pub available_admission_capacity: u16,
}

impl BssLoad {
    pub const NAME: &'static str = "BSS Load";
    pub const ID: u8 = 11;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const LENGTH: usize = 5;

    pub fn summary(&self) -> String {
        format!(
            "Station Count: {}, Channel Utilization: {}%",
            self.station_count, self.channel_utilization
        )
    }

    pub fn fields(&self) -> Vec<Field> {
        vec![
            Field::builder()
                .title("Station Count")
                .value(self.station_count)
                .bytes(self.station_count.to_be_bytes().to_vec())
                .build(),
            Field::builder()
                .title("Channel Utilization")
                .value(format!("{}%", self.channel_utilization))
                .byte(self.channel_utilization)
                .build(),
            Field::builder()
                .title("Available Admission Capacity")
                .value(self.available_admission_capacity)
                .bytes(self.available_admission_capacity.to_be_bytes().to_vec())
                .build(),
        ]
    }
}
