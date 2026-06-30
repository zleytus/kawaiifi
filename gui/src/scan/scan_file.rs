use kawaiifi::Bss;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

use gtk::{
    gio::{self, prelude::FileExtManual},
    glib,
};

use crate::objects::BssInternal;

const CURRENT_VERSION: u32 = 1;
const CURRENT_PLATFORM: &str = "linux";

#[derive(Debug)]
pub enum ScanFileError {
    File(glib::Error),
    Json(serde_json::Error),
    UnsupportedVersion(u32),
    UnsupportedPlatform(String),
}

impl fmt::Display for ScanFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::File(err) => write!(f, "{err}"),
            Self::Json(err) => write!(f, "{err}"),
            Self::UnsupportedVersion(version) => {
                write!(f, "Unsupported scan file version: {version}")
            }
            Self::UnsupportedPlatform(platform) => {
                write!(f, "Unsupported scan file platform: {platform}")
            }
        }
    }
}

impl Error for ScanFileError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::File(err) => Some(err),
            Self::Json(err) => Some(err),
            Self::UnsupportedVersion(_) | Self::UnsupportedPlatform(_) => None,
        }
    }
}

impl From<serde_json::Error> for ScanFileError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

#[derive(Serialize, Deserialize)]
struct ScanFileBss {
    bss: Bss,
    vendor: Option<String>,
}

/// A serializable snapshot of scan results, used for saving and loading scans to/from disk.
#[derive(Serialize, Deserialize)]
pub struct ScanFile {
    /// Format version, for future compatibility.
    version: u32,
    /// Platform the scan was captured on (e.g. "linux").
    platform: String,
    bss_list: Vec<ScanFileBss>,
}

impl ScanFile {
    pub fn new(bss_list: Vec<BssInternal>) -> Self {
        Self {
            version: CURRENT_VERSION,
            platform: CURRENT_PLATFORM.to_string(),
            bss_list: bss_list
                .iter()
                .map(|bss: &BssInternal| ScanFileBss {
                    bss: Bss::clone(bss),
                    vendor: bss.vendor().map(str::to_string),
                })
                .collect(),
        }
    }

    pub fn bss_list(&self) -> Vec<BssInternal> {
        self.bss_list
            .iter()
            .map(|bss| {
                let mut bss_internal = BssInternal::new(bss.bss.clone());
                // Prefer a freshly resolved vendor; saved vendors are only used when lookup
                // cannot resolve the BSS
                if let Some(vendor) = &bss.vendor
                    && bss_internal.vendor().is_none()
                {
                    bss_internal.set_vendor(vendor.to_string());
                }
                bss_internal
            })
            .collect()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, ScanFileError> {
        let scan_file: Self = serde_json::from_slice(bytes)?;
        if scan_file.version != CURRENT_VERSION {
            return Err(ScanFileError::UnsupportedVersion(scan_file.version));
        }
        if scan_file.platform != CURRENT_PLATFORM {
            return Err(ScanFileError::UnsupportedPlatform(scan_file.platform));
        }
        Ok(scan_file)
    }

    pub async fn open_bss_list(file: &gio::File) -> Result<Vec<BssInternal>, ScanFileError> {
        let (contents, _) = file
            .load_contents_future()
            .await
            .map_err(ScanFileError::File)?;
        gtk::gio::spawn_blocking(move || {
            let scan_file = ScanFile::from_bytes(&contents)?;
            Ok(scan_file.bss_list())
        })
        .await
        .expect("Scan file worker panicked")
    }

    pub async fn save(&self, file: &gio::File) -> Result<(), ScanFileError> {
        let json = serde_json::to_vec_pretty(self)?;
        file.replace_contents_future(json, None, false, gio::FileCreateFlags::REPLACE_DESTINATION)
            .await
            .map(|_| ())
            .map_err(|(_, err)| ScanFileError::File(err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_json_accepts_current_format() {
        let scan_file = ScanFile::from_bytes(
            r#"{
                "version": 1,
                "platform": "linux",
                "bss_list": []
            }"#
            .as_bytes(),
        )
        .unwrap();

        assert!(scan_file.bss_list().is_empty());
    }

    #[test]
    fn from_json_rejects_unsupported_version() {
        let result = ScanFile::from_bytes(
            r#"{
                "version": 2,
                "platform": "linux",
                "bss_list": []
            }"#
            .as_bytes(),
        );

        assert!(matches!(result, Err(ScanFileError::UnsupportedVersion(2))));
    }

    #[test]
    fn from_json_rejects_unsupported_platform() {
        let result = ScanFile::from_bytes(
            r#"{
                "version": 1,
                "platform": "windows",
                "bss_list": []
            }"#
            .as_bytes(),
        );

        assert!(matches!(
            result,
            Err(ScanFileError::UnsupportedPlatform(platform)) if platform == "windows"
        ));
    }

    #[test]
    fn from_json_reports_malformed_json() {
        let result = ScanFile::from_bytes(b"not json");

        assert!(matches!(result, Err(ScanFileError::Json(_))));
    }
}
