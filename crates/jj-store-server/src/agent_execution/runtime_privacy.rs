//! JJ host adaptation of shared in-memory stream privacy.
use super::{Error, Result};
use jj_store_capture::transcript_sanitization::TranscriptSanitizer;
use jj_store_conversations::{Error as ConversationError, privacy};
use serde_json::Value;
#[cfg(test)]
use serde_json::json;

pub(super) fn error(error: ConversationError) -> Error {
    match error {
        ConversationError::Invalid => Error::Invalid,
        ConversationError::Unavailable => Error::Unavailable,
        ConversationError::Limit => Error::Limit,
    }
}
pub(super) struct AdmittedSanitizer<'a>(pub(super) &'a TranscriptSanitizer);
impl privacy::Sanitizer for AdmittedSanitizer<'_> {
    fn sanitize(&self, value: Value) -> jj_store_conversations::Result<Value> {
        let payload = self
            .0
            .sanitize(value)
            .ok_or(ConversationError::Unavailable)?;
        serde_json::from_slice(payload.bytes()).map_err(|_| ConversationError::Unavailable)
    }
    fn replace_stream_text(
        &self,
        pending: &mut String,
        chunk: &str,
        complete: bool,
    ) -> jj_store_conversations::Result<String> {
        self.0
            .replace_stream_text(pending, chunk, complete)
            .map_err(|_| ConversationError::Unavailable)
    }
}
pub(super) fn safe(sanitizer: &TranscriptSanitizer, value: Value) -> Result<Value> {
    privacy::safe(&AdmittedSanitizer(sanitizer), value).map_err(error)
}
#[derive(Default)]
pub(super) struct StreamPrivacy(privacy::StreamPrivacy);
impl StreamPrivacy {
    #[cfg(test)]
    pub(super) fn has_pending_text(&self) -> bool {
        self.0.has_pending_text()
    }
    pub(super) fn frames(
        &mut self,
        sanitizer: &TranscriptSanitizer,
        raw: Value,
    ) -> Result<Vec<Value>> {
        self.0
            .frames(&AdmittedSanitizer(sanitizer), raw)
            .map_err(error)
    }
}
#[cfg(test)]
mod tests;
