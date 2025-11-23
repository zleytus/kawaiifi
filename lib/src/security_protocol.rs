use crate::Ie;
use derive_more::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, DerefMut, From, Not,
};
use enumflags2::{bitflags, BitFlags};
use std::convert::From;

#[bitflags]
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
#[repr(u8)]
pub enum SecurityProtocol {
    WEP = 1 << 0,
    WPA = 1 << 1,
    WPA2 = 1 << 2,
    WPA3 = 1 << 3,
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
    fn from(_: &[Ie]) -> Self {
        SecurityProtocols(BitFlags::empty())
    }
}
