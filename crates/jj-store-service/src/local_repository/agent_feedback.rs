use super::*;
impl LocalRepository {
    pub async fn agent_feedback(
        &self,
        rows: &[(
            String,
            jj_store_wal::publication::AgentSessionId,
            crate::agent_execution::FeedbackSubject,
        )],
    ) -> crate::agent_execution::Result<crate::agent_execution::Feedback> {
        use crate::agent_execution::Error;
        let actor = self.ensure_actor().map_err(|_| Error::Denied)?;
        let pin = self.metadata.read().map_err(|_| Error::Unavailable)?;
        let repository = pin
            .as_ref()
            .map_or(jj_store_model::RepositoryId::new([0; 16]), |s| {
                s.repository()
            });
        let threads = self
            .thread_store_at(pin.as_ref())
            .await
            .map_err(|_| Error::Unavailable)?;
        let agents = self
            .agent_store_at(pin.as_ref(), repository)
            .await
            .map_err(|_| Error::Unavailable)?;
        let result =
            crate::agent_feedback::project(&agents, &threads.state, threads.graph(), rows).await?;
        if self.ensure_actor().map_err(|_| Error::Denied)? != actor {
            return Err(Error::Denied);
        }
        Ok(result)
    }
    pub async fn agent_prompt_context(
        &self,
        rows: &[(
            String,
            jj_store_wal::publication::AgentSessionId,
            crate::agent_execution::FeedbackSubject,
        )],
        sessions: &std::collections::BTreeSet<jj_store_wal::publication::AgentSessionId>,
        excluded: &std::collections::BTreeSet<crate::agent_execution::context::ReplyRevision>,
    ) -> crate::agent_execution::Result<crate::agent_execution::context::ContextBatch> {
        use crate::agent_execution::Error;
        let actor = self.ensure_actor().map_err(|_| Error::Denied)?;
        let pin = self.metadata.read().map_err(|_| Error::Unavailable)?;
        let repository = pin
            .as_ref()
            .map_or(jj_store_model::RepositoryId::new([0; 16]), |s| {
                s.repository()
            });
        let threads = self
            .thread_store_at(pin.as_ref())
            .await
            .map_err(|_| Error::Unavailable)?;
        let agents = self
            .agent_store_at(pin.as_ref(), repository)
            .await
            .map_err(|_| Error::Unavailable)?;
        let result = crate::agent_execution::context::collect(
            &agents,
            &threads.state,
            threads.graph(),
            rows,
            sessions,
            excluded,
        )
        .await?;
        if self.ensure_actor().map_err(|_| Error::Denied)? != actor {
            return Err(Error::Denied);
        }
        Ok(result)
    }
}

impl LocalRepository {
    pub async fn conversation_submission_context(
        &self,
        thread: jj_collab::ThreadId,
        batch: jj_collab::CollabObjectRef,
    ) -> crate::agent_execution::Result<crate::agent_execution::context::ContextBatch> {
        self.conversation_submission_context_with_reviews(thread, batch, None)
            .await
    }
    pub async fn conversation_submission_context_with_reviews(
        &self,
        thread: jj_collab::ThreadId,
        batch: jj_collab::CollabObjectRef,
        qualified: Option<&dyn jj_collab::QualifiedReviewRead>,
    ) -> crate::agent_execution::Result<crate::agent_execution::context::ContextBatch> {
        use crate::agent_execution::Error;
        use jj_store_wal::publication::CollabTargetReaderV1;
        let actor = self.ensure_actor().map_err(|_| Error::Denied)?;
        let pin = self.metadata.read().map_err(|_| Error::Unavailable)?;
        let repository = pin.as_ref().ok_or(Error::Unavailable)?.repository();
        let threads = self
            .thread_store_at(pin.as_ref())
            .await
            .map_err(|_| Error::Unavailable)?;
        let agents = self
            .agent_store_at(pin.as_ref(), repository)
            .await
            .map_err(|_| Error::Unavailable)?;
        let reviews = self
            .review_store_at(pin.as_ref(), repository)
            .await
            .map_err(|_| Error::Unavailable)?;
        let reader = CollabTargetReaderV1::new(
            repository,
            Some((threads.state.root.clone(), threads.graph())),
            Some(&agents),
            Some(&reviews),
        )
        .map_err(|_| Error::Unavailable)?
        .with_qualified_reviews(qualified);
        let result = crate::agent_execution::context_submission::collect_submission(
            &reader, repository, thread, batch,
        )
        .await?;
        if self.ensure_actor().map_err(|_| Error::Denied)? != actor {
            return Err(Error::Denied);
        }
        Ok(result)
    }
}
