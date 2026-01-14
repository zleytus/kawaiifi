use deku::{DekuRead, DekuWrite};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct MeshConfiguration {
    pub active_path_selection_protocol_identifier: ActivePathSelectionProtocolIdentifier,
    pub active_path_selection_metric_identifier: ActivePathSelectionMetricIdentifier,
    pub congestion_control_mode_identifier: CongestionControlModeIdentifier,
    pub synchronization_method_identifier: SynchronizationMethodIdentifier,
    pub authentication_protocol_identifier: AuthenticationProtocolIdentifier,
    pub mesh_formation_info: MeshFormationInfo,
    pub mesh_capability: MeshCapability,
}

impl MeshConfiguration {
    pub const NAME: &'static str = "Mesh Configuration";
    pub const ID: u8 = 113;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const MIN_LENGTH: usize = 7;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct MeshCapability {
    #[deku(bits = 1)]
    pub accepting_additional_mesh_peerings: bool,
    #[deku(bits = 1)]
    pub mcca_supported: bool,
    #[deku(bits = 1)]
    pub mcca_enabled: bool,
    #[deku(bits = 1)]
    pub forwarding: bool,
    #[deku(bits = 1)]
    pub mbca_enabled: bool,
    #[deku(bits = 1)]
    pub tbtt_adjusting: bool,
    #[deku(bits = 1)]
    pub mesh_power_save_level: bool,
    #[deku(bits = 1)]
    reserved: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct MeshFormationInfo {
    #[deku(bits = 1)]
    pub connected_to_mesh_gate: bool,
    #[deku(bits = 6)]
    pub number_of_peerings: u8,
    #[deku(bits = 1)]
    pub connected_to_as: bool,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    TryFromPrimitive,
    IntoPrimitive,
    DekuRead,
    DekuWrite,
    Serialize,
    Deserialize,
)]
#[deku(id_type = "u8")]
#[repr(u8)]
pub enum ActivePathSelectionProtocolIdentifier {
    Hybrid = 1,
    VendorSpecific = 255,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    TryFromPrimitive,
    IntoPrimitive,
    DekuRead,
    DekuWrite,
    Serialize,
    Deserialize,
)]
#[deku(id_type = "u8")]
#[repr(u8)]
pub enum ActivePathSelectionMetricIdentifier {
    AirtimeLink = 1,
    VendorSpecific = 255,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    TryFromPrimitive,
    IntoPrimitive,
    DekuRead,
    DekuWrite,
    Serialize,
    Deserialize,
)]
#[deku(id_type = "u8")]
#[repr(u8)]
pub enum CongestionControlModeIdentifier {
    NotActivated = 0,
    CongestionControlSignaling = 1,
    VendorSpecific = 255,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    TryFromPrimitive,
    IntoPrimitive,
    DekuRead,
    DekuWrite,
    Serialize,
    Deserialize,
)]
#[deku(id_type = "u8")]
#[repr(u8)]
pub enum SynchronizationMethodIdentifier {
    NeighborOffset = 1,
    VendorSpecific = 255,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    TryFromPrimitive,
    IntoPrimitive,
    DekuRead,
    DekuWrite,
    Serialize,
    Deserialize,
)]
#[deku(id_type = "u8")]
#[repr(u8)]
pub enum AuthenticationProtocolIdentifier {
    NotRequired = 0,
    Sae = 1,
    Ieee8021X = 2,
    VendorSpecific = 255,
}
