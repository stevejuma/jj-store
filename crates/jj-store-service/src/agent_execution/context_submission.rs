//! Frozen publication-selected batch context, independent of the current message head.
use super::{
    Error, Result,
    context::{ContextBatch, ReplyRevision},
};
use jj_collab::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FrozenComment {
    pub reference: CommentReplyReferenceV1,
    pub body: String,
    pub source: Option<ConversationSourceV1>,
    #[serde(default)]
    pub surrounding: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubmittedBatch {
    pub thread_id: ThreadId,
    pub batch: CollabObjectRef,
    pub request_id: jj_store_model::RequestId,
}

pub async fn collect_submission(
    reader: &dyn ReactionTargetRead,
    repository: jj_store_model::RepositoryId,
    thread_id: ThreadId,
    batch: CollabObjectRef,
) -> Result<ContextBatch> {
    let Some(CollabObjectV1::ThreadLink(link)) = reader
        .collab_object(batch)
        .await
        .map_err(|_| Error::Unavailable)?
    else {
        return Err(Error::Invalid);
    };
    if link.thread_id != thread_id || !link.active {
        return Err(Error::Invalid);
    }
    let ThreadLinkTargetV1::CommentBatch {
        comments,
        request_id,
        ..
    } = link.target
    else {
        return Err(Error::Invalid);
    };
    let mut result = ContextBatch::default();
    result.submission = Some(SubmittedBatch {
        thread_id,
        batch,
        request_id,
    });
    let mut bytes = 0usize;
    for target in comments {
        resolve_reaction_target(reader, &target)
            .await
            .map_err(|_| Error::Unavailable)?;
        let (body, source) = match &target {
            ReactionTargetV1::HumanMessage {
                thread_id,
                message_id,
                revision,
            } => {
                let Some(CollabObjectV1::HumanMessageRevision(message)) = reader
                    .collab_object(*revision)
                    .await
                    .map_err(|_| Error::Unavailable)?
                else {
                    return Err(Error::Invalid);
                };
                let selected = reader
                    .thread_selection(*thread_id)
                    .await
                    .map_err(|_| Error::Unavailable)?
                    .ok_or(Error::Unavailable)?;
                let Some(CollabObjectV1::ThreadVersion(thread)) = reader
                    .collab_object(selected)
                    .await
                    .map_err(|_| Error::Unavailable)?
                else {
                    return Err(Error::Invalid);
                };
                result.revisions.insert(ReplyRevision {
                    repository,
                    thread: *thread_id,
                    message: *message_id,
                    revision: *revision,
                });
                (
                    message.body.ok_or(Error::Unavailable)?,
                    thread.comment_source,
                )
            }
            ReactionTargetV1::ReviewComment { revision, .. } => {
                use jj_review::{CommentRevisionContentV1, ReviewObjectV1};
                let Some(ReviewObjectV1::CommentRevision(comment)) = reader
                    .review_object(*revision)
                    .await
                    .map_err(|_| Error::Unavailable)?
                else {
                    return Err(Error::Invalid);
                };
                let CommentRevisionContentV1::Visible { body } = comment.content else {
                    return Err(Error::Unavailable);
                };
                let Some(ReviewObjectV1::CommentBody(body)) = reader
                    .review_object(body)
                    .await
                    .map_err(|_| Error::Unavailable)?
                else {
                    return Err(Error::Invalid);
                };
                (body.text, None)
            }
            _ => return Err(Error::Invalid),
        };
        let surrounding = if let Some(source) = &source {
            resolve_conversation_source(reader, source)
                .await
                .map_err(|_| Error::Unavailable)?;
            match conversation_source_text(reader, source).await {
                Ok(text) => source_context(&text, source),
                Err(_) if source.selection.is_none() => String::new(),
                Err(_) => return Err(Error::Unavailable),
            }
        } else {
            String::new()
        };
        let comment = FrozenComment {
            reference: CommentReplyReferenceV1::new(target).map_err(|_| Error::Invalid)?,
            body,
            source,
            surrounding,
        };
        bytes += serde_json::to_vec(&comment)
            .map_err(|_| Error::Invalid)?
            .len();
        if bytes > 32768 || result.comments.len() >= 128 {
            return Err(Error::FeedbackLimit);
        }
        result.comments.push(comment);
    }
    Ok(result)
}

fn source_context(text: &str, source: &ConversationSourceV1) -> String {
    let (start, end) = source
        .selection
        .as_ref()
        .map(|s| (s.start_byte.get() as usize, s.end_byte.get() as usize))
        .unwrap_or((0, 0));
    let mut from = start.saturating_sub(512);
    while !text.is_char_boundary(from) {
        from += 1;
    }
    let mut to = end.saturating_add(512).min(text.len());
    while !text.is_char_boundary(to) {
        to -= 1;
    }
    text[from..to].to_owned()
}
