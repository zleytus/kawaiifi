use std::{collections::HashSet, convert::From, fmt::Display, hash::Hash};

use derive_more::{Deref, DerefMut, From};
use num_enum::TryFromPrimitive;

use crate::{Ie, IeData, ies::ht_operation, nl80211::ChanWidth};

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
        // Check for an EHT Operation element
        if let Some(eht_operation) = ies.iter().find_map(|ie| match &ie.data {
            IeData::EhtOperation(eht_operation) => Some(eht_operation),
            _ => None,
        }) {
            if let Some(channel_width) = eht_operation.channel_width() {
                return channel_width;
            }
        }

        // Check for an HE Operation element
        if let Some(he_operation) = ies.iter().find_map(|ie| match &ie.data {
            IeData::HeOperation(he_operation) => Some(he_operation),
            _ => None,
        }) {
            if let Some(channel_width) = he_operation.channel_width() {
                return channel_width;
            }
        }

        // Check for a VHT Operation element
        if let Some(vht_operation) = ies.iter().find_map(|ie| match &ie.data {
            IeData::VhtOperation(vht_operation) => Some(vht_operation),
            _ => None,
        }) {
            if let Some(channel_width) = vht_operation.channel_width() {
                return channel_width;
            }
        }

        // Check for an HT Operation element
        if let Some(ht_operation) = ies.iter().find_map(|ie| match &ie.data {
            IeData::HtOperation(ht_operation) => Some(ht_operation),
            _ => None,
        }) {
            match ht_operation.ht_operation_information.sta_channel_width {
                ht_operation::SupportedChannelWidths::TwentyMhz => return ChannelWidth::TwentyMhz,
                ht_operation::SupportedChannelWidths::Any => return ChannelWidth::FortyMhz,
            }
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
