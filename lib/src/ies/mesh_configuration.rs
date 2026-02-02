use std::fmt::Display;

use deku::{DekuContainerWrite, DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::IeId;
use crate::{BitRange, Field};

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

    pub fn summary(&self) -> String {
        format!(
            "Path Selection Protocol: {}, Path Selection Metrix: {}",
            self.active_path_selection_protocol_identifier,
            self.active_path_selection_metric_identifier
        )
    }

    pub fn fields(&self) -> Vec<Field> {
        let bytes = self.to_bytes().unwrap_or_default();
        vec![
            Field::builder()
                .title("Active Path Selection Protocol ID")
                .value(self.active_path_selection_protocol_identifier)
                .byte(*bytes.get(0).unwrap_or(&0))
                .build(),
            Field::builder()
                .title("Active Path Selection Metric ID")
                .value(self.active_path_selection_metric_identifier)
                .byte(*bytes.get(1).unwrap_or(&0))
                .build(),
            Field::builder()
                .title("Congestion Control Mode ID")
                .value(self.congestion_control_mode_identifier)
                .byte(*bytes.get(2).unwrap_or(&0))
                .build(),
            Field::builder()
                .title("Synchronization Method ID")
                .value(self.synchronization_method_identifier)
                .byte(*bytes.get(3).unwrap_or(&0))
                .build(),
            Field::builder()
                .title("Authentication Protocol ID")
                .value(self.authentication_protocol_identifier)
                .byte(*bytes.get(4).unwrap_or(&0))
                .build(),
            self.mesh_formation_info.to_field(),
            self.mesh_capability.to_field(),
        ]
    }
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

impl MeshCapability {
    pub fn to_field(&self) -> Field {
        let byte = self
            .to_bytes()
            .unwrap_or_default()
            .first()
            .cloned()
            .unwrap_or_default();
        Field::builder()
            .title("Mesh Capability")
            .value("")
            .subfields([
                Field::builder()
                    .title("Accepting Additional Mesh Peerings")
                    .value(self.accepting_additional_mesh_peerings)
                    .bits(BitRange::from_byte(byte, 0, 1))
                    .build(),
                Field::builder()
                    .title("MCCA Supported")
                    .value(self.mcca_supported)
                    .bits(BitRange::from_byte(byte, 1, 1))
                    .build(),
                Field::builder()
                    .title("MCCA Enabled")
                    .value(self.mcca_enabled)
                    .bits(BitRange::from_byte(byte, 2, 1))
                    .build(),
                Field::builder()
                    .title("Forwarding")
                    .value(self.forwarding)
                    .bits(BitRange::from_byte(byte, 3, 1))
                    .build(),
                Field::builder()
                    .title("MBCA Enabled")
                    .value(self.mbca_enabled)
                    .bits(BitRange::from_byte(byte, 4, 1))
                    .build(),
                Field::builder()
                    .title("TBTT Adjusting")
                    .value(self.tbtt_adjusting)
                    .bits(BitRange::from_byte(byte, 5, 1))
                    .build(),
                Field::builder()
                    .title("Mesh Power Save Level")
                    .value(self.mesh_power_save_level)
                    .bits(BitRange::from_byte(byte, 6, 1))
                    .build(),
                Field::reserved(BitRange::from_byte(byte, 7, 1)),
            ])
            .byte(byte)
            .build()
    }
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

impl MeshFormationInfo {
    pub fn to_field(&self) -> Field {
        let byte = self
            .to_bytes()
            .unwrap_or_default()
            .first()
            .cloned()
            .unwrap_or_default();

        Field::builder()
            .title("Mesh Formation Info")
            .value("")
            .subfields([
                Field::builder()
                    .title("Connected to Mesh Gate")
                    .value(self.connected_to_mesh_gate)
                    .bits(BitRange::from_byte(byte, 0, 1))
                    .build(),
                Field::builder()
                    .title("Number of Peerings")
                    .value(self.number_of_peerings)
                    .bits(BitRange::from_byte(byte, 1, 6))
                    .build(),
                Field::builder()
                    .title("Connected to AS")
                    .value(self.connected_to_as)
                    .bits(BitRange::from_byte(byte, 7, 1))
                    .build(),
            ])
            .byte(byte)
            .build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(id_type = "u8")]
#[repr(u8)]
pub enum ActivePathSelectionProtocolIdentifier {
    Hybrid = 1,
    #[deku(id_pat = "_")]
    Reserved(u8),
    VendorSpecific = 255,
}

impl Display for ActivePathSelectionProtocolIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hybrid => write!(f, "Hybrid"),
            Self::Reserved(val) => write!(f, "Reserved ({})", val),
            Self::VendorSpecific => write!(f, "Vendor Specific"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(id_type = "u8")]
#[repr(u8)]
pub enum ActivePathSelectionMetricIdentifier {
    AirtimeLink = 1,
    HighPhyRate = 2,
    #[deku(id_pat = "_")]
    Reserved(u8),
    VendorSpecific = 255,
}

impl Display for ActivePathSelectionMetricIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AirtimeLink => write!(f, "Airtime Link Metric"),
            Self::HighPhyRate => write!(f, "High PHY Rate Airtime Link Metric"),
            Self::Reserved(val) => write!(f, "Reserved ({})", val),
            Self::VendorSpecific => write!(f, "Vendor Specific"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(id_type = "u8")]
#[repr(u8)]
pub enum CongestionControlModeIdentifier {
    NotActivated = 0,
    CongestionControlSignaling = 1,
    #[deku(id_pat = "_")]
    Reserved(u8),
    VendorSpecific = 255,
}

impl Display for CongestionControlModeIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotActivated => write!(f, "Not Activated"),
            Self::CongestionControlSignaling => write!(f, "Congestion Control Signaling"),
            Self::Reserved(val) => write!(f, "Reserved ({})", val),
            Self::VendorSpecific => write!(f, "Vendor Specific"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(id_type = "u8")]
#[repr(u8)]
pub enum SynchronizationMethodIdentifier {
    NeighborOffset = 1,
    #[deku(id_pat = "_")]
    Reserved(u8),
    VendorSpecific = 255,
}

impl Display for SynchronizationMethodIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NeighborOffset => write!(f, "Neighbor Offset"),
            Self::Reserved(val) => write!(f, "Reserved ({})", val),
            Self::VendorSpecific => write!(f, "Vendor Specific"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(id_type = "u8")]
#[repr(u8)]
pub enum AuthenticationProtocolIdentifier {
    NotRequired = 0,
    Sae = 1,
    Ieee8021X = 2,
    #[deku(id_pat = "_")]
    Reserved(u8),
    VendorSpecific = 255,
}

impl Display for AuthenticationProtocolIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotRequired => write!(f, "Not Required"),
            Self::Sae => write!(f, "SAE"),
            Self::Ieee8021X => write!(f, "IEEE 802.1X"),
            Self::Reserved(val) => write!(f, "Reserved ({})", val),
            Self::VendorSpecific => write!(f, "Vendor Specific"),
        }
    }
}
