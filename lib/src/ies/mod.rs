//! Wi-Fi Information Element (IE) parsing and serialization.
//!
//! Information Elements are the building blocks of Wi-Fi management frames
//! (beacons, probe responses, etc.). Each IE contains specific information
//! about an access point or station's capabilities and configuration.
//!
//! # Parsing IEs
//!
//! Use [`from_bytes`] to parse a sequence of IEs from raw bytes.
//! Returns a `Vec<Ie>` containing all successfully parsed IEs:
//!
//! ```
//! # use kawaiifi::ies;
//! // Two IEs: SSID "Hello" + DS Parameter Set (channel 6)
//! let ie_bytes = &[
//!     0x00, 0x05, 0x48, 0x65, 0x6c, 0x6c, 0x6f,  // SSID IE
//!     0x03, 0x01, 0x06,                           // DS Parameter Set IE
//! ];
//! let ies = ies::from_bytes(ie_bytes);
//! assert_eq!(ies.len(), 2);
//! ```
//!
//! # Accessing IE Data
//!
//! Each [`Ie`] has `id`, `id_ext`, and `data` fields.
//! Use the `name()` method to get a human-readable IE name:
//!
//! ```
//! # use kawaiifi::ies::{self, IeData};
//! # let ie_bytes = &[
//! #     0x00, 0x05, 0x48, 0x65, 0x6c, 0x6c, 0x6f,
//! #     0x03, 0x01, 0x06,
//! # ];
//! # let ies = ies::from_bytes(ie_bytes);
//! for ie in &ies {
//!     println!("IE: {} (id={}, id_ext={:?})", ie.name(), ie.id, ie.id_ext);
//!
//!     match &ie.data {
//!         IeData::Ssid(ssid) => println!("  SSID: {}", ssid.to_string_lossy()),
//!         IeData::DsParameterSet(ds) => println!("  Channel: {}", ds.current_channel),
//!         _ => {}
//!     }
//! }
//! ```

mod elements;
mod field;
mod ie;
mod ie_data;
mod ie_id;
mod parse;
pub mod serde_raw;
mod write;

pub use elements::*;
pub use field::{BitRange, Field};
pub use ie::Ie;
pub use ie_data::IeData;
pub use ie_id::IeId;
pub use parse::from_bytes;
pub(crate) use write::write_bits_lsb0;
