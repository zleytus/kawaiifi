use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub(crate) enum ParseError {
    #[error("Missing required attribute: {0}")]
    MissingAttribute(&'static str),

    #[error("Unexpected command type: expected {expected:?}, got {got:?}")]
    UnexpectedCommand {
        expected: super::Cmd,
        got: super::Cmd,
    },

    #[error("Neli deserialization error: {0}")]
    DeError(#[from] neli::err::DeError),

    #[error("Deku parsing error: {0}")]
    Deku(#[from] deku::error::DekuError),

    #[error("Failed to convert from primitive")]
    TryFromPrimitive {
        primitive: &'static str,
        expected_type: &'static str,
    },
}
