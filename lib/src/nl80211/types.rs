use std::fmt::Display;

use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};

/// The control channel width for a BSS.
/// Based on nl80211_bss_scan_width from linux/include/uapi/linux/nl80211.h
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, Serialize, Deserialize)]
#[repr(u32)]
// Keep the MHz suffix so scan-width values stay explicit in logs, serialization, and matches.
#[allow(clippy::enum_variant_names)]
pub enum BssScanWidth {
    TwentyMhz,
    TenMhz,
    FiveMhz,
    OneMhz,
    TwoMhz,
}

/// The status of a BSS.
/// Based on nl80211_bss_status from linux/include/uapi/linux/nl80211.h
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, Serialize, Deserialize)]
#[repr(u32)]
pub enum BssStatus {
    /// The local station is authenticated with the BSS.
    Authenticated = 0,
    /// The local station is associated with the BSS.
    Associated,
    /// The local station has joined the IBSS.
    IbssJoined,
    /// The BSS status is unavailable or unknown.
    Unknown,
}

impl Display for BssStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BssStatus::Authenticated => write!(f, "Authenticated"),
            BssStatus::Associated => write!(f, "Associated"),
            BssStatus::IbssJoined => write!(f, "IBSS Joined"),
            BssStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

/// The type of interface.
/// Based on nl80211_iftype from linux/include/uapi/linux/nl80211.h
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, Serialize, Deserialize)]
#[repr(u32)]
pub enum IfType {
    Unspecified = 0,
    Adhoc,
    Station,
    Ap,
    ApVlan,
    Wds,
    Monitor,
    MeshPoint,
    P2pClient,
    P2pGo,
    P2pDevice,
    Ocb,
    Nan,
}

/// The width of a channel
/// Based on nl80211_chan_width from linux/include/uapi/linux/nl80211.h
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, TryFromPrimitive)]
#[repr(u8)]
pub enum ChanWidth {
    TwentyMhzNoHt,
    TwentyMhz,
    FortyMhz,
    EightyMhz,
    EightyPlusEightyMhz,
    OneSixtyMhz,
    FiveMhzOfdm,
    TenMhzOfdm,
    OneMhzOfdm,
    TwoMhzOfdm,
    FourMhzOfdm,
    EightMhzOfdm,
    SixteenMhzOfdm,
    ThreeHundredTwentyMhz,
}
