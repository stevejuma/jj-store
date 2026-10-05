//! Repository-bound desktop shells. This module has no server/storage authority.
#[cfg(feature = "native")]
pub mod commands;
mod io;
mod runtime;
mod session;
mod targets;
mod transport;
pub mod wire;
pub use runtime::Runtime;
pub use session::SessionInfo;
pub use targets::{Target, TargetResolver};
pub type Result<T> = std::result::Result<T, String>;
pub type Sink = std::sync::Arc<dyn Fn(wire::TerminalMessage) -> Result<()> + Send + Sync>;
