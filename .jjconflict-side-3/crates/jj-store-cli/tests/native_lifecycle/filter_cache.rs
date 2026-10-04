use super::*;
use jj_store_jj::query::content as c;

fn request(operation: m::OperationId) -> c::references::Request {
    c::references::Request {
        operation,
        query: "needle".into(),
        exact: None,
        kind: c::references::Kind::Changes,
        limit: 1,
        anchor: None,
        direction: c::Direction::Forward,
    }
}
fn files(path: &std::path::Path) -> serde_json::Value {
    serde_json::from_slice::<serde_json::Value>(&fs::read(path.join("query-cache.json")).unwrap())
        .unwrap()["files"]
        .clone()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_filter_upgrade_and_pinned_cold_recovery_preserve_exact_generation() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let local = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let root = tempfile::tempdir().unwrap();
            let (base, a) = initial(&f, local.path(), work.path()).await;
            let (repo, _) = child(&base, &a, "needle").await;
            let read = pin(&f, oid(repo.op_id())).await;
            let executors = q::workers::Executors::new(
                root.path().into(),
                q::Limits::default(),
                c::Limits::default(),
                q::cache::Policy::default(),
                2,
            )
            .unwrap();
            // Inline index construction remains cheap and description-free.
            read.graph(executors.graph(), log("all()"), page(100))
                .await
                .unwrap();
            let path = cache_slot(root.path(), oid(repo.op_id()));
            assert!(!path.join("index/local-reference-filter").exists());
            assert_eq!(
                read.references(executors.content(), request(oid(repo.op_id())))
                    .await
                    .unwrap_err(),
                q::Error::IndexRebuildRequired
            );
            read.prepare_index(executors.graph(), q::Cancellation::default())
                .await
                .unwrap();
            assert!(path.join("index/local-reference-filter").is_file());
            let expected = read
                .references(executors.content(), request(oid(repo.op_id())))
                .await
                .unwrap();
            assert_eq!(expected.references.len(), 1);
            let complete = files(&path);
            let cold = q::workers::Executors::new(
                root.path().into(),
                q::Limits {
                    max_inline_rebuild_commits: 1,
                    ..Default::default()
                },
                c::Limits::default(),
                q::cache::Policy::default(),
                2,
            )
            .unwrap();
            // Another executor owns an open readonly DB. Failed inline recovery
            // must not transfer/delete its files or stop charging the slot.
            assert_eq!(
                read.references(cold.content(), request(oid(repo.op_id())))
                    .await
                    .unwrap_err(),
                q::Error::IndexRebuildRequired
            );
            assert_eq!(files(&path), complete);
            let recovered = read
                .prepare_index(cold.graph(), q::Cancellation::default())
                .await
                .unwrap();
            assert_eq!(recovered.cache.retained_entries, 1);
            assert_eq!(recovered.cache.retained_bytes, cache_bytes(root.path()));
            let actual = read
                .references(cold.content(), request(oid(repo.op_id())))
                .await
                .unwrap();
            assert_eq!(actual.references, expected.references);
            assert_eq!(files(&path), complete);
            drop(cold);
            drop(executors);
            // With no pin owner, restart recovery may move its generation privately.
            let restarted = q::workers::Executors::new(
                root.path().into(),
                q::Limits::default(),
                c::Limits::default(),
                q::cache::Policy::default(),
                2,
            )
            .unwrap();
            read.prepare_index(restarted.graph(), q::Cancellation::default())
                .await
                .unwrap();
            assert_eq!(
                read.references(restarted.content(), request(oid(repo.op_id())))
                    .await
                    .unwrap()
                    .references,
                expected.references
            );
            assert_eq!(files(&cache_slot(root.path(), oid(repo.op_id()))), complete);
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_filter_quota_reclaims_idle_residents_and_preserves_live_reader_files() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let local = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let root = tempfile::tempdir().unwrap();
            let (first, a) = initial(&f, local.path(), work.path()).await;
            let (second, b) = child(&first, &a, "second").await;
            let (third, _) = child(&second, &b, "third").await;
            let old = pin(&f, oid(first.op_id())).await;
            let policy = q::cache::Policy {
                max_entries: 2,
                max_entry_bytes: 64 * 1024 * 1024,
                max_repository_bytes: 128 * 1024 * 1024,
                max_total_bytes: 128 * 1024 * 1024,
                ..Default::default()
            };
            let exec = q::GraphExecutor::with_cache_policy(
                root.path().into(),
                q::Limits::default(),
                2,
                policy,
            )
            .unwrap();
            for operation in [first.op_id(), second.op_id()] {
                pin(&f, oid(operation))
                    .await
                    .prepare_index(&exec, q::Cancellation::default())
                    .await
                    .unwrap();
            }
            let expected = old.graph(&exec, log("all()"), page(100)).await.unwrap();
            let old_path = cache_slot(root.path(), oid(first.op_id()));
            let old_files = files(&old_path);
            let gate = Arc::new(HeldCommitRead {
                inner: f.store.clone(),
                target: m::ObjectKey::Commit(cid(a.id())).storage_path(REPO),
                entered: Default::default(),
                release: Default::default(),
            });
            let pending = {
                let exec = exec.clone();
                let gate = gate.clone();
                let old = old.clone();
                tokio::spawn(async move { exec.execute(gate, old, log("all()"), page(100)).await })
            };
            tokio::time::timeout(Duration::from_secs(2), gate.entered.notified())
                .await
                .unwrap();
            let prepared = pin(&f, oid(third.op_id()))
                .await
                .prepare_index(&exec, q::Cancellation::default())
                .await;
            let preserved = old_path.join("index/local-reference-filter").is_file();
            gate.release.notify_one();
            let actual = pending.await.unwrap().unwrap();
            let prepared = prepared.expect("idle filtered generation prevented admission");
            assert!(preserved);
            assert_eq!(actual.data, expected.data);
            assert_eq!(files(&old_path), old_files);
            assert_eq!(prepared.cache.evicted, 1);
            assert_eq!(prepared.cache.retained_entries, 2);
            assert!(prepared.cache.retained_bytes <= policy.max_total_bytes);
            assert_eq!(prepared.cache.retained_bytes, cache_bytes(root.path()));
        })
        .await;
}
