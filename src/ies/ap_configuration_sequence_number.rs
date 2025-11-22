use deku::prelude::*;

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
pub struct ApConfigurationSequenceNumber {
    #[deku(bytes = 1)]
    pub ap_csn: u8,
}

impl ApConfigurationSequenceNumber {
    pub const ID: u8 = 239;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const NAME: &'static str = "AP Configuration Sequence Number";
}
