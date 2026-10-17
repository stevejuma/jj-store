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
    #[error("unsupported repository profile: {0}")]
    Unsupported(&'static str),
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

impl From<jj_store_core::Error> for Error {
    fn from(value: jj_store_core::Error) -> Self {
        use jj_store_core::Error as E;
        match value {
            E::Corrupt(v) | E::Invalid(v) => Self::Corrupt(v),
            E::Unsupported(v) => Self::Unsupported(v),
            E::Limit(v) => Self::Limit(v),
            E::OutcomeUnknown => Self::OutcomeUnknown,
            E::Stale => Self::Stale,
            E::IntentConflict => Self::IntentConflict,
            E::Denied => Self::Denied,
            E::Contended => Self::Contended,
            E::Json(v) => Self::Json(v),
            E::NotFound => Self::Storage(jj_store_object::Error::NotFound),
            E::Unavailable(_) => Self::Storage(jj_store_object::Error::Transport),
        }
    }
}
impl From<Error> for jj_store_core::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::Corrupt(v) => Self::Corrupt(v),
            Error::Unsupported(v) => Self::Unsupported(v),
            Error::Limit(v) => Self::Limit(v),
            Error::Stale => Self::Stale,
            Error::IntentConflict => Self::IntentConflict,
            Error::Denied => Self::Denied,
            Error::Contended => Self::Contended,
            Error::OutcomeUnknown => Self::OutcomeUnknown,
            Error::Json(v) => Self::Json(v),
            Error::Storage(jj_store_object::Error::NotFound) => Self::NotFound,
            other => Self::Unavailable(other.to_string()),
        }
    }
}

impl From<Error> for jj_store_core::engine::ReviewCommandError {
    fn from(value: Error) -> Self {
        Self::Publication(value.into())
    }
}
