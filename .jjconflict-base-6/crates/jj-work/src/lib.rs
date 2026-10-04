//! Neutral project, task and threaded-discussion contracts.
#![forbid(unsafe_code)]
mod apply;
mod commands;
mod ids;
mod state;
mod validation;
pub use commands::*;
pub use ids::*;
pub use state::*;
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("invalid work model: {0}")]
    Invalid(&'static str),
    #[error("work model exceeds its size limit")]
    Limit,
    #[error("work command is stale")]
    Stale,
    #[error("work target was not found: {0}")]
    NotFound(&'static str),
}
