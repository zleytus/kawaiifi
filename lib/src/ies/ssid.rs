use std::{
    ops::Deref,
    str::{self, Utf8Error},
};

use deku::prelude::*;

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(ctx = "len: usize")]
pub struct Ssid {
    #[deku(count = "len")]
    ssid: Vec<u8>,
}

impl Ssid {
    pub const NAME: &'static str = "SSID";
    pub const ID: u8 = 0;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);

    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(&self.ssid)
    }

    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(&self.ssid).into_owned()
    }
}

impl Deref for Ssid {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.ssid
    }
}
