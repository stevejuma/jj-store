use crate::*;
use serde::{Serialize, de::DeserializeOwned};
const MAX_OBJECT_BYTES: u64 = 16 * 1024 * 1024;
pub fn encode_object(value: &CollabObjectV1) -> Result<Vec<u8>> {
    value.validate()?;
    encode(value, MAX_OBJECT_BYTES)
}
pub fn decode_object(bytes: &[u8]) -> Result<CollabObjectV1> {
    let value: CollabObjectV1 = decode(bytes, MAX_OBJECT_BYTES)?;
    value.validate()?;
    Ok(value)
}
pub fn encode_thread_root(value: &ThreadRootV1) -> Result<Vec<u8>> {
    value.validate()?;
    encode(value, 1024 * 1024)
}
pub fn decode_thread_root(bytes: &[u8]) -> Result<ThreadRootV1> {
    let value: ThreadRootV1 = decode(bytes, 1024 * 1024)?;
    value.validate()?;
    Ok(value)
}
pub fn encode_reaction_root(value: &ReactionRootV1) -> Result<Vec<u8>> {
    value.validate()?;
    encode(value, 1024 * 1024)
}
pub fn decode_reaction_root(bytes: &[u8]) -> Result<ReactionRootV1> {
    let value: ReactionRootV1 = decode(bytes, 1024 * 1024)?;
    value.validate()?;
    Ok(value)
}
pub(crate) fn encode<T: Serialize>(value: &T, limit: u64) -> Result<Vec<u8>> {
    struct Limited {
        bytes: Vec<u8>,
        limit: u64,
    }
    impl std::io::Write for Limited {
        fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
            if (self.bytes.len() as u64).saturating_add(data.len() as u64) > self.limit {
                return Err(std::io::Error::other("collaboration object size limit"));
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

pub(crate) fn decode<T: Serialize + DeserializeOwned>(bytes: &[u8], limit: u64) -> Result<T> {
    if bytes.len() as u64 > limit {
        return Err(Error::Limit);
    }
    let value: T = serde_json::from_slice(bytes)?;
    if encode(&value, limit)? != bytes {
        return Err(Error::Invalid("noncanonical collaboration encoding"));
    }
    Ok(value)
}
