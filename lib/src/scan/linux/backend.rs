#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    Nl80211,
    NetworkManager,
}
