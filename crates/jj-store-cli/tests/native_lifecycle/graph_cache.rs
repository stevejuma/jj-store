use super::*;

struct HeldCommitRead {
    inner: Arc<dyn jj_store_object::ObjectStore>,
    target: String,
    entered: tokio::sync::Notify,
    release: tokio::sync::Notify,
}
#[async_trait::async_trait]
impl jj_store_object::ObjectStore for HeldCommitRead {
    async fn head(
        &self,
        key: &jj_store_object::Key,
    ) -> jj_store_object::Result<jj_store_object::Metadata> {
        self.inner.head(key).await
    }
    async fn get(
        &self,
        key: &jj_store_object::Key,
        range: Option<jj_store_object::ByteRange>,
    ) -> jj_store_object::Result<jj_store_object::Download> {
        if key.as_str() == self.target {
            self.entered.notify_one();
            self.release.notified().await;
        }
        self.inner.get(key, range).await
    }
    async fn put(
        &self,
        _: &jj_store_object::Key,
        _: jj_store_object::Condition,
        _: &jj_store_object::StagedUpload,
    ) -> jj_store_object::Result<jj_store_object::WriteOutcome> {
        panic!("query wrote native state")
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn warm_graph_survives_concurrent_preparation_and_cache_eviction() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let local = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let root = tempfile::tempdir().unwrap();
            let (repo, a) = initial(&f, local.path(), work.path()).await;
            let old = pin(&f, oid(repo.op_id())).await;
            let (later, _) = child(&repo, &a, "later").await;
            let newer = pin(&f, oid(later.op_id())).await;
            let exec = q::GraphExecutor::with_cache_policy(
                root.path().to_owned(),
                q::Limits::default(),
                2,
                q::cache::Policy {
                    max_entries: 1,
                    ..Default::default()
                },
            )
            .unwrap();
            let expected = old.graph(&exec, log("all()"), page(100)).await.unwrap();
            let gate = Arc::new(HeldCommitRead {
                inner: f.store.clone(),
                target: m::ObjectKey::Commit(cid(a.id())).storage_path(REPO),
                entered: Default::default(),
                release: Default::default(),
            });
            let pending = {
                let graph = exec.clone();
                let gate = gate.clone();
                let old = old.clone();
                tokio::spawn(async move { graph.execute(gate, old, log("all()"), page(100)).await })
            };
            tokio::time::timeout(Duration::from_secs(2), gate.entered.notified())
                .await
                .unwrap();
            let prepared = newer.prepare_index(&exec, q::Cancellation::default()).await;
            gate.release.notify_one();
            let result = pending.await.unwrap().unwrap();
            let prepared = prepared.expect("warm graph held the preparation namespace lock");
            assert_eq!(prepared.cache.evicted, 1);
            assert_eq!(result.operation, expected.operation);
            assert_eq!(result.data, expected.data);
            assert_eq!(
                newer
                    .graph(&exec, log("all()"), page(100))
                    .await
                    .unwrap()
                    .operation,
                oid(later.op_id())
            );
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn warm_graph_starts_while_another_operation_is_preparing() {
    use jj_store_service::administration::{TokenChange, TokenRequest};
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let local = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let root = tempfile::tempdir().unwrap();
            let (repo, a) = initial(&f, local.path(), work.path()).await;
            let old = pin(&f, oid(repo.op_id())).await;
            let (later, b) = child(&repo, &a, "later").await;
            let newer = pin(&f, oid(later.op_id())).await;
            let exec = q::GraphExecutor::with_cache_policy(
                root.path().to_owned(),
                q::Limits::default(),
                2,
                q::cache::Policy {
                    max_entries: 1,
                    ..Default::default()
                },
            )
            .unwrap();
            let expected = old.graph(&exec, log("all()"), page(100)).await.unwrap();
            let gate = Arc::new(HeldCommitRead {
                inner: f.store.clone(),
                target: m::ObjectKey::Commit(cid(b.id())).storage_path(REPO),
                entered: Default::default(),
                release: Default::default(),
            });
            let pending = {
                let graph = exec.clone();
                let gate = gate.clone();
                let newer = newer.clone();
                tokio::spawn(async move {
                    graph
                        .prepare_index(gate, newer, q::Cancellation::default())
                        .await
                })
            };
            tokio::time::timeout(Duration::from_secs(2), gate.entered.notified())
                .await
                .unwrap();
            let reader_session = f.session(f.reader.clone(), Protection::default());
            let OpenRead::Pinned(reader) = reader_session
                .open_read(Some(oid(repo.op_id())), ReadLimits::default())
                .await
                .unwrap()
            else {
                panic!("pinned reader")
            };
            let reader = Arc::new(reader);
            let result = reader.graph(&exec, log("all()"), page(100)).await;
            let revoked = f
                .session
                .change_token(&TokenRequest {
                    request_id: m::RequestId::new([210; 16]),
                    expected: f.session.snapshot().await.unwrap().manifest().snapshot(),
                    change: TokenChange::Revoke {
                        token: f.reader.id(),
                    },
                })
                .await;
            let denied = reader.graph(&exec, log("all()"), page(100)).await;
            gate.release.notify_one();
            pending.await.unwrap().unwrap();
            revoked.unwrap();
            assert_eq!(denied.unwrap_err(), q::Error::Denied);
            let result = result.expect("preparation blocked an already verified history page");
            assert_eq!(result.operation, expected.operation);
            assert_eq!(result.data, expected.data);
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn graph_cache_corrupt_operation_link_rebuilds_without_publication() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let graphs = tempfile::tempdir().unwrap();
            let (repo, _) = initial(&f, cache.path(), work.path()).await;
            let read = pin(&f, oid(repo.op_id())).await;
            let exec = executor(graphs.path(), q::Limits::default());
            let first = read.graph(&exec, log("all()"), page(100)).await.unwrap();
            let before = f.session.snapshot().await.unwrap().manifest().clone();
            let path = cache_slot(graphs.path(), oid(repo.op_id()));
            fs::write(
                path.join("index/op_links").join(repo.op_id().hex()),
                [255, 255, 255],
            )
            .unwrap();
            let recovered = read.graph(&exec, log("all()"), page(100)).await.unwrap();
            assert_eq!(recovered.data, first.data);
            assert_eq!(*f.session.snapshot().await.unwrap().manifest(), before);
        })
        .await;
}

fn cache_root(root: &std::path::Path) -> std::path::PathBuf {
    root.join("7c41cdeb16b6b321c64e789a966b6adf723816a5")
        .join(jj_store_jj::factories::PROFILE)
        .join("query-cache-v1")
}
pub(super) fn cache_slot(root: &std::path::Path, operation: m::OperationId) -> std::path::PathBuf {
    fs::read_dir(cache_root(root))
        .unwrap()
        .filter_map(|e| {
            let path = e.unwrap().path();
            let bytes = fs::read(path.join("query-cache.json")).ok()?;
            let value: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
            (value["operation"] == operation.to_string()).then_some(path)
        })
        .next()
        .expect("retained operation cache")
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn graph_cache_checksums_missing_files_and_receipts_rebuild_exact_snapshots() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let local = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let root = tempfile::tempdir().unwrap();
            let (repo, a) = initial(&f, local.path(), work.path()).await;
            let read = pin(&f, oid(repo.op_id())).await;
            let exec = executor(root.path(), q::Limits::default());
            let first = read.graph(&exec, log("all()"), page(100)).await.unwrap();
            assert_eq!(first.cache.state, q::cache::State::Built);
            let warm = read.graph(&exec, log("all()"), page(100)).await.unwrap();
            assert_eq!(warm.cache.state, q::cache::State::Reused);
            let (_later, _) = child(&repo, &a, "must not enter the old snapshot").await;
            let before = f.session.snapshot().await.unwrap().manifest().clone();
            for damage in 0..5 {
                let slot = cache_slot(root.path(), oid(repo.op_id()));
                let segment = fs::read_dir(slot.join("index/segments"))
                    .unwrap()
                    .next()
                    .unwrap()
                    .unwrap()
                    .path();
                match damage {
                    0 => {
                        // Same-sized damage must be caught before native decoding.
                        let mut bytes = fs::read(&segment).unwrap();
                        let last = bytes.len() - 1;
                        bytes[last] ^= 0xff;
                        fs::write(&segment, bytes).unwrap();
                    }
                    1 => fs::remove_file(&segment).unwrap(),
                    2 => fs::write(slot.join("query-cache.json"), b"partial receipt").unwrap(),
                    3 => fs::remove_file(slot.join("query-cache.json")).unwrap(),
                    4 => fs::write(slot.join("index/type"), b"unknown-index").unwrap(),
                    _ => unreachable!(),
                }
                let recovered = read.graph(&exec, log("all()"), page(100)).await.unwrap();
                assert_eq!(
                    recovered.cache.state,
                    q::cache::State::Rebuilt,
                    "damage {damage}"
                );
                assert_eq!(recovered.data, first.data, "damage {damage}");
                assert_eq!(recovered.operation, first.operation);
            }
            assert_eq!(*f.session.snapshot().await.unwrap().manifest(), before);
            fs::remove_dir_all(root.path()).unwrap();
            let cold = read.graph(&exec, log("all()"), page(100)).await.unwrap();
            assert_eq!(cold.cache.state, q::cache::State::Built);
            assert_eq!(cold.data, first.data);
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn graph_cache_retention_evicts_lru_and_bounds_bytes_files_and_slots() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let local = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let root = tempfile::tempdir().unwrap();
            let (a_repo, a) = initial(&f, local.path(), work.path()).await;
            let (b_repo, b) = child(&a_repo, &a, "B").await;
            let (c_repo, _) = child(&b_repo, &b, "C").await;
            let a_read = pin(&f, oid(a_repo.op_id())).await;
            let b_read = pin(&f, oid(b_repo.op_id())).await;
            let c_read = pin(&f, oid(c_repo.op_id())).await;
            let policy = q::cache::Policy {
                max_entries: 2,
                max_entry_bytes: 64 * 1024,
                max_total_bytes: 128 * 1024,
                ..q::cache::Policy::default()
            };
            let exec = q::GraphExecutor::with_cache_policy(
                root.path().to_owned(),
                q::Limits::default(),
                2,
                policy,
            )
            .unwrap();
            let before = f.session.snapshot().await.unwrap().manifest().clone();
            let a_data = a_read
                .graph(&exec, log("all()"), page(100))
                .await
                .unwrap()
                .data;
            let b_data = b_read
                .graph(&exec, log("all()"), page(100))
                .await
                .unwrap()
                .data;
            let warm = a_read.graph(&exec, log("all()"), page(100)).await.unwrap();
            assert_eq!(warm.data, a_data);
            assert_eq!(warm.cache.state, q::cache::State::Reused);
            let b_path = cache_slot(root.path(), oid(b_repo.op_id()));
            let third = c_read.graph(&exec, log("all()"), page(100)).await.unwrap();
            assert_eq!(third.cache.retained_entries, 2);
            assert_eq!(third.cache.evicted, 1);
            assert!(third.cache.retained_bytes <= policy.max_total_bytes);
            assert_eq!(
                third.cache.retained_bytes,
                cache_bytes(&cache_root(root.path()))
            );
            // The slot may be reused; the evicted identity must no longer be retained.
            assert_ne!(
                serde_json::from_slice::<serde_json::Value>(
                    &fs::read(b_path.join("query-cache.json")).unwrap()
                )
                .unwrap()["operation"],
                oid(b_repo.op_id()).to_string()
            );
            let rebuilt = b_read.graph(&exec, log("all()"), page(100)).await.unwrap();
            assert_eq!(rebuilt.data, b_data);
            assert_eq!(rebuilt.cache.retained_entries, 2);
            assert_eq!(rebuilt.cache.evicted, 1);
            // Reserve a full entry before a cold build; total capacity permits one.
            let tight = q::GraphExecutor::with_cache_policy(
                root.path().to_owned(),
                q::Limits::default(),
                2,
                q::cache::Policy {
                    max_entries: 4,
                    max_total_bytes: 64 * 1024,
                    ..policy
                },
            )
            .unwrap();
            // A cold root-operation query reserves space before native index construction.
            let other = Fixture::new().await;
            let other_read = pin(&other, roots::ROOT_OPERATION).await;
            let other_page = other_read
                .graph(&tight, log("all()"), page(100))
                .await
                .unwrap();
            assert_eq!(other_page.cache.retained_entries, 1);
            assert!(other_page.cache.retained_bytes <= 64 * 1024);
            for policy in [
                q::cache::Policy {
                    max_entry_bytes: 1,
                    max_total_bytes: 1,
                    ..policy
                },
                q::cache::Policy {
                    max_files: 1,
                    ..policy
                },
            ] {
                let empty = tempfile::tempdir().unwrap();
                let exec = q::GraphExecutor::with_cache_policy(
                    empty.path().to_owned(),
                    q::Limits::default(),
                    1,
                    policy,
                )
                .unwrap();
                assert_eq!(
                    a_read
                        .graph(&exec, log("all()"), page(100))
                        .await
                        .unwrap_err(),
                    q::Error::Limit
                );
                assert_eq!(
                    fs::read_dir(cache_root(empty.path())).unwrap().count(),
                    1,
                    "failed build leaves only stable lock"
                );
            }
            for policy in [
                q::cache::Policy {
                    max_entries: 0,
                    ..policy
                },
                q::cache::Policy {
                    max_files: 4097,
                    ..policy
                },
                q::cache::Policy {
                    max_total_bytes: 1,
                    ..policy
                },
            ] {
                assert!(
                    q::GraphExecutor::with_cache_policy(
                        root.path().to_owned(),
                        q::Limits::default(),
                        1,
                        policy
                    )
                    .is_err()
                );
            }
            assert_eq!(*f.session.snapshot().await.unwrap().manifest(), before);
        })
        .await;
}
fn cache_bytes(path: &std::path::Path) -> u64 {
    fs::read_dir(path)
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_dir() {
                cache_bytes(&entry.path())
            } else {
                entry.metadata().unwrap().len()
            }
        })
        .sum()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_preparation_fits_small_cache_and_preserves_historical_scopes() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let local = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let root = tempfile::tempdir().unwrap();
            let oracle = tempfile::tempdir().unwrap();
            let (repo, a) = initial(&f, local.path(), work.path()).await;
            let (repo, b) = child(&repo, &a, "B").await;
            let (repo, _) = child(&repo, &b, "C").await;
            let read = pin(&f, oid(repo.op_id())).await;
            let mut query = log("at_operation(@-, all()) | at_operation(@--, all())");
            if let q::Request::Log { workspace, .. } = &mut query {
                *workspace = Some("fixture".into());
            }
            let expected = read
                .graph(
                    &executor(oracle.path(), q::Limits::default()),
                    query.clone(),
                    page(100),
                )
                .await
                .unwrap();
            let policy = q::cache::Policy {
                max_entry_bytes: 64 * 1024,
                max_repository_bytes: 128 * 1024,
                max_total_bytes: 128 * 1024,
                ..Default::default()
            };
            let exec = q::GraphExecutor::with_cache_policy(
                root.path().to_owned(),
                q::Limits::default(),
                2,
                policy,
            )
            .unwrap();
            let before = f.session.snapshot().await.unwrap().manifest().clone();
            let prepared = read
                .prepare_index(&exec, q::Cancellation::default())
                .await
                .expect("small native preparation must not require a disk database");
            assert!(prepared.cache.retained_bytes <= policy.max_entry_bytes);
            let actual = read.graph(&exec, query.clone(), page(100)).await.unwrap();
            assert_eq!(actual.data, expected.data);
            assert!(actual.cache.retained_bytes <= policy.max_entry_bytes);
            assert_eq!(
                read.graph(&exec, query, page(100)).await.unwrap().data,
                expected.data
            );
            assert_eq!(*f.session.snapshot().await.unwrap().manifest(), before);
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn historical_predicates_reuse_separately_prepared_residents_without_slot_growth() {
    use jj_store_service::administration::{TokenChange, TokenRequest};
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let local = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let root = tempfile::tempdir().unwrap();
            let oracle = tempfile::tempdir().unwrap();
            let (first, a) = initial(&f, local.path(), work.path()).await;
            let (second, b) = child(&first, &a, "B").await;
            let (third, _) = child(&second, &b, "C").await;
            let read = pin(&f, oid(third.op_id())).await;
            let mut query = log("at_operation(@-, all()) | at_operation(@--, all())");
            if let q::Request::Log { workspace, .. } = &mut query {
                *workspace = Some("fixture".into());
            }
            let expected = read
                .graph(
                    &executor(oracle.path(), q::Limits::default()),
                    query.clone(),
                    page(100),
                )
                .await
                .unwrap();
            let exec = executor(
                root.path(),
                q::Limits {
                    max_inline_rebuild_commits: 1,
                    ..Default::default()
                },
            );
            for repo in [&first, &second, &third] {
                pin(&f, oid(repo.op_id()))
                    .await
                    .prepare_index(&exec, q::Cancellation::default())
                    .await
                    .unwrap();
            }
            let path = cache_slot(root.path(), oid(third.op_id()));
            let files = || {
                serde_json::from_slice::<serde_json::Value>(
                    &fs::read(path.join("query-cache.json")).unwrap(),
                )
                .unwrap()["files"]
                    .clone()
            };
            let before_files = files();
            let before = f.session.snapshot().await.unwrap().manifest().clone();
            let session = f.session(f.reader.clone(), Protection::default());
            let OpenRead::Pinned(reader) = session
                .open_read(Some(oid(third.op_id())), ReadLimits::default())
                .await
                .unwrap()
            else {
                panic!("pinned reader")
            };
            let reader = Arc::new(reader);
            let actual = reader
                .graph(&exec, query.clone(), page(100))
                .await
                .expect("prepared historical operations must not use interactive reconstruction");
            assert_eq!(actual.data, expected.data);
            assert_eq!(
                files(),
                before_files,
                "historical residents must not be copied into the selected slot"
            );
            for repo in [&first, &second] {
                fs::remove_dir_all(
                    cache_slot(root.path(), oid(repo.op_id())).join("index/segments"),
                )
                .unwrap();
            }
            assert_eq!(
                reader
                    .graph(&exec, query.clone(), page(100))
                    .await
                    .unwrap()
                    .data,
                expected.data
            );
            assert_eq!(*f.session.snapshot().await.unwrap().manifest(), before);
            f.session
                .change_token(&TokenRequest {
                    request_id: m::RequestId::new([218; 16]),
                    expected: f.session.snapshot().await.unwrap().manifest().snapshot(),
                    change: TokenChange::Revoke {
                        token: f.reader.id(),
                    },
                })
                .await
                .unwrap();
            assert_eq!(
                reader.graph(&exec, query, page(100)).await.unwrap_err(),
                q::Error::Denied
            );
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn resident_indexes_preserve_multiple_historical_operation_scopes() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let local = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let root = tempfile::tempdir().unwrap();
            let cold = tempfile::tempdir().unwrap();
            let (repo, a) = initial(&f, local.path(), work.path()).await;
            let (repo, b) = child(&repo, &a, "B").await;
            let (repo, _) = child(&repo, &b, "C").await;
            let read = pin(&f, oid(repo.op_id())).await;
            let mut query = log("at_operation(@-, all()) | at_operation(@--, all())");
            if let q::Request::Log { workspace, .. } = &mut query {
                *workspace = Some("fixture".into());
            }
            let expected = read
                .graph(
                    &executor(cold.path(), q::Limits::default()),
                    query.clone(),
                    page(100),
                )
                .await
                .unwrap();
            let exec = executor(root.path(), q::Limits::default());
            read.prepare_index(&exec, q::Cancellation::default())
                .await
                .unwrap();
            let warm = read.graph(&exec, query.clone(), page(100)).await.unwrap();
            assert_eq!(warm.data, expected.data);
            assert_eq!(
                read.graph(&exec, query, page(100)).await.unwrap().data,
                expected.data
            );
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn graph_cache_recovery_rechecks_permission_and_serializes_independent_executors() {
    use jj_store_service::administration::{TokenChange, TokenRequest};
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let root = tempfile::tempdir().unwrap();
            let reader = f.session(f.reader.clone(), Protection::default());
            let OpenRead::Pinned(read) =
                reader.open_read(None, ReadLimits::default()).await.unwrap()
            else {
                unreachable!()
            };
            let read = Arc::new(read);
            let exec = executor(root.path(), q::Limits::default());
            read.graph(&exec, log("all()"), page(100)).await.unwrap();
            let path = cache_slot(root.path(), roots::ROOT_OPERATION);
            let marker = path.join("index/type");
            fs::write(&marker, b"broken index").unwrap();
            f.session
                .change_token(&TokenRequest {
                    request_id: m::RequestId::new([116; 16]),
                    expected: f.session.snapshot().await.unwrap().manifest().snapshot(),
                    change: TokenChange::Revoke {
                        token: f.reader.id(),
                    },
                })
                .await
                .unwrap();
            let before = f.session.snapshot().await.unwrap().manifest().clone();
            assert_eq!(
                read.graph(&exec, log("all()"), page(100))
                    .await
                    .unwrap_err(),
                q::Error::Denied
            );
            assert_eq!(
                fs::read(&marker).unwrap(),
                b"broken index",
                "denied query did not touch cache"
            );
            let admin = pin(&f, roots::ROOT_OPERATION).await;
            let healed = admin.graph(&exec, log("all()"), page(100)).await.unwrap();
            assert_eq!(healed.cache.state, q::cache::State::Rebuilt);
            let lock = fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(cache_root(root.path()).join("lock"))
                .unwrap();
            lock.try_lock().unwrap();
            let separate = executor(root.path(), q::Limits::default());
            assert_eq!(
                admin
                    .graph(&separate, log("all()"), page(100))
                    .await
                    .unwrap_err(),
                q::Error::Busy
            );
            drop(lock);
            // Recover a partial build left by a terminated previous process.
            let abandoned = cache_root(root.path()).join(".building-interrupted");
            fs::create_dir(&abandoned).unwrap();
            fs::write(abandoned.join("partial"), b"unfinished").unwrap();
            let recovered = admin
                .graph(&separate, log("all()"), page(100))
                .await
                .unwrap();
            assert_eq!(recovered.data, healed.data);
            assert!(!abandoned.exists());
            // A failed native query cannot leave partially updated derived files.
            assert!(matches!(
                admin
                    .graph(&separate, log("@"), page(100))
                    .await
                    .unwrap_err(),
                q::Error::InvalidQuery(diagnostic) if diagnostic.kind == "missing_context"
            ));
            assert_eq!(fs::read_dir(cache_root(root.path())).unwrap().count(), 1);
            assert_eq!(*f.session.snapshot().await.unwrap().manifest(), before);
        })
        .await;
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn graph_cache_recovery_never_follows_symlinks_or_repairs_authoritative_objects() {
    use std::os::unix::fs::symlink;
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let local = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let root = tempfile::tempdir().unwrap();
            let outside = tempfile::tempdir().unwrap();
            let (repo, _) = initial(&f, local.path(), work.path()).await;
            let read = pin(&f, oid(repo.op_id())).await;
            let exec = executor(root.path(), q::Limits::default());
            let first = read.graph(&exec, log("all()"), page(100)).await.unwrap();
            let slot = cache_slot(root.path(), oid(repo.op_id()));
            let segments = slot.join("index/segments");
            fs::remove_dir_all(&segments).unwrap();
            fs::write(outside.path().join("keep"), b"not query cache").unwrap();
            symlink(outside.path(), &segments).unwrap();
            let recovered = read.graph(&exec, log("all()"), page(100)).await.unwrap();
            assert_eq!(recovered.data, first.data);
            assert_eq!(recovered.cache.state, q::cache::State::Rebuilt);
            assert_eq!(
                fs::read(outside.path().join("keep")).unwrap(),
                b"not query cache"
            );
            // Refuse namespace links before creating children outside the private root.
            let linked = outside.path().join("linked");
            symlink(root.path(), &linked).unwrap();
            assert_eq!(
                read.graph(
                    &executor(&linked, q::Limits::default()),
                    log("all()"),
                    page(100)
                )
                .await
                .unwrap_err(),
                q::Error::Unavailable
            );
            let slot = cache_slot(root.path(), oid(repo.op_id()));
            fs::write(slot.join("query-cache.json"), b"force rebuild").unwrap();
            use jj_store_object::{Condition, Key, ObjectStore as _, StagedUpload, WriteOutcome};
            let key =
                Key::new(m::ObjectKey::Operation(oid(repo.op_id())).storage_path(REPO)).unwrap();
            let version = f.store.head(&key).await.unwrap().version;
            let upload = StagedUpload::from_bytes(b"broken authoritative object", 1024)
                .await
                .unwrap();
            assert!(matches!(
                f.store
                    .put(&key, Condition::Matches(version), &upload)
                    .await
                    .unwrap(),
                WriteOutcome::Applied(_)
            ));
            assert!(read.graph(&exec, log("all()"), page(100)).await.is_err());
            assert_eq!(fs::read_dir(cache_root(root.path())).unwrap().count(), 1);
        })
        .await;
}

struct BoundAccess {
    inner: Arc<CommittedRead>,
    repo: m::RepositoryId,
    epoch: m::EpochId,
}
#[async_trait::async_trait]
impl q::ReadAccess for BoundAccess {
    fn repository(&self) -> m::RepositoryId {
        self.repo
    }
    fn epoch(&self) -> m::EpochId {
        self.epoch
    }
    fn operation(&self) -> m::OperationId {
        roots::ROOT_OPERATION
    }
    async fn contains(&self, key: m::ObjectKey) -> q::Result<bool> {
        q::ReadAccess::contains(self.inner.as_ref(), key).await
    }
    async fn inventory(&self) -> q::Result<Arc<BTreeSet<m::ObjectKey>>> {
        q::ReadAccess::inventory(self.inner.as_ref()).await
    }
    async fn authorize(&self) -> q::Result<()> {
        q::ReadAccess::authorize(self.inner.as_ref()).await
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn graph_cache_bindings_separate_repository_and_epoch_for_identical_root_ids() {
    // Trusted read capabilities with different bindings but the same virtual
    // native roots; this directly exercises query cache key isolation.
    let f = Fixture::new().await;
    let root = tempfile::tempdir().unwrap();
    let read = pin(&f, roots::ROOT_OPERATION).await;
    let exec = executor(root.path(), q::Limits::default());
    let bindings = [
        (REPO, m::EpochId::new([4; 16])),
        (REPO, m::EpochId::new([5; 16])),
        (m::RepositoryId::new([6; 16]), m::EpochId::new([4; 16])),
    ];
    for round in 0..2 {
        for (repo, epoch) in bindings {
            let access = Arc::new(BoundAccess {
                inner: read.clone(),
                repo,
                epoch,
            });
            let result = exec
                .execute(f.store.clone(), access, log("all()"), page(100))
                .await
                .unwrap();
            assert_eq!(
                result.cache.state,
                if round == 0 {
                    q::cache::State::Built
                } else {
                    q::cache::State::Reused
                }
            );
            assert_eq!(commits(&result).len(), 1);
            assert_eq!(commits(&result)[0].commit.id, roots::ROOT_COMMIT);
            if round == 1 {
                assert_eq!(result.cache.retained_entries, 3);
            }
        }
    }
}
