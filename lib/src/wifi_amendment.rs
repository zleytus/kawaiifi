use std::fmt::Display;

use derive_more::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, DerefMut, From, Not,
};
use enumflags2::{BitFlags, bitflags};

#[cfg(any(target_os = "linux", target_os = "windows"))]
use crate::bss::CapabilityInfo;
use crate::{Ie, IeData};

/// An 802.11 amendment supported by a BSS.
#[bitflags]
#[derive(Copy, Clone, Debug, PartialEq, Ord, PartialOrd, Eq)]
#[repr(u16)]
pub enum WifiAmendment {
    /// 802.11d — operation in additional regulatory domains.
    D = 1 << 0,
    /// 802.11e — QoS enhancements and EDCA.
    E = 1 << 1,
    /// 802.11h — spectrum management (DFS and TPC) for 5 GHz.
    H = 1 << 2,
    /// 802.11i — RSN security (WPA2).
    I = 1 << 3,
    /// 802.11k — radio resource measurement.
    K = 1 << 4,
    /// 802.11r — fast BSS transition (FT roaming).
    R = 1 << 5,
    /// 802.11s — mesh networking.
    S = 1 << 6,
    /// 802.11v — BSS transition management.
    V = 1 << 7,
    /// 802.11w — management frame protection (MFP).
    W = 1 << 8,
}

impl Display for WifiAmendment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WifiAmendment::D => write!(f, "d"),
            WifiAmendment::E => write!(f, "e"),
            WifiAmendment::H => write!(f, "h"),
            WifiAmendment::I => write!(f, "i"),
            WifiAmendment::K => write!(f, "k"),
            WifiAmendment::R => write!(f, "r"),
            WifiAmendment::S => write!(f, "s"),
            WifiAmendment::V => write!(f, "v"),
            WifiAmendment::W => write!(f, "w"),
        }
    }
}

/// A set of [`WifiAmendment`] flags indicating which 802.11 amendments a BSS supports.
#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Deref,
    DerefMut,
    BitAnd,
    BitAndAssign,
    BitOr,
    BitOrAssign,
    BitXor,
    BitXorAssign,
    From,
    Not,
)]
pub struct WifiAmendments(BitFlags<WifiAmendment>);

impl PartialEq<BitFlags<WifiAmendment, u16>> for WifiAmendments {
    fn eq(&self, other: &BitFlags<WifiAmendment, u16>) -> bool {
        self.0.eq(other)
    }
}

impl Ord for WifiAmendments {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.bits().cmp(&other.bits())
    }
}

impl PartialOrd for WifiAmendments {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
impl From<&CapabilityInfo> for WifiAmendments {
    fn from(capability_info: &CapabilityInfo) -> Self {
        let mut amendments = WifiAmendments(BitFlags::empty());

        if capability_info.qos {
            amendments.insert(WifiAmendment::E);
        }

        if capability_info.spectrum_management {
            amendments.insert(WifiAmendment::H);
        }

        if capability_info.radio_measurement {
            amendments.insert(WifiAmendment::K);
        }

        amendments
    }
}

impl From<&[Ie]> for WifiAmendments {
    fn from(ies: &[Ie]) -> Self {
        let mut amendments = WifiAmendments(BitFlags::empty());

        for ie in ies {
            match &ie.data {
                IeData::Country(_) => amendments.insert(WifiAmendment::D),
                IeData::Rsn(rsn) => {
                    amendments.insert(WifiAmendment::I);

                    if let Some(rsn_caps) = rsn.rsn_capabilities
                        && (rsn_caps.mfpc || rsn_caps.mfpr)
                    {
                        amendments.insert(WifiAmendment::W);
                    }
                }
                IeData::MobilityDomain(_) => amendments.insert(WifiAmendment::R),
                IeData::MeshId(_) | IeData::MeshConfiguration(_) => {
                    amendments.insert(WifiAmendment::S)
                }
                IeData::ExtendedCapabilities(extended_caps) if extended_caps.bss_transition() => {
                    amendments.insert(WifiAmendment::V);
                }
                _ => continue,
            }
        }

        amendments
    }
}

impl Display for WifiAmendments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.iter()
                .map(|amendment| amendment.to_string())
                .collect::<Vec<String>>()
                .join("/")
        )
    }
}
