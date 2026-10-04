//! Explicit capture links and complete staged session retention before ref CAS.
use super::{LocalAgentStore, LocalError, LocalRepository, LocalResult, collab_graph};
use jj_collab::ThreadBindingV1;
use jj_store_jj::local_metadata::{GitOid, RetainedKind, Snapshot};
use jj_store_model::{PrincipalId, RequestId};
use jj_store_wal::publication::{AgentDeltaV1, AgentProjector};
use std::collections::BTreeMap;

pub(super) struct CaptureBinding<'a> {
    pub binding: &'a ThreadBindingV1,
    pub delta: &'a AgentDeltaV1,
    pub actor: PrincipalId,
    pub request: RequestId,
    pub at: i64,
}
impl CaptureBinding<'_> {
    pub async fn stage(
        &self,
        repo: &LocalRepository,
        pin: Option<&Snapshot>,
        agent: &LocalAgentStore,
    ) -> LocalResult<(Vec<u8>, BTreeMap<GitOid, RetainedKind>)> {
        AgentProjector::validate_capture_binding_delta(self.delta)
            .map_err(|_| LocalError::Invalid)?;
        let command = AgentProjector::new(agent.io.as_ref())
            .capture_binding_command(self.delta, self.binding, self.actor, self.request)
            .await?;
        let mut threads = repo.thread_store_at(pin).await?;
        let (root, _) = threads.state.apply(
            &command,
            self.actor,
            self.request,
            self.at,
            &mut threads.graph,
        )?;
        threads.external.retain_staged_agent(agent)?;
        collab_graph::stage(
            repo,
            pin,
            agent.io.store.repository,
            root,
            &threads.graph,
            &threads.external,
        )
        .await
    }
}
