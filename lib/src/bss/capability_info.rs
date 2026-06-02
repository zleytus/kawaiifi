use std::fmt::Display;

use deku::prelude::*;
use serde::{Deserialize, Serialize};

use crate::ies::Field;

/// The 802.11 capability information flags advertised in beacon and probe response frames.
#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct CapabilityInfo {
    /// Set by an AP (1) or cleared by an IBSS or mesh STA (0).
    #[deku(bits = 1)]
    pub ess: bool,
    /// Set by an IBSS STA (1) or cleared by an AP or mesh STA (0).
    #[deku(bits = 1)]
    pub ibss: bool,
    #[deku(bits = 1)]
    reserved_1: bool,
    #[deku(bits = 1)]
    reserved_2: bool,
    /// Indicates data confidentiality is required for all Data frames exchanged within the BSS.
    #[deku(bits = 1)]
    pub privacy: bool,
    /// Indicates use of the short preamble is allowed within the BSS.
    #[deku(bits = 1)]
    pub short_preamble: bool,
    /// Set by an AP affiliated with an AP MLD to signal a critical update is pending. Reserved in other contexts.
    #[deku(bits = 1)]
    pub critical_update_flag: bool,
    /// Set by a transmitted-BSSID AP affiliated with an AP MLD if any nontransmitted BSS in its multiple BSSID set has a critical update pending. Reserved in other contexts.
    #[deku(bits = 1)]
    pub nontransmitted_bssids_critical_update_flag: bool,
    /// Indicates the STA implements spectrum management.
    #[deku(bits = 1)]
    pub spectrum_management: bool,
    /// Indicates the STA implements QoS.
    #[deku(bits = 1)]
    pub qos: bool,
    /// Indicates the BSS is currently using the short slot time. Always 0 for IBSS and mesh.
    #[deku(bits = 1)]
    pub short_slot_time: bool,
    /// Set by an AP to indicate Automatic Power Save Delivery (APSD) support. Always 0 for non-AP STAs.
    #[deku(bits = 1)]
    pub apsd: bool,
    /// Indicates the STA supports radio measurement.
    #[deku(bits = 1)]
    pub radio_measurement: bool,
    /// Indicates the STA implements EPD.
    #[deku(bits = 1)]
    pub epd: bool,
    #[deku(bits = 1)]
    reserved_3: bool,
    #[deku(bits = 1)]
    reserved_4: bool,
}

impl CapabilityInfo {
    /// The human-readable field name.
    pub const NAME: &'static str = "Capability Information";
    /// The encoded length, in bytes.
    pub const LENGTH: usize = 2;

    /// The capability flags as displayable fields.
    pub fn fields(&self) -> Vec<Field> {
        vec![
            Field::new("ESS", self.ess),
            Field::new("IBSS", self.ibss),
            Field::new("Privacy", self.privacy),
            Field::new("Short Preamble", self.short_preamble),
            Field::new("Critical Update Flag", self.critical_update_flag),
            Field::new(
                "Nontransmitted BSSIDs Critical Update Flag",
                self.nontransmitted_bssids_critical_update_flag,
            ),
            Field::new("Spectrum Management", self.spectrum_management),
            Field::new("QoS", self.qos),
            Field::new("Short Slot Time", self.short_slot_time),
            Field::new("APSD", self.apsd),
            Field::new("Radio Measurement", self.radio_measurement),
            Field::new("EPD", self.epd),
        ]
    }
}

impl Display for CapabilityInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut capabilities = Vec::new();

        if self.ess {
            capabilities.push("ESS");
        }

        if self.ibss {
            capabilities.push("IBSS");
        }

        if self.privacy {
            capabilities.push("Privacy");
        }

        if self.short_preamble {
            capabilities.push("Short Preamble");
        }

        if self.critical_update_flag {
            capabilities.push("Critical Update Flag");
        }

        if self.nontransmitted_bssids_critical_update_flag {
            capabilities.push("Nontransmitted BSSIDs Critical Update Flag");
        }

        if self.spectrum_management {
            capabilities.push("Spectrum Management");
        }

        if self.qos {
            capabilities.push("QoS");
        }

        if self.short_slot_time {
            capabilities.push("Short Slot Time");
        }

        if self.apsd {
            capabilities.push("APSD");
        }

        if self.radio_measurement {
            capabilities.push("Radio Measurement");
        }

        if self.epd {
            capabilities.push("EPD");
        }

        write!(f, "Capability Info:\n\t{}", capabilities.join("\r\n\t"))
    }
}
