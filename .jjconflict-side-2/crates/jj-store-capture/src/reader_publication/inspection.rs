//! Selected-boundary proof and separate tail coverage, with no pending-body reads.
use super::*;
use crate::reader_progress::ReaderProgressV1;
use std::{collections::BTreeMap, fs};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReaderPublicationInspection {
    pub boundary_available: bool,
    pub captured_byte_offset: u64,
    pub committed_byte_offset: u64,
    pub completed: bool,
    pub tail: Option<ReaderPublicationProgress>,
}
impl AgentCaptureSpool {
    /// Uses the existing fork coordination and lock, without initialization or recovery.
    /// Only publication and persisted reader metadata are read, never pending bodies.
    pub fn inspect_reader_publication(
        &self,
        integration: IntegrationId,
        source: SourceInstanceId,
        digest: [u8; 32],
        boundary: u64,
    ) -> CaptureResult<Option<ReaderPublicationInspection>> {
        if digest == [0; 32] || boundary == 0 {
            return Err("invalid publication inspection boundary".into());
        }
        self.inspect_publication_metadata(|catalog, records| {
            let mut result =
                selected_boundary(catalog, records, integration, source, digest, boundary)?;
            if let Some(result) = &mut result {
                // Tail failures do not invalidate independently proven accepted boundaries.
                result.tail =
                    projection::physical_history(catalog, records, integration, source, digest)
                        .ok()
                        .flatten();
            }
            Ok(result)
        })
    }
    pub(super) fn inspect_publication_metadata<T>(
        &self,
        project: impl FnOnce(&Catalog, &[ReaderProgressV1]) -> CaptureResult<Option<T>>,
    ) -> CaptureResult<Option<T>> {
        let _turn = self.fork_inspection_turn()?;
        let path = self.state_root().join("mutation.lock");
        let metadata = fs::symlink_metadata(&path)?;
        if !metadata.is_file() || metadata.file_type().is_symlink() {
            return Err("invalid inspection lock".into());
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            if metadata.permissions().mode() & 0o077 != 0 {
                return Err("inspection lock permissions".into());
            }
        }
        let file = fs::File::open(&path)?;
        file.try_lock().map_err(|_| "inspection lock unavailable")?;
        let _lock = crate::state_lease::StateLease::from_locked(file);
        let Some(catalog) = files::inspect(self.state_root())? else {
            return Ok(None);
        };
        let records = crate::reader_progress_files::records(self.state_root())?;
        project(&catalog, &records)
    }
}
fn selected_boundary(
    catalog: &Catalog,
    records: &[ReaderProgressV1],
    integration: IntegrationId,
    source: SourceInstanceId,
    digest: [u8; 32],
    boundary: u64,
) -> CaptureResult<Option<ReaderPublicationInspection>> {
    let mut channels: Vec<_> = catalog
        .channels
        .iter()
        .filter(|c| {
            c.key.integration == integration && c.key.source == source && c.key.digest == digest
        })
        .collect();
    channels.sort_by_key(|c| c.key.generation);
    if channels.is_empty() {
        return Ok(None);
    }
    let mut retained = BTreeMap::<u64, &ReaderProgressV1>::new();
    for p in records.iter().filter(|p| {
        p.cursor.integration == integration
            && p.cursor.source_instance_id == source
            && p.metadata.generation_digest == digest
    }) {
        let previous = retained.entry(p.cursor.generation).or_insert(p);
        if p.last_sequence > previous.last_sequence {
            *previous = p;
        }
    }
    let mut result = ReaderPublicationInspection {
        boundary_available: true,
        captured_byte_offset: 0,
        committed_byte_offset: 0,
        completed: false,
        tail: None,
    };
    let mut previous: Option<&Range> = None;
    let mut blocked = false;
    for channel in &channels {
        if channel.latest.snapshot.is_some() {
            return Err("publication byte position unavailable".into());
        }
        result.boundary_available &= channel.history_available
            && retained
                .get(&channel.key.generation)
                .is_some_and(|p| channel.latest.matches_progress(p))
            && retained
                .range(..=channel.key.generation)
                .all(|(generation, _)| channels.iter().any(|c| c.key.generation == *generation));
        let first = channel
            .frontier
            .as_ref()
            .or_else(|| channel.pending.first())
            .ok_or("publication channel range unavailable")?;
        if previous.is_some_and(|p| !first.range.follows(p)) {
            result.boundary_available = false;
        }
        previous = Some(&channel.latest);
        result.captured_byte_offset = channel.latest.offset.min(boundary);
        if !blocked && result.boundary_available {
            if let Some(frontier) = &channel.frontier {
                result.committed_byte_offset = frontier.range.offset.min(boundary);
            }
            if result.committed_byte_offset == boundary {
                result.completed = true;
                return Ok(Some(result));
            }
        }
        blocked |= !channel.pending.is_empty();
    }
    if !result.boundary_available {
        result.committed_byte_offset = 0;
    }
    Ok(Some(result))
}
