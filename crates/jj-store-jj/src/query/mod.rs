//! Owned, bounded native graph queries. Upstream objects never cross this API.
mod access;
pub mod content;
mod graphs;
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
use std::{collections::BTreeSet, fs, path::PathBuf, sync::Arc, time::Duration};
use tokio::sync::Semaphore;

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
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
    #[error("invalid query or missing query context")]
    Invalid,
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
    fn inventory(&self) -> &BTreeSet<m::ObjectKey>;
    async fn authorize(&self) -> Result<()>;
}
#[derive(Clone, Copy)]
pub struct Limits {
    pub max_page: usize,
    pub max_index_commits: usize,
    pub max_work: usize,
    pub max_object_bytes: u64,
    pub max_output_bytes: usize,
    pub max_revset_bytes: usize,
    pub timeout: Duration,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            max_page: 500,
            max_index_commits: 50_000,
            max_work: 1_000_000,
            max_object_bytes: 256 * 1024 * 1024,
            max_output_bytes: 8 * 1024 * 1024,
            max_revset_bytes: 4096,
            timeout: Duration::from_secs(10),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "graph", rename_all = "snake_case", deny_unknown_fields)]
pub enum Request {
    Log {
        revset: String,
        workspace: Option<String>,
        user_email: String,
        at_time_millis: i64,
    },
    Operations,
    Evolution {
        commit: m::CommitId,
    },
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Page {
    pub limit: usize,
    pub after: Option<m::ObjectKey>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Timestamp {
    pub millis: i64,
    pub offset_minutes: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Signature {
    pub name: String,
    pub email: String,
    pub timestamp: Timestamp,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Bookmark {
    pub name: String,
    pub added: bool,
    pub removed: bool,
    pub conflicted: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeKind {
    Direct,
    Indirect,
    Missing,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Edge {
    pub target: m::CommitId,
    pub kind: EdgeKind,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
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
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CommitNode {
    pub commit: Commit,
    pub edges: Vec<Edge>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
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
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct EvolutionNode {
    pub commit: Commit,
    pub operation: Option<m::OperationId>,
    pub predecessors: Vec<m::CommitId>,
    pub predecessors_known: bool,
    pub legacy_predecessors: Vec<m::CommitId>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "graph", content = "nodes", rename_all = "snake_case")]
pub enum Nodes {
    Log(Vec<CommitNode>),
    Operations(Vec<OperationNode>),
    Evolution(Vec<EvolutionNode>),
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Stats {
    pub object_reads: u64,
    pub object_bytes: u64,
    pub work: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct GraphPage {
    pub version: u32,
    pub operation: m::OperationId,
    pub data: Nodes,
    pub next: Option<m::ObjectKey>,
    pub stats: Stats,
}
/// Bounded jobs and operation-specific local caches; no physical working copy.
#[derive(Clone)]
pub struct GraphExecutor {
    root: PathBuf,
    limits: Limits,
    permits: Arc<Semaphore>,
}
impl GraphExecutor {
    pub fn new(root: PathBuf, limits: Limits, concurrency: usize) -> Result<Self> {
        if !root.is_absolute()
            || !(1..=64).contains(&concurrency)
            || limits.max_page == 0
            || limits.max_page > 10_000
            || limits.max_index_commits == 0
            || limits.max_work == 0
            || limits.max_object_bytes == 0
            || limits.max_output_bytes == 0
            || limits.max_revset_bytes == 0
            || limits.max_revset_bytes > 16_384
            || limits.timeout.is_zero()
            || limits.timeout > Duration::from_secs(300)
        {
            return Err(Error::Limit);
        }
        Ok(Self {
            root,
            limits,
            permits: Arc::new(Semaphore::new(concurrency)),
        })
    }
    pub async fn execute(
        &self,
        store: Arc<dyn ObjectStore>,
        access: Arc<dyn ReadAccess>,
        request: Request,
        page: Page,
    ) -> Result<GraphPage> {
        let budget = Arc::new(Budget::new(self.limits)?);
        let permit = self
            .permits
            .clone()
            .try_acquire_owned()
            .map_err(|_| Error::Busy)?;
        budget.authorize(access.as_ref()).await?;
        if page.limit == 0 || page.limit > self.limits.max_page {
            return Err(Error::Limit);
        }
        let executor = self.clone();
        let runtime = tokio::runtime::Handle::current();
        // The permit stays with the blocking job if the caller cancels its future.
        tokio::task::spawn_blocking(move || {
            let _permit = permit;
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                let result = tokio::time::timeout_at(
                    budget.deadline,
                    executor.run(store, access.clone(), request, page, budget.clone()),
                )
                .await
                .map_err(|_| budget.fail(Error::Limit))?;
                budget.authorize(access.as_ref()).await?;
                match result {
                    Ok(value) => Ok(value),
                    Err(error) => Err(budget.failure().unwrap_or(error)),
                }
            }))
        })
        .await
        .map_err(|_| Error::Unavailable)?
    }
    async fn run(
        &self,
        store: Arc<dyn ObjectStore>,
        access: Arc<dyn ReadAccess>,
        request: Request,
        page: Page,
        budget: Arc<Budget>,
    ) -> Result<GraphPage> {
        budget.check()?;
        let count = access
            .inventory()
            .iter()
            .filter(|key| matches!(key, m::ObjectKey::Commit(_)))
            .count();
        if count > self.limits.max_index_commits {
            return Err(Error::IndexBudget);
        }
        graphs::validate(&request, &page, count, &budget)?;
        let root = self
            .root
            .join("7c41cdeb16b6b321c64e789a966b6adf723816a5")
            .join(PROFILE)
            .join(access.repository().to_string())
            .join(access.epoch().to_string());
        fs::create_dir_all(&root).map_err(|_| Error::Unavailable)?;
        let path = root.join(access.operation().to_string());
        let lock = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(root.join(format!("{}.lock", access.operation())))
            .map_err(|_| Error::Unavailable)?;
        lock.try_lock().map_err(|_| Error::Busy)?;
        let objects = Arc::new(
            NativeObjects::new(
                Arc::new(ReadStore::new(store, access.clone(), budget.clone())),
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
        if !path.exists() {
            let staging = tempfile::tempdir_in(&root).map_err(|_| Error::Unavailable)?;
            stores
                .initialize_cache(&settings, staging.path())
                .await
                .map_err(|_| Error::Unavailable)?;
            fs::rename(staging.path(), &path).map_err(|_| Error::Unavailable)?;
        }
        let loader = stores
            .load_cache(&settings, &path)
            .map_err(|_| Error::Unavailable)?;
        let repo = stores
            .load_at(&loader, access.operation())
            .await
            .map_err(|_| Error::Unavailable)?;
        budget.check()?;
        let (data, next) = graphs::run(&repo, request, page, &access, &budget).await?;
        budget.check()?;
        budget.authorize(access.as_ref()).await?;
        let result = GraphPage {
            version: 1,
            operation: access.operation(),
            data,
            next,
            stats: budget.stats(),
        };
        budget.complete_output(&result)?;
        Ok(result)
    }
}
