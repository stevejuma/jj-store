//! Desktop process ownership, independent of repository and storage authority.
#![forbid(unsafe_code)]
pub mod installation;
pub mod process;
pub mod readiness;
pub mod runtime;

#[cfg(feature = "native")]
pub mod native;

pub type Result<T> = std::result::Result<T, String>;
