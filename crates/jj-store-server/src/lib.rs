//! Foreground server composition, with no command parser or upstream JJ imports.
#![forbid(unsafe_code)]
pub mod config;
mod listener;
pub mod policy;
pub mod process;
pub mod search;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid server configuration: {0}")]
    Config(&'static str),
    #[error("server filesystem operation failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("server service failed: {0}")]
    Service(&'static str),
}
pub type Result<T> = std::result::Result<T, Error>;
