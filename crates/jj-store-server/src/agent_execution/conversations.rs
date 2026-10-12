//! Durable explicit Send prepares all participants and repositories before prompting.
use super::*;
use jj_store_service::agent_execution::conversations as dto;
impl Supervisor {
    pub(super) async fn conversation_send(
        &self,
        actor: PrincipalId,
        command: dto::Send,
    ) -> Result<dto::SendOutcome> {
        self.actor(actor)?;
        let request = command.request_id;
        let digest = journal::digest(&command)?;
        match self.prepare_conversation_send(actor, command.clone()).await {
            Ok(outcome) => Ok(outcome),
            Err(error) => {
                // An admitted parent never silently retries provisioning or prompts.
                if !matches!(error, Error::Denied | Error::Conflict | Error::Invalid)
                    && self.read(|s| {
                        Ok(s.intents.get(&request).is_some_and(|i| {
                            i.digest == digest
                                && (i.outcome.state == "dispatching"
                                    || i.outcome.state.starts_with("preparing_"))
                        }))
                    })?
                {
                    let stage = self.read(|s| Ok(s.intents[&request].outcome.state.clone()))?;
                    let diagnostic = format!("chat_{stage}_failed_no_prompt");
                    self.finish(request, "failed", Some(&diagnostic))?;
                    return Ok(dto::SendOutcome {
                        comment_publication: self.comment_publication(command.request_id)?,
                        request_id: request,
                        conversation_id: command.conversation_id,
                        agents: command
                            .agent_ids
                            .iter()
                            .map(|id| Outcome {
                                comment_publication: None,
                                version: 1,
                                request_id: RequestId::new(publication::identity(&format!(
                                    "chat-send/{request}/{id}"
                                ))),
                                session_id: None,
                                state: "failed".into(),
                                error: Some(diagnostic.clone()),
                            })
                            .collect(),
                    });
                }
                Err(error)
            }
        }
    }

    async fn prepare_conversation_send(
        &self,
        actor: PrincipalId,
        command: dto::Send,
    ) -> Result<dto::SendOutcome> {
        self.actor(actor)?;
        if command.version != 1
            || command.conversation_id.parse::<RequestId>().is_err()
            || command.agent_ids.is_empty()
            || command.agent_ids.len() > 4
            || command.attachments.len() > 8
            || (command.text.trim().is_empty() && command.comments.is_none())
            || command.text.len() > 256 * 1024
            || command.text.contains('\0')
            || !(1..=3600000).contains(&command.max_duration_ms)
            || !label(&command.origin.scope, 256)
            || command.origin.context.len() > 32768
        {
            return Err(Error::Invalid);
        }
        if let Some(comments) = &command.comments {
            comments.validate().map_err(|_| Error::Invalid)?;
            if comments.text.is_some() || comments.message_id.is_some() {
                return Err(Error::Invalid);
            }
        }
        let ids = command
            .agent_ids
            .iter()
            .collect::<std::collections::BTreeSet<_>>();
        if ids.len() != command.agent_ids.len()
            || command
                .attachments
                .iter()
                .map(|a| a.repository)
                .collect::<std::collections::BTreeSet<_>>()
                .len()
                != command.attachments.len()
        {
            return Err(Error::Invalid);
        }
        let _dispatch = self.0.conversation_dispatch.lock().await;
        if let Some(home) = command.origin.home {
            let profile = if self
                .0
                .resources
                .folders
                .as_ref()
                .is_some_and(|f| f.open(home).is_ok())
            {
                "local_jj_git"
            } else {
                "native"
            };
            self.authorized(home, profile, true).await?;
        }
        for attachment in &command.attachments {
            let profile = if self
                .0
                .resources
                .folders
                .as_ref()
                .is_some_and(|f| f.open(attachment.repository).is_ok())
            {
                "local_jj_git"
            } else {
                "native"
            };
            self.authorized(attachment.repository, profile, true)
                .await?;
        }
        let digest = journal::digest(&command)?;
        let guard = self.0.mutation.lock().await;
        if let Some(parent) = self.duplicate(command.request_id, &digest)? {
            if command
                .comments
                .as_ref()
                .is_some_and(|s| !s.workspace_comments.is_empty())
            {
                let _ = self.recover_file_publication(command.request_id).await;
            }
            if command
                .comments
                .as_ref()
                .is_none_or(|s| s.workspace_comments.is_empty())
                && let Some(saved) =
                    self.read(|s| Ok(s.comment_submissions.get(&command.request_id).cloned()))?
            {
                // An explicit retry may recover publication receipts; it never
                // dispatches already admitted participants a second time.
                let _ = self.publish_chat_comments(&command, &saved.profile).await;
            }
            let agents = command
                .agent_ids
                .iter()
                .map(|id| {
                    let child = RequestId::new(publication::identity(&format!(
                        "chat-send/{}/{id}",
                        command.request_id
                    )));
                    self.read(|s| {
                        Ok(s.intents
                            .get(&child)
                            .map(|i| i.outcome.clone())
                            .unwrap_or(Outcome {
                                comment_publication: None,
                                version: 1,
                                request_id: child,
                                session_id: None,
                                state: if parent.state == "failed" {
                                    "failed"
                                } else {
                                    "outcome_unknown"
                                }
                                .into(),
                                error: parent
                                    .error
                                    .clone()
                                    .or(Some("preparation_interrupted_no_replay".into())),
                            }))
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            return Ok(dto::SendOutcome {
                comment_publication: self.comment_publication(command.request_id)?,
                request_id: command.request_id,
                conversation_id: command.conversation_id,
                agents,
            });
        }
        let setups = self.read(|s| {
            command
                .agent_ids
                .iter()
                .map(|id| {
                    s.setups
                        .get(id)
                        .map(|setup| configuration::resolve(setup, &s.settings))
                        .ok_or(Error::Stale)
                })
                .collect::<Result<Vec<_>>>()
        })?;
        if let Some(thread) = command.origin.thread {
            let home = command.origin.home.ok_or(Error::Invalid)?;
            let profile = if self
                .0
                .resources
                .folders
                .as_ref()
                .is_some_and(|f| f.open(home).is_ok())
            {
                "local_jj_git"
            } else {
                "native"
            };
            let selected = self.home_thread_summary(home, profile, thread).await?;
            if selected.value.codebase.is_some()
                && command.thread_codebase != Some(selected.reference)
            {
                return Err(Error::Stale);
            }
            let allowed = selected.value.codebase.unwrap_or_else(|| vec![home]);
            if command
                .attachments
                .iter()
                .any(|a| !allowed.contains(&a.repository))
            {
                return Err(Error::Denied);
            }
        }
        self.remember(command.request_id, digest, None)?;
        self.finish(command.request_id, "preparing_home", None)?;
        let (home, profile, source) = self.conversation_home(command.origin.home).await?;
        self.update(|s| {
            if let Some(old) = s.conversations.get(&command.conversation_id) {
                if old.origin.scope != command.origin.scope
                    || old.origin.home != command.origin.home
                    || old.origin.thread != command.origin.thread
                    || old.origin.agent_target != command.origin.agent_target
                    || old.home != home
                {
                    return Err(Error::Stale);
                }
            } else {
                if s.conversations.len() >= 128 {
                    return Err(Error::Limit);
                }
                s.conversations.insert(
                    command.conversation_id.clone(),
                    dto::Conversation {
                        id: command.conversation_id.clone(),
                        origin: command.origin.clone(),
                        home,
                        discussion_thread: command.origin.thread,
                        participants: Vec::new(),
                        metadata_auto_eligible: s.settings.metadata.is_some(),
                    },
                );
            }
            Ok(())
        })?;
        self.finish(command.request_id, "preparing_repositories", None)?;
        let mut targets = Vec::new();
        for attachment in &command.attachments {
            let existing = self.read(|s| {
                Ok(s.sessions
                    .values()
                    .find(|saved| {
                        saved.view.conversation_id.as_deref() == Some(&command.conversation_id)
                            && saved.code_workspaces.contains_key(&attachment.repository)
                    })
                    .map(|saved| {
                        let binding = saved
                            .view
                            .repository_bindings
                            .iter()
                            .find(|b| b.repository == attachment.repository)
                            .expect("validated binding");
                        Target {
                            id: binding.target_id.clone(),
                            repository: binding.repository,
                            name: binding.repository.to_string(),
                            folder: saved.code_workspaces[&binding.repository]
                                .source
                                .to_string_lossy()
                                .into_owned(),
                            profile: binding.profile.clone(),
                            operation: binding.selected_operation,
                            commit: binding.base_commit.clone(),
                        }
                    }))
            })?;
            targets.push(match existing {
                Some(target) => target,
                None => self.conversation_target(attachment).await?,
            });
        }
        let mut participants = Vec::new();
        for setup in setups {
            let base_id = hex::encode(publication::identity(&format!(
                "participation/{}/{}/{home}",
                command.conversation_id, setup.id
            )));
            let removed = self.read(|s| {
                Ok(s.sessions.get(&base_id).is_some_and(|saved| {
                    saved
                        .code_workspaces
                        .keys()
                        .any(|r| !command.attachments.iter().any(|a| a.repository == *r))
                }))
            })?;
            let reusable = self.read(|s| {
                Ok(s.sessions
                    .values()
                    .filter(|saved| {
                        saved.view.conversation_id.as_deref() == Some(&command.conversation_id)
                            && saved.view.setup_id.as_deref() == Some(&setup.id)
                            && saved
                                .code_workspaces
                                .keys()
                                .all(|r| command.attachments.iter().any(|a| a.repository == *r))
                    })
                    .max_by_key(|saved| saved.view.last_activity_at_millis)
                    .map(|saved| saved.view.id.clone()))
            })?;
            let id = if let Some(id) = reusable {
                id
            } else if removed {
                if let Some(thread) = command.origin.thread {
                    if self.thread_has_pending_work(home, thread)? {
                        return Err(Error::Busy);
                    }
                }
                hex::encode(publication::identity(&format!(
                    "participation-scope/{base_id}/{}",
                    command.request_id
                )))
            } else {
                base_id.clone()
            };
            let continuation = if removed {
                self.read(|s| Ok(prompt_context::retained(s, &base_id)))?
            } else {
                String::new()
            };
            if self.read(|s| Ok(!s.sessions.contains_key(&id)))? {
                let path = self
                    .0
                    .resources
                    .root
                    .join("workspaces")
                    .join(format!("agent-{id}"));
                let workspace = Workspace {
                    path: path.clone(),
                    name: format!("agent-{id}"),
                    ..source.clone()
                };
                let view = Session {
                    id: id.clone(),
                    conversation_id: Some(command.conversation_id.clone()),
                    repository_bindings: vec![],
                    setup_id: Some(setup.id.clone()),
                    recorded_session_ids: vec![],
                    workspace_name: None,
                    workspace_operation: None,
                    connected: false,
                    repository: home,
                    target_id: "conversation-home".into(),
                    runner: setup.runner.clone(),
                    title: setup.name.clone(),
                    conversation_title: None,
                    discovery_text: None,
                    avatar: setup.avatar.clone(),
                    workspace: path.to_string_lossy().into_owned(),
                    base_commit: source.commit.clone(),
                    review_commit: source.commit.clone(),
                    operation: source.operation,
                    state: "preparing".into(),
                    adapter_session_id: None,
                    capabilities: json!({}),
                    preferences: setup.preferences.clone(),
                    last_sequence: 0,
                    last_activity_at_millis: None,
                    published_sequence: 0,
                    last_error: None,
                };
                let saved = SavedSession {
                    options: setup.options.clone(),
                    view,
                    workspace,
                    profile: profile.clone(),
                    instructions: setup.instructions.clone(),
                    context: None,
                    continuation,
                    delivered_sections: BTreeMap::new(),
                    coordination: None,
                    events: vec![],
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
                journal::private_directory(&path)?;
            }
            if let Err(error) = self.prepare_coordination(&id).await {
                self.finish(
                    command.request_id,
                    "failed",
                    Some("coordination_preparation_failed_no_prompt"),
                )?;
                return Err(error);
            }
            self.prepare_bindings(&id, &targets, &command.origin)
                .await?;
            self.update(|s| {
                let c = s
                    .conversations
                    .get_mut(&command.conversation_id)
                    .ok_or(Error::Invalid)?;
                if !c.participants.contains(&id) {
                    c.participants.push(id.clone());
                }
                s.warm_agents.insert(setup.id.clone());
                Ok(())
            })?;
            participants.push((setup, id));
        }
        drop(guard);
        // Every adapter must be ready before the first real prompt is dispatched.
        for (setup, id) in &participants {
            if self.open_participation(setup, id).await.is_err() {
                self.finish(
                    command.request_id,
                    "failed",
                    Some("chat_preparation_failed_no_prompt"),
                )?;
                return Ok(dto::SendOutcome {
                    comment_publication: self.comment_publication(command.request_id)?,
                    request_id: command.request_id,
                    conversation_id: command.conversation_id,
                    agents: participants
                        .iter()
                        .map(|(setup, id)| Outcome {
                            comment_publication: None,
                            version: 1,
                            request_id: RequestId::new(publication::identity(&format!(
                                "chat-send/{}/{}",
                                command.request_id, setup.id
                            ))),
                            session_id: Some(id.clone()),
                            state: "failed".into(),
                            error: Some("chat_preparation_failed_no_prompt".into()),
                        })
                        .collect(),
                });
            }
        }
        self.finish(command.request_id, "preparing_publication", None)?;
        self.publish().await?;
        self.prepare_origin_discussion(&command.conversation_id, &profile)
            .await?;
        let deferred = command
            .comments
            .as_ref()
            .filter(|c| !c.workspace_comments.is_empty());
        if let Some(submission) = deferred {
            if Some(submission.thread_id)
                != self.read(|s| Ok(s.conversations[&command.conversation_id].discussion_thread))?
            {
                return Err(Error::Invalid);
            }
        }
        let comment_context = if deferred.is_some() {
            None
        } else {
            self.publish_chat_comments(&command, &profile).await?
        };
        for (_, id) in &participants {
            self.link_origin(id).await?;
        }
        let mut prepared = Vec::new();
        for (setup, id) in &participants {
            let child = RequestId::new(publication::identity(&format!(
                "chat-send/{}/{name}",
                command.request_id,
                name = setup.id
            )));
            let action = Command {
                version: 1,
                request_id: child,
                session_id: id.clone(),
                action: Action::Prompt {
                    comments: None,
                    text: command.text.clone(),
                    max_duration_ms: command.max_duration_ms,
                },
            };
            let preparation = if let Some(submission) = deferred {
                self.queue_file_prompt(&action, command.request_id, home, &profile, submission)
            } else {
                self.prepare_chat_prompt(&action, comment_context.as_ref())
                    .await
            };
            match preparation {
                Ok(frame) => prepared.push((action, frame)),
                Err(error) => {
                    for (action, _) in &prepared {
                        // These frames have not left the controller. Release
                        // their reserved bootstrap/reply context for the next
                        // explicit Send, using the existing no-dispatch fence.
                        self.update(|s| delivery::observe(s, &action.session_id, &json!({"type":"error", "requestId":action.request_id.to_string(), "code":"dispatch_not_sent"})))?;
                        self.finish(
                            action.request_id,
                            "failed",
                            Some("chat_context_preparation_failed_no_prompt"),
                        )?;
                    }
                    return Err(error);
                }
            }
        }
        self.finish(command.request_id, "prepared", None)?;
        let mut agents = Vec::new();
        for (action, frame) in prepared {
            if self.send(&action.session_id, frame).is_err() {
                self.update(|s| delivery::observe(s, &action.session_id, &json!({"type":"error", "requestId":action.request_id.to_string(), "code":"dispatch_not_sent"})))?;
                self.finish(action.request_id, "failed", Some("dispatch_not_sent"))?;
            }
            agents.push(self.read(|s| Ok(s.intents[&action.request_id].outcome.clone()))?);
        }
        self.finish(command.request_id, "accepted", None)?;
        Ok(dto::SendOutcome {
            comment_publication: self.comment_publication(command.request_id)?,
            request_id: command.request_id,
            conversation_id: command.conversation_id,
            agents,
        })
    }
    async fn conversation_target(&self, a: &dto::Attachment) -> Result<Target> {
        let mut targets = self.selections(Some(a.repository)).await?;
        if targets.is_empty() {
            let known = self
                .0
                .resources
                .registry
                .snapshot(None)
                .await
                .map_err(|_| Error::Unavailable)?
                .repositories
                .into_iter()
                .find(|r| r.id == a.repository)
                .ok_or(Error::Stale)?;
            self.authorized(a.repository, "native", true).await?;
            let socket = self
                .0
                .resources
                .native_socket
                .clone()
                .ok_or(Error::Unavailable)?;
            let root = self.0.resources.root.join("sources");
            journal::private_directory(&root)?;
            let source = self
                .0
                .resources
                .driver
                .materialize(a.repository, root.join(a.repository.to_string()), socket)
                .await?;
            if !self.remote_mode() {
                let proof = self
                    .0
                    .resources
                    .verifier
                    .as_ref()
                    .ok_or(Error::Unavailable)?
                    .inspect(source.path.to_str().ok_or(Error::Invalid)?)
                    .map_err(|_| Error::Stale)?;
                if proof.repository != a.repository
                    || proof.profile != jj_store_service::local_management::WorkspaceProfile::Native
                {
                    return Err(Error::Stale);
                }
            } else {
                self.0.resources.driver.verify(&source).await?;
            }
            targets.push(Target {
                id: format!("managed-{}", a.repository),
                repository: a.repository,
                name: known.name,
                folder: source.path.to_string_lossy().into_owned(),
                profile: "native".into(),
                operation: source.operation,
                commit: source.commit,
            });
        }
        if a.target_id.is_none() && targets.len() != 1 {
            return Err(Error::Invalid);
        }
        let mut target = targets
            .into_iter()
            .find(|t| a.target_id.as_ref().is_none_or(|id| *id == t.id))
            .ok_or(Error::Stale)?;
        if a.operation.is_some_and(|op| op != target.operation) {
            // A repository snapshot is independent of the source checkout's
            // recorded operation. Named workspace selections retain their
            // current-operation fence; repository-only pins require both IDs.
            if a.target_id.is_some() || a.commit.is_none() {
                return Err(Error::Stale);
            }
            target.operation = a.operation.expect("selected operation");
        }
        if let Some(commit) = &a.commit {
            if !valid_commit(commit) {
                return Err(Error::Invalid);
            }
            target.commit = commit.clone();
        }
        Ok(target)
    }
    async fn prepare_bindings(
        &self,
        id: &str,
        targets: &[Target],
        origin: &dto::Origin,
    ) -> Result<()> {
        self.initialize_section_tracking(id).await?;
        for target in targets {
            let saved = self.saved(id)?;
            if saved.code_workspaces.contains_key(&target.repository) {
                continue;
            }
            self.authorized(target.repository, &target.profile, true)
                .await?;
            let source = self
                .0
                .resources
                .driver
                .inspect(PathBuf::from(&target.folder))
                .await?;
            // Creation loads the immutable selected operation and revision;
            // inspecting the source proves metadata identity, not freshness.
            let parent = saved.workspace.path.join("repositories");
            journal::private_directory(&parent)?;
            let path = parent.join(target.repository.to_string());
            let name = format!(
                "agent-{}",
                hex::encode(publication::identity(&format!(
                    "binding/{id}/{}",
                    target.repository
                )))
            );
            let workspace = self
                .0
                .resources
                .driver
                .create(source, path, target.operation, target.commit.clone(), name)
                .await?;
            self.update(|s| {
                let saved = s.sessions.get_mut(id).ok_or(Error::Invalid)?;
                saved.view.repository_bindings.push(dto::Binding {
                    repository: target.repository,
                    target_id: target.id.clone(),
                    profile: target.profile.clone(),
                    workspace: workspace.path.to_string_lossy().into_owned(),
                    workspace_name: None,
                    selected_operation: target.operation,
                    base_commit: target.commit.clone(),
                    operation: workspace.operation,
                    commit: workspace.commit.clone(),
                });
                saved.code_workspaces.insert(target.repository, workspace);
                Ok(())
            })?;
        }
        let saved = self.saved(id)?;
        let bindings=saved.view.repository_bindings.iter().map(|b|json!({"repository":b.repository,"directory":format!("repositories/{}",b.repository),"operation":b.operation,"commit":b.commit})).collect::<Vec<_>>();
        let sanitizer = runtime::sanitizer(self, &saved.workspace.path).await?;
        let context = runtime::safe(&sanitizer, json!({"origin":origin,"repositories":bindings}))?
            .to_string();
        self.update(|s| {
            let saved = s.sessions.get_mut(id).ok_or(Error::Invalid)?;
            if saved.context.as_deref() != Some(&context) {
                saved.context = Some(context);
            }
            Ok(())
        })?;
        Ok(())
    }
}
