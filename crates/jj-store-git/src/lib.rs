//! Standalone Git-backed collaboration stores. Publication is an owned-ref CAS.
#![forbid(unsafe_code)]
mod domain;
pub mod evidence;
pub mod metadata;
#[doc(hidden)]
pub mod records;
pub mod repository;
pub mod work_api;
pub use repository::*;
pub mod factories;
pub use factories::GitProvider;
impl From<LocalError> for jj_store_core::Error {
    fn from(e: LocalError) -> Self {
        use LocalError as E;
        match e {
            E::Stale => Self::Stale,
            E::IntentConflict => Self::IntentConflict,
            E::NotFound => Self::NotFound,
            E::Limit => Self::Limit("Git store"),
            E::Corrupt => Self::Corrupt("Git store"),
            E::Invalid => Self::Invalid("Git command"),
            E::Unsupported => Self::Unsupported("Git store profile"),
            E::Busy => Self::Contended,
            _ => Self::Unavailable(e.to_string()),
        }
    }
}
impl From<work_api::Error> for jj_store_core::Error {
    fn from(e: work_api::Error) -> Self {
        use work_api::Error as E;
        match e {
            E::Stale => Self::Stale,
            E::IntentConflict => Self::IntentConflict,
            E::NotFound => Self::NotFound,
            E::Denied => Self::Denied,
            E::Limit => Self::Limit("Git WorkStore"),
            E::Corrupt => Self::Corrupt("Git WorkStore"),
            E::Invalid => Self::Invalid("Work command"),
            E::Busy => Self::Contended,
            _ => Self::Unavailable(e.to_string()),
        }
    }
}
