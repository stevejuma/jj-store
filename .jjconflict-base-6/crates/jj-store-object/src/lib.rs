//! Bounded object I/O with explicit conditional-write uncertainty. No LIST or
//! destructive methods are exposed by the V1 authority contract.
#![forbid(unsafe_code)]
use async_trait::async_trait;
use std::{fmt, pin::Pin};
use tokio::io::{AsyncRead, AsyncReadExt as _};

pub mod memory;
pub mod s3;
pub mod staging;
pub use staging::StagedUpload;

pub type Reader = Pin<Box<dyn AsyncRead + Send>>;
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid object-store configuration or key: {0}")]
    Invalid(&'static str),
    #[error("object not found")]
    NotFound,
    #[error("object exceeds configured byte limit")]
    Limit,
    #[error("invalid or unsatisfiable byte range")]
    Range,
    #[error("storage request rejected (HTTP {0})")]
    Rejected(u16),
    #[error("invalid storage response: {0}")]
    Response(&'static str),
    #[error("storage connection failed")]
    Transport,
    #[error("storage I/O failed: {0}")]
    Io(#[from] std::io::Error),
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Key(String);
impl Key {
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        if value.is_empty()
            || value.len() > 1024
            || value
                .split('/')
                .any(|s| s.is_empty() || s == "." || s == "..")
            || value
                .bytes()
                .any(|c| !c.is_ascii_alphanumeric() && !b"/-_.".contains(&c))
        {
            return Err(Error::Invalid("key must be a bounded canonical ASCII path"));
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
/// Equality token only. Do not interpret this as a checksum or ordered version.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VersionToken(String);
impl VersionToken {
    pub fn new(token: impl Into<String>) -> Result<Self> {
        let token = token.into();
        if token.is_empty() || token.len() > 1024 || token.bytes().any(|c| !c.is_ascii_graphic()) {
            return Err(Error::Response("invalid version token"));
        }
        Ok(Self(token))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ByteRange {
    start: u64,
    end: u64,
}
impl ByteRange {
    pub fn new(start: u64, end: u64) -> Result<Self> {
        if start >= end {
            return Err(Error::Range);
        }
        Ok(Self { start, end })
    }
    pub fn start(self) -> u64 {
        self.start
    }
    pub fn end(self) -> u64 {
        self.end
    }
    pub fn len(self) -> u64 {
        self.end - self.start
    }
    pub fn is_empty(self) -> bool {
        false
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Metadata {
    pub len: u64,
    pub version: VersionToken,
}
pub struct Download {
    pub metadata: Metadata,
    pub reader: Reader,
}
impl fmt::Debug for Download {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Download")
            .field("metadata", &self.metadata)
            .finish_non_exhaustive()
    }
}
impl Download {
    pub async fn read_bounded(self, limit: u64) -> Result<Vec<u8>> {
        if self.metadata.len > limit {
            return Err(Error::Limit);
        }
        let mut bytes = Vec::new();
        self.reader
            .take(limit.saturating_add(1))
            .read_to_end(&mut bytes)
            .await?;
        if bytes.len() as u64 > limit {
            return Err(Error::Limit);
        }
        if bytes.len() as u64 != self.metadata.len {
            return Err(Error::Response("body length mismatch"));
        }
        Ok(bytes)
    }
}
#[derive(Clone, Debug)]
pub enum Condition {
    Absent,
    Matches(VersionToken),
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WriteOutcome {
    Applied(VersionToken),
    PreconditionFailed,
    Unknown,
}
#[async_trait]
pub trait ObjectStore: Send + Sync {
    async fn head(&self, key: &Key) -> Result<Metadata>;
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> Result<Download>;
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> Result<WriteOutcome>;
}
