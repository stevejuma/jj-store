//! Canonical typed control-record bytes shared by persistence providers.
use crate::{Error, Result};
use jj_store_model::RecordDigest;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest as _, Sha256};
pub const MAX_RECORD_BYTES: u64 = 16 * 1024 * 1024;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Envelope<T, K> {
    pub format: u32,
    pub repository: jj_store_model::RepositoryId,
    pub kind: K,
    pub payload: T,
}
pub fn digest(bytes: &[u8]) -> RecordDigest {
    RecordDigest::new(Sha256::digest(bytes).into())
}
pub fn encode<T: Serialize>(value: &T, limit: u64) -> Result<Vec<u8>> {
    // Write through a limit instead of allocating an unbounded encoded payload.
    struct Limited {
        bytes: Vec<u8>,
        limit: u64,
    }
    impl std::io::Write for Limited {
        fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
            if (self.bytes.len() as u64).saturating_add(data.len() as u64) > self.limit {
                return Err(std::io::Error::other("control record size limit"));
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
    serde_json::to_writer(&mut writer, value)?;
    Ok(writer.bytes)
}
pub fn decode<T: Serialize + DeserializeOwned>(bytes: &[u8]) -> Result<T> {
    if bytes.len() as u64 > MAX_RECORD_BYTES {
        return Err(Error::Limit("control record bytes"));
    }
    let value: T = serde_json::from_slice(bytes)?;
    if encode(&value, MAX_RECORD_BYTES)? != bytes {
        return Err(Error::Corrupt("noncanonical control record"));
    }
    Ok(value)
}
