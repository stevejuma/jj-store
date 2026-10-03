use crate::{
    AgentObjectKind, AgentObjectRef, AgentSessionId, CoverageV1, Error, Result, SanitizedPayloadV1,
    SanitizedRepresentation, SourceInstanceId, activity::valid_label,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TranscriptChunkV1 {
    pub schema_version: u32,
    pub source_instance_id: SourceInstanceId,
    pub source_generation: u64,
    pub sequence: u64,
    pub payload: SanitizedPayloadV1,
}
impl TranscriptChunkV1 {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 1 {
            return Err(Error::Invalid("transcript chunk schema"));
        }
        self.payload.validate()
    }
    pub fn dependencies(&self) -> Result<BTreeSet<AgentObjectRef>> {
        self.validate()?;
        Ok(BTreeSet::new())
    }
}

/// Immutable append tail. Prefixes retain earlier current tails.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TranscriptTailPageV2 {
    pub schema_version: u32,
    pub source_instance_id: SourceInstanceId,
    pub source_generation: u64,
    pub chunks: Vec<AgentObjectRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<AgentObjectRef>,
}
impl TranscriptTailPageV2 {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 2
            || self.source_generation == 0
            || self.chunks.is_empty()
            || self.chunks.len() > 256
            || self
                .chunks
                .iter()
                .any(|r| r.kind != AgentObjectKind::TranscriptChunk)
            || self.chunks.iter().collect::<BTreeSet<_>>().len() != self.chunks.len()
            || self
                .prefix
                .is_some_and(|r| r.kind != AgentObjectKind::TranscriptPage)
        {
            return Err(Error::Invalid("transcript tail page"));
        }
        Ok(())
    }
    pub fn dependencies(&self) -> Result<BTreeSet<AgentObjectRef>> {
        self.validate()?;
        Ok(self.chunks.iter().copied().chain(self.prefix).collect())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TranscriptStreamV1 {
    pub source_instance_id: SourceInstanceId,
    pub source_generation: u64,
    pub source_format: String,
    pub decoder_version: String,
    pub sanitizer_version: String,
    pub representation: SanitizedRepresentation,
    pub ordered_chunk_root: AgentObjectRef,
    pub logical_byte_length: u64,
    pub record_count: u64,
    pub gaps: u32,
}
impl TranscriptStreamV1 {
    fn validate(&self) -> Result<()> {
        if !valid_label(&self.source_format, 128)
            || !valid_label(&self.decoder_version, 128)
            || !valid_label(&self.sanitizer_version, 128)
            || self.ordered_chunk_root.kind != AgentObjectKind::TranscriptPage
        {
            return Err(Error::Invalid("transcript stream"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TranscriptManifestV1 {
    pub schema_version: u32,
    pub session_id: AgentSessionId,
    pub streams: Vec<TranscriptStreamV1>,
    pub coverage: CoverageV1,
}
impl TranscriptManifestV1 {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 1 || self.streams.len() > 16 {
            return Err(Error::Invalid("transcript manifest"));
        }
        let mut streams = BTreeSet::new();
        for stream in &self.streams {
            stream.validate()?;
            if !streams.insert((stream.source_instance_id, stream.source_generation)) {
                return Err(Error::Invalid("duplicate transcript stream"));
            }
        }
        self.coverage.validate()
    }
    pub fn dependencies(&self) -> Result<BTreeSet<AgentObjectRef>> {
        self.validate()?;
        Ok(self
            .streams
            .iter()
            .map(|stream| stream.ordered_chunk_root)
            .collect())
    }
}
