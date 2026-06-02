use thiserror::Error;

/// Errors that can occur while scanning for Wi-Fi networks on Windows.
#[derive(Error, Debug)]
pub enum Error {
    /// The scan operation was denied by the operating system.
    #[error("Permission denied")]
    PermissionDenied,

    /// An I/O operation failed.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
