use crate::{
    CommentId, CommentSelectionV1, Error, Result, ReviewActionId, ReviewObjectKind,
    ReviewObjectRef, ReviewThreadId, ThreadStatusV1, expect_kind,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

const MAX_REVIEW_FACTS: usize = 4096;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReviewCommentConflictV1 {
    pub schema_version: u32,
    pub thread_id: ReviewThreadId,
    pub comment_id: CommentId,
    pub alternatives: Vec<ReviewObjectRef>,
}

impl ReviewCommentConflictV1 {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 1
            || self.alternatives.len() < 2
            || self.alternatives.len() > 64
            || !strictly_sorted(&self.alternatives)
        {
            return Err(Error::Invalid("review comment conflict"));
        }
        for alternative in &self.alternatives {
            expect_kind(*alternative, ReviewObjectKind::CommentRevision)?;
        }
        Ok(())
    }

    pub fn review_dependencies(&self) -> Result<BTreeSet<ReviewObjectRef>> {
        self.validate()?;
        Ok(self.alternatives.iter().copied().collect())
    }
}

#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, schemars::JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct ReviewEventNodeV1 {
    pub event: ReviewObjectRef,
    pub parents: Vec<ReviewObjectRef>,
}

impl ReviewEventNodeV1 {
    fn validate(&self) -> Result<()> {
        expect_kind(self.event, ReviewObjectKind::ThreadEvent)?;
        if self.parents.len() > MAX_REVIEW_FACTS || !strictly_sorted(&self.parents) {
            return Err(Error::Invalid("review event graph parents"));
        }
        for parent in &self.parents {
            expect_kind(*parent, ReviewObjectKind::ThreadEvent)?;
            if *parent == self.event {
                return Err(Error::Invalid("review event graph self parent"));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReviewEventGraphV1 {
    pub schema_version: u32,
    pub thread_id: ReviewThreadId,
    pub nodes: Vec<ReviewEventNodeV1>,
    pub heads: Vec<ReviewObjectRef>,
}

impl ReviewEventGraphV1 {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 1
            || self.nodes.is_empty()
            || self.nodes.len() > MAX_REVIEW_FACTS
            || self.heads.is_empty()
            || self.heads.len() > MAX_REVIEW_FACTS
            || !self
                .nodes
                .windows(2)
                .all(|pair| pair[0].event < pair[1].event)
            || !strictly_sorted(&self.heads)
        {
            return Err(Error::Invalid("review event graph"));
        }
        let events = self
            .nodes
            .iter()
            .map(|node| node.event)
            .collect::<BTreeSet<_>>();
        for node in &self.nodes {
            node.validate()?;
            if node.parents.iter().any(|parent| !events.contains(parent)) {
                return Err(Error::Invalid("review event graph parent"));
            }
        }
        let selected_heads = events
            .iter()
            .copied()
            .filter(|event| {
                !self
                    .nodes
                    .iter()
                    .any(|node| node.parents.binary_search(event).is_ok())
            })
            .collect::<Vec<_>>();
        if selected_heads != self.heads {
            return Err(Error::Invalid("review event graph heads"));
        }
        Ok(())
    }

    pub fn review_dependencies(&self) -> Result<BTreeSet<ReviewObjectRef>> {
        self.validate()?;
        Ok(self.nodes.iter().map(|node| node.event).collect())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReviewThreadMergeV1 {
    pub schema_version: u32,
    pub thread_id: ReviewThreadId,
    pub original_anchor: ReviewObjectRef,
    pub comments: Vec<CommentSelectionV1>,
    pub event_graph_root: ReviewObjectRef,
    pub status: ThreadStatusV1,
    pub selected_workflow_actions: Vec<ReviewActionId>,
    pub thread_version: u64,
    pub content_version: u64,
    pub workflow_version: u64,
}

impl ReviewThreadMergeV1 {
    pub fn validate(&self) -> Result<()> {
        expect_kind(self.original_anchor, ReviewObjectKind::Anchor)?;
        expect_kind(self.event_graph_root, ReviewObjectKind::EventPage)?;
        if self.schema_version != 1
            || self.comments.is_empty()
            || self.comments.len() > MAX_REVIEW_FACTS
            || self.thread_version == 0
            || self.content_version == 0
            || self.content_version > self.thread_version
            || self.workflow_version == 0
            || self.workflow_version > self.thread_version.saturating_add(1)
            || (self.status == ThreadStatusV1::Resolved)
                != !self.selected_workflow_actions.is_empty()
            || !strictly_sorted(&self.selected_workflow_actions)
        {
            return Err(Error::Invalid("review merged thread"));
        }
        let mut comments = BTreeSet::new();
        for selection in &self.comments {
            expect_kind(selection.revision, ReviewObjectKind::CommentRevision)?;
            if !comments.insert(selection.comment_id)
                || selection
                    .parent_comment_id
                    .is_some_and(|parent| !comments.contains(&parent))
            {
                return Err(Error::Invalid("review merged comment order"));
            }
        }
        Ok(())
    }

    pub fn review_dependencies(&self) -> Result<BTreeSet<ReviewObjectRef>> {
        self.validate()?;
        Ok(self
            .comments
            .iter()
            .map(|selection| selection.revision)
            .chain([self.original_anchor, self.event_graph_root])
            .collect())
    }
}

fn strictly_sorted<T: Ord>(values: &[T]) -> bool {
    values.windows(2).all(|pair| pair[0] < pair[1])
}
