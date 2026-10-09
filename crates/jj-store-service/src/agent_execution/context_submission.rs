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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<FrozenFileSource>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FrozenFileSource {
    pub anchor: jj_review::ArtifactAnchorV1,
    pub quote: String,
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
        let mut file = None;
        let mut file_surrounding = String::new();
        let (body, source) = match &target {
            ReactionTargetV1::RepositoryReviewComment { .. } => {
                let resolved = reader
                    .qualified_review_comment(&target)
                    .await
                    .map_err(|_| Error::Unavailable)?;
                let (source, surrounding) = frozen_file_source(&resolved)?;
                file = Some(source);
                file_surrounding = surrounding;
                (resolved.body, None)
            }
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
            file_surrounding
        };
        let comment = FrozenComment {
            reference: CommentReplyReferenceV1::new(target).map_err(|_| Error::Invalid)?,
            body,
            source,
            surrounding,
            file,
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

pub fn frozen_file_source(comment: &QualifiedReviewComment) -> Result<(FrozenFileSource, String)> {
    use jj_review::ArtifactTargetV1;
    comment
        .anchor
        .validate_text(&comment.file)
        .map_err(|_| Error::Invalid)?;
    let text = std::str::from_utf8(&comment.file).map_err(|_| Error::Unavailable)?;
    if text.len() > 256 * 1024 || text.contains('\0') {
        return Err(Error::FeedbackLimit);
    }
    let selection = match &comment.anchor.target {
        ArtifactTargetV1::CodeText(v) => Some(&v.text),
        ArtifactTargetV1::PortableCodeText(v) => Some(&v.text),
        ArtifactTargetV1::CodeArtifact(_) | ArtifactTargetV1::PortableCodeArtifact(_) => None,
        _ => return Err(Error::Invalid),
    };
    let (start, end) = selection
        .map(|s| (s.start_byte.get() as usize, s.end_byte.get() as usize))
        .unwrap_or((0, floor_boundary(text, 512.min(text.len()))));
    if end - start > 32768 {
        return Err(Error::FeedbackLimit);
    }
    let mut from = start.saturating_sub(512);
    while !text.is_char_boundary(from) {
        from += 1;
    }
    let to = floor_boundary(text, end.saturating_add(512).min(text.len()));
    Ok((
        FrozenFileSource {
            anchor: comment.anchor.clone(),
            quote: text[start..end].into(),
        },
        text[from..to].into(),
    ))
}
fn floor_boundary(text: &str, mut end: usize) -> usize {
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    end
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
