//! Shared local WorkStore command contract, independent of application services.
use jj_store_model as m;
pub use jj_work::*;
use serde::{Deserialize, Serialize};
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("access denied")]
    Denied,
    #[error("stale selection")]
    Stale,
    #[error("request identity has different intent")]
    IntentConflict,
    #[error("target not found")]
    NotFound,
    #[error("invalid request")]
    Invalid,
    #[error("integrity failure")]
    Corrupt,
    #[error("resource limit")]
    Limit,
    #[error("authority unavailable")]
    Unavailable,
    #[error("admission busy")]
    Busy,
}
impl From<jj_store_core::Error> for Error {
    fn from(e: jj_store_core::Error) -> Self {
        use jj_store_core::Error as E;
        match e {
            E::Stale => Self::Stale,
            E::IntentConflict => Self::IntentConflict,
            E::Denied => Self::Denied,
            E::NotFound => Self::Unavailable,
            E::Limit(_) => Self::Limit,
            E::Corrupt(_) | E::Json(_) | E::Invalid(_) => Self::Corrupt,
            E::Contended => Self::Busy,
            _ => Self::Unavailable,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "WorkCommandRequest")]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub version: u32,
    pub request_id: m::RequestId,
    pub command: WorkCommandV1,
}

impl From<crate::LocalError> for Error {
    fn from(e: crate::LocalError) -> Self {
        use crate::LocalError as E;
        match e {
            E::Stale => Self::Stale,
            E::IntentConflict => Self::IntentConflict,
            E::NotFound => Self::NotFound,
            E::Invalid => Self::Invalid,
            E::Corrupt => Self::Corrupt,
            E::Limit => Self::Limit,
            E::Busy => Self::Busy,
            _ => Self::Unavailable,
        }
    }
}
