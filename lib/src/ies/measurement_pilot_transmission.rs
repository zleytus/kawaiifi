use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::{Ie, IeId};

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "len: usize")]
pub struct MeasurementPilotTransmission {
    #[deku(bytes = 1)]
    pub measurement_pilot_interval_tu: u8,
    #[deku(count = "len.checked_sub(1).unwrap_or_default()")]
    subelements: Vec<u8>,
}

impl MeasurementPilotTransmission {
    pub const NAME: &'static str = "Measurement Pilot Transmission";
    pub const ID: u8 = 66;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const MIN_LENGTH: usize = 1;

    pub fn subelements(&self) -> Vec<Ie> {
        super::from_bytes(&self.subelements)
    }
}
