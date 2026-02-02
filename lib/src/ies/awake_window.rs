use deku::prelude::*;
use serde::{Deserialize, Serialize};

use super::IeId;
use crate::Field;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct AwakeWindow {
    #[deku(bytes = 2)]
    pub awake_window_duration_micros: u16,
}

impl AwakeWindow {
    pub const ID: u8 = 157;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const NAME: &'static str = "Awake Window";

    pub fn summary(&self) -> String {
        format!("Duration: {} μs", self.awake_window_duration_micros)
    }

    pub fn fields(&self) -> Vec<Field> {
        vec![
            Field::builder()
                .title("Awake Window Duration")
                .value(self.awake_window_duration_micros)
                .units("μs")
                .build(),
        ]
    }
}
