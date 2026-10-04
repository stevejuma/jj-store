use super::files::PrivateLock;
use crate::{
    CaptureResult,
    agent_capture::DurableCaptureAck,
    codex_discovery::CodexSourceCandidate,
    codex_source::BoundCodexTranscriptReader,
    codex_transcript::{TranscriptPoll, file},
    profile::CaptureProfile,
    reader_progress::{ReaderProgressV1, ReaderSourceProgress},
};
use jj_agent::SourceInstanceId;
use std::path::Path;

/// Device-memory deduplication only; never a source identity or authority.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CodexSourceFingerprint([u8; 32]);

/// Header-only qualification. No source allocation, poll or admission side effect.
#[derive(Clone)]
pub struct QualifiedCodexSource {
    pub(crate) candidate: CodexSourceCandidate,
    pub(crate) header: file::Header,
    pub(crate) profile: CaptureProfile,
}
impl QualifiedCodexSource {
    pub fn generation_digest(&self) -> [u8; 32] {
        self.header.witness
    }
    /// Read-only exact queued-range proof; no identity, lease or cursor allocation.
    /// The caller must separately verify retained native eligibility and authorization.
    pub fn recover_batch(
        &self,
        source: SourceInstanceId,
        entry: &crate::agent_capture::PendingCaptureV1,
    ) -> CaptureResult<crate::codex_transcript::CodexBatchRecovery> {
        self.revalidate()?;
        let recovery = crate::codex_transcript::CodexBatchRecovery::open(
            self.candidate.clone(),
            self.profile,
            source,
            entry,
        )?;
        self.revalidate()?;
        Ok(recovery)
    }
    pub fn fingerprint(&self) -> CodexSourceFingerprint {
        CodexSourceFingerprint(super::digest(
            b"codex-memory-fingerprint/v1",
            &[
                &super::keys(self).1,
                &self.header.witness,
                self.profile.product_version().as_bytes(),
            ],
        ))
    }
    pub fn inspect(
        profile: CaptureProfile,
        candidate: CodexSourceCandidate,
    ) -> CaptureResult<Option<Self>> {
        if !candidate.revalidate()? {
            return Ok(None);
        }
        let (_, header, _) =
            file::inspect_bound(profile, candidate.path(), Some(candidate.identity()))?;
        if !candidate.revalidate()? {
            return Ok(None);
        }
        Ok(Some(Self {
            candidate,
            header,
            profile,
        }))
    }
    pub fn profile(&self) -> CaptureProfile {
        self.profile
    }
    pub fn directory(&self) -> &Path {
        &self.header.directory
    }
    pub fn native_session_key(&self) -> &str {
        &self.header.session
    }
    /// Header-only current-binding inspection; no identity, lease or progress allocation.
    /// A negative result grants no replacement eligibility or publication authority.
    pub fn is_current(&self) -> CaptureResult<bool> {
        if !self.candidate.current_binding()? {
            return Ok(false);
        }
        let (_, header, _) = file::inspect_bound(
            self.profile,
            self.candidate.path(),
            Some(self.candidate.identity()),
        )?;
        Ok(header.witness == self.header.witness && self.candidate.current_binding()?)
    }
    pub(crate) fn revalidate(&self) -> CaptureResult<()> {
        if !self.is_current()? {
            return Err("qualified Codex source unavailable".into());
        }
        Ok(())
    }
}
/// Retains exclusive physical and channel leases; not serializable or debug-renderable.
pub struct RegisteredCodexSource {
    pub(super) qualified: QualifiedCodexSource,
    pub(super) source: SourceInstanceId,
    pub(super) physical_lease: PrivateLock,
    pub(super) source_lease: PrivateLock,
}
impl RegisteredCodexSource {
    pub fn qualified(&self) -> &QualifiedCodexSource {
        &self.qualified
    }
    pub fn open_reader_at(
        self,
        bounds: &ReaderSourceProgress,
    ) -> CaptureResult<Option<LeasedCodexTranscriptReader>> {
        let Some(mut reader) = self.open_reader(bounds.progress.clone())? else {
            return Ok(None);
        };
        reader.refresh_source_progress(bounds)?;
        Ok(Some(reader))
    }
    pub fn source_instance_id(&self) -> SourceInstanceId {
        self.source
    }
    pub fn open_reader(
        self,
        previous: Option<ReaderProgressV1>,
    ) -> CaptureResult<Option<LeasedCodexTranscriptReader>> {
        self.physical_lease.revalidate()?;
        self.source_lease.revalidate()?;
        self.qualified.revalidate()?;
        let witness = self.qualified.header.witness;
        let qualified = self.qualified.clone();
        let Some(reader) = BoundCodexTranscriptReader::open(
            self.qualified.profile,
            self.qualified.candidate,
            self.source,
            previous,
        )?
        else {
            return Ok(None);
        };
        self.physical_lease.revalidate()?;
        self.source_lease.revalidate()?;
        if reader.header_witness() != witness {
            return Err("registered Codex header changed".into());
        }
        Ok(Some(LeasedCodexTranscriptReader {
            reader,
            qualified,
            source: self.source,
            physical_lease: self.physical_lease,
            source_lease: self.source_lease,
            witness,
        }))
    }
}
pub struct LeasedCodexTranscriptReader {
    pub(super) reader: BoundCodexTranscriptReader,
    pub(super) qualified: QualifiedCodexSource,
    pub(super) source: SourceInstanceId,
    pub(super) physical_lease: PrivateLock,
    pub(super) source_lease: PrivateLock,
    pub(super) witness: [u8; 32],
}
impl LeasedCodexTranscriptReader {
    pub fn qualified(&self) -> &QualifiedCodexSource {
        &self.qualified
    }
    pub fn refresh_source_progress(&mut self, bounds: &ReaderSourceProgress) -> CaptureResult<()> {
        if !self.revalidate()? {
            return Err("registered Codex source unavailable".into());
        }
        self.reader.refresh_source_progress(bounds)
    }
    pub fn compact_idle(&mut self) -> bool {
        self.reader.compact_idle()
    }
    /// Reopen at sanitized durable source progress without relinquishing either lease.
    pub fn resume(&mut self, previous: ReaderProgressV1) -> CaptureResult<()> {
        if !self.revalidate()? {
            return Err("registered Codex source unavailable".into());
        }
        let reader = self
            .reader
            .resume(previous)?
            .ok_or("registered Codex source unavailable")?;
        if reader.header_witness() != self.witness || !self.revalidate()? {
            return Err("registered Codex header changed".into());
        }
        self.reader = reader;
        Ok(())
    }
    pub fn recover_batch(
        &self,
        entry: &crate::agent_capture::PendingCaptureV1,
    ) -> CaptureResult<crate::codex_transcript::CodexBatchRecovery> {
        if !self.revalidate()? {
            return Err("registered Codex source unavailable".into());
        }
        let recovery = self.reader.recover_batch(entry)?;
        if !self.revalidate()? {
            return Err("registered Codex source unavailable".into());
        }
        Ok(recovery)
    }
    pub fn source_instance_id(&self) -> SourceInstanceId {
        self.source
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
    pub fn revalidate(&self) -> CaptureResult<bool> {
        self.physical_lease.revalidate()?;
        self.source_lease.revalidate()?;
        Ok(self.reader.header_witness() == self.witness && self.reader.revalidate()?)
    }
    pub fn poll(&mut self) -> CaptureResult<TranscriptPoll> {
        if !self.revalidate()? {
            return Err("registered Codex source unavailable".into());
        }
        let result = self.reader.poll()?;
        self.physical_lease.revalidate()?;
        self.source_lease.revalidate()?;
        if self.reader.header_witness() != self.witness {
            return Err("registered Codex header changed".into());
        }
        Ok(result)
    }
    /// Durable acknowledgement remains repairable even after source/state unavailability.
    pub fn acknowledge(&mut self, ack: &DurableCaptureAck) -> CaptureResult<()> {
        self.reader.acknowledge(ack)
    }
}
