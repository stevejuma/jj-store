//! One warm harness per session; bounded frames and process group ownership.
use super::*;
use jj_store_capture::transcript_sanitization::TranscriptSanitizer;
use std::process::Stdio;
use tokio::{
    io::{AsyncBufReadExt as _, AsyncWriteExt as _, BufReader},
    process::{Child, Command},
};
pub(super) async fn sanitizer(this: &Supervisor, workspace: &Path) -> Result<TranscriptSanitizer> {
    let ready = this.0.sanitizer.lock().map_err(|_| Error::Unavailable)?;
    ready
        .as_ref()
        .ok_or(Error::Busy)?
        .for_workspace(workspace)
        .map_err(|_| Error::Unavailable)
}
pub(super) fn safe(sanitizer: &TranscriptSanitizer, value: Value) -> Result<Value> {
    let payload = sanitizer.sanitize(value).ok_or(Error::Unavailable)?;
    serde_json::from_slice(payload.bytes()).map_err(|_| Error::Unavailable)
}
struct OwnedChild(Child);
impl OwnedChild {
    async fn stop(&mut self) {
        if let Some(pid) = self
            .0
            .id()
            .and_then(|p| rustix::process::Pid::from_raw(p as i32))
        {
            let _ = rustix::process::kill_process_group(pid, rustix::process::Signal::TERM);
            if tokio::time::timeout(Duration::from_secs(2), self.0.wait())
                .await
                .is_err()
            {
                let _ = rustix::process::kill_process_group(pid, rustix::process::Signal::KILL);
                let _ = self.0.start_kill();
                let _ = tokio::time::timeout(Duration::from_secs(2), self.0.wait()).await;
            }
        }
    }
}
impl Drop for OwnedChild {
    fn drop(&mut self) {
        if let Some(pid) = self
            .0
            .id()
            .and_then(|p| rustix::process::Pid::from_raw(p as i32))
        {
            let _ = rustix::process::kill_process_group(pid, rustix::process::Signal::KILL);
        }
        let _ = self.0.start_kill();
    }
}
impl Supervisor {
    pub(super) async fn spawn(&self, id: &str, load: Option<String>) -> Result<()> {
        if self.tooling_busy()? {
            return Err(Error::Busy);
        }
        let paths = std::env::join_paths(self.tooling_paths()?).map_err(|_| Error::Unavailable)?;
        let saved = self.saved(id)?;
        let sanitizer = sanitizer(self, &saved.workspace.path).await?;
        let mut runtimes = self.0.runtimes.lock().map_err(|_| Error::Unavailable)?;
        if !runtimes.is_empty() {
            return Err(Error::Busy);
        }
        let mut command = Command::new(&self.0.resources.executable);
        command
            .env("PATH", paths)
            .args([
                "serve",
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
                "--control-version",
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
        let mut child = OwnedChild(command.spawn().map_err(|_| Error::Unavailable)?);
        let stdin = child.0.stdin.take().ok_or(Error::Unavailable)?;
        let stdout = child.0.stdout.take().ok_or(Error::Unavailable)?;
        let (tx, rx) = mpsc::channel(16);
        runtimes.insert(id.into(), tx);
        let this = self.clone();
        let id = id.to_owned();
        let task_id = id.clone();
        let task = tokio::spawn(async move {
            let result = this
                .run(&id, &mut child, stdin, stdout, rx, load, sanitizer)
                .await;
            child.stop().await;
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
        child: &mut OwnedChild,
        mut stdin: tokio::process::ChildStdin,
        stdout: tokio::process::ChildStdout,
        mut commands: mpsc::Receiver<Value>,
        load: Option<String>,
        sanitizer: TranscriptSanitizer,
    ) -> Result<()> {
        let mut reader = BufReader::new(stdout);
        let mut bytes = Vec::new();
        let mut assistant = BTreeMap::<String, String>::new();
        let mut tools = BTreeMap::<String, Value>::new();
        let startup = tokio::time::sleep(Duration::from_secs(90));
        tokio::pin!(startup);
        let mut opened = false;
        let mut closing = false;
        let mut closing_request = None;
        let close_deadline = tokio::time::sleep(Duration::from_secs(3600));
        tokio::pin!(close_deadline);
        loop {
            tokio::select! {
                _ = &mut startup, if !opened => return Err(Error::Unavailable),
                _ = &mut close_deadline, if closing => return Err(Error::Unavailable),
                frame = commands.recv(), if !closing => {
                    let Some(frame) = frame else { return Err(Error::Unavailable); };
                    closing = frame["type"] == "close";
                    if closing { closing_request = frame["requestId"].as_str().and_then(|id| id.parse::<RequestId>().ok()); }
                    if closing { close_deadline.as_mut().reset(tokio::time::Instant::now() + Duration::from_secs(15)); }
                    write(&mut stdin, frame).await?;
                },
                line = bounded_line(&mut reader, &mut bytes) => {
                    if !line? { break; }
                    let raw: Value = serde_json::from_slice(&bytes).map_err(|_| Error::Unavailable)?;
                    bytes.clear();
                    if raw["version"] != 2 { return Err(Error::Unavailable); }
                    if raw["type"] == "ready" {
                        write(&mut stdin, json!({"version":2,"type":"open","requestId":format!("open-{id}"),"loadSessionId":load})).await?;
                    }
                    let mut value = safe(&sanitizer, raw)?;
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
                    match value["type"].as_str().unwrap_or("") {
                        "opened" => {
                            opened = true;
                            self.update(|s| { let session = &mut s.sessions.get_mut(id).ok_or(Error::Invalid)?.view;
                                session.adapter_session_id = value["sessionId"].as_str().map(str::to_owned);
                                session.capabilities = json!({"initialize":value["capabilities"],"session":value["session"]}); session.state = "idle".into(); session.last_error = None; Ok(()) })?;
                            let p = self.saved(id)?.view.preferences;
                            if p.mode.is_some() { write(&mut stdin, json!({"version":2,"type":"configure","requestId":format!("configure-{id}"),"model":p.model,"effort":p.effort,"mode":p.mode})).await?; }
                        },
                        "started" => { self.session_state(id, "running", None)?; },
                        "record" => {
                            let usable = value["sessionUsable"].as_bool() == Some(true);
                            let state = value["record"]["status"].as_str().unwrap_or("outcome_unknown");
                            if let Ok(request) = request.parse() { self.finish(request, state, value["record"]["error"].as_str())?; }
                            if value["quiescent"] == true {
                                let saved = self.saved(id)?;
                                match self.0.resources.driver.snapshot(&saved.workspace).await {
                                    Ok(workspace) => { self.update(|s| { let saved = s.sessions.get_mut(id).ok_or(Error::Invalid)?; saved.workspace = workspace.clone(); saved.view.operation = workspace.operation; saved.view.review_commit = workspace.commit.clone(); Ok(()) })?; },
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
                            self.session_state(id, "ended", None)?; break;
                        }, _ => {},
                    }
                },
            }
        }
        tokio::time::timeout(Duration::from_secs(10), child.0.wait())
            .await
            .map_err(|_| Error::Unavailable)?
            .map_err(|_| Error::Unavailable)?;
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
    fn permission_kind(&self, id: &str, permission: &Value, option: &Value) -> Result<Value> {
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
async fn write(stdin: &mut tokio::process::ChildStdin, value: Value) -> Result<()> {
    let mut bytes = serde_json::to_vec(&value).map_err(|_| Error::Invalid)?;
    if bytes.len() > 1024 * 1024 {
        return Err(Error::Limit);
    }
    bytes.push(b'\n');
    tokio::time::timeout(Duration::from_secs(5), stdin.write_all(&bytes))
        .await
        .map_err(|_| Error::Unavailable)?
        .map_err(|_| Error::Unavailable)
}
async fn bounded_line(
    reader: &mut BufReader<tokio::process::ChildStdout>,
    bytes: &mut Vec<u8>,
) -> Result<bool> {
    loop {
        let buffer = reader.fill_buf().await.map_err(|_| Error::Unavailable)?;
        if buffer.is_empty() {
            return if bytes.is_empty() {
                Ok(false)
            } else {
                Err(Error::Unavailable)
            };
        }
        let n = buffer
            .iter()
            .position(|b| *b == b'\n')
            .map_or(buffer.len(), |i| i + 1);
        if bytes.len() + n > 1024 * 1024 {
            return Err(Error::Limit);
        }
        let complete = buffer[n - 1] == b'\n';
        bytes.extend_from_slice(&buffer[..n]);
        reader.consume(n);
        if complete {
            return Ok(true);
        }
    }
}
