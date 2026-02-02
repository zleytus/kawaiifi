use deku::prelude::*;
use serde::{Deserialize, Serialize};

use super::IeId;
use crate::{BitRange, Field};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(bit_order = "lsb")]
pub struct RmEnabledCapabilities {
    #[deku(bits = 1)]
    pub link_measurement_capability_enabled: bool,
    #[deku(bits = 1)]
    pub neighbor_report_capability_enabled: bool,
    #[deku(bits = 1)]
    pub parallel_measurements_capability_enabled: bool,
    #[deku(bits = 1)]
    pub repeated_measurements_capability_enabled: bool,
    #[deku(bits = 1)]
    pub beacon_passive_measurement_capability_enabled: bool,
    #[deku(bits = 1)]
    pub beacon_active_measurement_capability_enabled: bool,
    #[deku(bits = 1)]
    pub beacon_table_measurement_capability_enabled: bool,
    #[deku(bits = 1)]
    pub beacon_measurement_reporting_conditions_capability_enabled: bool,
    #[deku(bits = 1)]
    pub frame_measurement_capability_enabled: bool,
    #[deku(bits = 1)]
    pub channel_load_measurement_capability_enabled: bool,
    #[deku(bits = 1)]
    pub noise_histogram_measurement_capability_enabled: bool,
    #[deku(bits = 1)]
    pub statistics_measurement_capability_enabled: bool,
    #[deku(bits = 1)]
    pub lci_measurement_capability_enabled: bool,
    #[deku(bits = 1)]
    pub lci_azimuth_capability_enabled: bool,
    #[deku(bits = 1)]
    pub transmit_stream_category_measurement_capability_enabled: bool,
    #[deku(bits = 1)]
    pub triggered_transmit_stream_category_measurement_capability_enabled: bool,
    #[deku(bits = 1)]
    pub ap_channel_report_capability_enabled: bool,
    #[deku(bits = 1)]
    pub rm_mib_capability_enabled: bool,
    #[deku(bits = 3)]
    pub operating_channel_max_measurement_duration: u8,
    #[deku(bits = 3)]
    pub nonoperating_channel_max_measurement_duration: u8,
    #[deku(bits = 3)]
    pub measurement_pilot_capability: u8,
    #[deku(bits = 1)]
    pub measurement_pilot_transmission_information_capability_enabled: bool,
    #[deku(bits = 1)]
    pub neighbor_report_tsf_offset_capability_enabled: bool,
    #[deku(bits = 1)]
    pub rcpi_measurement_capability_enabled: bool,
    #[deku(bits = 1)]
    pub rsni_measurement_capability_enabled: bool,
    #[deku(bits = 1)]
    pub bss_average_access_delay_capability_enabled: bool,
    #[deku(bits = 1)]
    pub bss_available_admission_capacity_capability_enabled: bool,
    #[deku(bits = 1)]
    pub antenna_capability_enabled: bool,
    #[deku(bits = 1)]
    pub ftm_range_report_capability_enabled: bool,
    #[deku(bits = 1)]
    pub civic_location_measurement_capability_enabled: bool,
    #[deku(bits = 4)]
    reserved: u8,
}

impl RmEnabledCapabilities {
    pub const NAME: &'static str = "RM Enabled Capabilities";
    pub const ID: u8 = 70;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const MIN_LENGTH: usize = 5;

    pub fn summary(&self) -> String {
        let mut summary = Vec::new();
        if self.link_measurement_capability_enabled {
            summary.push("Link Measurement");
        }

        if self.neighbor_report_capability_enabled {
            summary.push("Neighbor Report");
        }

        if self.parallel_measurements_capability_enabled {
            summary.push("Parallel Measurements");
        }

        if self.repeated_measurements_capability_enabled {
            summary.push("Repeated Measurements");
        }

        if self.beacon_passive_measurement_capability_enabled {
            summary.push("Beacon Passive Measurement");
        }

        if self.beacon_active_measurement_capability_enabled {
            summary.push("Beacon Active Measurement");
        }

        if self.beacon_table_measurement_capability_enabled {
            summary.push("Beacon Table Measurement");
        }

        summary.join(", ")
    }

    pub fn fields(&self) -> Vec<Field> {
        let bytes = self.to_bytes().unwrap_or_default();

        vec![
            Field::builder()
                .title("Link Measurement")
                .value(self.link_measurement_capability_enabled)
                .bits(BitRange::new(&bytes, 0, 1))
                .build(),
            Field::builder()
                .title("Neighbor Report")
                .value(self.neighbor_report_capability_enabled)
                .bits(BitRange::new(&bytes, 1, 1))
                .build(),
            Field::builder()
                .title("Parallel Measurements")
                .value(self.parallel_measurements_capability_enabled)
                .bits(BitRange::new(&bytes, 2, 1))
                .build(),
            Field::builder()
                .title("Repeated Measurements")
                .value(self.repeated_measurements_capability_enabled)
                .bits(BitRange::new(&bytes, 3, 1))
                .build(),
            Field::builder()
                .title("Beacon Passive Measurement")
                .value(self.beacon_passive_measurement_capability_enabled)
                .bits(BitRange::new(&bytes, 4, 1))
                .build(),
            Field::builder()
                .title("Beacon Active Measurement")
                .value(self.beacon_active_measurement_capability_enabled)
                .bits(BitRange::new(&bytes, 5, 1))
                .build(),
            Field::builder()
                .title("Beacon Table Measurement")
                .value(self.beacon_table_measurement_capability_enabled)
                .bits(BitRange::new(&bytes, 6, 1))
                .build(),
            Field::builder()
                .title("Beacon Measurement Reporting Conditions")
                .value(self.beacon_measurement_reporting_conditions_capability_enabled)
                .bits(BitRange::new(&bytes, 7, 1))
                .build(),
            Field::builder()
                .title("Frame Measurement")
                .value(self.frame_measurement_capability_enabled)
                .bits(BitRange::new(&bytes, 8, 1))
                .build(),
            Field::builder()
                .title("Channel Load Measurement")
                .value(self.channel_load_measurement_capability_enabled)
                .bits(BitRange::new(&bytes, 9, 1))
                .build(),
            Field::builder()
                .title("Noise Histogram Measurement")
                .value(self.noise_histogram_measurement_capability_enabled)
                .bits(BitRange::new(&bytes, 10, 1))
                .build(),
            Field::builder()
                .title("Statistics Measurement")
                .value(self.statistics_measurement_capability_enabled)
                .bits(BitRange::new(&bytes, 11, 1))
                .build(),
            Field::builder()
                .title("LCI Measurement")
                .value(self.lci_measurement_capability_enabled)
                .bits(BitRange::new(&bytes, 12, 1))
                .build(),
            Field::builder()
                .title("LCI Azimuth")
                .value(self.lci_azimuth_capability_enabled)
                .bits(BitRange::new(&bytes, 13, 1))
                .build(),
            Field::builder()
                .title("Transmit Stream/Category Measurement")
                .value(self.transmit_stream_category_measurement_capability_enabled)
                .bits(BitRange::new(&bytes, 14, 1))
                .build(),
            Field::builder()
                .title("Triggered Transmit Stream/Category Measurement")
                .value(self.triggered_transmit_stream_category_measurement_capability_enabled)
                .bits(BitRange::new(&bytes, 15, 1))
                .build(),
            Field::builder()
                .title("AP Channel Report")
                .value(self.ap_channel_report_capability_enabled)
                .bits(BitRange::new(&bytes, 16, 1))
                .build(),
            Field::builder()
                .title("RM MIB")
                .value(self.rm_mib_capability_enabled)
                .bits(BitRange::new(&bytes, 17, 1))
                .build(),
            Field::builder()
                .title("Operating Channel Max Measurement Duration")
                .value(self.operating_channel_max_measurement_duration)
                .bits(BitRange::new(&bytes, 18, 3))
                .build(),
            Field::builder()
                .title("Nonoperating Channel Max Measurement Duration")
                .value(self.nonoperating_channel_max_measurement_duration)
                .bits(BitRange::new(&bytes, 21, 3))
                .build(),
            Field::builder()
                .title("Measurement Pilot Capability")
                .value(self.measurement_pilot_capability)
                .bits(BitRange::new(&bytes, 24, 3))
                .build(),
            Field::builder()
                .title("Measurement Pilot Transmission Information")
                .value(self.measurement_pilot_transmission_information_capability_enabled)
                .bits(BitRange::new(&bytes, 27, 1))
                .build(),
            Field::builder()
                .title("Neighbor Report TSF Offset")
                .value(self.neighbor_report_tsf_offset_capability_enabled)
                .bits(BitRange::new(&bytes, 28, 1))
                .build(),
            Field::builder()
                .title("RCPI Measurement")
                .value(self.rcpi_measurement_capability_enabled)
                .bits(BitRange::new(&bytes, 29, 1))
                .build(),
            Field::builder()
                .title("RSNI Measurement")
                .value(self.rsni_measurement_capability_enabled)
                .bits(BitRange::new(&bytes, 30, 1))
                .build(),
            Field::builder()
                .title("BSS Average Access Delay")
                .value(self.bss_average_access_delay_capability_enabled)
                .bits(BitRange::new(&bytes, 31, 1))
                .build(),
            Field::builder()
                .title("BSS Available Admission Capacity")
                .value(self.bss_available_admission_capacity_capability_enabled)
                .bits(BitRange::new(&bytes, 32, 1))
                .build(),
            Field::builder()
                .title("Antenna")
                .value(self.antenna_capability_enabled)
                .bits(BitRange::new(&bytes, 33, 1))
                .build(),
            Field::builder()
                .title("FTM Range Report")
                .value(self.ftm_range_report_capability_enabled)
                .bits(BitRange::new(&bytes, 34, 1))
                .build(),
            Field::builder()
                .title("Civic Location Measurement")
                .value(self.civic_location_measurement_capability_enabled)
                .bits(BitRange::new(&bytes, 35, 1))
                .build(),
            Field::reserved(BitRange::new(&bytes, 36, 4)),
        ]
    }
}
