#[allow(dead_code)]
mod support;
use jj_store_adapters::{
    ArtifactBlob, ArtifactBundle, CancellationToken, Identity, Limits as AdapterLimits, Runner,
};
use jj_store_model as m;
use jj_store_object::{Condition, Key, ObjectStore, StagedUpload, WriteOutcome};
use jj_store_tantivy::{Limits, Search, SearchLimits, SearchRequest, TantivyProjection};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::{
    path::Path,
    sync::{Arc, Mutex},
};
use support::*;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Pointer {
    version: u32,
    checkpoint: jj_store_adapters::Checkpoint,
    descriptor: ArtifactBlob,
}

fn runner(
    fixture: &Fixture,
    root: &Path,
    calls: Arc<Mutex<Vec<u64>>>,
) -> (Runner<TantivyProjection<FixtureExtractor>>, Search) {
    let (projection, search) = TantivyProjection::open(
        root.join("index"),
        FixtureExtractor { calls },
        Limits::default(),
    )
    .unwrap();
    let runner = Runner::new(
        fixture.source(),
        identity(),
        projection,
        root,
        AdapterLimits {
            snapshot_every: 1,
            ..Default::default()
        },
    )
    .unwrap();
    (runner, search)
}

async fn hits(search: &Search, fixture: &Fixture) -> usize {
    search
        .pin(fixture.access.as_ref())
        .await
        .unwrap()
        .query(
            fixture.access.as_ref(),
            SearchRequest::historical("generation", 0, 20),
            SearchLimits::default(),
        )
        .await
        .unwrap()
        .hits
        .len()
}

fn artifact_prefix(identity: &Identity) -> String {
    let digest = m::RecordDigest::new(Sha256::digest(serde_json::to_vec(identity).unwrap()).into());
    format!(
        "repos/{REPO}/adapters/v1/{}/snapshots/v1/{EPOCH}/{digest}",
        identity.name
    )
}

async fn bytes(store: &dyn ObjectStore, key: &Key, limit: u64) -> Vec<u8> {
    store
        .get(key, None)
        .await
        .unwrap()
        .read_bounded(limit)
        .await
        .unwrap()
}

async fn put_blob(store: &dyn ObjectStore, prefix: &str, value: &[u8]) -> ArtifactBlob {
    let blob = ArtifactBlob {
        bytes: value.len() as u64,
        digest: m::RecordDigest::new(Sha256::digest(value).into()),
    };
    let key = Key::new(format!("{prefix}/blobs/{}", blob.digest)).unwrap();
    let upload = StagedUpload::from_bytes(value, value.len() as u64)
        .await
        .unwrap();
    assert!(matches!(
        store.put(&key, Condition::Absent, &upload).await.unwrap(),
        WriteOutcome::Applied(_) | WriteOutcome::PreconditionFailed
    ));
    blob
}

async fn replace_with_incompatible_generation(fixture: &Fixture) {
    let identity = identity();
    let prefix = artifact_prefix(&identity);
    let pointer_key = Key::new(format!("{prefix}/snapshot.json")).unwrap();
    let download = fixture.store.get(&pointer_key, None).await.unwrap();
    let version = download.metadata.version.clone();
    let pointer_bytes = download.read_bounded(16 * 1024).await.unwrap();
    let mut pointer: Pointer = serde_json::from_slice(&pointer_bytes).unwrap();
    let descriptor_key = Key::new(format!("{prefix}/blobs/{}", pointer.descriptor.digest)).unwrap();
    let mut bundle: ArtifactBundle = serde_json::from_slice(
        &bytes(fixture.store.as_ref(), &descriptor_key, 4 * 1024 * 1024).await,
    )
    .unwrap();
    let generation = bundle.files["generation.json"].clone();
    let generation_key = Key::new(format!("{prefix}/blobs/{}", generation.digest)).unwrap();
    let mut generation: serde_json::Value = serde_json::from_slice(
        &bytes(fixture.store.as_ref(), &generation_key, 4 * 1024 * 1024).await,
    )
    .unwrap();
    generation["payload"]["identity"]["schema"] = 999.into();
    let generation = serde_json::to_vec(&generation).unwrap();
    bundle.files.insert(
        "generation.json".into(),
        put_blob(fixture.store.as_ref(), &prefix, &generation).await,
    );
    let descriptor = serde_json::to_vec(&bundle).unwrap();
    pointer.descriptor = put_blob(fixture.store.as_ref(), &prefix, &descriptor).await;
    let pointer = serde_json::to_vec(&pointer).unwrap();
    let upload = StagedUpload::from_bytes(&pointer, 16 * 1024).await.unwrap();
    assert!(matches!(
        fixture
            .store
            .put(&pointer_key, Condition::Matches(version), &upload)
            .await
            .unwrap(),
        WriteOutcome::Applied(_)
    ));
}

#[tokio::test]
async fn tantivy_snapshot_restores_an_older_generation_then_replays_forward() {
    let fixture = Fixture::new().await;
    let first = tempfile::tempdir().unwrap();
    let calls = Arc::new(Mutex::new(Vec::new()));
    let (mut initial, search) = runner(&fixture, first.path(), calls.clone());
    initial.step(CancellationToken::new()).await.unwrap();
    fixture.publish(1).await;
    initial.step(CancellationToken::new()).await.unwrap();
    assert_eq!(*calls.lock().unwrap(), vec![0, 1]);
    assert_eq!(hits(&search, &fixture).await, 2);
    drop(initial);
    drop(search);

    fixture.publish(2).await;
    let restored = tempfile::tempdir().unwrap();
    let calls = Arc::new(Mutex::new(Vec::new()));
    let (mut runner, search) = runner(&fixture, restored.path(), calls.clone());
    let health = runner.step(CancellationToken::new()).await.unwrap();
    assert_eq!(health.applied.unwrap().source.position.0, 2);
    assert_eq!(
        *calls.lock().unwrap(),
        vec![2],
        "restore must avoid bootstrap"
    );
    assert_eq!(hits(&search, &fixture).await, 3);
}

#[tokio::test]
async fn tantivy_snapshot_rebuilds_and_repairs_a_schema_incompatible_bundle() {
    let fixture = Fixture::new().await;
    let first = tempfile::tempdir().unwrap();
    let (mut initial, search) = runner(&fixture, first.path(), Arc::default());
    initial.step(CancellationToken::new()).await.unwrap();
    drop(initial);
    drop(search);
    replace_with_incompatible_generation(&fixture).await;

    let rebuilt = tempfile::tempdir().unwrap();
    let calls = Arc::new(Mutex::new(Vec::new()));
    let (mut rebuilt_runner, search) = runner(&fixture, rebuilt.path(), calls.clone());
    let health = rebuilt_runner.step(CancellationToken::new()).await.unwrap();
    assert!(!health.degraded);
    assert_eq!(*calls.lock().unwrap(), vec![0]);
    assert_eq!(hits(&search, &fixture).await, 1);
    drop(rebuilt_runner);
    drop(search);

    let restored = tempfile::tempdir().unwrap();
    let calls = Arc::new(Mutex::new(Vec::new()));
    let (mut runner, search) = runner(&fixture, restored.path(), calls.clone());
    runner.step(CancellationToken::new()).await.unwrap();
    assert!(
        calls.lock().unwrap().is_empty(),
        "repaired bundle must restore"
    );
    assert_eq!(hits(&search, &fixture).await, 1);
}
