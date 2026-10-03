//! Operation-pinned ordinary GitBackend reads. No checkout or publication methods.
mod budget;
pub mod conflicts;
pub mod diff;
mod diff_copies;
pub mod diff_summary;
pub mod dto;
mod graph_types;
mod graphs;
mod native;
pub mod operation_changes;
pub mod operations;
pub mod paths;
pub mod references;
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
    repo::{ReadonlyRepo, Repo as _, RepoLoader},
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
    QueryInfo,
    Commit { commit: m::GitSha1Id },
    File { file: m::GitSha1Id },
    Paths(paths::Request),
    References(references::Request),
    Operation { operation: m::OperationId },
    OperationDiff(operations::Request),
    OperationChanges(operation_changes::Request),
    Diff(diff::Request),
    DiffPaths(diff_summary::Request),
    Conflicts(conflicts::Request),
}
#[derive(Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalCodeResponse")]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
pub enum Response {
    Graph(GraphPage),
    QueryInfo(crate::query::language::QueryInfo),
    Commit(Box<CommitDetail>),
    File(FileContent),
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
                let context = Context::open(access, operation, budget, limits).await?;
                let value = match request {
                    Request::QueryInfo => Response::QueryInfo(crate::query::language::info(true)),
                    Request::Graph(request, page) => {
                        graphs::validate(
                            &request,
                            &page,
                            context.access.commits.len(),
                            &context.budget,
                        )?;
                        let (data, next) = graphs::run(
                            &context.repo,
                            request,
                            page,
                            &context.access,
                            &context.budget,
                        )
                        .await?;
                        Response::Graph(GraphPage {
                            version: 1,
                            operation,
                            data,
                            next,
                            stats: context.budget.stats(),
                        })
                    }
                    Request::Commit { commit } => {
                        let source = context.commit_record(commit).await?;
                        let native = context
                            .commit(&context.repo.store().clone(), commit)
                            .await?;
                        let card = graphs::describe(
                            &context.repo,
                            &native,
                            &context.access,
                            &context.budget,
                        )
                        .await?;
                        let object =
                            context.object(ObjectKey::Commit(commit), ObjectData::Commit(source));
                        Response::Commit(Box::new(CommitDetail {
                            object,
                            commit: card,
                        }))
                    }
                    Request::File { file } => Response::File(context.file_content(file).await?),
                    Request::Paths(request) => {
                        Response::Paths(paths::read(context, request).await?)
                    }
                    Request::References(request) => {
                        Response::References(references::read(context, request).await?)
                    }
                    Request::Operation { operation } => {
                        let data = context.operation(operation).await?;
                        let view = context.view(data.view_id).await?;
                        let result = OperationDetail {
                            view: context
                                .object(ObjectKey::View(data.view_id), ObjectData::View(view)),
                            operation: context.object(
                                ObjectKey::Operation(operation),
                                ObjectData::Operation(data),
                            ),
                        };
                        Response::Operation(Box::new(result))
                    }
                    Request::OperationDiff(request) => {
                        Response::OperationDiff(operations::read(context, request).await?)
                    }
                    Request::OperationChanges(request) => {
                        Response::OperationChanges(operation_changes::read(context, request).await?)
                    }
                    Request::Diff(request) => Response::Diff(diff::read(context, request).await?),
                    Request::DiffPaths(request) => {
                        Response::DiffPaths(diff_summary::read(context, request).await?)
                    }
                    Request::Conflicts(request) => {
                        Response::Conflicts(conflicts::read(context, request).await?)
                    }
                };
                Ok(value)
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
        access.authorize().await?;
        let budget = Arc::new(Budget::new(self.limits));
        let cancel = budget.cancellation.clone().drop_guard();
        let runtime = tokio::runtime::Handle::current();
        let result = tokio::task::spawn_blocking(move || {
            let _permit = permit;
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                let value = budget.until(run(access.clone(), budget.clone())).await??;
                access.authorize().await?;
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
fn loader(path: &Path) -> Result<RepoLoader> {
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
}
impl Pin {
    fn repository(&self) -> m::RepositoryId {
        self.binding.repository()
    }
    fn generation(&self) -> u64 {
        self.binding.generation()
    }
    fn inventory(&self) -> &BTreeSet<ObjectKey> {
        &self.inventory
    }
}
struct Context {
    repo: Arc<ReadonlyRepo>,
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
    ) -> Result<Self> {
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
        while let Some(id) = pending.pop_first() {
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
        };
        let mut pending = BTreeSet::from([operation]);
        let mut commits = BTreeSet::new();
        while let Some(id) = pending.pop_first() {
            budget.work(1)?;
            if pin.operations.contains_key(&id) {
                continue;
            }
            let (op, data) = available.get(&id).ok_or(Error::Incomplete)?;
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
            pending.extend(data.parents.iter().copied());
            pin.inventory.insert(ObjectKey::Operation(id));
            pin.inventory.insert(ObjectKey::View(data.view_id));
            pin.views.insert(data.view_id, view);
            pin.operations.insert(id, data.clone());
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
            repo,
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
        if self.access.inventory.contains(&key) {
            return Ok(());
        }
        Err(Error::NotCommitted)
    }
    fn code_store(&self) -> Result<Arc<jj_lib::store::Store>> {
        Ok(self.repo.store().clone())
    }
    async fn commit_record(&self, id: m::GitSha1Id) -> Result<dto::Commit> {
        self.member(ObjectKey::Commit(id)).await?;
        self.access
            .commits
            .get(&id)
            .cloned()
            .ok_or(Error::NotCommitted)
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
