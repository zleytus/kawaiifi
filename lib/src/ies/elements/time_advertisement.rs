use std::fmt::Display;

use deku::{DekuContainerWrite, DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use crate::{Field, ies::IeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "len: usize")]
pub struct TimeAdvertisement {
    pub timing_capabilities: TimingCapabilities,
    #[deku(
        cond = "*timing_capabilities == TimingCapabilities::TimestampOffsetBasedOnUtc",
        bytes = 10
    )]
    pub time_value_ns: Option<i128>,
    #[deku(cond = "*timing_capabilities == TimingCapabilities::UtcTimeAtWhichTsfTimerIs0")]
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

    pub fn summary(&self) -> String {
        self.timing_capabilities.to_string()
    }

    pub fn fields(&self) -> Vec<Field> {
        let mut fields = vec![
            Field::builder()
                .title("Timing Capabilities")
                .value(self.timing_capabilities)
                .bytes(self.timing_capabilities.to_bytes().unwrap_or_default())
                .build(),
        ];

        if let Some(time_value_ns) = self.time_value_ns {
            fields.push(
                Field::builder()
                    .title("Time Value")
                    .value(time_value_ns)
                    .units("ns")
                    .bytes(time_value_ns.to_le_bytes()[0..10].to_vec())
                    .build(),
            );
        }

        if let Some(time_value) = &self.time_value {
            fields.push(time_value.to_field());
        }

        if let Some(time_error) = self.time_error {
            match &self.timing_capabilities {
                TimingCapabilities::TimestampOffsetBasedOnUtc => {
                    fields.push(
                        Field::builder()
                            .title("Time Error")
                            .value(time_error)
                            .units("ns")
                            .bytes(time_error.to_le_bytes()[..5].to_vec())
                            .build(),
                    );
                }
                TimingCapabilities::UtcTimeAtWhichTsfTimerIs0 => {
                    fields.push(
                        Field::builder()
                            .title("Time Error")
                            .value(time_error)
                            .units("ms")
                            .bytes(time_error.to_le_bytes()[..5].to_vec())
                            .build(),
                    );
                }
                _ => (),
            };
        }

        if let Some(time_update_counter) = self.time_update_counter {
            fields.push(
                Field::builder()
                    .title("Time Update Counter")
                    .value(time_update_counter)
                    .byte(time_update_counter)
                    .build(),
            );
        }

        fields
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(id_type = "u8")]
#[repr(u8)]
pub enum TimingCapabilities {
    NoStandardizedExternalTimeSource = 0,
    TimestampOffsetBasedOnUtc = 1,
    UtcTimeAtWhichTsfTimerIs0 = 2,
    #[deku(id_pat = "_")]
    Reserved(u8),
}

impl Display for TimingCapabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoStandardizedExternalTimeSource => {
                write!(f, "No standardized external time source")
            }
            Self::TimestampOffsetBasedOnUtc => write!(f, "Timestamp offset based on UTC"),
            Self::UtcTimeAtWhichTsfTimerIs0 => write!(f, "UTC time at which the TSF timer is 0"),
            Self::Reserved(val) => write!(f, "Reserved ({})", val),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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

impl TimeValue {
    pub fn to_field(&self) -> Field {
        Field::builder()
            .title("Time Value")
            .value("")
            .subfields([
                Field::builder()
                    .title("Year")
                    .value(self.year)
                    .bytes(self.year.to_le_bytes().to_vec())
                    .build(),
                Field::builder()
                    .title("Month")
                    .value(self.month)
                    .byte(self.month)
                    .build(),
                Field::builder()
                    .title("Day")
                    .value(self.day_of_month)
                    .byte(self.day_of_month)
                    .build(),
                Field::builder()
                    .title("Hours")
                    .value(self.hours)
                    .byte(self.hours)
                    .build(),
                Field::builder()
                    .title("Minutes")
                    .value(self.minutes)
                    .byte(self.minutes)
                    .build(),
                Field::builder()
                    .title("Seconds")
                    .value(self.seconds)
                    .byte(self.seconds)
                    .build(),
                Field::builder()
                    .title("Milliseconds")
                    .value(self.milliseconds)
                    .bytes(self.milliseconds.to_le_bytes().to_vec())
                    .build(),
                Field::builder()
                    .title("Reserved")
                    .value(self.reserved)
                    .byte(self.reserved)
                    .build(),
            ])
            .bytes(self.to_bytes().unwrap_or_default())
            .build()
    }
}
