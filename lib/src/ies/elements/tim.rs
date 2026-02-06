use deku::{DekuContainerWrite, DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use crate::{BitRange, Field, ies::IeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "len: usize")]
pub struct Tim {
    #[deku(bytes = 1)]
    pub dtim_count: u8,
    #[deku(bytes = 1)]
    pub dtim_period: u8,
    #[deku(cond = "len >= 3")]
    pub bitmap_control: Option<BitmapControl>,
    #[deku(cond = "len >= 4", count = "len.checked_sub(3).unwrap_or_default()")]
    pub partial_virtual_bitmap: Option<Vec<u8>>,
}

impl Tim {
    pub const NAME: &'static str = "TIM";
    pub const ID: u8 = 5;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);

    pub fn is_dtim(&self) -> bool {
        self.dtim_count == 0
    }

    pub fn summary(&self) -> String {
        format!(
            "DTIM Count: {}, DTIM Period: {}",
            self.dtim_count, self.dtim_period
        )
    }

    pub fn fields(&self) -> Vec<Field> {
        let mut fields = vec![
            Field::builder()
                .title("DTIM Count")
                .value(self.dtim_count)
                .byte(self.dtim_count)
                .build(),
            Field::builder()
                .title("DTIM Period")
                .value(self.dtim_period)
                .byte(self.dtim_period)
                .build(),
        ];

        if let Some(bitmap_control) = self.bitmap_control {
            fields.push(bitmap_control.to_field());
        }

        fields
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct BitmapControl {
    #[deku(bits = 1)]
    pub traffic_indicator: bool,
    #[deku(bits = 7)]
    pub bitmap_offset: u8,
}

impl BitmapControl {
    pub fn to_field(&self) -> Field {
        let byte = self
            .to_bytes()
            .unwrap_or_default()
            .first()
            .cloned()
            .unwrap_or_default();
        Field::builder()
            .title("Bitmap Control")
            .value("")
            .byte(byte)
            .subfields([
                Field::builder()
                    .title("Traffic Indicator")
                    .value(self.traffic_indicator)
                    .bits(BitRange::from_byte(byte, 0, 1))
                    .build(),
                Field::builder()
                    .title("Bitmap Offset")
                    .value(self.bitmap_offset)
                    .bits(BitRange::from_byte(byte, 1, 7))
                    .build(),
            ])
            .build()
    }
}
