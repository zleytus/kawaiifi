use std::fmt::Display;

/// The backend used to perform Wi-Fi scans on Linux.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Backend {
    /// Scan via nl80211, the Linux kernel's native Wi-Fi netlink interface.
    #[allow(unused)]
    Nl80211,
    /// Scan via NetworkManager's D-Bus API.
    NetworkManager,
}

impl Display for Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Backend::Nl80211 => write!(f, "nl80211"),
            Backend::NetworkManager => write!(f, "NetworkManager"),
        }
    }
}
