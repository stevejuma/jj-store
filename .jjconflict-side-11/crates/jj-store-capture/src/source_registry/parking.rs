//! Device-memory frame handoff. Raw automatic allocations retain channel ownership.
use super::{LeasedCodexTranscriptReader, QualifiedCodexSource, RegisteredCodexSource};
use crate::{
    CaptureResult, codex_source::BoundCodexTranscriptReader, reader_progress::ReaderProgressV1,
};
use jj_agent::SourceInstanceId;
mod pinned;
pub use pinned::ReservedCodexPhysicalLease;

// Raw partial/pending bytes and paths intentionally have no Debug or encoding.
pub struct ParkedCodexTranscriptReader {
    reader: Option<BoundCodexTranscriptReader>,
    qualified: QualifiedCodexSource,
    source: SourceInstanceId,
    witness: [u8; 32],
    channel: Option<super::files::PrivateLock>,
}
impl LeasedCodexTranscriptReader {
    /// Consumes the leased owner; both locks are released on return.
    pub fn park(self) -> ParkedCodexTranscriptReader {
        self.park_with_channel(false)
    }
    /// Automatic frames reserve the channel only while a raw allocation is pending.
    /// The caller must bound noncompact frames independently of physical readers.
    pub fn park_automatic(self) -> ParkedCodexTranscriptReader {
        let reserve = self.reader.has_pending_batch();
        self.park_with_channel(reserve)
    }
    fn park_with_channel(self, reserve: bool) -> ParkedCodexTranscriptReader {
        ParkedCodexTranscriptReader {
            reader: Some(self.reader),
            qualified: self.qualified,
            source: self.source,
            witness: self.witness,
            channel: reserve.then_some(self.source_lease),
        }
    }
}
impl ParkedCodexTranscriptReader {
    /// Validate allocation before consuming the frame, retaining it on failure.
    pub fn resume_at(
        &mut self,
        registered: RegisteredCodexSource,
        bounds: &crate::reader_progress::ReaderSourceProgress,
    ) -> CaptureResult<LeasedCodexTranscriptReader> {
        self.validate_resume(&registered)?;
        self.reader
            .as_mut()
            .ok_or("parked Codex reader consumed")?
            .refresh_source_progress(bounds)?;
        self.resume(registered)
    }
    pub fn idle_unchanged(&self) -> CaptureResult<bool> {
        self.reader
            .as_ref()
            .map_or(Ok(false), |reader| reader.idle_unchanged())
    }
    pub fn compact_idle(&mut self) -> bool {
        self.reader
            .as_mut()
            .is_some_and(|reader| reader.compact_idle())
    }
    pub fn qualified(&self) -> &QualifiedCodexSource {
        &self.qualified
    }
    pub fn source_instance_id(&self) -> SourceInstanceId {
        self.source
    }
    /// Caller must independently recheck current admission and durable progress.
    /// Failure retains this frame; success transfers it exactly once.
    pub fn resume(
        &mut self,
        registered: RegisteredCodexSource,
    ) -> CaptureResult<LeasedCodexTranscriptReader> {
        self.validate_resume(&registered)?;
        Ok(LeasedCodexTranscriptReader {
            reader: self.reader.take().expect("checked parked reader"),
            qualified: self.qualified.clone(),
            source: self.source,
            witness: self.witness,
            physical_lease: registered.physical_lease,
            source_lease: registered.source_lease,
        })
    }
    fn validate_resume(&self, registered: &RegisteredCodexSource) -> CaptureResult<()> {
        if self.channel.is_some() {
            return Err("parked Codex reader requires reserved reacquisition".into());
        }
        registered.physical_lease.revalidate()?;
        registered.source_lease.revalidate()?;
        self.validate_source(&registered.qualified, registered.source)
    }
    fn validate_source(
        &self,
        qualified: &QualifiedCodexSource,
        source: SourceInstanceId,
    ) -> CaptureResult<()> {
        let reader = self.reader.as_ref().ok_or("parked Codex reader consumed")?;
        qualified.revalidate()?;
        self.qualified.revalidate()?;
        if source != self.source
            || qualified.profile != self.qualified.profile
            || qualified.header.witness != self.witness
            || qualified.candidate.identity() != self.qualified.candidate.identity()
            || reader.header_witness() != self.witness
            || !reader.revalidate()?
        {
            return Err("parked Codex reader qualification changed".into());
        }
        Ok(())
    }
}

/// A retained frame never supplies permission to rewind the durable source cursor.
pub fn validate_parked_progress(
    previous: Option<&ReaderProgressV1>,
    current: Option<&ReaderProgressV1>,
) -> CaptureResult<()> {
    if let Some(current) = current {
        current.validate()?;
    }
    let Some(previous) = previous else {
        return Ok(());
    };
    previous.validate()?;
    let current = current.ok_or("parked reader progress missing")?;
    if previous == current {
        return Ok(());
    }
    if current.cursor.generation == previous.cursor.generation {
        return current.validate_forward(previous);
    }
    if current.cursor.integration != previous.cursor.integration
        || current.cursor.source_instance_id != previous.cursor.source_instance_id
        || current.cursor.generation <= previous.cursor.generation
        || current.last_sequence <= previous.last_sequence
    {
        return Err("parked reader progress regressed".into());
    }
    Ok(())
}
