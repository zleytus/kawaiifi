use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::IeId;
use crate::Field;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct DsParameterSet {
    #[deku(bytes = 1)]
    pub current_channel: u8,
}

impl DsParameterSet {
    pub const NAME: &'static str = "DS Parameter Set";
    pub const ID: u8 = 3;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const LENGTH: usize = 1;

    pub fn summary(&self) -> String {
        format!("Current Channel: {}", self.current_channel)
    }

    pub fn fields(&self) -> Vec<Field> {
        vec![
            Field::builder()
                .title("Current Channel")
                .value(self.current_channel)
                .byte(self.current_channel)
                .build(),
        ]
    }
}
