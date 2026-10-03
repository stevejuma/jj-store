use super::*;
use crate::{
    capture_source::{CaptureSourceStep, CaptureStepContext},
    opencode_source::{OpenCodeCaptureSource, OpenCodePublicationProof},
};
use std::time::Instant;
pub(super) struct ApiSourceWork {
    api: OpenCodeReadApi,
    native_session: String,
    resources: CodexSupervisorResources,
    source: Option<OpenCodeCaptureSource>,
    snapshot: SourceSnapshot,
    interval: Duration,
    next: Instant,
    publication_query: Option<jj_store_capture::reader_publication::SnapshotPublicationQuery>,
}
pub(super) struct ApiSourceOutput {
    pub snapshot: SourceSnapshot,
    pub proofs: Vec<OpenCodePublicationProof>,
    pub enqueued: bool,
    pub unavailable: bool,
    pub publication_query: Option<jj_store_capture::reader_publication::SnapshotPublicationQuery>,
}
impl ApiSourceWork {
    pub fn new(
        api: OpenCodeReadApi,
        native_session: String,
        resources: CodexSupervisorResources,
        interval: Duration,
    ) -> Self {
        Self {
            api,
            native_session,
            resources,
            source: None,
            snapshot: SourceSnapshot {
                awaiting_activity: true,
                needs_frame: true,
                ..Default::default()
            },
            interval,
            next: Instant::now(),
            publication_query: None,
        }
    }
    pub fn rescan(&mut self) {
        self.next = Instant::now();
    }
    async fn advance(&mut self, raw: bool) -> Result<bool> {
        let compact = self.source.as_ref().is_none_or(|s| s.compact_idle());
        if compact && Instant::now() < self.next {
            return Ok(false);
        }
        if !raw {
            self.snapshot.needs_frame = true;
            return Ok(false);
        }
        self.snapshot.caught_up = false;
        self.snapshot.needs_frame = false;
        let r = &mut self.resources;
        let mut context = CaptureStepContext {
            authority: &r.authority,
            host: &r.host,
            admission: &r.admission,
            spool: &mut r.spool,
        };
        if let Some(source) = &mut self.source {
            if compact {
                source.refresh(&mut context)?;
            }
        } else {
            self.source = OpenCodeCaptureSource::acquire(
                self.api.clone(),
                r.sources.clone(),
                &self.native_session,
                r.privacy.clone(),
                &mut context,
            )
            .await?;
            self.snapshot.awaiting_activity = self.source.is_none();
            if self.source.is_none() {
                self.next = Instant::now() + self.interval;
                return Ok(false);
            }
            self.snapshot.observed = true;
        }
        let source = self.source.as_mut().expect("admitted source");
        let lifecycle = source.lifecycle_step(&mut context).await?;
        let step = if lifecycle == CaptureSourceStep::Enqueued {
            lifecycle
        } else {
            source.ingest_step(&mut context).await?
        };
        self.snapshot.anchor = Some(source.anchor().clone());
        self.snapshot.caught_up = step == CaptureSourceStep::Waiting && source.compact_idle();
        if self.snapshot.caught_up {
            self.next = Instant::now() + self.interval;
        }
        Ok(step == CaptureSourceStep::Enqueued)
    }
    fn settle(&mut self) {
        self.snapshot.compact = self.source.as_ref().is_none_or(|s| s.compact_idle());
        self.snapshot.caught_up &= self.snapshot.compact;
    }
}
#[async_trait::async_trait]
impl OwnedWork for ApiSourceWork {
    type Command = bool;
    type Output = ApiSourceOutput;
    async fn step(&mut self, raw: bool) -> Self::Output {
        let mut result = self.advance(raw).await;
        let proofs = if let Some(source) = &mut self.source {
            let r = &mut self.resources;
            match source.publication_proofs(&CaptureStepContext {
                authority: &r.authority,
                host: &r.host,
                admission: &r.admission,
                spool: &mut r.spool,
            }) {
                Ok(proofs) => {
                    self.publication_query = Some(source.publication_query());
                    proofs
                }
                Err(error) => {
                    result = Err(error);
                    Vec::new()
                }
            }
        } else {
            Vec::new()
        };
        self.snapshot.unavailable = result.is_err();
        self.snapshot.capacity_refusal = result.as_ref().err().and_then(Error::capacity_refusal);
        if result.is_err() && self.snapshot.capacity_refusal.is_none() {
            // Retained intents remain in the spool; a later acquisition must reconstruct
            // their exact witness/body proofs rather than borrow this failed source.
            self.source = None;
            self.next = Instant::now() + self.interval;
        }
        self.settle();
        ApiSourceOutput {
            snapshot: self.snapshot.clone(),
            proofs,
            enqueued: result.as_ref().is_ok_and(|v| *v),
            unavailable: result.is_err(),
            publication_query: self.publication_query,
        }
    }
    fn settled(&mut self) {
        self.settle();
    }
}
