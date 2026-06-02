use objc2_foundation::NSError;
use thiserror::Error;

/// Errors that can occur while scanning for Wi-Fi networks on macOS.
#[derive(Error, Debug)]
pub enum Error {
    /// A CoreWLAN operation failed.
    #[error("CoreWLAN error: {0}")]
    CoreWlan(String),

    /// An I/O operation failed.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

impl From<objc2::rc::Retained<NSError>> for Error {
    fn from(error: objc2::rc::Retained<NSError>) -> Self {
        Self::CoreWlan(error.to_string())
    }
}
