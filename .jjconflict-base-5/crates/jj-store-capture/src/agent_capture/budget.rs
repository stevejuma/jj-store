//! Physical reservations use legal JSON whitespace, so every existing writer
//! counts preparation capacity without a new marker, lease or pending schema.
use super::*;
use jj_agent::{
    AgentCatalogEditV1, AgentCatalogKey, AgentCatalogKind, AgentDeltaV1, AgentObjectId,
    AgentObjectKind, AgentObjectRef,
};
use jj_store_model::PrincipalId;
use jj_store_wal::publication::AgentThreadBindingV1;
use std::io::Write as _;

impl AgentCaptureSpool {
    pub(crate) fn check_capture_capacity(
        &self,
        path: &Path,
        entry: &PendingCaptureV1,
    ) -> AuthorityResult<()> {
        let bytes = serde_json::to_vec(entry)?;
        self.check_total(reservation_bytes(entry, bytes.len() as u64)?, Some(path))
    }
    pub(crate) fn persist(&self, path: &Path, entry: &PendingCaptureV1) -> AuthorityResult<()> {
        let bytes = serde_json::to_vec(entry)?;
        let budget = reservation_bytes(entry, bytes.len() as u64)?;
        self.check_total(budget, Some(path))?;
        let mut temporary = tempfile::NamedTempFile::new_in(&self.pending)?;
        temporary.write_all(&bytes)?;
        let mut padding = budget - bytes.len() as u64;
        let spaces = [b' '; 8192];
        while padding > 0 {
            let count = padding.min(spaces.len() as u64) as usize;
            temporary.write_all(&spaces[..count])?;
            padding -= count as u64;
        }
        temporary.as_file().sync_all()?;
        temporary.persist(path).map_err(|error| error.error)?;
        sync_dir(&self.pending)
    }

    pub(super) fn check_total(
        &self,
        replacement: u64,
        replaced: Option<&Path>,
    ) -> AuthorityResult<()> {
        let mut total = replacement;
        let mut count = usize::from(replaced.is_some());
        for entry in fs::read_dir(&self.pending)? {
            let path = entry?.path();
            if replaced.is_some_and(|value| value == path) {
                continue;
            }
            if path.extension().is_none_or(|value| value != "json") {
                return Err("unexpected capture spool entry".into());
            }
            let metadata = fs::symlink_metadata(path)?;
            if !metadata.is_file() || metadata.file_type().is_symlink() {
                return Err("capture spool entry must be a regular file".into());
            }
            count += 1;
            if count > MAX_ENTRIES {
                return Err("capture spool entry limit exceeded".into());
            }
            total = total
                .checked_add(metadata.len())
                .ok_or("capture spool size overflow")?;
        }
        if total > self.max_pending_bytes {
            return Err(CaptureCapacityRefusal {
                limit_bytes: self.max_pending_bytes,
                required_bytes: total,
            }
            .into());
        }
        Ok(())
    }
}

fn reservation_bytes(entry: &PendingCaptureV1, encoded: u64) -> AuthorityResult<u64> {
    if entry.reader_metadata.is_none() || !matches!(entry.state, PendingCaptureStateV1::Captured) {
        return Ok(encoded);
    }
    let captured = serde_json::to_vec(&PendingCaptureStateV1::Captured)?.len() as u64;
    let native = native_state_bytes(entry)?;
    let local = serde_json::to_vec(&PendingCaptureStateV1::LocalPrepared {
        request: crate::local_agent_publication::capture_request(entry),
    })?
    .len() as u64;
    encoded
        .checked_sub(captured)
        .and_then(|base| base.checked_add(native.max(local)))
        .ok_or_else(|| "capture preparation reservation overflow".into())
}

fn native_state_bytes(entry: &PendingCaptureV1) -> AuthorityResult<u64> {
    // Sizing only: deliberately overbounds valid catalog/reference combinations.
    // All V1 IDs are fixed-width hexadecimal. Associations has the longest
    // catalog spelling; TranscriptManifest has the longest reference spelling.
    // Both full references and the maximum V1 edit count are covered. This
    // scaffold never validates, stages, authors or publishes native intent.
    let reference = AgentObjectRef::new(
        AgentObjectKind::TranscriptManifest,
        AgentObjectId::new([255; 32]),
    );
    let edit = AgentCatalogEditV1 {
        catalog: AgentCatalogKind::Associations,
        key: AgentCatalogKey::new([255; 32]),
        expected: Some(reference),
        replacement: Some(reference),
    };
    let edit_bytes = serde_json::to_vec(&edit)?.len() as u64;
    let state = PendingCaptureStateV1::Prepared {
        request: AgentDeltaRequest {
            principal: PrincipalId::new([255; 16]),
            request_id: RequestId::new([255; 16]),
            publication_id: PublicationId::new([255; 16]),
            delta: AgentDeltaV1 {
                schema_version: 1,
                edits: vec![edit],
            },
            result: crate::agent_publication::capture_result(entry.batch.batch_id),
            thread_binding: entry
                .thread_binding
                .clone()
                .map(|binding| AgentThreadBindingV1 {
                    binding,
                    accepted_at_millis: i64::MAX,
                }),
        },
    };
    (serde_json::to_vec(&state)?.len() as u64)
        .checked_add(4095 * (edit_bytes + 1))
        .ok_or_else(|| "capture preparation reservation overflow".into())
}
