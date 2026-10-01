//! Shared immutable domain projection; adapters retain publication authority.
use super::*;
use jj_agent::{
    AgentCatalogKey, AgentCatalogKind, NormalizerProfileV1, SanitizedObservationBatchV1,
};

/// Implementations bind one repository and validate canonical identities,
/// semantic kinds and complete dependencies before returning a record.
#[async_trait::async_trait]
pub trait AgentObjectIo: Send + Sync {
    fn repository(&self) -> RepositoryId;
    async fn read_agent_object(&self, reference: AgentObjectRef) -> Result<AgentObjectV1>;
    async fn stage_agent_object(&self, object: &AgentObjectV1) -> Result<AgentObjectRef>;
}
/// Reads expose only the audited closure of one pinned root.
#[async_trait::async_trait]
pub trait AgentSnapshotRead: Send + Sync {
    fn repository(&self) -> RepositoryId;
    fn root(&self) -> &AgentRootV1;
    async fn lookup(
        &self,
        catalog: AgentCatalogKind,
        key: AgentCatalogKey,
    ) -> Result<Option<AgentObjectRef>>;
    async fn read_object(&self, reference: AgentObjectRef) -> Result<AgentObjectV1>;
}

pub struct AgentProjector<'a> {
    pub(super) objects: &'a dyn AgentObjectIo,
}
impl<'a> AgentProjector<'a> {
    pub fn new(objects: &'a dyn AgentObjectIo) -> Self {
        Self { objects }
    }
    pub(super) async fn stage_agent_object(
        &self,
        object: &AgentObjectV1,
    ) -> Result<AgentObjectRef> {
        self.objects.stage_agent_object(object).await
    }
}

#[async_trait::async_trait]
impl AgentObjectIo for Coordinator {
    fn repository(&self) -> RepositoryId {
        self.records.repository
    }
    async fn read_agent_object(&self, reference: AgentObjectRef) -> Result<AgentObjectV1> {
        Coordinator::read_agent_object(self, reference).await
    }
    async fn stage_agent_object(&self, object: &AgentObjectV1) -> Result<AgentObjectRef> {
        Coordinator::stage_agent_object(self, object).await
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
    async fn lookup(
        &self,
        catalog: AgentCatalogKind,
        key: AgentCatalogKey,
    ) -> Result<Option<AgentObjectRef>> {
        AgentStoreSnapshot::lookup(self, catalog, key).await
    }
    async fn read_object(&self, reference: AgentObjectRef) -> Result<AgentObjectV1> {
        AgentStoreSnapshot::read_object(self, reference).await
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
    }
}
