//! API snapshot coverage is ordered by source sequence, never JSONL byte offsets.
use super::*;
use crate::reader_progress::ReaderProgressV1;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnapshotPublicationProgress {
    pub generation: u64,
    pub history_available: bool,
    pub captured_sequence: u64,
    pub captured_logical_records: u64,
    pub committed_sequence: u64,
    pub committed_logical_records: Option<u64>,
    pub outstanding_ranges: usize,
    pub receipts: u64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnapshotPublicationInspection {
    pub boundary_available: bool,
    pub captured_sequence: u64,
    pub committed_sequence: u64,
    pub completed: bool,
    pub tail: Option<SnapshotPublicationProgress>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SnapshotPublicationQuery {
    pub integration: IntegrationId,
    pub source: SourceInstanceId,
    pub generation_digest: [u8; 32],
}
impl AgentCaptureSpool {
    /// One bounded body-free metadata read for a management page of at most 100 sources.
    pub fn inspect_snapshot_publication_progress_batch(
        &self,
        queries: &[SnapshotPublicationQuery],
    ) -> CaptureResult<Vec<Option<SnapshotPublicationProgress>>> {
        if queries.len() > 100 || queries.iter().any(|q| q.generation_digest == [0; 32]) {
            return Err("invalid snapshot publication query bounds".into());
        }
        if queries.is_empty() {
            return Ok(Vec::new());
        }
        let selected = self.inspect_publication_metadata(|catalog, records| {
            queries
                .iter()
                .map(|q| {
                    History::select(
                        catalog,
                        records,
                        q.integration,
                        q.source,
                        q.generation_digest,
                    )?
                    .tail()
                })
                .collect::<CaptureResult<Vec<_>>>()
                .map(Some)
        })?;
        Ok(selected.unwrap_or_else(|| vec![None; queries.len()]))
    }

    /// Body-free proof under existing FIFO/mutation coordination; no initialization.
    pub fn inspect_snapshot_publication(
        &self,
        integration: IntegrationId,
        source: SourceInstanceId,
        digest: [u8; 32],
        boundary: u64,
    ) -> CaptureResult<Option<SnapshotPublicationInspection>> {
        if digest == [0; 32] || boundary == 0 {
            return Err("invalid snapshot publication boundary".into());
        }
        self.inspect_publication_metadata(|catalog, records| {
            let history = History::select(catalog, records, integration, source, digest)?;
            let Some(mut result) = history.boundary(boundary)? else {
                return Ok(None);
            };
            // Accepted boundary proof survives unavailable later tail evidence.
            result.tail = history.tail().ok().flatten();
            Ok(Some(result))
        })
    }
}
struct History<'a> {
    channels: Vec<&'a Channel>,
    records: BTreeMap<u64, &'a ReaderProgressV1>,
}
impl<'a> History<'a> {
    fn select(
        catalog: &'a Catalog,
        records: &'a [ReaderProgressV1],
        integration: IntegrationId,
        source: SourceInstanceId,
        digest: [u8; 32],
    ) -> CaptureResult<Self> {
        let mut channels: Vec<_> = catalog
            .channels
            .iter()
            .filter(|c| {
                c.key.integration == integration && c.key.source == source && c.key.digest == digest
            })
            .collect();
        channels.sort_by_key(|c| c.key.generation);
        if channels.iter().any(|c| c.latest.snapshot.is_none()) {
            return Err("publication snapshot position unavailable".into());
        }
        let mut retained = BTreeMap::<u64, &ReaderProgressV1>::new();
        for progress in records.iter().filter(|p| {
            p.cursor.integration == integration
                && p.cursor.source_instance_id == source
                && p.metadata.generation_digest == digest
        }) {
            let old = retained
                .entry(progress.cursor.generation)
                .or_insert(progress);
            if progress.last_sequence > old.last_sequence {
                *old = progress;
            }
        }
        Ok(Self {
            channels,
            records: retained,
        })
    }
    fn qualified(&self, channel: &Channel) -> bool {
        channel.history_available
            && self
                .records
                .get(&channel.key.generation)
                .is_some_and(|p| channel.latest.matches_progress(p))
            && self
                .records
                .range(..=channel.key.generation)
                .all(|(g, _)| self.channels.iter().any(|c| c.key.generation == *g))
    }
    fn boundary(&self, boundary: u64) -> CaptureResult<Option<SnapshotPublicationInspection>> {
        if self.channels.is_empty() {
            return Ok(None);
        }
        let mut result = SnapshotPublicationInspection {
            boundary_available: true,
            captured_sequence: 0,
            committed_sequence: 0,
            completed: false,
            tail: None,
        };
        let mut previous: Option<&Range> = None;
        let mut blocked = false;
        for channel in &self.channels {
            result.boundary_available &= self.qualified(channel);
            let first = first_range(channel)?;
            if previous.is_some_and(|p| !first.follows(p)) {
                result.boundary_available = false;
            }
            previous = Some(&channel.latest);
            result.captured_sequence = channel.latest.last.min(boundary);
            if !blocked && result.boundary_available {
                if let Some(frontier) = &channel.frontier {
                    result.committed_sequence = frontier.range.last.min(boundary);
                }
                if result.committed_sequence == boundary {
                    result.completed = true;
                    return Ok(Some(result));
                }
            }
            blocked |= !channel.pending.is_empty();
        }
        if !result.boundary_available {
            result.committed_sequence = 0;
        }
        Ok(Some(result))
    }
    fn tail(&self) -> CaptureResult<Option<SnapshotPublicationProgress>> {
        let Some(latest) = self.channels.last() else {
            return Ok(None);
        };
        let snapshot = latest
            .latest
            .snapshot
            .ok_or("snapshot position unavailable")?;
        let mut result = SnapshotPublicationProgress {
            generation: latest.key.generation,
            history_available: self.records.len() == self.channels.len(),
            captured_sequence: latest.latest.last,
            captured_logical_records: snapshot.logical_records,
            committed_sequence: 0,
            committed_logical_records: None,
            outstanding_ranges: 0,
            receipts: 0,
        };
        let mut previous: Option<&Range> = None;
        let mut blocked = false;
        for channel in &self.channels {
            result.history_available &= self.qualified(channel);
            let first = first_range(channel)?;
            if previous.is_some_and(|p| !first.follows(p)) {
                return Err("publication snapshot generation order differs".into());
            }
            previous = Some(&channel.latest);
            if !blocked {
                if let Some(frontier) = &channel.frontier {
                    result.committed_sequence = frontier.range.last;
                    result.committed_logical_records = Some(
                        frontier
                            .range
                            .snapshot
                            .ok_or("snapshot frontier position unavailable")?
                            .logical_records,
                    );
                }
                blocked = !channel.pending.is_empty();
            }
            result.outstanding_ranges += channel.pending.len();
            result.receipts = result
                .receipts
                .checked_add(channel.receipts)
                .ok_or("publication receipt count exhausted")?;
        }
        if !result.history_available {
            result.committed_sequence = 0;
            result.committed_logical_records = None;
        }
        Ok(Some(result))
    }
}
fn first_range(channel: &Channel) -> CaptureResult<&Range> {
    channel
        .frontier
        .as_ref()
        .or_else(|| channel.pending.first())
        .map(|r| &r.range)
        .ok_or_else(|| "publication snapshot range unavailable".into())
}
