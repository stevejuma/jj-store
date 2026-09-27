#[path = "delivery/support.rs"]
mod support;
use jj_store_adapters::*;
use jj_store_model as m;
use jj_store_object::{Condition, Key, ObjectStore, StagedUpload};
use std::{
    sync::{Arc, atomic::Ordering},
    time::Duration,
};
use support::*;
fn limits() -> Limits {
    Limits {
        page: 1,
        ..Default::default()
    }
}
fn token() -> CancellationToken {
    CancellationToken::new()
}

#[tokio::test]
async fn adapter_replays_only_selected_records_and_bootstraps_after_artifact_loss() {
    let f = Fixture::new().await;
    f.publish(1).await;
    f.publish(2).await;
    f.arm("manifest.json", Fault::Before);
    assert!(matches!(
        f.publish(3).await,
        jj_store_wal::publication::PublishOutcome::OutcomeUnknown { .. }
    ));
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("artifact.json");
    let projection = FileProjection::new(path.clone());
    let control = projection.control.clone();
    let mut runner =
        Runner::new(f.source(), identity(), projection, root.path(), limits()).unwrap();
    let before = f.coordinator.read().await.unwrap().manifest().clone();
    let health = runner.step(token()).await.unwrap();
    assert_eq!(health.applied.as_ref().unwrap().source.position.0, 2);
    assert_eq!(f.remote().await, health.applied);
    assert_eq!(*f.coordinator.read().await.unwrap().manifest(), before);
    assert_eq!(control.lock().unwrap().bootstraps, 1);
    f.publish(3).await;
    f.publish(4).await;
    f.coordinator.checkpoint().await.unwrap();
    let first = runner.step(token()).await.unwrap();
    assert_eq!(first.lag, Some(1));
    let second = runner.step(token()).await.unwrap();
    assert_eq!(second.lag, Some(0));
    assert_eq!(control.lock().unwrap().attempts, vec![vec![3], vec![4]]);
    drop(runner);
    let projection = FileProjection::new(path.clone());
    let control = projection.control.clone();
    let mut restarted =
        Runner::new(f.source(), identity(), projection, root.path(), limits()).unwrap();
    assert_eq!(
        restarted.step(token()).await.unwrap().applied,
        second.applied
    );
    assert_eq!(control.lock().unwrap().bootstraps, 0);
    std::fs::remove_file(&path).unwrap();
    let rebuilt = restarted.step(token()).await.unwrap().applied.unwrap();
    assert_eq!(rebuilt.source, second.applied.as_ref().unwrap().source);
    assert_ne!(rebuilt.artifact, second.applied.unwrap().artifact);
    assert_eq!(f.remote().await, Some(rebuilt));
    assert_eq!(control.lock().unwrap().bootstraps, 1);
}

#[tokio::test]
async fn adapter_effect_crashes_and_poison_never_advance_or_skip_source() {
    for fault in [EffectFault::Before, EffectFault::After, EffectFault::Lie] {
        let f = Fixture::new().await;
        let root = tempfile::tempdir().unwrap();
        let projection = FileProjection::new(root.path().join("artifact.json"));
        let control = projection.control.clone();
        let mut runner =
            Runner::new(f.source(), identity(), projection, root.path(), limits()).unwrap();
        runner.step(token()).await.unwrap();
        f.publish(1).await;
        control.lock().unwrap().fault = Some(fault);
        assert!(runner.step(token()).await.is_err());
        assert_eq!(f.remote().await.unwrap().source.position.0, 0);
        assert!(runner.health().degraded);
        runner.step(token()).await.unwrap();
        assert_eq!(f.remote().await.unwrap().source.position.0, 1);
        let attempts = control.lock().unwrap().attempts.clone();
        assert_eq!(
            attempts,
            if matches!(fault, EffectFault::After) {
                vec![vec![1]]
            } else {
                vec![vec![1], vec![1]]
            }
        );
        f.publish(2).await;
        control.lock().unwrap().poison = Some(2);
        assert_eq!(runner.step(token()).await.unwrap_err(), Error::Projection);
        f.publish(3).await;
        assert_eq!(runner.step(token()).await.unwrap_err(), Error::Projection);
        assert_eq!(f.remote().await.unwrap().source.position.0, 1);
        assert_eq!(
            f.coordinator.read().await.unwrap().manifest().published.0,
            3
        );
        control.lock().unwrap().poison = None;
        assert_eq!(runner.step(token()).await.unwrap().lag, Some(1));
        assert_eq!(runner.step(token()).await.unwrap().lag, Some(0));
    }
}

#[tokio::test]
async fn adapter_cursor_crashes_reconcile_durable_artifact_after_restart() {
    for fault in [Fault::Before, Fault::After, Fault::Conflict] {
        let f = Fixture::new().await;
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("artifact.json");
        let projection = FileProjection::new(path.clone());
        let control = projection.control.clone();
        let mut runner =
            Runner::new(f.source(), identity(), projection, root.path(), limits()).unwrap();
        runner.step(token()).await.unwrap();
        f.publish(1).await;
        f.arm("/cursor.json", fault);
        let result = runner.step(token()).await;
        match fault {
            Fault::After => assert!(result.is_ok()),
            Fault::Before => assert_eq!(result.unwrap_err(), Error::Unavailable),
            Fault::Conflict => assert_eq!(result.unwrap_err(), Error::CursorConflict),
        }
        assert_eq!(control.lock().unwrap().attempts, vec![vec![1]]);
        drop(runner);
        let projection = FileProjection::new(path);
        let control = projection.control.clone();
        let mut restarted =
            Runner::new(f.source(), identity(), projection, root.path(), limits()).unwrap();
        let health = restarted.step(token()).await.unwrap();
        assert_eq!(health.applied, health.remote);
        assert_eq!(health.lag, Some(0));
        assert!(control.lock().unwrap().attempts.is_empty());
    }
}

#[tokio::test]
async fn adapter_restored_older_artifact_catches_up_without_remote_regression_and_config_rebuilds()
{
    let f = Fixture::new().await;
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("artifact.json");
    let projection = FileProjection::new(path.clone());
    let mut runner =
        Runner::new(f.source(), identity(), projection, root.path(), limits()).unwrap();
    runner.step(token()).await.unwrap();
    f.publish(1).await;
    runner.step(token()).await.unwrap();
    let restored = std::fs::read(&path).unwrap();
    for n in [2, 3] {
        f.publish(n).await;
        runner.step(token()).await.unwrap();
    }
    let remote = f.remote().await.unwrap();
    drop(runner);
    std::fs::write(&path, restored).unwrap();
    let projection = FileProjection::new(path.clone());
    let mut runner =
        Runner::new(f.source(), identity(), projection, root.path(), limits()).unwrap();
    assert_eq!(
        runner
            .step(token())
            .await
            .unwrap()
            .applied
            .unwrap()
            .source
            .position
            .0,
        2
    );
    assert_eq!(f.remote().await, Some(remote));
    assert_eq!(
        runner
            .step(token())
            .await
            .unwrap()
            .applied
            .unwrap()
            .source
            .position
            .0,
        3
    );
    drop(runner);
    let mut changed = identity();
    changed.schema += 1;
    changed.config = m::RecordDigest::new([9; 32]);
    let projection = FileProjection::new(path);
    let control = projection.control.clone();
    let mut runner = Runner::new(
        f.source(),
        changed.clone(),
        projection,
        root.path(),
        limits(),
    )
    .unwrap();
    assert_eq!(
        runner
            .step(token())
            .await
            .unwrap()
            .applied
            .unwrap()
            .identity,
        changed
    );
    assert_eq!(control.lock().unwrap().bootstraps, 1);
}

#[tokio::test]
async fn adapter_denial_limits_and_corrupt_selected_data_fail_without_advancing() {
    let f = Fixture::new().await;
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("artifact.json");
    let projection = FileProjection::new(path.clone());
    let control = projection.control.clone();
    let mut runner =
        Runner::new(f.source(), identity(), projection, root.path(), limits()).unwrap();
    f.access.allowed.store(false, Ordering::SeqCst);
    assert_eq!(runner.step(token()).await.unwrap_err(), Error::Denied);
    assert!(f.remote().await.is_none());
    f.access.allowed.store(true, Ordering::SeqCst);
    runner.step(token()).await.unwrap();
    f.publish(1).await;
    control.lock().unwrap().deny_after_effect = Some(f.access.clone());
    assert_eq!(runner.step(token()).await.unwrap_err(), Error::Denied);
    assert_eq!(f.remote().await.unwrap().source.position.0, 0);
    f.access.allowed.store(true, Ordering::SeqCst);
    runner.step(token()).await.unwrap();
    drop(runner);
    let mut low = limits();
    low.requests = 1;
    let mut runner = Runner::new(
        f.source(),
        identity(),
        FileProjection::new(path.clone()),
        root.path(),
        low,
    )
    .unwrap();
    assert_eq!(runner.step(token()).await.unwrap_err(), Error::Limit);
    drop(runner);
    // A source cursor with a plausible numeric position but a different digest is invalid.
    let key = Key::new(format!("repos/{REPO}/adapters/v1/test/cursor.json")).unwrap();
    let mut corrupt = f.remote().await.unwrap();
    corrupt.source.segment.as_mut().unwrap().digest = m::RecordDigest::new([99; 32]);
    f.store
        .put(
            &key,
            Condition::Matches(f.store.head(&key).await.unwrap().version),
            &StagedUpload::from_bytes(&serde_json::to_vec(&corrupt).unwrap(), 16384)
                .await
                .unwrap(),
        )
        .await
        .unwrap();
    let mut runner = Runner::new(
        f.source(),
        identity(),
        FileProjection::new(path),
        root.path(),
        limits(),
    )
    .unwrap();
    assert_eq!(runner.step(token()).await.unwrap_err(), Error::Corrupt);
    assert_eq!(
        f.coordinator.read().await.unwrap().manifest().published.0,
        1
    );
}

struct HoldingProjection {
    release: Arc<tokio::sync::Notify>,
    done: Option<tokio::sync::oneshot::Sender<()>>,
}
#[async_trait::async_trait]
impl Projection for HoldingProjection {
    async fn recover(&mut self, _: Context) -> Result<Option<Checkpoint>> {
        Ok(None)
    }
    async fn bootstrap(
        &mut self,
        _: Snapshot,
        _: &Identity,
        context: Context,
    ) -> Result<Checkpoint> {
        let release = self.release.clone();
        let done = self.done.take().unwrap();
        tokio::spawn(async move {
            release.notified().await;
            drop(context);
            let _ = done.send(());
        });
        std::future::pending().await
    }
    async fn apply(&mut self, _: Batch, _: &Identity, _: Context) -> Result<Checkpoint> {
        unreachable!()
    }
}
#[tokio::test]
async fn adapter_timeout_keeps_writer_and_admission_until_background_work_exits() {
    let f = Fixture::new().await;
    let root = tempfile::tempdir().unwrap();
    let release = Arc::new(tokio::sync::Notify::new());
    let (done, wait) = tokio::sync::oneshot::channel();
    let mut runner = Runner::new(
        f.source(),
        identity(),
        HoldingProjection {
            release: release.clone(),
            done: Some(done),
        },
        root.path(),
        Limits {
            timeout: Duration::from_millis(100),
            ..limits()
        },
    )
    .unwrap();
    assert_eq!(runner.step(token()).await.unwrap_err(), Error::Limit);
    assert_eq!(runner.step(token()).await.unwrap_err(), Error::Busy);
    drop(runner);
    let make = || {
        Runner::new(
            f.source(),
            identity(),
            FileProjection::new(root.path().join("artifact.json")),
            root.path(),
            limits(),
        )
    };
    assert!(matches!(make(), Err(Error::Busy)));
    release.notify_one();
    wait.await.unwrap();
    let mut next = make().unwrap();
    let cancel = token();
    cancel.cancel();
    assert_eq!(next.step(cancel).await.unwrap_err(), Error::Canceled);
    assert_eq!(next.step(token()).await.unwrap().lag, Some(0));
}

#[tokio::test]
async fn adapter_polling_retries_poison_with_health_and_stops_on_cancellation() {
    let f = Fixture::new().await;
    let root = tempfile::tempdir().unwrap();
    let projection = FileProjection::new(root.path().join("artifact.json"));
    let control = projection.control.clone();
    let mut runner = Runner::new(
        f.source(),
        identity(),
        projection,
        root.path(),
        Limits {
            poll: Duration::from_millis(10),
            max_backoff: Duration::from_millis(40),
            ..limits()
        },
    )
    .unwrap();
    runner.step(token()).await.unwrap();
    f.publish(1).await;
    control.lock().unwrap().poison = Some(1);
    let mut health = runner.subscribe();
    let cancel = token();
    let stop = cancel.clone();
    let task = tokio::spawn(async move {
        runner.run(stop).await.unwrap();
        runner
    });
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            if health.borrow().last_error == Some(Error::Projection) {
                break;
            }
            health.changed().await.unwrap();
        }
        assert_eq!(health.borrow().lag, Some(1));
        assert_eq!(f.remote().await.unwrap().source.position.0, 0);
        control.lock().unwrap().poison = None;
        loop {
            if health
                .borrow()
                .applied
                .as_ref()
                .is_some_and(|c| c.source.position.0 == 1)
                && !health.borrow().degraded
            {
                break;
            }
            health.changed().await.unwrap();
        }
    })
    .await
    .unwrap();
    cancel.cancel();
    let runner = task.await.unwrap();
    assert_eq!(runner.health().lag, Some(0));
    assert!(control.lock().unwrap().attempts.len() >= 2);
}

#[tokio::test]
async fn adapter_corrupt_native_payload_and_noncanonical_or_foreign_cursor_are_rejected() {
    let f = Fixture::new().await;
    f.publish(1).await;
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("artifact.json");
    let mut runner = Runner::new(
        f.source(),
        identity(),
        FileProjection::new(path),
        root.path(),
        limits(),
    )
    .unwrap();
    runner.step(token()).await.unwrap();
    let saved = f.remote().await.unwrap();
    let cursor = Key::new(format!("repos/{REPO}/adapters/v1/test/cursor.json")).unwrap();
    let mut foreign = saved.clone();
    foreign.source.repository = m::RepositoryId::new([99; 16]);
    let mut noncanonical = serde_json::to_vec(&saved).unwrap();
    noncanonical.push(b'\n');
    for bytes in [serde_json::to_vec(&foreign).unwrap(), noncanonical] {
        f.store
            .put(
                &cursor,
                Condition::Matches(f.store.head(&cursor).await.unwrap().version),
                &StagedUpload::from_bytes(&bytes, 16384).await.unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(runner.step(token()).await.unwrap_err(), Error::Corrupt);
    }
    drop(runner);
    let fresh = tempfile::tempdir().unwrap();
    let mut other = identity();
    other.name = "corruption".into();
    let key = Key::new(m::ObjectKey::Operation(op(1)).storage_path(REPO)).unwrap();
    let mut bytes = f
        .store
        .get(&key, None)
        .await
        .unwrap()
        .read_bounded(2048)
        .await
        .unwrap();
    *bytes.last_mut().unwrap() ^= 1;
    f.store
        .put(
            &key,
            Condition::Matches(f.store.head(&key).await.unwrap().version),
            &StagedUpload::from_bytes(&bytes, 2048).await.unwrap(),
        )
        .await
        .unwrap();
    let mut runner = Runner::new(
        f.source(),
        other,
        FileProjection::new(fresh.path().join("artifact.json")),
        fresh.path(),
        limits(),
    )
    .unwrap();
    assert_eq!(runner.step(token()).await.unwrap_err(), Error::Corrupt);
    assert!(!fresh.path().join("artifact.json").exists());
    assert_eq!(
        f.coordinator.read().await.unwrap().manifest().published.0,
        1
    );
}

#[tokio::test]
async fn adapter_bootstrap_rejects_corrupt_selected_tail_before_destination_effect() {
    let f = Fixture::new().await;
    f.publish(1).await;
    let manifest = f.coordinator.read().await.unwrap();
    let tail = manifest.manifest().tail.unwrap().key(REPO).unwrap();
    f.store
        .put(
            &tail,
            Condition::Matches(f.store.head(&tail).await.unwrap().version),
            &StagedUpload::from_bytes(b"corrupt", 128).await.unwrap(),
        )
        .await
        .unwrap();
    let root = tempfile::tempdir().unwrap();
    let projection = FileProjection::new(root.path().join("artifact.json"));
    let control = projection.control.clone();
    let mut runner =
        Runner::new(f.source(), identity(), projection, root.path(), limits()).unwrap();
    assert_eq!(runner.step(token()).await.unwrap_err(), Error::Corrupt);
    assert_eq!(control.lock().unwrap().bootstraps, 0);
    assert!(f.remote().await.is_none());
}
