mod processing;
mod scan_file;

pub use processing::{ProcessedScan, spawn_scan_processing};
pub use scan_file::{ScanFile, ScanFileError};
