//! Authorized root selections compose the same resolver for native and Git carriers.
use super::{AgentSnapshotRead, ReviewSnapshotRead};
use crate::{Error, Result};
use jj_agent::{
    AgentCatalogKey, AgentCatalogKind, AgentCheckpointId, AgentObjectRef, AgentObjectV1,
    AgentSessionId,
};
use jj_collab::{
    CollabGraphV1, CollabObjectRef, CollabObjectV1, HumanMessageId, ReactionTargetRead, ThreadId,
    ThreadRootV1, ThreadStateV1,
};
use jj_review::{
    ReviewCatalogKey, ReviewCatalogKind, ReviewObjectRef, ReviewObjectV1, ReviewThreadId,
};
use jj_store_model::RepositoryId;
use std::collections::BTreeSet;

/// Callers authorize the repository and capture every input root from one read
/// snapshot before constructing this facade. No object staging is exposed.
pub struct CollabTargetReaderV1<'a> {
    repository: RepositoryId,
    qualified: Option<&'a dyn jj_collab::QualifiedReviewRead>,
    work: Option<&'a jj_work::WorkStateV1>,
    thread: Option<(ThreadStateV1, &'a CollabGraphV1, BTreeSet<CollabObjectRef>)>,
    agent: Option<&'a dyn AgentSnapshotRead>,
    review: Option<&'a dyn ReviewSnapshotRead>,
}
impl<'a> CollabTargetReaderV1<'a> {
    pub fn new(
        repository: RepositoryId,
        thread: Option<(ThreadRootV1, &'a CollabGraphV1)>,
        agent: Option<&'a dyn AgentSnapshotRead>,
        review: Option<&'a dyn ReviewSnapshotRead>,
    ) -> Result<Self> {
        if agent.is_some_and(|s| s.repository() != repository)
            || review.is_some_and(|s| s.repository() != repository)
        {
            return Err(Error::Corrupt("target reader repository"));
        }
        let thread = thread
            .map(|(root, graph)| {
                let state = ThreadStateV1::read(root.clone(), graph)
                    .map_err(super::collab_store::collab_error)?;
                let closure = graph
                    .closure(root.dependencies())
                    .map_err(super::collab_store::collab_error)?;
                Ok::<_, Error>((state, graph, closure))
            })
            .transpose()?;
        Ok(Self {
            repository,
            work: None,
            qualified: None,
            thread,
            agent,
            review,
        })
    }
    pub fn with_qualified_reviews(
        mut self,
        reader: Option<&'a dyn jj_collab::QualifiedReviewRead>,
    ) -> Self {
        self.qualified = reader;
        self
    }
    pub fn with_work(
        mut self,
        repository: RepositoryId,
        work: &'a jj_work::WorkStateV1,
    ) -> Result<Self> {
        if repository != self.repository {
            return Err(Error::Corrupt("work target reader repository"));
        }
        work.validate()
            .map_err(|_| Error::Corrupt("work target reader state"))?;
        self.work = Some(work);
        Ok(self)
    }
}
fn read_error(error: Error) -> jj_collab::Error {
    match error {
        Error::Limit(_) => jj_collab::Error::Limit,
        Error::Corrupt(_) | Error::Json(_) => {
            jj_collab::Error::Invalid("pinned target reader data")
        }
        _ => jj_collab::Error::ReadUnavailable,
    }
}
fn read_result<T, E: Into<Error>>(
    result: std::result::Result<T, E>,
) -> jj_collab::Result<Option<T>> {
    match result.map_err(Into::into) {
        Ok(value) => Ok(Some(value)),
        Err(Error::Stale | Error::Denied | Error::Storage(jj_store_object::Error::NotFound)) => {
            Ok(None)
        }
        Err(error) => Err(read_error(error)),
    }
}
#[async_trait::async_trait]
impl ReactionTargetRead for CollabTargetReaderV1<'_> {
    async fn qualified_review_comment(
        &self,
        target: &jj_collab::ReactionTargetV1,
    ) -> jj_collab::Result<jj_collab::QualifiedReviewComment> {
        self.qualified
            .ok_or(jj_collab::Error::TargetUnavailable)?
            .comment(target)
            .await
    }
    async fn work_comment(
        &self,
        task: jj_work::TaskId,
        comment: jj_work::WorkCommentId,
    ) -> jj_collab::Result<Option<jj_work::WorkCommentV1>> {
        Ok(self
            .work
            .and_then(|state| state.comments.get(&comment))
            .filter(|value| value.task == task)
            .cloned())
    }
    async fn thread_selection(&self, id: ThreadId) -> jj_collab::Result<Option<CollabObjectRef>> {
        Ok(self
            .thread
            .as_ref()
            .and_then(|(state, _, _)| state.threads.get(&id).copied()))
    }
    async fn message_selection(
        &self,
        thread: ThreadId,
        message: HumanMessageId,
    ) -> jj_collab::Result<Option<CollabObjectRef>> {
        let Some((state, graph, _)) = &self.thread else {
            return Ok(None);
        };
        let Some(reference) = state.messages.get(&message).copied() else {
            return Ok(None);
        };
        let CollabObjectV1::HumanMessageRevision(value) = graph.get(reference)? else {
            return Err(jj_collab::Error::Invalid("selected human message kind"));
        };
        Ok((value.thread_id == thread).then_some(reference))
    }
    async fn collab_object(
        &self,
        reference: CollabObjectRef,
    ) -> jj_collab::Result<Option<CollabObjectV1>> {
        let Some((_, graph, closure)) = &self.thread else {
            return Ok(None);
        };
        if !closure.contains(&reference) {
            return Ok(None);
        }
        graph.get(reference).cloned().map(Some)
    }
    async fn session_selection(
        &self,
        id: AgentSessionId,
    ) -> jj_collab::Result<Option<AgentObjectRef>> {
        let Some(agent) = self.agent else {
            return Ok(None);
        };
        let key = AgentCatalogKey::derive("session", id.as_bytes())
            .map_err(|_| jj_collab::Error::Invalid("session target key"))?;
        read_result(agent.lookup(AgentCatalogKind::Sessions, key).await).map(Option::flatten)
    }
    async fn checkpoint_selection(
        &self,
        id: AgentCheckpointId,
    ) -> jj_collab::Result<Option<AgentObjectRef>> {
        let Some(agent) = self.agent else {
            return Ok(None);
        };
        let key = AgentCatalogKey::derive("checkpoint", id.as_bytes())
            .map_err(|_| jj_collab::Error::Invalid("checkpoint target key"))?;
        read_result(agent.lookup(AgentCatalogKind::Checkpoints, key).await).map(Option::flatten)
    }
    async fn agent_object(
        &self,
        reference: AgentObjectRef,
    ) -> jj_collab::Result<Option<AgentObjectV1>> {
        let Some(agent) = self.agent else {
            return Ok(None);
        };
        read_result(agent.read_object(reference).await)
    }
    async fn review_selection(
        &self,
        id: ReviewThreadId,
    ) -> jj_collab::Result<Option<ReviewObjectRef>> {
        let Some(review) = self.review else {
            return Ok(None);
        };
        let key = ReviewCatalogKey::derive("thread", id.as_bytes())
            .map_err(|_| jj_collab::Error::Invalid("review target key"))?;
        read_result(review.lookup(ReviewCatalogKind::Threads, key).await).map(Option::flatten)
    }
    async fn review_object(
        &self,
        reference: ReviewObjectRef,
    ) -> jj_collab::Result<Option<ReviewObjectV1>> {
        let Some(review) = self.review else {
            return Ok(None);
        };
        read_result(review.read_object(reference).await)
    }
}
