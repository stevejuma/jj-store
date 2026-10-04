use super::{ReadBinding, jj};
use jj_lib::{index::IndexStore as _, object_id::ObjectId as _};
use jj_store_jj::{local_browser as q, query::Cancellation};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

fn files(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    fn visit(path: &Path, root: &Path, out: &mut BTreeMap<PathBuf, Vec<u8>>) {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_dir() {
                visit(&entry.path(), root, out);
            } else {
                out.insert(
                    entry.path().strip_prefix(root).unwrap().into(),
                    fs::read(entry.path()).unwrap(),
                );
            }
        }
    }
    let mut out = BTreeMap::new();
    visit(root, root, &mut out);
    out
}

#[tokio::test]
async fn local_preparation_matches_upstream_without_trees_or_source_index_writes() {
    let source = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    let expected = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    fs::write(source.path().join("selected.txt"), "selected\n").unwrap();
    jj(source.path(), &["describe", "-m", "Original"]);
    jj(source.path(), &["describe", "-m", "Rewritten"]);
    jj(source.path(), &["new", "-m", "Second"]);
    let operation: jj_store_model::OperationId = jj(
        source.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    jj(source.path(), &["new", "-m", "Must not enter old index"]);
    let metadata = source.path().join(".jj/repo");
    let settings =
        jj_lib::settings::UserSettings::from_config(jj_lib::config::StackedConfig::with_defaults())
            .unwrap();
    let loader = jj_lib::repo::RepoLoader::init_from_file_system(
        &settings,
        &metadata,
        &jj_lib::default_backend_factories::default_backend_factories(),
    )
    .unwrap();
    let op = loader
        .load_operation(&jj_lib::op_store::OperationId::from_bytes(
            operation.as_bytes(),
        ))
        .await
        .unwrap();
    let expected_index = jj_lib::default_index::DefaultIndexStore::init(expected.path()).unwrap();
    expected_index
        .get_index_at_op(&op, loader.store())
        .await
        .unwrap();
    let expected_files = files(expected.path());
    let view = op.view().await.unwrap();
    let commit = loader
        .store()
        .get_commit_async(view.wc_commit_ids().values().next().unwrap())
        .await
        .unwrap();
    let tree = commit.tree_ids().as_resolved().unwrap().hex();
    fs::remove_file(
        source
            .path()
            .join(".git/objects")
            .join(&tree[..2])
            .join(&tree[2..]),
    )
    .unwrap();
    for entry in fs::read_dir(metadata.join("index/segments")).unwrap() {
        fs::write(entry.unwrap().path(), b"source index is unavailable").unwrap();
    }
    fs::write(source.path().join("dirty.txt"), "never snapshot this\n").unwrap();
    let before_metadata = files(&metadata);
    let before_refs = files(&source.path().join(".git/refs"));
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(metadata.clone()));
    let executor = q::Executor::new(1).unwrap();
    // The existing interactive graph path still preflights trees. Preparation
    // must be independently usable, without making that path's limits larger.
    assert!(
        executor
            .read(
                access.clone(),
                operation,
                q::Request::Graph(
                    q::GraphRequest::Log {
                        revset: "all()".into(),
                        workspace: None,
                        user_email: String::new(),
                        at_time_millis: 0,
                    },
                    q::GraphWindow {
                        limit: 1,
                        after: None
                    }
                )
            )
            .await
            .is_err()
    );
    let prepared = executor
        .prepare_index(
            access.clone(),
            operation,
            output.path().into(),
            16 * 1024 * 1024,
            q::preparation::Limits::default(),
            Cancellation::default(),
        )
        .await
        .unwrap();
    assert_eq!(prepared.snapshot.operation, operation);
    assert_eq!(files(prepared.path()), expected_files);
    assert!(prepared.stats.metadata_nodes > 3);
    assert_eq!(files(&metadata), before_metadata);
    assert_eq!(files(&source.path().join(".git/refs")), before_refs);
    assert_eq!(
        fs::read(source.path().join("dirty.txt")).unwrap(),
        b"never snapshot this\n"
    );
    let path = prepared.path().to_owned();
    drop(prepared);
    assert!(!path.exists());
    let limited = executor
        .prepare_index(
            access.clone(),
            operation,
            output.path().into(),
            16 * 1024 * 1024,
            q::preparation::Limits {
                max_commits: 1,
                ..Default::default()
            },
            Cancellation::default(),
        )
        .await;
    assert!(matches!(limited, Err(q::Error::IndexBudget)));
    assert_eq!(fs::read_dir(output.path()).unwrap().count(), 0);
    let canceled = Cancellation::default();
    canceled.cancel();
    assert!(matches!(
        executor
            .prepare_index(
                access.clone(),
                operation,
                output.path().into(),
                16 * 1024 * 1024,
                q::preparation::Limits::default(),
                canceled
            )
            .await,
        Err(q::Error::Canceled)
    ));
    assert_eq!(fs::read_dir(output.path()).unwrap().count(), 0);
    let heads = metadata.join("store/extra/heads");
    let selected_head = fs::read_dir(&heads)
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .file_name();
    fs::write(
        metadata.join("store/extra").join(selected_head),
        b"damaged table",
    )
    .unwrap();
    assert!(matches!(
        executor
            .prepare_index(
                access.clone(),
                operation,
                output.path().into(),
                16 * 1024 * 1024,
                q::preparation::Limits::default(),
                Cancellation::default()
            )
            .await,
        Err(q::Error::Corrupt)
    ));
    assert_eq!(fs::read_dir(output.path()).unwrap().count(), 0);
    for head in fs::read_dir(&heads).unwrap() {
        fs::remove_file(head.unwrap().path()).unwrap();
    }
    let missing_extras = files(&metadata);
    // Upstream read_commit would populate extra metadata and retention refs.
    let unimported = executor
        .prepare_index(
            access,
            operation,
            output.path().into(),
            16 * 1024 * 1024,
            q::preparation::Limits::default(),
            Cancellation::default(),
        )
        .await
        .unwrap();
    assert!(unimported.path().join("segments").is_dir());
    assert_eq!(files(&metadata), missing_extras);
    assert_eq!(files(&source.path().join(".git/refs")), before_refs);
}

struct HeldBinding {
    path: PathBuf,
    reads: std::sync::atomic::AtomicUsize,
    revoked: std::sync::atomic::AtomicBool,
    entered: tokio::sync::Notify,
    release: tokio::sync::Notify,
}
#[async_trait::async_trait]
impl q::Access for HeldBinding {
    fn repository(&self) -> jj_store_model::RepositoryId {
        jj_store_model::RepositoryId::new([20; 16])
    }
    fn generation(&self) -> u64 {
        1
    }
    fn metadata_path(&self) -> &Path {
        &self.path
    }
    async fn authorize(&self) -> q::Result<()> {
        use std::sync::atomic::Ordering;
        if self.reads.fetch_add(1, Ordering::SeqCst) == 2 {
            self.entered.notify_one();
            self.release.notified().await;
        }
        if self.revoked.load(Ordering::SeqCst) {
            return Err(q::Error::Denied);
        }
        Ok(())
    }
}

#[tokio::test]
async fn local_preparation_releases_canceled_workers_and_rechecks_binding() {
    let source = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    let operation = jj(
        source.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    let executor = q::Executor::new(1).unwrap();
    let root = output.path().to_owned();
    for revoke in [false, true] {
        let access = Arc::new(HeldBinding {
            path: source.path().join(".jj/repo"),
            reads: Default::default(),
            revoked: Default::default(),
            entered: Default::default(),
            release: Default::default(),
        });
        let cancellation = Cancellation::default();
        let pending = {
            let executor = executor.clone();
            let access = access.clone();
            let root = root.clone();
            let cancellation = cancellation.clone();
            tokio::spawn(async move {
                executor
                    .prepare_index(
                        access,
                        operation,
                        root,
                        16 * 1024 * 1024,
                        q::preparation::Limits::default(),
                        cancellation,
                    )
                    .await
            })
        };
        tokio::time::timeout(std::time::Duration::from_secs(2), access.entered.notified())
            .await
            .unwrap();
        assert!(matches!(
            executor
                .prepare_index(
                    access.clone(),
                    operation,
                    root.clone(),
                    16 * 1024 * 1024,
                    q::preparation::Limits::default(),
                    Cancellation::default()
                )
                .await,
            Err(q::Error::Busy)
        ));
        // The separate interactive worker remains available.
        executor
            .heads(Arc::new(ReadBinding(access.path.clone())))
            .await
            .unwrap();
        if revoke {
            access
                .revoked
                .store(true, std::sync::atomic::Ordering::SeqCst);
            access.release.notify_one();
        } else {
            cancellation.cancel();
        }
        let result = pending.await.unwrap();
        assert!(matches!(
            (revoke, result),
            (true, Err(q::Error::Denied)) | (false, Err(q::Error::Canceled))
        ));
        assert_eq!(fs::read_dir(&root).unwrap().count(), 0);
    }
}
