use deku::{DekuRead, DekuWrite};

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite)]
pub struct ParameterRecord {
    pub aci_aifsn: AciAifsn,
    pub ecw_min_ecw_max: EcwMinEcwMax,
    #[deku(bytes = 1)]
    pub mu_edca_timer: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite)]
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
}
