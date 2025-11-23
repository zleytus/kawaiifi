use std::{collections::HashSet, convert::From, fmt::Display, hash::Hash};

use derive_more::{Deref, DerefMut, From};
use num_enum::TryFromPrimitive;

use crate::{Ie, IeData, ies::ht_operation::SupportedChannelWidths};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, TryFromPrimitive)]
#[repr(u8)]
pub enum ChannelWidth {
    TwentyMhzNoHt,
    TwentyMhz,
    FortyMhz,
    EightyMhz,
    EightyPlusEightyMhz,
    OneSixtyMhz,
    FiveMhzOfdm,
    TenMhzOfdm,
    OneMhzOfdm,
    TwoMhzOfdm,
    FourMhzOfdm,
    EightMhzOfdm,
    SixteenMhzOfdm,
    ThreeHundredTwentyMhz,
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
                SupportedChannelWidths::TwentyMhz => return ChannelWidth::TwentyMhz,
                SupportedChannelWidths::Any => return ChannelWidth::FortyMhz,
            }
        }

        // If there is no EHT/HE/VHT/HT Information
        ChannelWidth::TwentyMhzNoHt
    }
}

impl Display for ChannelWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelWidth::TwentyMhzNoHt => write!(f, "20 MHz, non-HT"),
            ChannelWidth::TwentyMhz => write!(f, "20 MHz"),
            ChannelWidth::FortyMhz => write!(f, "40 MHz"),
            ChannelWidth::EightyMhz => write!(f, "80 MHz"),
            ChannelWidth::EightyPlusEightyMhz => write!(f, "80+80 MHz"),
            ChannelWidth::OneSixtyMhz => write!(f, "160 MHz"),
            ChannelWidth::FiveMhzOfdm => write!(f, "5 MHz OFDM"),
            ChannelWidth::TenMhzOfdm => write!(f, "10 MHz OFDM"),
            ChannelWidth::OneMhzOfdm => write!(f, "1 MHz OFDM"),
            ChannelWidth::TwoMhzOfdm => write!(f, "2 MHz OFDM"),
            ChannelWidth::FourMhzOfdm => write!(f, "4 MHz OFDM"),
            ChannelWidth::EightMhzOfdm => write!(f, "8 MHz OFDM"),
            ChannelWidth::SixteenMhzOfdm => write!(f, "16 MHz OFDM"),
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
