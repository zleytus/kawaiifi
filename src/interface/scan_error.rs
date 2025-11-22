use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScanError {
    #[error("Interface is already scanning")]
    AlreadyScanning,

    #[error("Permission denied")]
    PermissionDenied,

    #[cfg(target_os = "linux")]
    #[error("Netlink error: {0}")]
    NetlinkError(String),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

// Internal conversions from neli error types
#[cfg(target_os = "linux")]
mod neli_conversions {
    use neli::{
        consts::{
            genl::{CtrlAttr, CtrlCmd},
            nl::GenlId,
        },
        err::{MsgError, RouterError, SerError},
        genl::{AttrTypeBuilderError, Genlmsghdr, GenlmsghdrBuilderError, NlattrBuilderError},
        types::Buffer,
    };

    use super::super::{Nl80211Attr, Nl80211Cmd};
    use super::ScanError;

    impl From<MsgError> for ScanError {
        fn from(err: MsgError) -> Self {
            ScanError::NetlinkError(err.to_string())
        }
    }

    impl From<SerError> for ScanError {
        fn from(err: SerError) -> Self {
            ScanError::NetlinkError(err.to_string())
        }
    }

    impl From<RouterError<u16, Buffer>> for ScanError {
        fn from(err: RouterError<u16, Buffer>) -> Self {
            ScanError::NetlinkError(err.to_string())
        }
    }

    impl From<RouterError<GenlId, Genlmsghdr<CtrlCmd, CtrlAttr>>> for ScanError {
        fn from(err: RouterError<GenlId, Genlmsghdr<CtrlCmd, CtrlAttr>>) -> Self {
            ScanError::NetlinkError(err.to_string())
        }
    }

    impl From<RouterError<u16, Genlmsghdr<Nl80211Cmd, Nl80211Attr>>> for ScanError {
        fn from(err: RouterError<u16, Genlmsghdr<Nl80211Cmd, Nl80211Attr>>) -> Self {
            ScanError::NetlinkError(err.to_string())
        }
    }

    impl From<AttrTypeBuilderError> for ScanError {
        fn from(err: AttrTypeBuilderError) -> Self {
            ScanError::NetlinkError(err.to_string())
        }
    }

    impl From<NlattrBuilderError> for ScanError {
        fn from(err: NlattrBuilderError) -> Self {
            ScanError::NetlinkError(err.to_string())
        }
    }

    impl From<GenlmsghdrBuilderError> for ScanError {
        fn from(err: GenlmsghdrBuilderError) -> Self {
            ScanError::NetlinkError(err.to_string())
        }
    }
}
