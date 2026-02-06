use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use crate::ies::{Field, IeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct IbssParameterSet {
    #[deku(bytes = 2)]
    pub atim_window_tu: u16,
}

impl IbssParameterSet {
    pub const NAME: &'static str = "IBSS Parameter Set";
    pub const ID: u8 = 6;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const LENGTH: usize = 2;

    pub fn summary(&self) -> String {
        format!("ATIM Window: {} TU", self.atim_window_tu)
    }

    pub fn fields(&self) -> Vec<Field> {
        vec![
            Field::builder()
                .title("ATIM Window")
                .value(self.atim_window_tu)
                .units("TU")
                .build(),
        ]
    }
}
