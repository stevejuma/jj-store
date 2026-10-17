//! Host-admitted immutable local history. No renderer-supplied source authority.
#![forbid(unsafe_code)]
mod jobs;
mod policy;
mod scopes;
mod service;
#[cfg(feature = "tauri")]
mod transport;

pub use jj_store_jj::local_browser as local;
pub use jj_store_jj::query::{Cancellation, cache};
pub use jj_store_model::OperationId;
pub use policy::{Admission, Config, Context};
pub use service::{Handle, History, Status};
#[cfg(feature = "tauri")]
pub use transport::Builder;
pub type Result<T> = local::Result<T>;
pub use local::Error;
