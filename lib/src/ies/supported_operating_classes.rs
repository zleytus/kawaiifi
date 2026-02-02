use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::IeId;
use crate::Field;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "len: usize")]
pub struct SupportedOperatingClasses {
    #[deku(bytes = 1)]
    pub current_operating_class: u8,
    #[deku(count = "len.checked_sub(1).unwrap_or_default()")]
    pub operating_classes: Vec<u8>,
}

impl SupportedOperatingClasses {
    pub const NAME: &'static str = "Supported Operating Classes";
    pub const ID: u8 = 59;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);

    pub fn summary(&self) -> String {
        format!("Current Class: {}", self.current_operating_class)
    }

    pub fn fields(&self) -> Vec<Field> {
        vec![
            Field::builder()
                .title("Current Operating Class")
                .value(self.current_operating_class)
                .byte(self.current_operating_class)
                .build(),
            Field::builder()
                .title("Operating Classes")
                .value(format!(
                    "{}",
                    self.operating_classes
                        .iter()
                        .map(|operating_class| operating_class.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                ))
                .bytes(self.operating_classes.clone())
                .build(),
        ]
    }
}
