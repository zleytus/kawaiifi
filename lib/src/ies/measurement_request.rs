use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "len: usize")]
pub struct MeasurementRequest {
    #[deku(bytes = 1)]
    pub measurement_token: u8,
    pub measurement_request_mode: MeasurementRequestMode,
    pub measurement_type: MeasurementType,
    #[deku(count = "len - 3")]
    pub measurement_request: Vec<u8>,
}

impl MeasurementRequest {
    pub const NAME: &'static str = "Measurement Request";
    pub const ID: u8 = 38;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct MeasurementRequestMode {
    #[deku(bits = 1)]
    pub parallel: bool,
    #[deku(bits = 1)]
    pub enable: bool,
    #[deku(bits = 1)]
    pub request: bool,
    #[deku(bits = 1)]
    pub report: bool,
    #[deku(bits = 1)]
    pub duration_mandatory: bool,
    #[deku(bits = 3)]
    reserved: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(id_type = "u8")]
#[repr(u8)]
pub enum MeasurementType {
    Basic = 0,
    ClearChannelAssessment = 1,
    ReceivePowerIndicationHistogram = 2,
    ChannelLoad = 3,
    NoiseHistogram = 4,
    Beacon = 5,
    Frame = 6,
    StaStatistics = 7,
    Lci = 8,
    TransmitStreamCategoryMeasurement = 9,
    MulticastDiagnostics = 10,
    LocationCivic = 11,
    LocationIdentifier = 12,
    DirectionalChannelQuality = 13,
    DirectionalMeasurement = 14,
    DirectionalStatistics = 15,
    FineTimingMeasurementRange = 16,
    #[deku(id_pat = "17..=254")]
    Reserved(u8),
    MeasurementPause = 255,
}
