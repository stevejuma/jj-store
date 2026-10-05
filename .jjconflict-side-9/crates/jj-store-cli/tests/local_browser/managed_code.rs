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
