//! One target policy for authorized, pinned native and local read providers.
use crate::*;
use jj_agent::{AgentCheckpointId, AgentObjectRef, AgentObjectV1, AgentSessionId};
use jj_review::{CommentRevisionContentV1, ReviewObjectRef, ReviewObjectV1, ReviewThreadId};
use std::collections::BTreeSet;

pub const MAX_TARGET_RESOLUTION_OBJECTS: usize = 4096;

/// One authorized repository/root selection. None means absent or inaccessible.
/// Object reads MUST verify the owning store's canonical repository-bound
/// identity, semantic kind and captured root closure membership. They never
/// read staged objects. Conflicted selections are not resolved by time.
#[async_trait::async_trait]
pub trait ReactionTargetRead: Send + Sync {
    async fn thread_selection(&self, id: ThreadId) -> Result<Option<CollabObjectRef>>;
    async fn message_selection(
        &self,
        thread: ThreadId,
        message: HumanMessageId,
    ) -> Result<Option<CollabObjectRef>>;
    async fn collab_object(&self, reference: CollabObjectRef) -> Result<Option<CollabObjectV1>>;
    async fn session_selection(&self, id: AgentSessionId) -> Result<Option<AgentObjectRef>>;
    async fn checkpoint_selection(&self, id: AgentCheckpointId) -> Result<Option<AgentObjectRef>>;
    async fn agent_object(&self, reference: AgentObjectRef) -> Result<Option<AgentObjectV1>>;
    async fn review_selection(&self, id: ReviewThreadId) -> Result<Option<ReviewObjectRef>>;
    async fn review_object(&self, reference: ReviewObjectRef) -> Result<Option<ReviewObjectV1>>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TargetRevisionV1 {
    Current,
    Earlier,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResolvedReactionTargetV1 {
    pub exact_key: CollabCatalogKey,
    pub revision: TargetRevisionV1,
}

pub async fn resolve_reaction_target(
    reader: &dyn ReactionTargetRead,
    target: &ReactionTargetV1,
) -> Result<ResolvedReactionTargetV1> {
    target.validate()?;
    let mut resolver = Resolver { reader, reads: 0 };
    let revision = resolver.resolve(target).await?;
    Ok(ResolvedReactionTargetV1 {
        exact_key: target.exact_key()?,
        revision,
    })
}

struct Resolver<'a> {
    reader: &'a dyn ReactionTargetRead,
    reads: usize,
}
impl Resolver<'_> {
    fn admit(&mut self) -> Result<()> {
        self.reads += 1;
        if self.reads > MAX_TARGET_RESOLUTION_OBJECTS {
            Err(Error::Limit)
        } else {
            Ok(())
        }
    }
    async fn collab(&mut self, reference: CollabObjectRef) -> Result<CollabObjectV1> {
        self.admit()?;
        let object = self
            .reader
            .collab_object(reference)
            .await?
            .ok_or(Error::TargetUnavailable)?;
        if object.reference()? != reference {
            return Err(Error::Invalid("target object identity"));
        }
        Ok(object)
    }
    async fn agent(&mut self, reference: AgentObjectRef) -> Result<AgentObjectV1> {
        self.admit()?;
        let object = self
            .reader
            .agent_object(reference)
            .await?
            .ok_or(Error::TargetUnavailable)?;
        object
            .validate()
            .map_err(|_| Error::Invalid("agent target object"))?;
        if object.kind() != reference.kind {
            return Err(Error::Invalid("agent target kind"));
        }
        Ok(object)
    }
    async fn review(&mut self, reference: ReviewObjectRef) -> Result<ReviewObjectV1> {
        self.admit()?;
        let object = self
            .reader
            .review_object(reference)
            .await?
            .ok_or(Error::TargetUnavailable)?;
        object
            .validate()
            .map_err(|_| Error::Invalid("review target object"))?;
        if object.kind() != reference.kind {
            return Err(Error::Invalid("review target kind"));
        }
        Ok(object)
    }
    async fn thread(&mut self, id: ThreadId) -> Result<ThreadVersionV1> {
        let reference = self
            .reader
            .thread_selection(id)
            .await?
            .ok_or(Error::TargetUnavailable)?;
        let CollabObjectV1::ThreadVersion(thread) = self.collab(reference).await? else {
            return Err(Error::Invalid("selected Thread kind"));
        };
        if thread.thread_id != id {
            return Err(Error::Invalid("selected Thread identity"));
        }
        if thread.status == ThreadStatusV1::Tombstoned {
            return Err(Error::TargetUnavailable);
        }
        Ok(thread)
    }
    async fn session(&mut self, id: AgentSessionId) -> Result<jj_agent::AgentSessionSnapshotV1> {
        let reference = self
            .reader
            .session_selection(id)
            .await?
            .ok_or(Error::TargetUnavailable)?;
        let session = match self.agent(reference).await? {
            AgentObjectV1::SessionSnapshot(session) => session,
            AgentObjectV1::CatalogConflict(_) => return Err(Error::TargetUnavailable),
            _ => return Err(Error::Invalid("selected session kind")),
        };
        if session.session_id != id {
            return Err(Error::Invalid("selected session identity"));
        }
        Ok(session)
    }
    async fn resolve(&mut self, target: &ReactionTargetV1) -> Result<TargetRevisionV1> {
        use TargetRevisionV1::{Current, Earlier};
        match target {
            ReactionTargetV1::Thread { thread_id } => {
                self.thread(*thread_id).await?;
                Ok(Current)
            }
            ReactionTargetV1::HumanMessage {
                thread_id,
                message_id,
                revision,
            } => {
                self.thread(*thread_id).await?;
                let selected = self
                    .reader
                    .message_selection(*thread_id, *message_id)
                    .await?
                    .ok_or(Error::TargetUnavailable)?;
                let mut cursor = Some(selected);
                let mut seen = BTreeSet::new();
                let mut author = None;
                while let Some(reference) = cursor {
                    if !seen.insert(reference) {
                        return Err(Error::Invalid("message target history cycle"));
                    }
                    let CollabObjectV1::HumanMessageRevision(message) =
                        self.collab(reference).await?
                    else {
                        return Err(Error::Invalid("message target kind"));
                    };
                    if (message.thread_id, message.message_id) != (*thread_id, *message_id)
                        || author.is_some_and(|actor| actor != message.actor)
                    {
                        return Err(Error::Invalid("message target membership"));
                    }
                    author = Some(message.actor);
                    if reference == selected && message.body.is_none() {
                        return Err(Error::TargetUnavailable);
                    }
                    if reference == *revision {
                        if message.body.is_none() {
                            return Err(Error::TargetUnavailable);
                        }
                        return Ok(if reference == selected {
                            Current
                        } else {
                            Earlier
                        });
                    }
                    cursor = message.previous;
                }
                Err(Error::TargetUnavailable)
            }
            ReactionTargetV1::AgentMessage {
                session_id,
                message_id,
                event_id,
                batch,
                content,
            } => {
                let session = self.session(*session_id).await?;
                if !session.event_batches.contains(batch) {
                    return Err(Error::TargetUnavailable);
                }
                let AgentObjectV1::EventBatch(events) = self.agent(*batch).await? else {
                    return Err(Error::Invalid("message target batch"));
                };
                if !events.events.iter().any(|event| matches!(event,
                    jj_agent::AgentEventV1::MessageCompleted { event_id: id, session_id: session, message_id: message, content: body, .. }
                    if (id, session, message, body) == (event_id, session_id, message_id, content))) {
                    return Err(Error::TargetUnavailable);
                }
                if !matches!(self.agent(*content).await?, AgentObjectV1::Content(_)) {
                    return Err(Error::Invalid("message target content"));
                }
                Ok(Current)
            }
            ReactionTargetV1::ToolInvocation {
                session_id,
                tool_id,
                snapshot,
            }
            | ReactionTargetV1::ToolOutput {
                session_id,
                tool_id,
                snapshot,
                ..
            } => {
                let session = self.session(*session_id).await?;
                let selected = *session.tools.get(tool_id).ok_or(Error::TargetUnavailable)?;
                for reference in BTreeSet::from([selected, *snapshot]) {
                    let AgentObjectV1::ToolSnapshot(tool) = self.agent(reference).await? else {
                        return Err(Error::Invalid("tool target kind"));
                    };
                    if (tool.session_id, tool.tool_call_id) != (*session_id, *tool_id) {
                        return Err(Error::Invalid("tool target membership"));
                    }
                    if reference == *snapshot
                        && let ReactionTargetV1::ToolOutput { content, .. } = target
                    {
                        if tool.output != Some(*content) {
                            return Err(Error::TargetUnavailable);
                        }
                        if !matches!(self.agent(*content).await?, AgentObjectV1::Content(_)) {
                            return Err(Error::Invalid("tool output content"));
                        }
                    }
                }
                Ok(if selected == *snapshot {
                    Current
                } else {
                    Earlier
                })
            }
            ReactionTargetV1::ReviewThread { thread_id } => {
                self.review_comments(*thread_id).await?;
                Ok(Current)
            }
            ReactionTargetV1::ReviewComment {
                thread_id,
                comment_id,
                revision,
            } => {
                let comments = self.review_comments(*thread_id).await?;
                let selected = comments
                    .iter()
                    .find(|c| c.comment_id == *comment_id)
                    .ok_or(Error::TargetUnavailable)?
                    .revision;
                let mut pending = vec![selected];
                let mut seen = BTreeSet::new();
                let mut author = None;
                while let Some(reference) = pending.pop() {
                    if !seen.insert(reference) {
                        continue;
                    }
                    let comment = match self.review(reference).await? {
                        ReviewObjectV1::CommentRevision(comment) => comment,
                        ReviewObjectV1::CommentConflict(_) if reference == selected => {
                            return Err(Error::TargetUnavailable);
                        }
                        ReviewObjectV1::CommentConflict(conflict) => {
                            if (conflict.thread_id, conflict.comment_id)
                                != (*thread_id, *comment_id)
                            {
                                return Err(Error::Invalid("review conflict membership"));
                            }
                            pending.extend(conflict.alternatives);
                            continue;
                        }
                        _ => return Err(Error::Invalid("review target revision kind")),
                    };
                    if (comment.thread_id, comment.comment_id) != (*thread_id, *comment_id)
                        || author.is_some_and(|actor| actor != comment.author)
                    {
                        return Err(Error::Invalid("review target membership"));
                    }
                    author = Some(comment.author);
                    if reference == selected
                        && !matches!(comment.content, CommentRevisionContentV1::Visible { .. })
                    {
                        return Err(Error::TargetUnavailable);
                    }
                    if reference == *revision {
                        let CommentRevisionContentV1::Visible { body } = comment.content else {
                            return Err(Error::TargetUnavailable);
                        };
                        if !matches!(self.review(body).await?, ReviewObjectV1::CommentBody(_)) {
                            return Err(Error::Invalid("review target body kind"));
                        }
                        return Ok(if reference == selected {
                            Current
                        } else {
                            Earlier
                        });
                    }
                    pending.extend(comment.previous_revision);
                    pending.extend(comment.superseded_revisions);
                }
                Err(Error::TargetUnavailable)
            }
            ReactionTargetV1::AgentCheckpoint {
                session_id,
                checkpoint_id,
                version,
            } => {
                self.session(*session_id).await?;
                let selected = self
                    .reader
                    .checkpoint_selection(*checkpoint_id)
                    .await?
                    .ok_or(Error::TargetUnavailable)?;
                let mut cursor = Some(selected);
                let mut seen = BTreeSet::new();
                while let Some(reference) = cursor {
                    if !seen.insert(reference) {
                        return Err(Error::Invalid("checkpoint target cycle"));
                    }
                    let facts = match self.agent(reference).await? {
                        AgentObjectV1::CheckpointVersion(checkpoint) => checkpoint.facts(),
                        AgentObjectV1::PortableCheckpointVersion(checkpoint) => checkpoint.facts,
                        AgentObjectV1::CatalogConflict(_) if reference == selected => {
                            return Err(Error::TargetUnavailable);
                        }
                        _ => return Err(Error::Invalid("checkpoint target kind")),
                    };
                    if (facts.session_id, facts.checkpoint_id) != (*session_id, *checkpoint_id) {
                        return Err(Error::Invalid("checkpoint target membership"));
                    }
                    if reference == *version {
                        return Ok(if reference == selected {
                            Current
                        } else {
                            Earlier
                        });
                    }
                    cursor = facts.supersedes;
                }
                Err(Error::TargetUnavailable)
            }
        }
    }
    async fn review_comments(
        &mut self,
        id: ReviewThreadId,
    ) -> Result<Vec<jj_review::CommentSelectionV1>> {
        let reference = self
            .reader
            .review_selection(id)
            .await?
            .ok_or(Error::TargetUnavailable)?;
        let (thread_id, comments) = match self.review(reference).await? {
            ReviewObjectV1::ThreadSnapshot(thread) => (thread.thread_id, thread.comments),
            ReviewObjectV1::ThreadMerge(thread) => (thread.thread_id, thread.comments),
            _ => return Err(Error::Invalid("selected review kind")),
        };
        if thread_id != id {
            return Err(Error::Invalid("selected review identity"));
        }
        Ok(comments)
    }
}
