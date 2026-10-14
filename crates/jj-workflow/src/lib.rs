//! Durable workflow data and transitions, independent of executors and storage.
#![forbid(unsafe_code)]
mod filters;
pub mod github;
mod ids;
mod model;
mod state;
pub use filters::*;
pub use ids::*;
pub use model::*;
pub use state::*;
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("invalid workflow: {0}")]
    Invalid(&'static str),
    #[error("workflow limit exceeded")]
    Limit,
    #[error("workflow precondition changed")]
    Stale,
    #[error("workflow target missing")]
    NotFound,
    #[error("workflow execution denied")]
    Denied,
}

/// Derive a stable domain identity without reinterpreting source JJ identities.
pub fn identity(domain: &[u8], parts: &[&[u8]]) -> [u8; 16] {
    use sha2::{Digest, Sha256};
    let mut hash = Sha256::new_with_prefix(b"jj-store/workflow/id/v1\0");
    hash.update(domain);
    for part in parts {
        hash.update((part.len() as u64).to_be_bytes());
        hash.update(part);
    }
    let mut id = [0; 16];
    id.copy_from_slice(&hash.finalize()[..16]);
    id
}
