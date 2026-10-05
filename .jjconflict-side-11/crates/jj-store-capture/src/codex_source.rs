//! Read-only composition of approved discovery and version-qualified file reading.
use crate::{
    CaptureResult,
    agent_capture::DurableCaptureAck,
    codex_discovery::CodexSourceCandidate,
    codex_transcript::{CodexTranscriptReader, TranscriptPoll},
    profile::CaptureProfile,
    reader_progress::ReaderProgressV1,
};
use jj_agent::SourceInstanceId;
use std::path::Path;

/// The candidate is the only path constructor; this type has no persistence encoding.
/// It grants no source lease, repository binding, activity or publication authority.
pub struct BoundCodexTranscriptReader {
    candidate: CodexSourceCandidate,
    reader: CodexTranscriptReader,
}
impl BoundCodexTranscriptReader {
    pub(crate) fn has_pending_batch(&self) -> bool {
        self.reader.has_pending_batch()
    }
    pub(crate) fn refresh_source_progress(
        &mut self,
        bounds: &crate::reader_progress::ReaderSourceProgress,
    ) -> CaptureResult<()> {
        self.reader.refresh_source_progress(bounds)
    }
    pub(crate) fn idle_unchanged(&self) -> CaptureResult<bool> {
        if !self.candidate.revalidate()? {
            return Ok(false);
        }
        Ok(self.reader.idle_unchanged()? && self.candidate.revalidate()?)
    }
    pub(crate) fn compact_idle(&mut self) -> bool {
        self.reader.compact_idle()
    }
    pub fn open(
        profile: CaptureProfile,
        candidate: CodexSourceCandidate,
        source: SourceInstanceId,
        previous: Option<ReaderProgressV1>,
    ) -> CaptureResult<Option<Self>> {
        if !candidate.revalidate()? {
            return Ok(None);
        }
        let reader = CodexTranscriptReader::open_bound(
            profile,
            candidate.path(),
            source,
            previous,
            Some(candidate.identity()),
        )?;
        if !candidate.revalidate()? {
            return Ok(None);
        }
        Ok(Some(Self { candidate, reader }))
    }
    pub fn revalidate(&self) -> CaptureResult<bool> {
        self.candidate.revalidate()
    }
    pub fn directory(&self) -> &Path {
        self.reader.directory()
    }
    pub fn native_session_key(&self) -> &str {
        self.reader.native_session_key()
    }
    pub fn generation(&self) -> u64 {
        self.reader.generation()
    }
    pub(crate) fn header_witness(&self) -> [u8; 32] {
        self.reader.header_witness()
    }
    pub(crate) fn recover_batch(
        &self,
        entry: &crate::agent_capture::PendingCaptureV1,
    ) -> CaptureResult<crate::codex_transcript::CodexBatchRecovery> {
        crate::codex_transcript::CodexBatchRecovery::open(
            self.candidate.clone(),
            self.reader.profile,
            self.reader.source,
            entry,
        )
    }
    pub(crate) fn resume(&self, previous: ReaderProgressV1) -> CaptureResult<Option<Self>> {
        Self::open(
            self.reader.profile,
            self.candidate.clone(),
            self.reader.source,
            Some(previous),
        )
    }
    pub fn poll(&mut self) -> CaptureResult<TranscriptPoll> {
        if !self.revalidate()? {
            return Err("approved Codex source unavailable".into());
        }
        let result = self.reader.poll()?;
        if !self.revalidate()? {
            return Err("approved Codex source unavailable".into());
        }
        Ok(result)
    }
    /// Exact durable progress may be acknowledged even when later discovery is unavailable.
    pub fn acknowledge(&mut self, ack: &DurableCaptureAck) -> CaptureResult<()> {
        self.reader.acknowledge(ack)
    }
}
