use std::fmt::Display;

use derive_more::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, DerefMut, From, Not,
};
use enumflags2::{BitFlags, bitflags};

use crate::{Band, Ie, IeData, ies::supported_rates::DataRate};

/// An 802.11 Wi-Fi protocol generation supported by a BSS.
#[bitflags]
#[derive(Copy, Clone, Debug, PartialEq, Ord, PartialOrd, Eq)]
#[repr(u16)]
pub enum WifiProtocol {
    /// 802.11a — 5 GHz OFDM.
    A = 1 << 0,
    /// 802.11b — 2.4 GHz DSSS.
    B = 1 << 1,
    /// 802.11g — 2.4 GHz OFDM.
    G = 1 << 2,
    /// 802.11n — HT (Wi-Fi 4).
    N = 1 << 3,
    /// 802.11ac — VHT (Wi-Fi 5).
    AC = 1 << 4,
    /// 802.11ax — HE (Wi-Fi 6/6E).
    AX = 1 << 5,
    /// 802.11be — EHT (Wi-Fi 7).
    BE = 1 << 6,
}

impl Display for WifiProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WifiProtocol::A => write!(f, "a"),
            WifiProtocol::B => write!(f, "b"),
            WifiProtocol::G => write!(f, "g"),
            WifiProtocol::N => write!(f, "n"),
            WifiProtocol::AC => write!(f, "ac"),
            WifiProtocol::AX => write!(f, "ax"),
            WifiProtocol::BE => write!(f, "be"),
        }
    }
}

/// A set of [`WifiProtocol`] flags indicating which 802.11 protocol generations a BSS supports.
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Deref,
    DerefMut,
    BitAnd,
    BitAndAssign,
    BitOr,
    BitOrAssign,
    BitXor,
    BitXorAssign,
    From,
    Not,
)]
pub struct WifiProtocols(BitFlags<WifiProtocol>);

impl PartialEq<BitFlags<WifiProtocol, u16>> for WifiProtocols {
    fn eq(&self, other: &BitFlags<WifiProtocol, u16>) -> bool {
        self.0.eq(other)
    }
}

impl Ord for WifiProtocols {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.bits().cmp(&other.bits())
    }
}

impl PartialOrd for WifiProtocols {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl WifiProtocols {
    pub(crate) fn from_ies_for_band(ies: &[Ie], band: Band) -> Self {
        let mut protocols = WifiProtocols(BitFlags::empty());

        for ie in ies {
            match &ie.data {
                IeData::SupportedRates(supported_rates) => {
                    protocols.insert(*protocols_from_rates(supported_rates.rates(), band));
                }
                IeData::ExtendedSupportedRates(supported_rates) => {
                    protocols.insert(*protocols_from_rates(supported_rates.rates(), band));
                }
                IeData::HtCapabilities(_) => protocols.insert(WifiProtocol::N),
                IeData::VhtCapabilities(_) => protocols.insert(WifiProtocol::AC),
                IeData::HeCapabilities(_) => protocols.insert(WifiProtocol::AX),
                IeData::EhtCapabilities(_) => protocols.insert(WifiProtocol::BE),
                _ => continue,
            }
        }

        protocols
    }
}

impl Display for WifiProtocols {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.iter()
                .map(|protocol| protocol.to_string())
                .collect::<Vec<String>>()
                .join("/")
        )
    }
}

fn protocols_from_rates(rates: impl IntoIterator<Item = DataRate>, band: Band) -> WifiProtocols {
    let mut protocols = WifiProtocols(BitFlags::empty());

    for rate in rates {
        match rate {
            DataRate::OneMbps(_) | DataRate::TwoMbps(_) | DataRate::FivePointFiveMbps(_) => {
                if band == Band::TwoPointFourGhz {
                    protocols.insert(WifiProtocol::B)
                }
            }
            DataRate::SixMbps(_)
            | DataRate::NineMbps(_)
            | DataRate::TwelveMbps(_)
            | DataRate::EighteenMbps(_)
            | DataRate::TwentyFourMbps(_)
            | DataRate::ThirtySixMbps(_)
            | DataRate::FortyEightMbps(_)
            | DataRate::FiftyFourMbps(_) => match band {
                Band::TwoPointFourGhz => protocols.insert(WifiProtocol::G),
                Band::FiveGhz => protocols.insert(WifiProtocol::A),
                Band::SixGhz => {}
            },
            _ => continue,
        }
    }

    protocols
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ofdm_rates_are_802_11g_on_two_point_four_ghz() {
        let protocols = protocols_from_rates([DataRate::SixMbps(false)], Band::TwoPointFourGhz);

        assert!(protocols.contains(WifiProtocol::G));
        assert!(!protocols.contains(WifiProtocol::A));
    }

    #[test]
    fn ofdm_rates_are_802_11a_on_five_ghz() {
        let protocols = protocols_from_rates([DataRate::SixMbps(false)], Band::FiveGhz);

        assert!(protocols.contains(WifiProtocol::A));
        assert!(!protocols.contains(WifiProtocol::G));
    }

    #[test]
    fn legacy_rates_do_not_mark_six_ghz_as_802_11a_or_802_11g() {
        let protocols = protocols_from_rates([DataRate::SixMbps(false)], Band::SixGhz);

        assert!(!protocols.contains(WifiProtocol::A));
        assert!(!protocols.contains(WifiProtocol::G));
    }

    #[test]
    fn dsss_rates_are_802_11b_only_on_two_point_four_ghz() {
        let two_point_four =
            protocols_from_rates([DataRate::OneMbps(false)], Band::TwoPointFourGhz);
        let five = protocols_from_rates([DataRate::OneMbps(false)], Band::FiveGhz);

        assert!(two_point_four.contains(WifiProtocol::B));
        assert!(!five.contains(WifiProtocol::B));
    }
}
