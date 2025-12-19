use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct PowerConstraint {
    #[deku(bytes = 1)]
    pub local_power_constraint_db: u8,
}

impl PowerConstraint {
    pub const NAME: &'static str = "Power Constraint";
    pub const ID: u8 = 32;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const LENGTH: usize = 1;
}
