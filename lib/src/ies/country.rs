use std::{fmt::Display, str};

use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "len: usize")]
pub struct Country {
    #[deku(bytes = 2)]
    country_code: [u8; 2],
    #[deku(bytes = 1)]
    environment: u8,
    #[deku(count = "len.checked_sub(3).unwrap_or_default()")]
    triplets: Vec<u8>,
}

impl Country {
    pub const NAME: &'static str = "Country";
    pub const ID: u8 = 7;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const MIN_LENGTH: usize = 6;

    pub fn country_code(&self) -> &str {
        str::from_utf8(&self.country_code).unwrap_or("??")
    }

    pub fn environment(&self) -> Environment {
        Environment::from(self.environment)
    }

    pub fn subband_info(&self) -> Vec<SubbandInfo> {
        let mut subbands = Vec::new();
        let mut last_operating_info = None;

        for triplet in self.triplets.chunks_exact(3) {
            if triplet[0] <= 233 {
                subbands.push(SubbandInfo {
                    first_channel_number: triplet[0],
                    number_of_channels: triplet[1],
                    max_transmit_power_level_dbm: triplet[2] as i8,
                    operating_info: last_operating_info,
                });
            } else {
                last_operating_info = Some(OperatingInfo {
                    operating_extension_id: triplet[0],
                    operating_class: triplet[1],
                    coverage_class: triplet[2],
                });
            }
        }

        subbands
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Environment {
    Any,
    Outdoor,
    Indoor,
}

impl From<u8> for Environment {
    fn from(value: u8) -> Self {
        match value as char {
            'O' => Environment::Outdoor,
            'I' => Environment::Indoor,
            _ => Environment::Any,
        }
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::Any => write!(f, "Any"),
            Environment::Outdoor => write!(f, "Outdoor"),
            Environment::Indoor => write!(f, "Indoor"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OperatingInfo {
    pub operating_extension_id: u8,
    pub operating_class: u8,
    pub coverage_class: u8,
}

impl OperatingInfo {
    pub fn air_propagation_time_us(&self) -> Option<u16> {
        match self.coverage_class {
            0..=31 => Some(self.coverage_class as u16 * 3),
            _ => None,
        }
    }
}

impl Display for OperatingInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Operating Info:\n\tOperating Extension ID: {}\n\tOperating Class: {}\r\n\tCoverage Class: {}\r\n\t",
            self.operating_extension_id, self.operating_class, self.coverage_class
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubbandInfo {
    pub first_channel_number: u8,
    pub number_of_channels: u8,
    pub max_transmit_power_level_dbm: i8,
    pub operating_info: Option<OperatingInfo>,
}

impl Display for SubbandInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Subband Info:\n\tFirst Channel Number: {}\n\tNumber of Channels: {}\n\tMaximum Transmit Power Level: {} dBm",
            self.first_channel_number, self.number_of_channels, self.max_transmit_power_level_dbm
        )
    }
}
