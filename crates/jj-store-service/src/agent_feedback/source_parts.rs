//! Explicit captured JSON string pointers; no rendered-text or quote matching.
use super::*;
use crate::agent_execution::FeedbackTextPart;
use jj_store_wal::publication::{SanitizedMediaType, SanitizedPayloadV1};
pub(super) async fn parts(
    agents: &dyn AgentSnapshotRead,
    target: &ReactionTargetV1,
) -> Result<Vec<FeedbackTextPart>> {
    let content = match target {
        ReactionTargetV1::AgentMessage { content, .. }
        | ReactionTargetV1::ToolOutput { content, .. } => Some(*content),
        ReactionTargetV1::ToolInvocation { snapshot, .. } => match agents
            .read_object(*snapshot)
            .await
            .map_err(|_| Error::Unavailable)?
        {
            AgentObjectV1::ToolSnapshot(tool) => tool.input,
            _ => None,
        },
        _ => None,
    };
    let Some(content) = content else {
        return Ok(Vec::new());
    };
    let AgentObjectV1::Content(content) = agents
        .read_object(content)
        .await
        .map_err(|_| Error::Unavailable)?
    else {
        return Err(Error::Invalid);
    };
    Ok(text_parts(&content.payload))
}
fn text_parts(payload: &SanitizedPayloadV1) -> Vec<FeedbackTextPart> {
    const MAX: usize = 256 * 1024;
    let mut parts = Vec::new();
    if payload.media_type() == SanitizedMediaType::Text {
        if payload.bytes().len() <= MAX
            && let Ok(text) = std::str::from_utf8(payload.bytes())
        {
            parts.push(FeedbackTextPart {
                part: None,
                text: text.into(),
            });
        }
    } else if let Ok(value) = serde_json::from_slice(payload.bytes()) {
        let mut pending = vec![(String::new(), value)];
        let mut bytes = 0;
        while let Some((part, value)) = pending.pop() {
            match value {
                serde_json::Value::String(text) => {
                    bytes += text.len();
                    if bytes > MAX || parts.len() >= 128 {
                        return Vec::new();
                    }
                    parts.push(FeedbackTextPart {
                        part: Some(part),
                        text,
                    });
                }
                serde_json::Value::Object(values) => {
                    for (key, value) in values {
                        pending.push((
                            format!("{part}/{}", key.replace('~', "~0").replace('/', "~1")),
                            value,
                        ));
                    }
                }
                serde_json::Value::Array(values) => {
                    for (i, value) in values.into_iter().enumerate() {
                        pending.push((format!("{part}/{i}"), value));
                    }
                }
                _ => {}
            }
        }
    }
    parts
}
