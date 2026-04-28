#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::{BusType, Interface};

#[cfg(target_os = "windows")]
pub use windows::Interface;

/// Returns the first available Wi-Fi interface, or `None` if no interfaces are found.
pub fn default_interface() -> Option<Interface> {
    interfaces().into_iter().next()
}

/// Returns all available Wi-Fi interfaces on the system.
#[cfg(target_os = "linux")]
pub fn interfaces() -> Vec<Interface> {
    match linux::interfaces() {
        Ok(interfaces) => interfaces,
        Err(e) => {
            eprintln!("Failed to get interfaces: {:?}", e);
            Vec::new()
        }
    }
}

/// Returns all available Wi-Fi interfaces on the system.
#[cfg(target_os = "windows")]
pub fn interfaces() -> Vec<Interface> {
    windows::interfaces()
}
