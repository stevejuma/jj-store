//! Bounded, transport-neutral exchange between writable jj-store replicas.
#![forbid(unsafe_code)]

mod protocol;
mod transfer;

pub use protocol::*;
pub use transfer::{TransferLimits, TransferOutcome, transfer_closures};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Publication(#[from] jj_store_wal::Error),
    #[error(transparent)]
    Storage(#[from] jj_store_object::Error),
    #[error("replication integrity failure: {0}")]
    Integrity(&'static str),
    #[error("replication resource limit exceeded: {0}")]
    Limit(&'static str),
}

pub type Result<T> = std::result::Result<T, Error>;
