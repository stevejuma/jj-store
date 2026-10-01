mod file;
use crate::{
    config,
    search::{self, PreparedSearch},
    search_worker::Control,
};
use file::{SettingsFile, configuration};
use jj_store_model::RepositoryId;
use jj_store_service::{administration::RepositoryHost, search_management::*};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, OnceLock},
};
use tokio::sync::Mutex as AsyncMutex;
#[derive(Default)]
struct State {
    effective: SearchConfiguration,
    operation: SearchOperation,
    error: Option<String>,
}
pub type Observer = Arc<
    dyn Fn(RepositoryId, Option<tokio::sync::watch::Receiver<jj_store_adapters::Health>>)
        + Send
        + Sync,
>;
struct Inner {
    file: Arc<SettingsFile>,
    host: RepositoryHost,
    root: PathBuf,
    limits: config::Limits,
    access: search::SearchAccess,
    control: Arc<Control>,
    gate: Arc<AsyncMutex<()>>,
    states: Mutex<BTreeMap<RepositoryId, State>>,
    registry: OnceLock<jj_store_http::SearchRegistry>,
    observer: Observer,
}
#[derive(Clone)]
pub struct Manager(Arc<Inner>);
impl Manager {
    pub fn new(
        path: &Path,
        config: &config::Config,
        host: RepositoryHost,
        control: Arc<Control>,
        access: search::SearchAccess,
        observer: Observer,
    ) -> crate::Result<Self> {
        let file = SettingsFile::new(path)?;
        file.read()
            .map_err(|_| crate::Error::Config("local search configuration"))?;
        Ok(Self(Arc::new(Inner {
            file: Arc::new(file),
            host,
            root: config.state_dir.clone(),
            limits: config.limits.clone(),
            access,
            control,
            gate: Arc::new(AsyncMutex::new(())),
            states: Mutex::new(
                config
                    .search
                    .iter()
                    .map(|e| {
                        (
                            e.repository,
                            State {
                                effective: configuration(e),
                                ..Default::default()
                            },
                        )
                    })
                    .collect(),
            ),
            registry: OnceLock::new(),
            observer,
        })))
    }
    pub fn bind(&self, registry: jj_store_http::SearchRegistry) {
        let _ = self.0.registry.set(registry);
    }
    async fn snapshot(&self) -> Result<file::Snapshot, Error> {
        let file = self.0.file.clone();
        tokio::task::spawn_blocking(move || file.read())
            .await
            .map_err(|_| Error::Unavailable)?
    }
    fn report(&self, repo: RepositoryId, snapshot: &file::Snapshot) -> LocalSearchSettings {
        let states = self.0.states.lock().unwrap();
        let empty = State::default();
        let state = states.get(&repo).unwrap_or(&empty);
        LocalSearchSettings {
            version: 1,
            revision: snapshot.revision.clone(),
            desired: snapshot.configuration(repo),
            effective: state.effective.clone(),
            operation: state.operation,
            error: state.error.clone(),
        }
    }
    async fn execute(
        &self,
        repo: RepositoryId,
        request: SearchCommand,
        snapshot: file::Snapshot,
    ) -> Result<(), Error> {
        let registry = self.0.registry.get().ok_or(Error::Unavailable)?;
        if self.0.control.closed() {
            return Err(Error::Unavailable);
        }
        // Keep the stopped owner until a replacement is verified, so failure can resume it.
        let old = self
            .0
            .control
            .stop(repo)
            .await
            .map_err(|_| Error::Unavailable)?;
        if matches!(request.action, SearchAction::Retry) {
            if let Some(runner) = old {
                self.0.control.start(repo, runner).await;
                return Ok(());
            }
            return Err(Error::Unavailable);
        }
        let desired = snapshot.configuration(repo);
        let result = async {
            if !desired.enabled {
                if self.snapshot().await?.revision != request.expected_revision {
                    return Err(Error::Conflict);
                }
                registry
                    .replace(repo, None)
                    .map_err(|_| Error::Unavailable)?;
                (self.0.observer)(repo, None);
                return Ok(());
            }
            let entry = snapshot.entry(repo).ok_or(Error::Invalid)?.clone();
            let mut prepared = PreparedSearch::build(
                &self.0.host,
                &self.0.root,
                &[entry],
                &self.0.limits,
                self.0.access,
                search::environment,
                true,
            )
            .map_err(|_| Error::Unavailable)?;
            let (_, mut runner) = prepared.0.runners.pop().ok_or(Error::Unavailable)?;
            if matches!(request.action, SearchAction::Rebuild) {
                runner.skip_snapshot_restore();
            }
            let cancel = self.0.control.cancellation();
            let step = runner.step(cancel).await;
            // Retain the owner until every timed-out callback drains.
            runner.wait_idle().await.map_err(|_| Error::Unavailable)?;
            let health = step.map_err(|_| Error::Unavailable)?;
            if health.applied.is_none() || health.degraded {
                return Err(Error::Unavailable);
            }
            let index = prepared.1.remove(&repo).ok_or(Error::Unavailable)?;
            let session = self
                .0
                .host
                .endpoint(repo)
                .map_err(|_| Error::Unavailable)?
                .bind_local_trusted(
                    match self.0.access {
                        search::SearchAccess::LocalTrusted(p) => p,
                        _ => return Err(Error::Unavailable),
                    },
                    jj_store_service::auth::Permission::Read,
                )
                .map_err(|_| Error::Unavailable)?
                .into_session();
            index
                .search
                .pin(&session.adapter_source())
                .await
                .map_err(|_| Error::Unavailable)?;
            if self.snapshot().await?.revision != request.expected_revision {
                return Err(Error::Conflict);
            }
            if self.0.control.closed() {
                return Err(Error::Unavailable);
            }
            let health = index.health.clone();
            registry
                .replace(repo, Some(index))
                .map_err(|_| Error::Unavailable)?;
            (self.0.observer)(repo, Some(health));
            self.0.control.start(repo, runner).await;
            Ok(())
        }
        .await;
        if let Err(error) = result {
            if let Some(runner) = old {
                self.0.control.start(repo, runner).await;
            }
            return Err(error);
        }
        self.0
            .states
            .lock()
            .unwrap()
            .entry(repo)
            .or_default()
            .effective = desired;
        Ok(())
    }
}
#[async_trait::async_trait]
impl SearchManager for Manager {
    async fn status(&self, repo: RepositoryId) -> Result<LocalSearchSettings, Error> {
        Ok(self.report(repo, &self.snapshot().await?))
    }
    async fn save(
        &self,
        repo: RepositoryId,
        request: SaveSearchSettings,
    ) -> Result<LocalSearchSettings, Error> {
        if request.version != 1 {
            return Err(Error::Invalid);
        }
        let permit = self
            .0
            .gate
            .clone()
            .try_lock_owned()
            .map_err(|_| Error::Busy)?;
        let file = self.0.file.clone();
        tokio::task::spawn_blocking(move || {
            let _permit = permit;
            file.save(repo, &request.expected_revision, &request.configuration)
        })
        .await
        .map_err(|_| Error::Unknown)??;
        self.status(repo).await
    }
    async fn control(
        &self,
        repo: RepositoryId,
        request: SearchCommand,
    ) -> Result<LocalSearchSettings, Error> {
        if request.version != 1 {
            return Err(Error::Invalid);
        }
        let permit = self
            .0
            .gate
            .clone()
            .try_lock_owned()
            .map_err(|_| Error::Busy)?;
        let snapshot = self.snapshot().await?;
        if snapshot.revision != request.expected_revision {
            return Err(Error::Conflict);
        }
        {
            let mut states = self.0.states.lock().unwrap();
            let state = states.entry(repo).or_default();
            if matches!(request.action, SearchAction::Retry | SearchAction::Rebuild)
                && state.effective != snapshot.configuration(repo)
            {
                return Err(Error::Conflict);
            }
            state.operation = match request.action {
                SearchAction::Apply => SearchOperation::Applying,
                SearchAction::Retry => SearchOperation::Retrying,
                SearchAction::Rebuild => SearchOperation::Rebuilding,
            };
            state.error = None;
        }
        let report = self.report(repo, &snapshot);
        let this = self.clone();
        self.0
            .control
            .spawn(async move {
                let _permit = permit;
                let result = this.execute(repo, request, snapshot).await;
                let mut states = this.0.states.lock().unwrap();
                let state = states.entry(repo).or_default();
                state.operation = if result.is_ok() {
                    SearchOperation::Idle
                } else {
                    SearchOperation::Failed
                };
                state.error = result.err().map(|e| e.to_string());
            })
            .await?;
        Ok(report)
    }
}
