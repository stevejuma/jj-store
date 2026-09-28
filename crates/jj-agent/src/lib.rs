//! Repository-owned agent activity models and canonical immutable objects.
#![forbid(unsafe_code)]

mod activity;
mod capture;
mod catalog;
mod codec;
mod delta;
mod ids;
mod objects;
mod provenance;
mod transcript;

pub use activity::*;
pub use capture::*;
pub use catalog::*;
pub use codec::{decode_object, decode_root, encode_object, encode_root};
pub use delta::*;
pub use ids::*;
pub use objects::*;
pub use provenance::*;
pub use transcript::*;

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
