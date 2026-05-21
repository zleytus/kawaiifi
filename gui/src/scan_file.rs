use kawaiifi::Bss;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

const CURRENT_VERSION: u32 = 1;
const CURRENT_PLATFORM: &str = "linux";

#[derive(Debug)]
pub enum ScanFileError {
    Json(serde_json::Error),
    UnsupportedVersion(u32),
    UnsupportedPlatform(String),
}

impl fmt::Display for ScanFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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
            version: CURRENT_VERSION,
            platform: CURRENT_PLATFORM.to_string(),
            bss_list,
        }
    }

    pub fn bss_list(&self) -> &[Bss] {
        &self.bss_list
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json: &str) -> Result<Self, ScanFileError> {
        let scan_file: Self = serde_json::from_str(json)?;
        if scan_file.version != CURRENT_VERSION {
            return Err(ScanFileError::UnsupportedVersion(scan_file.version));
        }
        if scan_file.platform != CURRENT_PLATFORM {
            return Err(ScanFileError::UnsupportedPlatform(scan_file.platform));
        }
        Ok(scan_file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_json_accepts_current_format() {
        let scan_file = ScanFile::from_json(
            r#"{
                "version": 1,
                "platform": "linux",
                "bss_list": []
            }"#,
        )
        .unwrap();

        assert!(scan_file.bss_list().is_empty());
    }

    #[test]
    fn from_json_rejects_unsupported_version() {
        let result = ScanFile::from_json(
            r#"{
                "version": 2,
                "platform": "linux",
                "bss_list": []
            }"#,
        );

        assert!(matches!(result, Err(ScanFileError::UnsupportedVersion(2))));
    }

    #[test]
    fn from_json_rejects_unsupported_platform() {
        let result = ScanFile::from_json(
            r#"{
                "version": 1,
                "platform": "windows",
                "bss_list": []
            }"#,
        );

        assert!(matches!(
            result,
            Err(ScanFileError::UnsupportedPlatform(platform)) if platform == "windows"
        ));
    }

    #[test]
    fn from_json_reports_malformed_json() {
        let result = ScanFile::from_json("not json");

        assert!(matches!(result, Err(ScanFileError::Json(_))));
    }
}
