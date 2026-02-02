use deku::{DekuContainerWrite, DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::IeId;
use crate::{BitRange, Field};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct MuEdcaParameterSet {
    pub qos_info: QosInfo,
    pub mu_ac_be_parameter_record: ParameterRecord,
    pub mu_ac_bk_parameter_record: ParameterRecord,
    pub mu_ac_vi_parameter_record: ParameterRecord,
    pub mu_ac_vo_paremter_record: ParameterRecord,
}

impl MuEdcaParameterSet {
    pub const NAME: &'static str = "MU EDCA Parameter Set";
    pub const ID: u8 = 255;
    pub const ID_EXT: Option<u8> = Some(38);
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);

    pub fn summary(&self) -> String {
        format!(
            "EDCA Update Count: {}",
            self.qos_info.edca_parameter_set_update_count
        )
    }

    pub fn fields(&self) -> Vec<Field> {
        vec![
            self.qos_info.to_field(),
            self.mu_ac_be_parameter_record.to_field("MU AC_BE"),
            self.mu_ac_bk_parameter_record.to_field("MU AC_BK"),
            self.mu_ac_vi_parameter_record.to_field("MU AC_VI"),
            self.mu_ac_vo_paremter_record.to_field("MU AC_VO"),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct QosInfo {
    #[deku(bits = 4)]
    pub edca_parameter_set_update_count: u8,
    #[deku(bits = 1)]
    pub q_ack: bool,
    #[deku(bits = 1)]
    pub queue_request: bool,
    #[deku(bits = 1)]
    pub txop_request: bool,
    #[deku(bits = 1)]
    pub more_data_ack: bool,
}

impl QosInfo {
    pub fn to_field(&self) -> Field {
        let byte = self
            .to_bytes()
            .unwrap_or_default()
            .first()
            .cloned()
            .unwrap_or_default();

        Field::builder()
            .title("QoS Info")
            .value(format!(
                "EDCA Update Count: {}",
                self.edca_parameter_set_update_count
            ))
            .byte(byte)
            .subfields([
                Field::builder()
                    .title("EDCA Parameter Set Update Count")
                    .value(self.edca_parameter_set_update_count)
                    .bits(BitRange::from_byte(byte, 0, 4))
                    .build(),
                Field::builder()
                    .title("Q-Ack")
                    .value(self.q_ack)
                    .bits(BitRange::from_byte(byte, 4, 1))
                    .build(),
                Field::builder()
                    .title("Queue Request")
                    .value(self.queue_request)
                    .bits(BitRange::from_byte(byte, 5, 1))
                    .build(),
                Field::builder()
                    .title("TXOP Request")
                    .value(self.txop_request)
                    .bits(BitRange::from_byte(byte, 6, 1))
                    .build(),
                Field::builder()
                    .title("More Data Ack")
                    .value(self.more_data_ack)
                    .bits(BitRange::from_byte(byte, 7, 1))
                    .build(),
            ])
            .build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct ParameterRecord {
    pub aci_aifsn: AciAifsn,
    pub ecw_min_ecw_max: EcwMinEcwMax,
    #[deku(bytes = 1)]
    pub mu_edca_timer: u8,
}

impl ParameterRecord {
    pub fn to_field(&self, prefix: &str) -> Field {
        Field::builder()
            .title(format!("{} Parameter Record", prefix))
            .value("")
            .subfields([
                self.aci_aifsn.to_field(),
                self.ecw_min_ecw_max.to_field(),
                Field::builder()
                    .title("MU EDCA Timer")
                    .value(self.mu_edca_timer)
                    .byte(self.mu_edca_timer)
                    .build(),
            ])
            .build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct AciAifsn {
    #[deku(bits = 4)]
    pub aifsn: u8,
    #[deku(bits = 1)]
    pub acm: bool,
    #[deku(bits = 2)]
    pub aci: u8,
    #[deku(bits = 1)]
    reserved: bool,
}

impl AciAifsn {
    pub fn to_field(&self) -> Field {
        let byte = self
            .to_bytes()
            .unwrap_or_default()
            .first()
            .cloned()
            .unwrap_or_default();

        Field::builder()
            .title("ACI/AIFSN")
            .value("")
            .byte(byte)
            .subfields([
                Field::builder()
                    .title("AIFSN")
                    .value(self.aifsn)
                    .bits(BitRange::from_byte(byte, 0, 4))
                    .build(),
                Field::builder()
                    .title("ACM")
                    .value(self.acm)
                    .bits(BitRange::from_byte(byte, 4, 1))
                    .build(),
                Field::builder()
                    .title("ACI")
                    .value(self.acm)
                    .bits(BitRange::from_byte(byte, 5, 2))
                    .build(),
                Field::reserved(BitRange::from_byte(byte, 7, 1)),
            ])
            .build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct EcwMinEcwMax {
    #[deku(bits = 4)]
    pub ecw_min: u8,
    #[deku(bits = 4)]
    pub ecw_max: u8,
}

impl EcwMinEcwMax {
    pub fn cw_min(&self) -> u16 {
        2u16.pow(u32::from(self.ecw_min)) - 1
    }

    pub fn cw_max(&self) -> u16 {
        2u16.pow(u32::from(self.ecw_max)) - 1
    }

    pub fn to_field(&self) -> Field {
        let byte = self
            .to_bytes()
            .unwrap_or_default()
            .first()
            .cloned()
            .unwrap_or_default();

        Field::builder()
            .title("ECWmin/ECWmax")
            .value("")
            .byte(byte)
            .subfields([
                Field::builder()
                    .title("CWmin")
                    .value(self.cw_min())
                    .units(format!("({})", self.ecw_min))
                    .bits(BitRange::from_byte(byte, 0, 4))
                    .build(),
                Field::builder()
                    .title("CWmax")
                    .value(self.cw_max())
                    .units(format!("({})", self.ecw_max))
                    .bits(BitRange::from_byte(byte, 4, 4))
                    .build(),
            ])
            .build()
    }
}
