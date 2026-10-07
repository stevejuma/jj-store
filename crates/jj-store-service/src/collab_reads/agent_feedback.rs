use super::*;
impl PinnedCollabRead {
    pub async fn agent_feedback(
        &self,
        session: Arc<NativeSession>,
        rows: &[(
            String,
            jj_store_wal::publication::AgentSessionId,
            jj_store_wal::publication::MessageId,
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
            jj_store_wal::publication::MessageId,
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
