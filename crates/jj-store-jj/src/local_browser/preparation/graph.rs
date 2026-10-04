use super::*;
use blake2::{Blake2b512, Digest as _};
use jj_lib::{
    default_index::{DefaultIndexStore, DefaultReadonlyIndex},
    index::{
        Index as _, IndexStore, IndexStoreError, IndexStoreResult, MutableIndex, ReadonlyIndex,
    },
    store::Store,
};
use std::{
    fs,
    io::Read as _,
    sync::atomic::{AtomicU64, Ordering},
};

struct Memory {
    used: Arc<AtomicU64>,
    bytes: u64,
}
impl Memory {
    fn reserve(used: Arc<AtomicU64>, bytes: u64) -> Result<Self> {
        used.fetch_update(Ordering::AcqRel, Ordering::Acquire, |old| {
            old.checked_add(bytes).filter(|n| *n <= 256 * 1024 * 1024)
        })
        .map_err(|_| Error::Limit)?;
        Ok(Self { used, bytes })
    }
}
impl Drop for Memory {
    fn drop(&mut self) {
        self.used.fetch_sub(self.bytes, Ordering::AcqRel);
    }
}
pub(super) struct Resident {
    index: DefaultReadonlyIndex,
    metadata: PathBuf,
    extras: Arc<extras::Extras>,
    shallow: BTreeSet<gix::ObjectId>,
    _memory: Memory,
}
pub(super) async fn load(
    directory: &tempfile::TempDir,
    source: &LocalSource,
    operation: m::OperationId,
    memory: Arc<AtomicU64>,
) -> Result<Resident> {
    let mut segments =
        fs::read_dir(directory.path().join("segments")).map_err(|_| Error::Incomplete)?;
    let segment = segments
        .next()
        .ok_or(Error::Incomplete)?
        .map_err(|_| Error::Incomplete)?;
    if segments.next().is_some()
        || !segment
            .file_type()
            .map_err(|_| Error::Incomplete)?
            .is_file()
    {
        return Err(Error::Corrupt);
    }
    let size = segment.metadata().map_err(|_| Error::Incomplete)?.len();
    let memory = Memory::reserve(
        memory,
        size.saturating_mul(2)
            .saturating_add(512 * 1024 + source.shallow.len() as u64 * 128),
    )?;
    let mut input = fs::File::open(segment.path()).map_err(|_| Error::Incomplete)?;
    let mut digest = Blake2b512::new();
    let mut buffer = vec![0; 64 * 1024];
    let mut read = 0u64;
    loop {
        source.budget.work(1)?;
        let n = input.read(&mut buffer).map_err(|_| Error::Incomplete)?;
        if n == 0 {
            break;
        }
        read += n as u64;
        if read > size {
            return Err(Error::Corrupt);
        }
        source.budget.read_bytes(n)?;
        digest.update(&buffer[..n]);
    }
    let hash = digest.finalize();
    if read != size || segment.file_name().to_str() != Some(&format!("{hash:x}")) {
        return Err(Error::Corrupt);
    }
    let link = directory
        .path()
        .join("op_links")
        .join(operation.to_string());
    if !fs::symlink_metadata(&link)
        .map_err(|_| Error::Incomplete)?
        .is_file()
    {
        return Err(Error::Corrupt);
    }
    let mut expected = vec![10, 64];
    expected.extend_from_slice(&hash);
    // Fixed-length bounded link read; no unchecked segment path enters JJ.
    let mut bytes = vec![];
    fs::File::open(link)
        .map_err(|_| Error::Incomplete)?
        .take(67)
        .read_to_end(&mut bytes)
        .map_err(|_| Error::Incomplete)?;
    if bytes != expected {
        return Err(Error::Corrupt);
    }
    let context = Context {
        repo: None,
        store: source.loader.store().clone(),
        scoped: None,
        access: source.pin.clone(),
        budget: source.budget.clone(),
        limits: super::super::Limits::default(),
    };
    // This operation capability denies views, so unexpected upstream fallback
    // cannot traverse commits or import source metadata while loading the index.
    let selected = operation_store::selected(&context, operation).await?;
    let index = DefaultIndexStore::load(directory.path())
        .get_index_at_op(&selected, source.loader.store())
        .await
        .map_err(|_| Error::Unavailable)?;
    let index = index
        .downcast_ref::<DefaultReadonlyIndex>()
        .ok_or(Error::Unavailable)?
        .clone();
    if index.num_commits() as usize > source.limits.max_commits {
        return Err(Error::IndexBudget);
    }
    Ok(Resident {
        index,
        metadata: source.pin.binding.metadata_path().to_owned(),
        extras: source.extras.clone(),
        shallow: source.shallow.clone(),
        _memory: memory,
    })
}

impl Prepared {
    pub(in super::super) async fn contains(&self, id: m::GitSha1Id) -> Result<bool> {
        self.resident
            .index
            .has_id(&b::CommitId::from_bytes(id.as_bytes()))
            .await
            .map_err(|_| Error::Unavailable)
    }
    pub(in super::super) fn commit(
        &self,
        backend: &dyn b::Backend,
        id: &b::CommitId,
        budget: &Budget,
    ) -> Result<b::Commit> {
        let git = backend
            .downcast_ref::<jj_lib::git_backend::GitBackend>()
            .ok_or(Error::Invalid)?
            .git_repo();
        commit::read(
            backend,
            &git,
            &self.resident.extras,
            &self.resident.shallow,
            id,
            budget,
        )
    }
}
struct Index {
    prepared: Arc<Prepared>,
    budget: Arc<Budget>,
}
impl std::fmt::Debug for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("PreparedLocalIndex")
    }
}
#[async_trait::async_trait(?Send)]
impl IndexStore for Index {
    fn name(&self) -> &str {
        "default"
    }
    async fn get_index_at_op(
        &self,
        operation: &Operation,
        _: &Arc<Store>,
    ) -> IndexStoreResult<Box<dyn ReadonlyIndex>> {
        self.budget
            .check()
            .map_err(|e| IndexStoreError::Read(Box::new(e)))?;
        if operation.id().as_bytes() != self.prepared.snapshot.operation.as_bytes() {
            return Err(IndexStoreError::Read(Box::new(Error::NotCommitted)));
        }
        Ok(Box::new(self.prepared.resident.index.clone()))
    }
    fn write_index(
        &self,
        _: Box<dyn MutableIndex>,
        _: &Operation,
    ) -> IndexStoreResult<Box<dyn ReadonlyIndex>> {
        Err(IndexStoreError::Write(Box::new(Error::Denied)))
    }
}
#[derive(Debug)]
struct Submodules;
impl jj_lib::submodule_store::SubmoduleStore for Submodules {
    fn name(&self) -> &str {
        "read-only"
    }
}

impl Executor {
    /// An owned preparation handle is an exact immutable selection, not an
    /// authorization capability. Each call supplies and rechecks current access.
    pub async fn graph_prepared(
        &self,
        access: Arc<dyn Access>,
        prepared: Arc<Prepared>,
        request: GraphRequest,
        page: GraphWindow,
    ) -> Result<GraphPage> {
        if !matches!(request, GraphRequest::Log { .. }) {
            return Err(Error::Invalid);
        }
        let limits = self.limits;
        self.worker(access, move |access, budget| {
            Box::pin(async move {
                if access.repository() != prepared.snapshot.repository
                    || access.metadata_path() != prepared.resident.metadata
                {
                    return Err(Error::NotCommitted);
                }
                if access.generation() != prepared.snapshot.generation {
                    return Err(Error::EpochChanged);
                }
                graphs::validate(&request, &page, 0, &budget)?;
                let operation = prepared.snapshot.operation;
                let mut context =
                    Context::open(access, operation, budget.clone(), limits, true, None, false)
                        .await?;
                Arc::get_mut(&mut context.access)
                    .ok_or(Error::Unavailable)?
                    .prepared = Some(prepared.clone());
                let guard = scoped::Guard::new(
                    context.store.clone(),
                    &context.access,
                    budget.clone(),
                    limits,
                );
                let selected = operation_store::selected(&context, operation).await?;
                let raw = loader(context.access.binding.metadata_path())?;
                let settings = UserSettings::from_config(StackedConfig::with_defaults())
                    .map_err(|_| Error::Unavailable)?;
                let loader = RepoLoader::new(
                    settings,
                    guard.store(),
                    selected.op_store().clone(),
                    raw.op_heads_store().clone(),
                    Arc::new(Index {
                        prepared,
                        budget: budget.clone(),
                    }),
                    Arc::new(Submodules),
                );
                let repo = loader
                    .load_at(&selected)
                    .await
                    .map_err(|_| Error::Unavailable)?;
                let (data, next) =
                    graphs::run(&repo, request, page, &context.access, &budget).await?;
                budget.authorize(&context.access).await?;
                Ok(GraphPage {
                    version: 1,
                    operation,
                    data,
                    next,
                    stats: budget.stats(),
                })
            })
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resident_admission_releases_only_when_owner_drops() {
        let used = Arc::new(AtomicU64::new(0));
        let first = Memory::reserve(used.clone(), 200 * 1024 * 1024).unwrap();
        assert!(matches!(
            Memory::reserve(used.clone(), 60 * 1024 * 1024),
            Err(Error::Limit)
        ));
        assert_eq!(used.load(Ordering::Acquire), 200 * 1024 * 1024);
        drop(first);
        let second = Memory::reserve(used.clone(), 256 * 1024 * 1024).unwrap();
        assert!(matches!(
            Memory::reserve(used.clone(), 1),
            Err(Error::Limit)
        ));
        drop(second);
        assert_eq!(used.load(Ordering::Acquire), 0);
    }
}
