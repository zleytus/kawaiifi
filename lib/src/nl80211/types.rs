use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};

/// The control channel width for a BSS.
/// Based on nl80211_bss_scan_width from linux/include/uapi/linux/nl80211.h
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, Serialize, Deserialize)]
#[repr(u32)]
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
    Authenticated = 0,
    Associated,
    IbssJoined,
    NotAssociated = u32::MAX,
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
