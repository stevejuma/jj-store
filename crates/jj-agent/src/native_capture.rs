//! Exact native capture evidence; projection never rewrites the retained envelope.
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct NativeCaptureBindingV1 {
    pub scope: String,
    pub worker: String,
    pub request: String,
    pub target: String,
    pub window: String,
    pub admission: String,
}
impl NativeCaptureBindingV1 {
    fn validate(&self) -> Result<()> {
        if [
            &self.scope,
            &self.worker,
            &self.request,
            &self.target,
            &self.window,
            &self.admission,
        ]
        .iter()
        .any(|s| s.is_empty() || s.len() > 1024 || s.chars().any(char::is_control))
        {
            return Err(Error::Invalid("native capture binding"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum NativeCapturePublicationV1 {
    Facts,
    Event { sequence: u64 },
}

/// The existing `jj-store-conversations` schema-1 capture bytes, retained as one
/// sanitized observation. The observation order is the native processing ordinal;
/// publication cursors and original frame sequences remain separate exact facts.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct NativeCaptureEnvelopeV1 {
    pub schema: u32,
    pub policy: String,
    pub binding: NativeCaptureBindingV1,
    pub ordinal: u64,
    pub publication: NativeCapturePublicationV1,
    pub frames: Vec<Value>,
}
impl NativeCaptureEnvelopeV1 {
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.len() > 1024 * 1024 {
            return Err(Error::Limit);
        }
        let value: Self = serde_json::from_slice(bytes)?;
        value.validate()?;
        Ok(value)
    }
    pub fn validate(&self) -> Result<()> {
        self.binding.validate()?;
        if self.schema != 1
            || self.ordinal == 0
            || self.policy.is_empty()
            || self.policy.len() > 128
            || self.policy.chars().any(char::is_control)
            || !(1..=2).contains(&self.frames.len())
            || matches!(
                self.publication,
                NativeCapturePublicationV1::Event { sequence: 0 }
            )
        {
            return Err(Error::Invalid("native capture envelope"));
        }
        for frame in &self.frames {
            if frame["version"] != 3
                || !frame["type"].as_str().is_some_and(|v| {
                    !v.is_empty() && v.len() <= 64 && !v.chars().any(char::is_control)
                })
                || ["sessionId", "controllerSessionId"].iter().any(|key| {
                    frame
                        .get(*key)
                        .is_some_and(|value| value.as_str() != Some(self.binding.worker.as_str()))
                })
            {
                return Err(Error::Invalid("native capture frame"));
            }
        }
        if let [prefix, _] = self.frames.as_slice()
            && (prefix["type"] != "activity"
                || !matches!(
                    prefix["update"]["sessionUpdate"].as_str(),
                    Some("agent_message_chunk" | "agent_thought_chunk")
                )
                || prefix["update"]["content"]["type"] != "text"
                || !prefix["update"]["content"]["text"].is_string()
                || prefix.get("sequence").is_some())
        {
            return Err(Error::Invalid("native capture deferred frame"));
        }
        if self.publication == NativeCapturePublicationV1::Facts
            && self
                .frames
                .last()
                .is_none_or(|frame| frame["type"] != "opened")
        {
            return Err(Error::Invalid("native capture facts"));
        }
        Ok(())
    }
}
