use super::{ReadBinding, jj};
use jj_lib::{index::IndexStore as _, object_id::ObjectId as _};
use jj_store_jj::{local_browser as q, query::Cancellation};
use sha2::{Digest as _, Sha256};
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

fn prepared_index_files(
    root: &Path,
    selected: jj_store_model::OperationId,
) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut files = files(root);
    let operations = files
        .remove(Path::new("local-operations"))
        .expect("local preparation exports retained operation identities");
    assert!(operations.len() >= 104 + 128);
    assert_eq!((operations.len() - 104) % 128, 0);
    assert_eq!(&operations[..8], b"JSTLOP2\0");
    assert_eq!(&operations[8..72], selected.as_bytes());
    let checksum_at = operations.len() - 32;
    assert_eq!(
        &operations[checksum_at..],
        Sha256::digest(&operations[..checksum_at]).as_slice()
    );
    let records = &operations[72..checksum_at];
    assert!(
        records
            .chunks_exact(128)
            .any(|row| &row[..64] == selected.as_bytes())
    );
    for pair in records.chunks_exact(128).collect::<Vec<_>>().windows(2) {
        assert!(pair[0][..64] < pair[1][..64]);
    }
    // Every remaining file must be exactly the pinned upstream index output.
    files
}

/// Explicit external-repository qualification; only private derived files are written.
#[tokio::test]
#[ignore = "requires JJ_STORE_LARGE_REPO pointing at an existing JJ metadata directory"]
async fn registered_large_repository_preparation_smoke() {
    let metadata =
        PathBuf::from(std::env::var_os("JJ_STORE_LARGE_REPO").expect("explicit metadata path"));
    let heads_before = files(&metadata.join("op_heads"));
    let extras_before = files(&metadata.join("store/extra/heads"));
    let output = tempfile::tempdir().unwrap();
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(metadata.clone()));
    let executor = q::Executor::new(1).unwrap();
    let operation = executor.heads(access.clone()).await.unwrap().heads[0];
    let started = std::time::Instant::now();
    let preparation = tokio::spawn({
        let executor = executor.clone();
        let access = access.clone();
        let output = output.path().to_path_buf();
        async move {
            executor
                .prepare_index(
                    access,
                    operation,
                    output,
                    2 * 1024 * 1024 * 1024,
                    q::preparation::Limits::default(),
                    Cancellation::default(),
                )
                .await
        }
    });
    let interactive = executor
        .read(access.clone(), operation, q::Request::QueryInfo)
        .await
        .unwrap();
    assert!(matches!(interactive, q::Response::QueryInfo(_)));
    let prepared = Arc::new(preparation.await.unwrap().unwrap());
    eprintln!(
        "large preparation {:?}: {:?}",
        started.elapsed(),
        prepared.stats
    );
    let mut after = None;
    let mut ids = std::collections::BTreeSet::new();
    for number in 0..3 {
        let started = std::time::Instant::now();
        let page = executor
            .graph_prepared(
                access.clone(),
                prepared.clone(),
                q::GraphRequest::Log {
                    revset: "all()".into(),
                    workspace: None,
                    user_email: String::new(),
                    at_time_millis: 0,
                },
                q::GraphWindow { limit: 50, after },
            )
            .await
            .unwrap();
        eprintln!(
            "large page {number} {:?}: {:?}",
            started.elapsed(),
            page.stats
        );
        let q::Nodes::Log(nodes) = page.data else {
            panic!()
        };
        assert_eq!(nodes.len(), 50);
        for node in nodes {
            assert!(ids.insert(node.commit.id));
        }
        after = page.next;
        assert!(after.is_some());
    }
    assert_eq!(files(&metadata.join("op_heads")), heads_before);
    assert_eq!(files(&metadata.join("store/extra/heads")), extras_before);
    drop(prepared);
    assert_eq!(fs::read_dir(output.path()).unwrap().count(), 0);
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
    let expected_access: Arc<dyn q::Access> = Arc::new(ReadBinding(metadata.clone()));
    let log = || q::GraphRequest::Log {
        revset: "all()".into(),
        workspace: None,
        user_email: String::new(),
        at_time_millis: 0,
    };
    let q::Response::Graph(expected_page) = q::Executor::new(1)
        .unwrap()
        .read(
            expected_access,
            operation,
            q::Request::Graph(
                log(),
                q::GraphWindow {
                    limit: 100,
                    after: None,
                },
            ),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
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
    assert_eq!(prepared.snapshot().operation, operation);
    assert_eq!(
        prepared_index_files(prepared.path(), operation),
        expected_files
    );
    let prepared = Arc::new(prepared);
    let small = q::Executor::with_workers(
        jj_store_jj::query::workers::Workers::new(1).unwrap(),
        q::Limits {
            max_history_visits: 2,
            ..Default::default()
        },
    );
    let mut after = None;
    let mut joined = vec![];
    loop {
        let page = small
            .graph_prepared(
                access.clone(),
                prepared.clone(),
                log(),
                q::GraphWindow { limit: 1, after },
            )
            .await
            .unwrap();
        assert_eq!(page.operation, operation);
        assert!(page.stats.metadata_nodes < 12, "{:?}", page.stats);
        let q::Nodes::Log(nodes) = page.data else {
            panic!()
        };
        joined.extend(nodes);
        after = page.next;
        if after.is_none() {
            break;
        }
    }
    assert_eq!(q::Nodes::Log(joined), expected_page.data);
    // All subsequent pages use the verified memory index, not mutable cache files.
    fs::remove_dir_all(prepared.path().join("segments")).unwrap();
    let replay = small
        .graph_prepared(
            access.clone(),
            prepared.clone(),
            log(),
            q::GraphWindow {
                limit: 1,
                after: None,
            },
        )
        .await
        .unwrap();
    let q::Nodes::Log(expected_rows) = &expected_page.data else {
        panic!()
    };
    let q::Nodes::Log(replayed) = replay.data else {
        panic!()
    };
    assert_eq!(replayed, expected_rows[..1]);
    let denied = Arc::new(HeldBinding {
        path: metadata.clone(),
        reads: Default::default(),
        revoked: true.into(),
        entered: Default::default(),
        release: Default::default(),
    });
    assert!(matches!(
        small
            .graph_prepared(
                denied,
                prepared.clone(),
                log(),
                q::GraphWindow {
                    limit: 1,
                    after: None
                }
            )
            .await,
        Err(q::Error::Denied)
    ));

    assert!(prepared.stats.metadata_nodes > 3);
    assert_eq!(files(&metadata), before_metadata);
    assert_eq!(files(&source.path().join(".git/refs")), before_refs);
    assert_eq!(
        fs::read(source.path().join("dirty.txt")).unwrap(),
        b"never snapshot this\n"
    );
    // Same-size source mutation with restored mtime must invalidate retained
    // extra metadata even though the prepared index remains available.
    let segment = fs::read_dir(metadata.join("store/extra/heads"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .file_name();
    let segment = metadata.join("store/extra").join(segment);
    let original = fs::read(&segment).unwrap();
    let modified = fs::metadata(&segment).unwrap().modified().unwrap();
    let mut damaged = original.clone();
    *damaged.last_mut().unwrap() ^= 1;
    fs::write(&segment, damaged).unwrap();
    fs::File::options()
        .write(true)
        .open(&segment)
        .unwrap()
        .set_times(fs::FileTimes::new().set_modified(modified))
        .unwrap();
    assert!(matches!(
        small
            .graph_prepared(
                access.clone(),
                prepared.clone(),
                log(),
                q::GraphWindow {
                    limit: 100,
                    after: None
                }
            )
            .await,
        Err(q::Error::Corrupt)
    ));
    fs::write(&segment, original).unwrap();
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
    let unimported = Arc::new(unimported);
    executor
        .graph_prepared(
            Arc::new(ReadBinding(metadata.clone())),
            unimported,
            log(),
            q::GraphWindow {
                limit: 1,
                after: None,
            },
        )
        .await
        .unwrap();
    assert_eq!(files(&metadata), missing_extras);
    assert_eq!(files(&source.path().join(".git/refs")), before_refs);
}

#[tokio::test]
async fn prepared_local_log_preserves_conflicts_and_filtered_native_edges() {
    let source = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    fs::write(source.path().join("file"), "base\n").unwrap();
    jj(source.path(), &["describe", "-m", "Base"]);
    jj(source.path(), &["bookmark", "create", "base"]);
    jj(source.path(), &["new", "-m", "Left"]);
    fs::write(source.path().join("file"), "left\n").unwrap();
    jj(source.path(), &["bookmark", "create", "left"]);
    jj(source.path(), &["new", "base", "-m", "Right"]);
    fs::write(source.path().join("file"), "right\n").unwrap();
    jj(source.path(), &["bookmark", "create", "right"]);
    jj(source.path(), &["new", "left", "right", "-m", "Merge"]);
    let operation = jj(
        source.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    let metadata = source.path().join(".jj/repo");
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(metadata.clone()));
    let executor = q::Executor::new(1).unwrap();
    let prepared = Arc::new(
        executor
            .prepare_index(
                access.clone(),
                operation,
                output.path().into(),
                16 * 1024 * 1024,
                q::preparation::Limits::default(),
                Cancellation::default(),
            )
            .await
            .unwrap(),
    );
    // Compare with the pinned upstream cold builder. An existing incremental
    // index can order same-timestamp sibling commits differently from a cold
    // index. Require exact cold segment bytes as well as exact graph rows.
    let expected = tempfile::tempdir().unwrap();
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
    jj_lib::default_index::DefaultIndexStore::init(expected.path())
        .unwrap()
        .get_index_at_op(&op, loader.store())
        .await
        .unwrap();
    let expected_files = files(expected.path());
    assert_eq!(
        prepared_index_files(prepared.path(), operation),
        expected_files
    );
    for (relative, bytes) in expected_files {
        let path = metadata.join("index").join(relative);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, bytes).unwrap();
    }
    let before_refs = files(&source.path().join(".git/refs"));
    for expression in ["all()", "description(Left)", "ancestors(bookmarks(left))"] {
        let request = q::GraphRequest::Log {
            revset: expression.into(),
            workspace: Some("default".into()),
            user_email: "browser@example.invalid".into(),
            at_time_millis: 0,
        };
        let window = q::GraphWindow {
            limit: 100,
            after: None,
        };
        let q::Response::Graph(expected) = executor
            .read(
                access.clone(),
                operation,
                q::Request::Graph(request.clone(), window.clone()),
            )
            .await
            .unwrap()
        else {
            panic!()
        };
        let before = files(&metadata);
        let actual = executor
            .graph_prepared(access.clone(), prepared.clone(), request, window)
            .await
            .unwrap();
        assert_eq!(actual.data, expected.data, "{expression}");
        assert_eq!(actual.next, expected.next);
        assert_eq!(files(&metadata), before);
    }
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
