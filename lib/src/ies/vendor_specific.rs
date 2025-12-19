use std::ops::Deref;

use deku::prelude::*;
use serde::{Deserialize, Serialize};

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "len: usize")]
pub struct VendorSpecific {
    #[deku(count = "len")]
    data: Vec<u8>,
}

impl VendorSpecific {
    pub const NAME: &'static str = "Vendor Specific";
    pub const ID: u8 = 221;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);

    pub fn is_wpa(&self) -> bool {
        self.data.len() >= 4
            && self.data[0..3] == [0x00, 0x50, 0xF2] // Microsoft's OUI
            && self.data[3] == 0x01 // WPA type
    }
}

impl Deref for VendorSpecific {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
