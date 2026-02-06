use std::{fmt::Display, str};

use deku::{DekuContainerWrite, DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use crate::{Field, ies::IeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "len: usize")]
pub struct Country {
    #[deku(bytes = 2)]
    pub country_code: [u8; 2],
    #[deku(bytes = 1)]
    pub environment: u8,
    #[deku(count = "len.checked_sub(3).unwrap_or_default() / 3")]
    pub triplets: Vec<Triplet>,
    #[deku(cond = "len.checked_sub(3 + triplets.len() * 3).unwrap_or_default() == 1")]
    padding: Option<u8>,
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

    pub fn summary(&self) -> String {
        format!("{} ({})", self.country_code(), self.environment())
    }

    pub fn fields(&self) -> Vec<Field> {
        let mut fields = vec![
            Field::builder()
                .title("Country Code")
                .value(self.country_code())
                .bytes(self.country_code.to_vec())
                .build(),
            Field::builder()
                .title("Environment")
                .value(self.environment())
                .byte(self.environment)
                .build(),
        ];
        fields.extend(self.triplets.iter().map(|triplet| triplet.to_field()));

        fields
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(id_type = "u8")]
pub enum Triplet {
    #[deku(id_pat = "0..=200")]
    SubbandInfo {
        first_channel_number: u8,
        number_of_channels: u8,
        max_transmit_power_level: i8,
    },
    #[deku(id_pat = "_")]
    OperatingInfo {
        operating_extension_id: u8,
        operating_class: u8,
        coverage_class: u8,
    },
}

impl Triplet {
    pub fn to_field(&self) -> Field {
        match self {
            Self::SubbandInfo {
                first_channel_number,
                number_of_channels,
                max_transmit_power_level,
            } => Field::builder()
                .title("Subband Info")
                .value("")
                .subfields([
                    Field::builder()
                        .title("First Channel Number")
                        .value(first_channel_number)
                        .byte(*first_channel_number)
                        .build(),
                    Field::builder()
                        .title("Number of Channels")
                        .value(number_of_channels)
                        .byte(*number_of_channels)
                        .build(),
                    Field::builder()
                        .title("Max Tx Power Level")
                        .value(max_transmit_power_level)
                        .units("dBm")
                        .byte(*max_transmit_power_level as u8)
                        .build(),
                ])
                .bytes(self.to_bytes().unwrap_or_default())
                .build(),
            Self::OperatingInfo {
                operating_extension_id,
                operating_class,
                coverage_class,
            } => Field::builder()
                .title("Operating Info")
                .value("")
                .subfields([
                    Field::builder()
                        .title("Operating Extension ID")
                        .value(operating_extension_id)
                        .byte(*operating_extension_id)
                        .build(),
                    Field::builder()
                        .title("Operating Class")
                        .value(operating_class)
                        .byte(*operating_class)
                        .build(),
                    Field::builder()
                        .title("Coverage Class")
                        .value(coverage_class)
                        .byte(*coverage_class)
                        .build(),
                ])
                .bytes(self.to_bytes().unwrap_or_default())
                .build(),
        }
    }
}
