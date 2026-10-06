use super::{ReadBinding, jj};
use jj_store_jj::{local_browser as q, query::Cancellation};
use std::{fs, sync::Arc};

#[tokio::test]
async fn warm_code_index_does_not_walk_commit_bodies() {
    let source = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    for _ in 0..24 {
        jj(source.path(), &["new", "-m", "Indexed by JJ"]);
    }
    let metadata = source.path().join(".jj/repo");
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(metadata.clone()));
    let executor = q::Executor::new(1).unwrap();
    let operation = executor.heads(access.clone()).await.unwrap().heads[0];
    let before = super::preparation_acceptance::files(&metadata);
    let prepared = executor
        .prepare_code_index(
            access.clone(),
            operation,
            output.path().into(),
            16 * 1024 * 1024,
            q::preparation::Limits {
                max_metadata_records: 16,
                ..Default::default()
            },
            Cancellation::default(),
        )
        .await
        .unwrap();
    assert!(prepared.stats.metadata_nodes < 16, "{:?}", prepared.stats);
    assert_eq!(super::preparation_acceptance::files(&metadata), before);
    assert!(
        fs::read_dir(prepared.path().join("segments"))
            .unwrap()
            .next()
            .is_some()
    );
    assert!(!prepared.path().join("local-operations").exists());
    assert!(!prepared.path().join("local-reference-filter").exists());
    let prepared = Arc::new(prepared);
    let request = q::Request::Graph(
        log(),
        q::GraphWindow {
            limit: 5,
            after: None,
        },
    );
    let expected = executor
        .read(access.clone(), operation, request.clone())
        .await
        .unwrap();
    let actual = executor
        .read_code_index(access.clone(), prepared.clone(), request.clone())
        .await
        .unwrap();
    let (q::Response::Graph(expected), q::Response::Graph(actual)) = (expected, actual) else {
        panic!()
    };
    assert_eq!(actual.data, expected.data);
    assert!(actual.stats.metadata_nodes < 20, "{:?}", actual.stats);
    // The reader owns immutable memory, and doesn't reopen mutable disk files.
    fs::remove_dir_all(prepared.path().join("segments")).unwrap();
    executor
        .read_code_index(access, prepared, request)
        .await
        .unwrap();
    assert_eq!(super::preparation_acceptance::files(&metadata), before);
}

fn log() -> q::GraphRequest {
    q::GraphRequest::Log {
        revset: "all()".into(),
        workspace: None,
        user_email: String::new(),
        at_time_millis: 0,
    }
}

#[tokio::test]
async fn standard_code_construction_is_read_only_when_source_index_is_absent() {
    let source = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    jj(source.path(), &["new", "-m", "Read only"]);
    let metadata = source.path().join(".jj/repo");
    fs::remove_dir_all(metadata.join("index/segments")).unwrap();
    fs::remove_dir_all(metadata.join("index/op_links")).unwrap();
    let before = super::preparation_acceptance::files(&metadata);
    let refs = super::preparation_acceptance::files(&source.path().join(".git/refs"));
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(metadata.clone()));
    let executor = q::Executor::new(1).unwrap();
    let operation = executor.heads(access.clone()).await.unwrap().heads[0];
    let prepared = Arc::new(
        executor
            .prepare_code_index(
                access.clone(),
                operation,
                output.path().into(),
                16 * 1024 * 1024,
                Default::default(),
                Cancellation::default(),
            )
            .await
            .unwrap(),
    );
    let page = executor
        .read_code_index(
            access,
            prepared,
            q::Request::Graph(
                log(),
                q::GraphWindow {
                    limit: 10,
                    after: None,
                },
            ),
        )
        .await
        .unwrap();
    assert!(matches!(page, q::Response::Graph(_)));
    assert_eq!(super::preparation_acceptance::files(&metadata), before);
    assert_eq!(
        super::preparation_acceptance::files(&source.path().join(".git/refs")),
        refs
    );
}

#[tokio::test]
#[ignore = "requires JJ_STORE_LARGE_REPO pointing at an existing JJ metadata directory"]
async fn linux_standard_index_reuse_and_history_pages() {
    let metadata = std::path::PathBuf::from(
        std::env::var_os("JJ_STORE_LARGE_REPO").expect("explicit metadata path"),
    );
    let output = tempfile::tempdir().unwrap();
    let heads = super::preparation_acceptance::files(&metadata.join("op_heads"));
    let extras = super::preparation_acceptance::files(&metadata.join("store/extra/heads"));
    let index_before = super::preparation_acceptance::files(&metadata.join("index"));
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(metadata.clone()));
    let executor = q::Executor::new(1).unwrap();
    let operation = executor.heads(access.clone()).await.unwrap().heads[0];
    let start = std::time::Instant::now();
    let prepared = Arc::new(
        executor
            .prepare_code_index(
                access.clone(),
                operation,
                output.path().into(),
                2 * 1024 * 1024 * 1024,
                q::preparation::Limits {
                    max_metadata_records: 16,
                    ..Default::default()
                },
                Cancellation::default(),
            )
            .await
            .unwrap(),
    );
    eprintln!(
        "Linux warm reuse {:?}: {:?}",
        start.elapsed(),
        prepared.stats
    );
    assert!(prepared.stats.metadata_nodes < 16);
    let mut after = None;
    let mut seen = std::collections::BTreeSet::new();
    for number in 0..3 {
        let start = std::time::Instant::now();
        let q::Response::Graph(page) = executor
            .read_code_index(
                access.clone(),
                prepared.clone(),
                q::Request::Graph(log(), q::GraphWindow { limit: 50, after }),
            )
            .await
            .unwrap()
        else {
            panic!()
        };
        eprintln!(
            "Linux page {number} {:?}: {:?}",
            start.elapsed(),
            page.stats
        );
        assert_eq!(page.operation, operation);
        let q::Nodes::Log(nodes) = page.data else {
            panic!()
        };
        assert_eq!(nodes.len(), 50);
        assert!(page.stats.metadata_nodes < 200);
        for node in nodes {
            assert!(seen.insert(node.commit.id));
        }
        after = page.next;
        assert!(after.is_some());
    }
    assert_eq!(
        super::preparation_acceptance::files(&metadata.join("index")),
        index_before
    );
    assert_eq!(
        super::preparation_acceptance::files(&metadata.join("op_heads")),
        heads
    );
    assert_eq!(
        super::preparation_acceptance::files(&metadata.join("store/extra/heads")),
        extras
    );
}

#[tokio::test]
async fn indexed_path_metadata_finishes_with_snapshot_beyond_sixty_four_commits() {
    let source = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    fs::create_dir(source.path().join("old-directory")).unwrap();
    fs::write(source.path().join("old-directory/old.txt"), "original\n").unwrap();
    jj(source.path(), &["describe", "-m", "Original file change"]);
    for _ in 0..70 {
        jj(source.path(), &["new", "-m", "Unrelated commit"]);
    }
    let commit = jj(
        source.path(),
        &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
    )
    .parse()
    .unwrap();
    let metadata = source.path().join(".jj/repo");
    let before = super::preparation_acceptance::files(&metadata);
    let access: Arc<dyn q::Access> = Arc::new(CooperativeBinding(ReadBinding(metadata.clone())));
    let executor = q::Executor::new(1).unwrap();
    let operation = executor.heads(access.clone()).await.unwrap().heads[0];
    use jj_store_jj::query::cache::{
        Policy,
        managed::{Binding, Namespace},
    };
    let binding = Binding::Local {
        repository: access.repository(),
        generation: access.generation(),
    };
    let namespace = Namespace::acquire(output.path(), binding.clone(), Policy::default()).unwrap();
    let prepared = Arc::new(
        executor
            .prepare_managed_code_index(
                access.clone(),
                operation,
                namespace,
                Default::default(),
                Cancellation::default(),
            )
            .await
            .unwrap(),
    );
    let request = q::Request::Paths(q::paths::Request {
        commit,
        path: String::new(),
        recursive: false,
        include_last_change: true,
        last_change_mode: Default::default(),
        limit: 20,
        anchor: None,
        direction: Default::default(),
    });
    for _ in 0..2 {
        let q::Response::Paths(page) = executor
            .read_code_index(access.clone(), prepared.clone(), request.clone())
            .await
            .unwrap()
        else {
            panic!()
        };
        assert_eq!(page.entries[0].path, "old-directory");
        assert!(page.stats.metadata_nodes < 140, "{:?}", page.stats);
        assert!(matches!(&page.entries[0].last_change,
            q::paths::LastChange::Available { attribution: q::paths::Attribution::Snapshot, commit: id, description, .. }
            if *id == commit && description.trim() == "Unrelated commit"));
    }
    assert_eq!(super::preparation_acceptance::files(&metadata), before);
}

#[tokio::test]
async fn wide_path_metadata_finishes_in_one_shared_scan() {
    let source = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    fs::create_dir(source.path().join("old-directory")).unwrap();
    for row in 0..50 {
        fs::write(
            source
                .path()
                .join(format!("old-directory/file-{row:02}.txt")),
            "original\n",
        )
        .unwrap();
    }
    jj(
        source.path(),
        &["describe", "-m", "Original directory change"],
    );
    for row in 0..25 {
        jj(source.path(), &["new", "-m", "Unrelated change"]);
        fs::write(
            source.path().join("old-directory/unrelated.txt"),
            format!("{row}\n"),
        )
        .unwrap();
    }
    let commit = jj(
        source.path(),
        &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
    )
    .parse()
    .unwrap();
    let metadata = source.path().join(".jj/repo");
    let before = super::preparation_acceptance::files(&metadata);
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(metadata.clone()));
    let executor = q::Executor::new(1).unwrap();
    let operation = executor.heads(access.clone()).await.unwrap().heads[0];
    let prepared = Arc::new(
        executor
            .prepare_code_index(
                access.clone(),
                operation,
                output.path().into(),
                16 * 1024 * 1024,
                Default::default(),
                Cancellation::default(),
            )
            .await
            .unwrap(),
    );
    let request = q::Request::Paths(q::paths::Request {
        commit,
        path: "old-directory".into(),
        recursive: false,
        include_last_change: true,
        last_change_mode: Default::default(),
        limit: 50,
        anchor: None,
        direction: Default::default(),
    });
    let q::Response::Paths(first) = executor
        .read_code_index(access.clone(), prepared.clone(), request.clone())
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(first.entries.len(), 50);
    assert!(first.stats.work < 20_000, "{:?}", first.stats);
    assert!(first.entries.iter().all(|entry| matches!(
        &entry.last_change,
        q::paths::LastChange::Available { attribution: q::paths::Attribution::PathChange, description, .. }
            if description.trim() == "Original directory change"
    )));
    assert_eq!(super::preparation_acceptance::files(&metadata), before);
}

#[tokio::test]
async fn source_changed_path_index_is_copied_without_repair_or_history_tree_reads() {
    let source = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    fs::write(source.path().join("file.txt"), "original\n").unwrap();
    jj(source.path(), &["describe", "-m", "Original"]);
    jj(source.path(), &["new", "-m", "Unchanged"]);
    let commit = jj(
        source.path(),
        &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
    )
    .parse()
    .unwrap();
    jj(
        source.path(),
        &["debug", "index-changed-paths", "--limit", "100"],
    );
    let metadata = source.path().join(".jj/repo");
    let before = super::preparation_acceptance::files(&metadata);
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(metadata.clone()));
    let executor = q::Executor::new(1).unwrap();
    let operation = executor.heads(access.clone()).await.unwrap().heads[0];
    let prepared = Arc::new(
        executor
            .prepare_code_index(
                access.clone(),
                operation,
                output.path().into(),
                16 * 1024 * 1024,
                Default::default(),
                Cancellation::default(),
            )
            .await
            .unwrap(),
    );
    assert!(
        prepared
            .path()
            .join("changed_paths")
            .read_dir()
            .unwrap()
            .next()
            .is_some()
    );
    let q::Response::Paths(page) = executor
        .read_code_index(
            access,
            prepared,
            q::Request::Paths(q::paths::Request {
                commit,
                path: String::new(),
                recursive: false,
                include_last_change: true,
                last_change_mode: Default::default(),
                limit: 20,
                anchor: None,
                direction: Default::default(),
            }),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert!(
        matches!(&page.entries[0].last_change, q::paths::LastChange::Available { description, .. } if description.trim() == "Original")
    );
    assert!(page.stats.metadata_nodes < 8, "{:?}", page.stats);
    assert_eq!(super::preparation_acceptance::files(&metadata), before);
}

// Reproduce a backend admission future exhausting Tokio's cooperative budget
// while JJ polls file predicates synchronously through pollster.
struct CooperativeBinding(ReadBinding);
#[async_trait::async_trait]
impl q::Access for CooperativeBinding {
    fn repository(&self) -> jj_store_model::RepositoryId {
        self.0.repository()
    }
    fn generation(&self) -> u64 {
        self.0.generation()
    }
    fn metadata_path(&self) -> &std::path::Path {
        self.0.metadata_path()
    }
    async fn authorize(&self) -> q::Result<()> {
        for _ in 0..130 {
            tokio::task::consume_budget().await;
        }
        self.0.authorize().await
    }
}

#[tokio::test]
async fn indexed_merge_metadata_uses_integration_without_source_writes() {
    let source = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    fs::write(source.path().join("merged.txt"), "a\nb\nc\n").unwrap();
    jj(source.path(), &["describe", "-m", "Base"]);
    jj(source.path(), &["bookmark", "create", "base"]);
    jj(source.path(), &["new", "base", "-m", "Left edit"]);
    fs::write(source.path().join("merged.txt"), "A\nb\nc\n").unwrap();
    jj(source.path(), &["bookmark", "create", "left"]);
    jj(source.path(), &["new", "base", "-m", "Right edit"]);
    fs::write(source.path().join("merged.txt"), "a\nb\nC\n").unwrap();
    jj(source.path(), &["bookmark", "create", "right"]);
    jj(
        source.path(),
        &["new", "left", "right", "-m", "Merge both lines"],
    );
    let commit = jj(
        source.path(),
        &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
    )
    .parse()
    .unwrap();
    let metadata = source.path().join(".jj/repo");
    let before = super::preparation_acceptance::files(&metadata);
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(metadata.clone()));
    let executor = q::Executor::new(1).unwrap();
    let operation = executor.heads(access.clone()).await.unwrap().heads[0];
    let prepared = Arc::new(
        executor
            .prepare_code_index(
                access.clone(),
                operation,
                output.path().into(),
                16 * 1024 * 1024,
                Default::default(),
                Cancellation::default(),
            )
            .await
            .unwrap(),
    );
    let q::Response::Paths(page) = executor
        .read_code_index(
            access,
            prepared,
            q::Request::Paths(q::paths::Request {
                commit,
                path: String::new(),
                recursive: false,
                include_last_change: true,
                last_change_mode: Default::default(),
                limit: 20,
                anchor: None,
                direction: Default::default(),
            }),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(page.entries.len(), 1);
    assert!(
        matches!(&page.entries[0].last_change, q::paths::LastChange::Available { description, .. } if description.trim() == "Merge both lines"),
        "{:?}",
        page.entries[0].last_change
    );
    assert_eq!(super::preparation_acceptance::files(&metadata), before);
}

#[tokio::test]
async fn indexed_path_history_resumes_beyond_sixty_four_commits() {
    let source = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    fs::create_dir(source.path().join("old-directory")).unwrap();
    fs::write(source.path().join("old-directory/old.txt"), "original\n").unwrap();
    jj(source.path(), &["describe", "-m", "Original file change"]);
    for _ in 0..70 {
        jj(source.path(), &["new", "-m", "Unrelated commit"]);
    }
    let commit = jj(
        source.path(),
        &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
    )
    .parse()
    .unwrap();
    let metadata = source.path().join(".jj/repo");
    let before = super::preparation_acceptance::files(&metadata);
    let access: Arc<dyn q::Access> = Arc::new(CooperativeBinding(ReadBinding(metadata.clone())));
    let executor = q::Executor::new(1).unwrap();
    let operation = executor.heads(access.clone()).await.unwrap().heads[0];
    use jj_store_jj::query::cache::{
        Policy,
        managed::{Binding, Namespace},
    };
    let binding = Binding::Local {
        repository: access.repository(),
        generation: access.generation(),
    };
    let namespace = Namespace::acquire(output.path(), binding.clone(), Policy::default()).unwrap();
    let prepared = Arc::new(
        executor
            .prepare_managed_code_index(
                access.clone(),
                operation,
                namespace,
                Default::default(),
                Cancellation::default(),
            )
            .await
            .unwrap(),
    );
    let request = q::Request::Paths(q::paths::Request {
        commit,
        path: String::new(),
        recursive: false,
        include_last_change: true,
        last_change_mode: q::paths::LastChangeMode::History,
        limit: 20,
        anchor: None,
        direction: Default::default(),
    });
    let mut resolved = false;
    for batch in 0..4 {
        let q::Response::Paths(page) = executor
            .read_code_index(access.clone(), prepared.clone(), request.clone())
            .await
            .unwrap()
        else {
            panic!()
        };
        assert_eq!(page.entries[0].path, "old-directory");
        assert!(page.stats.metadata_nodes < 140, "{:?}", page.stats);
        if let q::paths::LastChange::Available { description, .. } = &page.entries[0].last_change {
            assert_eq!(description.trim(), "Original file change");
            resolved = true;
            let q::Response::Paths(cached) = executor
                .read_code_index(access.clone(), prepared.clone(), request.clone())
                .await
                .unwrap()
            else {
                panic!()
            };
            assert!(
                cached.stats.metadata_nodes < 6,
                "cached lookup must not revisit ancestry: {:?}",
                cached.stats
            );
            break;
        }
        assert!(
            batch < 2,
            "history must resume rather than replay the first batch"
        );
    }
    assert!(
        resolved,
        "bounded metadata batches must eventually reach history older than 64 commits"
    );
    assert_eq!(super::preparation_acceptance::files(&metadata), before);
}

#[tokio::test]
async fn indexed_merge_history_resolves_parent_contents_without_source_writes() {
    let source = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    fs::write(source.path().join("merged.txt"), "a\nb\nc\n").unwrap();
    jj(source.path(), &["describe", "-m", "Base"]);
    jj(source.path(), &["bookmark", "create", "base"]);
    jj(source.path(), &["new", "base", "-m", "Left edit"]);
    fs::write(source.path().join("merged.txt"), "A\nb\nc\n").unwrap();
    jj(source.path(), &["bookmark", "create", "left"]);
    jj(source.path(), &["new", "base", "-m", "Right edit"]);
    fs::write(source.path().join("merged.txt"), "a\nb\nC\n").unwrap();
    jj(source.path(), &["bookmark", "create", "right"]);
    jj(
        source.path(),
        &["new", "left", "right", "-m", "Merge both lines"],
    );
    let commit = jj(
        source.path(),
        &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
    )
    .parse()
    .unwrap();
    let metadata = source.path().join(".jj/repo");
    let before = super::preparation_acceptance::files(&metadata);
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(metadata.clone()));
    let executor = q::Executor::new(1).unwrap();
    let operation = executor.heads(access.clone()).await.unwrap().heads[0];
    let prepared = Arc::new(
        executor
            .prepare_code_index(
                access.clone(),
                operation,
                output.path().into(),
                16 * 1024 * 1024,
                Default::default(),
                Cancellation::default(),
            )
            .await
            .unwrap(),
    );
    let q::Response::Paths(page) = executor
        .read_code_index(
            access,
            prepared,
            q::Request::Paths(q::paths::Request {
                commit,
                path: String::new(),
                recursive: false,
                include_last_change: true,
                last_change_mode: q::paths::LastChangeMode::History,
                limit: 20,
                anchor: None,
                direction: Default::default(),
            }),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(page.entries.len(), 1);
    assert!(
        matches!(&page.entries[0].last_change, q::paths::LastChange::Available { description, .. } if description.trim() == "Right edit"),
        "{:?}",
        page.entries[0].last_change
    );
    assert_eq!(super::preparation_acceptance::files(&metadata), before);
}

#[tokio::test]
async fn progressive_blame_preserves_local_source_and_rejects_stale_generation() {
    let source = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    fs::write(source.path().join("file.txt"), "base\nunchanged\n").unwrap();
    jj(source.path(), &["describe", "-m", "Base"]);
    jj(source.path(), &["new", "-m", "Edit"]);
    fs::write(source.path().join("file.txt"), "edit\nunchanged\n").unwrap();
    let commit = jj(
        source.path(),
        &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
    )
    .trim()
    .to_owned();
    let metadata = source.path().join(".jj/repo");
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(metadata.clone()));
    let executor = q::Executor::new(2).unwrap();
    let operation = executor.heads(access.clone()).await.unwrap().heads[0];
    let before = super::preparation_acceptance::files(&metadata);
    let prepared = Arc::new(
        executor
            .prepare_code_index(
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
    let cache = Arc::new(jj_store_jj::query::blame::Cache::default());
    let key = jj_store_jj::query::blame::Selection {
        scope: "local-test".into(),
        repository: access.repository(),
        operation,
        commit,
        path: "file.txt".into(),
    };
    let page = executor
        .blame_code_index(
            access.clone(),
            prepared.clone(),
            cache.clone(),
            key.clone(),
            None,
        )
        .await
        .unwrap();
    assert!(page.complete);
    assert_eq!(page.line_count, 2);
    assert_eq!(
        page.ranges
            .iter()
            .map(|r| page.commits[&r.commit_id].description.trim())
            .collect::<Vec<_>>(),
        ["Edit", "Base"]
    );
    let cached = executor
        .blame_code_index(
            access.clone(),
            prepared.clone(),
            cache.clone(),
            key.clone(),
            None,
        )
        .await
        .unwrap();
    assert_eq!(cached.history_commits, page.history_commits);
    struct Changed {
        inner: Arc<dyn q::Access>,
        generation: u64,
        denied: bool,
    }
    #[async_trait::async_trait]
    impl q::Access for Changed {
        fn repository(&self) -> jj_store_model::RepositoryId {
            self.inner.repository()
        }
        fn generation(&self) -> u64 {
            self.generation
        }
        fn metadata_path(&self) -> &std::path::Path {
            self.inner.metadata_path()
        }
        async fn authorize(&self) -> q::Result<()> {
            if self.denied {
                Err(q::Error::Denied)
            } else {
                Ok(())
            }
        }
    }
    for (generation, denied, expected) in [
        (2, false, q::Error::EpochChanged),
        (1, true, q::Error::Denied),
    ] {
        let changed = Arc::new(Changed {
            inner: access.clone(),
            generation,
            denied,
        });
        assert_eq!(
            executor
                .blame_code_index(changed, prepared.clone(), cache.clone(), key.clone(), None)
                .await
                .unwrap_err(),
            expected
        );
    }
    assert_eq!(super::preparation_acceptance::files(&metadata), before);
    assert_eq!(
        fs::read_to_string(source.path().join("file.txt")).unwrap(),
        "edit\nunchanged\n"
    );
}
