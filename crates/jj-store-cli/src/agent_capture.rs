//! Durable local capture intent. A reader cursor is acknowledged only after
//! sanitized evidence reaches this journal; publication starts only after the
//! same entry records the exact delta and request identities.
use jj_agent::{
    CaptureBatchId, CaptureCursorV1, NormalizerProfileV1, SanitizedObservationBatchV1,
    SourcePositionV1,
};
use jj_store_jj::operation_stores::AuthorityResult;
use jj_store_model::{PublicationId, RepositoryId, RequestId};
use jj_store_wal::publication::AgentDeltaRequest;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{Read as _, Write as _},
    path::{Path, PathBuf},
};

use crate::agent_capture_files::{recover_pending, restrict_directory, sync_dir};

const MAX_ENTRIES: usize = 4096;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case", deny_unknown_fields)]
pub enum PendingCaptureStateV1 {
    Captured,
    Prepared { request: AgentDeltaRequest },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PendingCaptureV1 {
    pub schema_version: u32,
    pub repository: RepositoryId,
    pub profile: NormalizerProfileV1,
    pub cursor: CaptureCursorV1,
    pub batch: SanitizedObservationBatchV1,
    pub state: PendingCaptureStateV1,
}

impl PendingCaptureV1 {
    fn validate(&self) -> AuthorityResult<()> {
        self.profile.validate()?;
        self.cursor.validate()?;
        self.batch.validate()?;
        let last_sequence = self
            .batch
            .observations
            .last()
            .ok_or("capture batch must contain observations")?
            .source_sequence;
        if self.schema_version != 1
            || self.profile.integration != self.batch.integration
            || self.cursor.integration != self.batch.integration
            || self.cursor.source_instance_id != self.batch.source_instance_id
            || matches!(
                self.cursor.position,
                SourcePositionV1::NativeSequence { sequence } if sequence != last_sequence
            )
        {
            return Err("capture spool identity mismatch".into());
        }
        if let PendingCaptureStateV1::Prepared { request } = &self.state {
            AgentDeltaRequest::new(
                request.principal,
                request.request_id,
                request.publication_id,
                request.delta.clone(),
                request.result.clone(),
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DurableCaptureAck {
    pub batch_id: CaptureBatchId,
    pub position: SourcePositionV1,
}

pub struct AgentCaptureSpool {
    pending: PathBuf,
    max_pending_bytes: u64,
    _lock: File,
}

impl AgentCaptureSpool {
    pub fn open(path: &Path, max_pending_bytes: u64) -> AuthorityResult<Self> {
        if max_pending_bytes == 0 {
            return Err("capture spool byte limit must be positive".into());
        }
        let root = jj_store_server::policy::outside_checkout(path)?;
        if fs::symlink_metadata(&root).is_ok_and(|value| value.file_type().is_symlink()) {
            return Err("capture spool must not be a symlink".into());
        }
        fs::create_dir_all(&root)?;
        restrict_directory(&root)?;
        let pending = root.join("pending");
        fs::create_dir_all(&pending)?;
        restrict_directory(&pending)?;
        let lock = jj_store_server::policy::exclusive_lock(&root.join("capture.lock"))?;
        recover_pending(&pending)?;
        let spool = Self {
            pending,
            max_pending_bytes,
            _lock: lock,
        };
        spool.pending()?;
        Ok(spool)
    }

    pub fn enqueue(
        &mut self,
        repository: RepositoryId,
        profile: NormalizerProfileV1,
        cursor: CaptureCursorV1,
        batch: SanitizedObservationBatchV1,
    ) -> AuthorityResult<DurableCaptureAck> {
        let path = self.path(batch.batch_id);
        let candidate = PendingCaptureV1 {
            schema_version: 1,
            repository,
            profile,
            cursor,
            batch,
            state: PendingCaptureStateV1::Captured,
        };
        candidate.validate()?;
        if path.exists() {
            let existing = self.read(&path)?;
            if !same_capture(&existing, &candidate) {
                return Err("capture retry differs from durable intent".into());
            }
            return Ok(ack(&existing));
        }
        self.persist(&path, &candidate)?;
        Ok(ack(&candidate))
    }

    pub fn prepare(
        &mut self,
        batch_id: CaptureBatchId,
        request: AgentDeltaRequest,
    ) -> AuthorityResult<()> {
        let path = self.path(batch_id);
        let mut entry = self.read(&path)?;
        match &entry.state {
            PendingCaptureStateV1::Prepared { request: current } if current == &request => {
                return Ok(());
            }
            PendingCaptureStateV1::Prepared { .. } => {
                return Err("capture publication retry differs from durable intent".into());
            }
            PendingCaptureStateV1::Captured => {}
        }
        entry.state = PendingCaptureStateV1::Prepared { request };
        entry.validate()?;
        self.persist(&path, &entry)
    }

    pub fn pending(&self) -> AuthorityResult<Vec<PendingCaptureV1>> {
        let mut paths = fs::read_dir(&self.pending)?
            .map(|entry| entry.map(|value| value.path()))
            .collect::<Result<Vec<_>, _>>()?;
        paths.retain(|path| path.extension().is_some_and(|value| value == "json"));
        paths.sort();
        if paths.len() > MAX_ENTRIES {
            return Err("capture spool entry limit exceeded".into());
        }
        let entries = paths
            .iter()
            .map(|path| self.read(path))
            .collect::<AuthorityResult<Vec<_>>>()?;
        self.check_total(0, None)?;
        Ok(entries)
    }

    pub fn complete(
        &mut self,
        batch_id: CaptureBatchId,
        request_id: RequestId,
        publication_id: PublicationId,
    ) -> AuthorityResult<()> {
        let path = self.path(batch_id);
        if path.exists() {
            let entry = self.read(&path)?;
            match entry.state {
                PendingCaptureStateV1::Prepared { request }
                    if request.request_id == request_id
                        && request.publication_id == publication_id => {}
                PendingCaptureStateV1::Prepared { .. } => {
                    return Err("capture completion identity mismatch".into());
                }
                PendingCaptureStateV1::Captured => {
                    return Err("unprepared capture cannot be completed".into());
                }
            }
            fs::remove_file(path)?;
            sync_dir(&self.pending)?;
        }
        Ok(())
    }

    fn path(&self, batch_id: CaptureBatchId) -> PathBuf {
        self.pending.join(format!("{batch_id}.json"))
    }

    fn read(&self, path: &Path) -> AuthorityResult<PendingCaptureV1> {
        let metadata = fs::symlink_metadata(path)?;
        if !metadata.is_file() || metadata.file_type().is_symlink() {
            return Err("capture spool entry must be a regular file".into());
        }
        if metadata.len() > self.max_pending_bytes {
            return Err("capture spool entry exceeds byte limit".into());
        }
        let mut bytes = Vec::with_capacity(metadata.len() as usize);
        File::open(path)?
            .take(self.max_pending_bytes + 1)
            .read_to_end(&mut bytes)?;
        let entry: PendingCaptureV1 =
            serde_json::from_slice(&bytes).map_err(|_| "invalid capture spool entry")?;
        entry.validate()?;
        if self.path(entry.batch.batch_id) != path {
            return Err("capture spool filename mismatch".into());
        }
        Ok(entry)
    }

    fn persist(&self, path: &Path, entry: &PendingCaptureV1) -> AuthorityResult<()> {
        let bytes = serde_json::to_vec(entry)?;
        self.check_total(bytes.len() as u64, Some(path))?;
        let mut temporary = tempfile::NamedTempFile::new_in(&self.pending)?;
        temporary.write_all(&bytes)?;
        temporary.as_file().sync_all()?;
        temporary.persist(path).map_err(|error| error.error)?;
        sync_dir(&self.pending)
    }

    fn check_total(&self, replacement: u64, replaced: Option<&Path>) -> AuthorityResult<()> {
        let mut total = replacement;
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
            total = total
                .checked_add(metadata.len())
                .ok_or("capture spool size overflow")?;
        }
        if total > self.max_pending_bytes {
            return Err("capture spool byte limit exceeded".into());
        }
        Ok(())
    }
}

fn same_capture(left: &PendingCaptureV1, right: &PendingCaptureV1) -> bool {
    left.repository == right.repository
        && left.profile == right.profile
        && left.cursor == right.cursor
        && left.batch == right.batch
}

fn ack(entry: &PendingCaptureV1) -> DurableCaptureAck {
    DurableCaptureAck {
        batch_id: entry.batch.batch_id,
        position: entry.cursor.position.clone(),
    }
}
