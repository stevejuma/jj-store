//! Injectable collaboration libraries. Hosts retain authorization and publication authority.
#![forbid(unsafe_code)]
pub mod engine;
pub mod record_format;
pub mod stores;
pub use stores::*;

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid immutable record: {0}")]
    Corrupt(&'static str),
    #[error("unsupported store capability: {0}")]
    Unsupported(&'static str),
    #[error("store resource limit exceeded: {0}")]
    Limit(&'static str),
    #[error("invalid store request: {0}")]
    Invalid(&'static str),
    #[error("store target not found")]
    NotFound,
    #[error("store snapshot is stale")]
    Stale,
    #[error("request identity has different intent")]
    IntentConflict,
    #[error("store access denied")]
    Denied,
    #[error("publication contention")]
    Contended,
    #[error("publication outcome unknown; retain original request identity")]
    OutcomeUnknown,
    #[error("store unavailable: {0}")]
    Unavailable(String),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
