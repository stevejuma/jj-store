//! Explicit capture links and complete staged session retention before ref CAS.
use super::{LocalAgentStore, LocalError, LocalRepository, LocalResult, collab_graph};
use jj_collab::ThreadBindingV1;
use jj_store_jj::local_metadata::{GitOid, RetainedKind, Snapshot};
use jj_store_model::{PrincipalId, RequestId};
use jj_store_wal::publication::{
    AgentCatalogKey, AgentCatalogKind, AgentDeltaV1, AgentObjectIo, AgentObjectKind, AgentObjectV1,
};
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
        let mut sessions = self
            .delta
            .edits
            .iter()
            .filter(|e| e.catalog == AgentCatalogKind::Sessions);
        let edit = sessions.next().ok_or(LocalError::Invalid)?;
        if sessions.next().is_some() {
            return Err(LocalError::Invalid);
        }
        let snapshot = edit
            .replacement
            .filter(|r| r.kind == AgentObjectKind::SessionSnapshot)
            .ok_or(LocalError::Invalid)?;
        let AgentObjectV1::SessionSnapshot(session) = agent.io.read_agent_object(snapshot).await?
        else {
            return Err(LocalError::Corrupt);
        };
        if edit.key
            != AgentCatalogKey::derive("session", session.session_id.as_bytes())
                .map_err(|_| LocalError::Invalid)?
        {
            return Err(LocalError::Corrupt);
        }
        let command =
            self.binding
                .capture_link(session.session_id, snapshot, self.actor, self.request)?;
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
