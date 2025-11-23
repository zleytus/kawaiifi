use deku::{DekuRead, DekuWrite};

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(ctx = "len: usize")]
pub struct TimeAdvertisement {
    pub timing_capabilities: TimingCapabilities,
    #[deku(
        cond = "*timing_capabilities == TimingCapabilities::TimestampOffsetBasedOnUtc || *timing_capabilities == TimingCapabilities::UtcTimeAtWhichTsfTimerIs0"
    )]
    pub time_value: Option<TimeValue>,
    #[deku(
        cond = "*timing_capabilities == TimingCapabilities::TimestampOffsetBasedOnUtc || *timing_capabilities == TimingCapabilities::UtcTimeAtWhichTsfTimerIs0",
        bytes = 5
    )]
    pub time_error: Option<u64>,
    #[deku(
        cond = "*timing_capabilities == TimingCapabilities::UtcTimeAtWhichTsfTimerIs0",
        bytes = 1
    )]
    pub time_update_counter: Option<u8>,
    #[deku(
        cond = "matches!(*timing_capabilities, TimingCapabilities::Reserved(_))",
        count = "len.checked_sub(1).unwrap_or_default()"
    )]
    pub reserved: Option<Vec<u8>>,
}

impl TimeAdvertisement {
    pub const NAME: &'static str = "Time Advertisement";
    pub const ID: u8 = 69;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
#[repr(u8)]
pub enum TimingCapabilities {
    NoStandardizedExternalTimeSource = 0,
    TimestampOffsetBasedOnUtc = 1,
    UtcTimeAtWhichTsfTimerIs0 = 2,
    #[deku(id_pat = "_")]
    Reserved(u8),
}

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
pub struct TimeValue {
    #[deku(bytes = 2)]
    pub year: u16,
    #[deku(bytes = 1)]
    pub month: u8,
    #[deku(bytes = 1)]
    pub day_of_month: u8,
    #[deku(bytes = 1)]
    pub hours: u8,
    #[deku(bytes = 1)]
    pub minutes: u8,
    #[deku(bytes = 1)]
    pub seconds: u8,
    #[deku(bytes = 2)]
    pub milliseconds: u16,
    #[deku(bytes = 1)]
    reserved: u8,
}
