//! Cancellation has its own receipt and cannot retract publication.
use super::metadata_jobs::SavedJob;
use super::metadata_sources::Capture;
use super::*;
use jj_store_service::agent_execution::metadata as api;
impl Supervisor {
    pub(super) async fn metadata_cancel(
        &self,
        command: api::Command,
        digest: String,
        capture: Capture,
        id: RequestId,
    ) -> Result<api::Job> {
        let _guard = self.0.mutation.lock().await;
        if let Some(old) = self.read(|s| Ok(s.metadata_jobs.get(&command.request_id).cloned()))? {
            return if old.digest == digest {
                Ok(old.view)
            } else {
                Err(Error::Conflict)
            };
        }
        let view = self.update(|s| {
            if s.metadata_jobs.len() >= 4096 {
                return Err(Error::Limit);
            }
            let old = s.metadata_jobs.get_mut(&id).ok_or(Error::Stale)?;
            if old.capture.target != capture.target || old.capture.repository != capture.repository
            {
                return Err(Error::Denied);
            }
            let abandoned = old.view.state == "outcome_unknown" && old.publication.is_none();
            if !abandoned
                && !matches!(
                    old.view.state.as_str(),
                    "prepared" | "generating" | "cancelled"
                )
            {
                return Err(Error::Conflict);
            }
            old.view.state = "cancelled".into();
            old.view.error = Some(
                if abandoned {
                    "unknown_generation_abandoned_without_replay"
                } else {
                    "generation_cancelled_without_replay"
                }
                .into(),
            );
            old.capture.transcript.clear();
            old.config = None;
            let view = api::Job {
                request_id: command.request_id,
                state: "completed".into(),
                error: None,
                result: None,
                original_command: Some(Box::new(command.clone())),
            };
            let order = s.metadata_jobs.values().map(|j| j.order).max().unwrap_or(0) + 1;
            s.metadata_jobs.insert(
                command.request_id,
                SavedJob {
                    digest,
                    command,
                    capture,
                    config: None,
                    view: view.clone(),
                    publication: None,
                    order,
                },
            );
            Ok(view)
        })?;
        if let Some(task) = self
            .0
            .runtime_tasks
            .lock()
            .map_err(|_| Error::Unavailable)?
            .remove(&format!("metadata/{id}"))
        {
            task.abort();
        }
        self.execution_changed();
        Ok(view)
    }
    pub(super) fn metadata_stop(&self) -> Result<()> {
        self.update(|s| {
            for job in s.metadata_jobs.values_mut() {
                if matches!(
                    job.view.state.as_str(),
                    "prepared" | "generating" | "publishing"
                ) {
                    job.view.state = "outcome_unknown".into();
                    job.view.error = Some("interrupted_metadata_request_not_replayed".into());
                }
            }
            Ok(())
        })?;
        let mut tasks = self
            .0
            .runtime_tasks
            .lock()
            .map_err(|_| Error::Unavailable)?;
        let keys: Vec<_> = tasks
            .keys()
            .filter(|key| key.starts_with("metadata/"))
            .cloned()
            .collect();
        for key in keys {
            if let Some(task) = tasks.remove(&key) {
                task.abort();
            }
        }
        Ok(())
    }
}
