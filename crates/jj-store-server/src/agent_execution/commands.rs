use super::*;
#[async_trait::async_trait]
impl Manager for Supervisor {
    async fn catalog(
        &self,
        actor: PrincipalId,
        runner: String,
    ) -> Result<jj_store_service::agent_execution::catalog::Catalog> {
        self.actor(actor)?;
        self.model_catalog(runner).await
    }
    async fn send_conversation(
        &self,
        actor: PrincipalId,
        command: jj_store_service::agent_execution::conversations::Send,
    ) -> Result<jj_store_service::agent_execution::conversations::SendOutcome> {
        self.conversation_send(actor, command).await
    }
    async fn control_agent(
        &self,
        actor: PrincipalId,
        command: jj_store_service::agent_execution::fleet::Control,
    ) -> Result<Outcome> {
        self.agent_control(actor, command).await
    }
    async fn runtime_status(
        &self,
        actor: PrincipalId,
    ) -> Result<jj_store_service::agent_execution::tooling::Status> {
        self.actor(actor)?;
        self.tooling_status().await
    }
    async fn install_runtime(
        &self,
        actor: PrincipalId,
        command: jj_store_service::agent_execution::tooling::Command,
    ) -> Result<jj_store_service::agent_execution::tooling::Outcome> {
        self.actor(actor)?;
        self.tooling_install(command).await
    }
    async fn runtime_outcome(
        &self,
        actor: PrincipalId,
        request: RequestId,
    ) -> Result<jj_store_service::agent_execution::tooling::Outcome> {
        self.actor(actor)?;
        self.install_outcome(request)
    }
    async fn status(&self, actor: PrincipalId) -> Result<Status> {
        self.actor(actor)?;
        let mut status = self.read(|s| {
            Ok(Status {
                version: 1,
                runtime: if self.ready() {
                    "ready"
                } else if self
                    .0
                    .privacy_failed
                    .load(std::sync::atomic::Ordering::Acquire)
                {
                    "unavailable"
                } else {
                    "preparing"
                }
                .into(),
                revision: s.setup_revision.unwrap_or(s.revision),
                setups: s.setups.values().cloned().collect(),
                sessions: s.sessions.values().map(|s| s.view.clone()).collect(),
                runners: vec![],
                live_agents: vec![],
                conversations: s.conversations.values().cloned().collect(),
            })
        })?;
        status.runners = self.runners()?;
        status.live_agents = self.live_agents()?;
        let mut admitted = Vec::new();
        for mut session in status.sessions {
            let live = self.sender(&session.id).is_ok();
            session.connected = connection_ready(&session, live);
            let saved = self.saved(&session.id)?;
            if self
                .authorized(session.repository, &saved.profile, false)
                .await
                .is_ok()
            {
                feedback::attach_identity(&mut session, &saved)?;
                admitted.push(session);
            }
        }
        status.sessions = admitted;
        Ok(status)
    }
    async fn targets(
        &self,
        actor: PrincipalId,
        repository: Option<RepositoryId>,
    ) -> Result<Vec<Target>> {
        self.actor(actor)?;
        self.selections(repository).await
    }
    async fn save_setup(&self, actor: PrincipalId, mut command: SetupCommand) -> Result<Status> {
        self.actor(actor)?;
        if command.version != 1 || command.setup.is_some() == command.remove.is_some() {
            return Err(Error::Invalid);
        }
        let digest = journal::digest(&command)?;
        let _guard = self.0.mutation.lock().await;
        if self.duplicate(command.request_id, &digest)?.is_some() {
            return self.status(actor).await;
        }
        if let Some(setup) = &command.setup {
            if !label(&setup.id, 64)
                || !label(&setup.name, 128)
                || !avatar::valid(&setup.avatar)
                || setup.description.chars().count() > 280
                || setup.description.contains('\0')
                || setup.instructions.len() > 32768
                || !matches!(setup.runner.as_str(), "codex" | "cursor")
            {
                return Err(Error::Invalid);
            }
            preferences(&setup.preferences)?;
            let sanitizer =
                runtime::sanitizer(self, &self.0.resources.root.join("workspaces")).await?;
            let avatar = setup.avatar.clone();
            let mut authored = setup.clone();
            authored.avatar.clear();
            command.setup = Some(
                serde_json::from_value(runtime::safe(
                    &sanitizer,
                    serde_json::to_value(authored).map_err(|_| Error::Invalid)?,
                )?)
                .map_err(|_| Error::Invalid)?,
            );
            command.setup.as_mut().ok_or(Error::Invalid)?.avatar = avatar;
        }
        let removed = command.remove.clone();
        self.update(|s| {
            if command.expected_revision != s.setup_revision.unwrap_or(s.revision) {
                return Err(Error::Stale);
            }
            s.setup_revision = Some(
                command
                    .expected_revision
                    .checked_add(1)
                    .ok_or(Error::Limit)?,
            );
            if s.intents.len() >= 4096 {
                return Err(Error::Limit);
            }
            if let Some(setup) = command.setup {
                if s.setups.len() >= 64 && !s.setups.contains_key(&setup.id) {
                    return Err(Error::Limit);
                }
                s.setups.insert(setup.id.clone(), setup);
            }
            if let Some(id) = command.remove {
                s.setups.remove(&id);
                s.warm_agents.remove(&id);
            }
            s.intents.insert(
                command.request_id,
                Intent {
                    digest,
                    prompt: None,
                    outcome: Outcome {
                        version: 1,
                        request_id: command.request_id,
                        session_id: None,
                        state: "saved".into(),
                        error: None,
                    },
                },
            );
            Ok(())
        })?;
        if let Some(id) = removed {
            self.stop_worker(&id)?;
        }
        self.status(actor).await
    }
    async fn launch(&self, actor: PrincipalId, command: Launch) -> Result<Outcome> {
        self.launch_session(actor, command).await
    }
    async fn command(&self, actor: PrincipalId, command: Command) -> Result<Outcome> {
        self.actor(actor)?;
        if command.version != 1 || self.0.stopping.load(std::sync::atomic::Ordering::Acquire) {
            return Err(Error::Invalid);
        }
        let digest = journal::digest(&command)?;
        let _guard = self.0.mutation.lock().await;
        let saved = self.saved(&command.session_id)?;
        self.authorized(saved.view.repository, &saved.profile, true)
            .await?;
        for binding in &saved.view.repository_bindings {
            self.authorized(binding.repository, &binding.profile, true)
                .await?;
        }
        if let Some(outcome) = self.duplicate(command.request_id, &digest)? {
            return Ok(outcome);
        }
        let id = &command.session_id;
        let request = command.request_id.to_string();
        let mut context_batch = None;
        let mut carries_initial_context = false;
        let mut frame = json!({"version":2,"requestId":request});
        match &command.action {
            Action::Prompt {
                text,
                max_duration_ms,
            } => {
                if text.trim().is_empty()
                    || text.len() > 256 * 1024
                    || text.contains('\0')
                    || !(1..=3600000).contains(max_duration_ms)
                {
                    return Err(Error::Invalid);
                }
                self.sender(id)?;
                let first = self.read(|s| delivery::needs_initial_context(s, id))?;
                carries_initial_context = first;
                let mut feedback =
                    self.prompt_feedback(&saved, first)
                        .await
                        .map_err(|error| match error {
                            Error::Limit => Error::FeedbackLimit,
                            Error::Denied => Error::Denied,
                            _ => Error::FeedbackUnavailable,
                        })?;
                if first {
                    feedback
                        .revisions
                        .extend(saved.context_replies.iter().cloned());
                }
                self.authorized(saved.view.repository, &saved.profile, true)
                    .await?;
                let prepared = if first {
                    format!(
                        "{}\n{}\n{}\n{}",
                        saved.instructions,
                        saved.context.as_deref().unwrap_or(""),
                        retained_context(&saved),
                        text
                    )
                } else {
                    text.clone()
                };
                frame["type"] = json!("prompt");
                frame["text"] = json!(format!("{}{prepared}", feedback.text));
                context_batch = Some(feedback);
                frame["maxDurationMs"] = json!(max_duration_ms);
            }
            Action::Configure { preferences: p } => {
                preferences(p)?;
                self.sender(id)?;
                frame["type"] = json!("configure");
                frame["model"] = json!(p.model);
                frame["effort"] = json!(p.effort);
                frame["mode"] = json!(p.mode);
            }
            Action::Permission {
                permission_id,
                option_id,
            } => {
                self.sender(id)?;
                let pending = saved
                    .events
                    .iter()
                    .rev()
                    .find(|e| e.payload["permissionId"] == *permission_id)
                    .ok_or(Error::Stale)?;
                if pending.payload["type"] != "permission" || saved.view.state != "running" {
                    return Err(Error::Stale);
                }
                if let Some(option) = option_id
                    && !pending.payload["request"]["options"]
                        .as_array()
                        .is_some_and(|opts| opts.iter().any(|o| o["optionId"] == *option))
                {
                    return Err(Error::Invalid);
                }
                frame["type"] = json!("permission");
                frame["permissionId"] = json!(permission_id);
                frame["optionId"] = json!(option_id);
            }
            Action::Cancel { target } => {
                self.sender(id)?;
                self.read(|s| {
                    let r: RequestId = target.parse().map_err(|_| Error::Invalid)?;
                    let intent = s.intents.get(&r).ok_or(Error::Invalid)?;
                    if intent.outcome.session_id.as_deref() != Some(id) {
                        return Err(Error::Invalid);
                    }
                    Ok(())
                })?;
                frame["type"] = json!("cancel");
                frame["target"] = json!(target);
            }
            Action::End => {
                self.sender(id)?;
                frame["type"] = json!("close");
            }
            Action::Resume | Action::ContinueWithContext | Action::Cleanup => {
                if self.sender(id).is_ok() || saved.view.state == "cleaned" {
                    return Err(Error::Busy);
                }
                if saved.view.conversation_id.is_some() {
                    if command.action == Action::Cleanup {
                        return Err(Error::Invalid);
                    }
                    for workspace in saved.code_workspaces.values() {
                        let current = self
                            .0
                            .resources
                            .driver
                            .inspect(workspace.path.clone())
                            .await?;
                        if current.metadata != workspace.metadata || current.name != workspace.name
                        {
                            return Err(Error::Stale);
                        }
                    }
                } else if command.action != Action::Cleanup && !saved.workspace_ready {
                    return Err(Error::Stale);
                }
                let current = if saved.view.conversation_id.is_some() {
                    saved.workspace.clone()
                } else {
                    self.0
                        .resources
                        .driver
                        .inspect(saved.workspace.path.clone())
                        .await?
                };
                if current.metadata != saved.workspace.metadata
                    || current.name != saved.workspace.name
                {
                    return Err(Error::Stale);
                }
                if command.action == Action::Resume
                    && (saved.view.capabilities["initialize"]["loadSession"] != true
                        || saved.view.adapter_session_id.is_none())
                {
                    return Err(Error::Invalid);
                }
            }
        }
        if let Some(mut batch) = context_batch {
            let sanitizer = runtime::sanitizer(self, &saved.workspace.path).await?;
            let original = if let Action::Prompt { text, .. } = &command.action {
                text
            } else {
                return Err(Error::Invalid);
            };
            batch.text = runtime::safe(&sanitizer, json!({"text":batch.text}))?["text"]
                .as_str()
                .ok_or(Error::Invalid)?
                .to_owned();
            if batch.text.len() > 32768 {
                return Err(Error::FeedbackLimit);
            }
            let original = runtime::safe(&sanitizer, json!({"text":original}))?["text"]
                .as_str()
                .ok_or(Error::Invalid)?
                .to_owned();
            // Dispatch exactly the sanitized frame retained with the original intent.
            frame = runtime::safe(&sanitizer, frame)?;
            self.update(|s| {
                delivery::prepare(
                    s,
                    id,
                    command.request_id,
                    digest,
                    original,
                    batch,
                    frame.clone(),
                    carries_initial_context,
                )
            })?;
        } else {
            self.remember(command.request_id, digest, Some(id.clone()))?;
        }
        match command.action {
            Action::Cleanup => {
                if self
                    .0
                    .resources
                    .driver
                    .cleanup(&saved.workspace)
                    .await
                    .is_err()
                {
                    return self.finish(
                        command.request_id,
                        "failed",
                        Some("cleanup_refused_retained_workspace"),
                    );
                }
                self.session_state(id, "cleaned", None)?;
                self.finish(command.request_id, "cleaned", None)
            }
            Action::Resume | Action::ContinueWithContext => {
                if saved.view.conversation_id.is_some() {
                    let setup = self.read(|s| {
                        s.setups
                            .get(saved.view.setup_id.as_deref().ok_or(Error::Invalid)?)
                            .cloned()
                            .ok_or(Error::Stale)
                    })?;
                    if command.action == Action::ContinueWithContext {
                        self.update(|s| {
                            s.sessions
                                .get_mut(id)
                                .ok_or(Error::Invalid)?
                                .view
                                .adapter_session_id = None;
                            delivery::continuation(s, id)
                        })?;
                        self.append(id, json!({"type":"continuation","requestId":request,"label":"New adapter session with saved context"}))?;
                    }
                    drop(_guard);
                    self.open_participation(&setup, id).await?;
                    return self.finish(command.request_id, "opening", None);
                }
                let load = if command.action == Action::Resume {
                    saved.view.adapter_session_id
                } else {
                    None
                };
                if load.is_none() {
                    self.update(|s| delivery::continuation(s, id))?;
                    self.append(id, json!({"type":"continuation","requestId":request,"label":"New adapter session with saved context"}))?;
                }
                self.session_state(id, "opening", None)?;
                if self.spawn(id, load).await.is_err() {
                    self.session_state(id, "interrupted", Some("agent_unavailable"))?;
                    return self.finish(command.request_id, "failed", Some("agent_unavailable"));
                }
                self.finish(command.request_id, "opening", None)
            }
            action => {
                if let Action::Configure { preferences } = action {
                    let sanitizer = runtime::sanitizer(self, &saved.workspace.path).await?;
                    let preferences: Preferences = serde_json::from_value(runtime::safe(
                        &sanitizer,
                        serde_json::to_value(preferences).map_err(|_| Error::Invalid)?,
                    )?)
                    .map_err(|_| Error::Invalid)?;
                    self.update(|s| {
                        s.sessions
                            .get_mut(id)
                            .ok_or(Error::Invalid)?
                            .view
                            .preferences = preferences;
                        Ok(())
                    })?;
                }
                if self.send(id, frame).is_err() {
                    self.update(|s| delivery::observe(s, id, &json!({"type":"error", "requestId":request, "code":"dispatch_not_sent"})))?;
                    return self.finish(command.request_id, "failed", Some("dispatch_not_sent"));
                }
                self.read(|s| Ok(s.intents[&command.request_id].outcome.clone()))
            }
        }
    }
    async fn outcome(&self, actor: PrincipalId, request: RequestId) -> Result<Outcome> {
        self.actor(actor)?;
        let outcome = self.read(|s| {
            s.intents
                .get(&request)
                .map(|i| i.outcome.clone())
                .map_or_else(
                    || {
                        Ok(Outcome {
                            version: 1,
                            request_id: request,
                            session_id: None,
                            state: "not_known".into(),
                            error: None,
                        })
                    },
                    Ok,
                )
        })?;
        if let Some(id) = &outcome.session_id
            && let Ok(saved) = self.saved(id)
        {
            self.authorized(saved.view.repository, &saved.profile, false)
                .await?;
        }
        Ok(outcome)
    }
    async fn events(&self, actor: PrincipalId, session: String, after: u64) -> Result<EventPage> {
        self.actor(actor)?;
        let saved = self.saved(&session)?;
        self.authorized(saved.view.repository, &saved.profile, false)
            .await?;
        let events: Vec<_> = saved
            .events
            .into_iter()
            .filter(|e| e.sequence > after)
            .take(128)
            .collect();
        let next = events
            .last()
            .filter(|e| e.sequence < saved.view.last_sequence)
            .map(|e| e.sequence);
        Ok(EventPage {
            version: 1,
            events,
            next,
        })
    }
    async fn feedback(&self, actor: PrincipalId, session: String) -> Result<Feedback> {
        self.actor(actor)?;
        self.feedback_saved(&self.saved(&session)?).await
    }
    fn subscribe(&self) -> broadcast::Receiver<u64> {
        self.0.changed.subscribe()
    }
    async fn shutdown(&self) -> Result<()> {
        let _shutdown = self.0.shutdown.lock().await;
        if let Some(success) = *self
            .0
            .shutdown_finished
            .lock()
            .map_err(|_| Error::Unavailable)?
        {
            return if success {
                Ok(())
            } else {
                Err(Error::Unavailable)
            };
        }
        self.0
            .stopping
            .store(true, std::sync::atomic::Ordering::Release);
        let installer = self
            .0
            .installer_task
            .lock()
            .map_err(|_| Error::Unavailable)?
            .take();
        if let Some(installer) = installer {
            installer.abort();
            let _ = tokio::time::timeout(Duration::from_secs(3), installer).await;
        }
        if let Some(worker) = self
            .0
            .publication_worker
            .lock()
            .map_err(|_| Error::Unavailable)?
            .take()
        {
            worker.abort();
        }
        let senders = self
            .0
            .runtimes
            .lock()
            .map_err(|_| Error::Unavailable)?
            .clone();
        for (id, sender) in senders {
            let _ = sender
                .try_send(json!({"version":2,"type":"close","requestId":format!("shutdown-{id}")}));
        }
        {
            let mut workers = self.0.fleet.lock().map_err(|_| Error::Unavailable)?;
            for (id, worker) in workers.iter_mut() {
                worker.state = "stopping".into();
                let _ = worker.sender.try_send(
                    json!({"version":3,"type":"close","requestId":format!("shutdown-{id}")}),
                );
            }
        }
        let stopped = tokio::time::timeout(Duration::from_secs(20), async {
            loop {
                if self
                    .0
                    .runtimes
                    .lock()
                    .map_err(|_| Error::Unavailable)?
                    .is_empty()
                    && self
                        .0
                        .fleet
                        .lock()
                        .map_err(|_| Error::Unavailable)?
                        .values()
                        .all(|w| w.sender.is_closed())
                    && !self
                        .0
                        .catalog_active
                        .load(std::sync::atomic::Ordering::Acquire)
                {
                    return Ok(());
                }
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
        })
        .await;
        if stopped.is_err() {
            for task in self
                .0
                .runtime_tasks
                .lock()
                .map_err(|_| Error::Unavailable)?
                .values()
            {
                task.abort();
            }
            *self
                .0
                .shutdown_finished
                .lock()
                .map_err(|_| Error::Unavailable)? = Some(false);
            return Err(Error::Unavailable);
        }
        // Cancellation leaves the exact spool request and journal cursor pending.
        // A slow store must not hold installation shutdown indefinitely.
        let _ = tokio::time::timeout(Duration::from_secs(3), self.publish()).await;
        *self
            .0
            .shutdown_finished
            .lock()
            .map_err(|_| Error::Unavailable)? = Some(true);
        stopped.map_err(|_| Error::Unavailable)?
    }
}

fn connection_ready(session: &Session, live: bool) -> bool {
    live && session.adapter_session_id.is_some()
        && matches!(session.state.as_str(), "idle" | "running")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn presence_requires_live_process_and_completed_handshake() {
        let mut session: Session = serde_json::from_value(json!({
            "id":"a", "repository":"11".repeat(16), "target_id":"target",
            "runner":"codex", "title":"Helper", "avatar":"🤖", "workspace":"/tmp/a",
            "base_commit":"22".repeat(20), "review_commit":"22".repeat(20),
            "operation":"33".repeat(64), "state":"idle", "adapter_session_id":"adapter",
            "capabilities":{}, "preferences":{"model":null,"effort":null,"mode":null},
            "last_sequence":0,"published_sequence":0,"last_error":null
        }))
        .unwrap();
        assert_eq!(session.setup_id, None);
        assert!(!session.connected);
        assert!(!connection_ready(&session, false));
        assert!(connection_ready(&session, true));
        session.state = "running".into();
        assert!(connection_ready(&session, true));
        for state in ["preparing", "opening", "interrupted", "ended", "cleaned"] {
            session.state = state.into();
            assert!(!connection_ready(&session, true));
        }
        session.state = "idle".into();
        session.adapter_session_id = None;
        assert!(!connection_ready(&session, true));
        session.setup_id = Some("helper".into());
        let restored: Session =
            serde_json::from_value(serde_json::to_value(&session).unwrap()).unwrap();
        assert_eq!(restored.setup_id.as_deref(), Some("helper"));
    }
}

pub(super) fn retained_context(saved: &SavedSession) -> String {
    if saved.view.conversation_id.is_none()
        || !saved
            .events
            .iter()
            .any(|e| e.payload["type"] == "continuation")
    {
        return String::new();
    }
    let mut bytes = 0;
    let mut messages = saved
        .events
        .iter()
        .rev()
        .filter_map(|e| {
            let (role, text) = match e.payload["type"].as_str() {
                Some("prompt") => ("user", e.payload["text"].as_str()?),
                Some("record") => ("assistant", e.payload["assistantText"].as_str()?),
                _ => return None,
            };
            if text.len() > 16384 || bytes + text.len() > 16384 {
                return None;
            }
            bytes += text.len();
            Some(json!({"role":role,"text":text}).to_string())
        })
        .take(20)
        .collect::<Vec<_>>();
    messages.reverse();
    if messages.is_empty() {
        String::new()
    } else {
        format!(
            "Retained conversation history (continuation):\n{}\nEnd retained history.",
            messages.join("\n")
        )
    }
}
