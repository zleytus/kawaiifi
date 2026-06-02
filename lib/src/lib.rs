//! Cross-platform Wi-Fi scanning and monitoring.
//!
//! `kawaiifi` provides a common Rust API for discovering nearby Wi-Fi Basic
//! Service Sets (BSSs) on Linux, macOS, and Windows. Scan results expose common
//! properties such as SSID, BSSID, signal strength, channel, channel width,
//! security protocols, and parsed Information Elements (IEs).
//!
//! The crate is split into a small high-level scan API and lower-level parsed
//! 802.11 data types. Most applications can start with [`default_interface`],
//! [`Interface`], [`Scan`], and [`Bss`]. The [`ies`] module exposes parsed
//! Information Elements for callers that need to inspect management-frame
//! details directly.
//!
//! Platform scan APIs differ slightly. Linux supports multiple scan backends
//! through [`scan::Backend`], while macOS and Windows use the platform-native
//! CoreWLAN and Native WiFi APIs directly.
//!
//! # Linux
//!
//! On Linux, choose the backend that should trigger the scan. NetworkManager is
//! usually the right choice for desktop applications.
//!
//! ```ignore
//! use std::error::Error;
//!
//! use kawaiifi::scan::Backend;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let interface = kawaiifi::default_interface().ok_or("No Wi-Fi interface found")?;
//!     let scan = interface.scan_blocking(Backend::NetworkManager)?;
//!
//!     println!("Found {} BSS(s)", scan.bss_list().len());
//!     Ok(())
//! }
//! ```
//!
//! # Windows/macOS
//!
//! On Windows and macOS, scans use the platform's native Wi-Fi API directly.
//!
//! ```ignore
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let interface = kawaiifi::default_interface().ok_or("No Wi-Fi interface found")?;
//!     let scan = interface.scan_blocking()?;
//!
//!     println!("Found {} BSS(s)", scan.bss_list().len());
//!     Ok(())
//! }
//! ```
//!
//! # Async scans
//!
//! Each platform also exposes an async `scan` method. On Linux it accepts the
//! same [`scan::Backend`] argument as [`Interface::scan_blocking`].

mod band;
mod bss;
mod channel_width;
pub mod ies;
mod interface;
#[cfg(target_os = "linux")]
mod nl80211;
pub mod scan;
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
pub use scan::Scan;
pub use security_protocol::{SecurityProtocol, SecurityProtocols};
pub use wifi_amendment::{WifiAmendment, WifiAmendments};
pub use wifi_protocol::{WifiProtocol, WifiProtocols};
