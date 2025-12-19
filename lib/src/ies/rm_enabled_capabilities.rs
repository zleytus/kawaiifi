use deku::prelude::*;
use serde::{Deserialize, Serialize};

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite, Serialize, Deserialize)]
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
}
