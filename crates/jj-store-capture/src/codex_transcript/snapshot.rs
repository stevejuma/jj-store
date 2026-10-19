//! Bounded transient snapshots. No source cursor is acknowledged or persisted.
use super::{CodexTranscriptBatch, CodexTranscriptReader, TranscriptPoll, file};
use crate::{CaptureResult, codex_discovery::CodexSourceCandidate, profile::CaptureProfile};
use jj_agent::SourceInstanceId;
use std::fs::File;

pub const MAX_SNAPSHOT_BYTES: u64 = 64 * 1024 * 1024;

#[derive(Clone, PartialEq, Eq)]
pub struct SnapshotSummary {
    pub complete_bytes: u64,
    pub records: u64,
    pub sha256: [u8; 32],
}
/// Ready batches are tentative native-only data until Complete. Discard on error.
pub enum SnapshotPoll {
    Reading,
    Ready(Box<CodexTranscriptBatch>),
    Complete(SnapshotSummary),
}
/// Holds an immutable physical-file selection; offers no acknowledgement/resume API.
pub struct CodexSnapshotReader {
    reader: CodexTranscriptReader,
    candidate: CodexSourceCandidate,
    file: File,
    stamp: file::Stamp,
    failed: bool,
}
impl CodexSnapshotReader {
    pub fn open(
        profile: CaptureProfile,
        candidate: CodexSourceCandidate,
        source: SourceInstanceId,
        maximum_bytes: u64,
    ) -> CaptureResult<Self> {
        if !matches!(
            profile,
            CaptureProfile::CodexCliJsonlV1 | CaptureProfile::CodexDesktopJsonlV1
        ) || maximum_bytes == 0
            || maximum_bytes > MAX_SNAPSHOT_BYTES
            || !candidate.revalidate()?
        {
            return Err("invalid Codex snapshot selection or bound".into());
        }
        let (file, _, stamp) =
            file::inspect_bound(profile, candidate.path(), Some(candidate.identity()))?;
        if stamp.len > maximum_bytes {
            return Err("Codex snapshot exceeds its byte bound".into());
        }
        let reader = CodexTranscriptReader::open_bound(
            profile,
            candidate.path(),
            source,
            None,
            Some(candidate.identity()),
        )?;
        let snapshot = Self {
            reader,
            candidate,
            file,
            stamp,
            failed: false,
        };
        snapshot.check()?;
        Ok(snapshot)
    }
    pub fn directory(&self) -> &std::path::Path {
        self.reader.directory()
    }
    pub fn native_session_key(&self) -> &str {
        self.reader.native_session_key()
    }
    fn check(&self) -> CaptureResult<()> {
        if self.failed
            || !self.candidate.revalidate()?
            || !file::unchanged(&self.file, self.candidate.path(), &self.stamp)?
        {
            return Err("Codex snapshot changed or is unavailable".into());
        }
        Ok(())
    }
    pub fn poll(&mut self) -> CaptureResult<SnapshotPoll> {
        let result = self.step();
        if result.is_err() {
            self.failed = true;
        }
        result
    }
    fn step(&mut self) -> CaptureResult<SnapshotPoll> {
        self.check()?;
        let result = self.reader.poll()?;
        self.check()?;
        match result {
            TranscriptPoll::Ready(batch) => {
                // Transient reader state advances independently of durable ACK semantics.
                self.reader.advance_pending();
                Ok(SnapshotPoll::Ready(batch))
            }
            TranscriptPoll::ChangedDuringRead => Err("Codex snapshot changed during read".into()),
            TranscriptPoll::Verifying => Ok(SnapshotPoll::Reading),
            TranscriptPoll::Waiting => {
                let scanned = self
                    .reader
                    .scan
                    .as_ref()
                    .map_or(self.reader.offset, |scan| scan.offset);
                if scanned < self.stamp.len {
                    return Ok(SnapshotPoll::Reading);
                }
                if self.reader.offset != self.stamp.len {
                    return Err(
                        "Codex snapshot requires complete newline-terminated records".into(),
                    );
                }
                self.check()?;
                Ok(SnapshotPoll::Complete(SnapshotSummary {
                    complete_bytes: self.reader.offset,
                    records: self.reader.sequence,
                    sha256: self.reader.prefix,
                }))
            }
        }
    }
}
