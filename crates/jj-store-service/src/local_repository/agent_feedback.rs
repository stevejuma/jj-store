use super::*;
impl LocalRepository {
    pub async fn agent_feedback(
        &self,
        rows: &[(
            String,
            jj_store_wal::publication::AgentSessionId,
            jj_store_wal::publication::MessageId,
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
            crate::agent_feedback::project(&agents, &threads.state, &threads.graph, rows).await?;
        if self.ensure_actor().map_err(|_| Error::Denied)? != actor {
            return Err(Error::Denied);
        }
        Ok(result)
    }
}
