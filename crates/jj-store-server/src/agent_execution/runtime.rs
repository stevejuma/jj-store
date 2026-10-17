//! One warm harness per session; bounded frames and process group ownership.
use super::*;
use jj_store_capture::transcript_sanitization::TranscriptSanitizer;
use lapis_ai_runtime::{AcpClient, AcpCommands};
use std::process::Stdio;
use tokio::process::Command;
pub(super) async fn sanitizer(this: &Supervisor, workspace: &Path) -> Result<TranscriptSanitizer> {
    let ready = this.0.sanitizer.lock().map_err(|_| Error::Unavailable)?;
    ready
        .as_ref()
        .ok_or(Error::Busy)?
        .for_workspace(workspace)
        .map_err(|_| Error::Unavailable)
}
pub(super) use super::runtime_privacy::safe;
impl Supervisor {
    pub(super) async fn spawn(&self, id: &str, load: Option<String>) -> Result<()> {
        if self.tooling_busy()? {
            return Err(Error::Busy);
        }
        let paths = std::env::join_paths(self.tooling_paths()?).map_err(|_| Error::Unavailable)?;
        let saved = self.saved(id)?;
        let setup = Setup {
            id: saved.view.setup_id.clone().unwrap_or_default(),
            name: saved.view.title.clone(),
            avatar: saved.view.avatar.clone(),
            description: String::new(),
            instructions: saved.instructions.clone(),
            runner: saved.view.runner.clone(),
            preferences: saved.view.preferences.clone(),
            options: saved.options.clone(),
        };
        let mut command = Command::new(&self.0.resources.executable);
        self.launch_environment(&mut command, &setup)?;
        self.local_launch(&mut command, &setup, true).await?;
        let chatgpt_lease = self.chatgpt_launch(&mut command, &setup, id, 0).await?;
        self.register_launch_secrets(&command, &setup)?;
        let sanitizer = sanitizer(self, &saved.workspace.path).await?;

        command
            .env("PATH", paths)
            .args([
                "--max-turn-duration",
                "3600",
                "--idle-timeout",
                "300",
                "--queue-capacity",
                "16",
                "--cancel-grace",
                "2",
                "--reap-timeout",
                "2",
                "--runner",
                &saved.view.runner,
                "--cwd",
            ])
            .arg(&saved.workspace.path)
            .current_dir(&saved.workspace.path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .kill_on_drop(true);
        #[cfg(unix)]
        command.process_group(0);
        let mut child = native_runtime::start(&command).await?;
        let stdin = child.commands();
        let mut runtimes = self.0.runtimes.lock().map_err(|_| Error::Unavailable)?;
        if !runtimes.is_empty()
            || self
                .0
                .catalog_active
                .load(std::sync::atomic::Ordering::Acquire)
        {
            return Err(Error::Busy);
        }
        let (tx, rx) = mpsc::channel(16);
        runtimes.insert(id.into(), tx);
        let this = self.clone();
        let id = id.to_owned();
        let task_id = id.clone();
        let task = tokio::spawn(async move {
            let mut chatgpt_lease = chatgpt_lease;
            let result = tokio::select! {
                result=this.run(&id, &mut child, stdin, rx, load, sanitizer)=>result,
                _=chatgpt::broker::cancelled(&mut chatgpt_lease)=>Err(Error::Denied),
            };
            let _ = child.shutdown().await;
            if let Ok(mut map) = this.0.runtimes.lock() {
                map.remove(&id);
            }
            if let Ok(mut map) = this.0.runtime_tasks.lock() {
                map.remove(&id);
            }
            let _ = this.update(|s| {
                let saved = s.sessions.get_mut(&id).ok_or(Error::Invalid)?;
                if !matches!(saved.view.state.as_str(), "ended" | "cleaned") {
                    saved.view.state = "interrupted".into();
                    saved.view.last_error = Some(
                        if result.is_err() {
                            "process_outcome_unknown"
                        } else {
                            "process_stopped"
                        }
                        .into(),
                    );
                }
                for intent in s
                    .intents
                    .values_mut()
                    .filter(|i| i.outcome.session_id.as_deref() == Some(&id))
                {
                    if matches!(
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
            });
        });
        self.0
            .runtime_tasks
            .lock()
            .map_err(|_| Error::Unavailable)?
            .insert(task_id, task.abort_handle());
        Ok(())
    }
    #[allow(clippy::too_many_arguments)]
    async fn run(
        &self,
        id: &str,
        child: &mut AcpClient,
        stdin: AcpCommands,
        mut commands: mpsc::Receiver<Value>,
        load: Option<String>,
        sanitizer: TranscriptSanitizer,
    ) -> Result<()> {
        native_runtime::send(
            &stdin,
            json!({"version":3,"type":"warm","requestId":format!("warm-{id}")}),
        )
        .await?;
        let mut assistant = BTreeMap::<String, String>::new();
        let mut privacy = super::runtime_privacy::StreamPrivacy::default();
        let mut tools = BTreeMap::<String, Value>::new();
        let startup = tokio::time::sleep(Duration::from_secs(90));
        tokio::pin!(startup);
        let mut opened = false;
        let mut closing = false;
        let mut closing_request = None;
        let close_deadline = tokio::time::sleep(Duration::from_secs(3600));
        tokio::pin!(close_deadline);
        let mut prompts = super::workspace_turns::PromptQueue::default();
        let mut workspace_lease = None;
        let mut retry_at = tokio::time::Instant::now();
        'runtime: loop {
            if !prompts.is_active() {
                workspace_lease.take();
            }
            if !closing
                && tokio::time::Instant::now() >= retry_at
                && let Some(mut frame) = prompts.next()
            {
                let result = self.workspace_dispatch(frame.clone()).await;
                if matches!(result, Err(Error::Busy)) {
                    prompts.defer(frame);
                    retry_at = tokio::time::Instant::now() + Duration::from_millis(100);
                } else if let Ok((prepared, lease)) = result {
                    workspace_lease = Some(lease);
                    frame = prepared;
                    frame["version"] = json!(3);
                    frame["sessionId"] = json!(id);
                    if !matches!(frame["type"].as_str(), Some("close" | "status")) {
                        frame["sessionId"] = json!(id);
                    }
                    native_runtime::send(&stdin, frame).await?;
                } else {
                    let request = frame["requestId"].as_str().ok_or(Error::Invalid)?;
                    let failure = json!({"type":"error","sessionId":id,"requestId":request,"code":"dispatch_not_sent"});
                    self.update(|s| delivery::observe(s, id, &failure))?;
                    self.finish(
                        request.parse().map_err(|_| Error::Invalid)?,
                        "failed",
                        Some("workspace_start_snapshot_unavailable"),
                    )?;
                    prompts.completed(&failure);
                    continue;
                }
            }
            tokio::select! {
                _ = tokio::time::sleep_until(retry_at), if !closing && !prompts.is_active() && prompts.has_pending() => {},
                _ = &mut startup, if !opened => return Err(Error::Unavailable),
                _ = &mut close_deadline, if closing => return Err(Error::Unavailable),
                frame = commands.recv(), if !closing => {
                    let Some(mut frame) = frame else { return Err(Error::Unavailable); };
                    if frame["type"] == "prompt" {
                        let request = frame["requestId"].as_str().ok_or(Error::Invalid)?.parse().map_err(|_| Error::Invalid)?;
                        frame["sessionId"] = json!(id);
                        if prompts.push(frame).is_ok() { self.finish(request, "queued", None)?; }
                        else {
                            self.update(|s| delivery::observe(s,id,&json!({"type":"error","requestId":request,"code":"queue_full"})))?;
                            self.finish(request, "failed", Some("queue_full"))?;
                        }
                        continue;
                    }
                    if frame["type"] == "cancel" && let Some(target) = frame["target"].as_str()
                        && prompts.cancel(id,target).is_some() {
                        self.update(|s| delivery::observe(s,id,&json!({"type":"error","requestId":target,"code":"dispatch_not_sent"})))?;
                        self.finish(target.parse().map_err(|_| Error::Invalid)?, "cancelled", None)?;
                        if let Some(request)=frame["requestId"].as_str().and_then(|s|s.parse().ok()) { self.finish(request,"accepted",None)?; }
                        continue;
                    }
                    closing = frame["type"] == "close";
                    if closing { closing_request = frame["requestId"].as_str().and_then(|id| id.parse::<RequestId>().ok()); }
                    if closing { close_deadline.as_mut().reset(tokio::time::Instant::now() + Duration::from_secs(15)); }
                    if !matches!(frame["type"].as_str(), Some("close" | "status")) { frame["sessionId"] = json!(id); }
                    native_runtime::send(&stdin, frame).await?;
                },
                frame = child.next(Duration::from_secs(86400)) => {
                    let raw = frame.map_err(|_| Error::Unavailable)?;
                    if raw["version"] != 3 { return Err(Error::Unavailable); }
                    if raw["type"] == "warmed" {
                        native_runtime::send(&stdin, json!({"version":3,"type":"open","requestId":format!("open-{id}"),"sessionId":id,"cwd":self.saved(id)?.workspace.path,"loadSessionId":load})).await?;
                        continue;
                    }
                    if !matches!(raw["type"].as_str(), Some("closed" | "error" | "status")) && raw["sessionId"] != id { return Err(Error::Unavailable); }
                    let fresh=sanitizer.for_oauth_secrets(&self.oauth_redactions()?).map_err(|_|Error::Unavailable)?;
                    for mut value in privacy.frames(&fresh, raw)? {
                    let request = value["requestId"].as_str().unwrap_or("").to_owned();
                    if value["type"] == "activity" {
                        let update = &value["update"];
                        if update["sessionUpdate"] == "agent_message_chunk" && let Some(text) = update["content"]["text"].as_str() {
                            let target = assistant.entry(request.clone()).or_default();
                            if target.len() + text.len() > 512 * 1024 { return Err(Error::Limit); }
                            target.push_str(text);
                        }
                        if let Some(call) = update["toolCallId"].as_str() {
                            if tools.len() >= 256 && !tools.contains_key(&format!("{request}/{call}")) { return Err(Error::Limit); }
                            let target = tools.entry(format!("{request}/{call}")).or_insert_with(|| json!({}));
                            if let Some(fields) = update.as_object() { for (key, field) in fields { target[key] = field.clone(); } }
                            value["toolSnapshot"] = target.clone();
                        }
                    }
                    if value["type"] == "record" {
                        value["assistantText"] = json!(assistant.remove(&request).unwrap_or_default());
                        tools.retain(|key, _| !key.starts_with(&format!("{request}/")));
                    }
                    if value["type"] == "permission_resolved" {
                        value["decisionKind"] = self.permission_kind(id, &value["permissionId"], &value["optionId"])?;
                    }
                    self.append(id, value.clone())?;
                    let mut completion = value.clone();
                    completion["sessionId"] = json!(id);
                    prompts.completed(&completion);
                    match value["type"].as_str().unwrap_or("") {
                        "opened" => {
                            opened = true;
                            self.update(|s| { let session = &mut s.sessions.get_mut(id).ok_or(Error::Invalid)?.view;
                                session.adapter_session_id = value["adapterSessionId"].as_str().map(str::to_owned);
                                session.capabilities = json!({"initialize":value["capabilities"],"session":value["session"]}); session.state = "idle".into(); session.last_error = None; Ok(()) })?;
                            let p = self.saved(id)?.view.preferences;
                            if p.mode.is_some() { native_runtime::send(&stdin, json!({"version":3,"sessionId":id,"type":"configure","requestId":format!("configure-{id}"),"model":p.model,"effort":p.effort,"mode":p.mode})).await?; }
                        },
                        "started" => { self.session_state(id, "running", None)?; },
                        "record" => {
                            let usable = value["sessionUsable"].as_bool() == Some(true);
                            let state = value["record"]["status"].as_str().unwrap_or("outcome_unknown");
                            if let Ok(request) = request.parse() { self.finish(request, state, value["record"]["error"].as_str())?; }
                            if value["quiescent"] == true {
                                let saved = self.saved(id)?;
                                match self.0.resources.driver.snapshot(&saved.workspace).await {
                                    Ok(workspace) => { self.append(id, json!({"type":"repository_snapshot","requestId":request,"repository":saved.view.repository,"state":"completed","operation":workspace.operation,"commit":workspace.commit}))?; self.update(|s| { let saved = s.sessions.get_mut(id).ok_or(Error::Invalid)?; saved.workspace = workspace.clone(); saved.view.operation = workspace.operation; saved.view.review_commit = workspace.commit.clone(); Ok(()) })?; },
                                    Err(_) => { self.session_state(id, "interrupted", Some("workspace_snapshot_unknown"))?; return Err(Error::Unavailable); }
                                }
                            }
                            self.session_state(id, if usable { "idle" } else { "interrupted" }, if usable { None } else { Some("session_unusable") })?;
                        },
                        "configured" | "ack" => {
                            if value["type"] == "configured" { self.update(|s| { let view = &mut s.sessions.get_mut(id).ok_or(Error::Invalid)?.view;
                                view.capabilities["configuration"] = value["configuration"].clone(); view.capabilities["modeReady"] = value["modeReady"].clone(); Ok(()) })?; }
                            if let Ok(request) = request.parse() { self.finish(request, "accepted", None)?; }
                        },
                        "queued" => { if let Ok(request) = request.parse() { self.finish(request, "queued", None)?; } },
                        "error" => {
                            if let Ok(request) = request.parse() { self.finish(request, "failed", value["code"].as_str())?; }
                            // Open and configure protocol failures discard the ACP
                            // session. Only explicit admission errors leave it usable.
                            let usable = opened && matches!(value["code"].as_str(), Some("execution_mode_required" | "queue_full" | "duplicate_request" | "not_in_flight" | "stale_or_invalid_permission" | "request_limit"));
                            let state = if usable { self.saved(id)?.view.state } else { "interrupted".into() };
                            self.session_state(id, &state, value["code"].as_str())?;
                            if !usable { return Err(Error::Unavailable); }
                        },
                        "closed" => {
                            if value["confirmedStopped"] != true { return Err(Error::Unavailable); }
                            if let Some(request) = closing_request { self.finish(request, "ended", None)?; }
                            self.session_state(id, "ended", None)?; break 'runtime;
                        }, _ => {},
                    }
                    }
                },
            }
        }
        child.shutdown().await.map_err(|_| Error::Unavailable)?;
        Ok(())
    }
    pub(super) fn session_state(&self, id: &str, state: &str, error: Option<&str>) -> Result<()> {
        self.update(|s| {
            let view = &mut s.sessions.get_mut(id).ok_or(Error::Invalid)?.view;
            view.state = state.into();
            view.last_error = error.map(str::to_owned);
            Ok(())
        })
    }
    pub(super) fn permission_kind(
        &self,
        id: &str,
        permission: &Value,
        option: &Value,
    ) -> Result<Value> {
        Ok(self
            .saved(id)?
            .events
            .iter()
            .rev()
            .find(|e| e.payload["type"] == "permission" && e.payload["permissionId"] == *permission)
            .and_then(|e| e.payload["request"]["options"].as_array())
            .and_then(|options| options.iter().find(|o| o["optionId"] == *option))
            .map(|o| o["kind"].clone())
            .unwrap_or(Value::Null))
    }
}
