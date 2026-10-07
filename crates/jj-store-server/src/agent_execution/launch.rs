use super::*;
impl Supervisor {
    pub(super) async fn launch_session(
        &self,
        actor: PrincipalId,
        command: Launch,
    ) -> Result<Outcome> {
        self.actor(actor)?;
        if self.0.stopping.load(std::sync::atomic::Ordering::Acquire) {
            return Err(Error::Unavailable);
        }
        if command.version != 1
            || !valid_commit(&command.commit)
            || !matches!(command.runner.as_str(), "codex" | "cursor")
            || command
                .context
                .as_ref()
                .is_some_and(|c| c.len() > 32768 || c.contains('\0'))
        {
            return Err(Error::Invalid);
        }
        let digest = journal::digest(&command)?;
        if !self.ready() {
            return Err(Error::Busy);
        }
        let _guard = self.0.mutation.lock().await;
        if let Some(outcome) = self.duplicate(command.request_id, &digest)? {
            if let Some(id) = &outcome.session_id {
                let saved = self.saved(id)?;
                self.authorized(saved.view.repository, &saved.profile, true)
                    .await?;
            }
            return Ok(outcome);
        }
        if !self
            .0
            .runtimes
            .lock()
            .map_err(|_| Error::Unavailable)?
            .is_empty()
        {
            return Err(Error::Busy);
        }
        let target = self
            .selections(Some(command.repository))
            .await?
            .into_iter()
            .find(|t| t.id == command.target_id)
            .ok_or(Error::Stale)?;
        if target.operation != command.operation {
            return Err(Error::Stale);
        }
        self.authorized(target.repository, &target.profile, true)
            .await?;
        let source = self
            .0
            .resources
            .driver
            .inspect(PathBuf::from(&target.folder))
            .await?;
        if source.operation != target.operation {
            return Err(Error::Stale);
        }
        let setup = self.read(|s| {
            command
                .setup_id
                .as_ref()
                .map(|id| s.setups.get(id).cloned().ok_or(Error::Stale))
                .transpose()
        })?;
        if setup.as_ref().is_some_and(|s| s.runner != command.runner) {
            return Err(Error::Invalid);
        }
        let id = command.request_id.to_string();
        let name = format!("agent-{id}");
        let path = self.0.resources.root.join("workspaces").join(&name);
        self.read(|state| avatar::reserve(state, setup.as_ref().map_or(4, |s| s.avatar.len())))?;
        self.remember(command.request_id, digest, Some(id.clone()))?;
        let sanitizer = runtime::sanitizer(self, &path).await?;
        let context = runtime::safe(&sanitizer, json!({"context":command.context}))?["context"]
            .as_str()
            .map(str::to_owned);
        let view = Session {
            id: id.clone(),
            conversation_id: None,
            repository_bindings: vec![],
            setup_id: command.setup_id.clone(),
            recorded_session_ids: Vec::new(),
            workspace_name: None,
            workspace_operation: None,
            connected: false,
            repository: target.repository,
            target_id: target.id,
            runner: command.runner,
            title: setup
                .as_ref()
                .map_or_else(|| format!("{} session", target.name), |s| s.name.clone()),
            avatar: setup
                .as_ref()
                .map_or_else(|| "🤖".into(), |s| s.avatar.clone()),
            workspace: path.to_string_lossy().into_owned(),
            base_commit: command.commit.clone(),
            review_commit: command.commit.clone(),
            operation: command.operation,
            state: "preparing".into(),
            adapter_session_id: None,
            capabilities: json!({}),
            preferences: setup.as_ref().map_or(
                Preferences {
                    model: None,
                    effort: None,
                    mode: None,
                },
                |s| s.preferences.clone(),
            ),
            last_sequence: 0,
            last_activity_at_millis: None,
            published_sequence: 0,
            last_error: None,
        };
        let planned = Workspace {
            source: source.path.clone(),
            path: path.clone(),
            metadata: source.metadata.clone(),
            operation: command.operation,
            commit: command.commit.clone(),
            name: name.clone(),
        };
        let saved = SavedSession {
            view,
            workspace: planned,
            profile: target.profile,
            instructions: setup.map_or(String::new(), |s| s.instructions),
            context,
            events: Vec::new(),
            workspace_ready: false,
            pending_capture_end: None,
            delivered_replies: Default::default(),
            context_replies: Default::default(),
            context_generation: 0,
            initial_context_delivered: false,
            code_workspaces: BTreeMap::new(),
            origin_link: None,
            origin_linked: false,
        };
        self.update(|s| {
            if s.sessions.len() >= 64 {
                return Err(Error::Limit);
            }
            s.sessions.insert(id.clone(), saved);
            Ok(())
        })?;
        let workspace = match self
            .0
            .resources
            .driver
            .create(source, path, command.operation, command.commit, name)
            .await
        {
            Ok(value) => value,
            Err(_) => {
                self.session_state(&id, "interrupted", Some("workspace_creation_unknown"))?;
                return self.finish(
                    command.request_id,
                    "outcome_unknown",
                    Some("workspace_creation_unknown"),
                );
            }
        };
        self.update(|s| {
            let saved = s.sessions.get_mut(&id).ok_or(Error::Invalid)?;
            saved.workspace = workspace;
            saved.workspace_ready = true;
            Ok(())
        })?;
        if self.spawn(&id, None).await.is_err() {
            self.session_state(&id, "interrupted", Some("agent_unavailable"))?;
            return self.finish(command.request_id, "failed", Some("agent_unavailable"));
        }
        self.finish(command.request_id, "opening", None)
    }
}
