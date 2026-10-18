use jj_vault::{Backup, Capture, Phase, Repository, Timing};
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
    time::Duration,
};

struct Provider {
    calls: Arc<AtomicUsize>,
    caller: std::thread::ThreadId,
}
impl Repository for Provider {
    fn capture(&mut self) -> Result<Capture, String> {
        assert_ne!(
            self.caller,
            std::thread::current().id(),
            "Capture cannot run on the caller thread"
        );
        self.calls.fetch_add(1, Ordering::SeqCst);
        Ok(Capture {
            changed: false,
            checkpoint: None,
            operation: "op".into(),
            working_copy: "wc".into(),
        })
    }
}
fn timing() -> Timing {
    Timing {
        quiet: Duration::from_millis(20),
        maximum_delay: Duration::from_millis(100),
        reconciliation: Duration::from_secs(1),
    }
}
#[test]
fn idle_start_waits_for_watcher_registration_and_explicit_capture() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let calls = Arc::new(AtomicUsize::new(0));
        let backup = Backup::start_idle(
            Provider {
                calls: calls.clone(),
                caller: std::thread::current().id(),
            },
            timing(),
        )
        .unwrap();
        tokio::time::sleep(Duration::from_millis(60)).await;
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        backup.capture().await.unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        backup.close().await.unwrap();
    });
}
#[test]
fn runs_on_worker_and_drains_final_capture_before_close_returns() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let calls = Arc::new(AtomicUsize::new(0));
        let backup = Backup::start(
            Provider {
                calls: calls.clone(),
                caller: std::thread::current().id(),
            },
            timing(),
        )
        .unwrap();
        backup.capture().await.unwrap();
        for _ in 0..100 {
            backup.changed();
        }
        backup.close().await.unwrap();
        let count = calls.load(Ordering::SeqCst);
        assert!(count >= 2);
        tokio::time::sleep(Duration::from_millis(80)).await;
        assert_eq!(calls.load(Ordering::SeqCst), count);
    });
}
struct PausedProvider {
    calls: Arc<AtomicUsize>,
    started: mpsc::Sender<()>,
    release: mpsc::Receiver<()>,
}
impl Repository for PausedProvider {
    fn capture(&mut self) -> Result<Capture, String> {
        if self.calls.fetch_add(1, Ordering::SeqCst) == 0 {
            self.started.send(()).unwrap();
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
#[test]
fn changes_during_in_flight_capture_schedule_another_capture() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let calls = Arc::new(AtomicUsize::new(0));
        let (started, started_rx) = mpsc::channel();
        let (release, release_rx) = mpsc::channel();
        let backup = Backup::start(
            PausedProvider {
                calls: calls.clone(),
                started,
                release: release_rx,
            },
            timing(),
        )
        .unwrap();
        started_rx.recv_timeout(Duration::from_secs(2)).unwrap();
        let mut status = backup.subscribe();
        backup.changed();
        release.send(()).unwrap();
        tokio::time::timeout(Duration::from_secs(2), async {
            loop {
                status.changed().await.unwrap();
                if status.borrow().phase == Phase::Ready && calls.load(Ordering::SeqCst) >= 2 {
                    break;
                }
            }
        })
        .await
        .unwrap();
        backup.close().await.unwrap();
    });
}
