//! `kawaiifi` is a Wi-Fi scanning library for Linux, macOS, and Windows.
//!
//! It discovers local Basic Service Sets (BSSs) and reports their SSID, BSSID,
//! signal strength, channel, channel width, security protocols, and information
//! elements (IEs).
//!
//! ## Obtaining a Wi-Fi Interface
//!
//! Use [`default_interface`] to get the first available interface.
//!
//! ```
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! use kawaiifi::Interface;
//!
//! let interface: Interface = kawaiifi::default_interface().ok_or("No Wi-Fi interface found")?;
//! # Ok(())
//! # }
//! ```
//!
//! Use [`interfaces`] to get all available interfaces.
//!
//! ```
//! # fn example() {
//! use kawaiifi::Interface;
//!
//! let interfaces: Vec<Interface> = kawaiifi::interfaces();
//! # }
//! ```
//!
//! Some [`Interface`] properties are platform-specific.
//!
//! ```
//! # fn print_interface_properties(interface: &kawaiifi::Interface) {
//! #[cfg(target_os = "linux")]
//! println!("Index: {}", interface.index());
//!
//! #[cfg(target_os = "macos")]
//! println!("Noise: {} dBm", interface.noise_dbm());
//!
//! #[cfg(target_os = "windows")]
//! println!("Description: {}", interface.description());
//! # }
//! ```
//!
//! ## Triggering a Wi-Fi Scan
//!
//! Both blocking and asynchronous scans are available through
//! [`Interface::scan_blocking`] and [`Interface::scan`].
//!
//! On Linux, scans can be triggered through either NetworkManager or nl80211
//! (Netlink), so a [`Backend`] must be specified.
//!
//! ```no_run
//! # #[cfg(target_os = "linux")]
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use kawaiifi::{Backend, Scan};
//!
//! # let interface = kawaiifi::default_interface().ok_or("No Wi-Fi interface found")?;
//! let scan: Scan = interface.scan_blocking(Backend::NetworkManager)?;
//! # Ok(())
//! # }
//! # #[cfg(not(target_os = "linux"))]
//! # fn main() {}
//! ```
//!
//! On macOS and Windows, scans are triggered through CoreWLAN and Native WiFi
//! respectively.
//!
//! ```no_run
//! # #[cfg(any(target_os = "macos", target_os = "windows"))]
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use kawaiifi::Scan;
//!
//! # let interface = kawaiifi::default_interface().ok_or("No Wi-Fi interface found")?;
//! let scan: Scan = interface.scan_blocking()?;
//! # Ok(())
//! # }
//! # #[cfg(not(any(target_os = "macos", target_os = "windows")))]
//! # fn main() {}
//! ```
//!
//! ## Accessing BSS Data
//!
//! [`Scan`] contains a list of BSSs that are accessed through
//! [`Scan::bss_list`].
//!
//! ```
//! # fn print_bss_count(scan: &kawaiifi::Scan) {
//! use kawaiifi::Bss;
//!
//! let bss_list: &[Bss] = scan.bss_list();
//! println!("Found {} BSS(s)", bss_list.len());
//! # }
//! ```
//!
//! [`Bss`] exposes common properties that are available on all platforms.
//!
//! ```
//! # fn print_bss_data(bss: &kawaiifi::Bss) {
//! println!("BSSID: {:?}", bss.bssid());
//! println!("SSID: {:?}", bss.ssid());
//! println!("Frequency: {} MHz", bss.frequency_mhz());
//! println!("Channel: {}", bss.channel_number());
//! println!("Channel Width: {}", bss.channel_width());
//! println!("Signal: {} dBm", bss.signal_dbm());
//! println!("Security: {}", bss.security_protocols());
//! println!("Wi-Fi Protocols: {}", bss.wifi_protocols());
//! println!("Max Rate: {} Mbps", bss.max_rate_mbps());
//! # }
//! ```
//!
//! Some [`Bss`] properties are platform-specific.
//!
//! ```
//! # fn print_platform_bss_data(bss: &kawaiifi::Bss) {
//! #[cfg(target_os = "linux")]
//! println!("Status: {:?}", bss.status());
//!
//! #[cfg(target_os = "macos")]
//! println!("Noise: {} dBm", bss.noise_dbm());
//!
//! #[cfg(target_os = "windows")]
//! println!("Link Quality: {}", bss.link_quality());
//! # }
//! ```
//!
//! ## Accessing Information Elements
//!
//! [`Bss`] contains a list of 802.11 Information Elements (IEs) that are
//! accessed through [`Bss::ies`].
//!
//! ```
//! # fn print_ie_count(bss: &kawaiifi::Bss) {
//! use kawaiifi::Ie;
//!
//! let ies: &[Ie] = bss.ies();
//! println!("Found {} IE(s)", ies.len());
//! # }
//! ```
//!
//! [`Ie`] exposes basic properties such as the information element's name and
//! ID.
//!
//! ```
//! # fn print_ie(ie: &kawaiifi::Ie) {
//! println!("IE: {} ({})", ie.name(), ie.id);
//! # }
//! ```
//!
//! [`Ie`] also exposes the information element's underlying data through
//! [`Ie::data`].
//!
//! ```
//! use kawaiifi::IeData;
//!
//! # fn print_ie_data(ie: &kawaiifi::Ie) {
//! match &ie.data {
//!     IeData::Ssid(ssid) => println!("SSID: {}", ssid.to_string_lossy()),
//!     IeData::DsParameterSet(ds) => println!("Channel: {}", ds.current_channel),
//!     IeData::Tim(tim) => println!("DTIM Period: {}", tim.dtim_period),
//!     IeData::VhtCapabilities(vht_caps) => {
//!         println!("Max MPDU Length: {}", vht_caps.vht_capabilities_info.maximum_mpdu_length)
//!     }
//!     _ => {}
//! }
//! # }
//! ```

mod band;
mod bss;
mod channel_width;
pub mod ies;
mod interface;
#[cfg(target_os = "linux")]
mod nl80211;
mod scan;
mod security_protocol;
mod wifi_amendment;
mod wifi_protocol;

pub use band::Band;
pub use bss::Bss;
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub use bss::CapabilityInfo;
pub use channel_width::{ChannelWidth, ChannelWidths};
pub use ies::{Ie, IeData};
#[cfg(target_os = "linux")]
pub use interface::BusType;
pub use interface::{Interface, default_interface, interfaces};
#[cfg(target_os = "linux")]
pub use nl80211::BssStatus;
#[cfg(target_os = "linux")]
pub use scan::Backend;
pub use scan::Error as ScanError;
#[cfg(target_os = "linux")]
pub use scan::Flags as ScanFlags;
pub use scan::Scan;
pub use security_protocol::{SecurityProtocol, SecurityProtocols};
pub use wifi_amendment::{WifiAmendment, WifiAmendments};
pub use wifi_protocol::{WifiProtocol, WifiProtocols};
