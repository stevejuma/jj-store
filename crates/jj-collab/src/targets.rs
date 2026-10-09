use crate::*;
use jj_agent::{
    AgentCheckpointId, AgentEventId, AgentObjectKind, AgentObjectRef, AgentSessionId, MessageId,
    ToolCallId,
};
use jj_review::{CommentId, ReviewObjectKind, ReviewObjectRef, ReviewThreadId};
use jj_work::{TaskId, WorkCommentId};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Identity, immutable content and validation witnesses are explicit, never a UI position.
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, schemars::JsonSchema,
)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum ReactionTargetV1 {
    Thread {
        thread_id: ThreadId,
    },
    HumanMessage {
        thread_id: ThreadId,
        message_id: HumanMessageId,
        revision: CollabObjectRef,
    },
    AgentMessage {
        session_id: AgentSessionId,
        message_id: MessageId,
        event_id: AgentEventId,
        batch: AgentObjectRef,
        content: AgentObjectRef,
    },
    ToolInvocation {
        session_id: AgentSessionId,
        tool_id: ToolCallId,
        snapshot: AgentObjectRef,
    },
    ToolOutput {
        session_id: AgentSessionId,
        tool_id: ToolCallId,
        snapshot: AgentObjectRef,
        content: AgentObjectRef,
    },
    ReviewThread {
        thread_id: ReviewThreadId,
    },
    ReviewComment {
        thread_id: ReviewThreadId,
        comment_id: CommentId,
        revision: ReviewObjectRef,
    },
    /// An independently authorized ReviewStore. Its objects are not dependencies
    /// in the conversation repository's local review graph.
    RepositoryReviewComment {
        repository: jj_store_model::RepositoryId,
        thread_id: ReviewThreadId,
        comment_id: CommentId,
        revision: ReviewObjectRef,
    },
    WorkComment {
        task_id: TaskId,
        comment_id: WorkCommentId,
        version: u64,
    },
    AgentCheckpoint {
        session_id: AgentSessionId,
        checkpoint_id: AgentCheckpointId,
        version: AgentObjectRef,
    },
}
impl ReactionTargetV1 {
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::WorkComment { version: 0, .. } => {
                return Err(Error::Invalid("work comment target version"));
            }
            Self::HumanMessage { revision, .. } => {
                expect(*revision, CollabObjectKind::HumanMessageRevision)?
            }
            Self::AgentMessage { batch, content, .. } => {
                agent_kind(*batch, AgentObjectKind::EventBatch)?;
                agent_kind(*content, AgentObjectKind::Content)?;
            }
            Self::ToolInvocation { snapshot, .. } => {
                agent_kind(*snapshot, AgentObjectKind::ToolSnapshot)?
            }
            Self::ToolOutput {
                snapshot, content, ..
            } => {
                agent_kind(*snapshot, AgentObjectKind::ToolSnapshot)?;
                agent_kind(*content, AgentObjectKind::Content)?;
            }
            Self::ReviewComment { revision, .. }
            | Self::RepositoryReviewComment { revision, .. }
                if revision.kind != ReviewObjectKind::CommentRevision =>
            {
                return Err(Error::Invalid("review revision target"));
            }
            Self::AgentCheckpoint { version, .. } => {
                agent_kind(*version, AgentObjectKind::CheckpointVersion)?
            }
            _ => {}
        }
        Ok(())
    }
    /// A validation witness can change without changing the exact reacted-to content.
    pub fn exact_key(&self) -> Result<CollabCatalogKey> {
        self.validate()?;
        let identity = match self {
            Self::AgentMessage {
                session_id,
                message_id,
                content,
                ..
            } => serde_json::to_vec(&("agent_message", session_id, message_id, content))?,
            _ => serde_json::to_vec(self)?,
        };
        CollabCatalogKey::derive("reaction-target", &identity)
    }
    pub fn collab_dependencies(&self) -> Vec<CollabObjectRef> {
        match self {
            Self::HumanMessage { revision, .. } => vec![*revision],
            _ => Vec::new(),
        }
    }
    pub fn agent_dependencies(&self) -> BTreeSet<AgentObjectRef> {
        match self {
            Self::AgentMessage { batch, content, .. } => BTreeSet::from([*batch, *content]),
            Self::ToolInvocation { snapshot, .. } => BTreeSet::from([*snapshot]),
            Self::ToolOutput {
                snapshot, content, ..
            } => BTreeSet::from([*snapshot, *content]),
            Self::AgentCheckpoint { version, .. } => BTreeSet::from([*version]),
            _ => BTreeSet::new(),
        }
    }
    pub fn review_dependencies(&self) -> BTreeSet<ReviewObjectRef> {
        match self {
            Self::ReviewComment { revision, .. } => BTreeSet::from([*revision]),
            _ => BTreeSet::new(),
        }
    }
}
fn agent_kind(reference: AgentObjectRef, kind: AgentObjectKind) -> Result<()> {
    if reference.kind == kind {
        Ok(())
    } else {
        Err(Error::Invalid("agent target kind"))
    }
}
