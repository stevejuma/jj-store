use super::*;
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn standard_native_index_extends_indexed_ancestor() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let local = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let root = tempfile::tempdir().unwrap();
            let (mut repo, mut commit) = initial(&f, local.path(), work.path()).await;
            for _ in 0..12 {
                (repo, commit) = child(&repo, &commit, "Existing history").await;
            }
            let old = pin(&f, oid(repo.op_id())).await;
            let exec = executor(root.path(), q::Limits::default());
            let index = root.path().join("shared");
            let prepared = Arc::new(
                exec.prepare_code_index(
                    f.store.clone(),
                    old.clone(),
                    index.clone(),
                    q::Cancellation::default(),
                )
                .await
                .unwrap(),
            );
            let first = prepared.stats.object_reads;
            assert!(first > 12, "{:?}", prepared.stats);
            let original = exec
                .execute_code_index(
                    f.store.clone(),
                    old.clone(),
                    prepared.clone(),
                    log("all()"),
                    page(5),
                )
                .await
                .unwrap();
            let (repo, _) = child(&repo, &commit, "New history").await;
            let newer = pin(&f, oid(repo.op_id())).await;
            let next = Arc::new(
                exec.prepare_code_index(
                    f.store.clone(),
                    newer.clone(),
                    index.clone(),
                    q::Cancellation::default(),
                )
                .await
                .unwrap(),
            );
            eprintln!(
                "Native full {:?}; incremental {:?}",
                prepared.stats, next.stats
            );
            assert!(next.stats.object_reads < first / 2, "{:?}", next.stats);
            let page = exec
                .execute_code_index(f.store.clone(), newer, next, log("all()"), page(5))
                .await
                .unwrap();
            assert_eq!(commits(&page)[0].commit.description, "New history");
            assert_eq!(
                exec.execute_code_index(
                    f.store.clone(),
                    old.clone(),
                    prepared,
                    log("all()"),
                    super::page(5)
                )
                .await
                .unwrap()
                .data,
                original.data
            );
            // Restart/load of the same operation performs no commit-body construction.
            let warm = exec
                .prepare_code_index(f.store.clone(), old, index, q::Cancellation::default())
                .await
                .unwrap();
            assert!(warm.stats.object_reads <= 1, "{:?}", warm.stats);
        })
        .await;
}
