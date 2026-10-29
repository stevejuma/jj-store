//! Shared immutable domain projection; adapters retain publication authority.
use super::*;
use jj_agent::{AgentCatalogKey, AgentCatalogKind};

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
    fn transcript_boundary(&self, _root: AgentObjectRef) -> Option<TranscriptAppendBoundary> {
        None
    }
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
