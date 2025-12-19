use crate::nl80211::{Attr, Cmd};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Interface is already scanning")]
    AlreadyScanning,

    #[error("Permission denied")]
    PermissionDenied,

    #[cfg(target_os = "linux")]
    #[error("Nl80211 error: {0}")]
    Nl80211(String),

    #[error("Network Manager error: {0}")]
    NetworkManager(String),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

use neli::{
    consts::{
        genl::{CtrlAttr, CtrlCmd},
        nl::GenlId,
    },
    err::{MsgError, RouterError, SerError},
    genl::{AttrTypeBuilderError, Genlmsghdr, GenlmsghdrBuilderError, NlattrBuilderError},
    types::Buffer,
};

// Internal conversions from neli error types
impl From<MsgError> for Error {
    fn from(err: MsgError) -> Self {
        Error::Nl80211(err.to_string())
    }
}

impl From<SerError> for Error {
    fn from(err: SerError) -> Self {
        Error::Nl80211(err.to_string())
    }
}

impl From<RouterError<u16, Buffer>> for Error {
    fn from(err: RouterError<u16, Buffer>) -> Self {
        Error::Nl80211(err.to_string())
    }
}

impl From<RouterError<GenlId, Genlmsghdr<CtrlCmd, CtrlAttr>>> for Error {
    fn from(err: RouterError<GenlId, Genlmsghdr<CtrlCmd, CtrlAttr>>) -> Self {
        Error::Nl80211(err.to_string())
    }
}

impl From<RouterError<u16, Genlmsghdr<Cmd, Attr>>> for Error {
    fn from(err: RouterError<u16, Genlmsghdr<Cmd, Attr>>) -> Self {
        Error::Nl80211(err.to_string())
    }
}

impl From<AttrTypeBuilderError> for Error {
    fn from(err: AttrTypeBuilderError) -> Self {
        Error::Nl80211(err.to_string())
    }
}

impl From<NlattrBuilderError> for Error {
    fn from(err: NlattrBuilderError) -> Self {
        Error::Nl80211(err.to_string())
    }
}

impl From<GenlmsghdrBuilderError> for Error {
    fn from(err: GenlmsghdrBuilderError) -> Self {
        Error::Nl80211(err.to_string())
    }
}
