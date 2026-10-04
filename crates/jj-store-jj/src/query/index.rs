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
    cache_policy: cache::Policy,
    resident: Option<Arc<cache::resident::Saved>>,
    residents: Arc<cache::resident::Residents>,
    disk_verified: AtomicBool,
    access: Arc<dyn ReadAccess>,
    preparing: bool,
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
        let selected = m::OperationId::from_slice(operation.id().as_bytes())
            .map_err(|_| IndexStoreError::Read(Box::new(self.budget.fail(Error::Corrupt))))?;
        if selected != self.access.operation()
            && let Some(saved) = self.residents.get_memory_at(self.access.as_ref(), selected)
        {
            // The native operation/view still come from this request's Store.
            // Reuse only exact immutable data; never load/copy the other slot.
            self.budget
                .authorize(self.access.as_ref())
                .await
                .map_err(|e| IndexStoreError::Read(Box::new(e)))?;
            return saved
                .reader(self.budget.clone())
                .map_err(|e| IndexStoreError::Read(Box::new(e)));
        }
        if let Some(resident) = &self.resident
            && operation.id().as_bytes() != resident.operation.as_bytes()
            && !self.disk_verified.load(Ordering::Acquire)
        {
            resident
                .verify_disk(&self.budget)
                .map_err(|e| IndexStoreError::Read(Box::new(self.budget.fail(e))))?;
            // Verify the original receipt before adding bounded derived data.
            self.disk_verified.store(true, Ordering::Release);
        }
        if !self
            .path
            .join("op_links")
            .join(operation.id().hex())
            .try_exists()
            .map_err(|e| IndexStoreError::Read(Box::new(e)))?
        {
            let bytes = cache::construction_bytes(
                self.path
                    .parent()
                    .ok_or_else(|| IndexStoreError::Read(Box::new(Error::Unavailable)))?,
                self.cache_policy,
                &self.budget,
            )
            .map_err(|e| IndexStoreError::Read(Box::new(self.budget.fail(e))))?;
            super::prepared::build(
                &self.path,
                operation,
                store,
                &self.budget,
                bytes,
                !self.preparing,
            )
            .await
            .map_err(|e| IndexStoreError::Read(Box::new(self.budget.fail(e))))?;
        }
        let index = if let Some(resident) = &self.resident
            && operation.id().as_bytes() == resident.operation.as_bytes()
        {
            resident.index.clone()
        } else {
            self.budget
                .index_memory
                .reserve(
                    &self.path,
                    store.commit_id_length(),
                    store.change_id_length(),
                    self.budget.limits.max_index_commits,
                    || self.budget.work(1),
                )
                .map_err(|e| IndexStoreError::Read(Box::new(self.budget.fail(e))))?;
            let index = self.inner.get_index_at_op(operation, store).await?;
            let raw = index
                .downcast_ref::<DefaultReadonlyIndex>()
                .ok_or_else(|| {
                    IndexStoreError::Read(Box::new(self.budget.fail(Error::Unavailable)))
                })?;
            if raw.num_commits() as usize > self.budget.limits.max_index_commits {
                return Err(IndexStoreError::Read(Box::new(
                    self.budget.fail(Error::IndexBudget),
                )));
            }
            // A disk index alone lacks the resident visibility projection.
            // Large missing projections use explicit preparation's own budget.
            if !self.preparing
                && raw.num_commits() as usize > self.budget.limits.max_inline_rebuild_commits
            {
                return Err(IndexStoreError::Read(Box::new(
                    self.budget.fail(Error::IndexRebuildRequired),
                )));
            }
            let view = operation
                .view()
                .await
                .map_err(|e| IndexStoreError::Read(Box::new(e)))?;
            projection::Prepared::build(
                raw.clone(),
                &view,
                store,
                self.access.as_ref(),
                &self.budget,
            )
            .await
            .map_err(|e| IndexStoreError::Read(Box::new(self.budget.fail(e))))?
        };
        // Warm indexes need the same hard bound, even when no commits are read.
        // Also applies to additional operations loaded by native predicates.
        let count = index.num_commits() as usize;
        if count > self.budget.limits.max_index_commits {
            return Err(IndexStoreError::Read(Box::new(
                self.budget.fail(Error::IndexBudget),
            )));
        }
        self.budget
            .check()
            .map_err(|e| IndexStoreError::Read(Box::new(e)))?;
        Ok(index.reader(self.budget.clone()))
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
    cache_policy: cache::Policy,
    resident: Option<Arc<cache::resident::Saved>>,
    residents: Arc<cache::resident::Residents>,
    access: Arc<dyn ReadAccess>,
    preparing: bool,
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
                cache_policy,
                resident: resident.clone(),
                residents: residents.clone(),
                disk_verified: AtomicBool::new(false),
                access: access.clone(),
                preparing,
            }))
        }),
    );
    RepoLoader::init_from_file_system(settings, path, &factories).map_err(|_| Error::Unavailable)
}

struct ResidentIndexStore {
    saved: Arc<cache::resident::Saved>,
    budget: Arc<Budget>,
}
impl fmt::Debug for ResidentIndexStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ResidentQueryIndex")
    }
}
#[async_trait::async_trait(?Send)]
impl IndexStore for ResidentIndexStore {
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
        if operation.id().as_bytes() != self.saved.operation.as_bytes() {
            return Err(IndexStoreError::Read(Box::new(
                self.budget.fail(Error::NotCommitted),
            )));
        }
        self.saved
            .reader(self.budget.clone())
            .map_err(|e| IndexStoreError::Read(Box::new(e)))
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
pub(super) fn resident_loader(
    stores: &NativeStores,
    settings: &UserSettings,
    saved: Arc<cache::resident::Saved>,
    budget: Arc<Budget>,
) -> Result<RepoLoader> {
    stores
        .resident_loader(settings, Arc::new(ResidentIndexStore { saved, budget }))
        .map_err(|_| Error::Unavailable)
}
