use jj_store_model as m;
use jj_store_wal::publication::{
    ArtifactAnchorV1, CommentId, ReviewActionId, ReviewBodyFormatV1, ReviewObjectId,
    ReviewObjectRef, ReviewThreadEventKindV1, ReviewThreadId, ReviewThreadMergeV1,
    ReviewThreadSnapshotV1, ThreadStatusV1,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub type ReviewThreadIdentity = ReviewThreadId;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReviewReadSnapshot {
    pub repo_id: m::RepositoryId,
    pub epoch: m::EpochId,
    pub published_seq: m::WalPosition,
    pub manifest_revision: u64,
    pub review_root: m::RecordDigest,
    pub review_state_version: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReviewThreadSummary {
    pub thread_id: ReviewThreadId,
    pub snapshot_id: ReviewObjectId,
    pub original_anchor_id: ReviewObjectId,
    pub anchor: ArtifactAnchorV1,
    pub event_page_root_id: ReviewObjectId,
    pub status: ThreadStatusV1,
    pub selected_workflow_action: Option<ReviewActionId>,
    pub selected_workflow_actions: Vec<ReviewActionId>,
    pub thread_version: u64,
    pub content_version: u64,
    pub workflow_version: u64,
    pub comment_count: usize,
    pub comment_conflict_count: usize,
    pub last_event_sequence: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation: Option<ReviewSummaryPresentation>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReviewSummaryPresentation {
    pub created_by: m::PrincipalId,
    pub created_at_millis: i64,
    pub updated_at_millis: i64,
    pub excerpt: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReviewThreadPage {
    pub version: u32,
    pub snapshot: ReviewReadSnapshot,
    pub threads: Vec<ReviewThreadSummary>,
    pub next_cursor: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReviewAnchor {
    pub object_id: ReviewObjectId,
    pub value: ArtifactAnchorV1,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum ReviewCommentContent {
    Visible {
        body_id: ReviewObjectId,
        format: ReviewBodyFormatV1,
        text: String,
        sanitizer_version: String,
    },
    Tombstoned {
        reason: Option<String>,
    },
    Hidden {
        reason: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReviewComment {
    pub comment_id: CommentId,
    pub revision_id: ReviewObjectId,
    pub parent_comment_id: Option<CommentId>,
    pub author: m::PrincipalId,
    pub actor: m::PrincipalId,
    pub created_at_millis: i64,
    pub previous_revision_id: Option<ReviewObjectId>,
    pub superseded_revision_ids: Vec<ReviewObjectId>,
    pub content: ReviewCommentContent,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReviewCommentVersion {
    pub revision_id: ReviewObjectId,
    pub author: m::PrincipalId,
    pub actor: m::PrincipalId,
    pub created_at_millis: i64,
    pub previous_revision_id: Option<ReviewObjectId>,
    pub superseded_revision_ids: Vec<ReviewObjectId>,
    pub content: ReviewCommentContent,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReviewCommentConflict {
    pub comment_id: CommentId,
    pub parent_comment_id: Option<CommentId>,
    pub alternatives: Vec<ReviewCommentVersion>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum ReviewEventKind {
    Created {
        anchor_id: ReviewObjectId,
        initial_comment_id: CommentId,
        initial_revision_id: ReviewObjectId,
    },
    ReplyAdded {
        comment_id: CommentId,
        parent_comment_id: Option<CommentId>,
        revision_id: ReviewObjectId,
    },
    CommentRevised {
        comment_id: CommentId,
        previous_revision_id: ReviewObjectId,
        revision_id: ReviewObjectId,
    },
    CommentConflictResolved {
        comment_id: CommentId,
        superseded_revision_ids: Vec<ReviewObjectId>,
        revision_id: ReviewObjectId,
    },
    Resolved {
        action_id: ReviewActionId,
        reason: Option<String>,
    },
    Reopened {
        action_id: ReviewActionId,
        reason: Option<String>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReviewEvent {
    pub event_id: ReviewObjectId,
    pub causal_parent_ids: Vec<ReviewObjectId>,
    pub sequence: u64,
    pub actor: m::PrincipalId,
    pub accepted_at_millis: i64,
    pub event: ReviewEventKind,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReviewThreadDetail {
    pub version: u32,
    pub snapshot: ReviewReadSnapshot,
    pub thread: ReviewThreadSummary,
    pub anchor: ReviewAnchor,
    pub comments: Vec<ReviewComment>,
    pub comment_conflicts: Vec<ReviewCommentConflict>,
    pub events: Vec<ReviewEvent>,
}

pub(super) fn summary(
    reference: ReviewObjectRef,
    value: &ReviewThreadSnapshotV1,
    anchor: ArtifactAnchorV1,
) -> ReviewThreadSummary {
    ReviewThreadSummary {
        thread_id: value.thread_id,
        snapshot_id: reference.id,
        original_anchor_id: value.original_anchor.id,
        anchor,
        event_page_root_id: value.event_page_root.id,
        status: value.status,
        selected_workflow_action: value.selected_workflow_action,
        selected_workflow_actions: value.selected_workflow_action.into_iter().collect(),
        thread_version: value.thread_version,
        content_version: value.content_version,
        workflow_version: value.workflow_version,
        comment_count: value.comments.len(),
        comment_conflict_count: 0,
        last_event_sequence: value.last_event_sequence,
        presentation: None,
    }
}

pub(super) fn merged_summary(
    reference: ReviewObjectRef,
    value: &ReviewThreadMergeV1,
    anchor: ArtifactAnchorV1,
    comment_conflict_count: usize,
) -> ReviewThreadSummary {
    ReviewThreadSummary {
        thread_id: value.thread_id,
        snapshot_id: reference.id,
        original_anchor_id: value.original_anchor.id,
        anchor,
        event_page_root_id: value.event_graph_root.id,
        status: value.status,
        selected_workflow_action: value.selected_workflow_actions.first().copied(),
        selected_workflow_actions: value.selected_workflow_actions.clone(),
        thread_version: value.thread_version,
        content_version: value.content_version,
        workflow_version: value.workflow_version,
        comment_count: value.comments.len(),
        comment_conflict_count,
        last_event_sequence: value.thread_version,
        presentation: None,
    }
}

pub(super) fn event_kind(value: ReviewThreadEventKindV1) -> ReviewEventKind {
    match value {
        ReviewThreadEventKindV1::Created {
            anchor,
            initial_comment_id,
            initial_revision,
        } => ReviewEventKind::Created {
            anchor_id: anchor.id,
            initial_comment_id,
            initial_revision_id: initial_revision.id,
        },
        ReviewThreadEventKindV1::ReplyAdded {
            comment_id,
            parent_comment_id,
            revision,
        } => ReviewEventKind::ReplyAdded {
            comment_id,
            parent_comment_id,
            revision_id: revision.id,
        },
        ReviewThreadEventKindV1::CommentRevised {
            comment_id,
            previous_revision,
            revision,
        } => ReviewEventKind::CommentRevised {
            comment_id,
            previous_revision_id: previous_revision.id,
            revision_id: revision.id,
        },
        ReviewThreadEventKindV1::CommentConflictResolved {
            comment_id,
            superseded_revisions,
            revision,
        } => ReviewEventKind::CommentConflictResolved {
            comment_id,
            superseded_revision_ids: superseded_revisions
                .into_iter()
                .map(|reference| reference.id)
                .collect(),
            revision_id: revision.id,
        },
        ReviewThreadEventKindV1::Resolved { action_id, reason } => {
            ReviewEventKind::Resolved { action_id, reason }
        }
        ReviewThreadEventKindV1::Reopened { action_id, reason } => {
            ReviewEventKind::Reopened { action_id, reason }
        }
    }
}
