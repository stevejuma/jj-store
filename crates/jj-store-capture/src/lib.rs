//! Reusable sanitized capture durability, retry and private filesystem runtime.
#![forbid(unsafe_code)]
pub mod agent_capture;
#[cfg(feature = "application")]
mod agent_capture_binding;
#[cfg(feature = "application")]
pub use agent_capture_binding::CaptureSpoolBinding;
#[cfg(feature = "application")]
mod agent_capture_enqueue;
#[doc(hidden)]
pub mod agent_capture_files;
#[cfg(feature = "application")]
mod agent_capture_intent;
mod agent_capture_lock;
#[cfg(feature = "application")]
pub mod agent_publication;
pub mod agent_sanitization;
pub mod codex_activity;
pub mod codex_discovery;
pub mod codex_import;
pub mod codex_resume;
pub mod codex_source;
pub mod codex_transcript;
pub mod cursor_binding;
#[cfg(feature = "application")]
pub mod local_agent_publication;
pub mod opencode_checkpoint;
pub mod opencode_ingress;
pub mod opencode_snapshot;
pub mod path_policy;
pub mod profile;
pub mod reader_progress;
#[cfg(feature = "application")]
mod reader_progress_files;
#[cfg(feature = "application")]
pub mod reader_publication;
#[cfg(feature = "application")]
mod service_agent_publication;
pub mod source_registry;
#[doc(hidden)]
pub mod state_lease;
pub mod transcript_sanitization;

pub type CaptureError = Box<dyn std::error::Error + Send + Sync>;
pub type CaptureResult<T> = std::result::Result<T, CaptureError>;
