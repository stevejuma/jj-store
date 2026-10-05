use super::*;
use jj_store_jj::local_execution::TreeCheckoutPlanV1;
use std::{
    sync::{
        Mutex,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    time::Duration,
};
#[tokio::test]
async fn saved_original_plan_revalidates_after_reopen_without_new_preparation() {
    let f = Fixture::new().await;
    fs::write(f.root().join("file.txt"), "original\n").unwrap();
    let (recovery, witness, key) =
        retained_with_inputs(&f, vec![("file.txt", Some((b"candidate", false)))]).await;
    let (physical, digest) = guard(&f);
    let (recovery, plan) = recovery
        .prepare_tree_checkout(physical.clone(), digest)
        .await
        .unwrap();
    let bytes = plan.encode().unwrap();
    let id = plan.id().unwrap();
    let before = f.state();
    drop(recovery);
    let recovery = Coordinator::new(1)
        .unwrap()
        .restore_history(f.access(), f.root(), witness, key)
        .await
        .unwrap();
    let (_recovery, same) = recovery
        .revalidate_tree_checkout(TreeCheckoutPlanV1::decode(id, &bytes).unwrap(), physical)
        .await
        .unwrap();
    assert_eq!(same.id().unwrap(), id);
    assert_eq!(f.state(), before);
}
struct Changing {
    root: PathBuf,
    calls: AtomicUsize,
}
#[async_trait::async_trait]
impl PhysicalGuard for Changing {
    async fn validate(&self, _: &RecoveryBinding, _: m::RecordDigest) -> local_browser::Result<()> {
        if self.calls.fetch_add(1, Ordering::SeqCst) == 1 {
            fs::write(self.root.join("file.txt"), "editor during final guard\n").unwrap();
            return Err(local_browser::Error::EpochChanged);
        }
        Ok(())
    }
}
#[tokio::test]
async fn changing_physical_guard_cannot_return_a_plan_or_touch_checkout_metadata() {
    let f = Fixture::new().await;
    fs::write(f.root().join("file.txt"), "original\n").unwrap();
    let recovery = retained(&f, vec![("file.txt", Some((b"candidate", false)))]).await;
    let before = f.state();
    let physical = Arc::new(Changing {
        root: f.root(),
        calls: AtomicUsize::new(0),
    });
    assert!(matches!(
        recovery
            .prepare_tree_checkout(physical, m::RecordDigest::new([1; 32]))
            .await,
        Err(local_browser::Error::EpochChanged)
    ));
    let after = f.state();
    assert_eq!(&after[1..], &before[1..]);
    assert_eq!(after[0], b"editor during final guard\n");
}
struct Blocking {
    entered: Mutex<Option<tokio::sync::oneshot::Sender<()>>>,
    exited: AtomicBool,
}
#[async_trait::async_trait]
impl PhysicalGuard for Blocking {
    async fn validate(&self, _: &RecoveryBinding, _: m::RecordDigest) -> local_browser::Result<()> {
        if let Some(entered) = self.entered.lock().unwrap().take() {
            entered.send(()).unwrap();
            std::thread::sleep(Duration::from_millis(700));
            self.exited.store(true, Ordering::SeqCst);
        }
        Ok(())
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cancellation_retains_actual_foreground_lock_until_blocked_worker_exits() {
    let f = Fixture::new().await;
    fs::write(f.root().join("file.txt"), "original\n").unwrap();
    let recovery = retained(&f, vec![("file.txt", Some((b"candidate", false)))]).await;
    let (send, recv) = tokio::sync::oneshot::channel();
    let physical = Arc::new(Blocking {
        entered: Mutex::new(Some(send)),
        exited: AtomicBool::new(false),
    });
    let before = f.state();
    let worker = tokio::spawn(
        recovery.prepare_tree_checkout(physical.clone(), m::RecordDigest::new([1; 32])),
    );
    recv.await.unwrap();
    worker.abort();
    assert!(matches!(worker.await, Err(error) if error.is_cancelled()));
    let current = f.witness().await;
    assert!(matches!(
        Coordinator::new(1)
            .unwrap()
            .acquire(f.access(), f.root(), current.clone())
            .await,
        Err(local_browser::Error::Busy)
    ));
    assert!(!physical.exited.load(Ordering::SeqCst));
    tokio::time::sleep(Duration::from_millis(850)).await;
    assert!(physical.exited.load(Ordering::SeqCst));
    let lease = Coordinator::new(1)
        .unwrap()
        .acquire(f.access(), f.root(), current)
        .await
        .unwrap();
    drop(lease);
    assert_eq!(f.state(), before);
}
struct Denied;
#[async_trait::async_trait]
impl PhysicalGuard for Denied {
    async fn validate(&self, _: &RecoveryBinding, _: m::RecordDigest) -> local_browser::Result<()> {
        Err(local_browser::Error::Denied)
    }
}
#[tokio::test]
async fn denied_complete_guard_leaves_source_and_physical_state_intact() {
    let f = Fixture::new().await;
    let recovery = retained(&f, vec![("new", Some((b"added", false)))]).await;
    let before = f.state();
    assert!(matches!(
        recovery
            .prepare_tree_checkout(Arc::new(Denied), m::RecordDigest::new([1; 32]))
            .await,
        Err(local_browser::Error::Denied)
    ));
    assert_eq!(f.state(), before);
}
