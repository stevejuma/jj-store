use super::*;
use jj_agent::{AgentCatalogKey, AgentCatalogKind, AgentRelationSubjectV1, SourceInstanceId};
impl AgentStoreSnapshot {
    pub async fn list_sessions(
        &self,
        after: Option<AgentCatalogKey>,
        limit: usize,
    ) -> Result<AgentCatalogPage> {
        AgentReader::new(self)
            .list_sessions(after, limit)
            .await
            .map_err(Into::into)
    }
    pub async fn list_checkpoints(
        &self,
        after: Option<AgentCatalogKey>,
        limit: usize,
    ) -> Result<AgentCatalogPage> {
        AgentReader::new(self)
            .list_checkpoints(after, limit)
            .await
            .map_err(Into::into)
    }
    pub async fn list_catalog(
        &self,
        catalog: AgentCatalogKind,
        after: Option<AgentCatalogKey>,
        limit: usize,
    ) -> Result<AgentCatalogPage> {
        AgentReader::new(self)
            .list_catalog(catalog, after, limit)
            .await
            .map_err(Into::into)
    }
    pub async fn relations(
        &self,
        subject: &AgentRelationSubjectV1,
        cursor: Option<AgentRelationCursor>,
        limit: usize,
    ) -> Result<AgentRelationRange> {
        AgentReader::new(self)
            .relations(subject, cursor, limit)
            .await
            .map_err(Into::into)
    }
    pub async fn transcript_range(
        &self,
        manifest: AgentObjectRef,
        source: SourceInstanceId,
        generation: u64,
        after_sequence: Option<u64>,
        limit: usize,
        max_bytes: usize,
    ) -> Result<TranscriptRange> {
        AgentReader::new(self)
            .transcript_range(
                manifest,
                source,
                generation,
                after_sequence,
                limit,
                max_bytes,
            )
            .await
            .map_err(Into::into)
    }
}
