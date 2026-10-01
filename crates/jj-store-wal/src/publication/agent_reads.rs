//! Bounded root-pinned AgentStore catalog, relation and transcript reads.
use super::*;
use jj_agent::{
    AgentCatalogKey, AgentCatalogKind, AgentRelationSubjectV1, SourceInstanceId, TranscriptChunkV1,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentCatalogPage {
    pub selections: Vec<(AgentCatalogKey, AgentObjectRef)>,
    pub next_after: Option<AgentCatalogKey>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AgentRelationCursor {
    pub page: AgentObjectRef,
    pub offset: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentRelationRange {
    pub selections: Vec<AgentObjectRef>,
    pub next: Option<AgentRelationCursor>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TranscriptRange {
    pub chunks: Vec<TranscriptChunkV1>,
    pub next_after_sequence: Option<u64>,
}

impl AgentStoreSnapshot {
    pub async fn list_sessions(
        &self,
        after: Option<AgentCatalogKey>,
        limit: usize,
    ) -> Result<AgentCatalogPage> {
        AgentReader::new(self).list_sessions(after, limit).await
    }
    pub async fn list_checkpoints(
        &self,
        after: Option<AgentCatalogKey>,
        limit: usize,
    ) -> Result<AgentCatalogPage> {
        AgentReader::new(self).list_checkpoints(after, limit).await
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
    }
}
