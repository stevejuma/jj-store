use super::*;
use crate::operations::source_bytes;

fn query(path: &str, after: Option<local::ObjectKey>, limit: usize) -> local::Request {
    local::Request::Graph(
        local::GraphRequest::Log {
            revset: format!(
                "all() & files(root-file:{})",
                serde_json::to_string(path).unwrap()
            ),
            workspace: Some("default".into()),
            user_email: String::new(),
            at_time_millis: 1_790_000_000_000,
        },
        local::GraphWindow { limit, after },
    )
}
async fn compare(root: &Path, policy: Policy, path: &str) {
    let operation = jj(
        root,
        &[
            "--ignore-working-copy",
            "op",
            "log",
            "-n",
            "1",
            "--no-graph",
            "-T",
            "id",
        ],
    );
    let expected = jj(
        root,
        &[
            "--ignore-working-copy",
            "--at-op",
            &operation,
            "log",
            "-r",
            "all()",
            "--no-graph",
            "-T",
            "commit_id ++ '\n'",
            "--",
            &format!("root-file:{}", serde_json::to_string(path).unwrap()),
        ],
    );
    let cache = tempfile::tempdir().unwrap();
    let host = History::new(policy.clone(), cache.path().into(), Config::default()).unwrap();
    let main = caller("main");
    let handle = host
        .open(&main, "vault", operation.parse().unwrap())
        .await
        .unwrap();
    ready(&host, &main, handle).await;
    jj(root, &["new", "-m", "external later change"]);
    std::fs::write(root.join(path), "newer content outside the pin\n").unwrap();
    jj(root, &["describe", "-m", "external file edit"]);
    std::fs::write(root.join("dirty.txt"), "unsnapshotted\n").unwrap();
    let before = source_bytes(root);
    let mut after = None;
    let mut ids = Vec::new();
    loop {
        let local::Response::Graph(page) = host
            .read(&main, "vault", handle, query(path, after, 1))
            .await
            .unwrap()
        else {
            panic!()
        };
        assert_eq!(page.operation.to_string(), operation);
        let local::Nodes::Log(nodes) = page.data else {
            panic!()
        };
        ids.extend(nodes.into_iter().map(|n| n.commit.id.to_string()));
        after = page.next;
        if after.is_none() {
            break;
        }
    }
    assert_eq!(ids.join("\n"), expected);
    let local::Response::Graph(empty) = host
        .read(&main, "vault", handle, query("absent.md", None, 50))
        .await
        .unwrap()
    else {
        panic!()
    };
    assert!(matches!(empty.data, local::Nodes::Log(nodes) if nodes.is_empty()));
    assert_eq!(source_bytes(root), before);
    policy.0.enabled.store(false, Ordering::SeqCst);
    assert!(matches!(
        host.read(&main, "vault", handle, query(path, None, 1))
            .await,
        Err(Error::Denied)
    ));
    host.close(&main, "vault", handle).await.unwrap();
    host.shutdown().await.unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn file_history_matches_jj_across_path_batches_merges_and_deletion() {
    let (root, policy) = fixture();
    let first = jj(
        root.path(),
        &["log", "-r", "@", "--no-graph", "-T", "commit_id"],
    );
    jj(root.path(), &["new", &first, "-m", "left"]);
    std::fs::write(root.path().join("note.md"), "left\n").unwrap();
    jj(root.path(), &["describe", "-m", "left note"]);
    let left = jj(
        root.path(),
        &["log", "-r", "@", "--no-graph", "-T", "commit_id"],
    );
    jj(root.path(), &["new", &first, "-m", "right"]);
    std::fs::write(root.path().join("other.md"), "right\n").unwrap();
    jj(root.path(), &["describe", "-m", "right other"]);
    jj(root.path(), &["new", &left, "@", "-m", "clean integration"]);
    jj(root.path(), &["new", "-m", "remove note"]);
    std::fs::remove_file(root.path().join("note.md")).unwrap();
    jj(root.path(), &["describe", "-m", "removed"]);
    for _ in 0..70 {
        jj(root.path(), &["new", "-m", "unrelated history"]);
    }
    compare(root.path(), policy, "note.md").await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn file_history_retains_native_conflicted_merge_semantics() {
    let (root, policy) = fixture();
    let first = jj(
        root.path(),
        &["log", "-r", "@", "--no-graph", "-T", "commit_id"],
    );
    jj(root.path(), &["new", &first, "-m", "left"]);
    std::fs::write(root.path().join("note.md"), "left\n").unwrap();
    jj(root.path(), &["describe", "-m", "left"]);
    let left = jj(
        root.path(),
        &["log", "-r", "@", "--no-graph", "-T", "commit_id"],
    );
    jj(root.path(), &["new", &first, "-m", "right"]);
    std::fs::write(root.path().join("note.md"), "right\n").unwrap();
    jj(root.path(), &["describe", "-m", "right"]);
    jj(
        root.path(),
        &["new", &left, "@", "-m", "conflicted integration"],
    );
    compare(root.path(), policy, "note.md").await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn file_history_keeps_spaces_quotes_and_pattern_characters_literal() {
    let (root, policy) = fixture();
    let path = "notes [draft] \"quoted\".md";
    std::fs::rename(root.path().join("note.md"), root.path().join(path)).unwrap();
    jj(root.path(), &["describe", "-m", "literal filename"]);
    compare(root.path(), policy, path).await;
}
