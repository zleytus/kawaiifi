use objc2_foundation::NSError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("CoreWLAN error: {0}")]
    CoreWlan(String),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

impl From<objc2::rc::Retained<NSError>> for Error {
    fn from(error: objc2::rc::Retained<NSError>) -> Self {
        Self::CoreWlan(error.to_string())
    }
}
