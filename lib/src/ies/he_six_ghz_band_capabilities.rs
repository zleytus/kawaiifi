use deku::{DekuContainerWrite, DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::{IeId, ht_capabilities::SmPowerSave, write_bits_lsb0};
use crate::{BitRange, Field};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct HeSixGhzBandCapabilities {
    pub capabilities_information: CapabilitiesInformation,
}

impl HeSixGhzBandCapabilities {
    pub const LENGTH: usize = 2;
    pub const NAME: &'static str = "HE 6 GHz Band Capabilities";
    pub const ID: u8 = 255;
    pub const ID_EXT: Option<u8> = Some(59);
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);

    pub fn summary(&self) -> String {
        "".to_string()
    }

    pub fn fields(&self) -> Vec<Field> {
        vec![self.capabilities_information.to_field()]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct CapabilitiesInformation {
    #[deku(bits = 3)]
    pub minimum_mpdu_start_spacing: u8,
    #[deku(bits = 3)]
    pub maximum_ampdu_length_exponent: u8,
    #[deku(bits = 2)]
    pub maximum_mpdu_length: u8,
    #[deku(bits = 1)]
    reserved_1: bool,
    #[deku(
        bits = 2,
        map = "|value: u8| SmPowerSave::try_from(value).map_err(|_| deku::DekuError::Parse(\"Invalid SmPowerSave\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*sm_power_save), 2)"
    )]
    pub sm_power_save: SmPowerSave,
    #[deku(bits = 1)]
    pub rd_responder: bool,
    #[deku(bits = 1)]
    pub rx_antenna_pattern_consistency: bool,
    #[deku(bits = 1)]
    pub tx_antenna_pattern_consistency: bool,
    #[deku(bits = 2)]
    reserved_2: u8,
}

impl CapabilitiesInformation {
    pub fn to_field(&self) -> Field {
        let bytes = self.to_bytes().unwrap_or_default();

        Field::builder()
            .title("Capabilities Information")
            .value("")
            .subfields([
                Field::builder()
                    .title("Minimum MPDU Start Spacing")
                    .value(self.minimum_mpdu_start_spacing)
                    .bits(BitRange::new(&bytes, 0, 3))
                    .build(),
                Field::builder()
                    .title("Maximum AMPDU Length Exponent")
                    .value(self.maximum_ampdu_length_exponent)
                    .bits(BitRange::new(&bytes, 3, 3))
                    .build(),
                Field::builder()
                    .title("Maximum MPDU Length")
                    .value(self.maximum_mpdu_length)
                    .bits(BitRange::new(&bytes, 6, 2))
                    .build(),
                Field::reserved(BitRange::new(&bytes, 8, 1)),
                Field::builder()
                    .title("SM Power Save")
                    .value(self.sm_power_save)
                    .bits(BitRange::new(&bytes, 9, 2))
                    .build(),
                Field::builder()
                    .title("RD Responder")
                    .value(self.rd_responder)
                    .bits(BitRange::new(&bytes, 11, 1))
                    .build(),
                Field::builder()
                    .title("Rx Antenna Pattern Consistency")
                    .value(self.rx_antenna_pattern_consistency)
                    .bits(BitRange::new(&bytes, 12, 1))
                    .build(),
                Field::builder()
                    .title("Tx Antenna Pattern Consistency")
                    .value(self.tx_antenna_pattern_consistency)
                    .bits(BitRange::new(&bytes, 13, 1))
                    .build(),
                Field::reserved(BitRange::new(&bytes, 14, 2)),
            ])
            .bytes(bytes)
            .build()
    }
}
