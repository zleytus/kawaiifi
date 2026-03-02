//! Serde helpers for serializing parsed Wi-Fi structures as raw bytes.
//!
//! Instead of serializing each parsed field individually, these helpers
//! convert structures to/from their raw wire-format bytes. This preserves
//! reserved fields that may gain meaning in future spec revisions.

use base64::{Engine, engine::general_purpose::STANDARD};
use deku::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::{Ie, from_bytes};
use crate::CapabilityInfo;

/// Serialize/deserialize `Vec<Ie>` as base64-encoded raw bytes.
pub mod ies_as_base64 {
    use super::*;

    pub fn serialize<S: Serializer>(ies: &Vec<Ie>, serializer: S) -> Result<S::Ok, S::Error> {
        let bytes = ies_to_bytes(ies);
        STANDARD.encode(&bytes).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<Ie>, D::Error> {
        let encoded = String::deserialize(deserializer)?;
        let bytes = STANDARD
            .decode(&encoded)
            .map_err(serde::de::Error::custom)?;
        Ok(from_bytes(&bytes))
    }
}

/// Serialize/deserialize `Option<Vec<Ie>>` as an optional base64-encoded string.
pub mod option_ies_as_base64 {
    use super::*;

    pub fn serialize<S: Serializer>(
        ies: &Option<Vec<Ie>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match ies {
            Some(ies) => {
                let bytes = ies_to_bytes(ies);
                Some(STANDARD.encode(&bytes)).serialize(serializer)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Vec<Ie>>, D::Error> {
        let encoded: Option<String> = Option::deserialize(deserializer)?;
        match encoded {
            Some(s) => {
                let bytes = STANDARD.decode(&s).map_err(serde::de::Error::custom)?;
                Ok(Some(from_bytes(&bytes)))
            }
            None => Ok(None),
        }
    }
}

/// Serialize/deserialize `CapabilityInfo` as a u16.
pub mod capability_info_as_u16 {
    use super::*;

    pub fn serialize<S: Serializer>(
        cap: &CapabilityInfo,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let bytes = cap.to_bytes().map_err(serde::ser::Error::custom)?;
        u16::from_le_bytes([bytes[0], bytes[1]]).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<CapabilityInfo, D::Error> {
        let val = u16::deserialize(deserializer)?;
        let bytes = val.to_le_bytes();
        CapabilityInfo::from_bytes((&bytes, 0))
            .map(|(_, cap)| cap)
            .map_err(serde::de::Error::custom)
    }
}

fn ies_to_bytes(ies: &[Ie]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for ie in ies {
        if let Ok(ie_bytes) = ie.to_bytes() {
            bytes.extend_from_slice(&ie_bytes);
        }
    }
    bytes
}
