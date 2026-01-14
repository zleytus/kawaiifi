use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::IeId;

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
    pub pmkid_list: Option<Vec<u64>>,
    #[deku(
        cond = "len >= 8 + usize::from(4 * pairwise_cipher_suite_count.unwrap_or_default()) + 2 + usize::from(4 * akm_suite_count.unwrap_or_default()) + 2 + 2 + usize::from(16 * pmkid_count.unwrap_or_default()) + 4"
    )]
    pub group_management_cipher_suite: Option<CipherSuite>,
}

impl Rsn {
    pub const NAME: &'static str = "RSN";
    pub const ID: u8 = 48;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct CipherSuite {
    pub oui: [u8; 3],
    pub suite_type: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct AkmSuite {
    pub oui: [u8; 3],
    pub suite_type: u8,
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
