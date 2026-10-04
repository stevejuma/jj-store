//! Explicit construction in caller-reserved private scratch; no source index IO.
mod extras;
use super::*;
use crate::query::{Cancellation, prepared};
use jj_lib::{operation::Operation, view::View};
use std::{path::PathBuf, time::Duration};

/// Trusted composition policy, never deserialized from a browser request.
#[derive(Clone, Copy)]
pub struct Limits {
    pub max_commits: usize,
    pub max_work: usize,
    pub max_read_bytes: u64,
    pub max_metadata_records: u64,
    pub timeout: Duration,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            max_commits: 4_000_000,
            max_work: 500_000_000,
            max_read_bytes: 16 * 1024 * 1024 * 1024,
            max_metadata_records: 8_000_000,
            timeout: Duration::from_secs(3600),
        }
    }
}
impl Limits {
    fn validate(self) -> Result<()> {
        if self.max_commits == 0
            || self.max_commits > 4_000_000
            || self.max_work == 0
            || self.max_work > 500_000_000
            || self.max_read_bytes == 0
            || self.max_read_bytes > 16 * 1024 * 1024 * 1024
            || self.max_metadata_records == 0
            || self.max_metadata_records > 8_000_000
            || self.timeout.is_zero()
            || self.timeout > Duration::from_secs(3600)
        {
            return Err(Error::Limit);
        }
        Ok(())
    }
}

/// Owned library handle. It contains no credentials; dropping it removes output.
/// The caller must keep its disk reservation until this handle is dropped.
pub struct Prepared {
    directory: tempfile::TempDir,
    pub snapshot: Snapshot,
    pub stats: Stats,
}
impl Prepared {
    /// Complete derived index directory for a future guarded cache consumer.
    pub fn path(&self) -> &Path {
        self.directory.path()
    }
}

struct LocalSource {
    loader: RepoLoader,
    pin: Arc<Pin>,
    budget: Arc<Budget>,
    limits: Limits,
    git: gix::Repository,
    shallow: BTreeSet<gix::ObjectId>,
    extras: extras::Extras,
}
impl prepared::Source for LocalSource {
    fn check(&self) -> Result<()> {
        self.budget.check()
    }
    fn work(&self, amount: usize) -> Result<()> {
        self.budget.work(amount)
    }
    fn max_commits(&self) -> usize {
        self.limits.max_commits
    }
    fn max_parents(&self) -> usize {
        256
    }
    fn commit_len(&self) -> usize {
        self.loader.store().commit_id_length()
    }
    fn change_len(&self) -> usize {
        self.loader.store().change_id_length()
    }
    async fn operation(&self, id: &o::OperationId) -> Result<Operation> {
        self.budget.authorize(&self.pin).await?;
        bounded_op_file(
            self.pin.binding.metadata_path(),
            "operations",
            &id.hex(),
            &self.budget,
        )?;
        let operation = self
            .budget
            .until(self.loader.load_operation(id))
            .await?
            .map_err(|_| Error::Incomplete)?;
        let data =
            dto::Operation::try_from(operation.store_operation()).map_err(|_| Error::Corrupt)?;
        self.budget.metadata(&data)?;
        if data.parents.len() > 256 {
            return Err(Error::Limit);
        }
        Ok(operation)
    }
    async fn view(&self, operation: &Operation) -> Result<View> {
        self.budget.authorize(&self.pin).await?;
        bounded_op_file(
            self.pin.binding.metadata_path(),
            "views",
            &operation.store_operation().view_id.hex(),
            &self.budget,
        )?;
        let view = self
            .budget
            .until(operation.view())
            .await?
            .map_err(|_| Error::Incomplete)?;
        let data = dto::View::try_from(view.store_view()).map_err(|_| Error::Corrupt)?;
        self.budget.metadata(&data)?;
        Ok(view)
    }
    async fn commit(&self, id: &b::CommitId) -> Result<prepared::CommitFields> {
        self.budget.authorize(&self.pin).await?;
        let backend = self.loader.store().backend();
        if id == backend.root_commit_id() {
            return Ok(prepared::CommitFields {
                parents: vec![],
                change: backend.root_change_id().clone(),
                timestamp: 0,
            });
        }
        bounded_git_object(
            backend,
            git_id(id)?,
            gix::object::Kind::Commit,
            &self.budget,
        )?;
        let oid = gix::ObjectId::from_bytes_or_panic(id.as_bytes());
        let object = self.git.find_object(oid).map_err(|_| Error::Incomplete)?;
        let commit = object.try_to_commit_ref().map_err(|_| Error::Corrupt)?;
        let mut parents = vec![];
        if !self.shallow.contains(&oid) {
            for id in commit.parents() {
                self.budget.work(1)?;
                if parents.len() == 256 {
                    return Err(Error::Limit);
                }
                parents.push(b::CommitId::from_bytes(id.as_bytes()));
            }
        }
        if parents.is_empty() {
            parents.push(backend.root_commit_id().clone());
        }
        let change = self
            .extras
            .change(id.as_bytes(), &self.budget)?
            .or_else(|| jj_lib::git_backend::extract_change_id_from_commit(&commit))
            .unwrap_or_else(|| jj_lib::git_backend::synthetic_change_id_from_git_commit_id(id));
        let timestamp = commit
            .committer()
            .map_err(|_| Error::Corrupt)?
            .time()
            .unwrap_or_default()
            .seconds
            .checked_mul(1000)
            .ok_or(Error::Corrupt)?;
        self.budget
            .metadata(&(parents.len(), change.hex(), timestamp))?;
        Ok(prepared::CommitFields {
            parents,
            change,
            timestamp,
        })
    }
}

impl Executor {
    /// Prepare one exact retained operation using the reserved background worker.
    /// `root` is an existing private directory outside source metadata;
    /// `reserved_bytes` must already be charged to the caller's cache quota.
    /// This library method does not register a browser task or retain a cache.
    pub async fn prepare_index(
        &self,
        access: Arc<dyn Access>,
        operation: m::OperationId,
        root: PathBuf,
        reserved_bytes: u64,
        limits: Limits,
        cancellation: Cancellation,
    ) -> Result<Prepared> {
        limits.validate()?;
        if !root.is_absolute() || reserved_bytes == 0 || reserved_bytes > 2 * 1024 * 1024 * 1024 {
            return Err(Error::Limit);
        }
        let permit = self.workers.acquire_preparation()?;
        access.authorize().await?;
        let budget = Arc::new(Budget::preparation(&limits, &cancellation));
        let cancel = cancellation.token().drop_guard();
        let capture_limits = self.limits;
        let runtime = tokio::runtime::Handle::current();
        let result = tokio::task::spawn_blocking(move || {
            let _permit = permit;
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                budget
                    .until(async {
                        let root = std::fs::canonicalize(root).map_err(|_| Error::Unavailable)?;
                        let metadata = std::fs::canonicalize(access.metadata_path())
                            .map_err(|_| Error::Unavailable)?;
                        if root.starts_with(metadata) {
                            return Err(Error::Invalid);
                        }
                        // This proves retention without reading a commit or source index.
                        let context = Context::open(
                            access,
                            operation,
                            budget.clone(),
                            capture_limits,
                            true,
                            None,
                            false,
                        )
                        .await?;
                        let directory = tempfile::Builder::new()
                            .prefix(".local-index-")
                            .tempdir_in(root)
                            .map_err(|_| Error::Unavailable)?;
                        let loader = loader(context.access.binding.metadata_path())?;
                        let git = loader
                            .store()
                            .backend()
                            .downcast_ref::<jj_lib::git_backend::GitBackend>()
                            .ok_or(Error::Invalid)?
                            .git_repo();
                        let shallow = shallow_roots(&git, &budget)?;
                        let extras =
                            extras::Extras::open(context.access.binding.metadata_path(), &budget)?;
                        let source = LocalSource {
                            loader,
                            git,
                            shallow,
                            extras,
                            pin: context.access.clone(),
                            budget: budget.clone(),
                            limits,
                        };
                        prepared::build_from(
                            directory.path(),
                            &o::OperationId::from_bytes(operation.as_bytes()),
                            &source,
                            reserved_bytes,
                        )
                        .await?;
                        budget.authorize(&context.access).await?;
                        Ok(Prepared {
                            directory,
                            snapshot: Snapshot {
                                version: 1,
                                profile: PROFILE,
                                repository: context.access.repository(),
                                generation: context.access.generation(),
                                operation,
                            },
                            stats: budget.stats(),
                        })
                    })
                    .await?
            }))
        })
        .await
        .map_err(|_| Error::Unavailable)?;
        cancel.disarm();
        result
    }
}

fn shallow_roots(git: &gix::Repository, budget: &Budget) -> Result<BTreeSet<gix::ObjectId>> {
    let path = git.shallow_file();
    let info = match std::fs::symlink_metadata(&path) {
        Ok(info) => info,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(BTreeSet::new()),
        Err(_) => return Err(Error::Incomplete),
    };
    if !info.is_file() || info.len() > 1024 * 1024 {
        return Err(Error::Limit);
    }
    budget.read_bytes(info.len() as usize)?;
    // A bounded read also prevents a concurrently growing file from allocating
    // more than admitted before parsing the captured shallow-root set.
    use std::io::Read as _;
    let mut bytes = vec![];
    std::fs::File::open(path)
        .map_err(|_| Error::Incomplete)?
        .take(1024 * 1024 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| Error::Incomplete)?;
    if bytes.len() as u64 != info.len() {
        return Err(Error::Incomplete);
    }
    let text = std::str::from_utf8(&bytes).map_err(|_| Error::Corrupt)?;
    text.lines()
        .map(|line| {
            budget.work(1)?;
            if line.len() != 40 {
                return Err(Error::Corrupt);
            }
            gix::ObjectId::from_hex(line.as_bytes()).map_err(|_| Error::Corrupt)
        })
        .collect()
}
