use super::*;

fn executors(root: &std::path::Path, concurrency: usize) -> (q::GraphExecutor, ContentReader) {
    let shared = q::workers::Executors::new(
        root.to_owned(),
        q::Limits::default(),
        c::Limits::default(),
        q::cache::Policy::default(),
        concurrency,
    )
    .unwrap();
    (shared.graph().clone(), shared.content().clone())
}
fn page() -> q::Page {
    q::Page {
        limit: 10,
        after: None,
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn query_workers_file_lifetime_limits_graphs_and_content_together() {
    let f = ContentFixture::new().await;
    let root = tempfile::tempdir().unwrap();
    let (graph, content) = executors(root.path(), 1);
    let before = f.f.session.snapshot().await.unwrap().manifest().clone();
    let mut file = f.read.file(&content, f.file, None).await.unwrap();
    assert_eq!(
        f.read
            .graph(&graph, q::Request::Operations, page())
            .await
            .unwrap_err(),
        q::Error::Busy
    );
    assert_eq!(
        f.read.object(&content, f.tree.key()).await.unwrap_err(),
        q::Error::Busy
    );
    while file.next_chunk().await.unwrap().is_some() {}
    // The handle still retains its spool after EOF; admission follows ownership.
    assert_eq!(
        f.read
            .graph(&graph.clone(), q::Request::Operations, page())
            .await
            .unwrap_err(),
        q::Error::Busy
    );
    drop(file);
    f.read
        .graph(&graph, q::Request::Operations, page())
        .await
        .unwrap();
    f.read.object(&content.clone(), f.tree.key()).await.unwrap();
    assert_eq!(*f.f.session.snapshot().await.unwrap().manifest(), before);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn query_workers_graph_and_content_cancel_release_shared_capacity() {
    let f = ContentFixture::new().await;
    let root = tempfile::tempdir().unwrap();
    let (graph, content) = executors(root.path(), 1);
    for use_graph in [true, false] {
        let access = Arc::new(PauseContent {
            inner: f.read.clone(),
            remaining: 2.into(),
            entered: Default::default(),
            release: Default::default(),
        });
        let task = {
            let graph = graph.clone();
            let content = content.clone();
            let access = access.clone();
            let store = f.f.store.clone();
            tokio::spawn(async move {
                if use_graph {
                    graph
                        .execute(store, access, q::Request::Operations, page())
                        .await
                        .map(|_| ())
                } else {
                    content
                        .object(store, access, m::ObjectKey::Commit(roots::ROOT_COMMIT))
                        .await
                        .map(|_| ())
                }
            })
        };
        tokio::time::timeout(Duration::from_secs(2), access.entered.notified())
            .await
            .unwrap();
        // Collect the result before aborting so a failing assertion cannot leave
        // a test task waiting on an unnotified fixture gate.
        let competing = if use_graph {
            f.read.object(&content, f.tree.key()).await.map(|_| ())
        } else {
            f.read
                .graph(&graph, q::Request::Operations, page())
                .await
                .map(|_| ())
        };
        task.abort();
        assert!(task.await.unwrap_err().is_cancelled());
        assert_eq!(competing, Err(q::Error::Busy));
        let deadline = tokio::time::Instant::now() + Duration::from_secs(1);
        loop {
            match f.read.object(&content, f.tree.key()).await {
                Ok(_) => break,
                Err(q::Error::Busy) => {
                    assert!(
                        tokio::time::Instant::now() < deadline,
                        "capacity retained after cancellation"
                    );
                    tokio::time::sleep(Duration::from_millis(5)).await;
                }
                error => panic!("unexpected result: {error:?}"),
            }
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn query_workers_mixed_capacity_clones_and_errors_release_slots() {
    let f = ContentFixture::new().await;
    let root = tempfile::tempdir().unwrap();
    let shared = q::workers::Executors::new(
        root.path().to_owned(),
        q::Limits::default(),
        c::Limits::default(),
        q::cache::Policy::default(),
        2,
    )
    .unwrap();
    let clone = shared.clone();
    let workers = shared.workers().clone();
    assert_eq!(
        workers.status(),
        q::workers::Status {
            capacity: 2,
            in_use: 0
        }
    );
    let file = f.read.file(shared.content(), f.file, None).await.unwrap();
    assert_eq!(workers.status().in_use, 1);
    let access = Arc::new(PauseContent {
        inner: f.read.clone(),
        remaining: 2.into(),
        entered: Default::default(),
        release: Default::default(),
    });
    let task = {
        let executor = clone.graph().clone();
        let store = f.f.store.clone();
        let access = access.clone();
        tokio::spawn(async move {
            executor
                .execute(store, access, q::Request::Operations, page())
                .await
        })
    };
    tokio::time::timeout(Duration::from_secs(2), access.entered.notified())
        .await
        .unwrap();
    assert_eq!(workers.status().in_use, 2);
    let other_cache = tempfile::tempdir().unwrap();
    let other = q::GraphExecutor::with_workers(
        other_cache.path().to_owned(),
        q::Limits::default(),
        q::cache::Policy::default(),
        workers.clone(),
    )
    .unwrap();
    let saturated = f.read.graph(&other, q::Request::Operations, page()).await;
    access.release.notify_one();
    task.await.unwrap().unwrap();
    assert_eq!(saturated.unwrap_err(), q::Error::Busy);
    assert_eq!(workers.status().in_use, 1);
    f.read
        .graph(&other, q::Request::Operations, page())
        .await
        .unwrap();
    assert_eq!(workers.status().in_use, 1);
    drop(file);
    assert_eq!(workers.status().in_use, 0);
    // Both pre-worker validation and errors after worker entry return capacity.
    assert_eq!(
        f.read
            .graph(
                shared.graph(),
                q::Request::Operations,
                q::Page {
                    limit: 0,
                    after: None
                }
            )
            .await
            .unwrap_err(),
        q::Error::Limit
    );
    assert_eq!(
        f.read
            .object(
                clone.content(),
                m::ObjectKey::Commit(m::CommitId::new([222; 64]))
            )
            .await
            .unwrap_err(),
        q::Error::NotCommitted
    );
    assert_eq!(workers.status().in_use, 0);
    assert!(q::workers::Workers::new(0).is_err());
    assert!(q::workers::Workers::new(65).is_err());
}
