//! Bounded append over immutable tails; provider closure audits remain authoritative.
use super::agent_capture_records::TranscriptChunkSelection;
use super::*;
use jj_agent::{SanitizedObservationBatchV1, TranscriptStreamV1, TranscriptTailPageV2};

impl AgentProjector<'_> {
    pub(super) async fn append_transcript_tail(
        &self,
        base: &dyn AgentSnapshotRead,
        batch: &SanitizedObservationBatchV1,
        previous: Option<&TranscriptStreamV1>,
        additions: &[TranscriptChunkSelection],
    ) -> Result<AgentObjectRef> {
        let Some(first) = additions.first() else {
            return Err(Error::Corrupt("empty transcript append"));
        };
        if additions.windows(2).any(|w| w[0].sequence >= w[1].sequence) {
            return Err(Error::Corrupt("transcript append order"));
        }
        let mut chunks = Vec::new();
        let mut prefix = None;
        if let Some(stream) = previous {
            let reference = stream.ordered_chunk_root;
            let boundary = base
                .transcript_boundary(reference)
                .ok_or(Error::Corrupt("transcript append ordering proof"))?;
            check_stream(batch, boundary.source, boundary.generation)?;
            if boundary.count != stream.record_count {
                return Err(Error::Corrupt("transcript append record count"));
            }
            if boundary.last >= first.sequence {
                return Err(Error::Stale);
            }
            match base.read_object(reference).await? {
                AgentObjectV1::TranscriptTailPage(tail) => {
                    check_stream(batch, tail.source_instance_id, tail.source_generation)?;
                    let last = *tail
                        .chunks
                        .last()
                        .ok_or(Error::Corrupt("empty transcript tail"))?;
                    let AgentObjectV1::TranscriptChunk(chunk) = base.read_object(last).await?
                    else {
                        return Err(Error::Corrupt("transcript tail chunk"));
                    };
                    check_stream(batch, chunk.source_instance_id, chunk.source_generation)?;
                    if chunk.sequence >= first.sequence {
                        return Err(Error::Stale);
                    }
                    if tail.chunks.len() == 256 {
                        prefix = Some(reference);
                    } else {
                        chunks = tail.chunks;
                        prefix = tail.prefix;
                    }
                }
                _ => return Err(Error::Corrupt("transcript append page")),
            }
        }
        let mut offset = 0;
        let mut result = None;
        while offset < additions.len() {
            let end = (offset + 256 - chunks.len()).min(additions.len());
            chunks.extend(additions[offset..end].iter().map(|c| c.reference));
            let reference = self
                .stage_agent_object(&AgentObjectV1::TranscriptTailPage(TranscriptTailPageV2 {
                    schema_version: 2,
                    source_instance_id: batch.source_instance_id,
                    source_generation: 1,
                    chunks: std::mem::take(&mut chunks),
                    prefix,
                }))
                .await?;
            result = Some(reference);
            prefix = Some(reference);
            offset = end;
        }
        result.ok_or(Error::Corrupt("empty transcript append"))
    }
}
fn check_stream(
    batch: &SanitizedObservationBatchV1,
    source: jj_agent::SourceInstanceId,
    generation: u64,
) -> Result<()> {
    if source != batch.source_instance_id || generation != 1 {
        return Err(Error::Corrupt("transcript append stream"));
    }
    Ok(())
}
