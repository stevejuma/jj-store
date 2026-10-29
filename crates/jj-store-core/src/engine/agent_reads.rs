//! Bounded root-pinned AgentStore catalog, relation and transcript reads.
use super::*;
use jj_agent::{AgentCatalogKey, TranscriptChunkV1};

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
