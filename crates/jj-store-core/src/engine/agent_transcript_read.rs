//! Chronological bounded reads of current immutable append tails.
use super::*;
use jj_agent::{TranscriptChunkV1, TranscriptStreamV1};

enum Frame {
    Page(AgentObjectRef),
    Chunks(Vec<AgentObjectRef>),
}
pub(super) async fn read_range(
    snapshot: &dyn AgentSnapshotRead,
    stream: &TranscriptStreamV1,
    after_sequence: Option<u64>,
    limit: usize,
    max_bytes: usize,
) -> Result<TranscriptRange> {
    let mut pending = vec![Frame::Page(stream.ordered_chunk_root)];
    let mut seen = BTreeSet::new();
    let mut chunks: Vec<TranscriptChunkV1> = Vec::with_capacity(limit);
    let mut bytes = 0;
    let mut previous = None;
    while let Some(frame) = pending.pop() {
        match frame {
            Frame::Page(reference) => {
                if !seen.insert(reference) || seen.len() > 4096 {
                    return Err(Error::Limit("agent transcript pages"));
                }
                let (source, generation) = match snapshot.read_object(reference).await? {
                    AgentObjectV1::TranscriptTailPage(page) => {
                        pending.push(Frame::Chunks(page.chunks));
                        if let Some(prefix) = page.prefix {
                            pending.push(Frame::Page(prefix));
                        }
                        (page.source_instance_id, page.source_generation)
                    }
                    _ => return Err(Error::Corrupt("agent transcript page kind")),
                };
                if source != stream.source_instance_id || generation != stream.source_generation {
                    return Err(Error::Corrupt("agent transcript stream identity"));
                }
            }
            Frame::Chunks(references) => {
                for reference in references {
                    let AgentObjectV1::TranscriptChunk(chunk) =
                        snapshot.read_object(reference).await?
                    else {
                        return Err(Error::Corrupt("agent transcript chunk kind"));
                    };
                    if chunk.source_instance_id != stream.source_instance_id
                        || chunk.source_generation != stream.source_generation
                        || previous.is_some_and(|value| chunk.sequence <= value)
                    {
                        return Err(Error::Corrupt("agent transcript chunk order"));
                    }
                    previous = Some(chunk.sequence);
                    if after_sequence.is_some_and(|value| chunk.sequence <= value) {
                        continue;
                    }
                    if chunks.len() == limit || bytes + chunk.payload.bytes().len() > max_bytes {
                        if chunks.is_empty() {
                            return Err(Error::Limit("agent transcript record"));
                        }
                        return Ok(TranscriptRange {
                            next_after_sequence: chunks.last().map(|c| c.sequence),
                            chunks,
                        });
                    }
                    bytes += chunk.payload.bytes().len();
                    chunks.push(chunk);
                }
            }
        }
    }
    Ok(TranscriptRange {
        chunks,
        next_after_sequence: None,
    })
}
