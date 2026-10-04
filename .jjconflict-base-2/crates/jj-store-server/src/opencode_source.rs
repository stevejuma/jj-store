//! Admitted read-only API ingestion; scheduling and configured startup remain separate.
use crate::{
    Error, Result,
    capture_source::{CapturePrivacyRoots, CaptureSourceStep, CaptureStepContext},
    opencode_read_api::{OpenCodeMessageAcquisition, OpenCodeReadApi},
};
use jj_store_capture::opencode_ingress::{CaptureBatchId, IntegrationId, SourcePositionV1};
use jj_store_capture::{
    CaptureSpoolBinding,
    agent_capture::PendingCaptureV1,
    opencode_ingress::{OpenCodeEvidence, OpenCodeRecordPosition},
    opencode_snapshot::{OpenCodeMessageSnapshot, OpenCodeSessionSnapshot},
    reader_progress::{ReaderCapture, ReaderProgressV1, ReaderSourceProgress},
    source_registry::{LeasedOpenCodeSource, SourceInstanceId, SourceRegistry},
    transcript_sanitization::TranscriptSanitizer,
};
use jj_store_service::local_management::VerifiedCaptureWorkspace;
use std::{
    collections::{BTreeMap, VecDeque},
    sync::Arc,
};
mod conversion;
mod ingress;
mod lifecycle;
mod publication;
mod recovery;
mod snapshot;
pub(crate) use publication::OpenCodePublicationProof;
use snapshot::SnapshotPass;

// Native snapshots, directories, sanitizers and current proofs stay memory-only.
pub struct OpenCodeCaptureSource {
    api: OpenCodeReadApi,
    session: OpenCodeSessionSnapshot,
    lease: Arc<LeasedOpenCodeSource>,
    anchor: VerifiedCaptureWorkspace,
    spool_binding: CaptureSpoolBinding,
    spool_capacity: u64,
    privacy: CapturePrivacyRoots,
    sanitizer: Option<(String, TranscriptSanitizer)>,
    pass: SnapshotPass,
    proofs: BTreeMap<CaptureBatchId, OpenCodePublicationProof>,
    delivery: jj_store_capture::agent_publication::CaptureDrainScheduler,
    statuses_recovered: bool,
}
impl OpenCodeCaptureSource {
    pub async fn acquire(
        api: OpenCodeReadApi,
        sources: Arc<SourceRegistry>,
        native_session: &str,
        privacy: CapturePrivacyRoots,
        context: &mut CaptureStepContext<'_>,
    ) -> Result<Option<Self>> {
        let observed = api.observe(native_session).await?;
        let session = observed.session().clone();
        let binding = api.source_binding(&session)?;
        let admitted = sources
            .lookup_opencode_admitted(&binding)
            .map_err(|_| Error::Service("OpenCode admission history unavailable"))?;
        if !observed.activity().active() && admitted.is_none() {
            return Ok(None);
        }
        let directory = session
            .directory()
            .to_str()
            .ok_or(Error::Service("OpenCode directory unavailable"))?;
        let anchor = context
            .admission
            .inspect(directory)
            .await
            .map_err(|_| Error::Service("OpenCode directory admission unavailable"))?;
        context
            .authority
            .bind(context.host, context.admission, &anchor)
            .await?;
        let active = observed.activity();
        let lease = context
            .admission
            .source_io(move || {
                if active.active() {
                    sources.acquire_opencode_active(binding, active).map(Some)
                } else {
                    sources.acquire_opencode_admitted(binding)
                }
            })
            .await
            .map_err(|_| Error::Service("OpenCode acquisition unavailable"))?
            .map_err(|_| Error::Service("OpenCode source lease unavailable"))?
            .ok_or(Error::Service("OpenCode source admission changed"))?;
        if api.observe(native_session).await?.session() != &session {
            return Err(Error::Service("OpenCode session binding changed"));
        }
        context
            .admission
            .revalidate(&anchor)
            .await
            .map_err(|_| Error::Service("OpenCode admission changed"))?;
        let spool_binding = context
            .spool
            .binding()
            .map_err(|_| Error::Service("OpenCode spool binding unavailable"))?;
        let bounds = context
            .spool
            .source_reader_progress(
                IntegrationId::OpenCode,
                lease.source_instance_id(),
                lease.binding().generation_digest(),
            )
            .map_err(|error| Error::capture_spool(error, "OpenCode source progress unavailable"))?;
        let pass = SnapshotPass::new(&api, &session, &lease, bounds, false)?;
        Ok(Some(Self {
            api,
            session,
            lease: Arc::new(lease),
            anchor,
            spool_binding,
            spool_capacity: context.spool.pending_capacity(),
            privacy,
            sanitizer: None,
            pass,
            proofs: BTreeMap::new(),
            delivery: Default::default(),
            statuses_recovered: false,
        }))
    }
    pub(crate) fn publication_query(
        &self,
    ) -> jj_store_capture::reader_publication::SnapshotPublicationQuery {
        jj_store_capture::reader_publication::SnapshotPublicationQuery {
            integration: IntegrationId::OpenCode,
            source: self.source_instance_id(),
            generation_digest: self.lease.binding().generation_digest(),
        }
    }
    pub fn source_instance_id(&self) -> SourceInstanceId {
        self.lease.source_instance_id()
    }
    pub fn anchor(&self) -> &VerifiedCaptureWorkspace {
        &self.anchor
    }
    pub fn compact_idle(&self) -> bool {
        self.pass.finished && self.pass.records.is_empty() && self.pass.acquisition.is_none()
    }
    pub(crate) fn publication_proofs(
        &mut self,
        context: &CaptureStepContext<'_>,
    ) -> Result<Vec<OpenCodePublicationProof>> {
        self.check(context)?;
        let pending = context
            .spool
            .pending()
            .map_err(|_| Error::Service("OpenCode queue unavailable"))?;
        self.proofs
            .retain(|id, _| pending.iter().any(|entry| entry.batch.batch_id == *id));
        Ok(self.proofs.values().cloned().collect())
    }
    pub fn refresh(&mut self, context: &mut CaptureStepContext<'_>) -> Result<()> {
        self.check(context)?;
        if !self.pass.finished {
            return Err(Error::Service("OpenCode acquisition is not settled"));
        }
        self.restart(context, false)
    }
    fn restart(&mut self, context: &mut CaptureStepContext<'_>, changed: bool) -> Result<()> {
        let bounds = context
            .spool
            .source_reader_progress(
                IntegrationId::OpenCode,
                self.source_instance_id(),
                self.lease.binding().generation_digest(),
            )
            .map_err(|error| Error::capture_spool(error, "OpenCode source progress unavailable"))?;
        self.pass = SnapshotPass::new(&self.api, &self.session, &self.lease, bounds, changed)?;
        self.proofs.clear();
        self.statuses_recovered = false;
        Ok(())
    }
    fn check(&self, context: &CaptureStepContext<'_>) -> Result<()> {
        self.lease
            .revalidate()
            .map_err(|_| Error::Service("OpenCode source lease changed"))?;
        if context.spool.pending_capacity() != self.spool_capacity
            || context
                .spool
                .binding()
                .map_err(|_| Error::Service("OpenCode spool binding unavailable"))?
                != self.spool_binding
        {
            return Err(Error::Service("OpenCode spool binding changed"));
        }
        Ok(())
    }
    async fn current(&self, context: &CaptureStepContext<'_>) -> Result<()> {
        self.check(context)?;
        if self
            .api
            .observe(self.session.native_session_key())
            .await?
            .session()
            != &self.session
        {
            return Err(Error::Service("OpenCode session binding changed"));
        }
        self.check(context)
    }
}
