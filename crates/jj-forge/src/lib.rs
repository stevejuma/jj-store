//! Forge integrations are external effects; repository code remains JJ-owned.
#![forbid(unsafe_code)]
pub mod accounts;
pub mod client;
pub mod engine;
pub mod journal;
pub mod planner;
mod providers;
pub mod types;
pub use client::Client;
pub use types::*;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid forge connection or request")]
    Invalid,
    #[error("Forge operation is unsupported by this instance")]
    Unsupported,
    #[error("Forge authentication or permission is unavailable")]
    Denied,
    #[error("Forge state changed; refresh the preview before continuing")]
    Stale,
    #[error("Forge rate limit reached; retry after the provider cooldown")]
    RateLimited,
    #[error("Forge outcome is unknown; inspect the original request before retrying")]
    Unknown,
    #[error("Forge rejected the operation (HTTP {0})")]
    Rejected(u16),
    #[error("Forge device state is unavailable or busy")]
    State,
    #[error("Forge response exceeds the bounded profile")]
    Limit,
}
pub type Result<T> = std::result::Result<T, Error>;
