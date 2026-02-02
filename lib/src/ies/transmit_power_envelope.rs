use std::fmt::Display;

use deku::{DekuContainerWrite, DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::{IeId, write_bits_lsb0};
use crate::{BitRange, Field};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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

    pub fn summary(&self) -> String {
        format!(
            "Local Maximum for 20 MHz: {:.1}",
            self.local_maximum_transmit_power_for_twenty_mhz_dbm()
        )
        .trim_end_matches("0")
        .trim_end_matches(".")
        .to_string()
            + " dBm"
    }

    pub fn fields(&self) -> Vec<Field> {
        let mut fields = vec![
            self.transmit_power_information.to_field(),
            Field::builder()
                .title("Local Maximum Transmit Power for 20 MHz")
                .value(self.local_maximum_transmit_power_for_twenty_mhz_dbm())
                .units("dBm")
                .byte(self.local_maximum_transmit_power_for_twenty_mhz as u8)
                .build(),
        ];

        if let Some(max_transmit_power) = self.local_maximum_transmit_power_for_forty_mhz_dbm() {
            fields.push(
                Field::builder()
                    .title("Local Maximum Transmit Power for 40 MHz")
                    .value(max_transmit_power)
                    .units("dBm")
                    .byte(
                        self.local_maximum_transmit_power_for_forty_mhz
                            .map(|power_byte| power_byte as u8)
                            .unwrap_or_default(),
                    )
                    .build(),
            );
        }

        if let Some(max_transmit_power) = self.local_maximum_transmit_power_for_eighty_mhz_dbm() {
            fields.push(
                Field::builder()
                    .title("Local Maximum Transmit Power for 80 MHz")
                    .value(max_transmit_power)
                    .units("dBm")
                    .byte(
                        self.local_maximum_transmit_power_for_eighty_mhz
                            .map(|power_byte| power_byte as u8)
                            .unwrap_or_default(),
                    )
                    .build(),
            );
        }

        if let Some(max_transmit_power) =
            self.local_maximum_transmit_power_for_one_hundred_sixty_mhz_dbm()
        {
            fields.push(
                Field::builder()
                    .title("Local Maximum Transmit Power for 160 MHz")
                    .value(max_transmit_power)
                    .units("dBm")
                    .byte(
                        self.local_maximum_transmit_power_for_one_hundred_sixty_mhz
                            .map(|byte| byte as u8)
                            .unwrap_or_default(),
                    )
                    .build(),
            );
        }

        fields
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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

impl TransmitPowerInformation {
    pub fn to_field(&self) -> Field {
        let byte = self
            .to_bytes()
            .unwrap_or_default()
            .first()
            .cloned()
            .unwrap_or_default();
        Field::builder()
            .title("Transmit Power Information")
            .value("")
            .bytes(self.to_bytes().unwrap_or_default())
            .subfields([
                Field::builder()
                    .title("Local Maximum Transmit Power Count")
                    .value(self.local_maximum_transmit_power_count)
                    .bits(BitRange::from_byte(byte, 0, 3))
                    .build(),
                Field::builder()
                    .title("Local Maximum Transmit Power Unit Interpretation")
                    .value(self.local_maximum_transmit_power_unit_interpretation)
                    .bits(BitRange::from_byte(byte, 3, 3))
                    .build(),
                Field::reserved(BitRange::from_byte(byte, 6, 2)),
            ])
            .build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
