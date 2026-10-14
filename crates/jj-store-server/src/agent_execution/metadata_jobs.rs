//! Journal admission precedes inference; unknown jobs never replay model requests.
use super::metadata_sources::Capture;
use super::*;
use super::{Error, Result};
use jj_store_service::agent_execution::metadata as api;
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub(super) struct SavedJob {
    pub digest: String,
    pub command: api::Command,
    pub capture: Capture,
    pub config: Option<api::Config>,
    pub view: api::Job,
    pub publication: Option<jj_store_service::collab_mutations::Request>,
    #[serde(default)]
    pub order: u64,
}
impl Supervisor {
    pub(super) async fn metadata_view(
        &self,
        actor: PrincipalId,
        query: api::Query,
    ) -> Result<api::View> {
        self.actor(actor)?;
        let c = self.metadata_capture(query).await?;
        let job = self.read(|s| {
            Ok(s.metadata_jobs
                .values()
                .filter(|j| j.capture.repository == c.repository && j.capture.target == c.target)
                .max_by_key(|j| j.order)
                .map(|j| j.view.clone()))
        })?;
        let title = c
            .selected_value
            .title()
            .map(str::to_owned)
            .or_else(|| first_title(&c.transcript));
        let fresh = c
            .value
            .generated
            .as_ref()
            .is_some_and(|g| g.sources == c.sources);
        Ok(api::View {
            version: 1,
            repository: c.repository,
            target: c.target,
            thread: c.owner,
            revision: c.revision,
            value: c.selected_value,
            sources: c.sources,
            title,
            fresh,
            writable: c.writable,
            unavailable: (!c.writable)
                .then(|| "This repository profile does not publish conversation metadata.".into()),
            details: c.details,
            job,
        })
    }
    pub(super) async fn metadata_submit(
        &self,
        actor: PrincipalId,
        command: api::Command,
    ) -> Result<api::Job> {
        self.actor(actor)?;
        if self.0.stopping.load(std::sync::atomic::Ordering::Acquire) {
            return Err(Error::Unavailable);
        }
        if command.version != 1 {
            return Err(Error::Invalid);
        }
        let digest = journal::digest(&command)?;
        if let Some(job) = self.read(|s| Ok(s.metadata_jobs.get(&command.request_id).cloned()))? {
            self.authorized(job.capture.repository, &job.capture.profile, false)
                .await?;
            if job.digest != digest {
                return Err(Error::Conflict);
            }
            if job.view.state == "outcome_unknown" && job.publication.is_some() {
                return self.metadata_recover(job).await;
            }
            return Ok(job.view);
        }
        let capture = self.metadata_capture(command.query.clone()).await?;
        if !capture.writable {
            return Err(Error::Denied);
        }
        if let api::Action::Cancel { job } = command.action {
            return self
                .metadata_cancel(command.clone(), digest, capture, job)
                .await;
        }
        let config = self.read(|s| Ok(s.settings.metadata.clone()))?;
        if matches!(
            command.action,
            api::Action::Refresh | api::Action::RegenerateTitle
        ) && (config.is_none()
            || capture.sources.is_empty()
            || capture.transcript.trim().is_empty())
        {
            return Err(Error::Unavailable);
        }
        if let api::Action::Rename { title } = &command.action
            && !label(title, 1024)
        {
            return Err(Error::Invalid);
        }
        let _guard = self.0.mutation.lock().await;
        if let Some(existing) =
            self.read(|s| Ok(s.metadata_jobs.get(&command.request_id).cloned()))?
        {
            if existing.digest != digest {
                return Err(Error::Conflict);
            }
            return Ok(existing.view);
        }
        if capture.revision != command.expected {
            return Err(Error::Stale);
        }
        let view = api::Job {
            request_id: command.request_id,
            state: "prepared".into(),
            error: None,
            result: None,
            original_command: Some(Box::new(command.clone())),
        };
        let job = self.update(|s| {
            if s.metadata_jobs
                .values()
                .filter(|j| {
                    matches!(
                        j.view.state.as_str(),
                        "prepared" | "generating" | "publishing"
                    ) && matches!(
                        j.command.action,
                        api::Action::Refresh | api::Action::RegenerateTitle
                    )
                })
                .count()
                >= 8
                || s.metadata_jobs.values().any(|j| {
                    j.capture.target == capture.target
                        && j.capture.repository == capture.repository
                        && matches!(
                            j.view.state.as_str(),
                            "prepared" | "generating" | "publishing"
                        )
                        && matches!(
                            j.command.action,
                            api::Action::Refresh | api::Action::RegenerateTitle
                        )
                        && matches!(
                            command.action,
                            api::Action::Refresh | api::Action::RegenerateTitle
                        )
                })
            {
                return Err(Error::Busy);
            }
            if s.metadata_jobs.len() >= 4096 {
                return Err(Error::Limit);
            }
            let order = s.metadata_jobs.values().map(|j| j.order).max().unwrap_or(0) + 1;
            let job = SavedJob {
                digest,
                command: command.clone(),
                capture,
                config,
                view: view.clone(),
                publication: None,
                order,
            };
            s.metadata_jobs.insert(command.request_id, job.clone());
            Ok(job)
        })?;
        let this = self.clone();
        let id = command.request_id;
        let (start, ready) = tokio::sync::oneshot::channel();
        let task = tokio::spawn(async move {
            if ready.await.is_ok() {
                this.run_metadata_job(job).await;
            }
        });
        self.0
            .runtime_tasks
            .lock()
            .map_err(|_| Error::Unavailable)?
            .insert(format!("metadata/{id}"), task.abort_handle());
        let _ = start.send(());
        drop(_guard);
        Ok(view)
    }
    pub(super) async fn run_metadata_job(&self, mut job: SavedJob) {
        let id = job.command.request_id;
        let result: Result<()> = async {
            let mut value = job.capture.value.clone();
            match &job.command.action {
                api::Action::Rename { title } => value.custom_title = Some(title.clone()),
                api::Action::ResetTitle => value.custom_title = None,
                api::Action::CaptureTitles => {}
                api::Action::Refresh | api::Action::RegenerateTitle => {
                    let _inference = self.0.metadata_inference.lock().await;
                    self.metadata_phase(id, "generating", None)?;
                    let output = self
                        .metadata_infer(
                            job.config.as_ref().ok_or(Error::Unavailable)?,
                            &job.capture.transcript,
                        )
                        .await?;
                    let generated = jj_collab::GeneratedConversationMetadataV1 {
                        title: output.title,
                        summary: output.summary,
                        keywords: output.keywords,
                        goals: output.goals,
                        sources: job.capture.sources.clone(),
                        provider: job
                            .config
                            .as_ref()
                            .unwrap()
                            .options
                            .provider
                            .clone()
                            .unwrap_or("local".into()),
                        model: job.config.as_ref().unwrap().model.clone(),
                        prompt_version: 1,
                        generated_at_millis: now(),
                        truncated: job.capture.truncated,
                    };
                    generated.validate().map_err(|_| Error::Invalid)?;
                    self.update(|s| {
                        s.metadata_jobs
                            .get_mut(&id)
                            .ok_or(Error::Stale)?
                            .view
                            .result = Some(generated.clone());
                        Ok(())
                    })?;
                    if matches!(job.command.action, api::Action::RegenerateTitle) {
                        value.custom_title = None;
                    }
                    value.generated = Some(generated);
                }
                api::Action::Publish { job: previous } => {
                    let previous =
                        self.read(|s| s.metadata_jobs.get(previous).cloned().ok_or(Error::Stale))?;
                    if previous.capture.target != job.capture.target
                        || previous.capture.repository != job.capture.repository
                    {
                        return Err(Error::Denied);
                    }
                    if !matches!(previous.view.state.as_str(), "failed" | "outcome_unknown") {
                        return Err(Error::Conflict);
                    }
                    value.generated = Some(previous.view.result.ok_or(Error::Unavailable)?);
                }
                api::Action::Cancel { .. } => return Err(Error::Invalid),
            }
            let _publication = self.0.mutation.lock().await;
            if self.read(|s| {
                Ok(s.metadata_jobs.get(&id).is_none_or(|j| {
                    matches!(j.view.state.as_str(), "cancelled" | "outcome_unknown")
                }))
            })? {
                return Err(Error::Stale);
            }
            let retained = if let api::Action::Publish { job: previous } = &job.command.action {
                self.read(|s| {
                    Ok(s.metadata_jobs
                        .get(previous)
                        .filter(|j| j.view.state == "outcome_unknown")
                        .and_then(|j| j.publication.clone()))
                })?
            } else {
                None
            };
            let request = if let Some(retained) = retained {
                retained
            } else {
                self.metadata_request(&mut job.capture, id, value).await?
            };
            self.update(|s| {
                let j = s.metadata_jobs.get_mut(&id).ok_or(Error::Stale)?;
                j.publication = Some(request.clone());
                j.capture.owner = job.capture.owner;
                j.view.state = "publishing".into();
                Ok(())
            })?;
            drop(_publication);
            self.metadata_publish(&job.capture, &request).await?;
            self.metadata_phase(id, "completed", None)?;
            Ok(())
        }
        .await;
        if let Err(error) = result {
            let state = if matches!(error, Error::Unavailable) {
                "outcome_unknown"
            } else {
                "failed"
            };
            let message = if matches!(error, Error::Stale) {
                "metadata_changed"
            } else if matches!(error, Error::Invalid) {
                "invalid_metadata_result"
            } else {
                "metadata_generation_unavailable"
            };
            let _ = self.metadata_phase(id, state, Some(message.into()));
        }
        if let Ok(mut tasks) = self.0.runtime_tasks.lock() {
            tasks.remove(&format!("metadata/{id}"));
        }
    }
    fn metadata_phase(&self, id: RequestId, phase: &str, error: Option<String>) -> Result<()> {
        self.update(|s| {
            let j = s.metadata_jobs.get_mut(&id).ok_or(Error::Stale)?;
            if matches!(j.view.state.as_str(), "cancelled" | "outcome_unknown")
                && phase != "completed"
            {
                return Err(Error::Stale);
            }
            j.view.state = phase.into();
            j.view.error = error;
            if !matches!(phase, "prepared" | "generating" | "publishing") {
                j.capture.transcript.clear();
                j.config = None;
                if phase == "completed" {
                    j.capture.value = Default::default();
                    j.capture.selected_value = Default::default();
                    j.capture.details = Value::Null;
                    j.capture.sources.clear();
                    j.publication = None;
                }
            }
            Ok(())
        })?;
        self.execution_changed();
        Ok(())
    }
    pub(super) async fn metadata_automatic(&self, id: &str) -> Result<()> {
        let saved = self.saved(id)?;
        let Some(conversation) = saved.view.conversation_id else {
            return Ok(());
        };
        let eligible = self.read(|s| {
            Ok(s.conversations
                .get(&conversation)
                .is_some_and(|c| c.metadata_auto_eligible)
                && s.sessions
                    .values()
                    .filter(|v| v.view.conversation_id.as_ref() == Some(&conversation))
                    .all(|v| {
                        !matches!(v.view.state.as_str(), "running" | "opening" | "preparing")
                            && v.view.published_sequence == v.view.last_sequence
                    })
                && saved.events.iter().any(|e| {
                    e.committed
                        && e.payload["type"] == "record"
                        && e.payload["record"]["status"] == "completed"
                        && e.payload["assistantText"]
                            .as_str()
                            .is_some_and(|t| !t.trim().is_empty())
                }))
        })?;
        if !eligible {
            return Ok(());
        }
        let request = RequestId::new(publication::identity(&format!(
            "metadata-initial/{conversation}"
        )));
        if self.read(|s| Ok(s.metadata_jobs.contains_key(&request)))? {
            return Ok(());
        }
        if !self.read(|s| Ok(s.settings.metadata.is_some()))? {
            self.update(|s| {
                if let Some(c) = s.conversations.get_mut(&conversation) {
                    c.metadata_auto_eligible = false;
                }
                Ok(())
            })?;
            return Ok(());
        }
        let query = api::Query {
            session: id.into(),
            repository: None,
        };
        let c = self.metadata_capture(query.clone()).await?;
        let command = api::Command {
            version: 1,
            request_id: request,
            query,
            expected: c.revision,
            action: api::Action::Refresh,
        };
        let _ = self
            .metadata_submit(self.0.resources.actor, command)
            .await?;
        self.update(|s| {
            if let Some(c) = s.conversations.get_mut(&conversation) {
                c.metadata_auto_eligible = false;
            }
            Ok(())
        })?;
        Ok(())
    }
}
fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .min(i64::MAX as u128) as i64
}
fn first_title(text: &str) -> Option<String> {
    text.lines()
        .find_map(|line| {
            line.strip_prefix("user: ")
                .or_else(|| line.strip_prefix("human: "))
        })
        .map(|s| s.chars().take(80).collect::<String>())
        .filter(|s| !s.trim().is_empty())
}
