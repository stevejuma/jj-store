//! Validation and local preparation for the shared sanitized capture journal.
use crate::CaptureResult as AuthorityResult;
use crate::agent_capture::{AgentCaptureSpool, PendingCaptureStateV1, PendingCaptureV1};
use crate::agent_capture_files::sync_dir;
use jj_agent::{CaptureBatchId, SourcePositionV1};
use jj_store_model::RequestId;
use jj_store_service::local_repository::{LocalAgentCommand, LocalAgentRequest};
use std::fs;

impl PendingCaptureV1 {
    pub(crate) fn validate(&self) -> AuthorityResult<()> {
        self.profile.validate()?;
        self.cursor.validate()?;
        self.batch.validate()?;
        if let Some(metadata) = &self.reader_metadata {
            metadata.validate()?;
        }
        let last_sequence = self
            .batch
            .observations
            .last()
            .ok_or("capture batch must contain observations")?
            .source_sequence;
        if self.schema_version != 1
            || self.profile.integration != self.batch.integration
            || self.cursor.integration != self.batch.integration
            || self.cursor.source_instance_id != self.batch.source_instance_id
            || matches!(
                self.cursor.position,
                SourcePositionV1::NativeSequence { sequence } if sequence != last_sequence
            )
        {
            return Err("capture spool identity mismatch".into());
        }
        if let PendingCaptureStateV1::Prepared { request } = &self.state {
            request.validate()?;
            if request.thread_binding.as_ref().map(|v| &v.binding) != self.thread_binding.as_ref() {
                return Err("prepared capture Thread binding mismatch".into());
            }
        }
        if let Some(binding) = &self.thread_binding {
            binding.validate()?;
        }
        if let PendingCaptureStateV1::LocalPrepared { request } = &self.state {
            request.validate()?;
            match &request.command {
                LocalAgentCommand::Capture { profile, batch }
                    if self.thread_binding.is_none()
                        && profile == &self.profile
                        && batch.as_ref() == &self.batch => {}
                LocalAgentCommand::BoundCapture {
                    profile,
                    batch,
                    binding,
                } if self.thread_binding.as_ref() == Some(binding)
                    && profile == &self.profile
                    && batch.as_ref() == &self.batch => {}
                _ => return Err("local capture prepared intent mismatch".into()),
            }
        }
        Ok(())
    }
}

impl AgentCaptureSpool {
    pub fn prepare_local(
        &mut self,
        batch: CaptureBatchId,
        request: LocalAgentRequest,
    ) -> AuthorityResult<()> {
        let _lock = self.mutation_lock()?;
        let path = self.path(batch);
        let mut entry = self.read(&path)?;
        match &entry.state {
            PendingCaptureStateV1::LocalPrepared { request: current } if current == &request => {
                return Ok(());
            }
            PendingCaptureStateV1::Captured => {}
            _ => return Err("local capture retry differs from durable intent".into()),
        }
        entry.state = PendingCaptureStateV1::LocalPrepared { request };
        entry.validate()?;
        self.persist(&path, &entry)
    }
    pub fn complete_local(
        &mut self,
        batch: CaptureBatchId,
        request_id: RequestId,
    ) -> AuthorityResult<()> {
        let _lock = self.mutation_lock()?;
        let path = self.path(batch);
        if path.exists() {
            let entry = self.read(&path)?;
            match &entry.state {
                PendingCaptureStateV1::LocalPrepared { request }
                    if request.request_id == request_id => {}
                _ => return Err("local capture completion identity mismatch".into()),
            }
            self.record_reader_progress(&entry)?;
            fs::remove_file(&path)?;
            sync_dir(path.parent().ok_or("capture spool parent missing")?)?;
        }
        Ok(())
    }
}
