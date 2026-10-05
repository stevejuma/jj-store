mod support;
use jj_store_adapters::{CancellationToken, Error, Runner};
use jj_store_tantivy::*;
use std::{
    collections::BTreeSet,
    sync::{Arc, atomic::Ordering},
};
use support::*;

#[tokio::test]
async fn tantivy_recovers_committed_payload_without_repeating_effects() {
    let f = Fixture::new().await;
    let root = tempfile::tempdir().unwrap();
    let calls: Arc<std::sync::Mutex<Vec<u64>>> = Arc::default();
    let (mut runner, search, faults) = f.runner(root.path(), calls, Limits::default());
    runner.step(token()).await.unwrap();
    let pin = search.pin(f.access.as_ref()).await.unwrap();
    f.publish(1).await;
    faults.once(Fault::BeforeReader);
    assert_eq!(runner.step(token()).await.unwrap_err(), Error::Projection);
    let status = search.status(f.access.as_ref()).await.unwrap();
    assert!(status.degraded);
    assert_eq!(status.durable.unwrap().source.position.0, 1);
    assert_eq!(status.visible.unwrap().source.position.0, 0);
    assert_eq!(f.remote().await.unwrap().source.position.0, 0);
    assert_eq!(
        pin.query(
            f.access.as_ref(),
            SearchRequest::historical("generation", 0, 20),
            SearchLimits::default()
        )
        .await
        .unwrap()
        .hits
        .len(),
        1
    );
    drop(pin);
    drop(runner);
    drop(search);
    let calls: Arc<std::sync::Mutex<Vec<u64>>> = Arc::default();
    let (mut restarted, search, _) = f.runner(root.path(), calls.clone(), Limits::default());
    let health = restarted.step(token()).await.unwrap();
    assert_eq!(health.applied, f.remote().await);
    assert_eq!(calls.lock().unwrap().as_slice(), &[] as &[u64]);
    let page = search
        .pin(f.access.as_ref())
        .await
        .unwrap()
        .query(
            f.access.as_ref(),
            SearchRequest::historical("generation", 0, 20),
            SearchLimits::default(),
        )
        .await
        .unwrap();
    assert_eq!(page.hits.len(), 2);
    assert_eq!(page.checkpoint.source.position.0, 1);
    assert!(page.hits.iter().all(|d| d.body.is_empty()));
}

#[tokio::test]
async fn tantivy_crashes_before_pointer_replay_and_after_reader_recover() {
    for fault in [Fault::BeforeCommit, Fault::AfterCommit, Fault::AfterReader] {
        let f = Fixture::new().await;
        let root = tempfile::tempdir().unwrap();
        let calls: Arc<std::sync::Mutex<Vec<u64>>> = Arc::default();
        let (mut runner, search, faults) = f.runner(root.path(), calls.clone(), Limits::default());
        runner.step(token()).await.unwrap();
        f.publish(1).await;
        faults.once(fault);
        assert!(runner.step(token()).await.is_err());
        assert_eq!(f.remote().await.unwrap().source.position.0, 0);
        drop(runner);
        drop(search);
        let (mut runner, search, _) = f.runner(root.path(), calls.clone(), Limits::default());
        runner.step(token()).await.unwrap();
        let repeats = calls.lock().unwrap().iter().filter(|p| **p == 1).count();
        assert_eq!(repeats, if fault == Fault::AfterReader { 1 } else { 2 });
        let page = search
            .pin(f.access.as_ref())
            .await
            .unwrap()
            .query(
                f.access.as_ref(),
                SearchRequest::historical("generation", 0, 20),
                SearchLimits::default(),
            )
            .await
            .unwrap();
        assert_eq!(page.hits.len(), 2);
        assert_eq!(f.remote().await.unwrap(), page.checkpoint);
    }
}

#[tokio::test]
async fn tantivy_generation_pagination_retains_old_files_and_checks_each_authorization() {
    let f = Fixture::new().await;
    f.publish(1).await;
    let root = tempfile::tempdir().unwrap();
    let (mut runner, search, _) = f.runner(root.path(), Arc::default(), Limits::default());
    runner.step(token()).await.unwrap();
    let old_dir = active_dir(root.path());
    let pin = search.pin(f.access.as_ref()).await.unwrap();
    let page = pin
        .query(
            f.access.as_ref(),
            SearchRequest::historical("generation", 0, 1),
            SearchLimits::default(),
        )
        .await
        .unwrap();
    assert_eq!(page.next_offset, Some(1));
    f.publish(2).await;
    runner.step(token()).await.unwrap();
    assert!(old_dir.exists());
    let second = pin
        .query(
            f.access.as_ref(),
            SearchRequest::historical("generation", 1, 1),
            SearchLimits::default(),
        )
        .await
        .unwrap();
    assert_eq!(page.checkpoint, second.checkpoint);
    assert_ne!(page.hits[0].key, second.hits[0].key);
    assert_eq!(second.next_offset, None);
    f.access.allowed.store(false, Ordering::SeqCst);
    assert!(matches!(
        pin.query(
            f.access.as_ref(),
            SearchRequest::historical("generation", 1, 1),
            SearchLimits::default()
        )
        .await,
        Err(Error::Denied)
    ));
    assert!(matches!(
        search.pin(f.access.as_ref()).await,
        Err(Error::Denied)
    ));
    f.access.allowed.store(true, Ordering::SeqCst);
    f.access.foreign.store(true, Ordering::SeqCst);
    assert!(matches!(
        pin.query(
            f.access.as_ref(),
            SearchRequest::historical("generation", 0, 1),
            SearchLimits::default()
        )
        .await,
        Err(Error::Denied)
    ));
    f.access.foreign.store(false, Ordering::SeqCst);
    f.access.changed_epoch.store(true, Ordering::SeqCst);
    assert!(matches!(
        pin.query(
            f.access.as_ref(),
            SearchRequest::historical("generation", 0, 1),
            SearchLimits::default()
        )
        .await,
        Err(Error::Denied)
    ));
    drop(pin);
    assert!(!old_dir.exists());
}

#[tokio::test]
async fn tantivy_missing_corrupt_artifacts_and_config_changes_rebuild() {
    for damage in ["meta", "segment", "descriptor", "directory"] {
        let f = Fixture::new().await;
        f.publish(1).await;
        let root = tempfile::tempdir().unwrap();
        let calls: Arc<std::sync::Mutex<Vec<u64>>> = Arc::default();
        let (mut runner, search, _) = f.runner(root.path(), calls.clone(), Limits::default());
        let original = runner.step(token()).await.unwrap().applied.unwrap();
        drop(runner);
        drop(search);
        let directory = active_dir(root.path());
        match damage {
            "meta" => std::fs::write(directory.join("meta.json"), b"{}").unwrap(),
            "segment" => {
                let file = std::fs::read_dir(&directory)
                    .unwrap()
                    .map(|p| p.unwrap().path())
                    .find(|p| p.extension().is_some_and(|e| e == "store"))
                    .unwrap();
                std::fs::remove_file(file).unwrap();
            }
            "descriptor" => std::fs::write(directory.join("generation.json"), b"{}").unwrap(),
            _ => std::fs::remove_dir_all(&directory).unwrap(),
        }
        let (mut runner, search, _) = f.runner(root.path(), calls.clone(), Limits::default());
        let rebuilt = runner.step(token()).await.unwrap().applied.unwrap();
        assert_eq!(rebuilt.source, original.source);
        assert_ne!(rebuilt.artifact, original.artifact);
        assert_eq!(calls.lock().unwrap().len(), 2);
        assert_eq!(
            search
                .pin(f.access.as_ref())
                .await
                .unwrap()
                .query(
                    f.access.as_ref(),
                    SearchRequest::historical("generation", 0, 10),
                    SearchLimits::default()
                )
                .await
                .unwrap()
                .hits
                .len(),
            2
        );
        drop(runner);
        drop(search);
        let (projection, search) = TantivyProjection::open(
            root.path().join("index"),
            FixtureExtractor {
                calls: calls.clone(),
            },
            Limits::default(),
        )
        .unwrap();
        let mut changed = identity();
        changed.config = jj_store_model::RecordDigest::new([7; 32]);
        let mut runner = Runner::new(
            f.source(),
            changed.clone(),
            projection,
            root.path(),
            Default::default(),
        )
        .unwrap();
        let updated = runner.step(token()).await.unwrap().applied.unwrap();
        assert_eq!(updated.identity, changed);
        assert_eq!(calls.lock().unwrap().len(), 3);
        assert_eq!(
            search.status(f.access.as_ref()).await.unwrap().visible,
            Some(updated)
        );
    }
}

#[tokio::test]
async fn tantivy_limits_and_writer_owner_fail_without_advancing() {
    let f = Fixture::new().await;
    let root = tempfile::tempdir().unwrap();
    let limits = Limits {
        documents: 1,
        ..Default::default()
    };
    let (mut runner, search, _) = f.runner(root.path(), Arc::default(), limits);
    runner.step(token()).await.unwrap();
    assert!(matches!(
        TantivyProjection::open(
            root.path().join("index"),
            FixtureExtractor {
                calls: Arc::default()
            },
            limits
        ),
        Err(Error::Busy)
    ));
    f.publish(1).await;
    assert_eq!(runner.step(token()).await.unwrap_err(), Error::Limit);
    assert_eq!(f.remote().await.unwrap().source.position.0, 0);
    let pin = search.pin(f.access.as_ref()).await.unwrap();
    assert!(matches!(
        pin.query(
            f.access.as_ref(),
            SearchRequest::historical("generation", 0, 101),
            SearchLimits::default()
        )
        .await,
        Err(Error::Limit)
    ));
    assert!(matches!(
        pin.query(
            f.access.as_ref(),
            SearchRequest::historical("generation", 0, 1),
            SearchLimits {
                output_bytes: 1,
                ..Default::default()
            }
        )
        .await,
        Err(Error::Limit)
    ));
    drop(runner);
    drop(search);
    assert!(matches!(
        TantivyProjection::open(
            root.path().join("index"),
            FixtureExtractor {
                calls: Arc::default()
            },
            limits
        ),
        Err(Error::Busy)
    ));
    drop(pin);
    assert!(
        TantivyProjection::open(
            root.path().join("index"),
            FixtureExtractor {
                calls: Arc::default()
            },
            limits
        )
        .is_ok()
    );
}

#[tokio::test]
async fn tantivy_scope_changes_remove_stale_content_and_replay_upserts() {
    let f = Fixture::new().await;
    let root = tempfile::tempdir().unwrap();
    let (projection, search) =
        TantivyProjection::open(root.path().join("index"), Scoped, Limits::default()).unwrap();
    let mut runner = Runner::new(
        f.source(),
        identity(),
        projection,
        root.path(),
        Default::default(),
    )
    .unwrap();
    runner.step(token()).await.unwrap();
    let access = f.access.as_ref();
    let query = |pin: PinnedReader| async move {
        pin.query(
            access,
            SearchRequest::historical("secret", 0, 20),
            SearchLimits::default(),
        )
        .await
        .unwrap()
    };
    assert_eq!(
        query(search.pin(f.access.as_ref()).await.unwrap())
            .await
            .hits
            .len(),
        1
    );
    f.publish(1).await;
    runner.step(token()).await.unwrap();
    let page = query(search.pin(f.access.as_ref()).await.unwrap()).await;
    assert!(page.hits.is_empty());
    assert_eq!(page.scopes["bookmark:main"].state, ScopeState::Conflicted);
    f.publish(2).await;
    runner.step(token()).await.unwrap();
    let page = query(search.pin(f.access.as_ref()).await.unwrap()).await;
    assert_eq!(page.hits.len(), 1);
    assert_eq!(page.scopes["bookmark:main"].state, ScopeState::Ready);
}

struct Scoped;
impl Scoped {
    fn update(position: u64) -> Update {
        let scope = "bookmark:main".to_string();
        let conflict = position == 1;
        let snapshot = ScopeSnapshot {
            state: if conflict {
                ScopeState::Conflicted
            } else {
                ScopeState::Ready
            },
            operation: Some(op(position as u8)),
            commits: BTreeSet::from([jj_store_model::CommitId::new([1; 64])]),
        };
        let mut doc = document(0);
        doc.key = DocumentKey::File {
            scope: scope.clone(),
            path: "src/main.rs".into(),
        };
        doc.body = "secret".into();
        doc.operation = snapshot.operation;
        doc.commit = snapshot.commits.first().copied();
        doc.file = Some(jj_store_model::FileId::new([2; 64]));
        Update {
            upsert: if conflict { vec![] } else { vec![doc] },
            scopes: [(scope.clone(), snapshot)].into(),
            replace_scopes: [scope].into(),
            ..Default::default()
        }
    }
}
#[async_trait::async_trait]
impl Extractor for Scoped {
    async fn bootstrap(
        &mut self,
        snapshot: &jj_store_adapters::Snapshot,
        _: &jj_store_adapters::Context,
    ) -> jj_store_adapters::Result<Update> {
        Ok(Self::update(snapshot.position.position.0))
    }
    async fn update(
        &mut self,
        batch: &jj_store_adapters::Batch,
        _: &jj_store_adapters::Context,
    ) -> jj_store_adapters::Result<Update> {
        Ok(Self::update(batch.snapshot.position.position.0))
    }
}
fn token() -> CancellationToken {
    CancellationToken::new()
}

#[tokio::test]
async fn tantivy_remote_acknowledgement_loss_recovers_real_index() {
    for fault in [
        jj_store_object::memory::Fault::BeforeWrite,
        jj_store_object::memory::Fault::AfterWrite,
    ] {
        let f = Fixture::new().await;
        let root = tempfile::tempdir().unwrap();
        let calls = Arc::new(std::sync::Mutex::new(Vec::new()));
        let (mut runner, search, _) = f.runner(root.path(), calls.clone(), Limits::default());
        runner.step(token()).await.unwrap();
        f.publish(1).await;
        f.store.inject(fault);
        let result = runner.step(token()).await;
        assert_eq!(
            result.is_err(),
            matches!(fault, jj_store_object::memory::Fault::BeforeWrite)
        );
        let durable = search
            .status(f.access.as_ref())
            .await
            .unwrap()
            .durable
            .unwrap();
        assert_eq!(durable.source.position.0, 1);
        drop(runner);
        drop(search);
        let (mut restarted, search, _) = f.runner(root.path(), calls.clone(), Limits::default());
        assert_eq!(
            restarted.step(token()).await.unwrap().applied,
            Some(durable.clone())
        );
        assert_eq!(f.remote().await, Some(durable));
        assert_eq!(*calls.lock().unwrap(), vec![0, 1]);
        let page = search
            .pin(f.access.as_ref())
            .await
            .unwrap()
            .query(
                f.access.as_ref(),
                SearchRequest::historical(&op(1).to_string(), 0, 10),
                SearchLimits::default(),
            )
            .await
            .unwrap();
        assert_eq!(page.hits.len(), 1);
        assert_eq!(page.hits[0].key, DocumentKey::Operation { id: op(1) });
    }
}

#[tokio::test]
async fn tantivy_retained_generation_budget_recovers_after_reader_release() {
    let f = Fixture::new().await;
    let root = tempfile::tempdir().unwrap();
    let (mut runner, search, _) = f.runner(
        root.path(),
        Arc::default(),
        Limits {
            generations: 2,
            ..Default::default()
        },
    );
    runner.step(token()).await.unwrap();
    let old = search.pin(f.access.as_ref()).await.unwrap();
    f.publish(1).await;
    runner.step(token()).await.unwrap();
    f.publish(2).await;
    assert_eq!(runner.step(token()).await.unwrap_err(), Error::Limit);
    assert_eq!(f.remote().await.unwrap().source.position.0, 1);
    drop(old);
    runner.step(token()).await.unwrap();
    assert_eq!(f.remote().await.unwrap().source.position.0, 2);
}

#[tokio::test]
async fn tantivy_restored_older_generation_replays_without_cursor_regression() {
    let f = Fixture::new().await;
    let root = tempfile::tempdir().unwrap();
    let backup = tempfile::tempdir().unwrap();
    let (mut runner, search, _) = f.runner(root.path(), Arc::default(), Limits::default());
    runner.step(token()).await.unwrap();
    copy_tree(&root.path().join("index"), &backup.path().join("index"));
    f.publish(1).await;
    runner.step(token()).await.unwrap();
    f.publish(2).await;
    let latest = runner.step(token()).await.unwrap().applied.unwrap();
    drop(runner);
    drop(search);
    std::fs::remove_dir_all(root.path().join("index")).unwrap();
    copy_tree(&backup.path().join("index"), &root.path().join("index"));
    let (projection, search) = TantivyProjection::open(
        root.path().join("index"),
        FixtureExtractor {
            calls: Arc::default(),
        },
        Limits::default(),
    )
    .unwrap();
    let mut runner = Runner::new(
        f.source(),
        identity(),
        projection,
        root.path(),
        jj_store_adapters::Limits {
            page: 1,
            ..Default::default()
        },
    )
    .unwrap();
    let first = runner.step(token()).await.unwrap();
    assert_eq!(first.applied.unwrap().source.position.0, 1);
    assert_eq!(f.remote().await.unwrap(), latest);
    runner.step(token()).await.unwrap();
    let page = search
        .pin(f.access.as_ref())
        .await
        .unwrap()
        .query(
            f.access.as_ref(),
            SearchRequest::historical("generation", 0, 10),
            SearchLimits::default(),
        )
        .await
        .unwrap();
    assert_eq!(page.hits.len(), 3);
    assert_eq!(page.checkpoint.source, latest.source);
    assert_eq!(f.remote().await.unwrap(), page.checkpoint);
}
fn copy_tree(source: &std::path::Path, target: &std::path::Path) {
    std::fs::create_dir_all(target).unwrap();
    for entry in std::fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &target.join(entry.file_name()));
        } else {
            std::fs::copy(entry.path(), target.join(entry.file_name())).unwrap();
        }
    }
}

#[tokio::test]
async fn tantivy_rechecks_authorization_before_returning_query_results() {
    use jj_store_adapters::Authorizer;
    struct Revoke(std::sync::atomic::AtomicUsize);
    #[async_trait::async_trait]
    impl Authorizer for Revoke {
        fn repository(&self) -> jj_store_model::RepositoryId {
            REPO
        }
        async fn authorize(&self) -> jj_store_adapters::Result<jj_store_model::EpochId> {
            if self.0.fetch_add(1, Ordering::SeqCst) == 0 {
                Ok(EPOCH)
            } else {
                Err(Error::Denied)
            }
        }
    }
    let f = Fixture::new().await;
    let root = tempfile::tempdir().unwrap();
    let (mut runner, search, _) = f.runner(root.path(), Arc::default(), Limits::default());
    runner.step(token()).await.unwrap();
    let pin = search.pin(f.access.as_ref()).await.unwrap();
    let revoke = Revoke(Default::default());
    assert!(matches!(
        pin.query(
            &revoke,
            SearchRequest::historical("generation", 0, 10),
            SearchLimits::default()
        )
        .await,
        Err(Error::Denied)
    ));
    assert_eq!(revoke.0.load(Ordering::SeqCst), 2);
    assert!(matches!(
        pin.query(
            f.access.as_ref(),
            SearchRequest::historical(&"generation ".repeat(33), 0, 10),
            SearchLimits::default()
        )
        .await,
        Err(Error::Limit)
    ));
    assert!(matches!(
        pin.query(
            f.access.as_ref(),
            SearchRequest::historical("*", 0, 10),
            SearchLimits::default()
        )
        .await,
        Err(Error::Invalid)
    ));
}
