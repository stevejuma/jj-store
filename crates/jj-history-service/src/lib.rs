//! Host-admitted, transport-independent immutable local history.
#![forbid(unsafe_code)]
mod annotations;
mod file_history;
pub mod frames;
mod jobs;
mod policy;
mod scopes;
mod service;
pub mod wire;

pub use jj_store_jj::local_browser as local;
pub use jj_store_jj::query::{Cancellation, blame, cache};
pub use jj_store_model::OperationId;
pub use policy::{Admission, Config, Context};
pub use service::{Handle, History, Status};
pub type Result<T> = local::Result<T>;
pub use local::Error;
