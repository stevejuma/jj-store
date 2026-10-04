//! Bounded prefix reconstruction of unpinned API pages; no cached path or cursor trust.
use super::*;
pub(super) struct SnapshotPass {
    pub acquisition: Option<OpenCodeMessageAcquisition>,
    pub records: VecDeque<Arc<OpenCodeMessageSnapshot>>,
    pub prefix: [u8; 32],
    pub logical_records: u64,
    pub generation: u64,
    pub sequence: u64,
    pub target: Option<ReaderProgressV1>,
    pub finished: bool,
    pub lifecycle: Option<jj_store_capture::opencode_checkpoint::OpenCodeLifecycleV1>,
}
impl SnapshotPass {
    pub fn new(
        api: &OpenCodeReadApi,
        session: &OpenCodeSessionSnapshot,
        lease: &LeasedOpenCodeSource,
        bounds: ReaderSourceProgress,
        changed: bool,
    ) -> Result<Self> {
        let witness = lease.binding().generation_digest();
        let lifecycle = bounds
            .progress
            .as_ref()
            .and_then(|p| p.metadata.opencode_snapshot.as_ref().map(|s| s.lifecycle));
        let target = if changed { None } else { bounds.progress };
        if let Some(progress) = &target
            && (progress.cursor.integration != IntegrationId::OpenCode
                || progress.cursor.source_instance_id != lease.source_instance_id()
                || progress.cursor.parser_version != "opencode-snapshot@1.18.30/v1"
                || progress.cursor.native_session_key.as_deref()
                    != Some(session.native_session_key())
                || progress.metadata.generation_digest != witness
                || progress.cursor.generation > bounds.generation_floor
                || progress.last_sequence > bounds.sequence_floor
                || !matches!(
                    progress.cursor.position,
                    SourcePositionV1::Snapshot {
                        logical_records: 1..=131072,
                        ..
                    }
                ) && !(progress.metadata.opencode_snapshot.is_some()
                    && matches!(
                        progress.cursor.position,
                        SourcePositionV1::Snapshot {
                            logical_records: 0,
                            ..
                        }
                    )))
        {
            return Err(Error::Service("OpenCode source checkpoint unavailable"));
        }
        let generation = match &target {
            Some(progress)
                if progress.cursor.generation == bounds.generation_floor
                    && progress.last_sequence == bounds.sequence_floor =>
            {
                progress.cursor.generation
            }
            _ => bounds
                .generation_floor
                .checked_add(1)
                .ok_or(Error::Service("OpenCode generation exhausted"))?,
        };
        Ok(Self {
            acquisition: Some(api.messages(session)),
            records: VecDeque::new(),
            prefix: jj_store_capture::opencode_checkpoint::initial_message_prefix(witness),
            logical_records: 0,
            generation,
            sequence: bounds.sequence_floor,
            target,
            finished: false,
            lifecycle,
        })
    }
    pub fn next_position(
        &self,
        record: &OpenCodeMessageSnapshot,
        lease: &LeasedOpenCodeSource,
    ) -> Result<OpenCodeRecordPosition> {
        let logical_records = self
            .logical_records
            .checked_add(1)
            .filter(|n| *n <= 131072)
            .ok_or(Error::Service("OpenCode record budget exhausted"))?;
        let prefix_digest =
            jj_store_capture::opencode_checkpoint::next_message_prefix(self.prefix, record)
                .map_err(|_| Error::Service("OpenCode native record unavailable"))?;
        Ok(OpenCodeRecordPosition {
            source: lease.source_instance_id(),
            generation: self.generation,
            session_witness: lease.binding().generation_digest(),
            sequence: self
                .sequence
                .checked_add(1)
                .ok_or(Error::Service("OpenCode source sequence exhausted"))?,
            logical_records,
            prefix_digest,
            evidence: OpenCodeEvidence::Api,
        })
    }
    pub fn advance(&mut self, position: &OpenCodeRecordPosition, acknowledged: bool) {
        self.prefix = position.prefix_digest;
        self.logical_records = position.logical_records;
        if acknowledged {
            self.sequence = position.sequence;
        }
        self.records.pop_front();
    }
}
