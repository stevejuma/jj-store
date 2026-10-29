//! Owned standard JJ code-index handle; advanced preparation remains separate.
#[cfg(test)]
mod blame_probe;
mod operation_reads;
mod source;
use super::*;
use crate::query::{Cancellation, code_index as standard};
use jj_lib::{index::Index as _, repo::Repo as _};
use std::path::PathBuf;

pub struct Prepared {
    directory: Directory,
    snapshot: Snapshot,
    pub stats: Stats,
    pub(super) index: Arc<standard::Index>,
    selection: [[u8; 32]; 2],
    metadata: PathBuf,
}
enum Directory {
    Temporary(tempfile::TempDir),
    Managed(PathBuf),
}
impl Directory {
    fn path(&self) -> &Path {
        match self {
            Self::Temporary(d) => d.path(),
            Self::Managed(p) => p,
        }
    }
}
impl Prepared {
    pub fn snapshot(&self) -> &Snapshot {
        &self.snapshot
    }
    pub fn path(&self) -> &Path {
        self.directory.path()
    }
    pub(super) async fn contains(&self, id: m::GitSha1Id) -> Result<bool> {
        self.index
            .raw
            .has_id(&b::CommitId::from_bytes(id.as_bytes()))
            .await
            .map_err(|_| Error::Unavailable)
    }
}
fn selection(context: &Context, operation: m::OperationId) -> Result<[[u8; 32]; 2]> {
    let operation = context
        .access
        .operations
        .get(&operation)
        .ok_or(Error::Incomplete)?;
    let view = context
        .access
        .views
        .get(&operation.view_id)
        .ok_or(Error::Incomplete)?;
    Ok([
        operation_store::fingerprint(operation, &context.budget)?,
        operation_store::view_fingerprint(view, &context.budget)?,
    ])
}
#[derive(Debug)]
struct Submodules;
impl jj_lib::submodule_store::SubmoduleStore for Submodules {
    fn name(&self) -> &str {
        "read-only"
    }
}
impl Executor {
    /// Prepare the core graph in private storage, reusing the source JJ index.
    /// The caller owns admission and retention; the source is never repaired.
    pub async fn prepare_code_index(
        &self,
        access: Arc<dyn Access>,
        operation: m::OperationId,
        root: PathBuf,
        reserved_bytes: u64,
        limits: preparation::Limits,
        cancellation: Cancellation,
    ) -> Result<Prepared> {
        self.prepare_code_inner(
            access,
            operation,
            root,
            reserved_bytes,
            limits,
            cancellation,
            None,
            None,
        )
        .await
    }
    pub async fn prepare_managed_code_index(
        &self,
        access: Arc<dyn Access>,
        operation: m::OperationId,
        namespace: crate::query::cache::managed::Namespace,
        limits: preparation::Limits,
        cancellation: Cancellation,
    ) -> Result<Prepared> {
        if namespace.binding()
            != &(crate::query::cache::managed::Binding::Local {
                repository: access.repository(),
                generation: access.generation(),
            })
        {
            return Err(Error::EpochChanged);
        }
        self.prepare_code_inner(
            access,
            operation,
            namespace.path(),
            namespace.bytes(),
            limits,
            cancellation,
            Some(namespace),
            None,
        )
        .await
    }
    pub async fn prepare_managed_path_index(
        &self,
        access: Arc<dyn Access>,
        namespace: crate::query::cache::managed::Namespace,
        limits: preparation::Limits,
        cancellation: Cancellation,
        previous: Arc<Prepared>,
    ) -> Result<Prepared> {
        if namespace.binding()
            != &(crate::query::cache::managed::Binding::Local {
                repository: access.repository(),
                generation: access.generation(),
            })
            || previous.snapshot.repository != access.repository()
            || previous.snapshot.generation != access.generation()
            || previous.metadata != access.metadata_path()
        {
            return Err(Error::EpochChanged);
        }
        self.prepare_code_inner(
            access,
            previous.snapshot.operation,
            namespace.path(),
            namespace.bytes(),
            limits,
            cancellation,
            Some(namespace),
            Some(previous),
        )
        .await
    }
    #[allow(clippy::too_many_arguments)]
    async fn prepare_code_inner(
        &self,
        access: Arc<dyn Access>,
        operation: m::OperationId,
        root: PathBuf,
        reserved_bytes: u64,
        limits: preparation::Limits,
        cancellation: Cancellation,
        namespace: Option<crate::query::cache::managed::Namespace>,
        previous: Option<Arc<Prepared>>,
    ) -> Result<Prepared> {
        limits.validate()?;
        if !root.is_absolute() || reserved_bytes == 0 || reserved_bytes > 2 * 1024 * 1024 * 1024 {
            return Err(Error::Limit);
        }
        let permit = self.workers.acquire_preparation()?;
        let budget = Arc::new(Budget::preparation(&limits, &cancellation));
        let cancel = cancellation.token().drop_guard();
        budget.until(access.authorize()).await??;
        let capture_limits = self.current_limits();
        let runtime = tokio::runtime::Handle::current();
        let result = tokio::task::spawn_blocking(move || {
            let _permit = permit;
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                budget
                    .until(async {
                        let root = std::fs::canonicalize(root).map_err(|_| Error::Unavailable)?;
                        let metadata = std::fs::canonicalize(access.metadata_path())
                            .map_err(|_| Error::Unavailable)?;
                        if root.starts_with(&metadata) {
                            return Err(Error::Invalid);
                        }
                        let mut context = Context::open(
                            access,
                            operation,
                            budget.clone(),
                            capture_limits,
                            true,
                            vec![],
                            false,
                        )
                        .await?;
                        let selected = selection(&context, operation)?;
                        if let Some(previous) = &previous {
                            Arc::get_mut(&mut context.access)
                                .ok_or(Error::Unavailable)?
                                .code_index = Some(previous.clone());
                        }
                        let directory = if namespace.is_some() {
                            Directory::Managed(root)
                        } else {
                            Directory::Temporary(
                                tempfile::Builder::new()
                                    .prefix(".code-index-")
                                    .tempdir_in(root)
                                    .map_err(|_| Error::Unavailable)?,
                            )
                        };
                        let raw = loader(&metadata)?;
                        let guard = scoped::Guard::new(
                            raw.store().clone(),
                            &context.access,
                            budget.clone(),
                            Limits {
                                max_history_visits: limits.max_commits,
                                ..capture_limits
                            },
                        );
                        guard.construction();
                        let store = guard.store();
                        let operation_source = source::Source::new(
                            raw.op_store().clone(),
                            context.access.clone(),
                            guard,
                            budget.clone(),
                            limits.max_metadata_records,
                        );
                        let selected_operation = operation_source.selected(operation).await?;
                        if !directory
                            .path()
                            .join("segments")
                            .try_exists()
                            .map_err(|_| Error::Unavailable)?
                        {
                            standard::seed(
                                &metadata.join("index"),
                                directory.path(),
                                &selected_operation,
                                reserved_bytes,
                                |bytes| {
                                    budget.work(1)?;
                                    budget.read_bytes(bytes)
                                },
                            )
                            .await?;
                        }
                        let index = standard::Index::load(
                            directory.path(),
                            &selected_operation,
                            &store,
                            limits.max_commits,
                            || budget.work(1),
                        )
                        .await?;
                        let index = if let Some(previous) = previous {
                            index
                                .extend_paths(&previous.index, directory.path(), &store, || {
                                    budget.work(1)
                                })
                                .await?
                        } else {
                            index
                        };
                        budget.until(context.access.binding.authorize()).await??;
                        if let Some(namespace) = &namespace {
                            namespace.finish()?;
                        }
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
                            index: Arc::new(index),
                            selection: selected,
                            metadata: context.access.binding.metadata_path().to_owned(),
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
    /// Pages hold the exact immutable index; no source index or full projection is opened.
    pub async fn read_code_index(
        &self,
        access: Arc<dyn Access>,
        prepared: Arc<Prepared>,
        request: Request,
    ) -> Result<Response> {
        if !matches!(
            request,
            Request::Graph(GraphRequest::Log { .. }, _)
                | Request::Graph(GraphRequest::Operations, _)
                | Request::OperationDiff(_)
                | Request::OperationChanges(_)
                | Request::StackHistory(_)
                | Request::Commit { .. }
                | Request::CommitSummary { .. }
                | Request::Paths(_)
                | Request::FileAt { .. }
                | Request::File { .. }
                | Request::Diff(_)
                | Request::DiffPaths(_)
                | Request::Conflicts(_)
        ) {
            return Err(Error::IndexRebuildRequired);
        }
        let limits = self.current_limits();
        self.worker(access, move |access, budget| {
            Box::pin(async move {
                if access.repository() != prepared.snapshot.repository
                    || access.metadata_path() != prepared.metadata
                {
                    return Err(Error::NotCommitted);
                }
                if access.generation() != prepared.snapshot.generation {
                    return Err(Error::EpochChanged);
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
                if selection(&context, operation)? != prepared.selection {
                    return Err(Error::Corrupt);
                }
                Arc::get_mut(&mut context.access)
                    .ok_or(Error::Unavailable)?
                    .code_index = Some(prepared.clone());
                if matches!(
                    request,
                    Request::Graph(GraphRequest::Operations, _)
                        | Request::OperationDiff(_)
                        | Request::OperationChanges(_)
                ) {
                    return operation_reads::read(context, operation, request).await;
                }
                let source = context
                    .scoped
                    .as_ref()
                    .map_or(&context.store, |guard| &guard.source)
                    .clone();
                let guard = scoped::Guard::new(source, &context.access, budget.clone(), limits);
                if !matches!(
                    request,
                    Request::Graph(_, _) | Request::StackHistory(_) | Request::Commit { .. }
                ) {
                    context.store = guard.store();
                    context.scoped = Some(guard);
                    return context.dispatch(operation, request, false).await;
                }
                let selected = operation_store::selected(&context, operation).await?;
                let raw = loader(context.access.binding.metadata_path())?;
                let settings = UserSettings::from_config(StackedConfig::with_defaults())
                    .map_err(|_| Error::Unavailable)?;
                let repo_loader = RepoLoader::new(
                    settings,
                    guard.store(),
                    selected.op_store().clone(),
                    raw.op_heads_store().clone(),
                    prepared.index.clone(),
                    Arc::new(Submodules),
                );
                let repo = repo_loader
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

impl Executor {
    pub async fn blame_code_index(
        &self,
        access: Arc<dyn Access>,
        prepared: Arc<Prepared>,
        cache: Arc<crate::query::blame::Cache>,
        key: crate::query::blame::Selection,
        continuation: Option<String>,
    ) -> Result<crate::query::blame::Page> {
        if key.repository != access.repository() || key.operation != prepared.snapshot.operation {
            return Err(Error::NotCommitted);
        }
        let mut limits = self.current_limits();
        limits.max_blob_bytes = crate::query::blame::MAX_FILE_BYTES as u64;
        let executor = Executor::with_workers(self.workers.clone(), limits);
        let permit = self.workers.acquire()?;
        executor
            .worker_with_permit(access, permit, move |access, budget| {
                Box::pin(async move {
                    if access.repository() != prepared.snapshot.repository
                        || access.metadata_path() != prepared.metadata
                    {
                        return Err(Error::NotCommitted);
                    }
                    if access.generation() != prepared.snapshot.generation {
                        return Err(Error::EpochChanged);
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
                    if selection(&context, operation)? != prepared.selection {
                        return Err(Error::Corrupt);
                    }
                    Arc::get_mut(&mut context.access)
                        .ok_or(Error::Unavailable)?
                        .code_index = Some(prepared.clone());
                    let source = context
                        .scoped
                        .as_ref()
                        .map_or(&context.store, |guard| &guard.source)
                        .clone();
                    let guard = scoped::Guard::new(source, &context.access, budget.clone(), limits);
                    let selected = operation_store::selected(&context, operation).await?;
                    let raw = loader(context.access.binding.metadata_path())?;
                    let settings = UserSettings::from_config(StackedConfig::with_defaults())
                        .map_err(|_| Error::Unavailable)?;
                    let loader = RepoLoader::new(
                        settings,
                        guard.store(),
                        selected.op_store().clone(),
                        raw.op_heads_store().clone(),
                        prepared.index.clone(),
                        Arc::new(Submodules),
                    );
                    let repo = loader
                        .load_at(&selected)
                        .await
                        .map_err(|_| Error::Unavailable)?;
                    crate::query::blame::read(
                        repo.as_ref(),
                        &cache,
                        key,
                        continuation.as_deref(),
                        || budget.check(),
                        || {
                            let s = budget.stats();
                            crate::query::Stats {
                                metadata_nodes: s.metadata_nodes,
                                object_bytes: s.object_bytes,
                                work: s.work,
                                ..Default::default()
                            }
                        },
                        async || {
                            budget.until(context.access.binding.authorize()).await??;
                            Ok(())
                        },
                    )
                    .await
                })
            })
            .await
    }
}
