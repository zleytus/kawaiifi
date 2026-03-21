use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::Bss;

/// Results of a WiFi scan operation.
///
/// A `Scan` contains all Basic Service Sets (BSSs) discovered during a scan, along with
/// the start and end time of the scan.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Scan {
    bss_list: Vec<Bss>,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
}

impl Scan {
    pub(crate) fn new(bss_list: Vec<Bss>, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> Self {
        Self { bss_list, start_time, end_time }
    }

    /// Returns all BSSs (access points) discovered during the scan.
    ///
    /// If multiple sub-scans were performed, this list contains the combined results
    /// from all sub-scans.
    pub fn bss_list(&self) -> &[Bss] {
        &self.bss_list
    }

    /// The time at which the scan started.
    pub fn start_time(&self) -> DateTime<Utc> {
        self.start_time
    }

    /// The time at which the scan ended.
    pub fn end_time(&self) -> DateTime<Utc> {
        self.end_time
    }

    /// The duration of the scan.
    pub fn duration(&self) -> std::time::Duration {
        (self.end_time() - self.start_time())
            .to_std()
            .unwrap_or_default()
    }
}
