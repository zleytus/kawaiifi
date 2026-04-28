/// The backend used to perform Wi-Fi scans on Linux.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    /// Scan via nl80211, the Linux kernel's native Wi-Fi netlink interface.
    Nl80211,
    /// Scan via NetworkManager's D-Bus API.
    NetworkManager,
}
