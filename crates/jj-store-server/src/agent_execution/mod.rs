//! Local device execution; journal admission precedes every process side effect.
mod avatar;
mod catalog;
mod commands;
mod comment_answers;
mod comment_submissions;
mod compute;
mod configuration;
mod conversation_frames;
mod conversation_home;
mod conversation_links;
mod conversation_origin;
mod conversation_prompts;
mod conversations;
mod coordination;
mod delivery;
pub mod driver;
mod feedback;
mod fleet;
mod freshness;
mod journal;
mod launch;
mod local_agent;
mod metadata_cancel;
mod metadata_directory;
mod metadata_jobs;
mod metadata_runtime;
mod metadata_sources;
mod metadata_store;
mod metadata_suggestions;
mod metadata_text;
mod provider_catalog;
mod provider_launch;
mod qualified_reviews;

mod prompt_context;
mod publication;
mod runtime;
mod secrets;
mod sent_context;
mod targets;
mod tooling;
mod workspace_deferred;
mod workspace_freeze;
mod workspace_inspection;
mod workspace_publication;
mod workspace_sources;
mod workspace_turns;
use jj_store_model::{OperationId, PrincipalId, RepositoryId, RequestId};
use jj_store_service::{
    administration::RepositoryHost,
    agent_execution::*,
    local_browser::LocalRepositories,
    local_management::{WorkspaceManager, WorkspaceVerifier},
    registry::RepositoryRegistry,
};
use journal::{Intent, SavedSession, State};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::sync::{broadcast, mpsc};
#[derive(Clone)]
pub struct Resources {
    pub actor: PrincipalId,
    pub root: PathBuf,
    pub executable: PathBuf,
    pub jj_executable: PathBuf,
    pub driver: Arc<dyn WorkspaceDriver>,
    pub folders: Option<Arc<dyn LocalRepositories>>,
    pub workspaces: Option<Arc<dyn WorkspaceManager>>,
    pub verifier: Option<Arc<dyn WorkspaceVerifier>>,
    pub registry: Arc<dyn RepositoryRegistry>,
    pub host: Option<RepositoryHost>,
    pub native_socket: Option<PathBuf>,
}
#[derive(Clone)]
pub struct Supervisor(Arc<Inner>);
struct Inner {
    resources: Resources,
    state: Mutex<State>,
    runtimes: Mutex<BTreeMap<String, mpsc::Sender<Value>>>,
    fleet: Mutex<BTreeMap<String, fleet::Worker>>,
    compute: tokio::sync::Mutex<compute::Runtime>,
    catalogs: tokio::sync::Mutex<catalog::Cache>,
    catalog_active: std::sync::atomic::AtomicBool,
    runtime_tasks: Mutex<BTreeMap<String, tokio::task::AbortHandle>>,
    publication_worker: Mutex<Option<tokio::task::AbortHandle>>,
    installer_task: Mutex<Option<tokio::task::JoinHandle<()>>>,
    shutdown: tokio::sync::Mutex<()>,
    shutdown_finished: Mutex<Option<bool>>,
    metadata_inference: tokio::sync::Mutex<()>,
    mutation: tokio::sync::Mutex<()>,
    workspace_gate: tokio::sync::Mutex<()>,
    workspace_activity: Mutex<BTreeMap<String, RequestId>>,
    inspection_watches: Mutex<BTreeMap<(String, RepositoryId), workspace_inspection::watch::Watch>>,
    inspection_slots: tokio::sync::Semaphore,
    publication: tokio::sync::Mutex<()>,
    conversation_dispatch: tokio::sync::Mutex<()>,
    changed: broadcast::Sender<u64>,
    freshness: jj_store_service::freshness::Signals,
    stopping: std::sync::atomic::AtomicBool,
    sanitizer: Mutex<Option<jj_store_capture::transcript_sanitization::TranscriptSanitizer>>,
    privacy_failed: std::sync::atomic::AtomicBool,
}
impl Supervisor {
    pub fn open(mut resources: Resources) -> Result<Self> {
        resources.root =
            crate::policy::outside_checkout(&resources.root).map_err(|_| Error::Invalid)?;
        resources.executable = resources
            .executable
            .canonicalize()
            .map_err(|_| Error::Unavailable)?;
        if !resources.executable.is_file() {
            return Err(Error::Unavailable);
        }
        resources.jj_executable = resources
            .jj_executable
            .canonicalize()
            .map_err(|_| Error::Unavailable)?;
        if !resources.jj_executable.is_file() {
            return Err(Error::Unavailable);
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            if resources
                .jj_executable
                .metadata()
                .map_err(|_| Error::Unavailable)?
                .permissions()
                .mode()
                & 0o111
                == 0
            {
                return Err(Error::Unavailable);
            }
        }
        journal::private_directory(&resources.root)?;
        journal::private_directory(&resources.root.join("workspaces"))?;
        let mut state = journal::read(&resources.root)?;
        if state.owner.is_some_and(|owner| owner != resources.actor) {
            return Err(Error::Stale);
        }
        state.owner = Some(resources.actor);
        // Older private journals used their installation revision for setup CAS.
        // Retain that precondition once, then isolate it from execution activity.
        state.setup_revision = Some(state.setup_revision.unwrap_or(state.revision));
        journal::persist(&resources.root, &state)?;
        let (changed, _) = broadcast::channel(32);
        let supervisor = Self(Arc::new(Inner {
            resources,
            state: Mutex::new(state),
            runtimes: Mutex::new(BTreeMap::new()),
            fleet: Mutex::new(BTreeMap::new()),
            compute: tokio::sync::Mutex::new(Default::default()),
            catalogs: tokio::sync::Mutex::new(Default::default()),
            catalog_active: std::sync::atomic::AtomicBool::new(false),
            runtime_tasks: Mutex::new(BTreeMap::new()),
            publication_worker: Mutex::new(None),
            installer_task: Mutex::new(None),
            shutdown: tokio::sync::Mutex::new(()),
            shutdown_finished: Mutex::new(None),
            metadata_inference: tokio::sync::Mutex::new(()),
            mutation: tokio::sync::Mutex::new(()),
            workspace_gate: tokio::sync::Mutex::new(()),
            workspace_activity: Mutex::new(BTreeMap::new()),
            inspection_watches: Mutex::new(BTreeMap::new()),
            inspection_slots: tokio::sync::Semaphore::new(2),
            publication: tokio::sync::Mutex::new(()),
            conversation_dispatch: tokio::sync::Mutex::new(()),
            changed,
            freshness: Default::default(),
            stopping: std::sync::atomic::AtomicBool::new(false),
            sanitizer: Mutex::new(None),
            privacy_failed: std::sync::atomic::AtomicBool::new(false),
        }));
        supervisor.prepare_privacy();
        Ok(supervisor)
    }
    fn prepare_privacy(&self) {
        let this = self.clone();
        // Detached rule compilation owns no child process or publication. A
        // tokio blocking worker would delay runtime teardown until compilation
        // completed, violating the desktop parent-EOF deadline.
        std::thread::spawn(move || {
            let result = (|| {
                let root = this.0.resources.root.clone();
                let home = std::env::var_os("HOME")
                    .map(PathBuf::from)
                    .ok_or(Error::Unavailable)?;
                let cache = std::env::var_os("JJ_STORE_AGENT_PRIVACY_CACHE")
                    .map(PathBuf::from)
                    .unwrap_or_else(|| root.join("privacy-cache"));
                jj_store_capture::transcript_sanitization::TranscriptSanitizer::new(
                    &root.join("workspaces"),
                    &home,
                    &std::env::temp_dir(),
                    &cache,
                )
                .map_err(|_| Error::Unavailable)
            })();
            if this.0.stopping.load(std::sync::atomic::Ordering::Acquire) {
                return;
            }
            match result {
                Ok(sanitizer) => {
                    if let Ok(mut ready) = this.0.sanitizer.lock() {
                        *ready = Some(sanitizer);
                    }
                }
                Err(_) => {
                    this.0
                        .privacy_failed
                        .store(true, std::sync::atomic::Ordering::Release);
                }
            }
            this.execution_changed();
        });
    }
    pub fn ready(&self) -> bool {
        self.0.sanitizer.lock().is_ok_and(|s| s.is_some())
    }
    pub fn start_publication(&self) -> tokio::task::JoinHandle<()> {
        let this = self.clone();
        let worker = tokio::spawn(async move {
            while !this.0.stopping.load(std::sync::atomic::Ordering::Acquire) {
                let _ = this.warm_enabled().await;
                let _ = this.publish().await;
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        });
        if let Ok(mut slot) = self.0.publication_worker.lock() {
            if let Some(previous) = slot.replace(worker.abort_handle()) {
                previous.abort();
            }
        }
        worker
    }
    fn actor(&self, actor: PrincipalId) -> Result<()> {
        if actor != self.0.resources.actor {
            return Err(Error::Denied);
        }
        Ok(())
    }
    fn read<T>(&self, f: impl FnOnce(&State) -> Result<T>) -> Result<T> {
        let state = self.0.state.lock().map_err(|_| Error::Unavailable)?;
        f(&state)
    }
    fn update<T>(&self, f: impl FnOnce(&mut State) -> Result<T>) -> Result<T> {
        let mut state = self.0.state.lock().map_err(|_| Error::Unavailable)?;
        let mut candidate = state.clone();
        let result = f(&mut candidate)?;
        let execution_changed =
            freshness::execution_witness(&candidate)? != freshness::execution_witness(&state)?;
        let settings_changed = candidate.settings.revision != state.settings.revision;
        candidate.revision = candidate.revision.checked_add(1).ok_or(Error::Limit)?;
        journal::persist(&self.0.resources.root, &candidate)?;
        *state = candidate;
        let _ = self.0.changed.send(state.revision);
        if execution_changed {
            self.0.freshness.notify(
                None,
                jj_store_service::freshness::Domain::AgentExecution,
                None,
            );
        }
        if settings_changed {
            self.0.freshness.notify(
                None,
                jj_store_service::freshness::Domain::AgentSettings,
                None,
            );
        }
        Ok(result)
    }
    fn duplicate(&self, request: RequestId, digest: &str) -> Result<Option<Outcome>> {
        self.read(|s| match s.intents.get(&request) {
            Some(intent) if intent.digest == digest => Ok(Some(intent.outcome.clone())),
            Some(_) => Err(Error::Conflict),
            None => Ok(None),
        })
    }
    fn remember(
        &self,
        request: RequestId,
        digest: String,
        session: Option<String>,
    ) -> Result<Outcome> {
        self.update(|s| {
            if s.intents.len() >= 4096 {
                return Err(Error::Limit);
            }
            let outcome = Outcome {
                comment_publication: None,
                version: 1,
                request_id: request,
                session_id: session,
                state: "dispatching".into(),
                error: None,
            };
            s.intents.insert(
                request,
                Intent {
                    digest,
                    prompt: None,
                    outcome: outcome.clone(),
                },
            );
            Ok(outcome)
        })
    }
    fn finish(&self, request: RequestId, state: &str, error: Option<&str>) -> Result<Outcome> {
        self.update(|s| {
            let intent = s.intents.get_mut(&request).ok_or(Error::Invalid)?;
            intent.outcome.state = state.into();
            intent.outcome.error = error.map(str::to_owned);
            let outcome = intent.outcome.clone();
            sent_context::outcome(s, request, state)?;
            Ok(outcome)
        })
    }
    fn saved(&self, id: &str) -> Result<SavedSession> {
        self.read(|s| s.sessions.get(id).cloned().ok_or(Error::Invalid))
    }
    fn append(&self, id: &str, mut payload: Value) -> Result<()> {
        payload["controllerSessionId"] = json!(id);
        self.update(|s| {
            delivery::observe(s, id, &payload)?;
            delivery::append(s, id, payload)?;
            Ok(())
        })
    }
    fn sender(&self, id: &str) -> Result<mpsc::Sender<Value>> {
        let saved = self.saved(id)?;
        if saved.view.conversation_id.is_some() {
            if !matches!(saved.view.state.as_str(), "idle" | "running" | "opening") {
                return Err(Error::Unavailable);
            }
            let workers = self.0.fleet.lock().map_err(|_| Error::Unavailable)?;
            let worker = workers
                .get(saved.view.setup_id.as_deref().ok_or(Error::Invalid)?)
                .ok_or(Error::Unavailable)?;
            if saved.view.capabilities["workerGeneration"] != worker.generation
                || worker.sender.is_closed()
                || matches!(worker.state.as_str(), "stopping" | "offline" | "error")
            {
                return Err(Error::Unavailable);
            }
            return Ok(worker.sender.clone());
        }
        self.0
            .runtimes
            .lock()
            .map_err(|_| Error::Unavailable)?
            .get(id)
            .cloned()
            .ok_or(Error::Unavailable)
    }
    fn send(&self, id: &str, mut frame: Value) -> Result<()> {
        if self.saved(id)?.view.conversation_id.is_some() {
            frame["version"] = json!(3);
            frame["sessionId"] = json!(id);
            if frame["type"] == "close" {
                frame["type"] = json!("close_session");
            }
        }
        self.sender(id)?.try_send(frame).map_err(|_| Error::Busy)
    }
}
fn label(value: &str, max: usize) -> bool {
    !value.trim().is_empty() && value.len() <= max && !value.chars().any(char::is_control)
}
fn preferences(value: &Preferences) -> Result<()> {
    for text in [&value.model, &value.effort, &value.mode]
        .into_iter()
        .flatten()
    {
        if !label(text, 256) {
            return Err(Error::Invalid);
        }
    }
    Ok(())
}
