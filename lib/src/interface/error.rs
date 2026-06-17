#[cfg(target_os = "linux")]
use neli::{
    consts::{
        genl::{CtrlAttr, CtrlCmd},
        nl::GenlId,
    },
    err::{MsgError, RouterError, SerError},
    genl::{AttrTypeBuilderError, Genlmsghdr, GenlmsghdrBuilderError, NlattrBuilderError},
    types::Buffer,
};
use thiserror::Error;

#[cfg(target_os = "linux")]
use crate::nl80211::{Attr, Cmd};

/// Errors that can occur while enumerating Wi-Fi interfaces.
#[derive(Error, Debug)]
pub enum Error {
    /// An nl80211 operation failed.
    #[cfg(target_os = "linux")]
    #[error("Nl80211 error: {0}")]
    Nl80211(String),

    /// An I/O operation failed.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

// Internal conversions from neli error types
#[cfg(target_os = "linux")]
impl From<MsgError> for Error {
    fn from(err: MsgError) -> Self {
        Error::Nl80211(err.to_string())
    }
}

#[cfg(target_os = "linux")]
impl From<SerError> for Error {
    fn from(err: SerError) -> Self {
        Error::Nl80211(err.to_string())
    }
}

#[cfg(target_os = "linux")]
impl From<RouterError<u16, Buffer>> for Error {
    fn from(err: RouterError<u16, Buffer>) -> Self {
        Error::Nl80211(err.to_string())
    }
}

#[cfg(target_os = "linux")]
impl From<RouterError<GenlId, Genlmsghdr<CtrlCmd, CtrlAttr>>> for Error {
    fn from(err: RouterError<GenlId, Genlmsghdr<CtrlCmd, CtrlAttr>>) -> Self {
        Error::Nl80211(err.to_string())
    }
}

#[cfg(target_os = "linux")]
impl From<RouterError<u16, Genlmsghdr<Cmd, Attr>>> for Error {
    fn from(err: RouterError<u16, Genlmsghdr<Cmd, Attr>>) -> Self {
        Error::Nl80211(err.to_string())
    }
}

#[cfg(target_os = "linux")]
impl From<AttrTypeBuilderError> for Error {
    fn from(err: AttrTypeBuilderError) -> Self {
        Error::Nl80211(err.to_string())
    }
}

#[cfg(target_os = "linux")]
impl From<NlattrBuilderError> for Error {
    fn from(err: NlattrBuilderError) -> Self {
        Error::Nl80211(err.to_string())
    }
}

#[cfg(target_os = "linux")]
impl From<GenlmsghdrBuilderError> for Error {
    fn from(err: GenlmsghdrBuilderError) -> Self {
        Error::Nl80211(err.to_string())
    }
}
