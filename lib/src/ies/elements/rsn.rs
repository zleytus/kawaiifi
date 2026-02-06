use std::fmt::Display;

use deku::{DekuContainerWrite, DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use crate::ies::{BitRange, Field, IeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "len: usize")]
pub struct Rsn {
    #[deku(bytes = 2)]
    pub version: u16,
    #[deku(cond = "len >= 6")]
    pub group_data_cipher_suite: Option<CipherSuite>,
    #[deku(bytes = 2, cond = "len >= 8")]
    pub pairwise_cipher_suite_count: Option<u16>,
    #[deku(
        count = "pairwise_cipher_suite_count.unwrap_or_default()",
        cond = "len >= 8 + usize::from(4 * pairwise_cipher_suite_count.unwrap_or_default())"
    )]
    pub pairwise_cipher_suite_list: Option<Vec<CipherSuite>>,
    #[deku(
        bytes = 2,
        cond = "len >= 8 + usize::from(4 * pairwise_cipher_suite_count.unwrap_or_default()) + 2"
    )]
    pub akm_suite_count: Option<u16>,
    #[deku(
        count = "akm_suite_count.unwrap_or_default()",
        cond = "len >= 8 + usize::from(4 * pairwise_cipher_suite_count.unwrap_or_default()) + 2 + usize::from(4 * akm_suite_count.unwrap_or_default())"
    )]
    pub akm_suite_list: Option<Vec<AkmSuite>>,
    #[deku(
        cond = "len >= 8 + usize::from(4 * pairwise_cipher_suite_count.unwrap_or_default()) + 2 + usize::from(4 * akm_suite_count.unwrap_or_default()) + 2"
    )]
    pub rsn_capabilities: Option<RsnCapabilities>,
    #[deku(
        bytes = 2,
        cond = "len >= 8 + usize::from(4 * pairwise_cipher_suite_count.unwrap_or_default()) + 2 + usize::from(4 * akm_suite_count.unwrap_or_default()) + 2 + 2"
    )]
    pub pmkid_count: Option<u16>,
    #[deku(
        count = "pmkid_count.unwrap_or_default()",
        cond = "len >= 8 + usize::from(4 * pairwise_cipher_suite_count.unwrap_or_default()) + 2 + usize::from(4 * akm_suite_count.unwrap_or_default()) + 2 + 2 + usize::from(16 * pmkid_count.unwrap_or_default())"
    )]
    pub pmkid_list: Option<Vec<u128>>,
    #[deku(
        cond = "len >= 8 + usize::from(4 * pairwise_cipher_suite_count.unwrap_or_default()) + 2 + usize::from(4 * akm_suite_count.unwrap_or_default()) + 2 + 2 + usize::from(16 * pmkid_count.unwrap_or_default()) + 4"
    )]
    pub group_management_cipher_suite: Option<CipherSuite>,
}

impl Rsn {
    pub const NAME: &'static str = "RSNE";
    pub const ID: u8 = 48;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);

    pub fn summary(&self) -> String {
        let mut summary = Vec::new();

        if let Some(group_data_cipher_suite) = &self.group_data_cipher_suite {
            summary.push(format!(
                "Group Cipher: {}",
                group_data_cipher_suite.suite_type
            ));
        }

        if let Some(pairwise_cipher_suites) = &self.pairwise_cipher_suite_list {
            let pairwise_suites_string = pairwise_cipher_suites
                .iter()
                .map(|suite| suite.suite_type.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            if pairwise_cipher_suites.len() > 1 {
                summary.push(format!("Pairwise Ciphers: {}", pairwise_suites_string));
            } else {
                summary.push(format!("Pairwise Cipher: {}", pairwise_suites_string));
            }
        }

        if let Some(akm_suites) = &self.akm_suite_list {
            let akm_suites_string = akm_suites
                .iter()
                .map(|suite| suite.suite_type.authentication_type().to_string())
                .collect::<Vec<String>>()
                .join(", ");
            if akm_suites.len() > 1 {
                summary.push(format!("AKM Suites: {}", akm_suites_string));
            } else {
                summary.push(format!("AKM Suite: {}", akm_suites_string));
            }
        }

        summary.join(", ")
    }

    pub fn fields(&self) -> Vec<Field> {
        let mut fields = vec![
            Field::builder()
                .title("Version")
                .value(self.version)
                .bytes(self.version.to_le_bytes().to_vec())
                .build(),
        ];

        if let Some(group_data_cipher_suite) = self.group_data_cipher_suite {
            fields.extend_from_slice(&group_data_cipher_suite.to_fields("Group"));
        }

        if let Some(pairwise_cipher_suite_count) = self.pairwise_cipher_suite_count {
            fields.push(
                Field::builder()
                    .title("Pairwise Cipher Suite Count")
                    .value(pairwise_cipher_suite_count)
                    .bytes(pairwise_cipher_suite_count.to_le_bytes().to_vec())
                    .build(),
            );
        }

        if let Some(pairwise_suite_list) = &self.pairwise_cipher_suite_list {
            fields.push(
                Field::builder()
                    .title("Pairwise Cipher Suite List")
                    .value("")
                    .bytes(
                        pairwise_suite_list
                            .iter()
                            .flat_map(|suite| suite.to_bytes().unwrap_or_default())
                            .collect(),
                    )
                    .subfields(
                        pairwise_suite_list
                            .iter()
                            .flat_map(|suite| suite.to_fields("Pairwise")),
                    )
                    .build(),
            );
        }

        if let Some(akm_suite_count) = self.akm_suite_count {
            fields.push(
                Field::builder()
                    .title("AKM Suite Count")
                    .value(akm_suite_count)
                    .bytes(akm_suite_count.to_le_bytes().to_vec())
                    .build(),
            );
        }

        if let Some(akm_suite_list) = &self.akm_suite_list {
            fields.push(
                Field::builder()
                    .title("AKM Suite List")
                    .value("")
                    .bytes(
                        akm_suite_list
                            .iter()
                            .flat_map(|suite| suite.to_bytes().unwrap_or_default())
                            .collect(),
                    )
                    .subfields(akm_suite_list.iter().flat_map(|suite| suite.to_fields()))
                    .build(),
            );
        }

        if let Some(rsn_capabilities) = &self.rsn_capabilities {
            fields.push(rsn_capabilities.to_field());
        }

        if let Some(pmkid_count) = self.pmkid_count {
            fields.push(
                Field::builder()
                    .title("PMKID Count")
                    .value(pmkid_count)
                    .bytes(pmkid_count.to_le_bytes().to_vec())
                    .build(),
            );
        }

        if let Some(pmkid_list) = &self.pmkid_list {
            fields.push(
                Field::builder()
                    .title("PMKID List")
                    .value("")
                    .bytes(
                        pmkid_list
                            .iter()
                            .flat_map(|pmkid| pmkid.to_le_bytes())
                            .collect(),
                    )
                    .subfields(pmkid_list.iter().map(|pmkid| {
                        Field::builder()
                            .title("PMKID")
                            .value(*pmkid)
                            .bytes(pmkid.to_le_bytes().to_vec())
                            .build()
                    }))
                    .build(),
            );
        }

        if let Some(group_management_cipher_suite) = self.group_management_cipher_suite {
            let group_management_cipher_suite_fields =
                group_management_cipher_suite.to_fields("Group Management");
            for field in group_management_cipher_suite_fields {
                fields.push(field);
            }
        }

        fields
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct CipherSuite {
    pub oui: [u8; 3],
    pub suite_type: CipherSuiteType,
}

impl CipherSuite {
    pub fn to_fields(&self, prefix: &str) -> [Field; 2] {
        let cipher_suite_byte = self
            .suite_type
            .to_bytes()
            .unwrap_or_default()
            .first()
            .cloned()
            .unwrap_or_default();
        [
            Field::builder()
                .title(format!("{} Cipher Suite OUI", prefix))
                .value(format!("{:02X?}", self.oui))
                .bytes(self.oui.to_vec())
                .build(),
            Field::builder()
                .title(format!("{} Cipher Suite Type", prefix))
                .value(self.suite_type)
                .byte(cipher_suite_byte)
                .units(format!("({})", cipher_suite_byte))
                .build(),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(id_type = "u8")]
#[repr(u8)]
pub enum CipherSuiteType {
    UseGroupCipherSuite = 0,
    Wep40,
    Tkip,
    Reserved,
    Ccmp128,
    Wep104,
    BipCmac128,
    GroupAddressedTrafficNotAllowed,
    Gcmp128,
    Gcmp256,
    Ccmp256,
    BipGmac128,
    BipGmac256,
    BipCmac256,
    #[deku(id_pat = "_")]
    Unknown(u8),
}

impl Display for CipherSuiteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UseGroupCipherSuite => write!(f, "Use group cipher guite"),
            Self::Wep40 => write!(f, "WEP-40"),
            Self::Tkip => write!(f, "TKIP"),
            Self::Reserved => write!(f, "Reserved"),
            Self::Ccmp128 => write!(f, "CCMP-128"),
            Self::Wep104 => write!(f, "WEP-104"),
            Self::BipCmac128 => write!(f, "BIP-CMAC-128"),
            Self::GroupAddressedTrafficNotAllowed => {
                write!(f, "Group addressed traffic not allowed")
            }
            Self::Gcmp128 => write!(f, "GCMP-128"),
            Self::Gcmp256 => write!(f, "GCMP-256"),
            Self::Ccmp256 => write!(f, "CCMP-256"),
            Self::BipGmac128 => write!(f, "BIP-GMAC-128"),
            Self::BipGmac256 => write!(f, "BIP-GMAC-256"),
            Self::BipCmac256 => write!(f, "BIP-CMAC-256"),
            Self::Unknown(suite_type) => write!(f, "Unknown ({})", suite_type),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct AkmSuite {
    pub oui: [u8; 3],
    pub suite_type: AkmSuiteType,
}

impl AkmSuite {
    pub fn to_fields(&self) -> [Field; 2] {
        [
            Field::builder()
                .title("AKM Suite OUI")
                .value(format!("{:02X?}", self.oui))
                .bytes(self.oui.to_vec())
                .build(),
            Field::builder()
                .title("AKM Suite Type")
                .value(self.suite_type.authentication_type())
                .units(format!("({})", self.suite_type.0))
                .byte(
                    self.suite_type
                        .to_bytes()
                        .unwrap_or_default()
                        .first()
                        .cloned()
                        .unwrap_or_default(),
                )
                .build(),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct AkmSuiteType(pub u8);

impl AkmSuiteType {
    pub fn authentication_type(&self) -> AuthenticationType {
        match self.0 {
            0 => AuthenticationType::Reserved,
            1 => AuthenticationType::AuthOver8021X,
            2 => AuthenticationType::Psk,
            3 => AuthenticationType::FtAuthOver8021X,
            4 => AuthenticationType::FtAuthUsingPsk,
            5 => AuthenticationType::AuthOver8021X,
            6 => AuthenticationType::Psk,
            7 => AuthenticationType::Tdls,
            8 => AuthenticationType::Sae,
            9 => AuthenticationType::FtAuthOverSae,
            10 => AuthenticationType::ApPeerKeyAuthWithSha256,
            11 => AuthenticationType::AuthOver8021XUsingSuiteBCompliantEapMethod,
            12 => AuthenticationType::AuthOver8021XUsingCnsaSuiteCompliantEapMethod,
            13 => AuthenticationType::FtAuthOver8021X,
            14 => AuthenticationType::KeyManagementOverFilsUsingSha256AesSiv256OrAuthOver8021X,
            15 => AuthenticationType::KeyManagementOverFilsUsingSha384AesSiv512OrAuthOver8021X,
            16 => AuthenticationType::FtAuthOverFilsWithSha256AesSiv256OrAuthOver8021X,
            17 => AuthenticationType::FtAuthOverFilsWithSha384AesSiv512OrAuthOver8021X,
            18 => AuthenticationType::Reserved,
            19 => AuthenticationType::FtAuthUsingPsk,
            20 => AuthenticationType::Psk,
            _ => AuthenticationType::Reserved,
        }
    }

    pub fn key_management_type(&self) -> KeyManagementType {
        match self.0 {
            0 => KeyManagementType::Reserved,
            1 => KeyManagementType::Rsna,
            2 => KeyManagementType::Rsna,
            3 => KeyManagementType::Ft,
            4 => KeyManagementType::Ft,
            5 => KeyManagementType::Rsna,
            6 => KeyManagementType::Rsna,
            7 => KeyManagementType::TpkHandshake,
            8 => KeyManagementType::RsnaOrAuthenticatedMeshPeeringExchange,
            9 => KeyManagementType::Ft,
            10 => KeyManagementType::Rsna,
            11 => KeyManagementType::Rsna,
            12 => KeyManagementType::Rsna,
            13 => KeyManagementType::Ft,
            14 => KeyManagementType::Fils,
            15 => KeyManagementType::Fils,
            16 => KeyManagementType::Ft,
            17 => KeyManagementType::Ft,
            18 => KeyManagementType::Reserved,
            19 => KeyManagementType::Ft,
            20 => KeyManagementType::Rsna,
            _ => KeyManagementType::Reserved,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthenticationType {
    Reserved,
    AuthOver8021X,
    Psk,
    FtAuthOver8021X,
    FtAuthUsingPsk,
    Tdls,
    Sae,
    FtAuthOverSae,
    ApPeerKeyAuthWithSha256,
    AuthOver8021XUsingSuiteBCompliantEapMethod,
    AuthOver8021XUsingCnsaSuiteCompliantEapMethod,
    KeyManagementOverFilsUsingSha256AesSiv256OrAuthOver8021X,
    KeyManagementOverFilsUsingSha384AesSiv512OrAuthOver8021X,
    FtAuthOverFilsWithSha256AesSiv256OrAuthOver8021X,
    FtAuthOverFilsWithSha384AesSiv512OrAuthOver8021X,
}

impl Display for AuthenticationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reserved => write!(f, "Reserved"),
            Self::AuthOver8021X => write!(f, "802.1X"),
            Self::Psk => write!(f, "PSK"),
            Self::FtAuthOver8021X => write!(f, "FT Over 802.1X"),
            Self::FtAuthUsingPsk => write!(f, "FT Using PSK"),
            Self::Tdls => write!(f, "TDLS"),
            Self::Sae => write!(f, "SAE"),
            Self::FtAuthOverSae => write!(f, "FT Over SAE"),
            Self::ApPeerKeyAuthWithSha256 => write!(f, "APPeerKey"),
            Self::AuthOver8021XUsingSuiteBCompliantEapMethod => write!(f, "802.1X"),
            Self::AuthOver8021XUsingCnsaSuiteCompliantEapMethod => write!(f, "802.1X"),
            Self::KeyManagementOverFilsUsingSha256AesSiv256OrAuthOver8021X => write!(f, "802.1X"),
            Self::KeyManagementOverFilsUsingSha384AesSiv512OrAuthOver8021X => write!(f, "802.1X"),
            Self::FtAuthOverFilsWithSha256AesSiv256OrAuthOver8021X => {
                write!(f, "FT Auth Over FILS Or 802.1X")
            }
            Self::FtAuthOverFilsWithSha384AesSiv512OrAuthOver8021X => {
                write!(f, "FT Auth Over FILS Or 802.1X")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyManagementType {
    Rsna,
    Ft,
    TpkHandshake,
    RsnaOrAuthenticatedMeshPeeringExchange,
    Fils,
    Reserved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct RsnCapabilities {
    #[deku(bits = 1)]
    pub preauthentication: bool,
    #[deku(bits = 1)]
    pub no_pairwise: bool,
    #[deku(bits = 2)]
    pub ptksa_replay_counter: u8,
    #[deku(bits = 2)]
    pub gtksa_replay_counter: u8,
    #[deku(bits = 1)]
    pub mfpr: bool,
    #[deku(bits = 1)]
    pub mfpc: bool,
    #[deku(bits = 1)]
    pub joint_multiband_rsna: bool,
    #[deku(bits = 1)]
    pub peerkey_enabled: bool,
    #[deku(bits = 1)]
    pub spp_amsdu_capable: bool,
    #[deku(bits = 1)]
    pub spp_amsdu_required: bool,
    #[deku(bits = 1)]
    pub pbac: bool,
    #[deku(bits = 1)]
    pub extended_key_id_for_individually_address_frames: bool,
    #[deku(bits = 1)]
    pub ocvc: bool,
    #[deku(bits = 1)]
    reserved: bool,
}

impl RsnCapabilities {
    pub fn to_field(&self) -> Field {
        let bytes = self.to_bytes().unwrap_or_default();

        Field::builder()
            .title("RSN Capabilities")
            .value("")
            .bytes(bytes.clone())
            .subfields([
                Field::builder()
                    .title("Preauthentication")
                    .value(self.preauthentication)
                    .bits(BitRange::new(&bytes, 0, 1))
                    .build(),
                Field::builder()
                    .title("No Pairwise")
                    .value(self.no_pairwise)
                    .bits(BitRange::new(&bytes, 1, 1))
                    .build(),
                Field::builder()
                    .title("PTKSA Replay Counter")
                    .value(self.ptksa_replay_counter)
                    .bits(BitRange::new(&bytes, 2, 2))
                    .build(),
                Field::builder()
                    .title("GTKSA Replay Counter")
                    .value(self.gtksa_replay_counter)
                    .bits(BitRange::new(&bytes, 4, 2))
                    .build(),
                Field::builder()
                    .title("MFPR")
                    .value(self.mfpr)
                    .bits(BitRange::new(&bytes, 6, 1))
                    .build(),
                Field::builder()
                    .title("MFPC")
                    .value(self.mfpc)
                    .bits(BitRange::new(&bytes, 7, 1))
                    .build(),
                Field::builder()
                    .title("Joint Multi-band RSNA")
                    .value(self.joint_multiband_rsna)
                    .bits(BitRange::new(&bytes, 8, 1))
                    .build(),
                Field::builder()
                    .title("PeerKey Enabled")
                    .value(self.joint_multiband_rsna)
                    .bits(BitRange::new(&bytes, 9, 1))
                    .build(),
                Field::builder()
                    .title("SPP A-MSDU Capable")
                    .value(self.spp_amsdu_capable)
                    .bits(BitRange::new(&bytes, 10, 1))
                    .build(),
                Field::builder()
                    .title("SPP A-MSDU Required")
                    .value(self.spp_amsdu_required)
                    .bits(BitRange::new(&bytes, 11, 1))
                    .build(),
                Field::builder()
                    .title("PBAC")
                    .value(self.pbac)
                    .bits(BitRange::new(&bytes, 12, 1))
                    .build(),
                Field::builder()
                    .title("Extended Key ID for Individually Addressed Frames")
                    .value(self.extended_key_id_for_individually_address_frames)
                    .bits(BitRange::new(&bytes, 13, 1))
                    .build(),
                Field::builder()
                    .title("OCVC")
                    .value(self.ocvc)
                    .bits(BitRange::new(&bytes, 14, 1))
                    .build(),
                Field::reserved(BitRange::new(&bytes, 15, 1)),
            ])
            .build()
    }
}
