use std::ops::Deref;

use deku::prelude::*;
use serde::{Deserialize, Serialize};

use crate::ies::{Field, IeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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

    pub fn oui(&self) -> Option<&[u8; 3]> {
        self.data.get(0..3).and_then(|slice| slice.try_into().ok())
    }

    pub fn summary(&self) -> String {
        self.oui()
            .map(|oui| format!("OUI: {:02X}:{:02X}:{:02X}", oui[0], oui[1], oui[2]))
            .unwrap_or_default()
    }

    pub fn fields(&self) -> Vec<Field> {
        let mut fields = Vec::new();

        if let Some(oui) = self.oui() {
            fields.push(
                Field::builder()
                    .title("OUI")
                    .value(format!("{:02X}:{:02X}:{:02X}", oui[0], oui[1], oui[2]))
                    .bytes(oui.to_vec())
                    .build(),
            );
        }

        if let Some(data) = self.data.get(3..) {
            fields.push(
                Field::builder()
                    .title("Data")
                    .value("---")
                    .bytes(data.to_vec())
                    .build(),
            );
        }

        fields
    }
}

impl Deref for VendorSpecific {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
