use deku::{DekuRead, DekuWrite};

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
pub struct OverlappingBssScanParams {
    #[deku(bytes = 2)]
    pub obss_scan_passive_dwell_tu: u16,
    #[deku(bytes = 2)]
    pub obss_scan_active_dwell_tu: u16,
    #[deku(bytes = 2)]
    pub bss_channel_width_trigger_scan_interval_secs: u16,
    #[deku(bytes = 2)]
    pub obss_scan_passive_total_per_channel_tu: u16,
    #[deku(bytes = 2)]
    pub obss_scan_active_total_per_channel_tu: u16,
    #[deku(bytes = 2)]
    pub bss_width_channel_transition_delay_factor: u16,
    #[deku(bytes = 2)]
    pub obss_scan_activity_threshold: u16,
}

impl OverlappingBssScanParams {
    pub const NAME: &'static str = "Overlapping BSS Scan Parameters";
    pub const ID: u8 = 74;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const LENGTH: usize = 14;
}
