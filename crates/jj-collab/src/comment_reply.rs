//! Versioned answer selectors. Tags are evidence candidates, never commands.
use crate::*;
use jj_agent::{
    AgentEventV1, AgentObjectV1, IntegrationId, MessageRole, SanitizedMediaType, SourceKind,
    SourceTrust,
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, ops::Range};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CommentReplyReferenceV1 {
    pub discussion: String,
    pub comment: String,
    pub target: ReactionTargetV1,
}
impl CommentReplyReferenceV1 {
    pub fn new(target: ReactionTargetV1) -> Result<Self> {
        validate_comment_reference(&target)?;
        let discussion = match &target {
            ReactionTargetV1::HumanMessage { thread_id, .. } => format!("thread-{thread_id}"),
            ReactionTargetV1::ReviewComment { thread_id, .. } => format!("review-{thread_id}"),
            _ => return Err(Error::Invalid("comment protocol target")),
        };
        let comment = format!("comment-{}", target.exact_key()?);
        Ok(Self {
            discussion,
            comment,
            target,
        })
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CommentReplyProtocolV1 {
    pub version: u32,
    pub references: Vec<CommentReplyReferenceV1>,
}
impl CommentReplyProtocolV1 {
    pub fn validate(&self) -> Result<()> {
        if self.version != 1 || self.references.len() > 128 {
            return Err(Error::Invalid("comment protocol bounds"));
        }
        let mut seen = BTreeSet::new();
        for reference in &self.references {
            if &CommentReplyReferenceV1::new(reference.target.clone())? != reference
                || !seen.insert(&reference.comment)
            {
                return Err(Error::Invalid("comment protocol identity"));
            }
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CommentReplyLinkV1 {
    /// Exact user message capture containing a successful context delivery.
    pub prompt: ReactionTargetV1,
    pub answer: ConversationSourceV1,
    pub comments: Vec<ReactionTargetV1>,
}
impl CommentReplyLinkV1 {
    pub fn validate(&self) -> Result<()> {
        self.prompt.validate()?;
        self.answer.validate()?;
        if !matches!(self.prompt, ReactionTargetV1::AgentMessage { .. })
            || !matches!(self.answer.target, ReactionTargetV1::AgentMessage { .. })
            || self.answer.selection.is_none()
            || self.answer.part.is_some()
            || self.comments.is_empty()
            || self.comments.len() > 128
        {
            return Err(Error::Invalid("comment reply shape"));
        }
        let mut seen = BTreeSet::new();
        let mut discussion = None;
        for target in &self.comments {
            let reference = CommentReplyReferenceV1::new(target.clone())?;
            if discussion
                .as_ref()
                .is_some_and(|id| id != &reference.discussion)
                || !seen.insert(reference.comment)
            {
                return Err(Error::Invalid("comment reply discussion"));
            }
            discussion = Some(reference.discussion);
        }
        Ok(())
    }
    pub fn targets(&self) -> impl Iterator<Item = &ReactionTargetV1> {
        [&self.prompt, &self.answer.target]
            .into_iter()
            .chain(&self.comments)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParsedCommentReply {
    pub discussion: String,
    pub comments: Vec<String>,
    pub block: Range<usize>,
    pub answer: Range<usize>,
}
fn token(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'-')
}
/// Parse a complete buffer (or its streaming prefix). Only complete, well-formed
/// blocks outside literal Markdown code are returned. Offsets are UTF-8 bytes.
pub fn parse_comment_replies(text: &str) -> Vec<ParsedCommentReply> {
    use pulldown_cmark::{Event, Parser, Tag};
    if text.len() > 4 * 1024 * 1024 {
        return Vec::new();
    }
    // Mask tag openers without changing byte offsets so HTML parsing cannot
    // hide literal-code ranges inside a reply. One parse bounds work by input.
    let masked = text.replace('<', "x");
    let mut literal: Vec<_> = Parser::new(&masked)
        .into_offset_iter()
        .filter_map(|(event, range)| {
            matches!(event, Event::Code(_) | Event::Start(Tag::CodeBlock(_))).then_some(range)
        })
        .collect();
    literal.sort_by_key(|r| r.start);
    let protected = |range: &Range<usize>| {
        let next = literal.partition_point(|r| r.end <= range.start);
        literal.get(next).is_some_and(|r| r.start < range.end)
    };
    let mut result = Vec::new();
    let mut cursor = 0;
    let mut candidates = 0;
    while let Some(start) = text[cursor..].find("<comment-reply").map(|n| cursor + n) {
        candidates += 1;
        if candidates > 256 {
            break;
        }
        cursor = start + "<comment-reply".len();
        let Some(open_end) = text[cursor..].find('>').map(|n| cursor + n + 1) else {
            break;
        };
        if open_end - start > 32768 || protected(&(start..open_end)) {
            continue;
        }
        let Some(close) = text[open_end..]
            .match_indices("</comment-reply>")
            .find(|(at, tag)| !protected(&(open_end + at..open_end + at + tag.len())))
            .map(|(at, _)| open_end + at)
        else {
            break;
        };
        let end = close + "</comment-reply>".len();
        if text[open_end..close]
            .match_indices("<comment-reply")
            .any(|(at, tag)| !protected(&(open_end + at..open_end + at + tag.len())))
            || protected(&(close..end))
        {
            cursor = end;
            continue;
        }
        let opening = &text[start..open_end];
        let parsed = opening
            .strip_prefix("<comment-reply discussion=\"")
            .and_then(|s| s.strip_suffix("\">"))
            .and_then(|s| s.split_once("\" comments=\""));
        cursor = end;
        let Some((discussion, comments)) = parsed else {
            continue;
        };
        let comments: Vec<_> = comments.split(' ').map(str::to_owned).collect();
        if !token(discussion)
            || comments.is_empty()
            || comments.len() > 128
            || comments.iter().any(|c| !token(c))
            || comments.iter().collect::<BTreeSet<_>>().len() != comments.len()
        {
            continue;
        }
        let body = &text[open_end..close];
        let first = open_end + body.len() - body.trim_start().len();
        let last = open_end + body.trim_end().len();
        if first >= last {
            continue;
        }
        result.push(ParsedCommentReply {
            discussion: discussion.into(),
            comments,
            block: start..end,
            answer: first..last,
        });
        if result.len() == 128 {
            break;
        }
    }
    result
}

/// Raw Link commands are subject to this proof too. Neither a browser parse nor
/// a caller-supplied list of delivered IDs establishes reply status.
pub async fn resolve_comment_reply(
    reader: &dyn ReactionTargetRead,
    link: &CommentReplyLinkV1,
) -> Result<()> {
    link.validate()?;
    for target in link.targets() {
        resolve_reaction_target(reader, target).await?;
    }
    resolve_conversation_source(reader, &link.answer).await?;
    let (session, turn, prompt) = captured_message(reader, &link.prompt, MessageRole::User).await?;
    let (answer_session, answer_turn, answer) =
        captured_message(reader, &link.answer.target, MessageRole::Assistant).await?;
    if session != answer_session
        || turn != answer_turn
        || turn.is_none()
        || prompt.payload.media_type() != SanitizedMediaType::Json
    {
        return Err(Error::Invalid("reply participation"));
    }
    let prompt: serde_json::Value = serde_json::from_slice(prompt.payload.bytes())?;
    if prompt["schema"] != "jj-store.sent-context.v1" || prompt["context"]["delivery"] != "sent" {
        return Err(Error::Invalid("reply context not delivered"));
    }
    let protocol: CommentReplyProtocolV1 =
        serde_json::from_value(prompt["context"]["commentReplyProtocol"].clone())?;
    protocol.validate()?;
    for target in &link.comments {
        if !protocol
            .references
            .contains(&CommentReplyReferenceV1::new(target.clone())?)
        {
            return Err(Error::Invalid("reply reference not delivered"));
        }
    }
    let text = captured_text(&answer.payload, None)?;
    let selection = link
        .answer
        .selection
        .as_ref()
        .ok_or(Error::Invalid("reply selection"))?;
    let wanted: Vec<_> = link
        .comments
        .iter()
        .cloned()
        .map(CommentReplyReferenceV1::new)
        .collect::<Result<_>>()?;
    if !parse_comment_replies(&text).iter().any(|block| {
        block.answer == (selection.start_byte.get() as usize..selection.end_byte.get() as usize)
            && block.discussion == wanted[0].discussion
            && block.comments == wanted.iter().map(|r| r.comment.clone()).collect::<Vec<_>>()
    }) {
        return Err(Error::Invalid("reply answer range"));
    }
    Ok(())
}
async fn captured_message(
    reader: &dyn ReactionTargetRead,
    target: &ReactionTargetV1,
    role: MessageRole,
) -> Result<(
    jj_agent::AgentSessionId,
    Option<jj_agent::TurnId>,
    jj_agent::AgentContentV1,
)> {
    let ReactionTargetV1::AgentMessage {
        session_id,
        event_id,
        batch,
        content,
        ..
    } = target
    else {
        return Err(Error::Invalid("reply message"));
    };
    let Some(AgentObjectV1::EventBatch(batch)) = reader.agent_object(*batch).await? else {
        return Err(Error::TargetUnavailable);
    };
    let turn = batch
        .events
        .iter()
        .find_map(|e| match e {
            AgentEventV1::MessageCompleted {
                event_id: id,
                role: r,
                turn_id,
                ..
            } if id == event_id && *r == role => Some(*turn_id),
            _ => None,
        })
        .ok_or(Error::Invalid("reply message role"))?;
    let Some(AgentObjectV1::Content(content)) = reader.agent_object(*content).await? else {
        return Err(Error::TargetUnavailable);
    };
    let Some(AgentObjectV1::ObservationBatch(evidence)) =
        reader.agent_object(batch.evidence).await?
    else {
        return Err(Error::TargetUnavailable);
    };
    if evidence.integration != IntegrationId::OwnedController {
        return Err(Error::Invalid("reply capture owner"));
    }
    // Re-run the owned identity mapping on retained evidence, binding its body
    // to this exact session/turn/message rather than any observation in a batch.
    let normalized = jj_agent::normalize_batch(
        &jj_agent::NormalizerProfileV1::acp_controller_v2(),
        &evidence,
    )
    .map_err(|_| Error::Invalid("reply capture normalization"))?;
    let ReactionTargetV1::AgentMessage { message_id, .. } = target else {
        unreachable!()
    };
    if !normalized.records.iter().any(|record| matches!(record,
        jj_agent::NormalizedActivityRecordV1::Message(message)
            if message.session_id == *session_id && message.message_id == *message_id
                && message.turn_id == turn && message.role == role && message.content == content.payload)) {
        return Err(Error::Invalid("reply capture identity"));
    }
    let owned = evidence.observations.iter().any(|observation| {
        if observation.source_kind != SourceKind::Controller || observation.source_trust != SourceTrust::AuthenticatedController { return false; }
        let Ok(value) = serde_json::from_slice::<serde_json::Value>(observation.payload.bytes()) else { return false };
        if role == MessageRole::User {
            value["type"] == "context_delivery" && value["sentContext"]["delivery"] == "sent"
                && serde_json::from_slice::<serde_json::Value>(content.payload.bytes()).is_ok_and(|v|
                    v == serde_json::json!({"schema":"jj-store.sent-context.v1","text":value["userText"],"context":value["sentContext"]}))
        } else {
            value["type"] == "record" && value["assistantText"].as_str().is_some_and(|s| s.as_bytes() == content.payload.bytes())
        }
    });
    if !owned {
        return Err(Error::Invalid("reply delivery evidence"));
    }
    Ok((*session_id, turn, content))
}

impl ParsedCommentReply {
    pub fn link(
        &self,
        prompt: ReactionTargetV1,
        answer: ReactionTargetV1,
        protocol: &CommentReplyProtocolV1,
        text: &str,
    ) -> Result<CommentReplyLinkV1> {
        protocol.validate()?;
        let comments = self
            .comments
            .iter()
            .map(|id| {
                protocol
                    .references
                    .iter()
                    .find(|r| r.comment == *id && r.discussion == self.discussion)
                    .map(|r| r.target.clone())
                    .ok_or(Error::Invalid("reply reference"))
            })
            .collect::<Result<Vec<_>>>()?;
        let quote = text
            .get(self.answer.clone())
            .ok_or(Error::Invalid("reply range"))?;
        let link = CommentReplyLinkV1 {
            prompt,
            comments,
            answer: ConversationSourceV1 {
                target: answer,
                part: None,
                quote: quote.into(),
                selection: Some(jj_review::TextSelectionV1 {
                    encoding: jj_review::TextEncoding::Utf8,
                    start_byte: jj_review::ByteOffset::new(self.answer.start as u64),
                    end_byte: jj_review::ByteOffset::new(self.answer.end as u64),
                    selected_sha256: jj_review::ReviewDigest::sha256(quote.as_bytes()),
                }),
            },
        };
        link.validate()?;
        Ok(link)
    }
}
