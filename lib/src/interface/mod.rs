mod error;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

pub use error::Error;

#[cfg(target_os = "linux")]
pub use linux::{BusType, Interface};

#[cfg(target_os = "macos")]
pub use macos::Interface;

#[cfg(target_os = "macos")]
pub(crate) use macos::parse_bssid;

#[cfg(target_os = "windows")]
pub use windows::Interface;

/// Returns the first available Wi-Fi interface.
///
/// The interface is selected from the platform's enumeration order. That order
/// is platform-defined and is not guaranteed to remain stable.
///
/// Returns `Ok(Some(interface))` when an interface is available and `Ok(None)`
/// when enumeration succeeds but no Wi-Fi interfaces are found.
///
/// # Errors
///
/// Returns an [`InterfaceError`](crate::InterfaceError) if the system's Wi-Fi
/// interfaces cannot be enumerated. Currently, enumeration errors are reported
/// on Linux; the macOS and Windows implementations return `Ok`.
///
/// # Examples
///
/// ```
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// match kawaiifi::default_interface()? {
///     Some(interface) => println!("Found interface: {interface:?}"),
///     None => println!("No Wi-Fi interfaces found"),
/// }
/// # Ok(())
/// # }
/// ```
pub fn default_interface() -> Result<Option<Interface>, crate::InterfaceError> {
    Ok(interfaces()?.into_iter().next())
}

/// Returns all available Wi-Fi interfaces on the system.
///
/// An empty vector indicates that enumeration succeeded but no Wi-Fi
/// interfaces were found. The order of the returned interfaces is
/// platform-defined and is not guaranteed to remain stable.
///
/// # Errors
///
/// Returns an [`InterfaceError`](crate::InterfaceError) if the system's Wi-Fi
/// interfaces cannot be enumerated. Currently, enumeration errors are reported
/// on Linux; the macOS and Windows implementations return `Ok`.
///
/// # Examples
///
/// ```
/// # fn example() -> Result<(), kawaiifi::InterfaceError> {
/// let interfaces = kawaiifi::interfaces()?;
/// println!("Found {} Wi-Fi interface(s)", interfaces.len());
/// # Ok(())
/// # }
/// ```
pub fn interfaces() -> Result<Vec<Interface>, crate::InterfaceError> {
    #[cfg(target_os = "linux")]
    {
        linux::interfaces()
    }
    #[cfg(target_os = "macos")]
    {
        Ok(macos::interfaces())
    }
    #[cfg(target_os = "windows")]
    {
        Ok(windows::interfaces())
    }
}
