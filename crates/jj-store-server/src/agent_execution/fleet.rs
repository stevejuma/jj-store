//! Warm installation-owned ACP workers; opening a chat is a separate operation.
use super::*;
use jj_store_service::agent_execution::fleet::{Control, LiveAgent};
use std::process::Stdio;
use tokio::process::Command;
pub(super) struct Worker {
    pub configuration: String,
    pub inference_host: Option<String>,
    pub sender: mpsc::Sender<Value>,
    pub state: String,
    pub generation: u64,
    pub error: Option<String>,
    pub last_used: tokio::time::Instant,
    pub stop: tokio::sync::watch::Sender<bool>,
}
impl Worker {
    fn request_stop(&mut self) {
        if !self.sender.is_closed() {
            self.state = "stopping".into();
            // Shutdown has its own signal so a full command queue cannot
            // discard Stop or keep an agent permanently in Stopping.
            self.stop.send_replace(true);
        }
    }
}
#[cfg(test)]
#[path = "fleet_tests.rs"]
mod tests;
impl Supervisor {
    pub(super) fn live_agents(&self) -> Result<Vec<LiveAgent>> {
        let workers = self.0.fleet.lock().map_err(|_| Error::Unavailable)?;
        let legacy = self.0.runtimes.lock().map_err(|_| Error::Unavailable)?;
        self.read(|s| {
            Ok(s.setups
                .keys()
                .map(|id| {
                    let worker = workers.get(id);
                    let live = s
                        .sessions
                        .values()
                        .filter(|saved| {
                            saved.view.setup_id.as_deref() == Some(id.as_str())
                                && (legacy
                                    .get(&saved.view.id)
                                    .is_some_and(|sender| !sender.is_closed())
                                    || (worker.is_some_and(|w| !w.sender.is_closed())
                                        && saved.view.capabilities["workerGeneration"]
                                            == worker.map(|w| w.generation).unwrap_or(0)))
                        })
                        .collect::<Vec<_>>();
                    let attention = live.iter().any(|saved| {
                        saved.view.state == "running"
                            && saved
                                .events
                                .iter()
                                .rev()
                                .find(|e| {
                                    matches!(
                                        e.payload["type"].as_str(),
                                        Some("permission" | "permission_resolved" | "record")
                                    )
                                })
                                .is_some_and(|e| e.payload["type"] == "permission")
                    });
                    let working = live.iter().any(|saved| saved.view.state == "running");
                    let queued = s.intents.values().any(|i| {
                        i.outcome.state == "queued"
                            && live.iter().any(|saved| {
                                i.outcome.session_id.as_deref() == Some(&saved.view.id)
                            })
                    });
                    let starting = live.iter().any(|saved| saved.view.state == "opening");
                    let state = if worker.is_some_and(|w| {
                        matches!(w.state.as_str(), "stopping" | "error" | "starting")
                    }) {
                        worker.unwrap().state.as_str()
                    } else if attention {
                        "attention"
                    } else if working {
                        "working"
                    } else if queued {
                        "queued"
                    } else if starting {
                        "starting"
                    } else if !live.is_empty()
                        || worker.is_some_and(|w| w.state == "idle" && !w.sender.is_closed())
                    {
                        "idle"
                    } else {
                        "offline"
                    };
                    LiveAgent {
                        inference_host: worker.and_then(|w| w.inference_host.clone()).or_else(
                            || {
                                s.setups.get(id).and_then(|setup| {
                                    configuration::inference_host(
                                        &configuration::resolve(setup, &s.settings),
                                        s,
                                    )
                                })
                            },
                        ),
                        agent_id: id.clone(),
                        enabled: s.warm_agents.contains(id),
                        generation: worker.map_or(0, |w| w.generation),
                        state: state.into(),
                        error: worker.and_then(|w| w.error.clone()),
                        restart_required: worker.is_some_and(|w| {
                            !w.sender.is_closed()
                                && s.setups.get(id).is_some_and(|setup| {
                                    configuration::launch_digest(
                                        &configuration::resolve(setup, &s.settings),
                                        s,
                                    )
                                    .is_ok_and(|key| key != w.configuration)
                                })
                        }),
                    }
                })
                .collect())
        })
    }
    pub(super) async fn warm_enabled(&self) -> Result<()> {
        if !self.ready() || self.0.stopping.load(std::sync::atomic::Ordering::Acquire) {
            return Ok(());
        }
        let live = self.live_agents()?;
        let retired = {
            let workers = self.0.fleet.lock().map_err(|_| Error::Unavailable)?;
            let active = workers.values().filter(|w| !w.sender.is_closed()).count();
            workers
                .iter()
                .filter(|(id, w)| {
                    !w.sender.is_closed()
                        && w.last_used.elapsed() >= Duration::from_secs(600)
                        && live.iter().any(|a| a.agent_id == **id && a.state == "idle")
                })
                .take(active.saturating_sub(1))
                .map(|(id, _)| id.clone())
                .collect::<Vec<_>>()
        };
        for id in retired {
            self.stop_worker(&id)?;
        }
        let enabled = self.read(|s| Ok(s.warm_agents.iter().cloned().collect::<Vec<_>>()))?;
        for id in enabled {
            let attempted = self
                .0
                .fleet
                .lock()
                .map_err(|_| Error::Unavailable)?
                .contains_key(&id);
            if !attempted {
                let _ = self.start_worker(&id).await;
            }
        }
        Ok(())
    }
    pub(super) fn stop_worker(&self, id: &str) -> Result<()> {
        self.0.chatgpt.revoke_worker(id, None);
        if let Some(worker) = self
            .0
            .fleet
            .lock()
            .map_err(|_| Error::Unavailable)?
            .get_mut(id)
        {
            let changed = worker.state != "stopping";
            worker.request_stop();
            if changed {
                self.execution_changed();
            }
        }
        let ids = self.read(|s| {
            Ok(s.sessions
                .values()
                .filter(|s| {
                    s.view.setup_id.as_deref() == Some(id) && s.view.conversation_id.is_none()
                })
                .map(|s| s.view.id.clone())
                .collect::<Vec<_>>())
        })?;
        for session in ids {
            let _ = self.send(
                &session,
                json!({"version":3,"type":"close","requestId":"stop-agent"}),
            );
        }
        Ok(())
    }
    pub(super) async fn agent_control(
        &self,
        actor: PrincipalId,
        command: Control,
    ) -> Result<Outcome> {
        self.actor(actor)?;
        if command.version != 1
            || !self.ready()
            || self.0.stopping.load(std::sync::atomic::Ordering::Acquire)
        {
            return Err(Error::Unavailable);
        }
        let digest = journal::digest(&command)?;
        let guard = self.0.mutation.lock().await;
        if let Some(outcome) = self.duplicate(command.request_id, &digest)? {
            return Ok(outcome);
        }
        self.read(|s| {
            s.setups
                .get(&command.agent_id)
                .ok_or(Error::Stale)
                .map(|_| ())
        })?;
        self.remember(command.request_id, digest, None)?;
        self.update(|s| {
            if command.start {
                s.warm_agents.insert(command.agent_id.clone());
            } else {
                s.warm_agents.remove(&command.agent_id);
            }
            Ok(())
        })?;
        if command.start {
            if self.start_worker(&command.agent_id).await.is_err() {
                return self.finish(command.request_id, "failed", Some("agent_unavailable"));
            }
        } else {
            self.stop_worker(&command.agent_id)?;
        }
        drop(guard);
        self.finish(
            command.request_id,
            if command.start {
                "starting"
            } else {
                "stopping"
            },
            None,
        )
    }
    pub(super) async fn start_worker(&self, id: &str) -> Result<()> {
        if self.tooling_busy()? {
            return Err(Error::Busy);
        }
        let setup = self.effective_setup(id)?;
        let paths = std::env::join_paths(self.tooling_paths()?).map_err(|_| Error::Unavailable)?;
        let closing = {
            let workers = self.0.fleet.lock().map_err(|_| Error::Unavailable)?;
            workers
                .get(id)
                .filter(|w| w.state == "stopping" && !w.sender.is_closed())
                .map(|w| w.sender.clone())
        };
        if let Some(sender) = closing {
            tokio::time::timeout(Duration::from_secs(20), sender.closed())
                .await
                .map_err(|_| Error::Busy)?;
        }
        let retire = {
            let workers = self.0.fleet.lock().map_err(|_| Error::Unavailable)?;
            if workers.values().filter(|w| !w.sender.is_closed()).count() >= 4
                && !workers.get(id).is_some_and(|w| !w.sender.is_closed())
            {
                self.read(|s| {
                    Ok(workers
                        .iter()
                        .find(|(name, w)| {
                            name.as_str() != id
                                && !w.sender.is_closed()
                                && w.state == "idle"
                                && w.last_used.elapsed() >= Duration::from_secs(600)
                                && !s.sessions.values().any(|v| {
                                    v.view.setup_id.as_deref() == Some(name.as_str())
                                        && matches!(v.view.state.as_str(), "running" | "opening")
                                })
                                && !s.intents.values().any(|i| {
                                    matches!(
                                        i.outcome.state.as_str(),
                                        "queued" | "dispatching" | "started"
                                    ) && i.outcome.session_id.as_ref().is_some_and(|session| {
                                        s.sessions.get(session).is_some_and(|v| {
                                            v.view.setup_id.as_deref() == Some(name.as_str())
                                        })
                                    })
                                })
                        })
                        .map(|(name, w)| (name.clone(), w.sender.clone())))
                })?
            } else {
                None
            }
        };
        if let Some((name, sender)) = retire {
            self.stop_worker(&name)?;
            let _ = tokio::time::timeout(Duration::from_secs(20), sender.closed()).await;
        }
        {
            let workers = self.0.fleet.lock().map_err(|_| Error::Unavailable)?;
            if workers.get(id).is_some_and(|w| !w.sender.is_closed()) {
                return Ok(());
            }
        }
        let worker_permit = self
            .0
            .worker_slots
            .lock()
            .map_err(|_| Error::Unavailable)?
            .as_ref()
            .map(|slots| slots.clone().try_acquire_owned().map_err(|_| Error::Limit))
            .transpose()?;
        let root = self.0.resources.root.join("workspaces");
        let mut command = Command::new(&self.0.resources.executable);
        self.launch_environment(&mut command, &setup)?;
        self.local_launch(&mut command, &setup, true).await?;
        self.register_launch_secrets(&command, &setup)?;
        let generation = self.update(|s| {
            let generation = s
                .worker_generations
                .get(id)
                .copied()
                .unwrap_or(0)
                .checked_add(1)
                .ok_or(Error::Limit)?;
            s.worker_generations.insert(id.into(), generation);
            Ok(generation)
        })?;

        let chatgpt_lease = self
            .chatgpt_launch(&mut command, &setup, id, generation)
            .await?;
        self.register_launch_secrets(&command, &setup)?;
        command
            .env("PATH", paths)
            .args(["--runner", &setup.runner, "--cwd"])
            .arg(&root)
            .args([
                "--queue-capacity",
                "64",
                "--idle-timeout",
                "300",
                "--max-turn-duration",
                "3600",
                "--cancel-grace",
                "2",
                "--reap-timeout",
                "2",
            ])
            .current_dir(&root)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .kill_on_drop(true);
        #[cfg(unix)]
        command.process_group(0);
        let child = native_runtime::start(&command).await?;
        let mut workers = self.0.fleet.lock().map_err(|_| Error::Unavailable)?;
        if workers.get(id).is_some_and(|w| !w.sender.is_closed()) {
            self.0.chatgpt.revoke_worker(id, Some(generation));
            return Ok(());
        }
        if workers.values().filter(|w| !w.sender.is_closed()).count()
            + self
                .0
                .runtimes
                .lock()
                .map_err(|_| Error::Unavailable)?
                .len()
            + usize::from(
                self.0
                    .catalog_active
                    .load(std::sync::atomic::Ordering::Acquire),
            )
            >= 4
        {
            self.0.chatgpt.revoke_worker(id, Some(generation));
            return Err(Error::Limit);
        }

        let (sender, receiver) = mpsc::channel(64);
        let (stop, stopping) = tokio::sync::watch::channel(false);
        workers.insert(
            id.into(),
            Worker {
                inference_host: self.read(|s| Ok(configuration::inference_host(&setup, s)))?,
                configuration: self.read(|s| configuration::launch_digest(&setup, s))?,
                sender,
                state: "starting".into(),
                generation,
                error: None,
                last_used: tokio::time::Instant::now(),
                stop,
            },
        );
        let this = self.clone();
        let id = id.to_owned();
        let task_id = format!("fleet/{id}");
        let task = tokio::spawn(async move {
            let _worker_permit = worker_permit;
            let _chatgpt_lease = chatgpt_lease;
            this.run_worker(&id, generation, child, receiver, stopping)
                .await;
        });
        self.0
            .runtime_tasks
            .lock()
            .map_err(|_| Error::Unavailable)?
            .insert(task_id, task.abort_handle());
        self.execution_changed();
        Ok(())
    }
    async fn run_worker(
        &self,
        id: &str,
        generation: u64,
        child: lapis_ai_runtime::AcpClient,
        mut commands: mpsc::Receiver<Value>,
        mut stopping: tokio::sync::watch::Receiver<bool>,
    ) {
        let mut child = child;
        let mut failure_code = None;
        let result: Result<()> = async {
            let input=child.commands();
            native_runtime::send(&input,json!({"version":3,"type":"warm","requestId":format!("warm-{id}-{generation}")})).await?;
            let startup=tokio::time::sleep(Duration::from_secs(90));tokio::pin!(startup);
            let mut warmed=false;
            let mut closing=false;
            let deadline=tokio::time::sleep(Duration::from_secs(3600));tokio::pin!(deadline);
            let mut stream=super::conversation_frames::Stream::default();
            let mut prompts=super::workspace_turns::PromptQueue::default();
            let mut workspace_lease = None;
            let mut retry_at = tokio::time::Instant::now();
            loop {
                if !prompts.is_active() { workspace_lease.take(); }
                if !closing && tokio::time::Instant::now() >= retry_at && let Some(frame)=prompts.next() {
                    let result = self.workspace_dispatch(frame.clone()).await;
                    if matches!(result, Err(Error::Busy)) {
                        prompts.defer(frame);
                        retry_at = tokio::time::Instant::now() + Duration::from_millis(100);
                    } else if let Ok((prepared, lease)) = result {
                        workspace_lease = Some(lease);
                        native_runtime::send(&input,prepared).await?;
                    } else {
                        let session=frame["sessionId"].as_str().ok_or(Error::Invalid)?;
                        let request=frame["requestId"].as_str().ok_or(Error::Invalid)?;
                        let failed=json!({"version":3,"type":"error","sessionId":session,"requestId":request,"code":"dispatch_not_sent"});
                        self.update(|s| delivery::observe(s,session,&failed))?;
                        self.finish(request.parse().map_err(|_|Error::Invalid)?,"failed",Some("workspace_start_snapshot_unavailable"))?;
                        prompts.completed(&failed);
                        continue;
                    }
                }
                tokio::select! {
                biased;
                _ = tokio::time::sleep_until(retry_at), if !closing && !prompts.is_active() && prompts.has_pending() => {},
                _=stopping.changed(),if !closing => {
                    closing=true;deadline.as_mut().reset(tokio::time::Instant::now()+Duration::from_secs(15));
                    native_runtime::send(&input,json!({"version":3,"type":"close","requestId":format!("stop-{id}")})).await?;
                },
                _=&mut deadline,if closing => return Err(Error::Unavailable),
                _=&mut startup,if !warmed => return Err(Error::Unavailable),
                frame=commands.recv(),if !closing => { let Some(frame)=frame else {return Err(Error::Unavailable)};
                    if frame["type"]=="prompt" {
                        let request=frame["requestId"].as_str().ok_or(Error::Invalid)?.parse().map_err(|_|Error::Invalid)?;
                        let session=frame["sessionId"].as_str().ok_or(Error::Invalid)?.to_owned();
                        if prompts.push(frame).is_ok() { self.finish(request,"queued",None)?; }
                        else {
                            self.update(|s| delivery::observe(s,&session,&json!({"type":"error","requestId":request,"code":"queue_full"})))?;
                            self.finish(request,"failed",Some("queue_full"))?;
                        }
                        continue;
                    }
                    if frame["type"]=="cancel" && let (Some(session),Some(target))=(frame["sessionId"].as_str(),frame["target"].as_str())
                        && prompts.cancel(session,target).is_some() {
                        self.update(|s| delivery::observe(s,session,&json!({"type":"error","requestId":target,"code":"dispatch_not_sent"})))?;
                        self.finish(target.parse().map_err(|_|Error::Invalid)?,"cancelled",None)?;
                        if let Some(request)=frame["requestId"].as_str().and_then(|s|s.parse().ok()) { self.finish(request,"accepted",None)?; }
                        continue;
                    }
                    if frame["type"]=="close" { closing=true;deadline.as_mut().reset(tokio::time::Instant::now()+Duration::from_secs(15)); }
                    native_runtime::send(&input,frame).await?;
                },
                frame=child.next(Duration::from_secs(86400))=> {
                    let frame = frame.map_err(|_|Error::Unavailable)?;
                    if frame["version"]!=3 {return Err(Error::Unavailable);}
                    if frame["sessionId"].is_string(){
                        self.touch_worker(id,generation);
                        self.conversation_frame(id,generation,frame.clone(),&mut stream).await?;
                        prompts.completed(&frame);
                        // These failures tear down the adapter inside lapis-acp. Its
                        // control process may remain alive, but it is no longer warm.
                        if frame["type"] == "error" && matches!(frame["code"].as_str(),
                            Some("timed_out" | "agent_exited" | "protocol_error" | "cancelled")) {
                            failure_code = frame["code"].as_str().map(str::to_owned);
                            self.worker_state(id,generation,"error",failure_code.as_deref());
                            return Err(Error::Unavailable);
                        }
                        continue;
                    }
                    match frame["type"].as_str() {
                        Some("warmed")=>{warmed=true;self.worker_state(id,generation,"idle",None);},
                        Some("closed") if frame["confirmedStopped"]==true => return Ok(()),
                        Some("error")=>return Err(Error::Unavailable),
                        _=>{},
                    }
                }
            } }
        }.await;
        let _ = child.shutdown().await;
        self.0.chatgpt.revoke_worker(id, Some(generation));
        let _ = self.interrupt_participations(id, generation);
        self.worker_state(
            id,
            generation,
            if result.is_ok() { "offline" } else { "error" },
            if result.is_ok() {
                None
            } else {
                failure_code.as_deref().or(Some("process_outcome_unknown"))
            },
        );
        if let Ok(mut tasks) = self.0.runtime_tasks.lock() {
            tasks.remove(&format!("fleet/{id}"));
        }
    }
    fn touch_worker(&self, id: &str, generation: u64) {
        if let Ok(mut workers) = self.0.fleet.lock()
            && let Some(worker) = workers.get_mut(id)
            && worker.generation == generation
        {
            worker.last_used = tokio::time::Instant::now();
        }
    }
    fn worker_state(&self, id: &str, generation: u64, state: &str, error: Option<&str>) {
        if let Ok(mut workers) = self.0.fleet.lock()
            && let Some(worker) = workers.get_mut(id)
            && worker.generation == generation
        {
            if worker.state == "stopping" && state == "idle" {
                return;
            }
            if worker.state == state && worker.error.as_deref() == error {
                return;
            }
            worker.state = state.into();
            worker.error = error.map(str::to_owned);
            self.execution_changed();
        }
    }
}
