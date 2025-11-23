use std::fmt::Display;

use deku::prelude::*;

use crate::Field;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(bit_order = "lsb")]
pub struct CapabilityInfo {
    #[deku(bits = 1)]
    pub ess: bool,
    #[deku(bits = 1)]
    pub ibss: bool,
    #[deku(bits = 1)]
    reserved_1: bool,
    #[deku(bits = 1)]
    reserved_2: bool,
    #[deku(bits = 1)]
    pub privacy: bool,
    #[deku(bits = 1)]
    pub short_preamble: bool,
    #[deku(bits = 1)]
    pub critical_update_flag: bool,
    #[deku(bits = 1)]
    pub nontransmitted_bssids_critical_update_flag: bool,
    #[deku(bits = 1)]
    pub spectrum_management: bool,
    #[deku(bits = 1)]
    pub qos: bool,
    #[deku(bits = 1)]
    pub short_slot_time: bool,
    #[deku(bits = 1)]
    pub apsd: bool,
    #[deku(bits = 1)]
    pub radio_measurement: bool,
    #[deku(bits = 1)]
    pub epd: bool,
    #[deku(bits = 1)]
    reserved_3: bool,
    #[deku(bits = 1)]
    reserved_4: bool,
}

impl CapabilityInfo {
    pub const NAME: &'static str = "Capability Information";
    pub const LENGTH: usize = 2;

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
