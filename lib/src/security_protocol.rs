use std::{convert::From, fmt::Display};

use derive_more::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, DerefMut, From, Not,
};
use enumflags2::{BitFlags, bitflags};

use crate::{Ie, IeData};

#[bitflags]
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
#[repr(u8)]
pub enum SecurityProtocol {
    WEP = 1 << 0,
    WPA = 1 << 1,
    WPA2 = 1 << 2,
    WPA3 = 1 << 3,
}

impl Display for SecurityProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityProtocol::WEP => write!(f, "WEP"),
            SecurityProtocol::WPA => write!(f, "WPA"),
            SecurityProtocol::WPA2 => write!(f, "WPA2"),
            SecurityProtocol::WPA3 => write!(f, "WPA3"),
        }
    }
}

// Use the Newtype pattern to create a type alias (SecurityProtocols) and implement the From trait
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
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
pub struct SecurityProtocols(BitFlags<SecurityProtocol>);

impl PartialEq<BitFlags<SecurityProtocol, u8>> for SecurityProtocols {
    fn eq(&self, other: &BitFlags<SecurityProtocol, u8>) -> bool {
        self.0.eq(other)
    }
}

impl From<&[Ie]> for SecurityProtocols {
    fn from(ies: &[Ie]) -> Self {
        let mut security_protocols = BitFlags::empty();
        let rsn = ies.iter().find_map(|ie| match &ie.data {
            IeData::Rsn(rsn) => Some(rsn),
            _ => None,
        });
        let wpa = ies.iter().find_map(|ie| match &ie.data {
            IeData::VendorSpecific(vendor_specific) if vendor_specific.is_wpa() => {
                Some(vendor_specific)
            }
            _ => None,
        });
        if let Some(rsn) = rsn {
            if let Some(akm_suites) = &rsn.akm_suite_list {
                for akm_suite in akm_suites {
                    match akm_suite.suite_type.0 {
                        1 | 2 => security_protocols.set(SecurityProtocol::WPA2, true),
                        8 => security_protocols.set(SecurityProtocol::WPA3, true),
                        _ => continue,
                    }
                }
            }
        }
        if wpa.is_some() {
            security_protocols.set(SecurityProtocol::WPA, true);
        }
        SecurityProtocols(security_protocols)
    }
}

impl Display for SecurityProtocols {
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
