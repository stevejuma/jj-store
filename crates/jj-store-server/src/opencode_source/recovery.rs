//! Reconstruct each message using its original generation and lifecycle state.
use super::*;
impl OpenCodeCaptureSource {
    pub(super) async fn recover_record(
        &mut self,
        record: &OpenCodeMessageSnapshot,
        position: &OpenCodeRecordPosition,
        context: &mut CaptureStepContext<'_>,
    ) -> Result<bool> {
        let generation = self
            .pass
            .target
            .as_ref()
            .ok_or(Error::Service("OpenCode recovery target unavailable"))?
            .cursor
            .generation;
        let queued = context
            .spool
            .pending()
            .map_err(|_| Error::Service("OpenCode recovery queue unavailable"))?;
        let entries: Vec<_> = queued
            .into_iter()
            .filter(|e| {
                e.cursor.integration == IntegrationId::OpenCode
                    && e.cursor.source_instance_id == self.source_instance_id()
                    && e.cursor.generation == generation
                    && matches!(e.cursor.position, SourcePositionV1::Snapshot {logical_records, ..}
                    if logical_records == position.logical_records)
                    && !self.proofs.contains_key(&e.batch.batch_id)
            })
            .collect();
        if entries.len() > 1 {
            return Err(Error::Service("OpenCode recovery position ambiguous"));
        }
        if let Some(entry) = entries.first() {
            let state = entry
                .reader_metadata
                .as_ref()
                .and_then(|m| m.opencode_snapshot.as_ref());
            let expected_digest = if let Some(state) = state {
                if state.message_records != position.logical_records
                    || state.message_prefix != position.prefix_digest
                {
                    return Ok(false);
                }
                state.digest(position.session_witness)
            } else {
                position.prefix_digest
            };
            if entry.cursor.position
                != (SourcePositionV1::Snapshot {
                    generation_digest: expected_digest,
                    logical_records: position.logical_records,
                })
            {
                return Ok(false);
            }
            let original = OpenCodeRecordPosition {
                generation: entry.cursor.generation,
                sequence: entry.batch.observations[0].source_sequence,
                evidence: OpenCodeEvidence::Api,
                ..*position
            };
            let (capture, proof) = self
                .convert_with_lifecycle(record, &original, state.map(|s| s.lifecycle), context)
                .await?;
            if capture.repository != entry.repository
                || capture.profile != entry.profile
                || capture.cursor != entry.cursor
                || capture.batch != entry.batch
                || Some(&capture.metadata) != entry.reader_metadata.as_ref()
                || entry.thread_binding.is_some()
            {
                return Err(Error::Service("OpenCode queued record differs"));
            }
            self.proofs.insert(
                entry.batch.batch_id,
                OpenCodePublicationProof::new(self, proof, &capture)?,
            );
        }
        Ok(true)
    }
}
