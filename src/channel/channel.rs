use std::{
    convert::{From, TryFrom},
    fmt::Display,
};

use num_enum::TryFromPrimitive;

use crate::{ChannelWidth, Ie, IeData};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Channel {
    number: ChannelNumber,
    width: ChannelWidth,
}

impl Channel {
    pub fn number(&self) -> ChannelNumber {
        self.number
    }

    pub fn center_freq_mhz(&self) -> u32 {
        if self.number == ChannelNumber::Fourteen {
            return 2484;
        }

        match self.band() {
            ChannelBand::TwoPointFourGhz => 2407 + (self.number as u32 * 5),
            ChannelBand::FiveGhz => 5000 + (self.number as u32 * 5),
            ChannelBand::SixGhz => 5935 + (self.number as u32 * 5),
        }
    }

    pub fn band(&self) -> ChannelBand {
        match self.number as u8 {
            1..=14 => ChannelBand::TwoPointFourGhz,
            _ => ChannelBand::FiveGhz,
        }
    }

    pub fn width(&self) -> ChannelWidth {
        self.width
    }
}

impl From<&[Ie]> for Channel {
    fn from(ies: &[Ie]) -> Self {
        Channel {
            number: ChannelNumber::from_ies(ies).unwrap_or(ChannelNumber::One),
            width: ChannelWidth::from(ies),
        }
    }
}

impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Channel {}", self.number as u32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum ChannelBand {
    TwoPointFourGhz,
    FiveGhz,
    SixGhz,
}

impl Display for ChannelBand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelBand::TwoPointFourGhz => write!(f, "2.4 GHz"),
            ChannelBand::FiveGhz => write!(f, "5 GHz"),
            ChannelBand::SixGhz => write!(f, "6 GHz"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum ChannelNumber {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Eleven = 11,
    Twelve = 12,
    Thirteen = 13,
    Fourteen = 14,
    ThirtyTwo = 32,
    ThirtyFour = 34,
    ThirtySix = 36,
    ThirtyEight = 38,
    Forty = 40,
    FortyTwo = 42,
    FortyFour = 44,
    FortySix = 46,
    FortyEight = 48,
    Fifty = 50,
    FiftyTwo = 52,
    FiftyFour = 54,
    FiftySix = 56,
    FiftyEight = 58,
    Sixty = 60,
    SixtyTwo = 62,
    SixtyFour = 64,
    SixtyEight = 68,
    NinetySix = 96,
    OneHundred = 100,
    OneHundredTwo = 102,
    OneHundredFour = 104,
    OneHundredSix = 106,
    OneHundredEight = 108,
    OneHundredTen = 110,
    OneHundredTwelve = 112,
    OneHundredFourteen = 114,
    OneHundredSixteen = 116,
    OneHundredEighteen = 118,
    OneHundredTwenty = 120,
    OneHundredTwentyTwo = 122,
    OneHundredTwentyFour = 124,
    OneHundredTwentySix = 126,
    OneHundredTwentyEight = 128,
    OneHundredThirtyTwo = 132,
    OneHundredThirtyFour = 134,
    OneHundredThirtySix = 136,
    OneHundredThirtyEight = 138,
    OneHundredForty = 140,
    OneHundredFortyTwo = 142,
    OneHundredFortyFour = 144,
    OneHundredFortyNine = 149,
    OneHundredFiftyOne = 151,
    OneHundredFiftyThree = 153,
    OneHundredFiftyFive = 155,
    OneHundredFiftySeven = 157,
    OneHundredFiftyNine = 159,
    OneHundredSixtyOne = 161,
    OneHundredSixtyFive = 165,
    OneHundredSixtyNine = 169,
    OneHundredSeventyThree = 173,
    OneHundredSeventySeven = 177,
    OneHundredEightyOne = 181,
}

impl ChannelNumber {
    pub fn from_ies(ies: &[Ie]) -> Option<Self> {
        let channel_number = ies.iter().find_map(|ie| match &ie.data {
            IeData::DsParameterSet(ds_parameter_set) => Some(ds_parameter_set.current_channel),
            IeData::HtOperation(ht_operation) => Some(ht_operation.primary_channel),
            _ => None,
        });

        if let Some(channel_number) = channel_number {
            Self::try_from(channel_number).ok()
        } else {
            None
        }
    }
}

impl Display for ChannelNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_from_channel_number() {
        assert_eq!(ChannelNumber::try_from(1).unwrap(), ChannelNumber::One);
        assert_eq!(ChannelNumber::try_from(6).unwrap(), ChannelNumber::Six);
        assert_eq!(ChannelNumber::try_from(11).unwrap(), ChannelNumber::Eleven);
        assert_eq!(
            ChannelNumber::try_from(14).unwrap(),
            ChannelNumber::Fourteen
        );
    }

    #[test]
    #[should_panic]
    fn invalid_channel_number() {
        ChannelNumber::try_from(15).unwrap();
    }
}
