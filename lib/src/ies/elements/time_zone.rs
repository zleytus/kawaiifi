use std::{
    ops::Deref,
    str::{self, Utf8Error},
};

use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use crate::{Field, ies::IeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "len: usize")]
pub struct TimeZone {
    #[deku(count = "len")]
    time_zone: Vec<u8>,
}

impl TimeZone {
    pub const NAME: &'static str = "Time Zone";
    pub const ID: u8 = 98;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);

    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(&self.time_zone)
    }

    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(&self.time_zone).into_owned()
    }

    pub fn summary(&self) -> String {
        self.to_string_lossy()
    }

    pub fn fields(&self) -> Vec<Field> {
        vec![Field::new("Time Zone", self.to_string_lossy())]
    }
}

impl Deref for TimeZone {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.time_zone
    }
}
