use super::*;
impl OpenCodeCaptureSource {
    /// One bounded acquisition/reconstruction/message step. Failed enqueue never advances.
    pub async fn ingest_step(
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
        if self.pass.finished {
            return Ok(CaptureSourceStep::Waiting);
        }
        if self.pass.records.is_empty() {
            match self
                .pass
                .acquisition
                .as_mut()
                .expect("unfinished acquisition")
                .next_page()
                .await?
            {
                Some(page) => self.pass.records = page.records.into_iter().map(Arc::new).collect(),
                None => {
                    if self.pass.target.is_some() {
                        self.restart(context, true)?;
                        return Ok(CaptureSourceStep::ChangedDuringRead);
                    }
                    self.pass.finished = true;
                    self.pass.acquisition = None;
                    self.sanitizer = None;
                    return Ok(CaptureSourceStep::Waiting);
                }
            }
        }
        let Some(record) = self.pass.records.front().cloned() else {
            return Ok(CaptureSourceStep::Verifying);
        };
        // Native API assistant bodies are mutable until the completion flush.
        // Stop at that prefix and reacquire later; never enqueue a partial body
        // whose durable recovery proof would disappear on the next update.
        let info = &record.envelope()["record"]["info"];
        if info["role"] == "assistant"
            && info["time"].is_object()
            && info["time"]["completed"].as_i64().is_none()
        {
            self.pass.finished = true;
            self.pass.records.clear();
            self.pass.acquisition = None;
            self.sanitizer = None;
            return Ok(CaptureSourceStep::Waiting);
        }
        let position = self.pass.next_position(&record, &self.lease)?;
        if let Some(target) = &self.pass.target {
            let SourcePositionV1::Snapshot {
                generation_digest,
                logical_records,
            } = target.cursor.position
            else {
                return Err(Error::Service("OpenCode source checkpoint unavailable"));
            };
            let expected_prefix = target
                .metadata
                .opencode_snapshot
                .as_ref()
                .map_or(generation_digest, |s| s.message_prefix);
            if position.logical_records >= logical_records
                && (position.logical_records != logical_records
                    || position.prefix_digest != expected_prefix)
            {
                self.restart(context, true)?;
                return Ok(CaptureSourceStep::ChangedDuringRead);
            }
            if !self.recover_record(&record, &position, context).await? {
                self.restart(context, true)?;
                return Ok(CaptureSourceStep::ChangedDuringRead);
            }
            self.pass.advance(&position, false);
            if position.logical_records == logical_records {
                self.pass.target = None;
            }
            return Ok(CaptureSourceStep::Verifying);
        }
        let (capture, proof) = self.convert(&record, &position, context).await?;
        let cursor = capture.cursor.clone();
        let batch_id = capture.batch.batch_id;
        let proof = OpenCodePublicationProof::new(self, proof, &capture)?;
        self.check(context)?;
        let ack = context
            .spool
            .enqueue_from_automatic_reader(capture)
            .map_err(|error| Error::capture_spool(error, "OpenCode durable enqueue unavailable"))?;
        // Nothing awaits between durable enqueue, exact acknowledgement and memory progress.
        self.proofs.insert(batch_id, proof);
        if ack.batch_id != batch_id || ack.position != cursor.position {
            return Err(Error::Service("OpenCode acknowledgement differs"));
        }
        self.pass.advance(&position, true);
        Ok(CaptureSourceStep::Enqueued)
    }
}
