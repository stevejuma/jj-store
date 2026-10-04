use super::*;
use jj_store_wal::publication::{
    AgentEventV1, AgentSessionSnapshotV1, TranscriptChunkV1, normalized_event_id,
};

/// Match bounded selected chunks to their exact normalized messages and content.
pub(crate) async fn present_chunks(
    store: &dyn wal::publication::AgentSnapshotRead,
    session: &AgentSessionSnapshotV1,
    chunks: Vec<TranscriptChunkV1>,
) -> Result<Vec<AgentTranscriptChunk>> {
    let mut times = vec![None; chunks.len()];
    let candidates = chunks
        .iter()
        .enumerate()
        .filter(|(_, chunk)| chunk.source_generation == 1)
        .map(|(index, chunk)| {
            (
                normalized_event_id(
                    chunk.source_instance_id,
                    session.integration,
                    chunk.sequence,
                    "message",
                ),
                index,
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut inspected = 0usize;
    for reference in &session.event_batches {
        let AgentObjectV1::EventBatch(batch) =
            store.read_object(*reference).await.map_err(map_wal)?
        else {
            return Err(Error::Corrupt);
        };
        inspected = inspected
            .checked_add(batch.events.len())
            .ok_or(Error::Limit)?;
        if inspected > super::agent_session_timing::MAX_TIMING_EVENTS {
            times.fill(None);
            break;
        }
        let AgentObjectV1::ObservationBatch(evidence) =
            store.read_object(batch.evidence).await.map_err(map_wal)?
        else {
            return Err(Error::Corrupt);
        };
        if evidence.integration != session.integration {
            return Err(Error::Corrupt);
        }
        for event in batch.events {
            let AgentEventV1::MessageCompleted {
                event_id,
                session_id,
                content,
                ..
            } = event
            else {
                continue;
            };
            if session_id != session.session_id {
                continue;
            }
            let Some(&index) = candidates.get(&event_id) else {
                continue;
            };
            let chunk = &chunks[index];
            if chunk.source_instance_id != evidence.source_instance_id {
                continue;
            }
            let AgentObjectV1::Content(selected) =
                store.read_object(content).await.map_err(map_wal)?
            else {
                return Err(Error::Corrupt);
            };
            if selected.payload == chunk.payload {
                times[index] = super::agent_session_timing::event_at_millis(
                    &evidence,
                    &batch.normalizer_version,
                    event_id,
                    "message",
                );
            }
        }
    }
    chunks
        .into_iter()
        .zip(times)
        .map(|(chunk, occurred_at_millis)| {
            Ok(AgentTranscriptChunk {
                sequence: chunk.sequence,
                occurred_at_millis,
                media_type: chunk.payload.media_type(),
                content: String::from_utf8(chunk.payload.bytes().to_vec())
                    .map_err(|_| Error::Corrupt)?,
            })
        })
        .collect()
}
