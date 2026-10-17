//! Owned, bounded native graph queries. Upstream objects never cross this API.
pub use crate::read_support::{Cancellation, Error, GraphLimits as Limits, Result, Stats};
pub(crate) mod access;
pub mod admission;
pub mod blame;
#[cfg(test)]
pub(crate) mod blame_probe;
pub mod cache;
pub mod code_index;
pub mod content;
pub mod copy_index;
mod graphs;
mod index;
pub mod language;
pub(crate) mod memory;
mod operation_neighbors;
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

/// Exact identity projections; unrelated object kinds are never result rows.
#[derive(Clone, Copy)]
pub enum InventoryKind {
    Operations,
    Copies,
}
impl InventoryKind {
    pub fn matches(self, key: m::ObjectKey) -> bool {
        matches!(
            (self, key),
            (Self::Operations, m::ObjectKey::Operation(_)) | (Self::Copies, m::ObjectKey::Copy(_))
        )
    }
}
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
    /// Optional selected copy-family projection. Required seeds must be selected;
    /// optional seeds can be acknowledged staged ancestors in the caller.
    async fn copy_family(
        &self,
        _seeds: &BTreeSet<m::CopyId>,
        _required: &BTreeSet<m::CopyId>,
        _limits: copy_index::LookupLimits,
        _deadline: tokio::time::Instant,
    ) -> Result<Option<copy_index::Family>> {
        Ok(None)
    }
    /// Complete matching IDs at this capability's pin, within caller bounds.
    /// Compatibility providers retain their own full-inventory admission.
    async fn inventory_kind(
        &self,
        kind: InventoryKind,
        limit: usize,
        deadline: tokio::time::Instant,
    ) -> Result<BTreeSet<m::ObjectKey>> {
        if limit == 0 || tokio::time::Instant::now() >= deadline {
            return Err(Error::Limit);
        }
        let ids = tokio::time::timeout_at(deadline, async {
            self.authorize().await?;
            let inventory = self.inventory().await?;
            let mut ids = BTreeSet::new();
            for key in inventory.iter().copied() {
                if tokio::time::Instant::now() >= deadline {
                    return Err(Error::Limit);
                }
                if kind.matches(key) {
                    if ids.len() == limit {
                        return Err(Error::Limit);
                    }
                    ids.insert(key);
                }
            }
            self.authorize().await?;
            Ok(ids)
        })
        .await
        .map_err(|_| Error::Limit)??;
        if tokio::time::Instant::now() >= deadline {
            return Err(Error::Limit);
        }
        Ok(ids)
    }
    async fn authorize(&self) -> Result<()>;
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
    References(content::references::Request, content::Limits),
    Prepare,
}
#[derive(Serialize)]
#[serde(untagged)]
enum Output {
    Graph(GraphPage),
    Index(IndexReport),
    References(content::references::Page),
}
impl Work {
    fn is_query(&self) -> bool {
        matches!(self, Self::Graph(..) | Self::References(..))
    }
    async fn run(
        self,
        repo: &jj_lib::repo::ReadonlyRepo,
        access: &Arc<dyn ReadAccess>,
        budget: &Arc<Budget>,
        objects: Arc<NativeObjects>,
        cache: cache::Report,
    ) -> Result<Output> {
        match self {
            Self::Graph(request, page) => {
                let (data, next) = graphs::run(repo, request, page, access, budget).await?;
                Ok(Output::Graph(GraphPage {
                    version: 1,
                    cache,
                    operation: access.operation(),
                    data,
                    next,
                    stats: budget.stats(),
                }))
            }
            Self::References(request, limits) => {
                let reader = repo
                    .readonly_index()
                    .downcast_ref::<projection::Reader>()
                    .ok_or(Error::Unavailable)?;
                let context = content::Context {
                    limits,
                    code_index: None,
                    budget: budget.clone(),
                    access: access.clone(),
                    objects,
                    permit: None,
                };
                Ok(Output::References(
                    content::references::read_prepared(
                        context,
                        request,
                        reader,
                        repo.view().store_view(),
                    )
                    .await?,
                ))
            }
            Self::Prepare => Err(Error::Unavailable),
        }
    }
}
/// Bounded jobs and operation-specific local caches; no physical working copy.
#[derive(Clone)]
pub struct GraphExecutor {
    live_limits: Option<Arc<dyn Fn(Limits) -> Limits + Send + Sync>>,
    root: PathBuf,
    limits: Limits,
    preparation: preparation::Limits,
    workers: Workers,
    cache_policy: cache::Policy,
    residents: Arc<cache::resident::Residents>,
}
impl GraphExecutor {
    pub fn with_live_limits(
        mut self,
        limits: impl Fn(Limits) -> Limits + Send + Sync + 'static,
    ) -> Self {
        self.live_limits = Some(Arc::new(limits));
        self
    }
    fn current_limits(&self) -> Limits {
        self.live_limits
            .as_ref()
            .map_or(self.limits, |f| f(self.limits))
    }

    pub(super) fn reference_filters(&self) -> bool {
        self.cache_policy.reference_filters()
    }
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
            live_limits: None,
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
            _ => Err(Error::Unavailable),
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
            _ => Err(Error::Unavailable),
        }
    }
    pub(crate) async fn references(
        &self,
        store: Arc<dyn ObjectStore>,
        access: Arc<dyn ReadAccess>,
        request: content::references::Request,
        limits: content::Limits,
    ) -> Result<content::references::Page> {
        content::references::validate(&request, &limits)?;
        match self
            .execute_work(
                store,
                access,
                Work::References(request, limits),
                Cancellation::default(),
            )
            .await?
        {
            Output::References(page) => Ok(page),
            _ => Err(Error::Unavailable),
        }
    }
    async fn execute_work(
        &self,
        store: Arc<dyn ObjectStore>,
        access: Arc<dyn ReadAccess>,
        work: Work,
        cancellation: Cancellation,
    ) -> Result<Output> {
        let current = self.current_limits();
        let limits = match &work {
            Work::Prepare => self.preparation.budget(current),
            Work::Graph(..) => current,
            Work::References(_, content) => Limits {
                metadata: content.metadata,
                max_page: content.max_page,
                max_work: content.max_work,
                max_object_bytes: content.max_read_bytes,
                max_output_bytes: content.max_output_bytes,
                timeout: content.timeout,
                ..current
            },
        };
        let mut budget = Budget::new(limits)?;
        budget.cancellation = cancellation.token();
        let budget = Arc::new(budget);
        let permit = match &work {
            Work::Prepare => self.workers.acquire_preparation()?,
            Work::Graph(..) | Work::References(..) => self.workers.acquire()?,
        };
        budget.authorize(access.as_ref()).await?;
        let access = if let Work::References(request, _) = &work {
            access::select_operation(access, request.operation, &budget).await?
        } else {
            access
        };
        if let Work::Graph(_, page) = &work
            && (page.limit == 0 || page.limit > limits.max_page)
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
                    .until(executor.run(store, access.clone(), work, current, budget.clone()))
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
        limits: Limits,
        budget: Arc<Budget>,
    ) -> Result<Output> {
        budget.check()?;
        if let Work::Graph(request, page) = &work {
            let query = match request {
                Request::Operations => Some(""),
                Request::OperationsFiltered { query } => Some(query.as_str()),
                _ => None,
            };
            let filter = query.map(language::operations::Filter::parse).transpose()?;
            let saved = filter
                .as_ref()
                .and_then(|_| self.residents.get_memory(access.as_ref()))
                .filter(|saved| saved.index.operations.is_some());
            if filter
                .as_ref()
                .is_some_and(|filter| !filter.needs_prefix_lookup() || saved.is_some())
            {
                let lookup = saved
                    .as_ref()
                    .and_then(|saved| saved.index.operations.clone());
                graphs::validate(request, page, &budget)?;
                let objects = Arc::new(
                    NativeObjects::new(
                        Arc::new(ReadStore::new(store, access.clone(), budget.clone(), false)),
                        access.repository(),
                        limits.max_object_bytes,
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
                let (data, next) = graphs::run_operations(
                    &head,
                    request.clone(),
                    page.clone(),
                    &access,
                    &budget,
                    lookup,
                )
                .await?;
                let result = Output::Graph(GraphPage {
                    version: 1,
                    operation: access.operation(),
                    data,
                    next,
                    cache: saved
                        .as_ref()
                        .map(|saved| saved.report())
                        .unwrap_or(cache::Report {
                            state: cache::State::NotUsed,
                            evicted: 0,
                            retained_entries: 0,
                            retained_bytes: 0,
                        }),
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
                match &work {
                    Work::References(_, limits) => limits.max_blob_bytes,
                    _ => limits.max_object_bytes,
                },
            )
            .map_err(|_| Error::Limit)?,
        );
        let authority = Arc::new(ReadOnlyAuthority {
            access: access.clone(),
            budget: budget.clone(),
        });
        let stores = NativeStores::new(
            objects.clone(),
            authority,
            BackendLimits::default(),
            OperationLimits::default(),
        )
        .map_err(|_| Error::Unavailable)?;
        let settings = UserSettings::from_config(StackedConfig::with_defaults())
            .map_err(|_| Error::Unavailable)?;
        let required_filter =
            matches!(&work, Work::References(request, _) if !request.query.is_empty());
        let want_filters = matches!(work, Work::Prepare) && self.reference_filters();
        let mut reclaimed = false;
        let (cache, entry) = loop {
            let mut cache = match cache::Cache::open(&self.root, self.cache_policy, &budget) {
                Ok(cache) => Some(cache),
                Err(Error::Busy) if work.is_query() => None,
                Err(Error::Limit) if !reclaimed => {
                    reclaimed = true;
                    if self.residents.reclaim_filtered() {
                        continue;
                    }
                    return Err(Error::Limit);
                }
                Err(error) => return Err(error),
            };
            let result = match &mut cache {
                Some(cache) => cache
                    .prepare(
                        access.as_ref(),
                        &stores,
                        &budget,
                        &self.residents,
                        want_filters,
                    )
                    .map(Some),
                None => Ok(None),
            };
            let entry = match result {
                Ok(entry) => entry,
                Err(Error::Limit)
                    if work.is_query()
                        && cache
                            .as_ref()
                            .is_some_and(|c| c.can_use_reserved_fallback()) =>
                {
                    None
                }
                Err(error @ (Error::Limit | Error::Busy)) if !reclaimed => {
                    drop(cache);
                    reclaimed = true;
                    if self.residents.reclaim_filtered() {
                        continue;
                    }
                    return Err(error);
                }
                Err(error) => return Err(error),
            };
            break (cache, entry);
        };
        let mut cache = cache;
        if entry.is_none() {
            let blocked = if cache.is_some() {
                Error::Limit
            } else {
                Error::Busy
            };
            drop(cache);
            let saved = self.residents.get_memory(access.as_ref()).ok_or(blocked)?;
            if required_filter && !saved.index.has_reference_filter() {
                return Err(Error::IndexRebuildRequired);
            }
            budget.start_index(false)?;
            // Construct fresh request-scoped native capabilities. The saved
            // index carries no previous caller's authorization or backend.
            let loader = index::resident_loader(
                &stores,
                &settings,
                saved.clone(),
                budget.clone(),
                index::ResidentContext {
                    root: self.root.clone(),
                    cache_policy: self.cache_policy,
                    residents: self.residents.clone(),
                    access: access.clone(),
                },
            )?;
            let repo = stores
                .load_at(&loader, access.operation())
                .await
                .map_err(|_| Error::Unavailable)?;
            budget.finish_initial_index();
            let result = work
                .run(&repo, &access, &budget, objects, saved.report())
                .await?;
            budget.authorize(access.as_ref()).await?;
            budget.complete_output(&result)?;
            return Ok(result);
        }
        let mut entry = entry.ok_or(Error::Unavailable)?;
        if entry.private() {
            // Construction and recovered disk data have independent ownership.
            drop(cache.take());
        }
        if !entry.verify(self.cache_policy, &budget)? {
            // Damaged private files are disposable. Release their actual-size
            // reservation before admitting a fresh bounded construction.
            drop(entry);
            let mut admission =
                cache::Cache::reopen(&self.root, self.cache_policy, &budget).await?;
            entry = admission.rebuild(access.as_ref())?;
            drop(admission);
        }
        if work.is_query() {
            budget.start_index(entry.initialize)?;
        }
        if entry.initialize {
            stores
                .initialize_cache(&settings, &entry.path)
                .await
                .map_err(|_| Error::Unavailable)?;
        }
        let loader = index::loader(
            &stores,
            &settings,
            &entry.path,
            index::LoaderSelection {
                budget: budget.clone(),
                root: self.root.clone(),
                cache_policy: self.cache_policy,
                residents: self.residents.clone(),
                required_filter,
                resident: entry.resident.clone(),
                access: access.clone(),
                preparing: matches!(work, Work::Prepare),
            },
        )?;
        let repo = stores
            .load_at(&loader, access.operation())
            .await
            .map_err(|_| Error::Unavailable)?;
        budget.check()?;
        budget.finish_initial_index();
        budget.authorize(access.as_ref()).await?;
        entry.seal(self.cache_policy, &budget)?;
        let mut cache = match cache {
            Some(cache) => cache,
            None => cache::Cache::reopen(&self.root, self.cache_policy, &budget).await?,
        };
        if work.is_query() {
            // Publish only the complete primary generation, then release the
            // namespace. Additional operations use private reserved construction
            // and immutable residents; this loader never extends this slot.
            let index = repo
                .readonly_index()
                .downcast_ref::<projection::Reader>()
                .ok_or(Error::Unavailable)?
                .prepared
                .clone();
            let report = cache.finish(entry, access.as_ref(), &budget, index, &self.residents)?;
            drop(cache);
            let result = work.run(&repo, &access, &budget, objects, report).await?;
            budget.authorize(access.as_ref()).await?;
            budget.complete_output(&result)?;
            return Ok(result);
        }
        // All interactive work returned above; only explicit preparation remains.
        if !matches!(work, Work::Prepare) {
            return Err(Error::Unavailable);
        }
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
        let result = Output::Index(IndexReport {
            version: 1,
            cache,
            operation: access.operation(),
            stats: budget.stats(),
        });
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
