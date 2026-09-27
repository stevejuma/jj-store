//! Typed native transaction preparation. This executor cannot publish heads.
mod access;
mod execute;
mod reconcile;
use crate::query::{self, ReadAccess, access::Budget, workers::Workers};
use jj_store_model as m;
use jj_store_object::ObjectStore;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, sync::Arc};

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Query(#[from] query::Error),
    #[error("mutation requires the exact current snapshot")]
    Stale,
    #[error("mutation requires one explicit operation head")]
    MultipleHeads,
    #[error("invalid typed command")]
    Invalid,
    #[error("target is hidden or absent from this view")]
    Hidden,
    #[error("root commit cannot be rewritten")]
    Root,
    #[error("bookmark target differs from the expected native terms")]
    StaleBookmark,
}
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Command {
    New {
        parents: Vec<m::CommitId>,
        description: String,
        workspace: Option<String>,
    },
    Describe {
        commit: m::CommitId,
        description: String,
    },
    BookmarkCreate {
        name: String,
        target: m::CommitId,
    },
    BookmarkMove {
        name: String,
        expected: crate::native_dto::RefTarget,
        target: m::CommitId,
    },
    BookmarkDelete {
        name: String,
        expected: crate::native_dto::RefTarget,
    },
    ReconcileOperations {
        base: m::OperationId,
    },
}
impl Command {
    /// Shared shape admission for execution and canonical application intent.
    pub fn validate(&self) -> Result<()> {
        execute::validate(self)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct Rewrite {
    pub commit: m::CommitId,
    pub change: m::ChangeId,
    pub predecessors: Vec<m::CommitId>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct Change<T> {
    pub before: T,
    pub after: T,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct Prepared {
    pub version: u32,
    pub operation: m::OperationId,
    pub created: Vec<m::CommitId>,
    pub rewrites: Vec<Rewrite>,
    pub bookmarks: BTreeMap<String, Change<crate::native_dto::RefTarget>>,
    pub workspaces: BTreeMap<String, Change<Option<m::CommitId>>>,
    pub stats: query::Stats,
    pub staged_objects: usize,
    pub staged_bytes: u64,
}
/// Implemented by a checked service session, never deserialized from a request.
#[async_trait::async_trait]
pub trait MutationAccess: ReadAccess {
    fn principal(&self) -> m::PrincipalId;
    fn heads(&self) -> &std::collections::BTreeSet<m::OperationId>;
    async fn authorize_write(&self) -> query::Result<()>;
}
#[derive(Clone, Copy)]
pub struct Limits {
    pub query: query::Limits,
    pub max_staged_objects: usize,
    pub max_staged_bytes: u64,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            query: query::Limits {
                max_index_commits: 10_000,
                max_output_bytes: 512 * 1024,
                ..Default::default()
            },
            max_staged_objects: 40_000,
            max_staged_bytes: 256 * 1024 * 1024,
        }
    }
}
#[derive(Clone)]
pub struct Executor {
    limits: Limits,
    workers: Workers,
}
impl Executor {
    pub fn new(limits: Limits, capacity: usize) -> Result<Self> {
        Self::with_workers(limits, Workers::new(capacity)?)
    }
    pub fn with_workers(limits: Limits, workers: Workers) -> Result<Self> {
        // Reuse exact native read-budget validation without opening a cache.
        limits.query.validate()?;
        if limits.max_staged_objects == 0
            || limits.max_staged_objects > 100_000
            || limits.max_staged_bytes == 0
            || limits.max_staged_bytes > 1024 * 1024 * 1024
        {
            return Err(query::Error::Limit.into());
        }
        Ok(Self { limits, workers })
    }
    pub fn workers(&self) -> &Workers {
        &self.workers
    }
    pub async fn prepare(
        &self,
        store: Arc<dyn ObjectStore>,
        access: Arc<dyn MutationAccess>,
        command: Command,
    ) -> Result<Prepared> {
        let budget = Arc::new(Budget::new(self.limits.query)?);
        let permit = self.workers.acquire()?;
        budget.until(access.authorize_write()).await??;
        execute::validate(&command)?;
        let cancellation = budget.cancellation.clone().drop_guard();
        let executor = self.clone();
        let runtime = tokio::runtime::Handle::current();
        let result = tokio::task::spawn_blocking(move || {
            let _permit = permit;
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                let result = budget
                    .until(execute::run(
                        store,
                        access.clone(),
                        command,
                        executor.limits,
                        budget.clone(),
                    ))
                    .await?;
                budget.until(access.authorize_write()).await??;
                match result {
                    Ok(result) => Ok(result),
                    Err(error) => Err(budget.failure().map_or(error, Error::Query)),
                }
            }))
        })
        .await
        .map_err(|_| query::Error::Unavailable)?;
        cancellation.disarm();
        result
    }
}
