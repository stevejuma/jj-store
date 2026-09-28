use crate::{
    ArtifactAnchorV1, CommentId, CommentRevisionId, Error, Result, ReviewActionId,
    ReviewCommentBodyV1, ReviewObjectKind, ReviewObjectRef, ReviewThreadId, ThreadSnapshotId,
    expect_kind, valid_label,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "command", rename_all = "snake_case", deny_unknown_fields)]
pub enum ReviewCommandV1 {
    CreateThread {
        anchor: Box<ArtifactAnchorV1>,
        body: ReviewCommentBodyV1,
    },
    AppendReply {
        thread_id: ReviewThreadId,
        expected_workflow_version: u64,
        parent_comment_id: Option<CommentId>,
        body: ReviewCommentBodyV1,
    },
    EditComment {
        comment_id: CommentId,
        expected_revision_id: CommentRevisionId,
        replacement_body: ReviewCommentBodyV1,
    },
    TombstoneComment {
        comment_id: CommentId,
        expected_revision_id: CommentRevisionId,
        reason: Option<String>,
    },
    HideComment {
        comment_id: CommentId,
        expected_revision_id: CommentRevisionId,
        moderation_reason: String,
    },
    ResolveThread {
        thread_id: ReviewThreadId,
        expected_thread_version: ThreadSnapshotId,
        reason: Option<String>,
    },
    ReopenThread {
        thread_id: ReviewThreadId,
        expected_thread_version: ThreadSnapshotId,
        reason: Option<String>,
    },
}

impl ReviewCommandV1 {
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::CreateThread { anchor, body } => {
                anchor.validate()?;
                body.validate()
            }
            Self::AppendReply {
                expected_workflow_version,
                body,
                ..
            } => {
                if *expected_workflow_version == 0 {
                    return Err(Error::Invalid("review workflow version"));
                }
                body.validate()
            }
            Self::EditComment {
                replacement_body, ..
            } => replacement_body.validate(),
            Self::TombstoneComment { reason, .. }
            | Self::ResolveThread { reason, .. }
            | Self::ReopenThread { reason, .. } => validate_reason(reason.as_deref(), false),
            Self::HideComment {
                moderation_reason, ..
            } => validate_reason(Some(moderation_reason), true),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReviewCommandResultV1 {
    pub schema_version: u32,
    pub thread_id: ReviewThreadId,
    pub comment_id: Option<CommentId>,
    pub action_id: Option<ReviewActionId>,
    pub thread_snapshot: ReviewObjectRef,
    pub comment_revision: Option<ReviewObjectRef>,
    pub event: ReviewObjectRef,
}

impl ReviewCommandResultV1 {
    pub fn validate(&self) -> Result<()> {
        expect_kind(self.thread_snapshot, ReviewObjectKind::ThreadSnapshot)?;
        expect_kind(self.event, ReviewObjectKind::ThreadEvent)?;
        if let Some(revision) = self.comment_revision {
            expect_kind(revision, ReviewObjectKind::CommentRevision)?;
        }
        let comment_result = self.comment_id.is_some()
            && self.comment_revision.is_some()
            && self.action_id.is_none();
        let workflow_result = self.comment_id.is_none()
            && self.comment_revision.is_none()
            && self.action_id.is_some();
        if self.schema_version != 1 || !(comment_result || workflow_result) {
            return Err(Error::Invalid("review command result"));
        }
        Ok(())
    }
}

fn validate_reason(reason: Option<&str>, required: bool) -> Result<()> {
    if (required && reason.is_none()) || reason.is_some_and(|value| !valid_label(value, 4096)) {
        Err(Error::Invalid("review command reason"))
    } else {
        Ok(())
    }
}
