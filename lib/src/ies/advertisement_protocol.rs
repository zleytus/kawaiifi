use std::fmt::Display;

use deku::DekuContainerWrite;
use deku::prelude::*;
use serde::{Deserialize, Serialize};

use super::{IeId, VendorSpecific};
use crate::{BitRange, Field};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AdvertisementProtocol {
    pub advertisement_protocol_tuple_list: Vec<AdvertisementProtocolTuple>,
}

impl AdvertisementProtocol {
    pub const NAME: &'static str = "Advertisement Protocol";
    pub const ID: u8 = 108;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);

    pub fn summary(&self) -> String {
        self.advertisement_protocol_tuple_list
            .first()
            .map(|tuple| tuple.advertisement_protocol_id.to_string())
            .unwrap_or_default()
    }

    pub fn fields(&self) -> Vec<Field> {
        self.advertisement_protocol_tuple_list
            .iter()
            .map(|tuple| tuple.to_field())
            .collect()
    }
}

// We need custom implementations of DekuReader and DekuWriter because the number
// and size of AdvertisementProtocolTuples can't be known ahead of time
impl DekuReader<'_, usize> for AdvertisementProtocol {
    fn from_reader_with_ctx<R: deku::no_std_io::Read + deku::no_std_io::Seek>(
        reader: &mut Reader<R>,
        len: usize,
    ) -> Result<Self, DekuError>
    where
        Self: Sized,
    {
        let start_bit = reader.bits_read;
        let end_bit = start_bit + (len * 8);
        let mut tuples = Vec::new();

        while reader.bits_read < end_bit {
            let tuple = AdvertisementProtocolTuple::from_reader_with_ctx(reader, ())?;
            tuples.push(tuple);
        }

        Ok(Self {
            advertisement_protocol_tuple_list: tuples,
        })
    }
}

impl DekuWriter<usize> for AdvertisementProtocol {
    fn to_writer<W: deku::no_std_io::Write + deku::no_std_io::Seek>(
        &self,
        writer: &mut Writer<W>,
        _: usize,
    ) -> Result<(), DekuError> {
        for tuple in &self.advertisement_protocol_tuple_list {
            tuple.to_writer(writer, ())?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct AdvertisementProtocolTuple {
    pub query_response_info: QueryResponseInfo,
    pub advertisement_protocol_id: AdvertisementProtocolId,
}

impl AdvertisementProtocolTuple {
    pub fn to_field(&self) -> Field {
        Field::builder()
            .title("Advertisement Protocol Tuple")
            .value(self.advertisement_protocol_id.to_string())
            .bytes(self.to_bytes().unwrap_or_default())
            .subfields([
                self.query_response_info.to_field(),
                self.advertisement_protocol_id.to_field(),
            ])
            .build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct QueryResponseInfo {
    #[deku(bits = 7)]
    pub query_response_length_limit: u8,
    #[deku(bits = 1)]
    pub pame_bi: bool,
}

impl QueryResponseInfo {
    pub fn to_field(&self) -> Field {
        let byte = self
            .to_bytes()
            .unwrap_or_default()
            .first()
            .cloned()
            .unwrap_or_default();
        Field::builder()
            .title("Query Response Info")
            .value("")
            .subfields([
                Field::builder()
                    .title("Query Response Length Limit")
                    .value(self.query_response_length_limit)
                    .bits(BitRange::from_byte(byte, 0, 7))
                    .build(),
                Field::builder()
                    .title("PAME-BI")
                    .value(self.pame_bi)
                    .bits(BitRange::from_byte(byte, 7, 1))
                    .build(),
            ])
            .byte(byte)
            .build()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(id_type = "u8")]
pub enum AdvertisementProtocolId {
    #[deku(id = 0)]
    AccessNetworkQueryProtocol,
    #[deku(id = 1)]
    MisInformationService,
    #[deku(id = 2)]
    MisCommandAndEventServicesCapabilityDiscovery,
    #[deku(id = 3)]
    EmergencyAlertSystem,
    #[deku(id = 4)]
    RegisteredLocationQueryProtocol,
    #[deku(id = 221)]
    VendorSpecific {
        id: u8,
        len: u8,
        #[deku(ctx = "usize::from(*len)")]
        vendor_specific: VendorSpecific,
    },
    #[deku(id_pat = "_")]
    Reserved(u8),
}

impl AdvertisementProtocolId {
    pub fn to_field(&self) -> Field {
        let bytes = self.to_bytes().unwrap_or_default();
        let id = bytes.first().cloned().unwrap_or_default();
        match self {
            Self::AccessNetworkQueryProtocol => Field::builder()
                .title("Advertisement Protocol ID")
                .value(self.to_string())
                .units(format!("({})", id))
                .byte(id)
                .build(),
            Self::MisInformationService => Field::builder()
                .title("Advertisement Protocol ID")
                .value(self.to_string())
                .units(format!("({})", id))
                .byte(id)
                .build(),
            Self::MisCommandAndEventServicesCapabilityDiscovery => Field::builder()
                .title("Advertisement Protocol ID")
                .value(self.to_string())
                .units(format!("({})", id))
                .byte(id)
                .build(),
            Self::EmergencyAlertSystem => Field::builder()
                .title("Advertisement Protocol ID")
                .value(self.to_string())
                .units(format!("({})", id))
                .byte(id)
                .build(),
            Self::RegisteredLocationQueryProtocol => Field::builder()
                .title("Advertisement Protocol ID")
                .value(self.to_string())
                .units(format!("({})", id))
                .byte(id)
                .build(),
            Self::VendorSpecific {
                id,
                len,
                vendor_specific,
            } => Field::builder()
                .title("Advertisement Protocol ID")
                .value(self.to_string())
                .units(format!("({})", id))
                .bytes(self.to_bytes().unwrap_or_default())
                .subfields({
                    let mut subfields = vec![
                        Field::builder().title("ID").value(id).byte(*id).build(),
                        Field::builder()
                            .title("Length")
                            .value(len)
                            .units(if *len == 1 { "bytes" } else { "bytes" })
                            .byte(*len)
                            .build(),
                    ];
                    subfields.extend(vendor_specific.fields().iter().cloned());
                    subfields
                })
                .build(),
            Self::Reserved(id) => Field::builder()
                .title("Advertisement Protocol ID")
                .value(self.to_string())
                .units(format!("({})", id))
                .byte(*id)
                .build(),
        }
    }
}

impl Display for AdvertisementProtocolId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AccessNetworkQueryProtocol => write!(f, "Access Network Query Protocol (ANQP)"),
            Self::MisInformationService => write!(f, "MIS Information Service"),
            Self::MisCommandAndEventServicesCapabilityDiscovery => {
                write!(f, "MIS Command and Event Services Capability Discovery")
            }
            Self::EmergencyAlertSystem => write!(f, "Emergency Alert System (EAS)"),
            Self::RegisteredLocationQueryProtocol => {
                write!(f, "Registered Location Query Protocol (RLQP)")
            }
            Self::VendorSpecific {
                id: _,
                len: _,
                vendor_specific: _,
            } => write!(f, "Vendor Specific"),
            Self::Reserved(_) => write!(f, "Reserved"),
        }
    }
}
