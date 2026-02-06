use deku::prelude::*;
use serde::{Deserialize, Serialize};

use crate::ies::{BitRange, Field, IeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReducedNeighborReport {
    pub neighbor_ap_information_fields: Vec<NeighborApInformationField>,
}

impl ReducedNeighborReport {
    pub const NAME: &'static str = "Reduced Neighbor Report";
    pub const ID: u8 = 201;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);

    pub fn summary(&self) -> String {
        match self.neighbor_ap_information_fields.len() {
            1 => "1 Report".to_string(),
            reports => format!("{} Reports", reports),
        }
    }

    pub fn fields(&self) -> Vec<Field> {
        self.neighbor_ap_information_fields
            .iter()
            .map(|neighbor_ap_information_field| neighbor_ap_information_field.to_field())
            .collect()
    }
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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

impl NeighborApInformationField {
    pub fn to_field(&self) -> Field {
        let mut subfields = vec![
            self.tbtt_information_header.to_field(),
            Field::builder()
                .title("Operating Class")
                .value(self.operating_class)
                .byte(self.operating_class)
                .build(),
            Field::builder()
                .title("Channel Number")
                .value(self.channel_number)
                .byte(self.channel_number)
                .build(),
        ];
        subfields.extend(self.tbtt_information_set.iter().map(|tbtt_information| {
            tbtt_information.to_field(self.tbtt_information_header.tbtt_information_length)
        }));

        Field::builder()
            .title("Neighbor AP Information Field")
            .value("")
            .subfields(subfields)
            .bytes(self.to_bytes().unwrap_or_default())
            .build()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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

impl TbttInformationHeader {
    pub fn to_field(&self) -> Field {
        let bytes = self.to_bytes().unwrap_or_default();

        Field::builder()
            .title("TBTT Information Header")
            .value("")
            .subfields([
                Field::builder()
                    .title("TBTT Information Field Type")
                    .value(self.tbtt_information_field_type)
                    .bits(BitRange::new(&bytes, 0, 2))
                    .build(),
                Field::builder()
                    .title("Filtered Neighbor AP")
                    .value(self.filtered_neighbor_ap)
                    .bits(BitRange::new(&bytes, 2, 1))
                    .build(),
                Field::reserved(BitRange::new(&bytes, 3, 1)),
                Field::builder()
                    .title("TBTT Information Count")
                    .value(self.tbtt_information_count)
                    .bits(BitRange::new(&bytes, 4, 4))
                    .build(),
                Field::builder()
                    .title("TBTT Information Length")
                    .value(self.tbtt_information_length)
                    .bits(BitRange::new(&bytes, 8, 8))
                    .build(),
            ])
            .bytes(bytes)
            .build()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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
    pub twenty_mhz_psd: Option<i8>,
    #[deku(
        cond = "tbtt_information_length > 13",
        count = "tbtt_information_length.checked_sub(13).unwrap_or_default()"
    )]
    reserved: Option<Vec<u8>>,
}

impl TbttInformation {
    pub fn to_field(&self, tbtt_information_length: u8) -> Field {
        let mut subfields = vec![
            Field::builder()
                .title("Neighbor AP TBTT Offset")
                .value(self.neighbor_ap_tbtt_offset)
                .byte(self.neighbor_ap_tbtt_offset)
                .build(),
        ];

        if let Some(bssid) = self.bssid {
            subfields.push(
                Field::builder()
                    .title("BSSID")
                    .value(
                        bssid
                            .iter()
                            .map(|byte| format!("{:02X}", byte))
                            .collect::<Vec<String>>()
                            .join(":"),
                    )
                    .bytes(bssid.to_vec())
                    .build(),
            );
        }

        if let Some(short_ssid) = self.short_ssid {
            subfields.push(
                Field::builder()
                    .title("Short SSID")
                    .value(format!("{:#?}", short_ssid))
                    .bytes(short_ssid.to_vec())
                    .build(),
            );
        }

        if let Some(bss_parameters) = self.bss_parameters {
            subfields.push(bss_parameters.to_field());
        }

        if let Some(twenty_mhz_psd) = self.twenty_mhz_psd {
            subfields.push(
                Field::builder()
                    .title("20 MHz PSD")
                    .value(
                        format!("{:.1}", f32::from(twenty_mhz_psd) / 2.0)
                            .trim_end_matches("0")
                            .trim_end_matches("."),
                    )
                    .byte(twenty_mhz_psd as u8)
                    .build(),
            );
        }

        // Get bytes by writing with context
        let bytes = {
            let mut cursor = std::io::Cursor::new(Vec::new());
            let mut writer = Writer::new(&mut cursor);
            let _ = self.to_writer(&mut writer, tbtt_information_length);
            cursor.into_inner()
        };

        Field::builder()
            .title("TBTT Information")
            .value("")
            .subfields(subfields)
            .bytes(bytes)
            .build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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

impl BssParameters {
    pub fn to_field(&self) -> Field {
        let byte = self
            .to_bytes()
            .unwrap_or_default()
            .first()
            .cloned()
            .unwrap_or_default();

        Field::builder()
            .title("BSS Parameters")
            .value("")
            .subfields([
                Field::builder()
                    .title("OCT Recommended")
                    .value(self.oct_recommended)
                    .bits(BitRange::from_byte(byte, 0, 1))
                    .build(),
                Field::builder()
                    .title("Same SSID")
                    .value(self.same_ssid)
                    .bits(BitRange::from_byte(byte, 1, 1))
                    .build(),
                Field::builder()
                    .title("Multiple BSSID")
                    .value(self.multiple_bssid)
                    .bits(BitRange::from_byte(byte, 2, 1))
                    .build(),
                Field::builder()
                    .title("Transmitted BSSID")
                    .value(self.transmitted_bssid)
                    .bits(BitRange::from_byte(byte, 3, 1))
                    .build(),
                Field::builder()
                    .title("Member of ESS With 2.4/5 GHz Co-Located AP")
                    .value(self.member_of_ess_with_2_point_4_or_5_ghz_co_located_ap)
                    .bits(BitRange::from_byte(byte, 4, 1))
                    .build(),
                Field::builder()
                    .title("Unsolicited Probe Responses Active")
                    .value(self.unsolicited_probe_responses_active)
                    .bits(BitRange::from_byte(byte, 5, 1))
                    .build(),
                Field::builder()
                    .title("Co-Located AP")
                    .value(self.co_located_ap)
                    .bits(BitRange::from_byte(byte, 6, 1))
                    .build(),
                Field::reserved(BitRange::from_byte(byte, 7, 1)),
            ])
            .byte(byte)
            .build()
    }
}
