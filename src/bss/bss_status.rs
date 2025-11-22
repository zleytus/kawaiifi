use num_enum::TryFromPrimitive;

/// The status of a BSS.
/// Based on nl80211_bss_status from linux/include/uapi/linux/nl80211.h
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive)]
#[repr(u32)]
pub enum BssStatus {
    Authenticated = 0,
    Associated,
    IbssJoined,
    NotAssociated = u32::MAX,
}
