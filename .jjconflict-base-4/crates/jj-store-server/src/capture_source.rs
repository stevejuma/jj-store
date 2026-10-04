//! Explicit admitted-source data flow; automatic scheduling/startup remain separate.
use crate::{
    Error, Result, capture_admission::CaptureAdmission, capture_authority::CaptureAuthority,
};
use jj_store_capture::{
    CaptureSpoolBinding,
    agent_capture::{AgentCaptureSpool, DurableCaptureAck, PendingCaptureV1},
    agent_publication::{CaptureDrainPartition, CaptureDrainReport, drain_capture_spool_entry},
    codex_transcript::TranscriptPoll,
    source_registry::LeasedCodexTranscriptReader,
    transcript_sanitization::TranscriptSanitizer,
};
use jj_store_service::{
    administration::RepositoryHost, local_management::VerifiedCaptureWorkspace,
    transport::NormalizerProfileV1,
};
use std::path::PathBuf;
mod acquire;
mod import;
mod ingress;
mod parking;
mod pipeline;
mod recovery;
mod resume;
pub use import::{
    CaptureImportStep, CodexCaptureImport, ImportedCodexCapturePipeline,
    ParkedImportedCodexCapturePipeline,
};
pub use parking::{CaptureParkedResumeStep, ParkedCodexCapturePipeline};
pub use pipeline::CodexCapturePipeline;
pub use resume::{CaptureResumeStep, CodexCaptureResume};

// Device paths and raw reader state intentionally have no Debug or serialization.
#[derive(Clone)]
pub struct CapturePrivacyRoots {
    pub home: PathBuf,
    pub temporary: PathBuf,
    pub cache: PathBuf,
}
pub struct CaptureStepContext<'a> {
    pub authority: &'a CaptureAuthority,
    pub host: &'a RepositoryHost,
    pub admission: &'a CaptureAdmission,
    pub spool: &'a mut AgentCaptureSpool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaptureSourceStep {
    Waiting,
    Verifying,
    ChangedDuringRead,
    Withheld,
    Enqueued,
    Published(CaptureDrainReport),
}
pub struct CodexCaptureSource<R = LeasedCodexTranscriptReader> {
    spool_binding: CaptureSpoolBinding,
    reader: R,
    reader_progress: Option<jj_store_capture::reader_progress::ReaderProgressV1>,
    profile: NormalizerProfileV1,
    anchor: VerifiedCaptureWorkspace,
    pending: Option<PendingPublication>,
    recovery: Option<recovery::PendingRecovery>,
    needs_resume: bool,
    privacy: CapturePrivacyRoots,
    sanitizer: Option<(String, TranscriptSanitizer)>,
}
struct PendingPublication {
    proof: VerifiedCaptureWorkspace,
    ack: DurableCaptureAck,
}
impl CodexCaptureSource {
    pub fn partition(&self) -> CaptureDrainPartition {
        CaptureDrainPartition {
            repository: self.anchor.workspace.repository,
            integration: self.profile.integration,
            source: self.reader.source_instance_id(),
        }
    }
    /// The supervisor must own and schedule synchronous bounded I/O independently.
    /// A caller may cancel awaits; durable intent and the retained proof survive.
    pub async fn step(
        &mut self,
        context: &mut CaptureStepContext<'_>,
    ) -> Result<CaptureSourceStep> {
        self.check_spool(context.spool)?;
        if let Some(pending) = &self.pending {
            let proof = &pending.proof;
            let partition = CaptureDrainPartition {
                repository: proof.workspace.repository,
                ..self.partition()
            };
            let queue = context
                .spool
                .pending()
                .map_err(|_| Error::Service("capture pending source unavailable"))?;
            let selected: Vec<_> = queue
                .iter()
                .filter(|entry| {
                    entry.repository == partition.repository
                        && entry.batch.integration == partition.integration
                        && entry.batch.source_instance_id == partition.source
                })
                .collect();
            if !selected
                .iter()
                .any(|entry| entry.batch.batch_id == pending.ack.batch_id)
            {
                self.pending = None;
                self.needs_resume = true;
                return Ok(CaptureSourceStep::Published(CaptureDrainReport {
                    committed: 0,
                    remaining: queue.len(),
                    outcome_unknown: false,
                }));
            }
            let Some(entry) = selected
                .iter()
                .find(|entry| entry.batch.batch_id == pending.ack.batch_id)
            else {
                return Err(Error::Service(
                    "capture pending directory recovery required",
                ));
            };
            let publisher = context
                .authority
                .publisher(context.host, context.admission, proof);
            let report = drain_capture_spool_entry(context.spool, entry, &publisher)
                .await
                .map_err(|_| Error::Service("capture publication unavailable"))?;
            if !report.outcome_unknown {
                self.pending = None;
            }
            return Ok(CaptureSourceStep::Published(report));
        }
        if let Some(step) = self.recover_step(context).await? {
            return Ok(step);
        }
        self.ingest_step(context, true).await
    }
    fn check_spool(&self, spool: &AgentCaptureSpool) -> Result<()> {
        if spool
            .binding()
            .map_err(|_| Error::Service("capture spool binding unavailable"))?
            != self.spool_binding
        {
            return Err(Error::Service("capture spool binding changed"));
        }
        Ok(())
    }
}
