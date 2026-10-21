//! Tauri transport for the shared, host-admitted local history service.
#![forbid(unsafe_code)]
#[cfg(feature = "tauri")]
mod transport;

pub use jj_history_service::*;
#[cfg(feature = "tauri")]
pub use transport::Builder;
