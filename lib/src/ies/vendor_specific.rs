use std::ops::Deref;

use deku::prelude::*;

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
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
}

impl Deref for VendorSpecific {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
