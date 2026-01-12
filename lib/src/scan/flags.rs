use deku::prelude::*;
use serde::{Deserialize, Serialize};

/// Flags that control scan behavior and indicate scan characteristics.
///
/// These flags are provided by the nl80211 interface and indicate various
/// scan parameters and optimizations used during WiFi scanning operations.
/// They correspond to the `NL80211_ATTR_SCAN_FLAGS` attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct Flags {
    /// Low priority scan - may be interrupted by higher priority operations.
    ///
    /// The scan can be delayed or paused to allow normal data transmission
    /// or other higher priority operations to proceed.
    #[deku(bits = 1)]
    pub low_priority: bool,

    /// Flush cached scan results before starting a new scan.
    ///
    /// When set, the driver will discard previously cached BSS entries
    /// before reporting new scan results.
    #[deku(bits = 1)]
    pub flush: bool,

    /// Force a scan even if the interface is an AP.
    ///
    /// Indicates this scan was initiated by an AP, which may have
    /// different scanning behavior than client devices.
    #[deku(bits = 1)]
    pub ap: bool,

    /// Use a random MAC address for probe requests.
    ///
    /// Privacy feature that randomizes the device's MAC address during
    /// active scanning to prevent tracking across networks.
    #[deku(bits = 1)]
    pub random_addr: bool,

    /// Fill the dwell time in the FILS request parameters IE in the probe request
    #[deku(bits = 1)]
    pub fils_max_channel_time: bool,

    /// Accept broadcast probe responses.
    #[deku(bits = 1)]
    pub accept_bcast_probe_resp: bool,

    /// Send probe request frames at rate of at least 5.5M.
    #[deku(bits = 1)]
    pub oce_probe_req_high_tx_rate: bool,

    /// Allow probe request tx deferral and suppression.
    #[deku(bits = 1)]
    pub oce_probe_req_deferral_suppression: bool,

    /// Perform the scan with minimal time on each channel.
    #[deku(bits = 1)]
    pub low_span: bool,

    /// Perform the scan with lower power.
    #[deku(bits = 1)]
    pub low_power: bool,

    /// Perform the scan with highest accuracy to find all available networks.
    #[deku(bits = 1)]
    pub high_accuracy: bool,

    /// Use random sequence numbers in probe requests.
    #[deku(bits = 1)]
    pub random_sn: bool,

    /// Use minimal content in probe requests.
    #[deku(bits = 1)]
    pub min_preq_content: bool,

    /// Frequencies specified in kHz (not MHz).
    #[deku(bits = 1)]
    pub freq_khz: bool,

    /// Discover colocated 6 GHz APs through RNR.
    #[deku(bits = 1)]
    pub colocated_6ghz: bool,

    #[deku(bits = 17)]
    reserved: u32,
}
