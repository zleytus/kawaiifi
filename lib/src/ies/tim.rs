use deku::{DekuRead, DekuWrite};

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite)]
pub struct BitmapControl {
    #[deku(bits = 1)]
    pub traffic_indicator: u8,
    #[deku(bits = 7)]
    pub bitmap_offset: u8,
}
