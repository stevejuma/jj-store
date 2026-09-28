use crate::{ArtifactAnchorV1, Error, Result, ReviewObjectV1, ReviewRootV1};
use serde::{Serialize, de::DeserializeOwned};

const MAX_ANCHOR_BYTES: u64 = 1024 * 1024;
const MAX_ROOT_BYTES: u64 = 1024 * 1024;
const MAX_OBJECT_BYTES: u64 = 16 * 1024 * 1024;

pub fn encode_anchor(anchor: &ArtifactAnchorV1) -> Result<Vec<u8>> {
    anchor.validate()?;
    encode(anchor, MAX_ANCHOR_BYTES)
}

pub fn decode_anchor(bytes: &[u8]) -> Result<ArtifactAnchorV1> {
    let anchor: ArtifactAnchorV1 = decode(bytes, MAX_ANCHOR_BYTES)?;
    anchor.validate()?;
    Ok(anchor)
}

pub fn encode_root(root: &ReviewRootV1) -> Result<Vec<u8>> {
    root.validate()?;
    encode(root, MAX_ROOT_BYTES)
}

pub fn decode_root(bytes: &[u8]) -> Result<ReviewRootV1> {
    let root: ReviewRootV1 = decode(bytes, MAX_ROOT_BYTES)?;
    root.validate()?;
    Ok(root)
}

pub fn encode_object(object: &ReviewObjectV1) -> Result<Vec<u8>> {
    object.validate()?;
    encode(object, MAX_OBJECT_BYTES)
}

pub fn decode_object(bytes: &[u8]) -> Result<ReviewObjectV1> {
    let object: ReviewObjectV1 = decode(bytes, MAX_OBJECT_BYTES)?;
    object.validate()?;
    Ok(object)
}

fn encode<T: Serialize>(value: &T, limit: u64) -> Result<Vec<u8>> {
    struct Limited {
        bytes: Vec<u8>,
        limit: u64,
    }
    impl std::io::Write for Limited {
        fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
            if (self.bytes.len() as u64).saturating_add(data.len() as u64) > self.limit {
                return Err(std::io::Error::other("review object size limit"));
            }
            self.bytes.extend_from_slice(data);
            Ok(data.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut writer = Limited {
        bytes: Vec::new(),
        limit,
    };
    match serde_json::to_writer(&mut writer, value) {
        Ok(()) => Ok(writer.bytes),
        Err(error) if error.is_io() => Err(Error::Limit),
        Err(error) => Err(Error::Encoding(error)),
    }
}

fn decode<T: Serialize + DeserializeOwned>(bytes: &[u8], limit: u64) -> Result<T> {
    if bytes.len() as u64 > limit {
        return Err(Error::Limit);
    }
    let value: T = serde_json::from_slice(bytes)?;
    if encode(&value, limit)? != bytes {
        return Err(Error::Invalid("noncanonical review encoding"));
    }
    Ok(value)
}
