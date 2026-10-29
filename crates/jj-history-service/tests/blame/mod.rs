use super::*;
use crate::operations::source_bytes;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn blame_matches_jj_and_retains_pin_scope_and_source_bytes() {
    let (root, policy) = fixture();
    std::fs::write(root.path().join("note.md"), "first\nsecond\nthird\n").unwrap();
    jj(root.path(), &["describe", "-m", "original lines"]);
    jj(root.path(), &["new", "-m", "edit one line"]);
    std::fs::write(root.path().join("note.md"), "first\nupdated\nthird\n").unwrap();
    jj(root.path(), &["describe", "-m", "changed line"]);
    let commit = jj(
        root.path(),
        &["log", "-r", "@", "--no-graph", "-T", "commit_id"],
    );
    let operation = jj(
        root.path(),
        &["op", "log", "-n", "1", "--no-graph", "-T", "id"],
    );
    let expected = jj(
        root.path(),
        &[
            "--ignore-working-copy",
            "--at-op",
            &operation,
            "file",
            "annotate",
            "-r",
            &commit,
            "-T",
            "commit.commit_id() ++ \"\\n\"",
            "note.md",
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
    jj(root.path(), &["new", "-m", "external advance"]);
    std::fs::write(root.path().join("note.md"), "dirty outside pinned source\n").unwrap();
    let before = source_bytes(root.path());
    let mut query = jj_history_service::frames::BlameQuery {
        commit: commit.parse().unwrap(),
        path: "note.md".into(),
        continuation: None,
    };
    let mut complete = None;
    for _ in 0..128 {
        let page = host
            .blame(&main, "vault", handle, query.clone())
            .await
            .unwrap();
        assert_eq!(page.operation.to_string(), operation);
        assert_eq!(page.commit, commit);
        if page.complete {
            complete = Some(page);
            break;
        }
        query.continuation = page.continuation;
        assert!(query.continuation.is_some());
    }
    let page = complete.expect("bounded fixture completes");
    let mut actual = vec![String::new(); page.line_count];
    for range in &page.ranges {
        actual[range.start - 1..range.end - 1].fill(range.commit_id.clone());
    }
    assert_eq!(actual.join("\n"), expected);
    assert_eq!(source_bytes(root.path()), before);
    policy.0.generation.store(8, Ordering::SeqCst);
    assert!(matches!(
        host.blame(&main, "vault", handle, query.clone()).await,
        Err(Error::EpochChanged)
    ));
    policy.0.generation.store(7, Ordering::SeqCst);
    assert!(matches!(
        host.blame(&caller("popup"), "vault", handle, query.clone())
            .await,
        Err(Error::Denied)
    ));
    policy.0.enabled.store(false, Ordering::SeqCst);
    assert!(matches!(
        host.blame(&main, "vault", handle, query.clone()).await,
        Err(Error::Denied)
    ));
    host.close(&main, "vault", handle).await.unwrap();
    assert!(matches!(
        host.blame(&main, "vault", handle, query).await,
        Err(Error::Denied)
    ));
    host.shutdown().await.unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn blame_close_waits_for_the_owned_read_to_exit() {
    let (root, policy) = fixture();
    let commit = jj(
        root.path(),
        &["log", "-r", "@", "--no-graph", "-T", "commit_id"],
    );
    let cache = tempfile::tempdir().unwrap();
    let host = History::new(policy.clone(), cache.path().into(), Config::default()).unwrap();
    let main = caller("main");
    let operation = host.heads(&main, "vault").await.unwrap().heads[0];
    let handle = host.open(&main, "vault", operation).await.unwrap();
    ready(&host, &main, handle).await;
    policy.0.delay.store(2, Ordering::SeqCst);
    let reading = {
        let host = host.clone();
        let main = main.clone();
        tokio::spawn(async move {
            host.blame(
                &main,
                "vault",
                handle,
                jj_history_service::frames::BlameQuery {
                    commit: commit.parse().unwrap(),
                    path: "note.md".into(),
                    continuation: None,
                },
            )
            .await
        })
    };
    policy.0.entered.notified().await;
    let closing = {
        let host = host.clone();
        let main = main.clone();
        tokio::spawn(async move { host.close(&main, "vault", handle).await })
    };
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        while !matches!(
            host.status(&main, "vault", handle).await,
            Err(Error::Denied)
        ) {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    assert!(!closing.is_finished(), "close must retain the blocked read");
    policy.0.release.notify_one();
    assert!(reading.await.unwrap().is_err());
    closing.await.unwrap().unwrap();
    host.shutdown().await.unwrap();
}
