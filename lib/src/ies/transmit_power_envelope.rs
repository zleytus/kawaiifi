use std::fmt::Display;

use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::{IeId, write_bits_lsb0};

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "len: usize")]
pub struct TransmitPowerEnvelope {
    transmit_power_information: TransmitPowerInformation,
    #[deku(bytes = 1)]
    local_maximum_transmit_power_for_twenty_mhz: i8,
    #[deku(bytes = 1, cond = "len >= 3")]
    local_maximum_transmit_power_for_forty_mhz: Option<i8>,
    #[deku(bytes = 1, cond = "len >= 4")]
    local_maximum_transmit_power_for_eighty_mhz: Option<i8>,
    #[deku(bytes = 1, cond = "len >= 5")]
    local_maximum_transmit_power_for_one_hundred_sixty_mhz: Option<i8>,
}

impl TransmitPowerEnvelope {
    pub const NAME: &'static str = "Transmit Power Envelope";
    pub const ID: u8 = 195;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const MIN_LENGTH: usize = 2;

    pub fn transmit_power_information(&self) -> TransmitPowerInformation {
        self.transmit_power_information
    }

    pub fn local_maximum_transmit_power_for_twenty_mhz_dbm(&self) -> f64 {
        self.local_maximum_transmit_power_for_twenty_mhz as f64 / 2.0
    }

    pub fn local_maximum_transmit_power_for_forty_mhz_dbm(&self) -> Option<f64> {
        self.local_maximum_transmit_power_for_forty_mhz
            .map(|value| value as f64 / 2.0)
    }

    pub fn local_maximum_transmit_power_for_eighty_mhz_dbm(&self) -> Option<f64> {
        self.local_maximum_transmit_power_for_eighty_mhz
            .map(|value| value as f64 / 2.0)
    }

    pub fn local_maximum_transmit_power_for_one_hundred_sixty_mhz_dbm(&self) -> Option<f64> {
        self.local_maximum_transmit_power_for_one_hundred_sixty_mhz
            .map(|value| value as f64 / 2.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct TransmitPowerInformation {
    #[deku(bits = 3)]
    pub local_maximum_transmit_power_count: u8,
    #[deku(
        bits = 3,
        map = "|value: u8| -> Result<UnitInterpretation, deku::DekuError> { Ok(UnitInterpretation::from(value)) }",
        writer = "write_bits_lsb0(deku::writer, self.local_maximum_transmit_power_unit_interpretation.into(), 3)"
    )]
    pub local_maximum_transmit_power_unit_interpretation: UnitInterpretation,
    #[deku(bits = 2)]
    reserved: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum UnitInterpretation {
    Eirp = 0,
    Unknown(u8),
}

impl UnitInterpretation {
    pub fn value(&self) -> u8 {
        match self {
            UnitInterpretation::Eirp => 0,
            UnitInterpretation::Unknown(value) => *value,
        }
    }
}

impl std::convert::From<u8> for UnitInterpretation {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Eirp,
            _ => Self::Unknown(value),
        }
    }
}

impl Into<u8> for UnitInterpretation {
    fn into(self) -> u8 {
        match self {
            Self::Eirp => 0,
            Self::Unknown(val) => val,
        }
    }
}

impl Display for UnitInterpretation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnitInterpretation::Eirp => write!(f, "EIRP"),
            UnitInterpretation::Unknown(value) => write!(f, "Unknown ({})", value),
        }
    }
}
