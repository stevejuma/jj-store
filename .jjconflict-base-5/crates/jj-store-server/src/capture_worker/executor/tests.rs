use super::*;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    mpsc,
};
use tokio::sync::oneshot;

enum Command {
    Wait,
    Complete,
    LateComplete,
    Panic,
}
struct Fake {
    entered: Option<oneshot::Sender<()>>,
    release: mpsc::Receiver<()>,
    calls: Arc<AtomicUsize>,
    _life: Arc<()>,
}
#[async_trait::async_trait]
impl OwnedWork for Fake {
    type Command = Command;
    type Output = u64;
    async fn step(&mut self, command: Command) -> u64 {
        self.calls.fetch_add(1, Ordering::SeqCst);
        match command {
            Command::Complete => 42,
            Command::Panic => panic!("owned capture fixture task loss"),
            Command::Wait | Command::LateComplete => {
                if let Some(entered) = self.entered.take() {
                    let _ = entered.send(());
                }
                self.release.recv_timeout(Duration::from_secs(10)).unwrap();
                if matches!(command, Command::LateComplete) {
                    42
                } else {
                    std::future::pending().await
                }
            }
        }
    }
}
struct Release(mpsc::Sender<()>);
impl Drop for Release {
    fn drop(&mut self) {
        let _ = self.0.send(());
    }
}
fn fake() -> (Fake, oneshot::Receiver<()>, Release) {
    let (entered, receiver) = oneshot::channel();
    let (release, wait) = mpsc::channel();
    (
        Fake {
            entered: Some(entered),
            release: wait,
            calls: Arc::default(),
            _life: Arc::new(()),
        },
        receiver,
        Release(release),
    )
}
async fn finish(worker: &mut OwnedWorker<Fake>) -> Completion<u64> {
    tokio::time::timeout(Duration::from_secs(2), async {
        loop {
            let outcome = worker.poll().await;
            if !matches!(outcome, Completion::Running) {
                return outcome;
            }
            tokio::time::sleep(Duration::from_millis(2)).await;
        }
    })
    .await
    .unwrap()
}

struct SettledFake {
    state: Fake,
    settled: Arc<AtomicUsize>,
}
#[async_trait::async_trait]
impl OwnedWork for SettledFake {
    type Command = Command;
    type Output = u64;
    async fn step(&mut self, command: Command) -> u64 {
        self.state.step(command).await
    }
    fn settled(&mut self) {
        self.settled.fetch_add(1, Ordering::SeqCst);
    }
}
#[tokio::test(flavor = "current_thread")]
async fn settlement_hook_follows_actual_cancelled_future_and_keeps_original_state() {
    let (state, entered, release) = fake();
    let calls = state.calls.clone();
    let settled = Arc::new(AtomicUsize::new(0));
    let mut worker = OwnedWorker::new(
        SettledFake {
            state,
            settled: settled.clone(),
        },
        Duration::from_secs(1),
    )
    .unwrap();
    worker.start(Command::Wait).unwrap();
    entered.await.unwrap();
    worker.pause();
    assert_eq!(
        settled.load(Ordering::SeqCst),
        0,
        "hook ran before blocking step returned"
    );
    drop(release);
    let outcome = tokio::time::timeout(Duration::from_secs(2), async {
        loop {
            match worker.poll().await {
                Completion::Running => tokio::time::sleep(Duration::from_millis(2)).await,
                outcome => return outcome,
            }
        }
    })
    .await
    .unwrap();
    assert!(matches!(outcome, Completion::Cancelled));
    assert_eq!(settled.load(Ordering::SeqCst), 1);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    worker.resume().unwrap();
    worker.start(Command::Complete).unwrap();
    tokio::time::timeout(Duration::from_secs(2), async {
        loop {
            match worker.poll().await {
                Completion::Running => tokio::time::sleep(Duration::from_millis(2)).await,
                Completion::Completed(42) => break,
                _ => panic!("original state did not resume"),
            }
        }
    })
    .await
    .unwrap();
    assert_eq!(settled.load(Ordering::SeqCst), 2);
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[tokio::test(flavor = "current_thread")]
async fn blocking_io_deadline_retains_owner_and_keeps_runtime_responsive() {
    let (state, entered, release) = fake();
    let calls = state.calls.clone();
    let life = Arc::downgrade(&state._life);
    let mut worker = OwnedWorker::new(state, Duration::from_millis(100)).unwrap();
    worker.start(Command::Wait).unwrap();
    entered.await.unwrap();
    tokio::time::sleep(Duration::from_millis(120)).await;
    assert_eq!(worker.phase(), CaptureWorkerPhase::Overdue);
    assert!(matches!(worker.poll().await, Completion::Running));
    assert!(life.upgrade().is_some());
    assert!(worker.start(Command::Complete).is_err());
    drop(release);
    assert!(matches!(finish(&mut worker).await, Completion::Deadline));
    assert!(life.upgrade().is_some());
    worker.start(Command::Complete).unwrap();
    assert!(matches!(
        finish(&mut worker).await,
        Completion::Completed(42)
    ));
    assert_eq!(calls.load(Ordering::SeqCst), 2);
    worker.shutdown().await.unwrap();
    assert!(life.upgrade().is_none());
}

#[tokio::test(flavor = "current_thread")]
async fn pause_resume_cannot_replace_an_undrained_owner() {
    let (state, entered, release) = fake();
    let mut worker = OwnedWorker::new(state, Duration::from_secs(2)).unwrap();
    worker.start(Command::Wait).unwrap();
    entered.await.unwrap();
    worker.pause();
    assert_eq!(worker.phase(), CaptureWorkerPhase::Draining);
    assert!(worker.start(Command::Complete).is_err());
    worker.resume().unwrap();
    assert!(worker.start(Command::Complete).is_err());
    drop(release);
    assert!(matches!(finish(&mut worker).await, Completion::Cancelled));
    worker.start(Command::Complete).unwrap();
    assert!(matches!(
        finish(&mut worker).await,
        Completion::Completed(42)
    ));
    worker.shutdown().await.unwrap();
}

#[test]
fn expired_queued_work_never_starts_its_source_step() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .max_blocking_threads(1)
        .build()
        .unwrap();
    runtime.block_on(async {
        let (held, entered, release) = fake();
        let blocker = tokio::task::spawn_blocking(move || {
            let mut held = held;
            held.entered.take().unwrap().send(()).unwrap();
            held.release.recv_timeout(Duration::from_secs(10)).unwrap();
        });
        entered.await.unwrap();
        let (state, _, _unused_release) = fake();
        let calls = state.calls.clone();
        let mut worker = OwnedWorker::new(state, Duration::from_millis(20)).unwrap();
        worker.start(Command::Complete).unwrap();
        tokio::time::sleep(Duration::from_millis(40)).await;
        assert_eq!(worker.phase(), CaptureWorkerPhase::Overdue);
        drop(release);
        blocker.await.unwrap();
        assert!(matches!(finish(&mut worker).await, Completion::Deadline));
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        worker.shutdown().await.unwrap();
    });
}

#[tokio::test(flavor = "current_thread")]
async fn shutdown_timeout_retains_owner_until_settlement() {
    let (state, entered, release) = fake();
    let life = Arc::downgrade(&state._life);
    let mut worker = OwnedWorker::new(state, Duration::from_secs(30)).unwrap();
    worker.start(Command::Wait).unwrap();
    entered.await.unwrap();
    assert!(worker.shutdown().await.is_err());
    assert_eq!(worker.phase(), CaptureWorkerPhase::Draining);
    assert!(life.upgrade().is_some());
    assert!(worker.resume().is_err());
    assert!(worker.start(Command::Complete).is_err());
    drop(release);
    worker.shutdown().await.unwrap();
    assert_eq!(worker.phase(), CaptureWorkerPhase::Stopped);
    assert!(life.upgrade().is_none());
}

#[tokio::test(flavor = "current_thread")]
async fn completed_late_result_remains_known() {
    let (state, entered, release) = fake();
    let mut worker = OwnedWorker::new(state, Duration::from_millis(100)).unwrap();
    worker.start(Command::LateComplete).unwrap();
    entered.await.unwrap();
    tokio::time::sleep(Duration::from_millis(120)).await;
    assert_eq!(worker.phase(), CaptureWorkerPhase::Overdue);
    drop(release);
    assert!(matches!(
        finish(&mut worker).await,
        Completion::Completed(42)
    ));
    worker.shutdown().await.unwrap();
}

#[tokio::test]
async fn task_loss_closes_the_owner_without_replacement() {
    let (state, _, _release) = fake();
    let mut worker = OwnedWorker::new(state, Duration::from_secs(1)).unwrap();
    worker.start(Command::Panic).unwrap();
    assert!(matches!(finish(&mut worker).await, Completion::Lost));
    assert_eq!(worker.phase(), CaptureWorkerPhase::Stopped);
    assert!(worker.start(Command::Complete).is_err());
    assert!(worker.resume().is_err());
}

#[tokio::test(flavor = "current_thread")]
async fn dropped_controller_keeps_running_resources_until_settlement() {
    let (state, entered, release) = fake();
    let life = Arc::downgrade(&state._life);
    let mut worker = OwnedWorker::new(state, Duration::from_secs(30)).unwrap();
    worker.start(Command::Wait).unwrap();
    entered.await.unwrap();
    drop(worker);
    assert!(life.upgrade().is_some());
    drop(release);
    tokio::time::timeout(Duration::from_secs(2), async {
        while life.upgrade().is_some() {
            tokio::time::sleep(Duration::from_millis(2)).await;
        }
    })
    .await
    .unwrap();
}

#[test]
fn invalid_deadline_fails_before_dispatch() {
    for duration in [Duration::ZERO, Duration::from_secs(301)] {
        let (state, _, _release) = fake();
        assert!(OwnedWorker::new(state, duration).is_err());
    }
}
