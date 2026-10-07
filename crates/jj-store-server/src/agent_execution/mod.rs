//! Local device execution; journal admission precedes every process side effect.
mod avatar;
mod commands;
mod delivery;
pub mod driver;
mod feedback;
mod journal;
mod launch;
mod publication;
mod runtime;
mod targets;
mod tooling;
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
    pub driver: Arc<dyn WorkspaceDriver>,
    pub folders: Option<Arc<dyn LocalRepositories>>,
    pub workspaces: Option<Arc<dyn WorkspaceManager>>,
    pub verifier: Option<Arc<dyn WorkspaceVerifier>>,
    pub registry: Arc<dyn RepositoryRegistry>,
    pub host: Option<RepositoryHost>,
}
#[derive(Clone)]
pub struct Supervisor(Arc<Inner>);
struct Inner {
    resources: Resources,
    state: Mutex<State>,
    runtimes: Mutex<BTreeMap<String, mpsc::Sender<Value>>>,
    runtime_tasks: Mutex<BTreeMap<String, tokio::task::AbortHandle>>,
    publication_worker: Mutex<Option<tokio::task::AbortHandle>>,
    installer_task: Mutex<Option<tokio::task::JoinHandle<()>>>,
    shutdown: tokio::sync::Mutex<()>,
    shutdown_finished: Mutex<Option<bool>>,
    mutation: tokio::sync::Mutex<()>,
    publication: tokio::sync::Mutex<()>,
    changed: broadcast::Sender<u64>,
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
            runtime_tasks: Mutex::new(BTreeMap::new()),
            publication_worker: Mutex::new(None),
            installer_task: Mutex::new(None),
            shutdown: tokio::sync::Mutex::new(()),
            shutdown_finished: Mutex::new(None),
            mutation: tokio::sync::Mutex::new(()),
            publication: tokio::sync::Mutex::new(()),
            changed,
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
            let _ = this.0.changed.send(0);
        });
    }
    pub fn ready(&self) -> bool {
        self.0.sanitizer.lock().is_ok_and(|s| s.is_some())
    }
    pub fn start_publication(&self) -> tokio::task::JoinHandle<()> {
        let this = self.clone();
        let worker = tokio::spawn(async move {
            while !this.0.stopping.load(std::sync::atomic::Ordering::Acquire) {
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
        candidate.revision = candidate.revision.checked_add(1).ok_or(Error::Limit)?;
        journal::persist(&self.0.resources.root, &candidate)?;
        *state = candidate;
        let _ = self.0.changed.send(state.revision);
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
            Ok(intent.outcome.clone())
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
        self.0
            .runtimes
            .lock()
            .map_err(|_| Error::Unavailable)?
            .get(id)
            .cloned()
            .ok_or(Error::Unavailable)
    }
    fn send(&self, id: &str, frame: Value) -> Result<()> {
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
