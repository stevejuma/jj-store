//! Local, disposable Tantivy projection. Native history remains authoritative.
#![forbid(unsafe_code)]
mod disk;
mod document;
pub mod native;
pub mod paging;
mod projection;
mod search;
pub use document::{Document, DocumentKey, ScopeSnapshot, ScopeState, Update};
use jj_store_adapters::{Error, Result};
use jj_store_model as m;
pub use projection::{Extractor, Fault, Faults, TantivyProjection};
pub use search::{Page, PinnedReader, Search, SearchLimits, SearchRequest, SearchView};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::collections::BTreeMap;

pub const SCHEMA_VERSION: u32 = 2;
pub const IMPLEMENTATION: &str = "tantivy-0.26.2-v1";

#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub documents: usize,
    pub document_bytes: usize,
    pub update_bytes: usize,
    pub index_bytes: u64,
    pub index_files: usize,
    pub generations: usize,
    pub writer_bytes: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            documents: 100_000,
            document_bytes: 1024 * 1024,
            update_bytes: 64 * 1024 * 1024,
            index_bytes: 512 * 1024 * 1024,
            index_files: 4096,
            generations: 16,
            writer_bytes: 32 * 1024 * 1024,
        }
    }
}
impl Limits {
    fn validate(self) -> Result<()> {
        if self.documents == 0
            || self.documents > 1_000_000
            || self.document_bytes == 0
            || self.document_bytes > self.update_bytes
            || self.update_bytes > 256 * 1024 * 1024
            || self.index_bytes == 0
            || self.index_bytes > 2 * 1024 * 1024 * 1024
            || self.index_files == 0
            || self.index_files > 100_000
            || !(2..=128).contains(&self.generations)
            || !(16 * 1024 * 1024..=128 * 1024 * 1024).contains(&self.writer_bytes)
        {
            return Err(Error::Invalid);
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Payload {
    version: u32,
    identity: jj_store_adapters::Identity,
    source: jj_store_adapters::Position,
    previous: Option<m::RecordDigest>,
    generation: String,
    scopes: BTreeMap<String, ScopeSnapshot>,
    update: m::RecordDigest,
}
impl Payload {
    fn checkpoint(&self) -> Result<jj_store_adapters::Checkpoint> {
        Ok(jj_store_adapters::Checkpoint {
            version: 1,
            identity: self.identity.clone(),
            source: self.source.clone(),
            artifact: digest(&serde_json::to_vec(self).map_err(|_| Error::Corrupt)?),
        })
    }
}
fn digest(bytes: &[u8]) -> m::RecordDigest {
    m::RecordDigest::new(Sha256::digest(bytes).into())
}
fn bounded_json<T: Serialize>(value: &T, limit: usize) -> Result<Vec<u8>> {
    struct Buffer {
        bytes: Vec<u8>,
        limit: usize,
    }
    impl std::io::Write for Buffer {
        fn write(&mut self, value: &[u8]) -> std::io::Result<usize> {
            if value.len() > self.limit.saturating_sub(self.bytes.len()) {
                return Err(std::io::Error::other("JSON budget"));
            }
            self.bytes.extend_from_slice(value);
            Ok(value.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut output = Buffer {
        bytes: Vec::new(),
        limit,
    };
    serde_json::to_writer(&mut output, value).map_err(|_| Error::Limit)?;
    Ok(output.bytes)
}
fn io<T>(value: std::io::Result<T>) -> Result<T> {
    value.map_err(|_| Error::Unavailable)
}
fn tv<T>(value: tantivy::Result<T>) -> Result<T> {
    value.map_err(|_| Error::Projection)
}
