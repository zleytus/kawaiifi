use deku::prelude::*;
use serde::{Deserialize, Serialize};

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReducedNeighborReport {
    pub neighbor_ap_information_fields: Vec<NeighborApInformationField>,
}

impl ReducedNeighborReport {
    pub const NAME: &'static str = "Reduced Neighbor Report";
    pub const ID: u8 = 201;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
}

impl DekuReader<'_, usize> for ReducedNeighborReport {
    fn from_reader_with_ctx<R: deku::no_std_io::Read + deku::no_std_io::Seek>(
        reader: &mut Reader<R>,
        len: usize,
    ) -> Result<Self, DekuError>
    where
        Self: Sized,
    {
        let start_bit = reader.bits_read;
        let end_bit = start_bit + (len * 8);
        let mut neighbor_ap_information_fields = Vec::new();

        while reader.bits_read < end_bit {
            let neighbor_ap_information_field =
                NeighborApInformationField::from_reader_with_ctx(reader, ())?;
            neighbor_ap_information_fields.push(neighbor_ap_information_field);
        }

        Ok(Self {
            neighbor_ap_information_fields,
        })
    }
}

impl DekuWriter<usize> for ReducedNeighborReport {
    fn to_writer<W: deku::no_std_io::Write + deku::no_std_io::Seek>(
        &self,
        writer: &mut Writer<W>,
        _: usize,
    ) -> Result<(), DekuError> {
        for neighbor_ap_information_field in &self.neighbor_ap_information_fields {
            neighbor_ap_information_field.to_writer(writer, ())?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct NeighborApInformationField {
    pub tbtt_information_header: TbttInformationHeader,
    pub operating_class: u8,
    pub channel_number: u8,
    #[deku(
        count = "tbtt_information_header.tbtt_information_count + 1",
        ctx = "tbtt_information_header.tbtt_information_length"
    )]
    pub tbtt_information_set: Vec<TbttInformation>,
}

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct TbttInformationHeader {
    #[deku(bits = 2)]
    tbtt_information_field_type: u8,
    #[deku(bits = 1)]
    filtered_neighbor_ap: bool,
    #[deku(bits = 1)]
    reserved: bool,
    #[deku(bits = 4)]
    tbtt_information_count: u8,
    #[deku(bits = 8)]
    tbtt_information_length: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "tbtt_information_length: u8")]
pub struct TbttInformation {
    #[deku(bytes = 1)]
    pub neighbor_ap_tbtt_offset: u8,
    #[deku(
        cond = "[7, 8, 9, 11, 12, 13].contains(&tbtt_information_length) || tbtt_information_length > 13",
        bytes = 6
    )]
    pub bssid: Option<[u8; 6]>,
    #[deku(
        cond = "[5, 11, 12, 13].contains(&tbtt_information_length) || tbtt_information_length > 13",
        bytes = 4
    )]
    pub short_ssid: Option<[u8; 4]>,
    #[deku(
        cond = "[2, 6, 8, 9, 12, 13].contains(&tbtt_information_length) || tbtt_information_length > 13"
    )]
    pub bss_parameters: Option<BssParameters>,
    #[deku(
        cond = "[9, 13].contains(&tbtt_information_length) || tbtt_information_length > 13",
        bytes = 1
    )]
    pub twenty_mhz_psd: Option<u8>,
    #[deku(
        cond = "tbtt_information_length > 13",
        count = "tbtt_information_length.checked_sub(13).unwrap_or_default()"
    )]
    reserved: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct BssParameters {
    #[deku(bits = 1)]
    oct_recommended: bool,
    #[deku(bits = 1)]
    same_ssid: bool,
    #[deku(bits = 1)]
    multiple_bssid: bool,
    #[deku(bits = 1)]
    transmitted_bssid: bool,
    #[deku(bits = 1)]
    member_of_ess_with_2_point_4_or_5_ghz_co_located_ap: bool,
    #[deku(bits = 1)]
    unsolicited_probe_responses_active: bool,
    #[deku(bits = 1)]
    co_located_ap: bool,
    #[deku(bits = 1)]
    reserved: bool,
}
