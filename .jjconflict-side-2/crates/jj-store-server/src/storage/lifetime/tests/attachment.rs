use super::*;

#[tokio::test(flavor = "current_thread")]
async fn late_lock_attachment_preserves_capture_lifetime_and_releases_rejected_lock() {
    let mut storage = external();
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("late.lock");
    let lifetime = storage.capture_lifetime();
    let weak = Arc::downgrade(&storage.store());
    let rejected = policy::exclusive_lock(&path).unwrap();
    let error = storage.adopt_lock(Some(rejected)).unwrap_err();
    assert!(error.to_string().contains("before capture lifetimes"));
    drop(policy::exclusive_lock(&path).unwrap());
    assert!(weak.upgrade().is_some());
    let stopped = storage.shutdown();
    tokio::pin!(stopped);
    assert!(
        tokio::time::timeout(Duration::from_millis(50), &mut stopped)
            .await
            .is_err()
    );
    drop(lifetime);
    stopped.await.unwrap();
    assert!(weak.upgrade().is_none());
}

#[tokio::test(flavor = "current_thread")]
async fn duplicate_lock_attachment_preserves_original_and_releases_rejected_lock() {
    let (f, mut storage) = fixture().await;
    let path = f.root.path().join("other.lock");
    let rejected = policy::exclusive_lock(&path).unwrap();
    let error = storage.adopt_lock(Some(rejected)).unwrap_err();
    assert!(error.to_string().contains("already owns a state lock"));
    drop(policy::exclusive_lock(&path).unwrap());
    assert!(f.alive() && f.locked());
    std::fs::write(f.volume().join("release-stop"), b"release").unwrap();
    storage.shutdown().await.unwrap();
    assert!(!f.alive() && !f.locked());
}
