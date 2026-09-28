use crate::{ArtifactAnchorV1, Error, Result};
use serde::{Serialize, de::DeserializeOwned};

const MAX_ANCHOR_BYTES: u64 = 1024 * 1024;

pub fn encode_anchor(anchor: &ArtifactAnchorV1) -> Result<Vec<u8>> {
    anchor.validate()?;
    encode(anchor)
}

pub fn decode_anchor(bytes: &[u8]) -> Result<ArtifactAnchorV1> {
    let anchor: ArtifactAnchorV1 = decode(bytes)?;
    anchor.validate()?;
    Ok(anchor)
}

fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let bytes = serde_json::to_vec(value)?;
    if bytes.len() as u64 > MAX_ANCHOR_BYTES {
        return Err(Error::Limit);
    }
    Ok(bytes)
}

fn decode<T: Serialize + DeserializeOwned>(bytes: &[u8]) -> Result<T> {
    if bytes.len() as u64 > MAX_ANCHOR_BYTES {
        return Err(Error::Limit);
    }
    let value: T = serde_json::from_slice(bytes)?;
    if encode(&value)? != bytes {
        return Err(Error::Invalid("noncanonical review encoding"));
    }
    Ok(value)
}
