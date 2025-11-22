use deku::prelude::*;

use super::{IeId, VendorSpecific};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdvertisementProtocol {
    pub advertisement_protocol_tuple_list: Vec<AdvertisementProtocolTuple>,
}

impl AdvertisementProtocol {
    pub const NAME: &'static str = "Advertisement Protocol";
    pub const ID: u8 = 108;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
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

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
pub struct AdvertisementProtocolTuple {
    pub query_response_info: QueryResponseInfo,
    pub advertisement_protocol_id: AdvertisementProtocolId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(bit_order = "lsb")]
pub struct QueryResponseInfo {
    #[deku(bits = 7)]
    pub query_response_length_limit: u8,
    #[deku(bits = 1)]
    pub pame_bi: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
pub struct AdvertisementProtocolId {
    #[deku(bytes = 1)]
    pub id: u8,
    #[deku(cond = "*id == VendorSpecific::ID", bytes = 1)]
    pub len: Option<u8>,
    #[deku(
        cond = "*id == VendorSpecific::ID",
        ctx = "usize::from(len.unwrap_or_default())"
    )]
    pub vendor_specific: Option<VendorSpecific>,
}
