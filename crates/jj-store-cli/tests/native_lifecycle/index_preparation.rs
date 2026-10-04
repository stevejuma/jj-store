use super::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn index_preparation_is_explicit_snapshot_bound_and_disposable() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let local = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let root = tempfile::tempdir().unwrap();
            let (repo, commit) = initial(&f, local.path(), work.path()).await;
            let read = pin(&f, oid(repo.op_id())).await;
            let exec = executor(
                root.path(),
                q::Limits {
                    max_inline_rebuild_commits: 0,
                    ..q::Limits::default()
                },
            );
            let before = f.session.snapshot().await.unwrap().manifest().clone();
            assert_eq!(
                read.graph(&exec, log("all()"), page(10)).await.unwrap_err(),
                q::Error::IndexRebuildRequired
            );
            let prepared = read
                .prepare_index(&exec, q::Cancellation::default())
                .await
                .unwrap();
            assert_eq!(prepared.operation, oid(repo.op_id()));
            assert_eq!(prepared.cache.state, q::cache::State::Built);
            let result = read.graph(&exec, log("all()"), page(10)).await.unwrap();
            assert_eq!(result.cache.state, q::cache::State::Reused);
            assert_eq!(commits(&result)[0].commit.id, cid(commit.id()));
            assert_eq!(*f.session.snapshot().await.unwrap().manifest(), before);
            let (later, _) = child(&repo, &commit, "later snapshot").await;
            let after_write = f.session.snapshot().await.unwrap().manifest().clone();
            // The task's snapshot remains explicit even if heads move while it runs.
            let again = read
                .prepare_index(&exec, q::Cancellation::default())
                .await
                .unwrap();
            assert_eq!(again.operation, oid(repo.op_id()));
            assert_eq!(again.cache.state, q::cache::State::Reused);
            let new_read = pin(&f, oid(later.op_id())).await;
            assert_eq!(
                new_read
                    .graph(&exec, log("all()"), page(10))
                    .await
                    .unwrap_err(),
                q::Error::IndexRebuildRequired
            );
            fs::remove_dir_all(root.path()).unwrap();
            assert_eq!(
                read.graph(&exec, log("all()"), page(10)).await.unwrap_err(),
                q::Error::IndexRebuildRequired
            );
            read.prepare_index(&exec, q::Cancellation::default())
                .await
                .unwrap();
            assert_eq!(
                read.graph(&exec, log("all()"), page(10))
                    .await
                    .unwrap()
                    .data,
                result.data
            );
            assert_eq!(*f.session.snapshot().await.unwrap().manifest(), after_write);
            let small = executor(
                root.path(),
                q::Limits {
                    max_index_commits: 1,
                    max_inline_rebuild_commits: 0,
                    ..q::Limits::default()
                },
            );
            assert_eq!(
                read.prepare_index(&small, q::Cancellation::default())
                    .await
                    .unwrap_err(),
                q::Error::IndexBudget
            );
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn index_preparation_cancels_pending_work_and_retains_shared_slot_until_exit() {
    let f = Fixture::new().await;
    let root = tempfile::tempdir().unwrap();
    let read = pin(&f, roots::ROOT_OPERATION).await;
    let access = Arc::new(PausedAccess {
        inner: read.clone(),
        calls: Default::default(),
        entered: Default::default(),
        release: Default::default(),
    });
    let shared = q::workers::Executors::new(
        root.path().to_owned(),
        q::Limits::default(),
        q::content::Limits::default(),
        q::cache::Policy::default(),
        1,
    )
    .unwrap();
    let cancellation = q::Cancellation::default();
    let task = {
        let executor = shared.graph().clone();
        let access = access.clone();
        let store = f.store.clone();
        let cancellation = cancellation.clone();
        tokio::spawn(async move { executor.prepare_index(store, access, cancellation).await })
    };
    tokio::time::timeout(Duration::from_secs(2), access.entered.notified())
        .await
        .unwrap();
    let busy = read
        .prepare_index(shared.graph(), q::Cancellation::default())
        .await;
    cancellation.cancel();
    let canceled = tokio::time::timeout(Duration::from_secs(1), task)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(busy.unwrap_err(), q::Error::Busy);
    assert_eq!(canceled.unwrap_err(), q::Error::Canceled);
    assert_eq!(shared.workers().status().in_use, 0);
    read.prepare_index(shared.graph(), q::Cancellation::default())
        .await
        .unwrap();
    assert_eq!(shared.workers().status().in_use, 0);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn graph_cursor_rebuild_reports_original_snapshot_after_later_publication() {
    use jj_store_service::pagination as p;
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let local = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let root = tempfile::tempdir().unwrap();
            let (repo, commit) = initial(&f, local.path(), work.path()).await;
            let read = pin(&f, oid(repo.op_id())).await;
            let exec = executor(
                root.path(),
                q::Limits {
                    max_inline_rebuild_commits: 0,
                    ..q::Limits::default()
                },
            );
            read.prepare_index(&exec, q::Cancellation::default())
                .await
                .unwrap();
            let pager =
                p::GraphPager::new(exec.clone(), p::Limits::default(), Arc::new(SystemClock))
                    .unwrap();
            let p::Start::Page(first) = pager
                .begin(f.session.clone(), None, log("all()"), 1)
                .await
                .unwrap()
            else {
                panic!("pinned page");
            };
            let cursor = first.next_cursor.unwrap();
            child(&repo, &commit, "later head").await;
            fs::remove_dir_all(root.path()).unwrap();
            let error = pager
                .resume(f.session.clone(), &cursor, None, log("all()"), 1)
                .await
                .unwrap_err();
            let p::Error::IndexRebuildRequired { snapshot } = error else {
                panic!("missing captured rebuild snapshot: {error:?}");
            };
            assert_eq!(*snapshot, first.snapshot);
            read.prepare_index(&exec, q::Cancellation::default())
                .await
                .unwrap();
            let next = pager
                .resume(f.session.clone(), &cursor, None, log("all()"), 1)
                .await
                .unwrap();
            assert_eq!(next.snapshot, first.snapshot);
        })
        .await;
}
