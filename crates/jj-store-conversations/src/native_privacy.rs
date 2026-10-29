//! Opt-in adaptation of the host-admitted native transcript policy.
use crate::{
    Error, Result,
    capture::{Binding, CaptureError, DurableCapture, NativeCapture},
    privacy::Sanitizer,
};
pub use lapis_agent_redaction::{TRANSCRIPT_POLICY, TranscriptSanitizer};
use serde_json::Value;
use std::sync::Arc;

impl Sanitizer for TranscriptSanitizer {
    fn sanitize(&self, value: Value) -> Result<Value> {
        let safe = TranscriptSanitizer::sanitize(self, value).ok_or(Error::Unavailable)?;
        serde_json::from_slice(safe.bytes()).map_err(|_| Error::Unavailable)
    }
    fn replace_stream_text(
        &self,
        pending: &mut String,
        chunk: &str,
        complete: bool,
    ) -> Result<String> {
        TranscriptSanitizer::replace_stream_text(self, pending, chunk, complete)
            .map_err(|_| Error::Unavailable)
    }
}
impl NativeCapture {
    /// The host must admit the sanitizer's private cache, paths and credentials
    /// before construction and retain current scope and sink ownership.
    pub fn with_native_privacy(
        binding: Binding,
        sanitizer: Arc<TranscriptSanitizer>,
        sink: Box<dyn DurableCapture>,
    ) -> std::result::Result<Self, CaptureError> {
        Self::new(binding, TRANSCRIPT_POLICY.into(), sanitizer, sink)
    }
}
