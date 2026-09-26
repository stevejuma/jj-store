//! Manifest-selected publication, immutable inventories, receipts and recovery.
#![forbid(unsafe_code)]
pub mod catalog;
pub mod inventory;
pub mod publication;
pub mod records;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Storage(#[from] jj_store_object::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Inventory(#[from] inventory::InventoryError),
    #[error("invalid publication record: {0}")]
    Corrupt(&'static str),
    #[error("publication resource limit exceeded: {0}")]
    Limit(&'static str),
    #[error("publication outcome is unknown; retain the original request ID")]
    OutcomeUnknown,
    #[error("secure attempt identity generation failed")]
    Entropy,
    #[error("repository already exists with a different epoch")]
    AlreadyExists,
    #[error("publication snapshot is stale")]
    Stale,
    #[error("request or publication identity was reused with different intent")]
    IntentConflict,
    #[error("request is not authorized by current publication policy")]
    Denied,
    #[error("publication CAS retry budget exhausted")]
    Contended,
}
pub type Result<T> = std::result::Result<T, Error>;
