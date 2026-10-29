use jj_vault::{Backup, Capture, Phase, Repository, Timing};
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize, Ordering},
    mpsc,
};
use std::time::Duration;
use tokio::sync::oneshot;

struct Provider {
    fail: Arc<AtomicBool>,
    calls: Arc<AtomicUsize>,
    dropped: Option<oneshot::Sender<()>>,
}
impl Repository for Provider {
    fn capture(&mut self) -> Result<Capture, String> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if self.fail.load(Ordering::SeqCst) {
            return Err("saved file is unreadable".into());
        }
        Ok(Capture {
            changed: false,
            checkpoint: None,
            operation: "op".into(),
            working_copy: "wc".into(),
        })
    }
}
impl Drop for Provider {
    fn drop(&mut self) {
        if let Some(dropped) = self.dropped.take() {
            let _ = dropped.send(());
        }
    }
}
fn provider() -> (Provider, oneshot::Receiver<()>) {
    let (sender, receiver) = oneshot::channel();
    (
        Provider {
            fail: Arc::new(AtomicBool::new(true)),
            calls: Arc::new(AtomicUsize::new(0)),
            dropped: Some(sender),
        },
        receiver,
    )
}

#[tokio::test]
async fn consuming_close_retires_the_worker_without_replacing_failure_with_closed() {
    let (provider, dropped) = provider();
    let backup = Backup::start_idle(provider, Timing::default()).unwrap();
    let state = backup.subscribe();
    assert_eq!(
        backup.close().await.unwrap_err(),
        "saved file is unreadable"
    );
    tokio::time::timeout(Duration::from_secs(2), dropped)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(state.borrow().phase, Phase::Failed);
    assert_eq!(
        state.borrow().error.as_deref(),
        Some("saved file is unreadable")
    );
}

#[tokio::test]
async fn dropping_the_owner_retires_the_worker_despite_retained_change_handles() {
    let (provider, dropped) = provider();
    let backup = Backup::start_idle(provider, Timing::default()).unwrap();
    let changes = backup.changes();
    drop(backup);
    let result = tokio::time::timeout(Duration::from_secs(2), dropped).await;
    drop(changes);
    result.unwrap().unwrap();
}

#[tokio::test]
async fn failed_close_keeps_the_original_worker_and_retry_captures_repaired_files() {
    let (provider, mut dropped) = provider();
    let fail = provider.fail.clone();
    let calls = provider.calls.clone();
    let mut backup = Backup::start_idle(provider, Timing::default()).unwrap();
    let state = backup.subscribe();
    assert_eq!(
        backup.try_close().await.unwrap_err(),
        "saved file is unreadable"
    );
    assert_eq!(state.borrow().phase, Phase::Failed);
    assert!(matches!(
        dropped.try_recv(),
        Err(oneshot::error::TryRecvError::Empty)
    ));
    fail.store(false, Ordering::SeqCst);
    backup.try_close().await.unwrap();
    dropped.await.unwrap();
    assert_eq!(state.borrow().phase, Phase::Closed);
    assert_eq!(state.borrow().error, None);
    assert_eq!(calls.load(Ordering::SeqCst), 2);
    backup.try_close().await.unwrap();
    assert_eq!(
        calls.load(Ordering::SeqCst),
        2,
        "Successful close is idempotent"
    );
    assert!(backup.capture().await.is_err());
}

#[tokio::test]
async fn dropping_after_failed_retryable_close_retires_the_retained_worker() {
    let (provider, dropped) = provider();
    let mut backup = Backup::start_idle(provider, Timing::default()).unwrap();
    let changes = backup.changes();
    let state = backup.subscribe();
    assert!(backup.try_close().await.is_err());
    drop(backup);
    tokio::time::timeout(Duration::from_secs(2), dropped)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(state.borrow().phase, Phase::Failed);
    drop(changes);
}

struct PausedProvider {
    calls: Arc<AtomicUsize>,
    release: mpsc::Receiver<()>,
}
impl Repository for PausedProvider {
    fn capture(&mut self) -> Result<Capture, String> {
        if self.calls.fetch_add(1, Ordering::SeqCst) == 0 {
            self.release.recv().unwrap();
        }
        Ok(Capture {
            changed: false,
            checkpoint: None,
            operation: "op".into(),
            working_copy: "wc".into(),
        })
    }
}
async fn paused() -> (Backup, mpsc::Sender<()>, Arc<AtomicUsize>) {
    let (release, receiver) = mpsc::channel();
    let calls = Arc::new(AtomicUsize::new(0));
    let backup = Backup::start(
        PausedProvider {
            calls: calls.clone(),
            release: receiver,
        },
        Timing::default(),
    )
    .unwrap();
    tokio::time::timeout(Duration::from_secs(2), async {
        while calls.load(Ordering::SeqCst) == 0 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    (backup, release, calls)
}

#[tokio::test]
async fn full_queue_preserves_close_barrier_without_blocking_the_async_runtime() {
    let (mut backup, release, calls) = paused().await;
    for _ in 0..100 {
        backup.changed();
    }
    let closing = backup.try_close();
    tokio::pin!(closing);
    assert!(
        tokio::time::timeout(Duration::from_millis(20), &mut closing)
            .await
            .is_err()
    );
    release.send(()).unwrap();
    tokio::time::timeout(Duration::from_secs(2), closing)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn cancelled_close_wait_can_retry_the_already_queued_barrier() {
    let (mut backup, release, calls) = paused().await;
    assert!(
        tokio::time::timeout(Duration::from_millis(20), backup.try_close())
            .await
            .is_err()
    );
    release.send(()).unwrap();
    tokio::time::timeout(Duration::from_secs(2), backup.try_close())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(backup.subscribe().borrow().phase, Phase::Closed);
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}
