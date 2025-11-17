/// Netlink attributes for a BSS.
/// Based on nl80211_bss from linux/include/uapi/linux/nl80211.h
#[neli::neli_enum(serialized_type = "u16")]
pub(crate) enum Nl80211Bss {
    Invalid = 0,
    Bssid = 1,
    Frequency = 2,
    Tsf = 3,
    BeaconInterval = 4,
    Capability = 5,
    InformationElements = 6,
    SignalMbm = 7,
    SignalUnspec = 8,
    Status = 9,
    SeenMsAgo = 10,
    BeaconIes = 11,
    ChanWidth = 12,
    BeaconTsf = 13,
    PrespData = 14,
    LastSeenBoottime = 15,
    Pad = 16,
    ParentTsf = 17,
    ParentBssid = 18,
    ChainSignal = 19,
    FrequencyOffset = 20,
    MloLinkId = 21,
    MldAddr = 22,
    UseFor = 23,
    CannotUseReasons = 24,
}

impl neli::consts::genl::NlAttrType for Nl80211Bss {}
