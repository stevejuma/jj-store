use super::*;
#[path = "tests/attachment.rs"]
mod attachment;
use crate::capture_worker::{
    CaptureWorkerPhase,
    executor::{OwnedWork, OwnedWorker},
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{os::unix::fs::PermissionsExt as _, process::Command, sync::mpsc};

struct Fixture {
    root: tempfile::TempDir,
    pid: u32,
}
impl Fixture {
    fn volume(&self) -> std::path::PathBuf {
        self.root.path().join("volume")
    }
    fn alive(&self) -> bool {
        Command::new("/bin/kill")
            .args(["-0", &self.pid.to_string()])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .unwrap()
            .success()
    }
    fn locked(&self) -> bool {
        policy::exclusive_lock(&self.root.path().join("server.lock")).is_err()
    }
}
async fn fixture() -> (Fixture, StorageOwner) {
    let root = tempfile::tempdir().unwrap();
    let executable = root.path().join("storage-fixture");
    std::fs::write(
        &executable,
        r#"#!/bin/sh
printf 'READY\n'
IFS= read -r command
if [ "$command" = shutdown ]; then
    printf 'requested' > "$2/stop-requested"
    while [ ! -e "$2/release-stop" ]; do /bin/sleep 0.01; done
fi
"#,
    )
    .unwrap();
    std::fs::set_permissions(&executable, std::fs::Permissions::from_mode(0o700)).unwrap();
    let volume = root.path().join("volume");
    std::fs::create_dir(&volume).unwrap();
    let address = std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap();
    let runtime = EmbeddedStorageRuntime::start(EmbeddedStorageConfig {
        runtime_executable: executable,
        volume,
        address: address.to_string(),
        access_key: "synthetic-storage-access".into(),
        secret_key: "synthetic-storage-secret".into(),
        region: "us-east-1".into(),
        bucket: "capture-fixture".into(),
        max_object_bytes: 1024,
        timeout: Duration::from_secs(2),
    })
    .await
    .unwrap();
    let pid = runtime.process_id().unwrap();
    let lock = policy::exclusive_lock(&root.path().join("server.lock")).unwrap();
    let owner = StorageOwner::new(runtime.store(), Some(runtime), Some(lock));
    (Fixture { root, pid }, owner)
}
async fn until(mut condition: impl FnMut() -> bool) {
    tokio::time::timeout(Duration::from_secs(3), async {
        while !condition() {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn storage_waits_for_last_capture_owner_and_holds_lock_through_child_shutdown() {
    let (f, storage) = fixture().await;
    let first = storage.capture_lifetime();
    let second = first.fork();
    let stopped = tokio::spawn(storage.shutdown());
    drop(first);
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert!(!stopped.is_finished());
    assert!(f.alive() && f.locked());
    assert!(!f.volume().join("stop-requested").exists());
    drop(second);
    until(|| f.volume().join("stop-requested").exists()).await;
    assert!(!stopped.is_finished());
    assert!(f.alive() && f.locked());
    std::fs::write(f.volume().join("release-stop"), b"release").unwrap();
    stopped.await.unwrap().unwrap();
    assert!(!f.alive() && !f.locked());
}

#[tokio::test(flavor = "current_thread")]
async fn dropped_storage_or_cancelled_wait_cannot_stop_a_retained_child() {
    for cancel in [false, true] {
        let (f, storage) = fixture().await;
        let retained = storage.capture_lifetime();
        if cancel {
            assert!(
                tokio::time::timeout(Duration::from_millis(50), storage.shutdown())
                    .await
                    .is_err()
            );
        } else {
            drop(storage);
        }
        assert!(f.alive() && f.locked());
        assert!(!f.volume().join("stop-requested").exists());
        drop(retained);
        until(|| !f.alive()).await;
        assert!(!f.locked());
    }
}

struct Blocked {
    entered: Option<tokio::sync::oneshot::Sender<()>>,
    release: mpsc::Receiver<()>,
    store: std::sync::Weak<S3Store>,
    dropped: Arc<AtomicBool>,
}
impl Drop for Blocked {
    fn drop(&mut self) {
        assert!(
            self.store.upgrade().is_some(),
            "storage dropped before its capture state"
        );
        self.dropped.store(true, Ordering::SeqCst);
    }
}
#[async_trait::async_trait]
impl OwnedWork for Blocked {
    type Command = bool;
    type Output = ();
    async fn step(&mut self, panic: bool) {
        self.entered.take().unwrap().send(()).unwrap();
        self.release.recv_timeout(Duration::from_secs(10)).unwrap();
        assert!(!panic, "synthetic lost blocking capture task");
    }
}

#[tokio::test(flavor = "current_thread")]
async fn detached_blocking_capture_task_retains_storage_on_completion_and_loss() {
    for panic in [false, true] {
        let (f, storage) = fixture().await;
        let (entered, observed) = tokio::sync::oneshot::channel();
        let (release, blocked) = mpsc::channel();
        let dropped = Arc::new(AtomicBool::new(false));
        let mut owner = OwnedWorker::new(
            Blocked {
                entered: Some(entered),
                release: blocked,
                store: Arc::downgrade(&storage.store()),
                dropped: dropped.clone(),
            },
            Duration::from_secs(10),
        )
        .unwrap();
        owner.retain_storage(storage.capture_lifetime());
        owner.start(panic).unwrap();
        observed.await.unwrap();
        owner.pause();
        assert_eq!(owner.phase(), CaptureWorkerPhase::Draining);
        drop(owner);
        drop(storage);
        assert!(f.alive() && f.locked());
        release.send(()).unwrap();
        until(|| !f.alive()).await;
        assert!(dropped.load(Ordering::SeqCst));
        assert!(!f.locked());
    }
}

#[tokio::test(flavor = "current_thread")]
async fn unconsumed_completion_retains_storage_until_source_state_is_dropped() {
    let storage = external();
    let weak = Arc::downgrade(&storage.store());
    let dropped = Arc::new(AtomicBool::new(false));
    let (entered, observed) = tokio::sync::oneshot::channel();
    let (release, blocked) = mpsc::channel();
    let mut owner = OwnedWorker::new(
        Blocked {
            entered: Some(entered),
            release: blocked,
            store: weak.clone(),
            dropped: dropped.clone(),
        },
        Duration::from_secs(5),
    )
    .unwrap();
    owner.retain_storage(storage.capture_lifetime());
    owner.start(false).unwrap();
    observed.await.unwrap();
    release.send(()).unwrap();
    tokio::time::sleep(Duration::from_millis(50)).await;
    owner.stop();
    let stopped = storage.shutdown();
    tokio::pin!(stopped);
    assert!(
        tokio::time::timeout(Duration::from_millis(50), &mut stopped)
            .await
            .is_err()
    );
    assert!(!dropped.load(Ordering::SeqCst));
    assert!(weak.upgrade().is_some());
    while matches!(
        owner.poll().await,
        crate::capture_worker::executor::Completion::Running
    ) {
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
    stopped.await.unwrap();
    assert!(dropped.load(Ordering::SeqCst));
    assert!(weak.upgrade().is_none());
}

fn external() -> StorageOwner {
    let store = S3Store::new(s3::Config {
        endpoint: "http://127.0.0.1:9000/".into(),
        bucket: "capture-fixture".into(),
        region: "us-east-1".into(),
        credentials: s3::Credentials {
            access_key: "synthetic-access".into(),
            secret_key: "synthetic-secret".into(),
            session_token: None,
        },
        allow_http_loopback: true,
        max_object_bytes: 1024,
        timeout: Duration::from_secs(1),
    })
    .unwrap();
    StorageOwner::new(Arc::new(store), None, None)
}

#[tokio::test]
async fn final_release_and_shutdown_registration_cannot_lose_a_wakeup() {
    for _ in 0..128 {
        let storage = external();
        let retained = storage.capture_lifetime();
        let waiter = tokio::spawn(storage.shutdown());
        let release = tokio::spawn(async move {
            drop(retained);
        });
        release.await.unwrap();
        tokio::time::timeout(Duration::from_secs(2), waiter)
            .await
            .unwrap()
            .unwrap()
            .unwrap();
    }
}
