//! Admission on actual native index reads, without catalog enumeration.
mod secondary;
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
use std::{fmt, path::Path, sync::Mutex};

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
    pub fn is_new(&self, id: m::CommitId) -> bool {
        !self.commits.contains(&id)
    }
    pub fn len(&self) -> usize {
        self.commits.len()
    }
    pub fn finish_initial_load(&mut self) {
        self.inline = false;
    }
}

struct GuardedIndexStore {
    inner: DefaultIndexStore,
    budget: Arc<Budget>,
    path: PathBuf,
    generation: PathBuf,
    cache_policy: cache::Policy,
    resident: Option<Arc<cache::resident::Saved>>,
    secondary: Arc<secondary::Secondary>,
    primary: Mutex<Option<cache::resident::Data>>,
    access: Arc<dyn ReadAccess>,
    preparing: bool,
    required_filter: bool,
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
        if selected != self.access.operation() {
            return self
                .secondary
                .read(operation, store)
                .await
                .map_err(|e| IndexStoreError::Read(Box::new(self.budget.fail(e))));
        }
        if let Some(data) = self.primary.lock().unwrap().clone() {
            return data
                .reader(self.budget.clone())
                .map_err(|e| IndexStoreError::Read(Box::new(e)));
        }
        if let Some(saved) = &self.resident {
            if self.required_filter && !saved.index.has_reference_filter() {
                return Err(IndexStoreError::Read(Box::new(
                    self.budget.fail(Error::IndexRebuildRequired),
                )));
            }
            if !(self.preparing
                && self.cache_policy.reference_filters()
                && saved.index.operations.is_none())
            {
                let data = saved.data();
                *self.primary.lock().unwrap() = Some(data.clone());
                return data
                    .reader(self.budget.clone())
                    .map_err(|e| IndexStoreError::Read(Box::new(e)));
            }
        }
        let missing_filter = !self
            .path
            .join(super::prepared::filters::NAME)
            .try_exists()
            .map_err(|e| IndexStoreError::Read(Box::new(e)))?;
        if self.required_filter && missing_filter && !self.preparing {
            return Err(IndexStoreError::Read(Box::new(
                self.budget.fail(Error::IndexRebuildRequired),
            )));
        }
        let missing_operations = [
            super::prepared::operations::NAME,
            super::prepared::operations::CHILDREN_NAME,
            super::prepared::operations::HISTORY_NAME,
        ]
        .iter()
        .try_fold(false, |missing, name| {
            Ok::<_, IndexStoreError>(
                missing
                    || !self
                        .path
                        .join(name)
                        .try_exists()
                        .map_err(|e| IndexStoreError::Read(Box::new(e)))?,
            )
        })?;
        if (self.preparing
            && self.cache_policy.reference_filters()
            && (missing_filter || missing_operations))
            || !self
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
            let prepare_operations =
                self.preparing && self.cache_policy.reference_filters() && missing_operations;
            if prepare_operations {
                // Explicit upgrade owns writable private staging and full quota.
                // Preserve existing native index/filter bytes; replace only an
                // incomplete operation projection before its create-new writer.
                for name in [
                    super::prepared::operations::NAME,
                    super::prepared::operations::CHILDREN_NAME,
                    super::prepared::operations::HISTORY_NAME,
                ] {
                    self.budget
                        .work(1)
                        .map_err(|e| IndexStoreError::Read(Box::new(e)))?;
                    match std::fs::remove_file(self.path.join(name)) {
                        Ok(()) => {}
                        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                        Err(error) => return Err(IndexStoreError::Read(Box::new(error))),
                    }
                }
            }
            super::prepared::build(
                &self.path,
                operation,
                store,
                &self.budget,
                bytes,
                !self.preparing,
                super::prepared::Features {
                    filtering: self.preparing
                        && self.cache_policy.reference_filters()
                        && missing_filter,
                    operations: prepare_operations,
                },
            )
            .await
            .map_err(|e| IndexStoreError::Read(Box::new(self.budget.fail(e))))?;
            cache::constructed::verify_primary(
                &self.path,
                selected,
                self.cache_policy,
                &self.budget,
            )
            .map_err(|e| IndexStoreError::Read(Box::new(self.budget.fail(e))))?;
        }
        let data = {
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
            let index = projection::Prepared::build(
                raw.clone(),
                &view,
                store,
                self.access.as_ref(),
                &self.budget,
                &self.generation,
                &self.path,
                selected,
            )
            .await
            .map_err(|e| IndexStoreError::Read(Box::new(self.budget.fail(e))))?;
            cache::resident::Data {
                index,
                memory: self.budget.index_memory.clone(),
            }
        };
        *self.primary.lock().unwrap() = Some(data.clone());
        data.reader(self.budget.clone())
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
pub(super) fn loader(
    stores: &NativeStores,
    settings: &UserSettings,
    path: &Path,
    root: PathBuf,
    budget: Arc<Budget>,
    cache_policy: cache::Policy,
    resident: Option<Arc<cache::resident::Saved>>,
    residents: Arc<cache::resident::Residents>,
    access: Arc<dyn ReadAccess>,
    preparing: bool,
    required_filter: bool,
) -> Result<RepoLoader> {
    stores
        .verify_cache_binding(path)
        .map_err(|_| Error::Unavailable)?;
    let secondary = Arc::new(secondary::Secondary {
        root,
        policy: cache_policy,
        residents,
        access: access.clone(),
        budget: budget.clone(),
    });
    let mut factories = default_backend_factories();
    let generation = path.to_owned();
    factories.merge(stores.factories());
    factories.add_index_store(
        "default",
        Box::new(move |_, path| {
            Ok(Box::new(GuardedIndexStore {
                inner: DefaultIndexStore::load(path),
                budget: budget.clone(),
                path: path.to_owned(),
                generation: generation.clone(),
                cache_policy,
                resident: resident.clone(),
                secondary: secondary.clone(),
                primary: Mutex::default(),
                access: access.clone(),
                preparing,
                required_filter,
            }))
        }),
    );
    RepoLoader::init_from_file_system(settings, path, &factories).map_err(|_| Error::Unavailable)
}

struct ResidentIndexStore {
    saved: Arc<cache::resident::Saved>,
    secondary: Arc<secondary::Secondary>,
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
        store: &Arc<Store>,
    ) -> IndexStoreResult<Box<dyn ReadonlyIndex>> {
        self.budget
            .check()
            .map_err(|e| IndexStoreError::Read(Box::new(e)))?;
        if operation.id().as_bytes() != self.saved.operation.as_bytes() {
            return self
                .secondary
                .read(operation, store)
                .await
                .map_err(|e| IndexStoreError::Read(Box::new(self.budget.fail(e))));
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
    root: PathBuf,
    cache_policy: cache::Policy,
    residents: Arc<cache::resident::Residents>,
    access: Arc<dyn ReadAccess>,
) -> Result<RepoLoader> {
    let secondary = Arc::new(secondary::Secondary {
        root,
        policy: cache_policy,
        residents,
        access,
        budget: budget.clone(),
    });
    stores
        .resident_loader(
            settings,
            Arc::new(ResidentIndexStore {
                saved,
                secondary,
                budget,
            }),
        )
        .map_err(|_| Error::Unavailable)
}
