//! Installation-owned compute process and durable settings/operation receipts.
mod hardware;
mod process;
use super::*;
use jj_store_service::agent_execution::compute as dto;
#[derive(Default)]
pub(super) struct Runtime {
    pub sender: Option<mpsc::Sender<Value>>,
    pub stop: Option<tokio::sync::watch::Sender<bool>>,
    pub state: String,
    pub connection: Option<lapis_ai_runtime::ComputeConnection>,
    pub invitation: Option<Value>,
    pub identity: Option<String>,
    pub progress: Option<Value>,
    pub error: Option<String>,
    pub operation: Option<RequestId>,
    pub task: Option<tokio::task::JoinHandle<()>>,
    pub relay_urls: Vec<String>,
}
impl Supervisor {
    pub(crate) async fn reload_compute_settings(
        &self,
    ) -> Result<(jj_store_service::runtime_reload::State, Option<String>)> {
        let saved = self.read(|s| Ok(s.compute.clone()))?;
        let runtime = self.0.compute.lock().await;
        let pending = runtime.sender.as_ref().is_some_and(|s| !s.is_closed())
            && runtime.relay_urls != saved.relay_urls;
        Ok(if pending {
            (jj_store_service::runtime_reload::State::Deferred,Some("Compute is retained. Saved transport relays apply after Stop compute and its next start.".into()))
        } else {
            (jj_store_service::runtime_reload::State::Current, None)
        })
    }
    pub(super) fn companion(&self, name: &str, environment: &str) -> Result<PathBuf> {
        let selected = std::env::var_os(environment)
            .map(PathBuf::from)
            .or_else(|| {
                self.0.ai_bundle.as_ref().and_then(|bundle| match name {
                    "lapis-agent" => bundle.runtime.agent.clone(),
                    "lapis-agent-tools" => bundle.runtime.tools.clone(),
                    "lapis-compute" => bundle.runtime.compute.clone(),
                    _ => None,
                })
            })
            .unwrap_or_else(|| self.0.resources.executable.with_file_name(name));
        let path = selected.canonicalize().map_err(|_| Error::Unavailable)?;
        if !path.is_file() {
            return Err(Error::Unavailable);
        }
        Ok(path)
    }
    pub(super) async fn compute_status_view(&self) -> Result<dto::Status> {
        let (settings, installed, prepared) = self.read(|s| {
            Ok((
                s.compute.clone(),
                s.compute_models.clone(),
                s.compute_runtime_prepared,
            ))
        })?;
        let runtime = self.0.compute.lock().await;
        let mut ids = vec![
            ("unsloth/Qwen3-0.6B-GGUF:Q4_K_M", "Qwen3 0.6B"),
            ("unsloth/Qwen3.5-4B-GGUF:Q4_K_M", "Qwen3.5 4B"),
        ];
        if let Some(model) = &settings.model
            && !ids.iter().any(|(id, _)| *id == model)
        {
            ids.push((model, model));
        }
        for model in &installed {
            if !ids.iter().any(|(id, _)| *id == model) {
                ids.push((model, model));
            }
        }
        Ok(dto::Status {
            hardware: hardware::facts(),
            model_file_picker: false, // The HTTP host owns OS picker composition.
            supported: self
                .companion("lapis-compute", "JJ_STORE_COMPUTE_EXECUTABLE")
                .is_ok(),
            state: if runtime.state.is_empty() {
                "off".into()
            } else {
                runtime.state.clone()
            },
            models: ids
                .into_iter()
                .map(|(id, name)| dto::Model {
                    id: id.into(),
                    name: name.into(),
                    installed: installed.contains(id),
                })
                .collect(),
            settings,
            runtime_prepared: prepared,
            invitation: runtime.invitation.clone(),
            identity: runtime.identity.clone(),
            progress: runtime.progress.clone(),
            error: runtime.error.clone(),
            operation: runtime.operation.and_then(|request| {
                self.read(|s| Ok(s.intents.get(&request).map(|i| i.outcome.clone())))
                    .ok()
                    .flatten()
            }),
        })
    }
    pub(super) async fn compute_control(
        &self,
        actor: PrincipalId,
        command: dto::Command,
    ) -> Result<Outcome> {
        self.actor(actor)?;
        if command.version != 1 || self.0.stopping.load(std::sync::atomic::Ordering::Acquire) {
            return Err(Error::Invalid);
        }
        if let dto::Action::Import { path } = &command.action
            && (!Path::new(path).is_absolute() || !label(path, 4096))
        {
            return Err(Error::Invalid);
        }
        if let dto::Action::Remove { model } = &command.action
            && !self.read(|s| Ok(s.compute_models.contains(model)))?
        {
            return Err(Error::Invalid);
        }
        let digest = journal::digest(&command)?;
        let _guard = self.0.mutation.lock().await;
        if let Some(outcome) = self.duplicate(command.request_id, &digest)? {
            return Ok(outcome);
        }
        let mut settings = self.read(|s| Ok(s.compute.clone()))?;
        if command.expected_revision != settings.revision {
            return Err(Error::Stale);
        }
        match &command.action {
            dto::Action::Configure { settings: next } => settings = next.clone(),
            dto::Action::Pair { name, invitation } => {
                let id = invitation["id"]
                    .as_str()
                    .filter(|id| label(id, 128))
                    .ok_or(Error::Invalid)?;
                if !label(name, 128) || invitation.to_string().len() > 16384 {
                    return Err(Error::Invalid);
                }
                settings.peers.insert(
                    id.into(),
                    dto::Peer {
                        name: name.clone(),
                        invitation: invitation.clone(),
                    },
                );
            }
            dto::Action::Revoke { identity } => {
                settings.peers.remove(identity);
                if settings.target_peer.as_ref() == Some(identity) {
                    settings.target_peer = None;
                }
            }
            _ => {}
        }
        process::validate(&settings)?;
        let current = self.0.compute.lock().await.state.clone();
        if matches!(current.as_str(), "loading" | "downloading" | "preparing")
            && !matches!(command.action, dto::Action::Stop)
        {
            return Err(Error::Busy);
        }
        let active = self.0.compute.lock().await.operation;
        if let Some(request) = active {
            let pending = self.read(|s| {
                Ok(s.intents.get(&request).is_some_and(|i| {
                    matches!(
                        i.outcome.state.as_str(),
                        "configuring" | "dispatching" | "stopping"
                    )
                }))
            })?;
            if pending && !matches!(command.action, dto::Action::Stop) {
                return Err(Error::Busy);
            }
        }
        let running = current == "serving";
        let previous = self.read(|s| Ok(s.compute.clone()))?;
        if running
            && (settings.model != previous.model
                || settings.memory_gb != previous.memory_gb
                || matches!(
                    command.action,
                    dto::Action::Prepare
                        | dto::Action::Download
                        | dto::Action::Import { .. }
                        | dto::Action::Remove { .. }
                        | dto::Action::Start
                ))
        {
            return Err(Error::Busy);
        }
        if matches!(command.action, dto::Action::Start)
            && !self.read(|s| {
                Ok(settings
                    .model
                    .as_ref()
                    .is_some_and(|m| s.compute_models.contains(m))
                    && s.compute_runtime_prepared)
            })?
        {
            return Err(Error::Invalid);
        }
        self.remember(command.request_id, digest, None)?;
        self.update(|s| {
            settings.revision = previous.revision.checked_add(1).ok_or(Error::Limit)?;
            s.compute = settings.clone();
            Ok(())
        })?;
        if matches!(command.action, dto::Action::Stop) {
            self.update(|s| {
                s.compute_generation = s.compute_generation.checked_add(1).ok_or(Error::Limit)?;
                Ok(())
            })?;
            let mut runtime = self.0.compute.lock().await;
            if let Some(previous) = runtime.operation
                && previous != command.request_id
            {
                let pending = self.read(|s| {
                    Ok(s.intents.get(&previous).is_some_and(|i| {
                        matches!(i.outcome.state.as_str(), "configuring" | "dispatching")
                    }))
                })?;
                if pending {
                    self.finish(
                        previous,
                        "outcome_unknown",
                        Some("compute_stopped_no_replay"),
                    )?;
                }
            }
            runtime.state = "stopping".into();
            runtime.operation = Some(command.request_id);
            if let Some(stop) = &runtime.stop {
                stop.send_replace(true);
            } else {
                runtime.state = "off".into();
                drop(runtime);
                return self.finish(command.request_id, "completed", None);
            }
            return self.finish(command.request_id, "stopping", None);
        }
        if self.ensure_compute().await.is_err() {
            return self.finish(command.request_id, "failed", Some("compute_unavailable"));
        }
        let mut runtime = self.0.compute.lock().await;
        runtime.operation = Some(command.request_id);
        runtime.error = None;
        let sender = runtime.sender.clone().ok_or(Error::Unavailable)?;
        let action = match &command.action {
            dto::Action::Prepare => "prepare",
            dto::Action::Download => "download",
            dto::Action::Start => "start",
            dto::Action::Import { .. } => "import",
            dto::Action::Remove { .. } => "remove",
            _ => "configure",
        };
        // Publish the expected receipt before the helper can acknowledge it.
        let outcome = self.finish(
            command.request_id,
            if action == "configure" {
                "configuring"
            } else {
                "dispatching"
            },
            None,
        )?;
        let config = process::wire_settings(&settings);
        if sender.capacity() < if action == "configure" { 1 } else { 2 } {
            return self.finish(
                command.request_id,
                "failed",
                Some("compute_dispatch_not_sent"),
            );
        }
        sender.try_send(json!({"requestId":command.request_id.to_string(),"action":"configure","config":config})).map_err(|_|Error::Busy)?;
        if action != "configure" {
            runtime.state = if action == "start" {
                "loading"
            } else if action == "download" {
                "downloading"
            } else {
                "preparing"
            }
            .into();
            let mut frame = json!({"requestId":command.request_id.to_string(),"action":action});
            if let dto::Action::Import { path } = &command.action {
                frame["path"] = json!(path);
            }
            if let dto::Action::Remove { model } = &command.action {
                frame["model"] = json!(model);
            }
            sender.try_send(frame).map_err(|_| Error::Busy)?;
        }
        Ok(outcome)
    }
}

#[cfg(test)]
mod tests;
