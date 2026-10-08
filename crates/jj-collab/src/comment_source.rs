//! Exact conversation source selectors. Presentation offsets never become authority.
use crate::*;
use jj_agent::{AgentObjectV1, SanitizedMediaType, SanitizedPayloadV1};
use jj_review::TextSelectionV1;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ConversationSourceV1 {
    pub target: ReactionTargetV1,
    /// JSON pointer to an exact string in captured JSON; None selects plain text.
    pub part: Option<String>,
    pub selection: Option<TextSelectionV1>,
    /// Retained display evidence, validated against the selected bytes.
    pub quote: String,
}
impl ConversationSourceV1 {
    pub fn validate(&self) -> Result<()> {
        if !matches!(
            self.target,
            ReactionTargetV1::HumanMessage { .. }
                | ReactionTargetV1::AgentMessage { .. }
                | ReactionTargetV1::ToolInvocation { .. }
                | ReactionTargetV1::ToolOutput { .. }
                | ReactionTargetV1::ReviewComment { .. }
        ) {
            return Err(Error::Invalid("conversation source target"));
        }
        self.target.validate()?;
        if self.quote.len() > 32768
            || self.quote.contains('\0')
            || self.part.as_ref().is_some_and(|p| {
                p.len() > 1024
                    || (!p.is_empty() && !p.starts_with('/'))
                    || p.chars().any(char::is_control)
            })
        {
            return Err(Error::Invalid("conversation source bounds"));
        }
        if let Some(selection) = &self.selection {
            selection
                .validate()
                .map_err(|_| Error::Invalid("conversation selection"))?;
            if self.quote.is_empty() {
                return Err(Error::Invalid("empty conversation quote"));
            }
        } else if self.part.is_some() || !self.quote.is_empty() {
            return Err(Error::Invalid("whole source has text metadata"));
        }
        Ok(())
    }
    pub fn validate_text(&self, text: &str) -> Result<()> {
        self.validate()?;
        let Some(selection) = &self.selection else {
            return Ok(());
        };
        selection
            .validate_bytes(text.as_bytes())
            .map_err(|_| Error::Invalid("conversation selection bytes"))?;
        if text.get(selection.start_byte.get() as usize..selection.end_byte.get() as usize)
            != Some(&self.quote)
        {
            return Err(Error::Invalid("conversation quote mismatch"));
        }
        Ok(())
    }
}

/// Providers first prove exact membership through the same bounded target reader.
/// This additional read selects one captured text part; it never fetches a file.
pub async fn resolve_conversation_source(
    reader: &dyn ReactionTargetRead,
    source: &ConversationSourceV1,
) -> Result<()> {
    source.validate()?;
    resolve_reaction_target(reader, &source.target).await?;
    if source.selection.is_none() {
        return Ok(());
    }
    let text = match &source.target {
        ReactionTargetV1::HumanMessage { revision, .. } => {
            if source.part.is_some() {
                return Err(Error::Invalid("human message part"));
            }
            match reader
                .collab_object(*revision)
                .await?
                .ok_or(Error::TargetUnavailable)?
            {
                CollabObjectV1::HumanMessageRevision(message) => {
                    message.body.ok_or(Error::TargetUnavailable)?
                }
                _ => return Err(Error::Invalid("human source kind")),
            }
        }
        ReactionTargetV1::AgentMessage { content, .. }
        | ReactionTargetV1::ToolOutput { content, .. } => {
            match reader
                .agent_object(*content)
                .await?
                .ok_or(Error::TargetUnavailable)?
            {
                AgentObjectV1::Content(content) => {
                    captured_text(&content.payload, source.part.as_deref())?
                }
                _ => return Err(Error::Invalid("agent source kind")),
            }
        }
        ReactionTargetV1::ToolInvocation { snapshot, .. } => {
            let AgentObjectV1::ToolSnapshot(tool) = reader
                .agent_object(*snapshot)
                .await?
                .ok_or(Error::TargetUnavailable)?
            else {
                return Err(Error::Invalid("tool source kind"));
            };
            let input = tool.input.ok_or(Error::TargetUnavailable)?;
            let AgentObjectV1::Content(content) = reader
                .agent_object(input)
                .await?
                .ok_or(Error::TargetUnavailable)?
            else {
                return Err(Error::Invalid("tool input kind"));
            };
            captured_text(&content.payload, source.part.as_deref())?
        }
        ReactionTargetV1::ReviewComment { revision, .. } => {
            if source.part.is_some() {
                return Err(Error::Invalid("review comment part"));
            }
            let jj_review::ReviewObjectV1::CommentRevision(comment) = reader
                .review_object(*revision)
                .await?
                .ok_or(Error::TargetUnavailable)?
            else {
                return Err(Error::Invalid("review source kind"));
            };
            let jj_review::CommentRevisionContentV1::Visible { body } = comment.content else {
                return Err(Error::TargetUnavailable);
            };
            let jj_review::ReviewObjectV1::CommentBody(body) = reader
                .review_object(body)
                .await?
                .ok_or(Error::TargetUnavailable)?
            else {
                return Err(Error::Invalid("review body kind"));
            };
            body.text
        }
        _ => return Err(Error::Invalid("conversation text target")),
    };
    source.validate_text(&text)
}

pub fn captured_text(payload: &SanitizedPayloadV1, part: Option<&str>) -> Result<String> {
    match (payload.media_type(), part) {
        (SanitizedMediaType::Text, None) => {
            String::from_utf8(payload.bytes().to_vec()).map_err(|_| Error::Invalid("source utf8"))
        }
        (SanitizedMediaType::Json, Some(pointer)) => {
            serde_json::from_slice::<serde_json::Value>(payload.bytes())?
                .pointer(pointer)
                .and_then(|v| v.as_str())
                .map(str::to_owned)
                .ok_or(Error::Invalid("source string part"))
        }
        _ => Err(Error::Invalid("source representation")),
    }
}
