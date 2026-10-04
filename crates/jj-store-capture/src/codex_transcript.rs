//! Version-bound read-only source. Raw transcript records remain in memory.
use crate::{
    CaptureResult,
    agent_capture::DurableCaptureAck,
    profile::CaptureProfile,
    reader_progress::{ReaderCaptureMetadataV1, ReaderProgressV1},
};
use jj_agent::{CaptureBatchId, CaptureCursorV1, SourceInstanceId, SourcePositionV1};
use sha2::{Digest as _, Sha256};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
pub(crate) mod context;
pub(crate) mod file;
mod poll;
mod progress;
pub(crate) mod recovery;
use crate::codex_discovery::PhysicalId;
pub use context::CodexDirectoryContext;
use context::{ContextReconstruction, DirectoryHistory};
use file::{Header, Stamp};
pub use recovery::{CodexBatchRecovery, CodexRecoveryPoll, RecoveredCodexDirectory};
pub(crate) fn native_session_key_valid(value: &str) -> bool {
    file::uuid_label(value)
}

pub const MAX_RECORD_BYTES: usize = 1024 * 1024;
pub const MAX_SCAN_BYTES: usize = 1024 * 1024;
pub const MAX_VERIFY_BYTES: usize = 4 * 1024 * 1024;
pub const MAX_BATCH_RECORDS: usize = 256;

// These types intentionally have no Serialize or Debug implementation.
pub struct RawTranscriptRecord {
    pub sequence: u64,
    /// None is an oversized complete record, represented later as a safe gap.
    pub bytes: Option<Vec<u8>>,
    pub record_key: Option<[u8; 32]>,
}
#[derive(Clone)]
pub struct CodexTranscriptBatch {
    pub batch_id: CaptureBatchId,
    pub cursor: CaptureCursorV1,
    pub metadata: ReaderCaptureMetadataV1,
    pub records: Arc<Vec<RawTranscriptRecord>>,
    pub directory_context: Arc<CodexDirectoryContext>,
}
impl CodexTranscriptBatch {
    pub fn directory(&self) -> Option<&Path> {
        self.directory_context.directory()
    }
}
pub enum TranscriptPoll {
    Verifying,
    Waiting,
    ChangedDuringRead,
    Ready(Box<CodexTranscriptBatch>),
}
struct Verification {
    stamp: Stamp,
    offset: u64,
    expected: [u8; 32],
    hash: Sha256,
    context: ContextReconstruction,
}
struct Scan {
    stamp: Stamp,
    offset: u64,
    hash: Sha256,
    line: Vec<u8>,
    oversized: bool,
    history: DirectoryHistory,
}
struct Pending {
    batch: CodexTranscriptBatch,
    hash: Sha256,
    last_sequence: u64,
    stamp: Stamp,
    history: DirectoryHistory,
}
pub struct CodexTranscriptReader {
    path: PathBuf,
    physical_identity: Option<PhysicalId>,
    pub(crate) profile: CaptureProfile,
    pub(crate) source: SourceInstanceId,
    header: Header,
    generation: u64,
    generation_floor: u64,
    sequence: u64,
    offset: u64,
    prefix: [u8; 32],
    hash: Sha256,
    stamp: Stamp,
    verification: Option<Verification>,
    scan: Option<Scan>,
    pending: Option<Pending>,
    history: DirectoryHistory,
}
impl CodexTranscriptReader {
    pub(crate) fn has_pending_batch(&self) -> bool {
        self.pending.is_some()
    }
    pub(crate) fn idle_unchanged(&self) -> CaptureResult<bool> {
        if self.pending.is_some() || self.verification.is_some() {
            return Ok(false);
        }
        let expected = if let Some(scan) = &self.scan {
            if scan.offset != scan.stamp.len {
                return Ok(false);
            }
            &scan.stamp
        } else {
            if self.offset != self.stamp.len {
                return Ok(false);
            }
            &self.stamp
        };
        let (_, header, stamp) =
            file::inspect_bound(self.profile, &self.path, self.physical_identity)?;
        Ok(header.witness == self.header.witness && &stamp == expected)
    }
    pub(crate) fn compact_idle(&mut self) -> bool {
        if self.pending.is_some() || self.verification.is_some() {
            return false;
        }
        let idle = self
            .scan
            .as_ref()
            .map_or(self.offset == self.stamp.len, |scan| {
                scan.offset == scan.stamp.len
            });
        if idle && let Some(scan) = &mut self.scan {
            scan.line = Vec::new();
        }
        idle
    }
    pub(crate) fn header_witness(&self) -> [u8; 32] {
        self.header.witness
    }
    pub fn open(
        profile: CaptureProfile,
        path: &Path,
        source: SourceInstanceId,
        previous: Option<ReaderProgressV1>,
    ) -> CaptureResult<Self> {
        Self::open_bound(profile, path, source, previous, None)
    }
    pub(crate) fn open_bound(
        profile: CaptureProfile,
        path: &Path,
        source: SourceInstanceId,
        previous: Option<ReaderProgressV1>,
        physical_identity: Option<PhysicalId>,
    ) -> CaptureResult<Self> {
        let (_, header, stamp) = file::inspect_bound(profile, path, physical_identity)?;
        let history = DirectoryHistory::new(&header.directory);
        let mut reader = Self {
            path: path.to_owned(),
            physical_identity,
            profile,
            source,
            header,
            generation: 1,
            generation_floor: 0,
            sequence: 0,
            offset: 0,
            prefix: [0; 32],
            hash: Sha256::new(),
            stamp,
            verification: None,
            scan: None,
            pending: None,
            history,
        };
        if let Some(previous) = previous {
            previous.validate()?;
            if previous.cursor.integration != profile.integration()
                || previous.cursor.source_instance_id != source
            {
                return Err("transcript reader source mismatch".into());
            }
            let SourcePositionV1::JsonLines {
                complete_byte_offset,
                prefix_sha256,
            } = previous.cursor.position
            else {
                return Err("transcript reader position mismatch".into());
            };
            reader.generation = previous.cursor.generation;
            reader.sequence = previous.last_sequence;
            if previous.metadata.generation_digest == reader.header.witness
                && reader.stamp.len >= complete_byte_offset
            {
                if previous.cursor.parser_version != reader.parser()
                    || previous.cursor.native_session_key.as_ref() != Some(&reader.header.session)
                {
                    return Err("transcript generation parser or alias mismatch".into());
                }
                reader.offset = complete_byte_offset;
                reader.prefix = prefix_sha256;
                reader.start_verification(reader.stamp.clone());
            } else {
                reader.new_generation()?;
            }
        }
        Ok(reader)
    }
    pub fn directory(&self) -> &Path {
        &self.header.directory
    }
    pub fn native_session_key(&self) -> &str {
        &self.header.session
    }
    pub fn generation(&self) -> u64 {
        self.generation
    }
    fn parser(&self) -> String {
        self.profile.parser()
    }
    fn start_verification(&mut self, stamp: Stamp) {
        self.verification = Some(Verification {
            stamp,
            offset: 0,
            expected: self.prefix,
            hash: Sha256::new(),
            context: ContextReconstruction::new(&self.header.directory),
        });
        self.scan = None;
    }
    fn new_generation(&mut self) -> CaptureResult<()> {
        self.generation = self
            .generation
            .max(self.generation_floor)
            .checked_add(1)
            .ok_or("transcript generation limit")?;
        self.offset = 0;
        self.prefix = [0; 32];
        self.hash = Sha256::new();
        self.history = DirectoryHistory::new(&self.header.directory);
        self.verification = None;
        self.scan = None;
        Ok(())
    }
    /// Only the sanitized durable spool acknowledgement advances this reader.
    pub fn acknowledge(&mut self, ack: &DurableCaptureAck) -> CaptureResult<()> {
        let pending = self
            .pending
            .as_ref()
            .ok_or("transcript acknowledgement without batch")?;
        if pending.batch.batch_id != ack.batch_id || pending.batch.cursor.position != ack.position {
            return Err("transcript acknowledgement mismatch".into());
        }
        let pending = self.pending.take().expect("checked pending batch");
        let SourcePositionV1::JsonLines {
            complete_byte_offset,
            prefix_sha256,
        } = pending.batch.cursor.position
        else {
            unreachable!()
        };
        self.offset = complete_byte_offset;
        self.prefix = prefix_sha256;
        self.sequence = pending.last_sequence;
        self.hash = pending.hash;
        self.stamp = pending.stamp;
        self.history = pending.history;
        Ok(())
    }
}
