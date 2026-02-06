use std::fmt::Display;

use deku::{DekuContainerWrite, DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use crate::ies::{BitRange, Field, IeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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

    pub fn summary(&self) -> String {
        format!("Measurement Type: {}", self.measurement_type)
    }

    pub fn fields(&self) -> Vec<Field> {
        vec![
            Field::builder()
                .title("Measurement Token")
                .value(self.measurement_token)
                .byte(self.measurement_token)
                .build(),
            self.measurement_request_mode.to_field(),
            Field::builder()
                .title("Measurement Type")
                .value(self.measurement_type)
                .bytes(self.measurement_type.to_bytes().unwrap_or_default())
                .build(),
            Field::builder()
                .title("Measurement Request")
                .value(format!("{:#?}", self.measurement_request))
                .bytes(self.measurement_request.clone())
                .build(),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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

impl MeasurementRequestMode {
    pub fn to_field(&self) -> Field {
        let byte = self
            .to_bytes()
            .unwrap_or_default()
            .first()
            .cloned()
            .unwrap_or_default();

        Field::builder()
            .title("Measurement Request Mode")
            .value("")
            .subfields([
                Field::builder()
                    .title("Parallel")
                    .value(self.parallel)
                    .bits(BitRange::from_byte(byte, 0, 1))
                    .build(),
                Field::builder()
                    .title("Enable")
                    .value(self.enable)
                    .bits(BitRange::from_byte(byte, 1, 1))
                    .build(),
                Field::builder()
                    .title("Request")
                    .value(self.request)
                    .bits(BitRange::from_byte(byte, 2, 1))
                    .build(),
                Field::builder()
                    .title("Report")
                    .value(self.report)
                    .bits(BitRange::from_byte(byte, 3, 1))
                    .build(),
                Field::builder()
                    .title("Duration Mandatory")
                    .value(self.duration_mandatory)
                    .bits(BitRange::from_byte(byte, 4, 1))
                    .build(),
                Field::reserved(BitRange::from_byte(byte, 5, 3)),
            ])
            .byte(byte)
            .build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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

impl Display for MeasurementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Basic => write!(f, "Basic"),
            Self::ClearChannelAssessment => write!(f, "Clear Channel Assessment"),
            Self::ReceivePowerIndicationHistogram => {
                write!(f, "Receive Power Indication Histogram")
            }
            Self::ChannelLoad => write!(f, "Channel Load"),
            Self::NoiseHistogram => write!(f, "Noise Histogram"),
            Self::Beacon => write!(f, "Beacon"),
            Self::Frame => write!(f, "Frame"),
            Self::StaStatistics => write!(f, "STA Statistics"),
            Self::Lci => write!(f, "LCI"),
            Self::TransmitStreamCategoryMeasurement => {
                write!(f, "Transmit Stream Category Measurement")
            }
            Self::MulticastDiagnostics => write!(f, "Multicast Diagnostics"),
            Self::LocationCivic => write!(f, "Location Civic"),
            Self::LocationIdentifier => write!(f, "Location Identifier"),
            Self::DirectionalChannelQuality => write!(f, "Directional Channel Quality"),
            Self::DirectionalMeasurement => write!(f, "Directional Measurement"),
            Self::DirectionalStatistics => write!(f, "Directional Statistics"),
            Self::FineTimingMeasurementRange => write!(f, "Fine Timing Measurement Range"),
            Self::Reserved(val) => write!(f, "Reserved ({})", val),
            Self::MeasurementPause => write!(f, "Measurement Pause"),
        }
    }
}
