//! Generation-fenced transcript delivery and independent repository snapshots.
use super::*;
#[derive(Default)]
pub(super) struct Stream {
    assistant: BTreeMap<String, String>,
    tools: BTreeMap<String, Value>,
}
impl Supervisor {
    pub(super) async fn open_participation(&self, setup: &Setup, id: &str) -> Result<()> {
        if self.saved(id)?.view.state == "running" {
            return Ok(());
        }
        self.start_worker(&setup.id).await?;
        tokio::time::timeout(Duration::from_secs(90), async {
            loop {
                let state = self
                    .0
                    .fleet
                    .lock()
                    .map_err(|_| Error::Unavailable)?
                    .get(&setup.id)
                    .map(|w| w.state.clone())
                    .ok_or(Error::Unavailable)?;
                if matches!(state.as_str(), "idle" | "working" | "queued" | "attention") {
                    return Ok(());
                }
                if matches!(state.as_str(), "error" | "offline" | "stopping") {
                    return Err(Error::Unavailable);
                }
                tokio::time::sleep(Duration::from_millis(25)).await;
            }
        })
        .await
        .map_err(|_| Error::Unavailable)??;
        let saved = self.saved(id)?;
        let generation = self
            .0
            .fleet
            .lock()
            .map_err(|_| Error::Unavailable)?
            .get(&setup.id)
            .ok_or(Error::Unavailable)?
            .generation;
        if saved.view.state == "idle"
            && saved.view.capabilities["workerGeneration"] == generation
            && saved.view.capabilities["modeReady"] == true
        {
            return Ok(());
        }
        let load = if saved.view.capabilities["initialize"]["loadSession"] == true {
            saved.view.adapter_session_id.clone()
        } else {
            None
        };
        if load.is_none() && saved.view.adapter_session_id.is_some() {
            self.update(|s| delivery::continuation(s, id))?;
            self.append(
                id,
                json!({"type":"continuation","label":"New adapter session with saved context"}),
            )?;
        }
        let mut random = [0; 16];
        getrandom::fill(&mut random).map_err(|_| Error::Unavailable)?;
        let opening = hex::encode(random);
        self.update(|s| {
            let view = &mut s.sessions.get_mut(id).ok_or(Error::Invalid)?.view;
            view.state = "opening".into();
            view.capabilities["workerGeneration"] = json!(generation);
            view.capabilities["modeReady"] = json!(false);
            view.capabilities["openingRequest"] = json!(format!("open-{id}-{opening}"));
            view.last_error = None;
            Ok(())
        })?;
        self.fleet_send(&setup.id,json!({"version":3,"type":"open","requestId":format!("open-{id}-{opening}"),"sessionId":id,"cwd":saved.workspace.path,"loadSessionId":load}))?;
        tokio::time::timeout(Duration::from_secs(90), async {
            loop {
                let saved = self.saved(id)?;
                if saved.view.state == "idle" && saved.view.capabilities["modeReady"] == true {
                    return Ok(());
                }
                if saved.view.state == "interrupted" || saved.view.last_error.is_some() {
                    return Err(Error::Unavailable);
                }
                tokio::time::sleep(Duration::from_millis(25)).await;
            }
        })
        .await
        .map_err(|_| Error::Unavailable)?
    }
    pub(super) fn fleet_send(&self, agent: &str, frame: Value) -> Result<()> {
        let sender = self
            .0
            .fleet
            .lock()
            .map_err(|_| Error::Unavailable)?
            .get(agent)
            .map(|w| w.sender.clone())
            .ok_or(Error::Unavailable)?;
        sender.try_send(frame).map_err(|_| Error::Busy)
    }
    pub(super) async fn conversation_frame(
        &self,
        agent: &str,
        generation: u64,
        raw: Value,
        stream: &mut Stream,
    ) -> Result<()> {
        let id = raw["sessionId"].as_str().ok_or(Error::Invalid)?.to_owned();
        let saved = self.saved(&id)?;
        if saved.view.setup_id.as_deref() != Some(agent)
            || saved.view.conversation_id.is_none()
            || saved.view.capabilities["workerGeneration"] != generation
        {
            return Err(Error::Stale);
        }
        self.authorized(saved.view.repository, &saved.profile, true)
            .await?;
        for binding in &saved.view.repository_bindings {
            self.authorized(binding.repository, &binding.profile, true)
                .await?;
        }
        let request = raw["requestId"].as_str().ok_or(Error::Invalid)?;
        let opening = saved.view.capabilities["openingRequest"]
            .as_str()
            .unwrap_or("");
        let handshake = request == opening || request == format!("configure-{opening}");
        if !handshake {
            let request: RequestId = request.parse().map_err(|_| Error::Stale)?;
            self.read(|s| {
                let intent = s.intents.get(&request).ok_or(Error::Stale)?;
                if intent.outcome.session_id.as_deref() != Some(&id)
                    || !matches!(
                        intent.outcome.state.as_str(),
                        "dispatching" | "queued" | "started"
                    )
                {
                    return Err(Error::Stale);
                }
                Ok(())
            })?;
        }
        let sanitizer = runtime::sanitizer(self, &saved.workspace.path).await?;
        let mut value = runtime::safe(&sanitizer, raw)?;
        let request = value["requestId"].as_str().unwrap_or("").to_owned();
        value["version"] = json!(2);
        if value["type"] == "opened" {
            value["sessionId"] = value["adapterSessionId"].clone();
        }
        if handshake && value["type"] == "activity" {
            self.append(&id, value)?;
            return Ok(());
        }
        let key = format!("{id}/{request}");
        if value["type"] == "activity" {
            let update = &value["update"];
            if update["sessionUpdate"] == "agent_message_chunk"
                && let Some(text) = update["content"]["text"].as_str()
            {
                let target = stream.assistant.entry(key.clone()).or_default();
                if target.len() + text.len() > 512 * 1024 {
                    return Err(Error::Limit);
                }
                target.push_str(text);
            }
            if let Some(call) = update["toolCallId"].as_str() {
                let tool = format!("{key}/{call}");
                if stream.tools.len() >= 256 && !stream.tools.contains_key(&tool) {
                    return Err(Error::Limit);
                }
                let target = stream.tools.entry(tool).or_insert_with(|| json!({}));
                if let Some(fields) = update.as_object() {
                    for (k, v) in fields {
                        target[k] = v.clone();
                    }
                }
                value["toolSnapshot"] = target.clone();
            }
        }
        if value["type"] == "record" {
            value["assistantText"] = json!(stream.assistant.remove(&key).unwrap_or_default());
            stream
                .tools
                .retain(|key2, _| !key2.starts_with(&format!("{key}/")));
        }
        if value["type"] == "permission_resolved" {
            value["decisionKind"] =
                self.permission_kind(&id, &value["permissionId"], &value["optionId"])?;
        }
        self.append(&id, value.clone())?;
        match value["type"].as_str() {
            Some("opened") => {
                self.update(|s| {
                    let view = &mut s.sessions.get_mut(&id).ok_or(Error::Invalid)?.view;
                    view.adapter_session_id = value["adapterSessionId"].as_str().map(str::to_owned);
                    view.capabilities["initialize"] = value["capabilities"].clone();
                    view.capabilities["session"] = value["session"].clone();
                    view.state = "opening".into();
                    view.last_error = None;
                    Ok(())
                })?;
                let p = saved.view.preferences;
                self.fleet_send(agent,json!({"version":3,"type":"configure","requestId":format!("configure-{request}"),"sessionId":id,"model":p.model,"effort":p.effort,"mode":p.mode}))?;
            }
            Some("configured") => {
                self.update(|s| {
                    let view = &mut s.sessions.get_mut(&id).ok_or(Error::Invalid)?.view;
                    view.capabilities["configuration"] = value["configuration"].clone();
                    view.capabilities["modeReady"] = value["modeReady"].clone();
                    view.state = if value["modeReady"] == true {
                        "idle"
                    } else {
                        "interrupted"
                    }
                    .into();
                    view.last_error = if value["modeReady"] == true {
                        None
                    } else {
                        Some("execution_mode_required".into())
                    };
                    Ok(())
                })?;
                if let Ok(r) = request.parse() {
                    self.finish(r, "accepted", None)?;
                }
            }
            Some("queued") => {
                if let Ok(r) = request.parse() {
                    self.finish(r, "queued", None)?;
                }
            }
            Some("started") => {
                self.session_state(&id, "running", None)?;
            }
            Some("record") => {
                let usable = value["sessionUsable"] == true;
                let mut snapshot_error = None;
                if value["quiescent"] == true {
                    for (repo, workspace) in saved.code_workspaces {
                        let snapshot = self.0.resources.driver.snapshot(&workspace).await;
                        if snapshot.is_err() {
                            snapshot_error = Some(format!("snapshot_unknown/{repo}"));
                        }
                        self.append(&id, json!({"type":"repository_snapshot","requestId":request,"repository":repo,"state":if snapshot.is_ok(){"completed"}else{"outcome_unknown"},"operation":snapshot.as_ref().ok().map(|w|w.operation),"commit":snapshot.as_ref().ok().map(|w|w.commit.clone())}))?;
                        self.update(|s| {
                            let saved = s.sessions.get_mut(&id).ok_or(Error::Invalid)?;
                            match &snapshot {
                                Ok(workspace) => {
                                    saved.code_workspaces.insert(repo, workspace.clone());
                                    if let Some(b) = saved
                                        .view
                                        .repository_bindings
                                        .iter_mut()
                                        .find(|b| b.repository == repo)
                                    {
                                        b.operation = workspace.operation;
                                        b.commit = workspace.commit.clone();
                                    }
                                }
                                Err(_) => {
                                    saved.view.last_error =
                                        Some(format!("snapshot_unknown/{repo}"));
                                }
                            }
                            Ok(())
                        })?;
                    }
                }
                if let Ok(r) = request.parse() {
                    self.finish(
                        r,
                        value["record"]["status"]
                            .as_str()
                            .unwrap_or("outcome_unknown"),
                        value["record"]["error"].as_str(),
                    )?;
                }
                self.session_state(
                    &id,
                    if usable { "idle" } else { "interrupted" },
                    if usable {
                        snapshot_error.as_deref()
                    } else {
                        Some("session_unusable")
                    },
                )?;
                if !usable {
                    return Err(Error::Unavailable);
                }
            }
            Some("session_closed") => {
                self.session_state(&id, "ended", None)?;
                if let Ok(r) = request.parse() {
                    self.finish(r, "ended", None)?;
                }
            }
            Some("ack") => {
                if let Ok(r) = request.parse() {
                    self.finish(r, "accepted", None)?;
                }
            }
            Some("error") => {
                if let Ok(r) = request.parse() {
                    self.finish(r, "failed", value["code"].as_str())?;
                }
                self.session_state(&id, "interrupted", value["code"].as_str())?;
            }
            _ => {}
        }
        Ok(())
    }
    pub(super) fn interrupt_participations(&self, agent: &str, generation: u64) -> Result<()> {
        self.update(|s| {
            for saved in s.sessions.values_mut().filter(|s| {
                s.view.conversation_id.is_some()
                    && s.view.setup_id.as_deref() == Some(agent)
                    && s.view.capabilities["workerGeneration"] == generation
            }) {
                if saved.view.state != "ended" {
                    saved.view.state = "interrupted".into();
                    saved.view.last_error = Some("process_stopped_no_replay".into());
                }
            }
            for intent in s.intents.values_mut() {
                if intent.outcome.session_id.as_ref().is_some_and(|id| {
                    s.sessions
                        .get(id)
                        .is_some_and(|s| s.view.setup_id.as_deref() == Some(agent))
                }) && matches!(
                    intent.outcome.state.as_str(),
                    "dispatching" | "queued" | "started"
                ) {
                    intent.outcome.state = "outcome_unknown".into();
                    intent.outcome.error = Some("process_stopped_no_replay".into());
                }
            }
            let uncertain: Vec<_> = s
                .intents
                .iter()
                .filter(|(_, i)| i.outcome.state == "outcome_unknown")
                .map(|(request, _)| *request)
                .collect();
            for request in uncertain {
                sent_context::outcome(s, request, "outcome_unknown")?;
            }
            Ok(())
        })
    }
}
