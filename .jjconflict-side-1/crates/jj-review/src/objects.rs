use crate::{
    ArtifactAnchorV1, Error, Result, ReviewCommentBodyV1, ReviewCommentConflictV1,
    ReviewCommentRevisionV1, ReviewEventGraphV1, ReviewEventPageV1, ReviewIndexPageV1,
    ReviewObjectId, ReviewRelationPageV1, ReviewStatusPageV1, ReviewThreadEventV1,
    ReviewThreadMergeV1, ReviewThreadSnapshotV1,
};
use jj_agent::AgentObjectRef;
use jj_store_model::ObjectKey;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ReviewObjectKind {
    IndexPage,
    Anchor,
    CommentBody,
    CommentRevision,
    ThreadEvent,
    EventPage,
    ThreadSnapshot,
    RelationPage,
    StatusPage,
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct ReviewObjectRef {
    pub kind: ReviewObjectKind,
    pub id: ReviewObjectId,
}
impl ReviewObjectRef {
    pub const fn new(kind: ReviewObjectKind, id: ReviewObjectId) -> Self {
        Self { kind, id }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ReviewObjectV1 {
    IndexPage(ReviewIndexPageV1),
    Anchor(Box<ArtifactAnchorV1>),
    CommentBody(ReviewCommentBodyV1),
    CommentRevision(Box<ReviewCommentRevisionV1>),
    CommentConflict(Box<ReviewCommentConflictV1>),
    ThreadEvent(Box<ReviewThreadEventV1>),
    EventPage(ReviewEventPageV1),
    EventGraph(ReviewEventGraphV1),
    ThreadSnapshot(Box<ReviewThreadSnapshotV1>),
    ThreadMerge(Box<ReviewThreadMergeV1>),
    RelationPage(ReviewRelationPageV1),
    StatusPage(ReviewStatusPageV1),
}
impl ReviewObjectV1 {
    pub const fn kind(&self) -> ReviewObjectKind {
        match self {
            Self::IndexPage(_) => ReviewObjectKind::IndexPage,
            Self::Anchor(_) => ReviewObjectKind::Anchor,
            Self::CommentBody(_) => ReviewObjectKind::CommentBody,
            Self::CommentRevision(_) => ReviewObjectKind::CommentRevision,
            Self::CommentConflict(_) => ReviewObjectKind::CommentRevision,
            Self::ThreadEvent(_) => ReviewObjectKind::ThreadEvent,
            Self::EventPage(_) => ReviewObjectKind::EventPage,
            Self::EventGraph(_) => ReviewObjectKind::EventPage,
            Self::ThreadSnapshot(_) => ReviewObjectKind::ThreadSnapshot,
            Self::ThreadMerge(_) => ReviewObjectKind::ThreadSnapshot,
            Self::RelationPage(_) => ReviewObjectKind::RelationPage,
            Self::StatusPage(_) => ReviewObjectKind::StatusPage,
        }
    }
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::IndexPage(value) => value.validate(),
            Self::Anchor(value) => value.validate(),
            Self::CommentBody(value) => value.validate(),
            Self::CommentRevision(value) => value.validate(),
            Self::CommentConflict(value) => value.validate(),
            Self::ThreadEvent(value) => value.validate(),
            Self::EventPage(value) => value.validate(),
            Self::EventGraph(value) => value.validate(),
            Self::ThreadSnapshot(value) => value.validate(),
            Self::ThreadMerge(value) => value.validate(),
            Self::RelationPage(value) => value.validate(),
            Self::StatusPage(value) => value.validate(),
        }
    }
    pub fn review_dependencies(&self) -> Result<Vec<ReviewObjectRef>> {
        self.validate()?;
        Ok(match self {
            Self::IndexPage(value) => value.dependencies(),
            Self::Anchor(_) | Self::CommentBody(_) => Vec::new(),
            Self::CommentRevision(value) => value.review_dependencies()?.into_iter().collect(),
            Self::CommentConflict(value) => value.review_dependencies()?.into_iter().collect(),
            Self::ThreadEvent(value) => value.review_dependencies()?.into_iter().collect(),
            Self::EventPage(value) => value.review_dependencies()?.into_iter().collect(),
            Self::EventGraph(value) => value.review_dependencies()?.into_iter().collect(),
            Self::ThreadSnapshot(value) => value.review_dependencies()?.into_iter().collect(),
            Self::ThreadMerge(value) => value.review_dependencies()?.into_iter().collect(),
            Self::RelationPage(value) => value.dependencies().into_iter().collect(),
            Self::StatusPage(value) => value.dependencies().into_iter().collect(),
        })
    }
    pub fn native_dependencies(&self) -> Result<BTreeSet<ObjectKey>> {
        self.validate()?;
        match self {
            Self::Anchor(value) => value.native_dependencies(),
            _ => Ok(BTreeSet::new()),
        }
    }
    pub fn agent_dependencies(&self) -> Result<BTreeSet<AgentObjectRef>> {
        self.validate()?;
        match self {
            Self::Anchor(value) => value.agent_dependencies(),
            _ => Ok(BTreeSet::new()),
        }
    }
}

pub(crate) fn expect_kind(reference: ReviewObjectRef, kind: ReviewObjectKind) -> Result<()> {
    if reference.kind == kind {
        Ok(())
    } else {
        Err(Error::Invalid("review object kind"))
    }
}
