use super::*;

struct PausedOperation {
    inner: Arc<dyn jj_store_object::ObjectStore>,
    operation: String,
    stopped: std::sync::atomic::AtomicBool,
    entered: tokio::sync::Notify,
    release: tokio::sync::Notify,
}
#[async_trait::async_trait]
impl jj_store_object::ObjectStore for PausedOperation {
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
        if key.as_str() == self.operation
            && !self.stopped.swap(true, std::sync::atomic::Ordering::AcqRel)
        {
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
        panic!("recovery wrote native state")
    }
}
fn files(path: &std::path::Path) -> serde_json::Value {
    serde_json::from_slice::<serde_json::Value>(&fs::read(path.join("query-cache.json")).unwrap())
        .unwrap()["files"]
        .clone()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn existing_disk_projection_recovery_releases_namespace_and_preserves_generations() {
    tokio::task::LocalSet::new()
        .run_until(async {
            for (preparing, same_operation) in [(false, false), (true, false), (false, true)] {
                let f = Fixture::new().await;
                let local = tempfile::tempdir().unwrap();
                let work = tempfile::tempdir().unwrap();
                let root = tempfile::tempdir().unwrap();
                let (old, base) = initial(&f, local.path(), work.path()).await;
                let (new, _) = child(&old, &base, "Later").await;
                let older = pin(&f, oid(old.op_id())).await;
                let other_operation = if same_operation {
                    oid(old.op_id())
                } else {
                    oid(new.op_id())
                };
                let newer = pin(&f, other_operation).await;
                let before = f.session.snapshot().await.unwrap().manifest().clone();
                let original = executor(root.path(), q::Limits::default());
                let expected = older
                    .graph(&original, log("all()"), page(100))
                    .await
                    .unwrap();
                let old_slot = cache_slot(root.path(), oid(old.op_id()));
                let namespace = old_slot.parent().unwrap().to_owned();
                let old_files = files(&old_slot);
                drop(original);
                let recovery = executor(root.path(), q::Limits::default());
                let separate = executor(root.path(), q::Limits::default());
                let gate = Arc::new(PausedOperation {
                    inner: f.store.clone(),
                    operation: m::ObjectKey::Operation(oid(old.op_id())).storage_path(REPO),
                    stopped: Default::default(),
                    entered: Default::default(),
                    release: Default::default(),
                });
                let pending = {
                    let (recovery, gate, older) = (recovery.clone(), gate.clone(), older.clone());
                    tokio::spawn(async move {
                        if preparing {
                            recovery
                                .prepare_index(gate, older.clone(), q::Cancellation::default())
                                .await?;
                            older.graph(&recovery, log("all()"), page(100)).await
                        } else {
                            recovery.execute(gate, older, log("all()"), page(100)).await
                        }
                    })
                };
                let entered =
                    tokio::time::timeout(Duration::from_secs(2), gate.entered.notified()).await;
                let private = fs::read_dir(&namespace)
                    .unwrap()
                    .filter_map(|e| {
                        let directory = e.unwrap().path();
                        directory
                            .join("generation/index/op_links")
                            .join(oid(old.op_id()).to_string())
                            .is_file()
                            .then_some(directory)
                    })
                    .next();
                let charge = private.as_ref().map(|path| {
                    serde_json::from_slice::<serde_json::Value>(
                        &fs::read(path.join("reservation.json")).unwrap(),
                    )
                    .unwrap()["bytes"]
                        .as_u64()
                        .unwrap()
                });
                let concurrent = newer
                    .prepare_index(&separate, q::Cancellation::default())
                    .await;
                gate.release.notify_one();
                let recovered = pending.await.unwrap();
                entered.unwrap();
                assert!(
                    private.is_some(),
                    "recovery did not privately own the selected files"
                );
                assert!(
                    charge.unwrap() < q::cache::Policy::default().max_entry_bytes,
                    "recovery charged an unnecessary full construction allowance"
                );
                let concurrent =
                    concurrent.expect("existing disk projection recovery held the namespace lock");
                assert_eq!(concurrent.operation, other_operation);
                let recovered = recovered.unwrap();
                assert_eq!(recovered.operation, oid(old.op_id()));
                assert_eq!(recovered.data, expected.data);
                assert_eq!(recovered.cache.state, q::cache::State::Reused);
                assert_eq!(
                    recovered.cache.retained_entries,
                    if same_operation { 1 } else { 2 }
                );
                assert_eq!(files(&cache_slot(root.path(), oid(old.op_id()))), old_files);
                assert_eq!(
                    newer
                        .graph(&separate, log("all()"), page(100))
                        .await
                        .unwrap()
                        .operation,
                    other_operation
                );
                assert_eq!(*f.session.snapshot().await.unwrap().manifest(), before);
                let namespace = cache_slot(root.path(), oid(old.op_id()))
                    .parent()
                    .unwrap()
                    .to_owned();
                assert!(fs::read_dir(namespace).unwrap().all(|e| {
                    !e.unwrap()
                        .file_name()
                        .to_string_lossy()
                        .starts_with(".reserved-")
                }));
            }
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn canceled_disk_projection_recovery_cleans_up_while_namespace_is_busy() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let local = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let root = tempfile::tempdir().unwrap();
            let (repo, _) = initial(&f, local.path(), work.path()).await;
            let read = pin(&f, oid(repo.op_id())).await;
            let original = executor(root.path(), q::Limits::default());
            let expected = read
                .graph(&original, log("all()"), page(100))
                .await
                .unwrap();
            let namespace = cache_slot(root.path(), oid(repo.op_id()))
                .parent()
                .unwrap()
                .to_owned();
            drop(original);
            let workers = q::workers::Workers::new(2).unwrap();
            let exec = q::GraphExecutor::with_workers(
                root.path().to_owned(),
                q::Limits::default(),
                q::cache::Policy::default(),
                workers.clone(),
            )
            .unwrap();
            let cancellation = q::Cancellation::default();
            let gate = Arc::new(PausedOperation {
                inner: f.store.clone(),
                operation: m::ObjectKey::Operation(oid(repo.op_id())).storage_path(REPO),
                stopped: Default::default(),
                entered: Default::default(),
                release: Default::default(),
            });
            let mut pending = {
                let (exec, gate, read, cancellation) = (
                    exec.clone(),
                    gate.clone(),
                    read.clone(),
                    cancellation.clone(),
                );
                tokio::spawn(async move { exec.prepare_index(gate, read, cancellation).await })
            };
            let entered =
                tokio::time::timeout(Duration::from_secs(2), gate.entered.notified()).await;
            let lock = fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(namespace.join("lock"))
                .unwrap();
            let acquired = lock.try_lock();
            gate.release.notify_one();
            tokio::time::sleep(Duration::from_millis(30)).await;
            let unfinished = !pending.is_finished();
            cancellation.cancel();
            let canceled = tokio::time::timeout(Duration::from_secs(2), &mut pending).await;
            let prompt = canceled.is_ok();
            drop(lock);
            let result = match canceled {
                Ok(result) => result.unwrap(),
                Err(_) => pending.await.unwrap(),
            };
            entered.unwrap();
            acquired.expect("disk recovery retained the namespace");
            assert!(unfinished, "recovery finished despite locked installation");
            assert!(prompt, "recovery cancellation waited for the namespace");
            assert_eq!(result.unwrap_err(), q::Error::Canceled);
            assert_eq!(workers.status().in_use, 0);
            assert!(fs::read_dir(&namespace).unwrap().all(|e| {
                !e.unwrap()
                    .file_name()
                    .to_string_lossy()
                    .starts_with(".reserved-")
            }));
            assert_eq!(
                read.graph(&exec, log("all()"), page(100))
                    .await
                    .unwrap()
                    .data,
                expected.data
            );
        })
        .await;
}
