//! Per-participation privacy and safe assistant/tool accumulation for a fleet worker.
use super::{Error, Result, runtime_privacy::StreamPrivacy};
use jj_store_capture::transcript_sanitization::TranscriptSanitizer;
use serde_json::{Value, json};
use std::collections::BTreeMap;
#[derive(Default)]
pub(super) struct Stream {
    assistant: BTreeMap<String, String>,
    tools: BTreeMap<String, Value>,
    privacy: BTreeMap<String, StreamPrivacy>,
}
impl Stream {
    pub(super) fn frames(
        &mut self,
        id: &str,
        sanitizer: &TranscriptSanitizer,
        raw: Value,
    ) -> Result<Vec<Value>> {
        if self.privacy.len() >= 256 && !self.privacy.contains_key(id) {
            return Err(Error::Limit);
        }
        let terminal = matches!(
            raw["type"].as_str(),
            Some("record" | "session_closed" | "error")
        );
        let output = self
            .privacy
            .entry(id.to_owned())
            .or_default()
            .frames(sanitizer, raw)?;
        if terminal {
            self.privacy.remove(id);
        }
        Ok(output)
    }
    pub(super) fn observe(&mut self, id: &str, request: &str, value: &mut Value) -> Result<()> {
        let key = format!("{id}/{request}");
        if value["type"] == "activity" {
            let update = &value["update"];
            if update["sessionUpdate"] == "agent_message_chunk"
                && let Some(text) = update["content"]["text"].as_str()
            {
                let target = self.assistant.entry(key.clone()).or_default();
                if target.len() + text.len() > 512 * 1024 {
                    return Err(Error::Limit);
                }
                target.push_str(text);
            }
            if let Some(call) = update["toolCallId"].as_str() {
                let tool = format!("{key}/{call}");
                if self.tools.len() >= 256 && !self.tools.contains_key(&tool) {
                    return Err(Error::Limit);
                }
                let target = self.tools.entry(tool).or_insert_with(|| json!({}));
                if let Some(fields) = update.as_object() {
                    for (k, v) in fields {
                        target[k] = v.clone();
                    }
                }
                value["toolSnapshot"] = target.clone();
            }
        }
        if value["type"] == "record" {
            value["assistantText"] = json!(self.assistant.remove(&key).unwrap_or_default());
            self.tools
                .retain(|key2, _| !key2.starts_with(&format!("{key}/")));
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests;
