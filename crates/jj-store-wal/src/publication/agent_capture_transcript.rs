use super::agent_capture_records::TranscriptChunkSelection;
use super::*;
use jj_agent::{
    AgentObjectV1, AgentSessionSnapshotV1, CompletenessV1, CoverageV1, IntegrationId,
    NormalizationBatchV1, SanitizedObservationBatchV1, TranscriptManifestV1, TranscriptPageV1,
    TranscriptStreamV1,
};

const TRANSCRIPT_PAGE_ENTRIES: usize = 256;
const SOURCE_GENERATION: u64 = 1;

impl AgentProjector<'_> {
    pub(super) async fn stage_capture_transcript(
        &self,
        base: &dyn AgentSnapshotRead,
        batch: &SanitizedObservationBatchV1,
        normalized: &NormalizationBatchV1,
        session: &mut AgentSessionSnapshotV1,
        additions: &[TranscriptChunkSelection],
    ) -> Result<()> {
        let mut chunks = Vec::new();
        let mut logical_byte_length = 0;
        let mut record_count = 0;
        let mut gaps = 0;
        let mut replaced = None;

        for (index, reference) in session.transcripts.iter().copied().enumerate() {
            let AgentObjectV1::TranscriptManifest(manifest) = base.read_object(reference).await?
            else {
                return Err(Error::Corrupt("agent transcript selection"));
            };
            if manifest.session_id != session.session_id {
                return Err(Error::Corrupt("agent transcript session"));
            }
            let Some(stream) = manifest.streams.iter().find(|stream| {
                stream.source_instance_id == batch.source_instance_id
                    && stream.source_generation == SOURCE_GENERATION
            }) else {
                continue;
            };
            if replaced.replace(index).is_some() {
                return Err(Error::Corrupt("duplicate agent transcript stream"));
            }
            chunks = transcript_chunk_references(base, stream).await?;
            logical_byte_length = stream.logical_byte_length;
            record_count = stream.record_count;
            gaps = stream.gaps;
        }

        if let (Some(previous), Some(next)) = (chunks.last(), additions.first())
            && previous.0 >= next.sequence
        {
            return Err(Error::Stale);
        }
        chunks.extend(
            additions
                .iter()
                .map(|chunk| (chunk.sequence, chunk.reference)),
        );
        logical_byte_length = logical_byte_length
            .saturating_add(additions.iter().map(|chunk| chunk.bytes).sum::<u64>());
        record_count = record_count.saturating_add(additions.len() as u64);
        gaps = gaps.saturating_add(super::agent_capture::capture_gaps(batch, normalized));

        let ordered_chunk_root = self
            .stage_transcript_pages(batch.source_instance_id, &chunks)
            .await?;
        let manifest = self
            .stage_agent_object(&AgentObjectV1::TranscriptManifest(TranscriptManifestV1 {
                schema_version: 1,
                session_id: session.session_id,
                streams: vec![TranscriptStreamV1 {
                    source_instance_id: batch.source_instance_id,
                    source_generation: SOURCE_GENERATION,
                    source_format: source_format(batch.integration).into(),
                    decoder_version: normalized.profile.identity(),
                    sanitizer_version: batch.sanitizer_version.clone(),
                    representation: batch.representation,
                    ordered_chunk_root,
                    logical_byte_length,
                    record_count,
                    gaps,
                }],
                coverage: CoverageV1 {
                    completeness: CompletenessV1::Partial,
                    gaps,
                    redacted_fields: 0,
                    truncated: false,
                },
            }))
            .await?;
        match replaced {
            Some(index) => session.transcripts[index] = manifest,
            None => session.transcripts.push(manifest),
        }
        Ok(())
    }

    async fn stage_transcript_pages(
        &self,
        source_instance_id: jj_agent::SourceInstanceId,
        chunks: &[(u64, AgentObjectRef)],
    ) -> Result<AgentObjectRef> {
        let mut next = None;
        for chunk in chunks.chunks(TRANSCRIPT_PAGE_ENTRIES).rev() {
            next = Some(
                self.stage_agent_object(&AgentObjectV1::TranscriptPage(TranscriptPageV1 {
                    schema_version: 1,
                    source_instance_id,
                    source_generation: SOURCE_GENERATION,
                    chunks: chunk.iter().map(|(_, reference)| *reference).collect(),
                    next,
                }))
                .await?,
            );
        }
        next.ok_or(Error::Corrupt("empty agent transcript"))
    }
}

async fn transcript_chunk_references(
    base: &dyn AgentSnapshotRead,
    stream: &TranscriptStreamV1,
) -> Result<Vec<(u64, AgentObjectRef)>> {
    let mut values = Vec::new();
    let mut page = Some(stream.ordered_chunk_root);
    let mut seen = BTreeSet::new();
    while let Some(reference) = page {
        if !seen.insert(reference) {
            return Err(Error::Corrupt("agent transcript page cycle"));
        }
        let AgentObjectV1::TranscriptPage(value) = base.read_object(reference).await? else {
            return Err(Error::Corrupt("agent transcript page"));
        };
        if value.source_instance_id != stream.source_instance_id
            || value.source_generation != stream.source_generation
        {
            return Err(Error::Corrupt("agent transcript stream identity"));
        }
        for chunk in value.chunks {
            let AgentObjectV1::TranscriptChunk(value) = base.read_object(chunk).await? else {
                return Err(Error::Corrupt("agent transcript chunk"));
            };
            if value.source_instance_id != stream.source_instance_id
                || value.source_generation != stream.source_generation
                || values
                    .last()
                    .is_some_and(|(sequence, _)| *sequence >= value.sequence)
            {
                return Err(Error::Corrupt("agent transcript chunk order"));
            }
            values.push((value.sequence, chunk));
        }
        page = value.next;
    }
    if values.len() as u64 != stream.record_count {
        return Err(Error::Corrupt("agent transcript record count"));
    }
    Ok(values)
}

fn source_format(integration: IntegrationId) -> &'static str {
    match integration {
        IntegrationId::Codex => "codex-normalized-messages-v1",
        IntegrationId::Cursor => "cursor-normalized-messages-v1",
        IntegrationId::OpenCode => "opencode-normalized-messages-v1",
        IntegrationId::OwnedController => "owned-controller-normalized-messages-v1",
    }
}
