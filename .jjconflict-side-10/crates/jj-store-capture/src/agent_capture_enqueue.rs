use crate::{
    CaptureResult as AuthorityResult,
    agent_capture::{
        AgentCaptureSpool, DurableCaptureAck, PendingCaptureStateV1, PendingCaptureV1, ack,
        same_capture,
    },
};
use jj_agent::{CaptureCursorV1, NormalizerProfileV1, SanitizedObservationBatchV1};
use jj_store_model::RepositoryId;
impl AgentCaptureSpool {
    pub fn enqueue(
        &mut self,
        repository: RepositoryId,
        profile: NormalizerProfileV1,
        cursor: CaptureCursorV1,
        batch: SanitizedObservationBatchV1,
    ) -> AuthorityResult<DurableCaptureAck> {
        self.enqueue_with_binding(repository, profile, cursor, batch, None)
    }
    pub fn enqueue_with_binding(
        &mut self,
        repository: RepositoryId,
        profile: NormalizerProfileV1,
        cursor: CaptureCursorV1,
        batch: SanitizedObservationBatchV1,
        thread_binding: Option<jj_collab::ThreadBindingV1>,
    ) -> AuthorityResult<DurableCaptureAck> {
        self.enqueue_candidate(PendingCaptureV1 {
            schema_version: 1,
            repository,
            profile,
            cursor,
            batch,
            state: PendingCaptureStateV1::Captured,
            thread_binding,
            reader_metadata: None,
        })
    }
    pub(crate) fn enqueue_candidate(
        &mut self,
        candidate: PendingCaptureV1,
    ) -> AuthorityResult<DurableCaptureAck> {
        self.enqueue_candidate_checked(candidate, false)
    }
    pub(crate) fn enqueue_candidate_checked(
        &mut self,
        candidate: PendingCaptureV1,
        monotonic: bool,
    ) -> AuthorityResult<DurableCaptureAck> {
        let _lock = self.mutation_lock()?;
        candidate.validate()?;
        let path = self.path(candidate.batch.batch_id);
        if path.exists() {
            let existing = self.read(&path)?;
            if !same_capture(&existing, &candidate) {
                return Err("capture retry differs from durable intent".into());
            }
            if existing.reader_metadata.is_some() {
                self.register_reader_publication(&existing, monotonic)?;
            }
            self.record_reader_progress(&existing)?;
            return Ok(ack(&existing));
        }
        if monotonic {
            self.validate_monotonic_reader(&candidate)?;
        }
        self.validate_reader_admission(&candidate)?;
        if candidate.reader_metadata.is_some() {
            self.check_capture_capacity(&path, &candidate)?;
            self.register_reader_publication(&candidate, monotonic)?;
        }
        self.persist(&path, &candidate)?;
        self.record_reader_progress(&candidate)?;
        Ok(ack(&candidate))
    }
}
