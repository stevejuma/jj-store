//! Owned, bounded native graph queries. Upstream objects never cross this API.
pub(crate) mod access;
pub mod admission;
pub mod cache;
pub mod content;
mod graphs;
mod index;
pub mod language;
pub(crate) mod memory;
pub mod predicates;
pub mod preparation;
pub(crate) mod prepared;
mod projection;
pub mod workers;
use crate::{
    backend::BackendLimits,
    factories::{NativeStores, PROFILE},
    objects::NativeObjects,
    operation_stores::OperationLimits,
};
use access::{Budget, ReadOnlyAuthority, ReadStore};
use jj_lib::{config::StackedConfig, settings::UserSettings};
use jj_store_model as m;
use jj_store_object::ObjectStore;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, path::PathBuf, sync::Arc, time::Duration};
use workers::Workers;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, thiserror::Error, schemars::JsonSchema)]
#[schemars(rename = "GraphError")]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[error("query request was canceled")]
    Canceled,
    #[error("query is not authorized")]
    Denied,
    #[error("query object is not committed in this snapshot")]
    NotCommitted,
    #[error("repository epoch changed")]
    EpochChanged,
    #[error("query resource limit exceeded")]
    Limit,
    #[error("query cache or worker is busy")]
    Busy,
    #[error("native index exceeds the configured rebuild budget")]
    IndexBudget,
    #[error("native index requires an explicit preparation task")]
    IndexRebuildRequired,
    #[error("invalid query or missing query context")]
    Invalid,
    #[error("{0}")]
    InvalidQuery(language::Diagnostic),
    #[error("native query or cache is unavailable")]
    Unavailable,
    #[error("committed native object is corrupt")]
    Corrupt,
    #[error("committed native history is incomplete")]
    Incomplete,
    #[error("repository path is absent")]
    NotFound,
    #[error("repository path is not an unambiguous directory")]
    NotDirectory,
    #[error("file range is invalid or unsatisfiable")]
    Range,
}
pub type Result<T> = std::result::Result<T, Error>;
/// Implemented by a private service capability, not by request-deserialized data.
#[async_trait::async_trait]
pub trait ReadAccess: Send + Sync {
    fn repository(&self) -> m::RepositoryId;
    fn epoch(&self) -> m::EpochId;
    fn operation(&self) -> m::OperationId;
    /// Point membership must not require complete inventory enumeration.
    async fn contains(&self, key: m::ObjectKey) -> Result<bool>;
    /// Explicit complete enumeration under the capability's trusted bound.
    async fn inventory(&self) -> Result<Arc<BTreeSet<m::ObjectKey>>>;
    async fn authorize(&self) -> Result<()>;
}
#[derive(Clone, Copy)]
pub struct Limits {
    pub metadata: admission::Limits,
    pub predicates: predicates::Limits,
    pub max_page: usize,
    pub max_index_commits: usize,
    pub max_inline_rebuild_commits: usize,
    pub max_work: usize,
    pub max_object_bytes: u64,
    pub max_output_bytes: usize,
    pub max_revset_bytes: usize,
    pub timeout: Duration,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            metadata: admission::Limits::default(),
            predicates: predicates::Limits::default(),
            max_page: 500,
            max_index_commits: 4_000_000,
            max_inline_rebuild_commits: 1_000,
            max_work: 1_000_000,
            max_object_bytes: 256 * 1024 * 1024,
            max_output_bytes: 8 * 1024 * 1024,
            max_revset_bytes: 4096,
            timeout: Duration::from_secs(10),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "GraphRequest")]
#[serde(tag = "graph", rename_all = "snake_case", deny_unknown_fields)]
pub enum Request {
    Log {
        revset: String,
        workspace: Option<String>,
        user_email: String,
        at_time_millis: i64,
    },
    Operations,
    OperationsFiltered {
        query: String,
    },
    Evolution {
        commit: m::CommitId,
    },
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "GraphPageRequest")]
#[serde(deny_unknown_fields)]
pub struct Page {
    pub limit: usize,
    pub after: Option<m::ObjectKey>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "GraphTimestamp")]
pub struct Timestamp {
    pub millis: i64,
    pub offset_minutes: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "GraphSignature")]
pub struct Signature {
    pub name: String,
    pub email: String,
    pub timestamp: Timestamp,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "GraphBookmark")]
pub struct Bookmark {
    pub name: String,
    pub remote: Option<String>,
    pub added: bool,
    pub removed: bool,
    pub conflicted: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "GraphEdgeKind")]
#[serde(rename_all = "snake_case")]
pub enum EdgeKind {
    Direct,
    Indirect,
    Missing,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "GraphEdge")]
pub struct Edge {
    pub target: m::CommitId,
    pub kind: EdgeKind,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "GraphCommit")]
pub struct Commit {
    pub id: m::CommitId,
    pub change_id: m::ChangeId,
    pub parents: Vec<m::CommitId>,
    pub description: String,
    pub author: Signature,
    pub committer: Signature,
    pub tree_terms: Vec<m::TreeId>,
    pub conflicted: bool,
    pub bookmarks: Vec<Bookmark>,
    pub workspaces: Vec<String>,
    pub visible: bool,
    pub divergent: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "GraphCommitNode")]
pub struct CommitNode {
    pub commit: Commit,
    pub edges: Vec<Edge>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "GraphOperationNode")]
pub struct OperationNode {
    pub id: m::OperationId,
    pub parents: Vec<m::OperationId>,
    pub view: m::ViewId,
    pub description: String,
    pub user: String,
    pub host: String,
    pub start: Timestamp,
    pub end: Timestamp,
    pub is_snapshot: bool,
    pub workspace: Option<String>,
    pub attributes: std::collections::BTreeMap<String, String>,
    pub predecessor_data_present: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "GraphEvolutionNode")]
pub struct EvolutionNode {
    pub commit: Commit,
    pub operation: Option<m::OperationId>,
    pub predecessors: Vec<m::CommitId>,
    pub predecessors_known: bool,
    pub legacy_predecessors: Vec<m::CommitId>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "GraphNodes")]
#[serde(tag = "graph", content = "nodes", rename_all = "snake_case")]
pub enum Nodes {
    Log(Vec<CommitNode>),
    Operations(Vec<OperationNode>),
    Evolution(Vec<EvolutionNode>),
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "GraphStats")]
pub struct Stats {
    pub metadata_nodes: u64,
    pub predicate_file_bytes: u64,
    pub predicate_lines: u64,
    pub object_reads: u64,
    pub object_bytes: u64,
    pub work: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "GraphPage")]
pub struct GraphPage {
    pub version: u32,
    pub cache: cache::Report,
    pub operation: m::OperationId,
    pub data: Nodes,
    pub next: Option<m::ObjectKey>,
    pub stats: Stats,
}
/// Cancellation is separate from dropping a request: a task owner can signal it
/// and still await actual worker exit before reporting completion.
#[derive(Clone, Default)]
pub struct Cancellation(tokio_util::sync::CancellationToken);
impl Cancellation {
    pub(crate) fn token(&self) -> tokio_util::sync::CancellationToken {
        self.0.clone()
    }
    pub async fn cancelled(&self) {
        self.0.cancelled().await;
    }
    pub fn is_cancelled(&self) -> bool {
        self.0.is_cancelled()
    }
    pub fn cancel(&self) {
        self.0.cancel();
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "IndexReport")]
pub struct IndexReport {
    pub version: u32,
    pub operation: m::OperationId,
    pub cache: cache::Report,
    pub stats: Stats,
}
enum Work {
    Graph(Request, Page),
    Prepare,
}
#[derive(Serialize)]
#[serde(untagged)]
enum Output {
    Graph(GraphPage),
    Index(IndexReport),
}
/// Bounded jobs and operation-specific local caches; no physical working copy.
#[derive(Clone)]
pub struct GraphExecutor {
    root: PathBuf,
    limits: Limits,
    preparation: preparation::Limits,
    workers: Workers,
    cache_policy: cache::Policy,
    residents: Arc<cache::resident::Residents>,
}
impl GraphExecutor {
    pub fn new(root: PathBuf, limits: Limits, concurrency: usize) -> Result<Self> {
        Self::with_cache_policy(root, limits, concurrency, cache::Policy::default())
    }
    pub fn with_cache_policy(
        root: PathBuf,
        limits: Limits,
        concurrency: usize,
        cache_policy: cache::Policy,
    ) -> Result<Self> {
        Self::with_workers(root, limits, cache_policy, Workers::new(concurrency)?)
    }
    pub fn with_workers(
        root: PathBuf,
        limits: Limits,
        cache_policy: cache::Policy,
        workers: Workers,
    ) -> Result<Self> {
        cache_policy.validate()?;
        limits.validate()?;
        if !root.is_absolute() {
            return Err(Error::Limit);
        }
        Ok(Self {
            root,
            limits,
            preparation: Default::default(),
            workers,
            cache_policy,
            residents: Arc::default(),
        })
    }
    pub async fn execute(
        &self,
        store: Arc<dyn ObjectStore>,
        access: Arc<dyn ReadAccess>,
        request: Request,
        page: Page,
    ) -> Result<GraphPage> {
        match self
            .execute_work(
                store,
                access,
                Work::Graph(request, page),
                Cancellation::default(),
            )
            .await?
        {
            Output::Graph(page) => Ok(page),
            Output::Index(_) => Err(Error::Unavailable),
        }
    }
    pub fn with_preparation_limits(mut self, limits: preparation::Limits) -> Result<Self> {
        limits.validate()?;
        self.preparation = limits;
        Ok(self)
    }
    pub async fn prepare_index(
        &self,
        store: Arc<dyn ObjectStore>,
        access: Arc<dyn ReadAccess>,
        cancellation: Cancellation,
    ) -> Result<IndexReport> {
        match self
            .execute_work(store, access, Work::Prepare, cancellation)
            .await?
        {
            Output::Index(report) => Ok(report),
            Output::Graph(_) => Err(Error::Unavailable),
        }
    }
    async fn execute_work(
        &self,
        store: Arc<dyn ObjectStore>,
        access: Arc<dyn ReadAccess>,
        work: Work,
        cancellation: Cancellation,
    ) -> Result<Output> {
        let limits = match &work {
            Work::Prepare => self.preparation.budget(self.limits),
            Work::Graph(..) => self.limits,
        };
        let mut budget = Budget::new(limits)?;
        budget.cancellation = cancellation.0;
        let budget = Arc::new(budget);
        let permit = match &work {
            Work::Prepare => self.workers.acquire_preparation()?,
            Work::Graph(..) => self.workers.acquire()?,
        };
        budget.authorize(access.as_ref()).await?;
        if let Work::Graph(_, page) = &work
            && (page.limit == 0 || page.limit > self.limits.max_page)
        {
            return Err(Error::Limit);
        }
        let cancellation = budget.cancellation.clone().drop_guard();
        let executor = self.clone();
        let runtime = tokio::runtime::Handle::current();
        // The permit stays with the blocking job if the caller cancels its future.
        let result = tokio::task::spawn_blocking(move || {
            let _permit = permit;
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                let result = budget
                    .until(executor.run(store, access.clone(), work, budget.clone()))
                    .await?;
                budget.authorize(access.as_ref()).await?;
                match result {
                    Ok(value) => Ok(value),
                    Err(error) => Err(budget.failure().unwrap_or(error)),
                }
            }))
        })
        .await
        .map_err(|_| Error::Unavailable)?;
        cancellation.disarm();
        result
    }
    async fn run(
        &self,
        store: Arc<dyn ObjectStore>,
        access: Arc<dyn ReadAccess>,
        work: Work,
        budget: Arc<Budget>,
    ) -> Result<Output> {
        budget.check()?;
        if let Work::Graph(request, page) = &work {
            let query = match request {
                Request::Operations => Some(""),
                Request::OperationsFiltered { query } => Some(query.as_str()),
                _ => None,
            };
            if let Some(query) = query
                && !language::operations::Filter::parse(query)?.needs_prefix_lookup()
            {
                graphs::validate(request, page, &budget)?;
                let objects = Arc::new(
                    NativeObjects::new(
                        Arc::new(ReadStore::new(store, access.clone(), budget.clone(), false)),
                        access.repository(),
                        self.limits.max_object_bytes,
                    )
                    .map_err(|_| Error::Limit)?,
                );
                let authority = Arc::new(ReadOnlyAuthority {
                    access: access.clone(),
                    budget: budget.clone(),
                });
                let operations: Arc<dyn jj_lib::op_store::OpStore> = Arc::new(
                    crate::operation_stores::NativeOpStore::new(
                        objects,
                        authority,
                        OperationLimits::default(),
                    )
                    .map_err(|_| Error::Unavailable)?,
                );
                let id = jj_lib::op_store::OperationId::from_bytes(access.operation().as_bytes());
                let data = operations
                    .read_operation(&id)
                    .await
                    .map_err(|_| Error::Unavailable)?;
                let head = jj_lib::operation::Operation::new(operations, id, data);
                let (data, next) =
                    graphs::run_operations(&head, request.clone(), page.clone(), &access, &budget)
                        .await?;
                let result = Output::Graph(GraphPage {
                    version: 1,
                    operation: access.operation(),
                    data,
                    next,
                    cache: cache::Report {
                        state: cache::State::NotUsed,
                        evicted: 0,
                        retained_entries: 0,
                        retained_bytes: 0,
                    },
                    stats: budget.stats(),
                });
                budget.complete_output(&result)?;
                return Ok(result);
            }
        }
        if let Work::Graph(request, page) = &work {
            graphs::validate(request, page, &budget)?;
        }
        let objects = Arc::new(
            NativeObjects::new(
                Arc::new(ReadStore::new(store, access.clone(), budget.clone(), true)),
                access.repository(),
                self.limits.max_object_bytes,
            )
            .map_err(|_| Error::Limit)?,
        );
        let authority = Arc::new(ReadOnlyAuthority {
            access: access.clone(),
            budget: budget.clone(),
        });
        let stores = NativeStores::new(
            objects,
            authority,
            BackendLimits::default(),
            OperationLimits::default(),
        )
        .map_err(|_| Error::Unavailable)?;
        let settings = UserSettings::from_config(StackedConfig::with_defaults())
            .map_err(|_| Error::Unavailable)?;
        let mut cache = match cache::Cache::open(&self.root, self.cache_policy, &budget) {
            Ok(cache) => cache,
            Err(Error::Busy) if matches!(&work, Work::Graph(Request::Log { revset, .. }, _) if revset.trim() == "all()") =>
            {
                let saved = self
                    .residents
                    .get_memory(access.as_ref())
                    .ok_or(Error::Busy)?;
                budget.start_index(false)?;
                // Construct fresh request-scoped native capabilities. The saved
                // index carries no previous caller's authorization or backend.
                let loader =
                    index::resident_loader(&stores, &settings, saved.clone(), budget.clone())?;
                let repo = stores
                    .load_at(&loader, access.operation())
                    .await
                    .map_err(|_| Error::Unavailable)?;
                budget.finish_initial_index();
                let Work::Graph(request, page) = work else {
                    unreachable!()
                };
                let (data, next) = graphs::run(&repo, request, page, &access, &budget).await?;
                budget.authorize(access.as_ref()).await?;
                let result = Output::Graph(GraphPage {
                    version: 1,
                    cache: saved.report(),
                    operation: access.operation(),
                    data,
                    next,
                    stats: budget.stats(),
                });
                budget.complete_output(&result)?;
                return Ok(result);
            }
            Err(error) => return Err(error),
        };
        let entry = cache.prepare(access.as_ref(), &stores, &budget, &self.residents)?;
        if matches!(work, Work::Graph(..)) {
            budget.start_index(entry.initialize)?;
        }
        if entry.initialize {
            stores
                .initialize_cache(&settings, &entry.path)
                .await
                .map_err(|_| Error::Unavailable)?;
        }
        let preparation_bytes = if matches!(work, Work::Prepare) && entry.initialize {
            Some(cache.remaining_bytes(&entry, &budget)?)
        } else {
            None
        };
        let loader = index::loader(
            &stores,
            &settings,
            &entry.path,
            budget.clone(),
            preparation_bytes,
            entry.resident.clone(),
            access.clone(),
            matches!(work, Work::Prepare),
        )?;
        let repo = stores
            .load_at(&loader, access.operation())
            .await
            .map_err(|_| Error::Unavailable)?;
        budget.check()?;
        budget.finish_initial_index();
        if entry.resident.is_some()
            && matches!(&work, Work::Graph(Request::Log { revset, .. }, _) if revset.trim() == "all()")
        {
            // This exact expression only consults the already-loaded index and
            // view. Finish cache bookkeeping before releasing the namespace;
            // the loader retains the immutable resident until the query exits.
            let index = repo
                .readonly_index()
                .downcast_ref::<projection::Reader>()
                .ok_or(Error::Unavailable)?
                .prepared
                .clone();
            let report = cache.finish(entry, access.as_ref(), &budget, index, &self.residents)?;
            drop(cache);
            let Work::Graph(request, page) = work else {
                unreachable!()
            };
            let (data, next) = graphs::run(&repo, request, page, &access, &budget).await?;
            budget.authorize(access.as_ref()).await?;
            let result = Output::Graph(GraphPage {
                version: 1,
                cache: report,
                operation: access.operation(),
                data,
                next,
                stats: budget.stats(),
            });
            budget.complete_output(&result)?;
            return Ok(result);
        }
        let data = match work {
            Work::Graph(request, page) => {
                Some(graphs::run(&repo, request, page, &access, &budget).await?)
            }
            Work::Prepare => None,
        };
        budget.check()?;
        budget.authorize(access.as_ref()).await?;
        // Drop native loaders before replacing/removing their derived files.
        let resident_index = repo
            .readonly_index()
            .downcast_ref::<projection::Reader>()
            .ok_or(Error::Unavailable)?
            .prepared
            .clone();
        drop(repo);
        drop(loader);
        let cache = cache.finish(
            entry,
            access.as_ref(),
            &budget,
            resident_index,
            &self.residents,
        )?;
        let result = match data {
            Some((data, next)) => Output::Graph(GraphPage {
                version: 1,
                cache,
                operation: access.operation(),
                data,
                next,
                stats: budget.stats(),
            }),
            None => Output::Index(IndexReport {
                version: 1,
                cache,
                operation: access.operation(),
                stats: budget.stats(),
            }),
        };
        budget.complete_output(&result)?;
        Ok(result)
    }
}

impl Limits {
    pub(crate) fn validate(&self) -> Result<()> {
        self.metadata.validate()?;
        self.predicates.validate()?;
        if self.max_page == 0
            || self.max_page > 10_000
            || self.max_index_commits == 0
            || self.max_index_commits > 4_000_000
            || self.max_work == 0
            || self.max_object_bytes == 0
            || self.max_output_bytes == 0
            || self.max_revset_bytes == 0
            || self.max_revset_bytes > 16_384
            || self.timeout.is_zero()
            || self.timeout > Duration::from_secs(300)
        {
            return Err(Error::Limit);
        }
        Ok(())
    }
}
