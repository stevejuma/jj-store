use crate::scopes::{Scope, scoped};
use crate::{Admission, Config, Context, Error, Result, local};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tokio::sync::{OwnedRwLockReadGuard, RwLock, Semaphore};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(transparent)]
pub struct Handle(#[schemars(with = "String")] pub(crate) Uuid);
#[derive(Clone, Serialize, schemars::JsonSchema)]
#[serde(tag = "state", content = "data", rename_all = "snake_case")]
pub enum Status {
    Preparing,
    Ready(local::Snapshot),
    Failed(Error),
}
pub(crate) enum Output {
    Preparing,
    Ready(Arc<local::code_index::Prepared>),
    Failed(Error),
}
pub(crate) struct Entry {
    pub caller: Context,
    pub vault: String,
    pub repository: jj_store_model::RepositoryId,
    pub generation: u64,
    pub cancellation: crate::Cancellation,
    pub task: Option<tokio::task::JoinHandle<()>>,
    pub output: Output,
    pub scope: Arc<Scope>,
    pub readers: Arc<RwLock<()>>,
}
/// Native service lifetime. Hosts must await shutdown before releasing admission.
pub struct History {
    pub(crate) admission: Arc<dyn Admission>,
    pub(crate) root: PathBuf,
    pub(crate) config: Config,
    pub(crate) executor: local::Executor,
    pub(crate) blame: Arc<crate::blame::Cache>,
    pub(crate) entries: Mutex<HashMap<Handle, Entry>>,
    pub(crate) windows: Mutex<HashMap<String, Option<Arc<Scope>>>>,
    pub(crate) preparation: Arc<Semaphore>,
    pub(crate) lifecycle: tokio::sync::Mutex<()>,
    pub(crate) closing: std::sync::atomic::AtomicBool,
}
impl History {
    pub fn new(admission: impl Admission, root: PathBuf, config: Config) -> Result<Arc<Self>> {
        if !root.is_absolute() || !(1..=128).contains(&config.max_handles) {
            return Err(Error::Invalid);
        }
        let workers = jj_store_jj::query::workers::Workers::new(config.concurrency)?;
        Ok(Arc::new(Self {
            admission: Arc::new(admission),
            root,
            config,
            executor: local::Executor::with_workers(workers, config.reads),
            blame: Arc::new(crate::blame::Cache::default()),
            entries: Mutex::new(HashMap::new()),
            windows: Mutex::new(HashMap::new()),
            preparation: Arc::new(Semaphore::new(1)),
            lifecycle: tokio::sync::Mutex::new(()),
            closing: std::sync::atomic::AtomicBool::new(false),
        }))
    }
    pub(crate) async fn admit(
        &self,
        caller: &Context,
        vault: &str,
    ) -> Result<Arc<dyn local::Access>> {
        if self.closing.load(std::sync::atomic::Ordering::SeqCst)
            || caller.window.is_empty()
            || caller.window.len() > 256
            || caller.url.len() > 8192
            || vault.is_empty()
            || vault.len() > 256
            || vault.chars().any(char::is_control)
        {
            return Err(Error::Denied);
        }
        let access = self.admission.admit(caller, vault).await?;
        access.authorize().await?;
        Ok(access)
    }
    pub async fn heads(&self, caller: &Context, vault: &str) -> Result<local::Heads> {
        let access = self.admit(caller, vault).await?;
        let scope = self.scope(caller)?;
        let _lease = scope.lease().await?;
        self.executor
            .heads(scoped(access, vec![scope.cancellation.clone()]))
            .await
    }
    pub(crate) async fn selected(
        &self,
        caller: &Context,
        vault: &str,
        handle: Handle,
    ) -> Result<Selection> {
        let access = self.admit(caller, vault).await?;
        let (scope, readers, cancellation) = {
            let entries = self.entries.lock().map_err(|_| Error::Unavailable)?;
            let entry = entries.get(&handle).ok_or(Error::Denied)?;
            if &entry.caller != caller || entry.vault != vault {
                return Err(Error::Denied);
            }
            if entry.repository != access.repository() || entry.generation != access.generation() {
                return Err(Error::EpochChanged);
            }
            (
                entry.scope.clone(),
                entry.readers.clone(),
                entry.cancellation.clone(),
            )
        };
        let window = scope.lease().await?;
        let read = readers.read_owned().await;
        let access = scoped(access, vec![scope.cancellation.clone(), cancellation]);
        access.authorize().await?;
        Ok(Selection {
            access,
            _window: window,
            _read: read,
        })
    }
    pub async fn status(&self, caller: &Context, vault: &str, handle: Handle) -> Result<Status> {
        let selection = self.selected(caller, vault, handle).await?;
        let status = {
            let entries = self.entries.lock().map_err(|_| Error::Unavailable)?;
            match &entries.get(&handle).ok_or(Error::Denied)?.output {
                Output::Preparing => Status::Preparing,
                Output::Ready(prepared) => Status::Ready(prepared.snapshot().clone()),
                Output::Failed(error) => Status::Failed(error.clone()),
            }
        };
        selection.access.authorize().await?;
        Ok(status)
    }
    pub async fn read(
        &self,
        caller: &Context,
        vault: &str,
        handle: Handle,
        request: local::Request,
    ) -> Result<local::Response> {
        let selection = self.selected(caller, vault, handle).await?;
        let prepared = {
            let entries = self.entries.lock().map_err(|_| Error::Unavailable)?;
            match &entries.get(&handle).ok_or(Error::Denied)?.output {
                Output::Ready(prepared) => prepared.clone(),
                Output::Preparing => return Err(Error::Busy),
                Output::Failed(error) => return Err(error.clone()),
            }
        };
        tokio::time::timeout(self.config.preparation.timeout, async {
            // Point metadata uses the existing bounded reader, not a graph-index
            // rebuild. The handle still owns caller admission and the outer pin.
            let metadata = matches!(&request, local::Request::QueryInfo | local::Request::Operation { .. })
                || matches!(&request, local::Request::References(query)
                    if matches!(query.kind, local::references::Kind::Bookmarks | local::references::Kind::Workspaces)
                    && query.query.is_empty());
            if metadata {
                let anchor = prepared.snapshot().operation;
                let operation = if let local::Request::Operation { operation } = &request {
                    if *operation != anchor {
                        self.executor.validate_reference_operation(selection.access.clone(), anchor, *operation).await?;
                    }
                    *operation
                } else { anchor };
                return self.executor.read(selection.access.clone(), operation, request).await;
            }
            let mut prepared = prepared;
            loop {
                let result = self.executor
                    .read_code_index(selection.access.clone(), prepared.clone(), request.clone())
                    .await;
                if !matches!(&result, Err(Error::InvalidQuery(diagnostic)) if diagnostic.kind == "path_index_required") {
                    return result;
                }
                prepared = self.prepare_file_history(&selection, handle, prepared).await?;
            }
        }).await.map_err(|_| Error::Limit)?
    }
}

pub(crate) struct Selection {
    pub access: Arc<dyn local::Access>,
    _window: OwnedRwLockReadGuard<()>,
    _read: OwnedRwLockReadGuard<()>,
}
