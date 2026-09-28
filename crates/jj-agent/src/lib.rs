//! Repository-owned agent activity models and canonical immutable objects.
#![forbid(unsafe_code)]

mod codec;
mod ids;
mod objects;

pub use codec::{decode_object, decode_root, encode_object, encode_root};
pub use ids::*;
pub use objects::*;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid agent model: {0}")]
    Invalid(&'static str),
    #[error("agent model exceeds its size limit")]
    Limit,
    #[error("invalid canonical agent encoding")]
    Encoding(#[from] serde_json::Error),
}
