use std::{ops::Deref, str::Utf8Error};

use deku::{DekuRead, DekuWrite};

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(ctx = "len: usize")]
pub struct MeshId {
    #[deku(count = "len")]
    mesh_id: Vec<u8>,
}

impl MeshId {
    pub const NAME: &'static str = "Mesh ID";
    pub const ID: u8 = 114;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);

    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(&self.mesh_id)
    }

    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(&self.mesh_id).into_owned()
    }
}

impl Deref for MeshId {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.mesh_id
    }
}
