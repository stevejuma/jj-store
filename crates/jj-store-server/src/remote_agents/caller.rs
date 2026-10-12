use super::*;
use jj_store_model::RequestId;
use jj_store_service::directory;
pub(super) struct Caller {
    pub broker: Broker,
    pub principal: PrincipalId,
    pub token: BearerToken,
}
impl Caller {
    async fn check(&self, actor: PrincipalId) -> Result<()> {
        if actor != self.principal
            || self
                .broker
                .0
                .stopping
                .load(std::sync::atomic::Ordering::Acquire)
            || self
                .broker
                .0
                .directory
                .principal(&directory::Caller::Token(self.token.clone()))
                .await
                .map_err(|_| Error::Denied)?
                != actor
        {
            return Err(Error::Denied);
        }
        Ok(())
    }
    fn scopes(&self) -> Result<Vec<Scope>> {
        Ok(self
            .broker
            .0
            .scopes
            .lock()
            .map_err(|_| Error::Unavailable)?
            .values()
            .filter(|s| s.principal == self.principal)
            .cloned()
            .collect())
    }
    async fn session(&self, actor: PrincipalId, id: &str) -> Result<Arc<Participation>> {
        self.check(actor).await?;
        if id.parse::<RequestId>().is_err() {
            return Err(Error::Invalid);
        }
        for scope in self.scopes()? {
            let owner = self
                .broker
                .owner(scope.clone(), self.token.clone(), false)
                .await?;
            if let Some(session) = owner
                .supervisor
                .status(actor)
                .await?
                .sessions
                .iter()
                .find(|s| s.id == id)
            {
                if session.setup_id.as_ref().is_none_or(|setup| {
                    !scope.agents.contains(setup)
                        || !self
                            .broker
                            .configuration()
                            .is_ok_and(|c| c.allows(setup, actor))
                }) {
                    return Err(Error::Denied);
                }
                return Ok(owner);
            }
        }
        Err(Error::Denied)
    }
}
#[async_trait::async_trait]
impl Manager for Caller {
    async fn status(&self, actor: PrincipalId) -> Result<Status> {
        self.check(actor).await?;
        let mut setups = self.broker.configuration()?.setups(actor);
        for setup in &mut setups {
            setup.instructions.clear();
            setup.options = Default::default();
        }
        let mut status = Status {
            version: 1,
            runtime: "ready".into(),
            revision: 0,
            setups,
            sessions: vec![],
            runners: vec![],
            live_agents: vec![],
            conversations: vec![],
        };
        for scope in self.scopes()? {
            let Ok(owner) = self.broker.owner(scope, self.token.clone(), false).await else {
                continue;
            };
            let value = owner.supervisor.status(actor).await?;
            status.revision = status.revision.saturating_add(value.revision);
            if value.runtime != "ready" {
                status.runtime = value.runtime;
            }
            for mut session in value.sessions {
                if session.setup_id.as_ref().is_none_or(|setup| {
                    !self
                        .broker
                        .configuration()
                        .is_ok_and(|c| c.allows(setup, actor))
                }) {
                    continue;
                }
                session.workspace.clear();
                for binding in &mut session.repository_bindings {
                    binding.workspace.clear();
                }
                status.sessions.push(session);
            }
            status.conversations.extend(value.conversations);
            status
                .live_agents
                .extend(value.live_agents.into_iter().map(|mut a| {
                    a.inference_host = None;
                    a
                }));
        }
        Ok(status)
    }
    async fn send_conversation(
        &self,
        actor: PrincipalId,
        command: conversations::Send,
    ) -> Result<conversations::SendOutcome> {
        self.check(actor).await?;
        let home = command.origin.home.ok_or(Error::Denied)?;
        if command.version != 1
            || command.conversation_id.parse::<RequestId>().is_err()
            || command.agent_ids.is_empty()
            || command.agent_ids.len() > 4
            || command.attachments.len() > 8
            || command.agent_ids.iter().any(|id| {
                !self
                    .broker
                    .configuration()
                    .is_ok_and(|c| c.allows(id, actor))
            })
            || command.attachments.iter().any(|a| a.target_id.is_some())
        {
            return Err(Error::Denied);
        }
        let mut repositories: BTreeSet<_> =
            command.attachments.iter().map(|a| a.repository).collect();
        repositories.insert(home);
        let scope = Scope {
            principal: actor,
            conversation: command.conversation_id.clone(),
            home,
            repositories,
            agents: command.agent_ids.iter().cloned().collect(),
        };
        self.broker.check_scope(&scope, &self.token).await?;
        let id = key(actor, &scope.conversation);
        let _dispatch = self.broker.0.dispatch.lock().await;
        // Recovery is checked against the old authority before any scope replacement.
        if let Some(previous) = self
            .scopes()?
            .into_iter()
            .find(|s| s.conversation == scope.conversation)
        {
            if previous.home != home {
                return Err(Error::Stale);
            }
            let owner = self
                .broker
                .owner(previous, self.token.clone(), false)
                .await?;
            owner.supervisor.remote_origin_matches(&command)?;
            if owner
                .supervisor
                .remote_request_known(command.request_id, &command)?
            {
                return owner.supervisor.send_conversation(actor, command).await;
            }
        }
        self.broker.queue_budget(1 + command.agent_ids.len())?;
        {
            let mut scopes = self
                .broker
                .0
                .scopes
                .lock()
                .map_err(|_| Error::Unavailable)?;
            if !scopes.contains_key(&id) {
                if scopes.len() >= 64 {
                    return Err(Error::Limit);
                }
                if scopes.values().any(|s| {
                    s.home == home && s.conversation == scope.conversation && s.principal != actor
                }) {
                    return Err(Error::Denied);
                }
                let mut saved = scopes.clone();
                saved.insert(id.clone(), scope.clone());
                self.broker.save(&saved)?;
                *scopes = saved;
            }
        }
        let owner = self
            .broker
            .owner(scope.clone(), self.token.clone(), true)
            .await?;
        if !owner.supervisor.ready() {
            return Err(Error::Busy);
        }
        {
            let mut scopes = self
                .broker
                .0
                .scopes
                .lock()
                .map_err(|_| Error::Unavailable)?;
            if scopes.get(&id) != Some(&scope) {
                let mut saved = scopes.clone();
                saved.insert(id, scope);
                self.broker.save(&saved)?;
                *scopes = saved;
            }
        }
        owner.supervisor.send_conversation(actor, command).await
    }
    async fn command(&self, actor: PrincipalId, command: Command) -> Result<Outcome> {
        let _dispatch = self.broker.0.dispatch.lock().await;
        let owner = self.session(actor, &command.session_id).await?;
        if matches!(command.action, Action::Configure { .. }) {
            return Err(Error::Denied);
        }
        if matches!(command.action, Action::Prompt { .. })
            && !owner
                .supervisor
                .remote_request_known(command.request_id, &command)?
        {
            self.broker.queue_budget(1)?;
        }
        owner.supervisor.command(actor, command).await
    }
    async fn outcome(&self, actor: PrincipalId, request: RequestId) -> Result<Outcome> {
        self.check(actor).await?;
        for scope in self.scopes()? {
            let owner = self
                .broker
                .owner(scope.clone(), self.token.clone(), false)
                .await?;
            match owner.supervisor.outcome(actor, request).await {
                Ok(outcome) if outcome.state != "not_known" => return Ok(outcome),
                Ok(_) | Err(Error::Invalid) => {}
                Err(error) => return Err(error),
            }
        }
        Ok(Outcome {
            version: 1,
            request_id: request,
            session_id: None,
            state: "not_known".into(),
            error: None,
            comment_publication: None,
        })
    }
    async fn events(&self, actor: PrincipalId, session: String, after: u64) -> Result<EventPage> {
        self.session(actor, &session)
            .await?
            .supervisor
            .events(actor, session, after)
            .await
    }
    async fn inspect_workspace(
        &self,
        actor: PrincipalId,
        request: workspace_inspection::Request,
    ) -> Result<workspace_inspection::Inspection> {
        let mut value = self
            .session(actor, &request.session)
            .await?
            .supervisor
            .inspect_workspace(actor, request)
            .await?;
        value.location.clear();
        value.workspace.clear();
        Ok(value)
    }
    async fn feedback(&self, actor: PrincipalId, session: String) -> Result<Feedback> {
        self.session(actor, &session)
            .await?
            .supervisor
            .feedback(actor, session)
            .await
    }
    async fn targets(
        &self,
        actor: PrincipalId,
        _repository: Option<RepositoryId>,
    ) -> Result<Vec<Target>> {
        self.check(actor).await?;
        Ok(vec![])
    }
    async fn launch(&self, _actor: PrincipalId, _command: Launch) -> Result<Outcome> {
        Err(Error::Denied)
    }
    async fn save_setup(&self, _actor: PrincipalId, _command: SetupCommand) -> Result<Status> {
        Err(Error::Denied)
    }
    fn subscribe(&self) -> tokio::sync::broadcast::Receiver<u64> {
        self.broker.0.changed.subscribe()
    }
    async fn shutdown(&self) -> Result<()> {
        Err(Error::Denied)
    }
}
