//! Repository-owned persistent Threads and rich Reactions. No publication or runner authority.
#![forbid(unsafe_code)]
mod assets;
mod capture_binding;
mod catalog;
mod codec;
mod command_types;
mod commands;
mod comment_reply;
mod comment_source;
pub use comment_reply::*;
mod conversation_metadata;
mod conversation_submission;
pub use conversation_metadata::*;
mod graph;
mod ids;
mod objects;
mod reaction_state;
mod reactions;
mod resolver;
mod targets;
mod threads;

pub use assets::*;
pub use catalog::*;
pub use codec::*;
pub use command_types::*;
pub use commands::*;
pub use comment_source::*;
pub use conversation_submission::*;
pub use graph::*;
pub use ids::*;
pub use objects::*;
pub use reaction_state::*;
pub use reactions::*;
pub use resolver::*;
pub use targets::*;
pub use threads::*;

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid collaboration model: {0}")]
    Invalid(&'static str),
    #[error("collaboration model exceeds its size limit")]
    Limit,
    #[error("observed collaboration state is stale")]
    Stale,
    #[error("reaction target is unavailable")]
    TargetUnavailable,
    #[error("pinned reaction target reader is unavailable")]
    ReadUnavailable,
    #[error("invalid canonical collaboration encoding")]
    Encoding(#[from] serde_json::Error),
    #[error("invalid reaction image")]
    Image(#[from] image::ImageError),
}
pub(crate) fn label(value: &str, max: usize) -> bool {
    !value.trim().is_empty() && value.len() <= max && !value.chars().any(char::is_control)
}
pub(crate) fn schema(version: u32) -> Result<()> {
    if version == 1 {
        Ok(())
    } else {
        Err(Error::Invalid("schema version"))
    }
}

mod workspace_comments;
pub use workspace_comments::*;
