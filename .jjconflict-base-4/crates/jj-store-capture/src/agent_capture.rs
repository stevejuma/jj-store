//! Durable local capture intent. A reader cursor is acknowledged only after
//! sanitized evidence reaches this journal; publication requires exact durable intent.
use crate::CaptureResult as AuthorityResult;
use jj_agent::{
    CaptureBatchId, CaptureCursorV1, NormalizerProfileV1, SanitizedObservationBatchV1,
    SourcePositionV1,
};
use jj_store_model::{PublicationId, RepositoryId, RequestId};
use jj_store_wal::publication::AgentDeltaRequest;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Read as _,
    path::{Path, PathBuf},
};

use crate::agent_capture_files::{
    compatibility_lock, recover_pending, restrict_directory, sync_dir,
};

mod budget;

/// Body-free exact physical byte requirement of a rejected spool operation.
/// This observation supplies no authorization or guarantee about later capacity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("capture spool byte limit exceeded")]
pub struct CaptureCapacityRefusal {
    pub limit_bytes: u64,
    pub required_bytes: u64,
}

pub(crate) const MAX_ENTRIES: usize = 4096;

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reader_metadata: Option<crate::reader_progress::ReaderCaptureMetadataV1>,
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
    _compatibility_lock: std::sync::Arc<crate::state_lease::StateLease>,
    fork_mutations: std::sync::Arc<crate::agent_capture_lock::ForkMutations>,
}

impl AgentCaptureSpool {
    pub(crate) fn fork_inspection_turn(
        &self,
    ) -> AuthorityResult<crate::agent_capture_lock::ForkTurn> {
        self.fork_mutations.acquire()
    }
    pub fn pending_capacity(&self) -> u64 {
        self.max_pending_bytes
    }
    /// Independent in-process handle; existing file locks still serialize every
    /// mutation and drain. No filesystem work, rebind or raw copying is performed.
    pub fn fork_handle(&self) -> Self {
        Self {
            root: self.root.clone(),
            pending: self.pending.clone(),
            max_pending_bytes: self.max_pending_bytes,
            _compatibility_lock: self._compatibility_lock.clone(),
            fork_mutations: self.fork_mutations.clone(),
        }
    }
    pub fn open(path: &Path, max_pending_bytes: u64) -> AuthorityResult<Self> {
        if max_pending_bytes == 0 {
            return Err("capture spool byte limit must be positive".into());
        }
        let root = crate::path_policy::outside_checkout(path)?;
        if fs::symlink_metadata(&root).is_ok_and(|value| value.file_type().is_symlink()) {
            return Err("capture spool must not be a symlink".into());
        }
        fs::create_dir_all(&root)?;
        restrict_directory(&root)?;
        let compatibility = compatibility_lock(&root.join("capture.lock"))?;
        let lock = crate::state_lease::StateLease::exclusive(&root.join("mutation.lock"))?;
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
            _compatibility_lock: std::sync::Arc::new(compatibility),
            fork_mutations: Default::default(),
        };
        spool.recover_reader_progress()?;
        spool.recover_reader_publication()?;
        drop(lock);
        Ok(spool)
    }

    // Drainers hold this lease across awaits, but never the mutation lock.
    // Independent handles can enqueue during a drain.
    pub(crate) fn drain_lease(&self) -> AuthorityResult<crate::state_lease::StateLease> {
        crate::state_lease::StateLease::exclusive(&self.root.join("drain.lock"))
    }

    pub(crate) fn mutation_lock(
        &self,
    ) -> AuthorityResult<crate::agent_capture_lock::CaptureMutationLease> {
        let turn = self.fork_mutations.acquire()?;
        let lock = crate::state_lease::StateLease::exclusive(&self.root.join("mutation.lock"))?;
        if !fs::symlink_metadata(&self.pending)?.is_dir() {
            return Err("capture spool pending path must be a directory".into());
        }
        // A crashed writer may have left a temporary file since this handle opened.
        recover_pending(&self.pending)?;
        crate::reader_progress_files::recover_temporary(&self.root)?;
        Ok(crate::agent_capture_lock::CaptureMutationLease {
            _file: lock,
            _turn: turn,
        })
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

    pub(crate) fn pending_locked(&self) -> AuthorityResult<Vec<PendingCaptureV1>> {
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
            match &entry.state {
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
            self.record_reader_progress(&entry)?;
            fs::remove_file(path)?;
            sync_dir(&self.pending)?;
        }
        Ok(())
    }

    pub(crate) fn state_root(&self) -> &Path {
        &self.root
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
}

pub(crate) fn same_capture(left: &PendingCaptureV1, right: &PendingCaptureV1) -> bool {
    left.repository == right.repository
        && left.profile == right.profile
        && left.cursor == right.cursor
        && left.batch == right.batch
        && left.thread_binding == right.thread_binding
        && left.reader_metadata == right.reader_metadata
}

pub(crate) fn ack(entry: &PendingCaptureV1) -> DurableCaptureAck {
    DurableCaptureAck {
        batch_id: entry.batch.batch_id,
        position: entry.cursor.position.clone(),
    }
}
