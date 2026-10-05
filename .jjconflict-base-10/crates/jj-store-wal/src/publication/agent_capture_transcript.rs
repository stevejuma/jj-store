use super::agent_capture_records::TranscriptChunkSelection;
use super::*;
use jj_agent::{
    AgentObjectV1, AgentSessionSnapshotV1, CompletenessV1, CoverageV1, IntegrationId,
    NormalizationBatchV1, SanitizedObservationBatchV1, TranscriptManifestV1, TranscriptStreamV1,
};

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
        let mut replaced = None;
        let mut retained = None;
        for (index, reference) in session.transcripts.iter().copied().enumerate() {
            let AgentObjectV1::TranscriptManifest(manifest) = base.read_object(reference).await?
            else {
                return Err(Error::Corrupt("agent transcript selection"));
            };
            if manifest.session_id != session.session_id {
                return Err(Error::Corrupt("agent transcript session"));
            }
            let Some(stream_index) = manifest.streams.iter().position(|stream| {
                stream.source_instance_id == batch.source_instance_id
                    && stream.source_generation == SOURCE_GENERATION
            }) else {
                continue;
            };
            if replaced.replace((index, stream_index)).is_some() {
                return Err(Error::Corrupt("duplicate agent transcript stream"));
            }
            retained = Some(manifest);
        }
        let prior = retained
            .as_ref()
            .zip(replaced)
            .map(|(manifest, (_, index))| &manifest.streams[index]);
        let added_bytes = additions
            .iter()
            .try_fold(0u64, |sum, c| sum.checked_add(c.bytes))
            .ok_or(Error::Limit("transcript length"))?;
        let logical_byte_length = prior
            .map_or(0, |s| s.logical_byte_length)
            .checked_add(added_bytes)
            .ok_or(Error::Limit("transcript length"))?;
        let record_count = prior
            .map_or(0, |s| s.record_count)
            .checked_add(additions.len() as u64)
            .ok_or(Error::Limit("transcript records"))?;
        let added_gaps = super::agent_capture::capture_gaps(batch, normalized);
        let gaps = prior.map_or(0, |s| s.gaps).saturating_add(added_gaps);
        let ordered_chunk_root = self
            .append_transcript_tail(base, batch, prior, additions)
            .await?;
        let stream = TranscriptStreamV1 {
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
        };
        let mut manifest = retained.unwrap_or(TranscriptManifestV1 {
            schema_version: 1,
            session_id: session.session_id,
            streams: Vec::new(),
            coverage: CoverageV1 {
                completeness: CompletenessV1::Partial,
                gaps: 0,
                redacted_fields: 0,
                truncated: false,
            },
        });
        match replaced {
            Some((_, index)) => manifest.streams[index] = stream,
            None => manifest.streams.push(stream),
        }
        manifest.coverage.completeness = CompletenessV1::Partial;
        manifest.coverage.gaps = manifest.coverage.gaps.saturating_add(added_gaps);
        let reference = self
            .stage_agent_object(&AgentObjectV1::TranscriptManifest(manifest))
            .await?;
        match replaced {
            Some((index, _)) => session.transcripts[index] = reference,
            None => session.transcripts.push(reference),
        }
        Ok(())
    }
}
fn source_format(integration: IntegrationId) -> &'static str {
    match integration {
        IntegrationId::Codex => "codex-normalized-messages-v1",
        IntegrationId::Cursor => "cursor-normalized-messages-v1",
        IntegrationId::OpenCode => "opencode-normalized-messages-v1",
        IntegrationId::OwnedController => "owned-controller-normalized-messages-v1",
    }
}
