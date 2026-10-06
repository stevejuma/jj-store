//! Native source adapter for the standard graph shared with local code reads.
use super::*;
use crate::query::*;

pub struct Prepared {
    pub operation: m::OperationId,
    pub stats: Stats,
    repository: m::RepositoryId,
    epoch: m::EpochId,
    pub(crate) index: Arc<Index>,
}
impl Prepared {
    pub(crate) fn validate(&self, access: &dyn ReadAccess) -> Result<()> {
        if access.repository() != self.repository || access.operation() != self.operation {
            return Err(Error::NotCommitted);
        }
        if access.epoch() != self.epoch {
            return Err(Error::EpochChanged);
        }
        Ok(())
    }
}
fn stores(
    store: Arc<dyn ObjectStore>,
    access: Arc<dyn ReadAccess>,
    budget: Arc<Budget>,
    max_object_bytes: u64,
    blame: bool,
) -> Result<NativeStores> {
    let objects = Arc::new(
        NativeObjects::new(
            Arc::new(if blame {
                ReadStore::new(store, access.clone(), budget.clone(), false)
                    .with_file_limit(blame::MAX_FILE_BYTES as u64)
            } else {
                ReadStore::new(store, access.clone(), budget.clone(), true)
            }),
            access.repository(),
            max_object_bytes,
        )
        .map_err(|_| Error::Limit)?,
    );
    let charge = budget.clone();
    let files = merge_files::Files::new(
        merge_files::Hash::Native,
        max_object_bytes.min(BackendLimits::default().max_bytes as u64) as usize,
        BackendLimits::default().max_bytes,
        move |bytes| {
            charge.work(1)?;
            charge.read_bytes(bytes)
        },
    );
    NativeStores::new(
        objects,
        Arc::new(ReadOnlyAuthority { access, budget }),
        BackendLimits::default(),
        OperationLimits::default(),
    )
    .map(|stores| stores.with_derived_files(files))
    .map_err(|_| Error::Unavailable)
}
impl GraphExecutor {
    pub async fn prepare_managed_code_index(
        &self,
        store: Arc<dyn ObjectStore>,
        access: Arc<dyn ReadAccess>,
        namespace: cache::managed::Namespace,
        cancellation: Cancellation,
    ) -> Result<Prepared> {
        if namespace.binding()
            != &(cache::managed::Binding::Native {
                repository: access.repository(),
                epoch: access.epoch(),
            })
        {
            return Err(Error::EpochChanged);
        }
        self.prepare_inner(
            store,
            access,
            namespace.path(),
            cancellation,
            Some(namespace),
            None,
        )
        .await
    }
    /// Caller owns this private namespace and its disk reservation. Reusing it
    /// across operations lets JJ extend the latest indexed ancestor.
    pub async fn prepare_code_index(
        &self,
        store: Arc<dyn ObjectStore>,
        access: Arc<dyn ReadAccess>,
        path: PathBuf,
        cancellation: Cancellation,
    ) -> Result<Prepared> {
        self.prepare_inner(store, access, path, cancellation, None, None)
            .await
    }
    pub async fn prepare_managed_path_index(
        &self,
        store: Arc<dyn ObjectStore>,
        access: Arc<dyn ReadAccess>,
        namespace: cache::managed::Namespace,
        cancellation: Cancellation,
        previous: Arc<Prepared>,
    ) -> Result<Prepared> {
        previous.validate(access.as_ref())?;
        if namespace.binding()
            != &(cache::managed::Binding::Native {
                repository: access.repository(),
                epoch: access.epoch(),
            })
        {
            return Err(Error::EpochChanged);
        }
        self.prepare_inner(
            store,
            access,
            namespace.path(),
            cancellation,
            Some(namespace),
            Some(previous.index.clone()),
        )
        .await
    }
    async fn prepare_inner(
        &self,
        store: Arc<dyn ObjectStore>,
        access: Arc<dyn ReadAccess>,
        path: PathBuf,
        cancellation: Cancellation,
        namespace: Option<cache::managed::Namespace>,
        previous: Option<Arc<Index>>,
    ) -> Result<Prepared> {
        if !path.is_absolute() {
            return Err(Error::Invalid);
        }
        let permit = self.workers.acquire_preparation()?;
        let mut budget = Budget::new(self.preparation.budget(self.limits))?;
        budget.cancellation = cancellation.token();
        let budget = Arc::new(budget);
        budget.authorize(access.as_ref()).await?;
        let cancel = cancellation.token().drop_guard();
        let runtime = tokio::runtime::Handle::current();
        let limits = self.limits;
        let result = tokio::task::spawn_blocking(move || {
            let _permit = permit;
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                let result = budget
                    .until(async {
                        fs::create_dir_all(&path).map_err(|_| Error::Unavailable)?;
                        if !fs::symlink_metadata(&path)
                            .map_err(|_| Error::Unavailable)?
                            .is_dir()
                        {
                            return Err(Error::Corrupt);
                        }
                        let lock = fs::OpenOptions::new()
                            .create(true)
                            .truncate(false)
                            .write(true)
                            .open(path.join(".lock"))
                            .map_err(|_| Error::Unavailable)?;
                        lock.try_lock().map_err(|_| Error::Busy)?;
                        let binding = format!(
                            "jj-store-code-index-v1\n{PROFILE}\n{}\n{}\n",
                            access.repository(),
                            access.epoch()
                        );
                        let marker = path.join(".binding");
                        match fs::OpenOptions::new()
                            .write(true)
                            .create_new(true)
                            .open(&marker)
                        {
                            Ok(mut file) => file
                                .write_all(binding.as_bytes())
                                .map_err(|_| Error::Unavailable)?,
                            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                                if !fs::symlink_metadata(&marker)
                                    .map_err(|_| Error::Unavailable)?
                                    .is_file()
                                {
                                    return Err(Error::Corrupt);
                                }
                                let mut bytes = vec![];
                                fs::File::open(&marker)
                                    .map_err(|_| Error::Unavailable)?
                                    .take(1025)
                                    .read_to_end(&mut bytes)
                                    .map_err(|_| Error::Unavailable)?;
                                if bytes != binding.as_bytes() {
                                    return Err(Error::Corrupt);
                                }
                            }
                            Err(_) => return Err(Error::Unavailable),
                        }
                        budget.start_code_index()?;
                        let stores = stores(
                            store,
                            access.clone(),
                            budget.clone(),
                            limits.max_object_bytes,
                            false,
                        )?;
                        let settings = UserSettings::from_config(StackedConfig::with_defaults())
                            .map_err(|_| Error::Unavailable)?;
                        let loader = stores
                            .resident_loader(
                                &settings,
                                Arc::new(jj_lib::default_index::DefaultIndexStore::load(&path)),
                            )
                            .map_err(|_| Error::Unavailable)?;
                        let selected = loader
                            .load_operation(&jj_lib::op_store::OperationId::from_bytes(
                                access.operation().as_bytes(),
                            ))
                            .await
                            .map_err(|_| Error::Unavailable)?;
                        let index = Index::load(
                            &path,
                            &selected,
                            loader.store(),
                            budget.limits.max_index_commits,
                            || budget.work(1),
                        )
                        .await?;
                        let index = if let Some(previous) = previous {
                            index
                                .extend_paths(&previous, &path, loader.store(), || budget.work(1))
                                .await?
                        } else {
                            index
                        };
                        budget.authorize(access.as_ref()).await?;
                        if let Some(namespace) = &namespace {
                            namespace.finish()?;
                        }
                        Ok(Prepared {
                            operation: access.operation(),
                            repository: access.repository(),
                            epoch: access.epoch(),
                            index: Arc::new(index),
                            stats: budget.stats(),
                        })
                    })
                    .await?;
                result.map_err(|e| budget.failure().unwrap_or(e))
            }))
        })
        .await
        .map_err(|_| Error::Unavailable)?;
        cancel.disarm();
        result
    }
    pub async fn execute_code_index(
        &self,
        store: Arc<dyn ObjectStore>,
        access: Arc<dyn ReadAccess>,
        prepared: Arc<Prepared>,
        request: Request,
        page: Page,
    ) -> Result<GraphPage> {
        if !matches!(request, Request::Log { .. }) {
            return Err(Error::IndexRebuildRequired);
        }
        if access.repository() != prepared.repository || access.operation() != prepared.operation {
            return Err(Error::NotCommitted);
        }
        if access.epoch() != prepared.epoch {
            return Err(Error::EpochChanged);
        }
        let permit = self.workers.acquire()?;
        let budget = Arc::new(Budget::new(self.limits)?);
        budget.authorize(access.as_ref()).await?;
        graphs::validate(&request, &page, &budget)?;
        let cancel = budget.cancellation.clone().drop_guard();
        let runtime = tokio::runtime::Handle::current();
        let limits = self.limits;
        let result = tokio::task::spawn_blocking(move || {
            let _permit = permit;
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                let result = budget
                    .until(async {
                        let stores = stores(
                            store,
                            access.clone(),
                            budget.clone(),
                            limits.max_object_bytes,
                            false,
                        )?;
                        let settings = UserSettings::from_config(StackedConfig::with_defaults())
                            .map_err(|_| Error::Unavailable)?;
                        let loader = stores
                            .resident_loader(&settings, prepared.index.clone())
                            .map_err(|_| Error::Unavailable)?;
                        let repo = stores
                            .load_at(&loader, access.operation())
                            .await
                            .map_err(|_| Error::Unavailable)?;
                        let (data, next) =
                            graphs::run(&repo, request, page, &access, &budget).await?;
                        let page = GraphPage {
                            version: 1,
                            operation: access.operation(),
                            data,
                            next,
                            stats: budget.stats(),
                            cache: cache::Report {
                                state: cache::State::Reused,
                                evicted: 0,
                                retained_entries: 1,
                                retained_bytes: 0,
                            },
                        };
                        budget.authorize(access.as_ref()).await?;
                        budget.complete_output(&page)?;
                        Ok(page)
                    })
                    .await?;
                result.map_err(|e| budget.failure().unwrap_or(e))
            }))
        })
        .await
        .map_err(|_| Error::Unavailable)?;
        cancel.disarm();
        result
    }
}

impl GraphExecutor {
    pub async fn blame_code_index(
        &self,
        store: Arc<dyn ObjectStore>,
        access: Arc<dyn ReadAccess>,
        prepared: Arc<Prepared>,
        cache: Arc<blame::Cache>,
        key: blame::Selection,
        continuation: Option<String>,
    ) -> Result<blame::Page> {
        prepared.validate(access.as_ref())?;
        if key.repository != access.repository() || key.operation != access.operation() {
            return Err(Error::NotCommitted);
        }
        let permit = self.workers.acquire()?;
        let budget = Arc::new(Budget::new(self.limits)?);
        budget.authorize(access.as_ref()).await?;
        let cancel = budget.cancellation.clone().drop_guard();
        let runtime = tokio::runtime::Handle::current();
        let result = tokio::task::spawn_blocking(move || {
            let _permit = permit;
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                let stores = stores(
                    store,
                    access.clone(),
                    budget.clone(),
                    budget.limits.max_object_bytes,
                    true,
                )?;
                let settings = UserSettings::from_config(StackedConfig::with_defaults())
                    .map_err(|_| Error::Unavailable)?;
                let loader = stores
                    .resident_loader(&settings, prepared.index.clone())
                    .map_err(|_| Error::Unavailable)?;
                let repo = stores
                    .load_at(&loader, access.operation())
                    .await
                    .map_err(|_| Error::Unavailable)?;
                let result = budget
                    .until(blame::read(
                        repo.as_ref(),
                        &cache,
                        key,
                        continuation.as_deref(),
                        || budget.check(),
                        || budget.stats(),
                        async || budget.authorize(access.as_ref()).await,
                    ))
                    .await?;
                budget.authorize(access.as_ref()).await?;
                let page = result.map_err(|e| budget.failure().unwrap_or(e))?;
                budget.complete_output(&page)?;
                Ok(page)
            }))
        })
        .await
        .map_err(|_| Error::Unavailable)?;
        cancel.disarm();
        result
    }
}
