//! Sidecar ownership, identity custody and bounded stdio control.
use super::*;
use std::process::Stdio;
use tokio::process::Command;
pub(super) fn validate(settings: &dto::Settings) -> Result<()> {
    if settings.peers.len() > 16
        || settings.relay_urls.len() > 8
        || settings.memory_gb.is_some_and(|m| m == 0 || m > 1024)
    {
        return Err(Error::Invalid);
    }
    if settings.model.as_ref().is_some_and(|m| {
        !label(m, 256)
            || m.starts_with('/')
            || m.contains("..")
            || m.contains('@')
            || m.contains("://")
    }) {
        return Err(Error::Invalid);
    }
    for (id, peer) in &settings.peers {
        if !label(&peer.name, 128)
            || peer.invitation["id"] != *id
            || !valid_invitation(&peer.invitation)
        {
            return Err(Error::Invalid);
        }
    }
    for relay in &settings.relay_urls {
        let url = url::Url::parse(relay).map_err(|_| Error::Invalid)?;
        if url.scheme() != "https"
            || !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
        {
            return Err(Error::Invalid);
        }
    }
    if settings
        .target_peer
        .as_ref()
        .is_some_and(|id| !settings.peers.contains_key(id))
    {
        return Err(Error::Invalid);
    }
    Ok(())
}
pub(super) fn valid_invitation(value: &Value) -> bool {
    let Some(object) = value.as_object() else {
        return false;
    };
    if value.to_string().len() > 2048 {
        return false;
    }
    if object.len() != 2 {
        return false;
    }
    let Some(id) = value["id"].as_str() else {
        return false;
    };
    if id.len() != 64 || hex::decode(id).is_err() {
        return false;
    }
    let Ok(bytes): std::result::Result<[u8; 32], _> =
        hex::decode(id).unwrap_or_default().try_into()
    else {
        return false;
    };
    if ed25519_dalek::VerifyingKey::from_bytes(&bytes).is_err() {
        return false;
    }
    let Some(addrs) = value["addrs"].as_array() else {
        return false;
    };
    !addrs.is_empty()
        && addrs.len() <= 16
        && addrs.iter().all(|addr| {
            let Some(fields) = addr.as_object() else {
                return false;
            };
            if fields.len() != 1 {
                return false;
            }
            if let Some(ip) = addr["Ip"].as_str() {
                return ip.parse::<std::net::SocketAddr>().is_ok_and(|a| {
                    a.port() != 0 && !a.ip().is_unspecified() && !a.ip().is_multicast()
                });
            }
            if let Some(relay) = addr["Relay"].as_str() {
                return url::Url::parse(relay).is_ok_and(|u| {
                    u.scheme() == "https"
                        && u.host_str().is_some()
                        && u.username().is_empty()
                        && u.password().is_none()
                        && u.query().is_none()
                        && u.fragment().is_none()
                });
            }
            false
        })
}
pub(super) fn wire_settings(settings: &dto::Settings) -> Value {
    json!({"model":settings.model,"memory_gb":settings.memory_gb,"sharing":settings.sharing,"relay_urls":settings.relay_urls,
        "peers":settings.peers.iter().map(|(id,peer)|(id.clone(),peer.invitation.clone())).collect::<BTreeMap<_,_>>()})
}
fn identity(root: &Path) -> Result<String> {
    if let Ok(value) = std::env::var("JJ_STORE_COMPUTE_IDENTITY") {
        if value.len() == 64 && hex::decode(&value).is_ok() {
            return Ok(value);
        }
        return Err(Error::Invalid);
    }
    #[cfg(target_os = "macos")]
    {
        super::super::secrets::compute_identity(root)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = root;
        Err(Error::Unavailable)
    }
}
impl Supervisor {
    pub(in crate::agent_execution) async fn ensure_compute(&self) -> Result<()> {
        let mut runtime = self.0.compute.lock().await;
        if runtime.sender.as_ref().is_some_and(|s| !s.is_closed()) {
            return Ok(());
        }
        let root = self.0.resources.root.join("compute");
        journal::private_directory(&root)?;
        let key = zeroize::Zeroizing::new(identity(&self.0.resources.root)?);
        let mut random = [0u8; 32];
        getrandom::fill(&mut random).map_err(|_| Error::Unavailable)?;
        let token = hex::encode(random);
        self.update(|s| {
            s.compute_generation = s.compute_generation.checked_add(1).ok_or(Error::Limit)?;
            Ok(())
        })?;
        let settings = self.read(|s| Ok(s.compute.clone()))?;
        runtime.relay_urls = settings.relay_urls.clone();
        let port = if let Some(port) = self.read(|s| Ok(s.compute_peer_port))? {
            port
        } else {
            let socket = std::net::UdpSocket::bind("0.0.0.0:0").map_err(|_| Error::Unavailable)?;
            let port = socket.local_addr().map_err(|_| Error::Unavailable)?.port();
            self.update(|s| {
                s.compute_peer_port = Some(port);
                Ok(())
            })?;
            port
        };
        let mut command =
            Command::new(self.companion("lapis-compute", "JJ_STORE_COMPUTE_EXECUTABLE")?);
        command
            .env_clear()
            .env("HOME", &root)
            .env("PATH", "/usr/bin:/bin")
            .env("JJ_STORE_COMPUTE_IDENTITY", key.as_str())
            .env("JJ_STORE_COMPUTE_TOKEN", &token)
            .env("JJ_STORE_COMPUTE_PORT", port.to_string())
            .env("JJ_STORE_COMPUTE_RELAYS", settings.relay_urls.join(","))
            .env(
                "MESH_LLM_NATIVE_RUNTIME_CACHE_DIR",
                root.join("native-runtime"),
            )
            .current_dir(&root)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .kill_on_drop(true);
        if let Some(bundle) = std::env::var_os("JJ_STORE_COMPUTE_NATIVE_RUNTIME_DIR")
            .map(PathBuf::from)
            .or_else(|| {
                self.0
                    .ai_bundle
                    .as_ref()
                    .and_then(|config| config.compute.native_runtime.clone())
            })
        {
            command.env("MESH_LLM_NATIVE_RUNTIME_BUNDLE_DIR", bundle);
        }
        #[cfg(unix)]
        command.process_group(0);
        let plan = lapis_ai_runtime::PreparedLaunch::from_command(command.as_std())
            .map_err(|_| Error::Invalid)?;
        let mut child = lapis_ai_runtime::ComputeClient::start_prepared(
            plan,
            native_runtime::limits(1024 * 1024),
        )
        .await
        .map_err(|_| Error::Unavailable)?;
        let ready = child
            .next_event(Duration::from_secs(1))
            .await
            .map_err(|_| Error::Unavailable)?;
        let input = child.commands();
        let (sender, mut frames) = mpsc::channel(16);
        let (stop, mut stopping) = tokio::sync::watch::channel(false);
        runtime.sender = Some(sender);
        runtime.stop = Some(stop);
        runtime.connection = Some(child.connection().clone());
        self.update(|s| {
            s.compute_identity = ready["identity"].as_str().map(str::to_owned);
            Ok(())
        })?;
        runtime.identity = ready["identity"].as_str().map(str::to_owned);
        runtime.invitation = ready.get("invitation").cloned();
        runtime.state = "ready".into();
        let this = self.clone();
        runtime.task = Some(tokio::spawn(async move {
            let result:Result<()>=async {loop {
                tokio::select! {
                    _=stopping.changed()=>{break;},
                    frame=frames.recv()=>{let Some(frame)=frame else{break};input.send_frame(&frame).await.map_err(|_|Error::Unavailable)?;},
                    event=child.next_event(Duration::from_secs(86400))=>{
                        let frame=event.map_err(|_|Error::Unavailable)?;
                        this.compute_frame(frame).await?;
                    }
                }
            }Ok(())}.await;
            let result = result.and(child.shutdown().await.map_err(|_| Error::Unavailable));
            let mut state = this.0.compute.lock().await;
            state.sender = None;
            state.stop = None;
            state.connection = None;
            state.state = if result.is_ok() && *stopping.borrow() {
                "off"
            } else {
                "error"
            }
            .into();
            state.error = if state.state == "error" {
                Some("compute_interrupted_no_replay".into())
            } else {
                None
            };
            if let Some(request) = state.operation {
                let _ = this.finish(
                    request,
                    if state.state == "off" {
                        "completed"
                    } else {
                        "outcome_unknown"
                    },
                    state.error.as_deref(),
                );
            }
        }));
        Ok(())
    }
    async fn compute_frame(&self, frame: Value) -> Result<()> {
        let request = frame["requestId"]
            .as_str()
            .and_then(|r| r.parse::<RequestId>().ok());
        let mut state = self.0.compute.lock().await;
        match frame["type"].as_str() {
            Some("progress") => state.progress = Some(frame),
            Some("serving") => state.state = "serving".into(),
            Some("configured") => {
                if let Some(request) = request {
                    let action = self
                        .read(|s| Ok(s.intents.get(&request).map(|i| i.outcome.state.clone())))?;
                    if action.as_deref() == Some("configuring") {
                        let _ = self.finish(request, "completed", None);
                    }
                }
            }
            Some("imported") => {
                let model = frame["model"]
                    .as_str()
                    .filter(|id| id.starts_with("imported-") && id.len() == 73)
                    .ok_or(Error::Invalid)?;
                self.update(|s| {
                    s.compute_models.insert(model.into());
                    Ok(())
                })?;
            }
            Some("completed") => {
                let action = frame["action"].as_str().unwrap_or("");
                self.update(|s| {
                    if action == "remove"
                        && let Some(model) = frame["model"].as_str()
                    {
                        s.compute_models.remove(model);
                    }
                    if action == "prepare" {
                        s.compute_runtime_prepared = true;
                    }
                    if action == "download"
                        && let Some(model) = &s.compute.model
                    {
                        s.compute_models.insert(model.clone());
                    }
                    Ok(())
                })?;
                if action != "start" {
                    state.state = "ready".into();
                }
                state.progress = None;
                if let Some(request) = request {
                    self.finish(request, "completed", None)?;
                }
            }
            Some("error") => {
                state.error = Some("compute_operation_failed".into());
                if state.state != "serving" {
                    state.state = "error".into();
                }
                if let Some(request) = request {
                    self.finish(request, "failed", Some("compute_operation_failed"))?;
                }
            }
            _ => return Err(Error::Invalid),
        }
        Ok(())
    }
}
