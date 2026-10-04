//! Repository-owned review anchors, comments and thread state.
#![forbid(unsafe_code)]

mod anchors;
mod catalog;
mod codec;
mod commands;
mod comments;
mod delta;
mod dependencies;
mod ids;
mod objects;
mod portable;
mod portable_agent;
mod relations;
mod replication;
mod resolver;
mod resolver_agent;
mod resolver_code;
mod resolver_text;
mod selectors;
mod threads;
mod validation;

pub use anchors::*;
pub use catalog::*;
pub use codec::*;
pub use commands::*;
pub use comments::*;
pub use delta::*;
pub use ids::*;
pub use objects::*;
pub use portable::*;
pub use portable_agent::*;
pub use relations::*;
pub use replication::*;
pub use resolver::*;
pub use selectors::*;
pub use threads::*;

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
