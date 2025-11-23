use std::{collections::HashSet, convert::TryFrom, fmt::Display};

use deku::{DekuRead, DekuWrite};

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(ctx = "len: usize")]
pub struct SupportedRates {
    #[deku(count = "len")]
    bytes: Vec<u8>,
}

impl SupportedRates {
    pub const NAME: &'static str = "Supported Rates";
    pub const ID: u8 = 1;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);

    pub fn rates(&self) -> HashSet<DataRate> {
        self.bytes
            .iter()
            .filter_map(|byte| {
                if let Ok(rate) = DataRate::try_from(*byte) {
                    Some(rate)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn basic_rates(&self) -> Vec<f64> {
        self.bytes
            .iter()
            .filter_map(|byte| DataRate::try_from(*byte).ok())
            .filter_map(|rate| {
                if rate.is_basic() {
                    Some(rate.value())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn all_rates(&self) -> Vec<f64> {
        self.bytes
            .iter()
            .map(|&byte| (byte >> 1) as f64 / 2.0)
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(ctx = "len: usize")]
pub struct ExtendedSupportedRates {
    #[deku(ctx = "len")]
    supported_rates: SupportedRates,
}

impl ExtendedSupportedRates {
    pub const NAME: &'static str = "Extended Supported Rates";
    pub const ID: u8 = 50;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);

    pub fn rates(&self) -> HashSet<DataRate> {
        self.supported_rates.rates()
    }

    pub fn basic_rates(&self) -> Vec<f64> {
        self.supported_rates.basic_rates()
    }

    pub fn all_rates(&self) -> Vec<f64> {
        self.supported_rates.all_rates()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum DataRate {
    OneMbps(bool),
    TwoMbps(bool),
    FivePointFiveMbps(bool),
    SixMbps(bool),
    NineMbps(bool),
    ElevenMbps(bool),
    TwelveMbps(bool),
    EighteenMbps(bool),
    TwentyTwoMbps(bool),
    TwentyFourMbps(bool),
    ThirtyThreeMbps(bool),
    ThirtySixMbps(bool),
    FortyEightMbps(bool),
    FiftyFourMbps(bool),
}

impl DataRate {
    pub fn is_basic(&self) -> bool {
        match self {
            DataRate::OneMbps(is_basic) => *is_basic,
            DataRate::TwoMbps(is_basic) => *is_basic,
            DataRate::FivePointFiveMbps(is_basic) => *is_basic,
            DataRate::SixMbps(is_basic) => *is_basic,
            DataRate::NineMbps(is_basic) => *is_basic,
            DataRate::ElevenMbps(is_basic) => *is_basic,
            DataRate::TwelveMbps(is_basic) => *is_basic,
            DataRate::EighteenMbps(is_basic) => *is_basic,
            DataRate::TwentyTwoMbps(is_basic) => *is_basic,
            DataRate::TwentyFourMbps(is_basic) => *is_basic,
            DataRate::ThirtyThreeMbps(is_basic) => *is_basic,
            DataRate::ThirtySixMbps(is_basic) => *is_basic,
            DataRate::FortyEightMbps(is_basic) => *is_basic,
            DataRate::FiftyFourMbps(is_basic) => *is_basic,
        }
    }

    pub fn value(&self) -> f64 {
        match self {
            DataRate::OneMbps(_) => 1.0,
            DataRate::TwoMbps(_) => 2.0,
            DataRate::FivePointFiveMbps(_) => 5.5,
            DataRate::SixMbps(_) => 6.0,
            DataRate::NineMbps(_) => 9.0,
            DataRate::ElevenMbps(_) => 11.0,
            DataRate::TwelveMbps(_) => 12.0,
            DataRate::EighteenMbps(_) => 18.0,
            DataRate::TwentyTwoMbps(_) => 22.0,
            DataRate::TwentyFourMbps(_) => 24.0,
            DataRate::ThirtyThreeMbps(_) => 33.0,
            DataRate::ThirtySixMbps(_) => 36.0,
            DataRate::FortyEightMbps(_) => 48.0,
            DataRate::FiftyFourMbps(_) => 54.0,
        }
    }
}

impl Display for DataRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_basic() {
            write!(f, "{:.1}*", self.value())
        } else {
            write!(f, "{:.1}", self.value())
        }
    }
}

impl TryFrom<u8> for DataRate {
    type Error = &'static str;

    fn try_from(rate_byte: u8) -> Result<Self, Self::Error> {
        let is_basic = (rate_byte & 0b10000000) > 0;
        let encoded_rate = rate_byte & 0b01111111;

        match encoded_rate {
            2 => Ok(DataRate::OneMbps(is_basic)),
            4 => Ok(DataRate::TwoMbps(is_basic)),
            11 => Ok(DataRate::FivePointFiveMbps(is_basic)),
            12 => Ok(DataRate::SixMbps(is_basic)),
            18 => Ok(DataRate::NineMbps(is_basic)),
            22 => Ok(DataRate::ElevenMbps(is_basic)),
            24 => Ok(DataRate::TwelveMbps(is_basic)),
            36 => Ok(DataRate::EighteenMbps(is_basic)),
            44 => Ok(DataRate::TwentyTwoMbps(is_basic)),
            48 => Ok(DataRate::TwentyFourMbps(is_basic)),
            66 => Ok(DataRate::ThirtyThreeMbps(is_basic)),
            72 => Ok(DataRate::ThirtySixMbps(is_basic)),
            96 => Ok(DataRate::FortyEightMbps(is_basic)),
            108 => Ok(DataRate::FiftyFourMbps(is_basic)),
            _ => Err("Invalid data rate"),
        }
    }
}
