use deku::prelude::*;
use serde::{Deserialize, Serialize};

/// Information Element identifier consisting of an ID and optional extension ID.
///
/// Used internally for IE type discrimination during parsing.
/// Standard IEs use only `id`, while extended IEs (id=255) also have `id_ext`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, DekuWrite, Serialize, Deserialize)]
pub struct IeId {
    #[deku(skip)]
    pub id: u8,
    #[deku(skip)]
    pub id_ext: Option<u8>,
}

impl IeId {
    pub const fn new(id: u8, id_ext: Option<u8>) -> Self {
        Self { id, id_ext }
    }
}
