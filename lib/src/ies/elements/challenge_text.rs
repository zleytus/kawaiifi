use std::{ops::Deref, str::Utf8Error};

use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use crate::ies::{Field, IeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "len: usize")]
pub struct ChallengeText {
    #[deku(count = "len")]
    challenge_text: Vec<u8>,
}

impl ChallengeText {
    pub const ID: u8 = 16;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const NAME: &'static str = "Challenge Text";

    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(&self.challenge_text)
    }

    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(&self.challenge_text).into_owned()
    }

    pub fn summary(&self) -> String {
        self.to_string_lossy()
    }

    pub fn fields(&self) -> Vec<Field> {
        vec![
            Field::builder()
                .title("Challenge Text")
                .value(self.to_string_lossy())
                .bytes(self.challenge_text.clone())
                .build(),
        ]
    }
}

impl Deref for ChallengeText {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.challenge_text
    }
}
