use deku::{DekuContainerWrite, DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::IeId;
use crate::{BitRange, Field};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct SpatialReuseParameterSet {
    pub sr_control: SrControl,
    #[deku(cond = "sr_control.non_srg_offset_present", bytes = "1")]
    pub non_srg_obss_pd_max_offset: Option<u8>,
    #[deku(cond = "sr_control.srg_information_present", bytes = "1")]
    pub srg_obss_pd_min_offset: Option<u8>,
    #[deku(cond = "sr_control.srg_information_present", bytes = "1")]
    pub srg_obss_pd_max_offset: Option<u8>,
    #[deku(cond = "sr_control.srg_information_present", bytes = "8")]
    pub srg_bss_color_bitmap: Option<[u8; 8]>,
    #[deku(cond = "sr_control.srg_information_present", bytes = "8")]
    pub srg_partial_bssid_bitmap: Option<[u8; 8]>,
}

impl SpatialReuseParameterSet {
    pub const NAME: &'static str = "Spatial Reuse Parameter Set";
    pub const ID: u8 = 255;
    pub const ID_EXT: Option<u8> = Some(39);
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);

    pub fn summary(&self) -> String {
        "".to_string()
    }

    pub fn fields(&self) -> Vec<Field> {
        let mut fields = vec![self.sr_control.to_field()];

        if let Some(offset) = self.non_srg_obss_pd_max_offset {
            fields.push(
                Field::builder()
                    .title("Non-SRG OBSS PD Max Offset")
                    .value(offset)
                    .byte(offset)
                    .build(),
            );
        }

        if let Some(offset) = self.srg_obss_pd_min_offset {
            fields.push(
                Field::builder()
                    .title("SRG OBSS PD Min Offset")
                    .value(offset)
                    .byte(offset)
                    .build(),
            );
        }

        if let Some(offset) = self.srg_obss_pd_max_offset {
            fields.push(
                Field::builder()
                    .title("SRG OBSS PD Max Offset")
                    .value(offset)
                    .byte(offset)
                    .build(),
            );
        }

        fields
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct SrControl {
    #[deku(bits = 1)]
    pub psr_disallowed: bool,
    #[deku(bits = 1)]
    pub non_srg_obss_pd_sr_disallowed: bool,
    #[deku(bits = 1)]
    pub non_srg_offset_present: bool,
    #[deku(bits = 1)]
    pub srg_information_present: bool,
    #[deku(bits = 1)]
    pub hesiga_spatial_reuse_value15_allowed: bool,
    #[deku(bits = 3)]
    reserved: u8,
}

impl SrControl {
    pub fn to_field(&self) -> Field {
        let byte = self
            .to_bytes()
            .unwrap_or_default()
            .first()
            .cloned()
            .unwrap_or_default();
        Field::builder()
            .title("SR Control")
            .value("")
            .byte(byte)
            .subfields([
                Field::builder()
                    .title("PSR Disallowed")
                    .value(self.psr_disallowed)
                    .bits(BitRange::new(&[byte], 0, 1))
                    .build(),
                Field::builder()
                    .title("Non-SRG OBSS PD SR Disallowed")
                    .value(self.non_srg_obss_pd_sr_disallowed)
                    .bits(BitRange::new(&[byte], 1, 1))
                    .build(),
                Field::builder()
                    .title("Non-SRG Offset Present")
                    .value(self.non_srg_offset_present)
                    .bits(BitRange::new(&[byte], 2, 1))
                    .build(),
                Field::builder()
                    .title("SRG Information Present")
                    .value(self.srg_information_present)
                    .bits(BitRange::new(&[byte], 3, 1))
                    .build(),
                Field::builder()
                    .title("HESIGA Spatial Reuse Value 15 Allowed")
                    .value(self.hesiga_spatial_reuse_value15_allowed)
                    .bits(BitRange::new(&[byte], 4, 1))
                    .build(),
                Field::reserved(BitRange::new(&[byte], 5, 3)),
            ])
            .build()
    }
}
