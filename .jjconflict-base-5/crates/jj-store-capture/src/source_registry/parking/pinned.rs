//! Fresh physical ownership without moving the retained channel through an await.
use super::super::{catalog::Catalog, files::PrivateLock};
use super::*;
use crate::{reader_progress::ReaderSourceProgress, source_registry::SourceRegistry};

/// Opaque physical-only acquisition. Failure/drop cannot release the parked channel.
pub struct ReservedCodexPhysicalLease {
    qualified: QualifiedCodexSource,
    source: SourceInstanceId,
    physical: PrivateLock,
}
impl ParkedCodexTranscriptReader {
    pub fn has_channel_reservation(&self) -> bool {
        self.channel.is_some()
    }
    /// Bounded synchronous metadata work, for the supervisor's owned blocking step.
    /// The channel remains in this frame across all subsequent admission awaits.
    pub fn acquire_reserved(
        &self,
        registry: &SourceRegistry,
    ) -> CaptureResult<Option<ReservedCodexPhysicalLease>> {
        self.acquire_reserved_checked(registry, super::super::keys(&self.qualified).2, || Ok(()))
    }
    /// Import-only frames require exact accepted-prefix reconstruction and retained
    /// durable consent/current policy/Write gates before physical reacquisition.
    pub fn acquire_reserved_import(
        &self,
        registry: &SourceRegistry,
        preparation: &crate::codex_import::CodexImportProbe,
        accepted: crate::codex_import::CodexImportBinding,
        mut consent: impl FnMut() -> CaptureResult<()>,
    ) -> CaptureResult<Option<ReservedCodexPhysicalLease>> {
        let mut proof = super::super::import::ImportAdmission {
            preparation,
            accepted,
            consent: &mut consent,
        };
        self.acquire_reserved_checked(registry, super::super::import_key(&self.qualified), || {
            proof.revalidate(&self.qualified)
        })
    }
    fn acquire_reserved_checked(
        &self,
        registry: &SourceRegistry,
        marker: [u8; 32],
        mut check: impl FnMut() -> CaptureResult<()>,
    ) -> CaptureResult<Option<ReservedCodexPhysicalLease>> {
        check()?;
        let channel = self.channel.as_ref().ok_or("parked channel unavailable")?;
        let _turn = registry.acquisitions.acquire()?;
        channel.validate_directory(&registry.directory)?;
        self.validate_source(&self.qualified, self.source)?;
        let gate = registry.directory.lock("registry.lock")?;
        registry.directory.recover()?;
        let catalog: Catalog = registry
            .directory
            .load()?
            .ok_or("source catalog unavailable")?;
        catalog.validate(registry.installation, registry.capacity)?;
        let (_, physical, _) = super::super::keys(&self.qualified);
        if super::super::admitted_source(&catalog, &self.qualified, marker)? != Some(self.source) {
            return Ok(None);
        }
        check()?;
        let physical = registry
            .directory
            .lock(&format!("physical-{}.lock", super::super::hex(&physical)))?;
        let acquired = ReservedCodexPhysicalLease {
            qualified: self.qualified.clone(),
            source: self.source,
            physical,
        };
        check()?;
        self.validate_reserved(&acquired)?;
        gate.revalidate()?;
        registry.directory.revalidate()?;
        Ok(Some(acquired))
    }
    /// Caller rechecks admission and durable bounds before this exact-once transfer.
    /// Failed refresh/reopen retains the original raw frame and channel lease.
    pub fn resume_reserved_at(
        &mut self,
        acquired: ReservedCodexPhysicalLease,
        bounds: &ReaderSourceProgress,
        reopen: bool,
    ) -> CaptureResult<Option<LeasedCodexTranscriptReader>> {
        self.validate_reserved(&acquired)?;
        let replacement = if reopen {
            let progress = bounds
                .progress
                .clone()
                .ok_or("reserved Codex reopen progress missing")?;
            let Some(mut reader) = BoundCodexTranscriptReader::open(
                self.qualified.profile,
                self.qualified.candidate.clone(),
                self.source,
                Some(progress),
            )?
            else {
                return Ok(None);
            };
            reader.refresh_source_progress(bounds)?;
            if reader.header_witness() != self.witness || !reader.revalidate()? {
                return Err("reserved Codex reader qualification changed".into());
            }
            Some(reader)
        } else {
            self.reader
                .as_mut()
                .ok_or("parked Codex reader consumed")?
                .refresh_source_progress(bounds)?;
            None
        };
        self.validate_reserved(&acquired)?;
        let previous = self.reader.take().expect("checked retained reader");
        Ok(Some(LeasedCodexTranscriptReader {
            reader: replacement.unwrap_or(previous),
            qualified: self.qualified.clone(),
            source: self.source,
            witness: self.witness,
            physical_lease: acquired.physical,
            source_lease: self.channel.take().expect("checked retained channel"),
        }))
    }
    fn validate_reserved(&self, acquired: &ReservedCodexPhysicalLease) -> CaptureResult<()> {
        let channel = self.channel.as_ref().ok_or("parked channel unavailable")?;
        channel.validate_peer(&acquired.physical)?;
        self.validate_source(&acquired.qualified, acquired.source)
    }
}
