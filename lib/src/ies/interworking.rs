use std::fmt::Display;

use deku::{DekuContainerWrite, DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::IeId;
use crate::{BitRange, Field};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "len: usize")]
pub struct Interworking {
    pub access_network_options: AccessNetworkOptions,
    #[deku(cond = "len == 3 || len == 9")]
    pub venue_info: Option<VenueInfo>,
    #[deku(cond = "len == 7 || len == 9")]
    pub hessid: Option<[u8; 6]>,
}

impl Interworking {
    pub const ID: u8 = 107;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const NAME: &'static str = "Interworking";

    pub fn summary(&self) -> String {
        format!(
            "Access Network Type: {}",
            self.access_network_options.access_network_type
        )
    }

    pub fn fields(&self) -> Vec<Field> {
        let mut fields = vec![self.access_network_options.to_field()];

        if let Some(venue_info) = self.venue_info {
            fields.push(venue_info.to_field());
        }

        if let Some(hessid) = self.hessid {
            fields.push(
                Field::builder()
                    .title("HESSID")
                    .value(
                        hessid
                            .iter()
                            .map(|byte| format!("{:02X}", byte))
                            .collect::<Vec<String>>()
                            .join(":"),
                    )
                    .bytes(hessid.to_vec())
                    .build(),
            );
        }

        fields
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct AccessNetworkOptions {
    #[deku(bits = 4)]
    pub access_network_type: u8,
    #[deku(bits = 1)]
    pub internet: bool,
    #[deku(bits = 1)]
    pub asra: bool,
    #[deku(bits = 1)]
    pub esr: bool,
    #[deku(bits = 1)]
    pub uesa: bool,
}

impl AccessNetworkOptions {
    pub fn to_field(&self) -> Field {
        let byte = self
            .to_bytes()
            .unwrap_or_default()
            .first()
            .cloned()
            .unwrap_or_default();

        Field::builder()
            .title("Access Network Options")
            .value(format!("Type: {}", self.access_network_type))
            .byte(byte)
            .subfields([
                Field::builder()
                    .title("Access Network Type")
                    .value(self.access_network_type)
                    .bits(BitRange::new(&[byte], 0, 4))
                    .build(),
                Field::builder()
                    .title("Internet")
                    .value(self.internet)
                    .bits(BitRange::new(&[byte], 4, 1))
                    .build(),
                Field::builder()
                    .title("ASRA")
                    .value(self.asra)
                    .bits(BitRange::new(&[byte], 5, 1))
                    .build(),
                Field::builder()
                    .title("ESR")
                    .value(self.esr)
                    .bits(BitRange::new(&[byte], 6, 1))
                    .build(),
                Field::builder()
                    .title("UESA")
                    .value(self.uesa)
                    .bits(BitRange::new(&[byte], 7, 1))
                    .build(),
            ])
            .build()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct VenueInfo {
    pub venue_group: VenueGroup,
    #[deku(bytes = 1)]
    pub venue_type: u8,
}

impl VenueInfo {
    pub fn to_field(&self) -> Field {
        let bytes = self.to_bytes().unwrap_or_default();

        Field::builder()
            .title("Venue Info")
            .value(format!("{} ({})", self.venue_group, self.venue_type))
            .bytes(bytes.clone())
            .subfields([
                Field::builder()
                    .title("Venue Group")
                    .value(self.venue_group)
                    .byte(self.venue_group as u8)
                    .build(),
                Field::builder()
                    .title("Venue Type")
                    .value(self.venue_type)
                    .byte(self.venue_type)
                    .build(),
            ])
            .build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[repr(u8)]
#[deku(id_type = "u8")]
pub enum VenueGroup {
    Unspecified = 0,
    Assembly,
    Business,
    Educational,
    FactoryAndIndustrial,
    Institutional,
    Mercantile,
    Residential,
    Storage,
    UtilityAndMiscellaneous,
    Vehicular,
    Outdoor,
    #[deku(id_pat = "_")]
    Reserved,
}

impl Display for VenueGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unspecified => write!(f, "Unspecified"),
            Self::Assembly => write!(f, "Assembly"),
            Self::Business => write!(f, "Business"),
            Self::Educational => write!(f, "Educational"),
            Self::FactoryAndIndustrial => write!(f, "Factory and Industrial"),
            Self::Institutional => write!(f, "Institutional"),
            Self::Mercantile => write!(f, "Mercantile"),
            Self::Residential => write!(f, "Residential"),
            Self::Storage => write!(f, "Storage"),
            Self::UtilityAndMiscellaneous => write!(f, "Utility and Miscellaneous"),
            Self::Vehicular => write!(f, "Vehicular"),
            Self::Outdoor => write!(f, "Outdoor"),
            Self::Reserved => write!(f, "Reserved"),
        }
    }
}
