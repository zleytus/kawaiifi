use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use crate::{Field, ies::IeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
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

    pub fn summary(&self) -> String {
        "".to_string()
    }

    pub fn fields(&self) -> Vec<Field> {
        vec![
            Field::builder()
                .title("OBSS Scan Passive Dwell")
                .value(self.obss_scan_passive_dwell_tu)
                .units("TU")
                .bytes(self.obss_scan_passive_dwell_tu.to_le_bytes().to_vec())
                .build(),
            Field::builder()
                .title("OBSS Scan Active Dwell")
                .value(self.obss_scan_active_dwell_tu)
                .units("TU")
                .bytes(self.obss_scan_active_dwell_tu.to_le_bytes().to_vec())
                .build(),
            Field::builder()
                .title("BSS Channel Width Trigger Scan Interval")
                .value(self.bss_channel_width_trigger_scan_interval_secs)
                .units("seconds")
                .bytes(
                    self.bss_channel_width_trigger_scan_interval_secs
                        .to_le_bytes()
                        .to_vec(),
                )
                .build(),
            Field::builder()
                .title("OBSS Scan Passive Total Per Channel")
                .value(self.obss_scan_passive_total_per_channel_tu)
                .units("TU")
                .bytes(
                    self.obss_scan_passive_total_per_channel_tu
                        .to_le_bytes()
                        .to_vec(),
                )
                .build(),
            Field::builder()
                .title("OBSS Scan Active Total Per Channel")
                .value(self.obss_scan_active_total_per_channel_tu)
                .units("TU")
                .bytes(
                    self.obss_scan_active_total_per_channel_tu
                        .to_le_bytes()
                        .to_vec(),
                )
                .build(),
            Field::builder()
                .title("BSS Width Channel Transition Delay Factor")
                .value(self.bss_width_channel_transition_delay_factor)
                .bytes(
                    self.bss_width_channel_transition_delay_factor
                        .to_le_bytes()
                        .to_vec(),
                )
                .build(),
            Field::builder()
                .title("OBSS Scan Activity Threshold")
                .value(
                    format!(
                        "{:.2}%",
                        f64::from(self.obss_scan_activity_threshold) / 100.0
                    )
                    .trim_end_matches("0")
                    .trim_end_matches("."),
                )
                .units(format!("({})", self.obss_scan_activity_threshold))
                .bytes(self.obss_scan_activity_threshold.to_le_bytes().to_vec())
                .build(),
        ]
    }
}
