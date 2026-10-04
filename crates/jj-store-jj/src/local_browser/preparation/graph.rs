use super::*;
use crate::query::memory::Lease;
use blake2::{Blake2b512, Digest as _};
use jj_lib::{
    default_index::{DefaultIndexStore, DefaultReadonlyIndex},
    index::{
        Index as _, IndexStore, IndexStoreError, IndexStoreResult, MutableIndex, ReadonlyIndex,
    },
    repo::Repo as _,
    store::Store,
};
use sha2::Sha256;
use std::{fs, io::Read as _};
fn selection(pin: &Pin, operation: m::OperationId, budget: &Budget) -> Result<[u8; 32]> {
    struct Hash<'a> {
        digest: Sha256,
        budget: &'a Budget,
        failed: Option<Error>,
    }
    impl std::io::Write for Hash<'_> {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            if let Err(error) = self.budget.work(1) {
                self.failed = Some(error.clone());
                return Err(std::io::Error::other(error));
            }
            self.digest.update(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let operation = pin.operations.get(&operation).ok_or(Error::Incomplete)?;
    let view = pin.views.get(&operation.view_id).ok_or(Error::Incomplete)?;
    let mut hash = Hash {
        digest: Sha256::new(),
        budget,
        failed: None,
    };
    serde_json::to_writer(&mut hash, &(operation, view))
        .map_err(|_| hash.failed.take().unwrap_or(Error::Corrupt))?;
    Ok(hash.digest.finalize().into())
}
pub(super) struct Resident {
    index: DefaultReadonlyIndex,
    visibility: Arc<super::visibility::Visibility>,
    selection: [u8; 32],
    metadata: PathBuf,
    extras: Arc<extras::Extras>,
    shallow: BTreeSet<gix::ObjectId>,
    _memory: Lease,
}
pub(super) async fn load(
    directory: &tempfile::TempDir,
    source: &LocalSource,
    operation: m::OperationId,
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
    let weight = crate::query::memory::framing::weight(
        directory.path(),
        source.loader.store().commit_id_length(),
        source.loader.store().change_id_length(),
        source.limits.max_commits,
        &mut || source.budget.work(1),
    )?;
    let memory = Lease::reserve(weight.saturating_add(source.shallow.len() as u64 * 128))?;
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
        visibility: super::visibility::Visibility::build(&index, source).await?,
        selection: selection(&source.pin, operation, &source.budget)?,
        index,
        metadata: source.pin.binding.metadata_path().to_owned(),
        extras: source.extras.clone(),
        shallow: source.shallow.clone(),
        _memory: memory,
    })
}

impl Prepared {
    pub(in super::super) fn annotations(
        &self,
        id: m::GitSha1Id,
        budget: &Budget,
    ) -> Result<(Vec<Bookmark>, Vec<String>)> {
        self.resident.visibility.annotations(id, budget)
    }
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
        Ok(Box::new(super::visibility::PreparedIndex {
            index: self.prepared.resident.index.clone(),
            visibility: self.prepared.resident.visibility.clone(),
            budget: self.budget.clone(),
        }))
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
        let response = self
            .read_prepared(access, prepared, Request::Graph(request, page))
            .await?;
        let Response::Graph(page) = response else {
            return Err(Error::Invalid);
        };
        Ok(page)
    }
    /// Detailed history shares the exact prepared operation and current access.
    pub async fn read_prepared(
        &self,
        access: Arc<dyn Access>,
        prepared: Arc<Prepared>,
        request: Request,
    ) -> Result<Response> {
        if !request.needs_prepared_index() {
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
                if let Request::Graph(graph, page) = &request {
                    graphs::validate(graph, page, 0, &budget)?;
                }
                let operation = prepared.snapshot.operation;
                let mut context = Context::open(
                    access,
                    operation,
                    budget.clone(),
                    limits,
                    true,
                    vec![],
                    false,
                )
                .await?;
                if selection(&context.access, operation, &budget)? != prepared.resident.selection {
                    return Err(Error::Corrupt);
                }
                budget.check()?;
                Arc::get_mut(&mut context.access)
                    .ok_or(Error::Unavailable)?
                    .prepared = Some(prepared.clone());
                let guard = scoped::Guard::new(
                    context
                        .scoped
                        .as_ref()
                        .map_or(&context.store, |guard| &guard.source)
                        .clone(),
                    &context.access,
                    budget.clone(),
                    limits,
                );
                let selected = operation_store::selected_for(
                    &context,
                    operation,
                    matches!(request, Request::Graph(GraphRequest::Evolution { .. }, _)),
                )
                .await?;
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
                context.store = repo.store().clone();
                context.scoped = Some(guard);
                context.repo = Some(repo);
                context.dispatch(operation, request, false).await
            })
        })
        .await
    }
}
