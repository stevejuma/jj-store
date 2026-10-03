//! Desktop process ownership, independent of repository and storage authority.
#![deny(unsafe_code)]
pub mod installation;
pub mod process;
pub mod readiness;
pub mod runtime;

#[cfg(feature = "native")]
pub mod native;

#[cfg(feature = "native")]
mod native_window;

#[cfg(feature = "native")]
mod native_glass;

// Audited platform handle and retained view hierarchy borrows; all other desktop code denies unsafe.
#[cfg(all(feature = "native", target_os = "macos"))]
#[allow(unsafe_code)]
mod macos_chrome;

pub type Result<T> = std::result::Result<T, String>;
