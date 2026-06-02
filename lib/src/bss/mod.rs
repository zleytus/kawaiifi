mod basic_service_set;
#[cfg(any(target_os = "linux", target_os = "windows"))]
mod capability_info;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

pub use basic_service_set::Bss;
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub use capability_info::CapabilityInfo;
