//! Repository-owned review anchors, comments and thread state.
#![forbid(unsafe_code)]

mod anchors;
mod codec;
mod dependencies;
mod ids;
mod selectors;
mod validation;

pub use anchors::*;
pub use codec::*;
pub use ids::*;
pub use selectors::*;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid review model: {0}")]
    Invalid(&'static str),
    #[error("review model exceeds its size limit")]
    Limit,
    #[error("invalid canonical review encoding")]
    Encoding(#[from] serde_json::Error),
}
