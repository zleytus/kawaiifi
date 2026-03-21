#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::{BusType, Interface};

#[cfg(target_os = "windows")]
pub use windows::Interface;

pub fn default_interface() -> Option<Interface> {
    interfaces().into_iter().next()
}

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

#[cfg(target_os = "windows")]
pub fn interfaces() -> Vec<Interface> {
    windows::interfaces()
}
