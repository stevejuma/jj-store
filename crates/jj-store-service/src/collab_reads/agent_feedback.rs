use super::*;
impl PinnedCollabRead {
    pub(crate) async fn recorded_feedback(
        &self,
        session: Arc<NativeSession>,
        activity: &[crate::collaboration::AgentActivity],
    ) -> crate::agent_execution::Result<crate::agent_execution::Feedback> {
        use crate::agent_execution::Error;
        self.authorize(&session).await.map_err(|_| Error::Denied)?;
        let agents = self.agents.as_ref().ok_or(Error::Unavailable)?;
        let reader = session
            .coordinator
            .selected_agent_reader(agents)
            .map_err(|_| Error::Unavailable)?;
        let mut result = crate::agent_feedback::recorded(
            &reader,
            &self.thread.state,
            &self.thread.graph,
            activity,
        )
        .await?;
        if !self.feedback_writable {
            result.write_unavailable=Some("Conversation comments are unavailable until collaboration replication is qualified.".into());
        }
        self.authorize(&session).await.map_err(|_| Error::Denied)?;
        Ok(result)
    }
    pub async fn agent_feedback(
        &self,
        session: Arc<NativeSession>,
        rows: &[(
            String,
            jj_store_wal::publication::AgentSessionId,
            crate::agent_execution::FeedbackSubject,
        )],
    ) -> crate::agent_execution::Result<crate::agent_execution::Feedback> {
        use crate::agent_execution::Error;
        self.authorize(&session).await.map_err(|_| Error::Denied)?;
        let mut result = if let Some(agents) = &self.agents {
            let reader = session
                .coordinator
                .selected_agent_reader(agents)
                .map_err(|_| Error::Unavailable)?;
            crate::agent_feedback::project(&reader, &self.thread.state, &self.thread.graph, rows)
                .await?
        } else {
            crate::agent_execution::Feedback {
                messages: vec![],
                profile: None,
                write_unavailable: None,
            }
        };
        if !self.feedback_writable {
            result.write_unavailable = Some(
                "Replies and reactions are unavailable for this code repository until collaboration replication is qualified.".into(),
            );
        }
        self.authorize(&session).await.map_err(|_| Error::Denied)?;
        Ok(result)
    }
    pub async fn agent_prompt_context(
        &self,
        session: Arc<NativeSession>,
        rows: &[(
            String,
            jj_store_wal::publication::AgentSessionId,
            crate::agent_execution::FeedbackSubject,
        )],
        sessions: &std::collections::BTreeSet<jj_store_wal::publication::AgentSessionId>,
        excluded: &std::collections::BTreeSet<crate::agent_execution::context::ReplyRevision>,
    ) -> crate::agent_execution::Result<crate::agent_execution::context::ContextBatch> {
        use crate::agent_execution::{Error, context::ContextBatch};
        self.authorize(&session).await.map_err(|_| Error::Denied)?;
        let result = if let Some(agents) = &self.agents {
            let reader = session
                .coordinator
                .selected_agent_reader(agents)
                .map_err(|_| Error::Unavailable)?;
            crate::agent_execution::context::collect(
                &reader,
                &self.thread.state,
                &self.thread.graph,
                rows,
                sessions,
                excluded,
            )
            .await?
        } else {
            ContextBatch::default()
        };
        self.authorize(&session).await.map_err(|_| Error::Denied)?;
        Ok(result)
    }
}

impl PinnedCollabRead {
    pub async fn conversation_submission_context(
        &self,
        session: Arc<NativeSession>,
        thread: ThreadId,
        batch: CollabObjectRef,
    ) -> crate::agent_execution::Result<crate::agent_execution::context::ContextBatch> {
        use crate::agent_execution::Error;
        self.authorize(&session).await.map_err(|_| Error::Denied)?;
        let agents = self
            .agents
            .as_ref()
            .map(|s| session.coordinator.selected_agent_reader(s))
            .transpose()
            .map_err(|_| Error::Unavailable)?;
        let reviews = self
            .reviews
            .as_ref()
            .map(|s| session.coordinator.selected_review_reader(s))
            .transpose()
            .map_err(|_| Error::Unavailable)?;
        let reader = CollabTargetReaderV1::new(
            self.snapshot.repo_id,
            Some((self.thread.state.root.clone(), &self.thread.graph)),
            agents
                .as_ref()
                .map(|s| s as &dyn wal::publication::AgentSnapshotRead),
            reviews
                .as_ref()
                .map(|s| s as &dyn wal::publication::ReviewSnapshotRead),
        )
        .map_err(|_| Error::Unavailable)?;
        let result = crate::agent_execution::context_submission::collect_submission(
            &reader,
            self.snapshot.repo_id,
            thread,
            batch,
        )
        .await?;
        self.authorize(&session).await.map_err(|_| Error::Denied)?;
        Ok(result)
    }
}
