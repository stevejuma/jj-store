use super::*;

impl CodexCaptureSource {
    pub(super) async fn ingest_step(
        &mut self,
        context: &mut CaptureStepContext<'_>,
        retain_publication: bool,
    ) -> Result<CaptureSourceStep> {
        self.ingest_guarded(context, retain_publication, &|| Ok(()))
            .await
    }
    pub(super) async fn ingest_guarded(
        &mut self,
        context: &mut CaptureStepContext<'_>,
        retain_publication: bool,
        check: &(dyn Fn() -> Result<()> + Sync),
    ) -> Result<CaptureSourceStep> {
        check()?;
        self.check_spool(context.spool)?;
        let bounds = context
            .spool
            .source_reader_progress(
                self.profile.integration,
                self.reader.source_instance_id(),
                self.reader.qualified().generation_digest(),
            )
            .map_err(|error| Error::capture_spool(error, "capture source progress unavailable"))?;
        if self.needs_resume {
            let previous = bounds
                .progress
                .clone()
                .ok_or(Error::Service("capture recovered progress missing"))?;
            self.reader
                .resume(previous.clone())
                .map_err(|_| Error::Service("capture recovered reader unavailable"))?;
            self.reader_progress = Some(previous);
            self.needs_resume = false;
        }
        self.reader
            .refresh_source_progress(&bounds)
            .map_err(|_| Error::Service("capture source allocation unavailable"))?;
        let batch = match self
            .reader
            .poll()
            .map_err(|_| Error::Service("capture source unavailable"))?
        {
            TranscriptPoll::Waiting => return Ok(CaptureSourceStep::Waiting),
            TranscriptPoll::Verifying => return Ok(CaptureSourceStep::Verifying),
            TranscriptPoll::ChangedDuringRead => return Ok(CaptureSourceStep::ChangedDuringRead),
            TranscriptPoll::Ready(batch) => batch,
        };
        let (proof, body) = if batch.directory().is_some() {
            match context.admission.inspect_batch(&batch).await {
                Ok(proof) => (proof, true),
                Err(
                    crate::capture_admission::Error::Unbound
                    | crate::capture_admission::Error::Limit,
                ) => (self.anchor.clone(), false),
                Err(_) => return Ok(CaptureSourceStep::Withheld),
            }
        } else {
            (self.anchor.clone(), false)
        };
        context
            .authority
            .bind(context.host, context.admission, &proof)
            .await?;
        check()?;
        let repository = proof.workspace.repository;
        let capture = if body {
            if self
                .sanitizer
                .as_ref()
                .is_none_or(|(workspace, _)| workspace != &proof.workspace.path)
            {
                let sanitizer = TranscriptSanitizer::new(
                    std::path::Path::new(&proof.workspace.path),
                    &self.privacy.home,
                    &self.privacy.temporary,
                    &self.privacy.cache,
                )
                .map_err(|_| Error::Service("capture sanitizer unavailable"))?;
                self.sanitizer = Some((proof.workspace.path.clone(), sanitizer));
            }
            batch.sanitize(
                repository,
                &self.profile,
                &self.sanitizer.as_ref().expect("prepared sanitizer").1,
            )
        } else {
            batch.sanitize_omitted(repository, &self.profile)
        }
        .map_err(|_| Error::Service("capture conversion unavailable"))?;
        context
            .admission
            .revalidate(&proof)
            .await
            .map_err(|_| Error::Service("capture admission changed"))?;
        self.check_spool(context.spool)?;
        check()?;
        let progress = jj_store_capture::reader_progress::ReaderProgressV1 {
            schema_version: 1,
            repository,
            cursor: capture.cursor.clone(),
            metadata: capture.metadata.clone(),
            last_sequence: capture
                .batch
                .observations
                .last()
                .ok_or(Error::Service("capture observations unavailable"))?
                .source_sequence,
            last_batch: capture.batch.batch_id,
        };
        let ack = context
            .spool
            .enqueue_from_automatic_reader(capture)
            .map_err(|error| Error::capture_spool(error, "capture spool admission unavailable"))?;
        self.check_spool(context.spool)?;
        // No await splits durable enqueue, retained publication proof and exact ack.
        if retain_publication {
            self.pending = Some(PendingPublication {
                proof: proof.clone(),
                ack: ack.clone(),
            });
        }
        self.anchor = proof;
        self.reader
            .acknowledge(&ack)
            .map_err(|_| Error::Service("capture acknowledgement unavailable"))?;
        self.reader_progress = Some(progress);
        Ok(CaptureSourceStep::Enqueued)
    }
}
