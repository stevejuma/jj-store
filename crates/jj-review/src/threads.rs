use crate::{
    CommentId, Error, Result, ReviewActionId, ReviewEventId, ReviewObjectKind, ReviewObjectRef,
    ReviewThreadId, ThreadSnapshotId, expect_kind, valid_label,
};
use jj_store_model::PrincipalId;
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub enum ReviewThreadEventKindV1 {
    Created {
        anchor: ReviewObjectRef,
        initial_comment_id: CommentId,
        initial_revision: ReviewObjectRef,
    },
    ReplyAdded {
        comment_id: CommentId,
        parent_comment_id: Option<CommentId>,
        revision: ReviewObjectRef,
    },
    CommentRevised {
        comment_id: CommentId,
        previous_revision: ReviewObjectRef,
        revision: ReviewObjectRef,
    },
    CommentConflictResolved {
        comment_id: CommentId,
        superseded_revisions: Vec<ReviewObjectRef>,
        revision: ReviewObjectRef,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReviewThreadEventV1 {
    pub schema_version: u32,
    pub thread_id: ReviewThreadId,
    pub sequence: u64,
    pub actor: PrincipalId,
    pub accepted_at_millis: i64,
    pub event: ReviewThreadEventKindV1,
}
impl ReviewThreadEventV1 {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 1 || self.sequence == 0 {
            return Err(Error::Invalid("review thread event"));
        }
        match &self.event {
            ReviewThreadEventKindV1::Created {
                anchor,
                initial_revision,
                ..
            } => {
                if self.sequence != 1 {
                    return Err(Error::Invalid("review thread creation sequence"));
                }
                expect_kind(*anchor, ReviewObjectKind::Anchor)?;
                expect_kind(*initial_revision, ReviewObjectKind::CommentRevision)?;
            }
            ReviewThreadEventKindV1::ReplyAdded { revision, .. } => {
                if self.sequence == 1 {
                    return Err(Error::Invalid("review reply sequence"));
                }
                expect_kind(*revision, ReviewObjectKind::CommentRevision)?;
            }
            ReviewThreadEventKindV1::CommentRevised {
                previous_revision,
                revision,
                ..
            } => {
                if self.sequence == 1 || previous_revision == revision {
                    return Err(Error::Invalid("review comment event"));
                }
                expect_kind(*previous_revision, ReviewObjectKind::CommentRevision)?;
                expect_kind(*revision, ReviewObjectKind::CommentRevision)?;
            }
            ReviewThreadEventKindV1::CommentConflictResolved {
                superseded_revisions,
                revision,
                ..
            } => {
                if self.sequence == 1
                    || superseded_revisions.len() < 2
                    || superseded_revisions.len() > 64
                    || superseded_revisions
                        .windows(2)
                        .any(|pair| pair[0] >= pair[1])
                {
                    return Err(Error::Invalid("review comment conflict resolution"));
                }
                for reference in superseded_revisions {
                    expect_kind(*reference, ReviewObjectKind::CommentRevision)?;
                }
                expect_kind(*revision, ReviewObjectKind::CommentRevision)?;
            }
            ReviewThreadEventKindV1::Resolved { reason, .. }
            | ReviewThreadEventKindV1::Reopened { reason, .. } => {
                if self.sequence == 1
                    || reason
                        .as_deref()
                        .is_some_and(|value| !valid_label(value, 4096))
                {
                    return Err(Error::Invalid("review workflow event"));
                }
            }
        }
        Ok(())
    }
    pub fn review_dependencies(&self) -> Result<BTreeSet<ReviewObjectRef>> {
        self.validate()?;
        Ok(match self.event {
            ReviewThreadEventKindV1::Created {
                anchor,
                initial_revision,
                ..
            } => [anchor, initial_revision].into_iter().collect(),
            ReviewThreadEventKindV1::ReplyAdded { revision, .. } => {
                [revision].into_iter().collect()
            }
            ReviewThreadEventKindV1::CommentRevised {
                previous_revision,
                revision,
                ..
            } => [previous_revision, revision].into_iter().collect(),
            ReviewThreadEventKindV1::CommentConflictResolved {
                ref superseded_revisions,
                revision,
                ..
            } => superseded_revisions
                .iter()
                .copied()
                .chain([revision])
                .collect(),
            ReviewThreadEventKindV1::Resolved { .. } | ReviewThreadEventKindV1::Reopened { .. } => {
                BTreeSet::new()
            }
        })
    }
    pub fn event_id(&self) -> Result<ReviewEventId> {
        let object = crate::ReviewObjectV1::ThreadEvent(Box::new(self.clone()));
        Ok(ReviewEventId::new(
            Sha256::digest(crate::encode_object(&object)?).into(),
        ))
    }
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, schemars::JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct ReviewEventSelectionV1 {
    pub sequence: u64,
    pub event: ReviewObjectRef,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReviewEventPageV1 {
    pub schema_version: u32,
    pub thread_id: ReviewThreadId,
    pub entries: Vec<ReviewEventSelectionV1>,
    pub next: Option<ReviewObjectRef>,
}
impl ReviewEventPageV1 {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 1 || self.entries.is_empty() || self.entries.len() > 256 {
            return Err(Error::Invalid("review event page"));
        }
        let mut prior = None;
        let mut events = BTreeSet::new();
        for entry in &self.entries {
            expect_kind(entry.event, ReviewObjectKind::ThreadEvent)?;
            if entry.sequence == 0
                || prior.is_some_and(|value| entry.sequence != value + 1)
                || !events.insert(entry.event)
            {
                return Err(Error::Invalid("review event page order"));
            }
            prior = Some(entry.sequence);
        }
        if let Some(next) = self.next {
            expect_kind(next, ReviewObjectKind::EventPage)?;
        }
        Ok(())
    }
    pub fn review_dependencies(&self) -> Result<BTreeSet<ReviewObjectRef>> {
        self.validate()?;
        Ok(self
            .entries
            .iter()
            .map(|value| value.event)
            .chain(self.next)
            .collect())
    }
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ThreadStatusV1 {
    Open,
    Resolved,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CommentSelectionV1 {
    pub comment_id: CommentId,
    pub revision: ReviewObjectRef,
    pub parent_comment_id: Option<CommentId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReviewThreadSnapshotV1 {
    pub schema_version: u32,
    pub thread_id: ReviewThreadId,
    pub original_anchor: ReviewObjectRef,
    pub comments: Vec<CommentSelectionV1>,
    pub event_page_root: ReviewObjectRef,
    pub last_event_sequence: u64,
    pub status: ThreadStatusV1,
    pub selected_workflow_action: Option<ReviewActionId>,
    pub thread_version: u64,
    pub content_version: u64,
    pub workflow_version: u64,
}
impl ReviewThreadSnapshotV1 {
    pub fn validate(&self) -> Result<()> {
        expect_kind(self.original_anchor, ReviewObjectKind::Anchor)?;
        expect_kind(self.event_page_root, ReviewObjectKind::EventPage)?;
        if self.schema_version != 1
            || self.comments.is_empty()
            || self.comments.len() > 4096
            || self.thread_version == 0
            || self.last_event_sequence != self.thread_version
            || self.content_version == 0
            || self.content_version > self.thread_version
            || self.workflow_version == 0
            || self.workflow_version > self.thread_version
            || (self.status == ThreadStatusV1::Resolved) != self.selected_workflow_action.is_some()
        {
            return Err(Error::Invalid("review thread snapshot"));
        }
        let mut comments = BTreeSet::new();
        for selection in &self.comments {
            expect_kind(selection.revision, ReviewObjectKind::CommentRevision)?;
            if selection
                .parent_comment_id
                .is_some_and(|parent| !comments.contains(&parent))
                || !comments.insert(selection.comment_id)
            {
                return Err(Error::Invalid("review comment order"));
            }
        }
        Ok(())
    }
    pub fn review_dependencies(&self) -> Result<BTreeSet<ReviewObjectRef>> {
        self.validate()?;
        Ok(self
            .comments
            .iter()
            .map(|value| value.revision)
            .chain([self.original_anchor, self.event_page_root])
            .collect())
    }
    pub fn snapshot_id(&self) -> Result<ThreadSnapshotId> {
        let object = crate::ReviewObjectV1::ThreadSnapshot(Box::new(self.clone()));
        Ok(ThreadSnapshotId::new(
            Sha256::digest(crate::encode_object(&object)?).into(),
        ))
    }
}
