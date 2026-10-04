//! V1 framing. Integrity validation is separate from native semantic validation.
use sha2::{Digest as _, Sha256};

use crate::{EnvelopeDigest, ObjectKey};

pub const HEADER_LEN: usize = 113;
pub const PROFILE: u16 = 1;
const PREFIX_LEN: usize = 81;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum EnvelopeError {
    #[error("truncated or invalid envelope length")]
    Length,
    #[error("unsupported envelope magic, version, profile or object kind")]
    Format,
    #[error("object exceeds configured byte limit")]
    Limit,
    #[error("object key does not match requested identity")]
    Key,
    #[error("envelope integrity check failed")]
    Integrity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub key: ObjectKey,
    pub payload_len: u64,
    pub digest: EnvelopeDigest,
}
impl Header {
    fn prefix(key: ObjectKey, payload_len: u64) -> [u8; PREFIX_LEN] {
        let mut out = [0; PREFIX_LEN];
        out[..4].copy_from_slice(b"JJST");
        out[4..6].copy_from_slice(&1u16.to_be_bytes());
        out[6..8].copy_from_slice(&PROFILE.to_be_bytes());
        out[8] = key.kind() as u8;
        out[9..73].copy_from_slice(key.as_bytes());
        out[73..81].copy_from_slice(&payload_len.to_be_bytes());
        out
    }
    /// Seed a digest for incremental payload processing without buffering blobs.
    pub fn hasher(key: ObjectKey, payload_len: u64) -> Sha256 {
        Sha256::new_with_prefix(Self::prefix(key, payload_len))
    }
    pub fn encode(&self) -> [u8; HEADER_LEN] {
        let mut out = [0; HEADER_LEN];
        out[..PREFIX_LEN].copy_from_slice(&Self::prefix(self.key, self.payload_len));
        out[PREFIX_LEN..].copy_from_slice(self.digest.as_bytes());
        out
    }
    pub fn decode(
        bytes: &[u8],
        expected: ObjectKey,
        max_payload: u64,
    ) -> Result<Self, EnvelopeError> {
        let bytes: &[u8; HEADER_LEN] = bytes.try_into().map_err(|_| EnvelopeError::Length)?;
        if &bytes[..4] != b"JJST"
            || bytes[4..6] != 1u16.to_be_bytes()
            || bytes[6..8] != PROFILE.to_be_bytes()
        {
            return Err(EnvelopeError::Format);
        }
        let key = ObjectKey::from_parts(bytes[8], bytes[9..73].try_into().unwrap())
            .ok_or(EnvelopeError::Format)?;
        if key != expected {
            return Err(EnvelopeError::Key);
        }
        let payload_len = u64::from_be_bytes(bytes[73..81].try_into().unwrap());
        if payload_len > max_payload {
            return Err(EnvelopeError::Limit);
        }
        Ok(Self {
            key,
            payload_len,
            digest: EnvelopeDigest::new(bytes[81..].try_into().unwrap()),
        })
    }
}

pub fn encode(key: ObjectKey, payload: &[u8], max_payload: u64) -> Result<Vec<u8>, EnvelopeError> {
    if payload.len() as u64 > max_payload {
        return Err(EnvelopeError::Limit);
    }
    let mut hash = Header::hasher(key, payload.len() as u64);
    hash.update(payload);
    let header = Header {
        key,
        payload_len: payload.len() as u64,
        digest: EnvelopeDigest::new(hash.finalize().into()),
    };
    let mut out = Vec::with_capacity(HEADER_LEN + payload.len());
    out.extend_from_slice(&header.encode());
    out.extend_from_slice(payload);
    Ok(out)
}

/// Returns the verified payload without a copy. The native layer must still
/// decode and recompute semantic identity before accepting the object.
pub fn decode(bytes: &[u8], expected: ObjectKey, max_payload: u64) -> Result<&[u8], EnvelopeError> {
    let prefix = bytes.get(..HEADER_LEN).ok_or(EnvelopeError::Length)?;
    let header = Header::decode(prefix, expected, max_payload)?;
    let payload = &bytes[HEADER_LEN..];
    if payload.len() as u64 != header.payload_len {
        return Err(EnvelopeError::Length);
    }
    let mut hash = Header::hasher(header.key, header.payload_len);
    hash.update(payload);
    if hash.finalize().as_slice() != header.digest.as_bytes() {
        return Err(EnvelopeError::Integrity);
    }
    Ok(payload)
}
