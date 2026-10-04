//! Admission on actual native index reads, without catalog enumeration.
use super::*;
use jj_lib::{
    default_backend_factories::default_backend_factories,
    default_index::{DefaultIndexStore, DefaultReadonlyIndex},
    index::{IndexStore, IndexStoreError, IndexStoreResult, MutableIndex, ReadonlyIndex},
    object_id::ObjectId as _,
    operation::Operation,
    repo::RepoLoader,
    store::Store,
};
use std::{
    fmt,
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};

pub(super) struct Admission {
    commits: BTreeSet<m::CommitId>,
    inline: bool,
}
impl Admission {
    pub fn new(inline: bool) -> Self {
        Self {
            commits: BTreeSet::new(),
            inline,
        }
    }
    pub fn admit(&mut self, id: m::CommitId, limits: Limits) -> Result<()> {
        if self.commits.contains(&id) {
            return Ok(());
        }
        if self.commits.len() >= limits.max_index_commits {
            return Err(Error::IndexBudget);
        }
        if self.inline && self.commits.len() >= limits.max_inline_rebuild_commits {
            return Err(Error::IndexRebuildRequired);
        }
        self.commits.insert(id);
        Ok(())
    }
    pub fn finish_initial_load(&mut self) {
        self.inline = false;
    }
}

struct GuardedIndexStore {
    inner: DefaultIndexStore,
    budget: Arc<Budget>,
    path: PathBuf,
    preparation_bytes: Option<u64>,
    resident: Option<Arc<cache::resident::Saved>>,
    disk_verified: AtomicBool,
}
impl fmt::Debug for GuardedIndexStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GuardedQueryIndex")
    }
}
#[async_trait::async_trait(?Send)]
impl IndexStore for GuardedIndexStore {
    fn name(&self) -> &str {
        self.inner.name()
    }
    async fn get_index_at_op(
        &self,
        operation: &Operation,
        store: &Arc<Store>,
    ) -> IndexStoreResult<Box<dyn ReadonlyIndex>> {
        self.budget
            .check()
            .map_err(|e| IndexStoreError::Read(Box::new(e)))?;
        if let Some(bytes) = self.preparation_bytes
            && !self
                .path
                .join("op_links")
                .join(operation.id().hex())
                .try_exists()
                .map_err(|e| IndexStoreError::Read(Box::new(e)))?
        {
            super::prepared::build(&self.path, operation, store, &self.budget, bytes)
                .await
                .map_err(|e| IndexStoreError::Read(Box::new(self.budget.fail(e))))?;
        }
        let index: Box<dyn ReadonlyIndex> = if let Some(resident) = &self.resident
            && operation.id().as_bytes() == resident.operation.as_bytes()
        {
            Box::new(resident.index.clone())
        } else {
            if let Some(resident) = &self.resident
                && !self.disk_verified.load(Ordering::Acquire)
            {
                resident
                    .verify_disk(&self.budget)
                    .map_err(|e| IndexStoreError::Read(Box::new(self.budget.fail(e))))?;
                // The namespace lock remains held. This job may now build more
                // operation indexes; subsequent loads must not compare its own
                // new files against the entry's original receipt.
                self.disk_verified.store(true, Ordering::Release);
            }
            self.inner.get_index_at_op(operation, store).await?
        };
        // Warm indexes need the same hard bound, even when no commits are read.
        // Also applies to additional operations loaded by native predicates.
        let count = index
            .downcast_ref::<DefaultReadonlyIndex>()
            .ok_or_else(|| IndexStoreError::Read(Box::new(self.budget.fail(Error::Unavailable))))?
            .num_commits() as usize;
        if count > self.budget.limits.max_index_commits {
            return Err(IndexStoreError::Read(Box::new(
                self.budget.fail(Error::IndexBudget),
            )));
        }
        self.budget
            .check()
            .map_err(|e| IndexStoreError::Read(Box::new(e)))?;
        Ok(index)
    }
    fn write_index(
        &self,
        _: Box<dyn MutableIndex>,
        _: &Operation,
    ) -> IndexStoreResult<Box<dyn ReadonlyIndex>> {
        Err(IndexStoreError::Write(Box::new(
            self.budget.fail(Error::Denied),
        )))
    }
}
pub(super) fn loader(
    stores: &NativeStores,
    settings: &UserSettings,
    path: &Path,
    budget: Arc<Budget>,
    preparation_bytes: Option<u64>,
    resident: Option<Arc<cache::resident::Saved>>,
) -> Result<RepoLoader> {
    stores
        .verify_cache_binding(path)
        .map_err(|_| Error::Unavailable)?;
    let mut factories = default_backend_factories();
    factories.merge(stores.factories());
    factories.add_index_store(
        "default",
        Box::new(move |_, path| {
            Ok(Box::new(GuardedIndexStore {
                inner: DefaultIndexStore::load(path),
                budget: budget.clone(),
                path: path.to_owned(),
                preparation_bytes,
                resident: resident.clone(),
                disk_verified: AtomicBool::new(false),
            }))
        }),
    );
    RepoLoader::init_from_file_system(settings, path, &factories).map_err(|_| Error::Unavailable)
}
