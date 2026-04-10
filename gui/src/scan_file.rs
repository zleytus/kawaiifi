use kawaiifi::Bss;
use serde::{Deserialize, Serialize};

/// A serializable snapshot of scan results, used for saving and loading scans to/from disk.
#[derive(Serialize, Deserialize)]
pub struct ScanFile {
    /// Format version, for future compatibility.
    version: u32,
    /// Platform the scan was captured on (e.g. "linux").
    platform: String,
    bss_list: Vec<Bss>,
}

impl ScanFile {
    pub fn new(bss_list: Vec<Bss>) -> Self {
        Self {
            version: 1,
            platform: "linux".to_string(),
            bss_list,
        }
    }

    pub fn bss_list(&self) -> &[Bss] {
        &self.bss_list
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }
}
