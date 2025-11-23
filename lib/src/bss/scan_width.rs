use num_enum::TryFromPrimitive;

/// The control channel width for a BSS.
/// Based on nl80211_bss_scan_width from linux/include/uapi/linux/nl80211.h
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive)]
#[repr(u32)]
pub enum ScanWidth {
    TwentyMhz,
    TenMhz,
    FiveMhz,
    OneMhz,
    TwoMhz,
}
