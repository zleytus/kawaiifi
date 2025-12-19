use std::{fmt::Display, ops::RangeInclusive};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub enum Band {
    #[default]
    TwoPointFourGhz,
    FiveGhz,
    SixGhz,
}

impl Band {
    pub(crate) fn from_freq_mhz(freq_mhz: u32) -> Self {
        if Self::TwoPointFourGhz.range_mhz().contains(&freq_mhz) {
            Self::TwoPointFourGhz
        } else if Self::FiveGhz.range_mhz().contains(&freq_mhz) {
            Self::FiveGhz
        } else if Self::SixGhz.range_mhz().contains(&freq_mhz) {
            Self::SixGhz
        } else {
            Self::TwoPointFourGhz
        }
    }

    pub const fn min_freq_mhz(&self) -> u32 {
        match self {
            Band::TwoPointFourGhz => 2401,
            Band::FiveGhz => 5150,
            Band::SixGhz => 5945,
        }
    }

    pub const fn max_freq_mhz(&self) -> u32 {
        match self {
            Band::TwoPointFourGhz => 2495,
            Band::FiveGhz => 5895,
            Band::SixGhz => 7125,
        }
    }

    pub const fn range_mhz(&self) -> RangeInclusive<u32> {
        match self {
            Self::TwoPointFourGhz => {
                Self::TwoPointFourGhz.min_freq_mhz()..=Self::TwoPointFourGhz.max_freq_mhz()
            }
            Self::FiveGhz => Self::FiveGhz.min_freq_mhz()..=Self::FiveGhz.max_freq_mhz(),
            Self::SixGhz => Self::SixGhz.min_freq_mhz()..=Self::SixGhz.max_freq_mhz(),
        }
    }
}

impl Display for Band {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Band::TwoPointFourGhz => write!(f, "2.4 GHz"),
            Band::FiveGhz => write!(f, "5 GHz"),
            Band::SixGhz => write!(f, "6 GHz"),
        }
    }
}
