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

use crate::agent_capture_files::{
    compatibility_lock, recover_pending, restrict_directory, sync_dir,
};

const MAX_ENTRIES: usize = 4096;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case", deny_unknown_fields)]
pub enum PendingCaptureStateV1 {
    Captured,
    Prepared {
        request: AgentDeltaRequest,
    },
    LocalPrepared {
        request: jj_store_service::local_repository::LocalAgentRequest,
    },
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_binding: Option<jj_collab::ThreadBindingV1>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DurableCaptureAck {
    pub batch_id: CaptureBatchId,
    pub position: SourcePositionV1,
}

pub struct AgentCaptureSpool {
    root: PathBuf,
    pending: PathBuf,
    max_pending_bytes: u64,
    _compatibility_lock: File,
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
        let compatibility = compatibility_lock(&root.join("capture.lock"))?;
        let lock = jj_store_server::policy::exclusive_lock(&root.join("mutation.lock"))?;
        let pending = root.join("pending");
        if fs::symlink_metadata(&pending).is_ok_and(|value| !value.is_dir()) {
            return Err("capture spool pending path must be a directory".into());
        }
        fs::create_dir_all(&pending)?;
        restrict_directory(&pending)?;
        recover_pending(&pending)?;
        let spool = Self {
            root,
            pending,
            max_pending_bytes,
            _compatibility_lock: compatibility,
        };
        spool.pending_locked()?;
        drop(lock);
        Ok(spool)
    }

    // Drainers hold this lease across awaits, but never the mutation lock.
    // Returning an owned guard lets independent handles enqueue during a drain.
    pub(crate) fn drain_lease(&self) -> AuthorityResult<File> {
        Ok(jj_store_server::policy::exclusive_lock(
            &self.root.join("drain.lock"),
        )?)
    }

    pub(crate) fn mutation_lock(&self) -> AuthorityResult<File> {
        let lock = jj_store_server::policy::exclusive_lock(&self.root.join("mutation.lock"))?;
        if !fs::symlink_metadata(&self.pending)?.is_dir() {
            return Err("capture spool pending path must be a directory".into());
        }
        // A crashed writer may have left a temporary file since this handle opened.
        recover_pending(&self.pending)?;
        Ok(lock)
    }

    pub fn enqueue(
        &mut self,
        repository: RepositoryId,
        profile: NormalizerProfileV1,
        cursor: CaptureCursorV1,
        batch: SanitizedObservationBatchV1,
    ) -> AuthorityResult<DurableCaptureAck> {
        self.enqueue_with_binding(repository, profile, cursor, batch, None)
    }
    pub fn enqueue_with_binding(
        &mut self,
        repository: RepositoryId,
        profile: NormalizerProfileV1,
        cursor: CaptureCursorV1,
        batch: SanitizedObservationBatchV1,
        thread_binding: Option<jj_collab::ThreadBindingV1>,
    ) -> AuthorityResult<DurableCaptureAck> {
        let _lock = self.mutation_lock()?;
        let path = self.path(batch.batch_id);
        let candidate = PendingCaptureV1 {
            schema_version: 1,
            repository,
            profile,
            cursor,
            batch,
            state: PendingCaptureStateV1::Captured,
            thread_binding,
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
        let _lock = self.mutation_lock()?;
        let path = self.path(batch_id);
        let mut entry = self.read(&path)?;
        match &entry.state {
            PendingCaptureStateV1::Prepared { request: current } if current == &request => {
                return Ok(());
            }
            PendingCaptureStateV1::Prepared { .. }
            | PendingCaptureStateV1::LocalPrepared { .. } => {
                return Err("capture publication retry differs from durable intent".into());
            }
            PendingCaptureStateV1::Captured => {}
        }
        entry.state = PendingCaptureStateV1::Prepared { request };
        entry.validate()?;
        self.persist(&path, &entry)
    }

    pub fn pending(&self) -> AuthorityResult<Vec<PendingCaptureV1>> {
        let _lock = self.mutation_lock()?;
        self.pending_locked()
    }

    fn pending_locked(&self) -> AuthorityResult<Vec<PendingCaptureV1>> {
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
        let _lock = self.mutation_lock()?;
        let path = self.path(batch_id);
        if path.exists() {
            let entry = self.read(&path)?;
            match entry.state {
                PendingCaptureStateV1::Prepared { request }
                    if request.request_id == request_id
                        && request.publication_id == publication_id => {}
                PendingCaptureStateV1::Prepared { .. }
                | PendingCaptureStateV1::LocalPrepared { .. } => {
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

    pub(crate) fn path(&self, batch_id: CaptureBatchId) -> PathBuf {
        self.pending.join(format!("{batch_id}.json"))
    }

    pub(crate) fn read(&self, path: &Path) -> AuthorityResult<PendingCaptureV1> {
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

    pub(crate) fn persist(&self, path: &Path, entry: &PendingCaptureV1) -> AuthorityResult<()> {
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
        && left.thread_binding == right.thread_binding
}

fn ack(entry: &PendingCaptureV1) -> DurableCaptureAck {
    DurableCaptureAck {
        batch_id: entry.batch.batch_id,
        position: entry.cursor.position.clone(),
    }
}
