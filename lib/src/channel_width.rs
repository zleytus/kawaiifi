use std::{collections::HashSet, convert::From, fmt::Display, hash::Hash};

use derive_more::{Deref, DerefMut, From};
use num_enum::TryFromPrimitive;

#[cfg(target_os = "linux")]
use crate::nl80211::ChanWidth;
use crate::{Ie, IeData};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, TryFromPrimitive)]
#[repr(u8)]
pub enum ChannelWidth {
    TwentyMhz,
    FortyMhz,
    EightyMhz,
    EightyPlusEightyMhz,
    OneSixtyMhz,
    ThreeHundredTwentyMhz,
}

#[cfg(target_os = "linux")]
impl From<ChanWidth> for ChannelWidth {
    fn from(chan_width: ChanWidth) -> Self {
        match chan_width {
            ChanWidth::TwentyMhz => ChannelWidth::TwentyMhz,
            ChanWidth::FortyMhz => ChannelWidth::FortyMhz,
            ChanWidth::EightyMhz => ChannelWidth::EightyMhz,
            ChanWidth::EightyPlusEightyMhz => ChannelWidth::EightyPlusEightyMhz,
            ChanWidth::OneSixtyMhz => ChannelWidth::OneSixtyMhz,
            ChanWidth::ThreeHundredTwentyMhz => ChannelWidth::ThreeHundredTwentyMhz,
            _ => ChannelWidth::TwentyMhz,
        }
    }
}

impl From<&[Ie]> for ChannelWidth {
    // From Table 11-24 in IEEE Std 802.11-2016
    fn from(ies: &[Ie]) -> Self {
        // Iterate once through the IEs and find the IEs that report the BSSs channel width
        let (mut eht_op, mut he_op, mut vht_op, mut ht_op) = (None, None, None, None);

        for ie in ies {
            match &ie.data {
                IeData::EhtOperation(op) => eht_op = Some(op),
                IeData::HeOperation(op) => he_op = Some(op),
                IeData::VhtOperation(op) => vht_op = Some(op),
                IeData::HtOperation(op) => ht_op = Some(op),
                _ => continue,
            }
        }

        // Check for an EHT Operation element
        if let Some(eht_op) = eht_op
            && let Some(channel_width) = eht_op.channel_width()
        {
            return channel_width;
        }

        // Check for an HE Operation element
        if let Some(he_op) = he_op
            && let Some(channel_width) = he_op.channel_width()
        {
            return channel_width;
        }

        // Check for a VHT Operation element
        if let Some(vht_op) = vht_op
            && let Some(channel_width) = vht_op.channel_width()
        {
            return channel_width;
        }

        // Check for an HT Operation element
        if let Some(ht_op) = ht_op {
            return ht_op.channel_width();
        }

        // If there is no EHT/HE/VHT/HT Information
        ChannelWidth::TwentyMhz
    }
}

impl Display for ChannelWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelWidth::TwentyMhz => write!(f, "20 MHz"),
            ChannelWidth::FortyMhz => write!(f, "40 MHz"),
            ChannelWidth::EightyMhz => write!(f, "80 MHz"),
            ChannelWidth::EightyPlusEightyMhz => write!(f, "80+80 MHz"),
            ChannelWidth::OneSixtyMhz => write!(f, "160 MHz"),
            ChannelWidth::ThreeHundredTwentyMhz => write!(f, "320 MHz"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut, From)]
#[from(forward)]
pub struct ChannelWidths(HashSet<ChannelWidth>);

impl Display for ChannelWidths {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|channel_width| channel_width.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
