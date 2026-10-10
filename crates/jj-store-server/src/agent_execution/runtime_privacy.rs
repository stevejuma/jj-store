//! Prefix buffering is memory-only; all emitted frames pass the existing scanner.
use super::{Error, Result};
use jj_store_capture::transcript_sanitization::TranscriptSanitizer;
use serde_json::{Value, json};

pub(super) fn safe(sanitizer: &TranscriptSanitizer, value: Value) -> Result<Value> {
    let payload = sanitizer.sanitize(value).ok_or(Error::Unavailable)?;
    serde_json::from_slice(payload.bytes()).map_err(|_| Error::Unavailable)
}

#[derive(Default)]
pub(super) struct StreamPrivacy {
    key: Option<(String, String, String)>,
    pending: String,
    template: Option<Value>,
}
impl StreamPrivacy {
    pub(super) fn frames(
        &mut self,
        sanitizer: &TranscriptSanitizer,
        mut raw: Value,
    ) -> Result<Vec<Value>> {
        let kind = raw["update"]["sessionUpdate"].as_str();
        let key = (raw["type"] == "activity"
            && matches!(kind, Some("agent_message_chunk" | "agent_thought_chunk"))
            && raw["update"]["content"]["type"] == "text")
            .then(|| {
                (
                    raw["requestId"].as_str().unwrap_or("").to_owned(),
                    kind.unwrap_or("").to_owned(),
                    raw["update"]["messageId"].as_str().unwrap_or("").to_owned(),
                )
            });
        let mut output = Vec::new();
        if self.key != key {
            self.flush(sanitizer, &mut output)?;
        }
        self.key = key;
        if self.key.is_some() {
            let text = raw["update"]["content"]["text"]
                .as_str()
                .ok_or(Error::Invalid)?;
            let text = sanitizer
                .replace_stream_text(&mut self.pending, text, false)
                .map_err(|_| Error::Unavailable)?;
            raw["update"]["content"]["text"] = json!(text);
            let safe = safe(sanitizer, raw)?;
            self.template = Some(safe.clone());
            output.push(safe);
        } else {
            output.push(safe(sanitizer, raw)?);
        }
        Ok(output)
    }
    fn flush(&mut self, sanitizer: &TranscriptSanitizer, output: &mut Vec<Value>) -> Result<()> {
        let text = sanitizer
            .replace_stream_text(&mut self.pending, "", true)
            .map_err(|_| Error::Unavailable)?;
        if !text.is_empty() {
            let mut frame = self.template.take().ok_or(Error::Invalid)?;
            frame
                .as_object_mut()
                .ok_or(Error::Invalid)?
                .remove("sequence");
            frame["update"]["content"]["text"] = json!(text);
            output.push(safe(sanitizer, frame)?);
        }
        self.template = None;
        Ok(())
    }
}

#[cfg(test)]
mod tests;
