//! Headless conversation coordination; hosts own admission, privacy and durability.
#![forbid(unsafe_code)]
pub mod control_v3;
pub mod privacy;

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("invalid conversation input")]
    Invalid,
    #[error("conversation capability unavailable")]
    Unavailable,
    #[error("conversation bound exceeded")]
    Limit,
}
pub type Result<T> = std::result::Result<T, Error>;
