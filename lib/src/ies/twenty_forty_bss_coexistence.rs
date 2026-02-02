use deku::{DekuContainerWrite, DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::IeId;
use crate::{BitRange, Field};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct TwentyFortyBssCoexistence {
    #[deku(bits = 1)]
    pub information_request: bool,
    #[deku(bits = 1)]
    pub forty_mhz_intolerant: bool,
    #[deku(bits = 1)]
    pub twenty_mhz_bss_width_request: bool,
    #[deku(bits = 1)]
    pub obss_scanning_exemption_request: bool,
    #[deku(bits = 1)]
    pub obss_scanning_exemption_grant: bool,
    #[deku(bits = 3)]
    reserved: u8,
}

impl TwentyFortyBssCoexistence {
    pub const NAME: &'static str = "20/40 BSS Coexistence";
    pub const ID: u8 = 72;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);

    pub fn summary(&self) -> String {
        "".to_string()
    }

    pub fn fields(&self) -> Vec<Field> {
        let byte = self
            .to_bytes()
            .unwrap_or_default()
            .first()
            .cloned()
            .unwrap_or_default();
        vec![
            Field::builder()
                .title("Information Request")
                .value(self.information_request)
                .bits(BitRange::from_byte(byte, 0, 1))
                .build(),
            Field::builder()
                .title("40 MHz Intolerant")
                .value(self.forty_mhz_intolerant)
                .bits(BitRange::from_byte(byte, 1, 1))
                .build(),
            Field::builder()
                .title("20 MHz BSS Width Requesst")
                .value(self.twenty_mhz_bss_width_request)
                .bits(BitRange::from_byte(byte, 2, 1))
                .build(),
            Field::builder()
                .title("OBSS Scanning Exemption Request")
                .value(self.obss_scanning_exemption_request)
                .bits(BitRange::from_byte(byte, 3, 1))
                .build(),
            Field::builder()
                .title("OBSS Scanning Exemption Grant")
                .value(self.obss_scanning_exemption_grant)
                .bits(BitRange::from_byte(byte, 4, 1))
                .build(),
            Field::reserved(BitRange::from_byte(byte, 5, 3)),
        ]
    }
}
