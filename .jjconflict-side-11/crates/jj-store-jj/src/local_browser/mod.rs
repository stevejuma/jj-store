//! Operation-pinned ordinary GitBackend reads. No checkout or publication methods.
pub(crate) mod budget;
pub mod conflicts;
pub mod diff;
mod diff_copies;
pub mod diff_summary;
mod dispatch;
pub mod dto;
pub(crate) mod git_read;
mod graph_types;
mod graphs;
mod history;
mod native;
pub mod operation_changes;
mod operation_store;
pub mod operations;
pub mod paths;
pub mod preparation;
pub mod references;
mod scoped;
pub mod stacks;
pub mod working_tree;
pub use crate::query::Error;
pub use crate::query::content::{Direction, Limits};
use budget::Budget;
pub use graph_types::*;
use jj_lib::{
    backend as b,
    config::StackedConfig,
    default_backend_factories::default_backend_factories,
    object_id::ObjectId as _,
    op_store as o,
    repo::{ReadonlyRepo, RepoLoader},
    settings::UserSettings,
};
use jj_store_model as m;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    sync::Arc,
};
pub type Result<T> = std::result::Result<T, Error>;
pub const PROFILE: &str = "git_sha1_jj045";

/// Installation authorization and canonical identity proof, supplied by composition.
#[async_trait::async_trait]
pub trait Access: Send + Sync {
    fn repository(&self) -> m::RepositoryId;
    fn generation(&self) -> u64;
    fn metadata_path(&self) -> &Path;
    async fn authorize(&self) -> Result<()>;
}
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, schemars::JsonSchema,
)]
#[schemars(rename = "LocalCodeObjectKey")]
#[serde(tag = "kind", content = "id", rename_all = "snake_case")]
pub enum ObjectKey {
    Commit(m::GitSha1Id),
    Tree(m::GitSha1Id),
    File(m::GitSha1Id),
    Symlink(m::GitSha1Id),
    Operation(m::OperationId),
    View(m::ViewId),
}
#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalCodeHeads")]
pub struct Heads {
    pub version: u32,
    pub profile: &'static str,
    pub repository: m::RepositoryId,
    pub generation: u64,
    pub heads: Vec<m::OperationId>,
}
#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalCodeSnapshot")]
pub struct Snapshot {
    pub version: u32,
    pub profile: &'static str,
    pub repository: m::RepositoryId,
    pub generation: u64,
    pub operation: m::OperationId,
}
#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalCodeObjectData")]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
pub enum ObjectData {
    Commit(dto::Commit),
    Operation(dto::Operation),
    View(dto::View),
}
#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalCodeObject")]
pub struct Object {
    pub version: u32,
    pub profile: &'static str,
    pub key: ObjectKey,
    pub virtual_object: bool,
    pub data: ObjectData,
    pub stats: Stats,
}
#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalCommitDetail")]
pub struct CommitDetail {
    pub object: Object,
    pub commit: Commit,
}
#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalOperationDetail")]
pub struct OperationDetail {
    pub operation: Object,
    pub view: Object,
}
#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalFileContent")]
pub struct FileContent {
    pub bytes: usize,
    pub text: Option<String>,
    pub binary: bool,
    pub too_large: bool,
}

#[derive(Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "LocalCodeQuery")]
#[serde(
    tag = "kind",
    content = "query",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum Request {
    Graph(GraphRequest, GraphWindow),
    StackHistory(stacks::Request),
    QueryInfo,
    Commit {
        commit: m::GitSha1Id,
    },
    CommitSummary {
        commit: m::GitSha1Id,
    },
    File {
        file: m::GitSha1Id,
    },
    FileAt {
        commit: m::GitSha1Id,
        path: String,
        file: m::GitSha1Id,
    },
    Paths(paths::Request),
    References(references::Request),
    Operation {
        operation: m::OperationId,
    },
    OperationDiff(operations::Request),
    OperationChanges(operation_changes::Request),
    Diff(diff::Request),
    DiffPaths(diff_summary::Request),
    Conflicts(conflicts::Request),
}
impl Request {
    /// Query kinds with a qualified immutable prepared reader. This predicate
    /// also owns HTTP cold recovery; ordinary scoped reads remain independent.
    pub fn needs_prepared_index(&self) -> bool {
        matches!(
            self,
            Self::Commit { .. }
                | Self::Graph(GraphRequest::Log { .. } | GraphRequest::Evolution { .. }, _)
                | Self::StackHistory(_)
        ) || matches!(self, Self::References(r) if r.query.is_empty()
            && matches!(r.kind, references::Kind::Changes | references::Kind::All))
    }
    /// Ordinary scoped reads can reuse ready point membership while retaining
    /// their bounded cold reader. Bare file IDs have no selected tree proof.
    pub fn can_use_prepared_index(&self) -> bool {
        self.needs_prepared_index()
            || matches!(
                self,
                Self::CommitSummary { .. }
                    | Self::FileAt { .. }
                    | Self::Paths(_)
                    | Self::Diff(_)
                    | Self::DiffPaths(_)
                    | Self::Conflicts(_)
            )
    }
}
#[derive(Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalCodeResponse")]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
pub enum Response {
    Graph(GraphPage),
    StackHistory(stacks::Page),
    QueryInfo(crate::query::language::QueryInfo),
    Commit(Box<CommitDetail>),
    CommitSummary(Object),
    File(FileContent),
    FileAt(FileContent),
    Paths(paths::Page),
    References(references::Page),
    Operation(Box<OperationDetail>),
    OperationDiff(operations::Response),
    OperationChanges(operation_changes::Page),
    Diff(diff::Response),
    DiffPaths(diff_summary::Page),
    Conflicts(conflicts::Response),
}
#[derive(Clone)]
pub struct Executor {
    workers: crate::query::workers::Workers,
    limits: Limits,
}
impl Executor {
    pub fn new(concurrency: usize) -> Result<Self> {
        Ok(Self {
            workers: crate::query::workers::Workers::new(concurrency)?,
            limits: Limits::default(),
        })
    }
    pub fn with_workers(workers: crate::query::workers::Workers, limits: Limits) -> Self {
        Self { workers, limits }
    }
    pub async fn heads(&self, access: Arc<dyn Access>) -> Result<Heads> {
        self.worker(access, move |access, budget| {
            Box::pin(async move {
                let loader = loader(access.metadata_path())?;
                let heads = loader
                    .op_heads_store()
                    .get_op_heads()
                    .await
                    .map_err(|_| Error::Unavailable)?;
                budget.work(heads.len())?;
                if heads.len() > 64 {
                    return Err(Error::Limit);
                }
                let mut heads = heads.iter().map(operation_id).collect::<Result<Vec<_>>>()?;
                heads.sort();
                let value = Heads {
                    version: 1,
                    profile: PROFILE,
                    repository: access.repository(),
                    generation: access.generation(),
                    heads,
                };
                budget.complete_output(&value)?;
                Ok(value)
            })
        })
        .await
    }
    pub async fn read(
        &self,
        access: Arc<dyn Access>,
        operation: m::OperationId,
        request: Request,
    ) -> Result<Response> {
        let limits = self.limits;
        self.worker(access, move |access, budget| {
            Box::pin(async move {
                let operation_only = match &request {
                    Request::Graph(GraphRequest::Operations, _) => true,
                    Request::Graph(GraphRequest::OperationsFiltered {query}, _) => !crate::query::language::operations::Filter::parse(query)?.needs_prefix_lookup(),
                    _ => false,
                };
                let metadata_only = operation_only || matches!(&request, Request::QueryInfo)
                    || matches!(&request, Request::References(r) if matches!(r.kind, references::Kind::Bookmarks | references::Kind::Workspaces) && r.operation == operation)
                    || matches!(&request, Request::Operation { operation: selected } if *selected == operation);
                let scoped_commits = match &request {
                    Request::Paths(r) => vec![r.commit],
                    Request::FileAt { commit, .. } | Request::CommitSummary { commit } => vec![*commit],
                    Request::Conflicts(r) => vec![r.commit],
                    Request::Diff(r) => vec![r.from, r.to],
                    Request::DiffPaths(r) => vec![r.from, r.to],
                    _ => vec![],
                };
                let context = Context::open(access, operation, budget, limits, metadata_only, scoped_commits, operation_only).await?;
                context.dispatch(operation, request, operation_only).await
            })
        })
        .await
    }
    async fn worker<T: Send + Serialize + 'static>(
        &self,
        access: Arc<dyn Access>,
        run: impl FnOnce(
            Arc<dyn Access>,
            Arc<Budget>,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T>>>>
        + Send
        + 'static,
    ) -> Result<T> {
        let permit = self.workers.acquire()?;
        let budget = Arc::new(Budget::new(self.limits));
        let cancel = budget.cancellation.clone().drop_guard();
        budget.until(access.authorize()).await??;
        let runtime = tokio::runtime::Handle::current();
        let result = tokio::task::spawn_blocking(move || {
            let _permit = permit;
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                let value = budget.until(run(access.clone(), budget.clone())).await??;
                budget.until(access.authorize()).await??;
                budget.complete_output(&value)?;
                Ok(value)
            }))
        })
        .await
        .map_err(|_| Error::Unavailable)?;
        cancel.disarm();
        result
    }
}
pub(crate) fn loader(path: &Path) -> Result<RepoLoader> {
    let settings = UserSettings::from_config(StackedConfig::with_defaults())
        .map_err(|_| Error::Unavailable)?;
    if std::fs::read_to_string(path.join("store/type"))
        .map_err(|_| Error::Unavailable)?
        .trim()
        != "git"
    {
        return Err(Error::Invalid);
    }
    if std::fs::read_to_string(path.join("op_store/type"))
        .map_err(|_| Error::Unavailable)?
        .trim()
        != jj_lib::simple_op_store::SimpleOpStore::name()
    {
        return Err(Error::Invalid);
    }
    RepoLoader::init_from_file_system(&settings, path, &default_backend_factories())
        .map_err(|_| Error::Unavailable)
}
fn git_id(id: &impl jj_lib::object_id::ObjectId) -> Result<m::GitSha1Id> {
    m::GitSha1Id::from_slice(id.as_bytes()).map_err(|_| Error::Corrupt)
}
fn operation_id(id: &o::OperationId) -> Result<m::OperationId> {
    m::OperationId::from_slice(id.as_bytes()).map_err(|_| Error::Corrupt)
}
struct Pin {
    binding: Arc<dyn Access>,
    commits: BTreeMap<m::GitSha1Id, dto::Commit>,
    operations: BTreeMap<m::OperationId, dto::Operation>,
    views: BTreeMap<m::ViewId, dto::View>,
    inventory: BTreeSet<ObjectKey>,
    prepared: Option<Arc<preparation::Prepared>>,
}
impl Pin {
    fn repository(&self) -> m::RepositoryId {
        self.binding.repository()
    }
    fn generation(&self) -> u64 {
        self.binding.generation()
    }
    async fn contains_commit(&self, id: m::GitSha1Id) -> Result<bool> {
        if self.inventory.contains(&ObjectKey::Commit(id)) {
            return Ok(true);
        }
        match &self.prepared {
            Some(prepared) => prepared.contains(id).await,
            None => Ok(false),
        }
    }
}
struct Context {
    repo: Option<Arc<ReadonlyRepo>>,
    store: Arc<jj_lib::store::Store>,
    scoped: Option<Arc<scoped::Guard>>,
    access: Arc<Pin>,
    budget: Arc<Budget>,
    limits: Limits,
}
impl Context {
    async fn open(
        binding: Arc<dyn Access>,
        operation: m::OperationId,
        budget: Arc<Budget>,
        limits: Limits,
        metadata_only: bool,
        scoped_commits: Vec<m::GitSha1Id>,
        operation_only: bool,
    ) -> Result<Self> {
        let metadata_only = metadata_only || !scoped_commits.is_empty();
        let loader = loader(binding.metadata_path())?;
        // Only operations in the current retained operation ancestry may be selected.
        let heads = loader
            .op_heads_store()
            .get_op_heads()
            .await
            .map_err(|_| Error::Unavailable)?;
        if heads.len() > 64 {
            return Err(Error::Limit);
        }
        let mut pending: BTreeSet<_> = heads.iter().map(operation_id).collect::<Result<_>>()?;
        let mut available = BTreeMap::new();
        while let Some(id) = if metadata_only && pending.remove(&operation) {
            Some(operation)
        } else {
            pending.pop_first()
        } {
            budget.work(1)?;
            if available.contains_key(&id) {
                continue;
            }
            if available.len() >= limits.max_history_visits {
                return Err(Error::Limit);
            }
            bounded_op_file(
                binding.metadata_path(),
                "operations",
                &id.to_string(),
                &budget,
            )?;
            let op = loader
                .load_operation(&o::OperationId::from_bytes(id.as_bytes()))
                .await
                .map_err(|_| Error::NotFound)?;
            let data =
                dto::Operation::try_from(op.store_operation()).map_err(|_| Error::Corrupt)?;
            budget.metadata(&data)?;
            pending.extend(data.parents.iter().copied());
            available.insert(id, (op, data));
            if metadata_only && id == operation {
                break;
            }
        }
        let selected = available
            .get(&operation)
            .ok_or(Error::NotCommitted)?
            .0
            .clone();
        let mut pin = Pin {
            binding,
            commits: BTreeMap::new(),
            operations: BTreeMap::new(),
            views: BTreeMap::new(),
            inventory: BTreeSet::new(),
            prepared: None,
        };
        let mut pending = BTreeSet::from([operation]);
        let mut commits = BTreeSet::new();
        while let Some(id) = pending.pop_first() {
            budget.work(1)?;
            if pin.operations.contains_key(&id) {
                continue;
            }
            let (op, data) = available.get(&id).ok_or(Error::Incomplete)?;
            if operation_only {
                pin.inventory.insert(ObjectKey::Operation(id));
                pin.operations.insert(id, data.clone());
                continue;
            }
            bounded_op_file(
                pin.binding.metadata_path(),
                "views",
                &data.view_id.to_string(),
                &budget,
            )?;
            let view = dto::View::try_from(
                op.view()
                    .await
                    .map_err(|_| Error::Unavailable)?
                    .store_view(),
            )
            .map_err(|_| Error::Corrupt)?;
            budget.metadata(&view)?;
            commits.extend(view.head_ids.iter().copied());
            commits.extend(view.wc_commit_ids.values().copied());
            for target in view
                .local_bookmarks
                .values()
                .chain(view.local_tags.values())
                .chain(view.git_refs.values())
                .chain(view.git_heads.values())
            {
                commits.extend(target.0.iter().flatten().copied());
            }
            for remote in view.remote_views.values() {
                for value in remote.bookmarks.values().chain(remote.tags.values()) {
                    commits.extend(value.target.0.iter().flatten().copied());
                }
            }
            if let Some(map) = &data.commit_predecessors {
                for (commit, prior) in map {
                    commits.insert(*commit);
                    commits.extend(prior.iter().copied());
                }
            }
            if !metadata_only {
                pending.extend(data.parents.iter().copied());
            }
            pin.inventory.insert(ObjectKey::Operation(id));
            pin.inventory.insert(ObjectKey::View(data.view_id));
            pin.views.insert(data.view_id, view);
            pin.operations.insert(id, data.clone());
        }
        if metadata_only {
            // View references prove these targets without reading their ancestry.
            pin.inventory
                .extend(commits.into_iter().map(ObjectKey::Commit));
            let scoped = (!operation_only)
                .then(|| scoped::Guard::new(loader.store().clone(), &pin, budget.clone(), limits));
            if let Some(guard) = &scoped {
                for commit in scoped_commits {
                    history::prove_commit(commit, &selected, &loader, &pin, guard, &budget, limits)
                        .await?;
                }
            }
            let store = scoped
                .as_ref()
                .map_or_else(|| loader.store().clone(), |guard| guard.store());
            return Ok(Self {
                repo: None,
                store,
                scoped,
                access: Arc::new(pin),
                budget,
                limits,
            });
        }
        // Preflight closure before any graph/index query. Virtual root remains upstream-owned.
        let backend = loader.store().backend();
        while let Some(id) = commits.pop_first() {
            budget.work(1)?;
            if pin.commits.contains_key(&id) {
                continue;
            }
            if pin.commits.len() >= limits.max_history_visits {
                return Err(Error::Limit);
            }
            bounded_git_object(backend, id, gix::object::Kind::Commit, &budget)?;
            let data = backend
                .read_commit(&b::CommitId::from_bytes(id.as_bytes()))
                .await
                .map_err(native::map_backend)?;
            let wire = dto::Commit::try_from(&data).map_err(|_| Error::Corrupt)?;
            budget.metadata(&wire)?;
            commits.extend(wire.parents.iter().copied());
            commits.extend(wire.predecessors.iter().copied());
            pin.inventory.insert(ObjectKey::Commit(id));
            pin.commits.insert(id, wire);
        }
        let mut pending: BTreeSet<_> = pin
            .commits
            .values()
            .flat_map(|c| c.root_tree.iter().copied())
            .collect();
        let mut visited = BTreeSet::new();
        while let Some(id) = pending.pop_first() {
            budget.work(1)?;
            if !visited.insert(id) {
                continue;
            }
            pin.inventory.insert(ObjectKey::Tree(id));
            bounded_git_object(backend, id, gix::object::Kind::Tree, &budget)?;
            let tree = backend
                .read_tree(
                    jj_lib::repo_path::RepoPath::root(),
                    &b::TreeId::from_bytes(id.as_bytes()),
                )
                .await
                .map_err(native::map_backend)?;
            for entry in tree.entries() {
                budget.work(1)?;
                let object = match entry.value() {
                    b::TreeValue::File { id, .. } => Some(ObjectKey::File(git_id(id)?)),
                    b::TreeValue::Symlink(id) => Some(ObjectKey::Symlink(git_id(id)?)),
                    b::TreeValue::Tree(id) => {
                        let id = git_id(id)?;
                        pending.insert(id);
                        Some(ObjectKey::Tree(id))
                    }
                    _ => None,
                };
                if let Some(object) = object {
                    pin.inventory.insert(object);
                }
            }
        }
        let repo = loader
            .load_at(&selected)
            .await
            .map_err(|_| Error::Unavailable)?;
        Ok(Self {
            repo: Some(repo),
            store: loader.store().clone(),
            scoped: None,
            access: Arc::new(pin),
            budget,
            limits,
        })
    }
    fn object(&self, key: ObjectKey, data: ObjectData) -> Object {
        Object {
            version: 1,
            profile: PROFILE,
            key,
            virtual_object: matches!(key, ObjectKey::Commit(id) if id.as_bytes().iter().all(|b| *b == 0)),
            data,
            stats: self.budget.stats(),
        }
    }
    async fn member(&self, key: ObjectKey) -> Result<()> {
        self.budget.authorize(self.access.as_ref()).await?;
        if self.access.inventory.contains(&key)
            || self
                .scoped
                .as_ref()
                .is_some_and(|guard| guard.contains(key))
        {
            return Ok(());
        }
        if let ObjectKey::Commit(id) = key
            && self.access.contains_commit(id).await?
        {
            return Ok(());
        }
        Err(Error::NotCommitted)
    }
    fn code_store(&self) -> Result<Arc<jj_lib::store::Store>> {
        Ok(self.store.clone())
    }
    async fn commit_record(&self, id: m::GitSha1Id) -> Result<dto::Commit> {
        self.member(ObjectKey::Commit(id)).await?;
        if let Some(data) = self.access.commits.get(&id) {
            return Ok(data.clone());
        }
        if self.scoped.is_some() {
            let commit = self
                .store
                .get_commit_async(&b::CommitId::from_bytes(id.as_bytes()))
                .await
                .map_err(native::map_backend)?;
            return dto::Commit::try_from(commit.store_commit().as_ref())
                .map_err(|_| Error::Corrupt);
        }
        bounded_git_object(
            self.store.backend(),
            id,
            gix::object::Kind::Commit,
            &self.budget,
        )?;
        let data = self
            .store
            .backend()
            .read_commit(&b::CommitId::from_bytes(id.as_bytes()))
            .await
            .map_err(native::map_backend)?;
        let data = dto::Commit::try_from(&data).map_err(|_| Error::Corrupt)?;
        self.budget.metadata(&data)?;
        Ok(data)
    }
    async fn operation(&self, id: m::OperationId) -> Result<dto::Operation> {
        self.member(ObjectKey::Operation(id)).await?;
        self.access
            .operations
            .get(&id)
            .cloned()
            .ok_or(Error::NotCommitted)
    }
    async fn view(&self, id: m::ViewId) -> Result<dto::View> {
        self.member(ObjectKey::View(id)).await?;
        self.access
            .views
            .get(&id)
            .cloned()
            .ok_or(Error::NotCommitted)
    }
}

fn bounded_op_file(path: &Path, directory: &str, id: &str, budget: &Budget) -> Result<()> {
    if id.bytes().all(|b| b == b'0') {
        return Ok(());
    }
    let file = path.join("op_store").join(directory).join(id);
    let info = std::fs::symlink_metadata(file).map_err(|_| Error::Incomplete)?;
    if !info.is_file() || info.file_type().is_symlink() || info.len() > 4 * 1024 * 1024 {
        return Err(Error::Limit);
    }
    budget.read_bytes(info.len() as usize)
}
fn bounded_git_object(
    backend: &dyn b::Backend,
    id: m::GitSha1Id,
    kind: gix::object::Kind,
    budget: &Budget,
) -> Result<()> {
    if id.as_bytes().iter().all(|b| *b == 0) {
        return Ok(());
    }
    let repo = backend
        .downcast_ref::<jj_lib::git_backend::GitBackend>()
        .ok_or(Error::Invalid)?
        .git_repo();
    let header = repo
        .find_header(gix::ObjectId::from_bytes_or_panic(id.as_bytes()))
        .map_err(|_| Error::Incomplete)?;
    if header.kind() != kind {
        return Err(Error::Corrupt);
    }
    if header.size() > 4 * 1024 * 1024 {
        return Err(Error::Limit);
    }
    budget.read_bytes(header.size() as usize)
}
