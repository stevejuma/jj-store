#[allow(dead_code)]
#[path = "delivery/support.rs"]
mod support;
use jj_store_adapters::*;
use jj_store_model as m;
use jj_store_object::{Condition, Key, ObjectStore, StagedUpload};
use std::sync::atomic::Ordering;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};
use support::*;

struct Capture {
    inner: FileProjection,
    captured: Arc<Mutex<Option<(Artifacts, Checkpoint)>>>,
}
#[async_trait::async_trait]
impl Projection for Capture {
    async fn recover(&mut self, context: Context) -> Result<Option<Checkpoint>> {
        self.inner.recover(context).await
    }
    async fn bootstrap(
        &mut self,
        snapshot: Snapshot,
        identity: &Identity,
        context: Context,
    ) -> Result<Checkpoint> {
        let artifacts = snapshot.artifacts(identity)?;
        let checkpoint = self.inner.bootstrap(snapshot, identity, context).await?;
        *self.captured.lock().unwrap() = Some((artifacts, checkpoint.clone()));
        Ok(checkpoint)
    }
    async fn apply(&mut self, _: Batch, _: &Identity, _: Context) -> Result<Checkpoint> {
        unreachable!()
    }
}
async fn capture(f: &Fixture, identity: Identity, limits: Limits) -> (Artifacts, Checkpoint) {
    capture_from(f.source(), identity, limits, CancellationToken::new()).await
}
async fn capture_from(
    source: Source,
    identity: Identity,
    limits: Limits,
    cancel: CancellationToken,
) -> (Artifacts, Checkpoint) {
    let root = tempfile::tempdir().unwrap();
    let captured = Arc::default();
    let mut runner = Runner::new(
        source,
        identity,
        Capture {
            inner: FileProjection::new(root.path().join("artifact.json")),
            captured: Arc::clone(&captured),
        },
        root.path(),
        limits,
    )
    .unwrap();
    runner.step(cancel).await.unwrap();
    captured.lock().unwrap().take().unwrap()
}

#[tokio::test]
async fn adapter_snapshot_streams_verify_before_pointer_and_preserve_authority() {
    let f = Fixture::new().await;
    f.publish(1).await;
    let authority = f.coordinator.read().await.unwrap().manifest().clone();
    let (artifacts, checkpoint) = capture(&f, identity(), Limits::default()).await;
    assert!(artifacts.selected().await.unwrap().is_none());
    let content = vec![b'x'; 150_001];
    let blob = artifacts.upload(content.as_slice(), 200_000).await.unwrap();
    let bundle = ArtifactBundle {
        version: 1,
        checkpoint,
        files: BTreeMap::from([("segment.store".into(), blob.clone())]),
    };
    assert_eq!(artifacts.publish(&bundle).await.unwrap(), bundle);
    assert_eq!(artifacts.selected().await.unwrap().unwrap(), bundle);
    let mut downloaded = Vec::new();
    artifacts.download(&blob, &mut downloaded).await.unwrap();
    assert_eq!(downloaded, content);
    assert_eq!(*f.coordinator.read().await.unwrap().manifest(), authority);
    let writes = f.store.writes.lock().unwrap();
    assert!(writes.last().unwrap().ends_with("/snapshot.json"));
    assert!(writes.iter().rev().nth(1).unwrap().contains("/blobs/"));
}

fn bundle(checkpoint: Checkpoint, blob: ArtifactBlob) -> ArtifactBundle {
    ArtifactBundle {
        version: 1,
        checkpoint,
        files: BTreeMap::from([("segment.store".into(), blob)]),
    }
}
fn pointer_key(f: &Fixture) -> Key {
    Key::new(
        f.store
            .writes
            .lock()
            .unwrap()
            .iter()
            .rev()
            .find(|p| p.ends_with("/snapshot.json"))
            .unwrap()
            .clone(),
    )
    .unwrap()
}
async fn replace(f: &Fixture, key: &Key, bytes: &[u8]) {
    let version = f.store.head(key).await.unwrap().version;
    f.store
        .put(
            key,
            Condition::Matches(version),
            &StagedUpload::from_bytes(bytes, 4 * 1024 * 1024)
                .await
                .unwrap(),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn adapter_snapshot_missing_or_corrupt_files_never_select_a_pointer() {
    for corruption in [false, true] {
        let f = Fixture::new().await;
        let (artifacts, checkpoint) = capture(&f, identity(), Limits::default()).await;
        let mut blob = artifacts.upload(b"content".as_slice(), 1024).await.unwrap();
        if corruption {
            let key = Key::new(f.store.writes.lock().unwrap().last().unwrap().clone()).unwrap();
            replace(&f, &key, b"CORRUPT").await;
        } else {
            blob.digest = m::RecordDigest::new([42; 32]);
        }
        assert!(artifacts.publish(&bundle(checkpoint, blob)).await.is_err());
        assert!(artifacts.selected().await.unwrap().is_none());
        assert!(
            !f.store
                .writes
                .lock()
                .unwrap()
                .iter()
                .any(|p| p.ends_with("/snapshot.json"))
        );
    }
}

#[tokio::test]
async fn adapter_snapshot_unknown_upload_descriptor_and_pointer_outcomes_reconcile() {
    for stage in ["upload", "descriptor", "pointer"] {
        for fault in [Fault::Before, Fault::After, Fault::Conflict] {
            let f = Fixture::new().await;
            let (artifacts, checkpoint) = capture(&f, identity(), Limits::default()).await;
            if stage == "upload" {
                f.arm("/blobs/", fault);
            }
            let uploaded = artifacts.upload(b"content".as_slice(), 1024).await;
            if stage == "upload" {
                assert_eq!(uploaded.is_ok(), matches!(fault, Fault::After));
            }
            let blob = artifacts.upload(b"content".as_slice(), 1024).await.unwrap();
            let bundle = bundle(checkpoint, blob);
            if stage != "upload" {
                f.arm(
                    if stage == "pointer" {
                        "/snapshot.json"
                    } else {
                        "/blobs/"
                    },
                    fault,
                );
                let result = artifacts.publish(&bundle).await;
                assert_eq!(result.is_ok(), matches!(fault, Fault::After));
                assert_eq!(
                    artifacts.selected().await.unwrap().is_some(),
                    matches!(fault, Fault::After)
                );
            }
            assert_eq!(artifacts.publish(&bundle).await.unwrap(), bundle);
            assert_eq!(artifacts.selected().await.unwrap(), Some(bundle));
        }
    }
}

#[tokio::test]
async fn adapter_snapshot_rechecks_revocation_and_confines_identity_and_lineage() {
    let f = Fixture::new().await;
    let (artifacts, checkpoint) = capture(&f, identity(), Limits::default()).await;
    let blob = artifacts.upload(b"private".as_slice(), 1024).await.unwrap();
    let bundle = bundle(checkpoint, blob.clone());
    artifacts.publish(&bundle).await.unwrap();
    let writes = f.store.writes.lock().unwrap().len();
    f.access.allowed.store(false, Ordering::SeqCst);
    assert_eq!(
        artifacts.upload(b"new".as_slice(), 1024).await,
        Err(Error::Denied)
    );
    assert_eq!(
        artifacts.download(&blob, tokio::io::sink()).await,
        Err(Error::Denied)
    );
    assert_eq!(artifacts.selected().await, Err(Error::Denied));
    assert_eq!(artifacts.publish(&bundle).await, Err(Error::Denied));
    assert_eq!(f.store.writes.lock().unwrap().len(), writes);
    f.access.allowed.store(true, Ordering::SeqCst);
    for n in 0..4 {
        let mut alternate = identity();
        match n {
            0 => alternate.config = m::RecordDigest::new([4; 32]),
            1 => alternate.schema += 1,
            2 => alternate.implementation = "different".into(),
            _ => alternate.name = "other".into(),
        }
        let (other, _) = capture(&f, alternate, Limits::default()).await;
        assert!(other.selected().await.unwrap().is_none());
        assert!(other.download(&blob, tokio::io::sink()).await.is_err());
        assert_eq!(other.publish(&bundle).await, Err(Error::Invalid));
    }
    for n in 0..4 {
        let mut invalid = bundle.clone();
        match n {
            0 => invalid.checkpoint.source.repository = m::RepositoryId::new([4; 16]),
            1 => invalid.checkpoint.source.epoch = m::EpochId::new([4; 16]),
            2 => invalid.checkpoint.source.position = m::WalPosition(1),
            _ => invalid.checkpoint.version += 1,
        }
        assert!(artifacts.publish(&invalid).await.is_err());
        assert_eq!(artifacts.selected().await.unwrap(), Some(bundle.clone()));
    }
}

#[tokio::test]
async fn adapter_snapshot_rejects_unsafe_names_forged_selected_records_and_rollback() {
    let f = Fixture::new().await;
    f.publish(1).await;
    let (first, checkpoint) = capture(&f, identity(), Limits::default()).await;
    let blob = first.upload(b"content".as_slice(), 1024).await.unwrap();
    let first_bundle = bundle(checkpoint, blob);
    for name in [
        "../manifest.json",
        "/etc/file",
        "a/b",
        "a\\b",
        ".",
        "..",
        "",
    ] {
        let mut unsafe_bundle = first_bundle.clone();
        let blob = unsafe_bundle.files.pop_first().unwrap().1;
        unsafe_bundle.files.insert(name.into(), blob);
        assert_eq!(first.publish(&unsafe_bundle).await, Err(Error::Invalid));
    }
    let mut forged = first_bundle.clone();
    forged.checkpoint.source.segment.as_mut().unwrap().digest = m::RecordDigest::new([99; 32]);
    assert_eq!(first.publish(&forged).await, Err(Error::Corrupt));
    first.publish(&first_bundle).await.unwrap();
    f.publish(2).await;
    let (second, checkpoint) = capture(&f, identity(), Limits::default()).await;
    let second_bundle = ArtifactBundle {
        checkpoint,
        ..first_bundle.clone()
    };
    second.publish(&second_bundle).await.unwrap();
    assert!(first.publish(&first_bundle).await.is_err());
    assert_eq!(
        second.publish(&first_bundle).await,
        Err(Error::CursorConflict)
    );
    assert_eq!(second.selected().await.unwrap(), Some(second_bundle));
}

#[tokio::test]
async fn adapter_snapshot_rejects_corrupt_and_noncanonical_descriptors_and_pointers() {
    for damage in ["descriptor", "pointer", "foreign", "fields"] {
        let f = Fixture::new().await;
        let (artifacts, checkpoint) = capture(&f, identity(), Limits::default()).await;
        let blob = artifacts.upload(b"content".as_slice(), 1024).await.unwrap();
        artifacts.publish(&bundle(checkpoint, blob)).await.unwrap();
        let key = pointer_key(&f);
        let mut bytes = f
            .store
            .get(&key, None)
            .await
            .unwrap()
            .read_bounded(16384)
            .await
            .unwrap();
        let mut pointer: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        if damage == "descriptor" {
            let key = Key::new(format!(
                "{}/blobs/{}",
                key.as_str().strip_suffix("/snapshot.json").unwrap(),
                pointer["descriptor"]["digest"].as_str().unwrap()
            ))
            .unwrap();
            let mut bytes = f
                .store
                .get(&key, None)
                .await
                .unwrap()
                .read_bounded(4 * 1024 * 1024)
                .await
                .unwrap();
            bytes[0] ^= 1;
            replace(&f, &key, &bytes).await;
        } else {
            match damage {
                "pointer" => bytes.push(b' '),
                "foreign" => {
                    pointer["checkpoint"]["source"]["repository"] =
                        serde_json::to_value(m::RepositoryId::new([44; 16])).unwrap();
                    bytes = serde_json::to_vec(&pointer).unwrap();
                }
                _ => {
                    pointer["ignored"] = true.into();
                    bytes = serde_json::to_vec(&pointer).unwrap();
                }
            }
            replace(&f, &key, &bytes).await;
        }
        assert!(artifacts.selected().await.is_err(), "{damage}");
    }
}

#[tokio::test]
async fn adapter_snapshot_bounds_file_bytes_and_shared_transfer_budgets() {
    let f = Fixture::new().await;
    let limits = Limits {
        bytes: 48 * 1024,
        object_bytes: 16 * 1024,
        ..Default::default()
    };
    let (artifacts, checkpoint) = capture(&f, identity(), limits).await;
    assert_eq!(
        artifacts.upload(b"1234".as_slice(), 3).await,
        Err(Error::Limit)
    );
    assert_eq!(
        artifacts.upload(b"1234".as_slice(), 17 * 1024).await,
        Err(Error::Limit)
    );
    let blob = artifacts
        .upload(vec![b'a'; 16 * 1024].as_slice(), 16 * 1024)
        .await
        .unwrap();
    artifacts.download(&blob, tokio::io::sink()).await.unwrap();
    assert_eq!(
        artifacts.download(&blob, tokio::io::sink()).await,
        Err(Error::Limit)
    );
    assert!(artifacts.publish(&bundle(checkpoint, blob)).await.is_err());
    assert!(
        !f.store
            .writes
            .lock()
            .unwrap()
            .iter()
            .any(|p| p.ends_with("/snapshot.json"))
    );
}

struct RaceStore {
    inner: Arc<Store>,
    pointers: tokio::sync::Barrier,
    revoke: Option<Arc<Access>>,
    armed: std::sync::atomic::AtomicBool,
}
#[async_trait::async_trait]
impl ObjectStore for RaceStore {
    async fn head(&self, key: &Key) -> jj_store_object::Result<jj_store_object::Metadata> {
        self.inner.head(key).await
    }
    async fn get(
        &self,
        key: &Key,
        range: Option<jj_store_object::ByteRange>,
    ) -> jj_store_object::Result<jj_store_object::Download> {
        let result = self.inner.get(key, range).await;
        if key.as_str().contains("/blobs/")
            && self.armed.load(Ordering::SeqCst)
            && let Some(access) = &self.revoke
        {
            access.allowed.store(false, Ordering::SeqCst);
        }
        result
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        value: &StagedUpload,
    ) -> jj_store_object::Result<jj_store_object::WriteOutcome> {
        if self.armed.load(Ordering::SeqCst) && key.as_str().ends_with("/snapshot.json") {
            self.pointers.wait().await;
        }
        self.inner.put(key, condition, value).await
    }
}

#[tokio::test]
async fn adapter_snapshot_competing_pointer_candidates_preserve_one_complete_selection() {
    let f = Fixture::new().await;
    let store = Arc::new(RaceStore {
        inner: f.store.clone(),
        pointers: tokio::sync::Barrier::new(2),
        revoke: None,
        armed: false.into(),
    });
    let (a, checkpoint) = capture_from(
        Source::new(store.clone(), f.access.clone()),
        identity(),
        Limits::default(),
        CancellationToken::new(),
    )
    .await;
    let (b, _) = capture_from(
        Source::new(store.clone(), f.access.clone()),
        identity(),
        Limits::default(),
        CancellationToken::new(),
    )
    .await;
    let first = bundle(
        checkpoint.clone(),
        a.upload(b"first".as_slice(), 1024).await.unwrap(),
    );
    let second = bundle(
        checkpoint,
        b.upload(b"second".as_slice(), 1024).await.unwrap(),
    );
    store.armed.store(true, Ordering::SeqCst);
    let (x, y) = tokio::join!(a.publish(&first), b.publish(&second));
    assert_ne!(x.is_ok(), y.is_ok());
    let selected = a.selected().await.unwrap().unwrap();
    assert_eq!(selected, if x.is_ok() { first } else { second });
    assert_eq!(x.err().or(y.err()), Some(Error::CursorConflict));
    for blob in selected.files.values() {
        a.download(blob, tokio::io::sink()).await.unwrap();
    }
    assert_eq!(
        f.coordinator.read().await.unwrap().manifest().published.0,
        0
    );
}

#[tokio::test]
async fn adapter_snapshot_revocation_during_download_returns_no_content() {
    let f = Fixture::new().await;
    let store = Arc::new(RaceStore {
        inner: f.store.clone(),
        pointers: tokio::sync::Barrier::new(2),
        revoke: Some(f.access.clone()),
        armed: false.into(),
    });
    let (artifacts, _) = capture_from(
        Source::new(store.clone(), f.access.clone()),
        identity(),
        Limits::default(),
        CancellationToken::new(),
    )
    .await;
    let blob = artifacts.upload(b"private".as_slice(), 1024).await.unwrap();
    store.armed.store(true, Ordering::SeqCst);
    let mut output = Vec::new();
    assert_eq!(
        artifacts.download(&blob, &mut output).await,
        Err(Error::Denied)
    );
    assert!(output.is_empty());
}

struct PendingReader(Option<tokio::sync::oneshot::Sender<()>>);
impl tokio::io::AsyncRead for PendingReader {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
        _: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        if let Some(started) = self.0.take() {
            let _ = started.send(());
        }
        std::task::Poll::Pending
    }
}
#[tokio::test]
async fn adapter_snapshot_cancellation_and_deadline_bound_streaming_work() {
    for cancel_work in [false, true] {
        let f = Fixture::new().await;
        let cancel = CancellationToken::new();
        let (artifacts, _) =
            capture_from(f.source(), identity(), Limits::default(), cancel.clone()).await;
        let writes = f.store.writes.lock().unwrap().len();
        let (start, started) = tokio::sync::oneshot::channel();
        let task =
            tokio::spawn(async move { artifacts.upload(PendingReader(Some(start)), 1024).await });
        started.await.unwrap();
        if cancel_work {
            cancel.cancel();
        } else {
            tokio::time::pause();
            tokio::time::advance(std::time::Duration::from_secs(61)).await;
        }
        assert_eq!(
            task.await.unwrap(),
            Err(if cancel_work {
                Error::Canceled
            } else {
                Error::Limit
            })
        );
        if !cancel_work {
            tokio::time::resume();
        }
        assert_eq!(f.store.writes.lock().unwrap().len(), writes);
    }
}

struct EpochAccess {
    inner: Arc<Access>,
    changed: std::sync::atomic::AtomicBool,
}
#[async_trait::async_trait]
impl Authorizer for EpochAccess {
    fn repository(&self) -> m::RepositoryId {
        REPO
    }
    async fn authorize(&self) -> Result<m::EpochId> {
        self.inner.authorize().await?;
        Ok(if self.changed.load(Ordering::SeqCst) {
            m::EpochId::new([99; 16])
        } else {
            EPOCH
        })
    }
}
#[tokio::test]
async fn adapter_snapshot_epoch_changes_and_request_exhaustion_stop_access() {
    let f = Fixture::new().await;
    let access = Arc::new(EpochAccess {
        inner: f.access.clone(),
        changed: false.into(),
    });
    let (artifacts, checkpoint) = capture_from(
        Source::new(f.store.clone(), access.clone()),
        identity(),
        Limits::default(),
        CancellationToken::new(),
    )
    .await;
    let blob = artifacts.upload(b"content".as_slice(), 1024).await.unwrap();
    let bundle = bundle(checkpoint, blob.clone());
    artifacts.publish(&bundle).await.unwrap();
    access.changed.store(true, Ordering::SeqCst);
    assert_eq!(artifacts.selected().await, Err(Error::Denied));
    assert_eq!(
        artifacts.download(&blob, tokio::io::sink()).await,
        Err(Error::Denied)
    );
    assert_eq!(artifacts.publish(&bundle).await, Err(Error::Denied));
    assert_eq!(
        artifacts.upload(b"content".as_slice(), 1024).await,
        Err(Error::Denied)
    );

    let (bounded, _) = capture(
        &f,
        identity(),
        Limits {
            requests: 64,
            ..Default::default()
        },
    )
    .await;
    let mut exceeded = false;
    for _ in 0..64 {
        match bounded.selected().await {
            Ok(Some(_)) => {}
            Err(Error::Limit) => {
                exceeded = true;
                break;
            }
            other => panic!("unexpected request budget result: {other:?}"),
        }
    }
    assert!(exceeded);
}
