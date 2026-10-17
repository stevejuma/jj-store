use super::*;
use jj_agent::{AgentCatalogKey, AgentCatalogKind};
#[async_trait::async_trait]
impl AgentObjectIo for Coordinator {
    fn repository(&self) -> RepositoryId {
        self.records.repository
    }
    async fn read_agent_object(
        &self,
        reference: AgentObjectRef,
    ) -> jj_store_core::Result<AgentObjectV1> {
        Coordinator::read_agent_object(self, reference)
            .await
            .map_err(Into::into)
    }
    async fn stage_agent_object(
        &self,
        object: &AgentObjectV1,
    ) -> jj_store_core::Result<AgentObjectRef> {
        Coordinator::stage_agent_object(self, object)
            .await
            .map_err(Into::into)
    }
}
#[async_trait::async_trait]
impl AgentSnapshotRead for AgentStoreSnapshot {
    fn repository(&self) -> RepositoryId {
        self.repository_id()
    }
    fn root(&self) -> &AgentRootV1 {
        AgentStoreSnapshot::root(self)
    }
    fn transcript_boundary(&self, root: AgentObjectRef) -> Option<TranscriptAppendBoundary> {
        self.transcript_proofs().boundary(root)
    }
    async fn lookup(
        &self,
        catalog: AgentCatalogKind,
        key: AgentCatalogKey,
    ) -> jj_store_core::Result<Option<AgentObjectRef>> {
        AgentStoreSnapshot::lookup(self, catalog, key)
            .await
            .map_err(Into::into)
    }
    async fn read_object(&self, reference: AgentObjectRef) -> jj_store_core::Result<AgentObjectV1> {
        AgentStoreSnapshot::read_object(self, reference)
            .await
            .map_err(Into::into)
    }
}

impl Coordinator {
    pub async fn stage_agent_capture(
        &self,
        base: &AgentStoreSnapshot,
        profile: &NormalizerProfileV1,
        batch: &SanitizedObservationBatchV1,
        activated_at_millis: i64,
    ) -> Result<AgentDeltaV1> {
        AgentProjector::new(self)
            .stage_agent_capture(base, profile, batch, activated_at_millis)
            .await
            .map_err(Into::into)
    }
}
