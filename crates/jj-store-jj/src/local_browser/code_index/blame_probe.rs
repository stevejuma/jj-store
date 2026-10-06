//! Opt-in measurements through production pinned stores, never source writes.
use super::*;
use crate::query::blame_probe::domain;
use jj_lib::{annotate::FileAnnotator, revset::RevsetExpression};
use std::{
    path::Path,
    time::{Duration, Instant, SystemTime},
};

// Metadata witness only: no source index repair, checkout snapshot or imports.
fn source_state(metadata: &Path) -> Vec<(PathBuf, u64, SystemTime)> {
    fn visit(path: &Path, output: &mut Vec<(PathBuf, u64, SystemTime)>) {
        let info = std::fs::symlink_metadata(path).unwrap();
        output.push((path.to_owned(), info.len(), info.modified().unwrap()));
        if info.is_dir() {
            for entry in std::fs::read_dir(path).unwrap() {
                visit(&entry.unwrap().path(), output);
            }
        }
    }
    let mut result = Vec::new();
    for path in [
        metadata.join("op_heads"),
        metadata.join("index"),
        metadata.join("store/extra"),
        metadata.parent().unwrap().join("working_copy"),
    ] {
        visit(&path, &mut result);
    }
    result.sort();
    result
}

struct Access(PathBuf);
#[async_trait::async_trait]
impl super::super::Access for Access {
    fn repository(&self) -> m::RepositoryId {
        m::RepositoryId::new([61; 16])
    }
    fn generation(&self) -> u64 {
        1
    }
    fn metadata_path(&self) -> &Path {
        &self.0
    }
    async fn authorize(&self) -> Result<()> {
        Ok(())
    }
}

async fn probe(
    executor: &Executor,
    access: Arc<dyn super::super::Access>,
    prepared: Arc<Prepared>,
    path: &'static str,
) -> Result<(Stats, usize, usize)> {
    let limits = executor.limits;
    executor
        .worker(access, move |access, budget| {
            Box::pin(async move {
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
                let commit = repo
                    .store()
                    .get_commit_async(&b::CommitId::from_hex(
                        "062871f1371b2e02a272ff5279c6479aff0a37ef",
                    ))
                    .await
                    .map_err(super::super::native::map_backend)?;
                eprintln!("blame probe {path}: setup stats {:?}", budget.stats());
                let mut annotator = FileAnnotator::from_commit(
                    &commit,
                    jj_lib::repo_path::RepoPath::from_internal_string(path).unwrap(),
                )
                .await
                .map_err(super::super::native::map_backend)?;
                let start = Instant::now();
                let ids = domain(repo.as_ref(), &annotator, 64)
                    .await
                    .map_err(|_| Error::Unavailable)?;
                eprintln!(
                    "blame probe {path}: domain={} in {:?}; starting compute",
                    ids.len(),
                    start.elapsed()
                );
                let result = tokio::task::unconstrained(
                    annotator.compute(repo.as_ref(), &RevsetExpression::commits(ids)),
                )
                .await;
                eprintln!(
                    "blame probe {path}: compute={result:?}, elapsed={:?}, stats={:?}",
                    start.elapsed(),
                    budget.stats()
                );
                result.map_err(|_| Error::Unavailable)?;
                budget.until(context.access.binding.authorize()).await??;
                let annotation = annotator.to_annotation();
                Ok((
                    budget.stats(),
                    annotation
                        .line_origins()
                        .filter(|(origin, _)| origin.is_ok())
                        .count(),
                    annotation.line_origins().count(),
                ))
            })
        })
        .await
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires JJ_STORE_BLAME_LINUX_METADATA pointing to the existing Linux .jj/repo"]
async fn linux_first_window_uses_guarded_pinned_reader() {
    let metadata = PathBuf::from(
        std::env::var_os("JJ_STORE_BLAME_LINUX_METADATA")
            .expect("explicit Linux metadata path required"),
    )
    .canonicalize()
    .unwrap();
    let before = source_state(&metadata);
    let access: Arc<dyn super::super::Access> = Arc::new(Access(metadata.clone()));
    let operation = "c29523a65dc707ab052cd66095169427e17ca54eebefc2ee497fb2fe6d78d5772a91ae9f071aa78e0565b585f2ab8b9dff734e3d47e93899683ee7070f66283e".parse().unwrap();
    let executor = Executor::new(1).unwrap();
    let cache = tempfile::tempdir().unwrap();
    let start = Instant::now();
    let prepared = Arc::new(
        executor
            .prepare_code_index(
                access.clone(),
                operation,
                cache.path().to_owned(),
                1024 * 1024 * 1024,
                preparation::Limits::default(),
                Cancellation::default(),
            )
            .await
            .unwrap(),
    );
    eprintln!("blame probe index preparation: {:?}", start.elapsed());
    let mut failures = Vec::new();
    for path in ["COPYING", "drivers/edac/altera_edac.c", "fs/inode.c"] {
        for run in 1..=2 {
            let start = Instant::now();
            let result = probe(&executor, access.clone(), prepared.clone(), path).await;
            let elapsed = start.elapsed();
            eprintln!("blame probe {path} run={run}: {elapsed:?}, {result:?}");
            if result.is_err() || (run == 2 && elapsed > Duration::from_secs(5)) {
                failures.push((path, run, elapsed));
            }
        }
    }
    assert_eq!(
        source_state(&metadata),
        before,
        "read probe changed source metadata"
    );
    assert!(
        failures.is_empty(),
        "Blame feasibility gate failed: {failures:?}"
    );
}
