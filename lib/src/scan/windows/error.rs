use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Permission denied")]
    PermissionDenied,

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
