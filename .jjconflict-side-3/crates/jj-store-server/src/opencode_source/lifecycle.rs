//! Explicit API class observations share one ordered source without advancing message count.
use super::*;
use jj_store_capture::opencode_checkpoint::{OpenCodeLifecycleV1, OpenCodeSnapshotStateV1};
impl OpenCodeCaptureSource {
    pub async fn lifecycle_step(
        &mut self,
        context: &mut CaptureStepContext<'_>,
    ) -> Result<CaptureSourceStep> {
        self.current(context).await?;
        self.recover_statuses(context).await?;
        if self.pass.target.as_ref().is_some_and(|p| {
            matches!(
                p.cursor.position,
                SourcePositionV1::Snapshot {
                    logical_records: 0,
                    ..
                }
            )
        }) {
            self.pass.target = None;
        }
        if self.pass.target.is_some() {
            return Ok(CaptureSourceStep::Verifying);
        }
        let observed = self.api.observe(self.session.native_session_key()).await?;
        if observed.session() != &self.session {
            return Err(Error::Service("OpenCode lifecycle binding changed"));
        }
        let Some(lifecycle) = OpenCodeLifecycleV1::from_activity(observed.activity()) else {
            return Err(Error::Service("OpenCode lifecycle class unavailable"));
        };
        if self.pass.lifecycle == Some(lifecycle) {
            return Ok(CaptureSourceStep::Waiting);
        }
        let state = OpenCodeSnapshotStateV1 {
            message_records: self.pass.logical_records,
            message_prefix: self.pass.prefix,
            lifecycle,
        };
        let sequence = self
            .pass
            .sequence
            .checked_add(1)
            .ok_or(Error::Service("OpenCode sequence exhausted"))?;
        let position = OpenCodeRecordPosition {
            source: self.source_instance_id(),
            generation: self.pass.generation,
            session_witness: self.lease.binding().generation_digest(),
            sequence,
            logical_records: state.message_records,
            prefix_digest: state.digest(self.lease.binding().generation_digest()),
            evidence: OpenCodeEvidence::Api,
        };
        let (capture, directory) = self.status_capture(&state, &position, context).await?;
        let cursor = capture.cursor.clone();
        let batch = capture.batch.batch_id;
        let proof = OpenCodePublicationProof::new(self, directory, &capture)?;
        self.check(context)?;
        let ack = context
            .spool
            .enqueue_from_automatic_reader(capture)
            .map_err(|e| Error::capture_spool(e, "OpenCode lifecycle enqueue unavailable"))?;
        // Durable evidence and composite checkpoint precede the exact acknowledgement.
        self.proofs.insert(batch, proof);
        if ack.batch_id != batch || ack.position != cursor.position {
            return Err(Error::Service("OpenCode lifecycle acknowledgement differs"));
        }
        self.pass.sequence = sequence;
        self.pass.lifecycle = Some(lifecycle);
        Ok(CaptureSourceStep::Enqueued)
    }
    async fn status_capture(
        &mut self,
        state: &OpenCodeSnapshotStateV1,
        position: &OpenCodeRecordPosition,
        context: &mut CaptureStepContext<'_>,
    ) -> Result<(ReaderCapture, VerifiedCaptureWorkspace)> {
        let record = self
            .session
            .status_record(state.lifecycle.activity())
            .map_err(|_| Error::Service("OpenCode status evidence unavailable"))?;
        context
            .authority
            .bind(context.host, context.admission, &self.anchor)
            .await?;
        let anchor = self.anchor.clone();
        let sanitizer = self.sanitizer_for(&anchor)?;
        let capture = record
            .sanitize(
                anchor.workspace.repository,
                position,
                std::path::Path::new(&anchor.directory),
                sanitizer,
            )
            .and_then(|c| c.with_opencode_state(state.clone()))
            .map_err(|_| Error::Service("OpenCode status conversion unavailable"))?;
        self.current(context).await?;
        context
            .admission
            .revalidate(&self.anchor)
            .await
            .map_err(|_| Error::Service("OpenCode status admission changed"))?;
        Ok((capture, self.anchor.clone()))
    }
    pub(super) async fn recover_statuses(
        &mut self,
        context: &mut CaptureStepContext<'_>,
    ) -> Result<()> {
        if self.statuses_recovered {
            return Ok(());
        }
        let entries = context
            .spool
            .pending()
            .map_err(|_| Error::Service("OpenCode status queue unavailable"))?;
        let source = self.source_instance_id();
        for entry in entries.into_iter().filter(|e| {
            e.cursor.integration == IntegrationId::OpenCode && e.cursor.source_instance_id == source
        }) {
            let Some(metadata) = &entry.reader_metadata else {
                continue;
            };
            let Some(state) = &metadata.opencode_snapshot else {
                continue;
            };
            if metadata.generation_digest != self.lease.binding().generation_digest()
                || self.proofs.contains_key(&entry.batch.batch_id)
            {
                continue;
            }
            let payload = entry
                .batch
                .observations
                .first()
                .ok_or(Error::Service("OpenCode status observation unavailable"))?
                .payload
                .bytes();
            let value: serde_json::Value = serde_json::from_slice(payload)
                .map_err(|_| Error::Service("OpenCode status evidence unavailable"))?;
            if value["record"]["type"].as_str() != Some("session_status") {
                continue;
            }
            let position = OpenCodeRecordPosition {
                source: self.source_instance_id(),
                generation: entry.cursor.generation,
                session_witness: metadata.generation_digest,
                sequence: entry.batch.observations[0].source_sequence,
                logical_records: state.message_records,
                prefix_digest: state.digest(metadata.generation_digest),
                evidence: OpenCodeEvidence::Api,
            };
            let (capture, directory) = self.status_capture(state, &position, context).await?;
            if capture.repository != entry.repository
                || capture.profile != entry.profile
                || capture.cursor != entry.cursor
                || capture.batch != entry.batch
                || Some(&capture.metadata) != entry.reader_metadata.as_ref()
                || entry.thread_binding.is_some()
            {
                return Err(Error::Service("OpenCode retained status differs"));
            }
            self.proofs.insert(
                entry.batch.batch_id,
                OpenCodePublicationProof::new(self, directory, &capture)?,
            );
        }
        self.statuses_recovered = true;
        Ok(())
    }
}
