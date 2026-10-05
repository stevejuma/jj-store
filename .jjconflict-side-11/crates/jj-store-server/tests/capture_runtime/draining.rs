use super::*;
use jj_store_service::local_management::{
    VerifiedCaptureWorkspace, VerifiedWorkspace, WorkspaceVerifier,
};
use std::sync::{
    Mutex,
    atomic::{AtomicBool, Ordering},
    mpsc,
};
struct Gate {
    proof: VerifiedCaptureWorkspace,
    armed: AtomicBool,
    entered: Mutex<Option<tokio::sync::oneshot::Sender<()>>>,
    release: Mutex<mpsc::Receiver<()>>,
}
impl WorkspaceVerifier for Gate {
    fn inspect(
        &self,
        _: &str,
    ) -> Result<VerifiedWorkspace, jj_store_service::local_management::Error> {
        Err(jj_store_service::local_management::Error::Unavailable)
    }
    fn inspect_capture_directory(
        &self,
        directory: &str,
    ) -> Result<VerifiedCaptureWorkspace, jj_store_service::local_management::Error> {
        if directory != self.proof.directory {
            return Err(jj_store_service::local_management::Error::Invalid);
        }
        if self.armed.swap(false, Ordering::SeqCst) {
            self.entered
                .lock()
                .unwrap()
                .take()
                .unwrap()
                .send(())
                .unwrap();
            self.release
                .lock()
                .unwrap()
                .recv_timeout(Duration::from_secs(30))
                .unwrap();
        }
        Ok(self.proof.clone())
    }
}
pub(super) struct Release(mpsc::Sender<()>);
impl Drop for Release {
    fn drop(&mut self) {
        let _ = self.0.send(());
    }
}

async fn blocked() -> (Fixture, RunningCodexCapture, Release) {
    blocked_with(|s| RunningCodexCapture::start(s).unwrap()).await
}

pub(super) async fn blocked_with(
    start: impl FnOnce(CodexCaptureSupervisor) -> RunningCodexCapture,
) -> (Fixture, RunningCodexCapture, Release) {
    let mut f = Fixture::new(1).await;
    f.message("pending", "exact raw driver frame");
    drop(f.sources.acquire_active(f.start()).unwrap());
    let (entered, receiver) = tokio::sync::oneshot::channel();
    let (release, blocked) = mpsc::channel();
    let gate = Arc::new(Gate {
        proof: f
            .admission
            .inspect(f.workspace(A).to_str().unwrap())
            .await
            .unwrap(),
        armed: AtomicBool::new(false),
        entered: Mutex::new(Some(entered)),
        release: Mutex::new(blocked),
    });
    let identity =
        jj_store_server::installation::Identity::load(&f.root.path().join("installation.json"))
            .unwrap();
    f.admission = jj_store_server::capture_admission::CaptureAdmission::new(
        Arc::new(
            jj_store_server::registry::Registry::new(
                f.root.path().join("repositories.json"),
                identity,
            )
            .unwrap(),
        ),
        gate.clone(),
    );
    let r = start(supervisor(&f));
    wait(&r, |s| {
        s.status.ingress_capacity_refusals == 1 && s.status.resident_source_frames == 1
    })
    .await;
    r.pause().unwrap();
    wait(&r, |s| s.phase == CaptureWorkerPhase::Paused).await;
    gate.armed.store(true, Ordering::SeqCst);
    r.resume().unwrap();
    tokio::time::timeout(Duration::from_secs(10), receiver)
        .await
        .unwrap()
        .unwrap();
    assert!(f.sources.acquire_admitted(f.qualified()).is_err());
    (f, r, Release(release))
}

#[tokio::test(flavor = "current_thread")]
async fn shutdown_timeout_retains_join_and_raw_ownership_until_a_later_wait() {
    let (f, mut r, release) = blocked().await;
    assert!(r.shutdown().await.is_err());
    assert_eq!(r.snapshot().phase, CaptureWorkerPhase::Draining);
    assert!(r.resume().is_err());
    assert!(f.sources.acquire_admitted(f.qualified()).is_err());
    assert!(f.spool.pending().unwrap().is_empty());
    // Also cross the supervisor's inner deadline before releasing the owner.
    tokio::time::sleep(Duration::from_millis(200)).await;
    drop(release);
    // A deadline is an incomplete wait, not a terminal drain failure. Once the
    // exact retained owner settles, retry confirms successful shutdown.
    r.shutdown().await.unwrap();
    assert_eq!(r.snapshot().phase, CaptureWorkerPhase::Stopped);
    assert_eq!(r.snapshot().status.resident_source_frames, 0);
    assert!(f.spool.pending().unwrap().is_empty());
    drop(f.sources.acquire_admitted(f.qualified()).unwrap().unwrap());
    r.shutdown().await.unwrap(); // a settled join is never polled twice
}

#[tokio::test(flavor = "current_thread")]
async fn dropping_runtime_requests_stop_while_blocking_owner_retains_raw_channel() {
    let (f, r, release) = blocked().await;
    let control = r.controls();
    drop(r);
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert!(control.pause().is_err() && control.resume().is_err());
    assert_eq!(
        control.snapshot().unwrap().phase,
        CaptureWorkerPhase::Draining
    );
    assert!(f.sources.acquire_admitted(f.qualified()).is_err());
    assert!(f.spool.pending().unwrap().is_empty());
    drop(release);
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            if let Ok(Some(source)) = f.sources.acquire_admitted(f.qualified()) {
                drop(source);
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    tokio::time::timeout(Duration::from_secs(10), async {
        while control.snapshot().unwrap().phase != CaptureWorkerPhase::Stopped {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    assert_eq!(control.snapshot().unwrap().status.resident_source_frames, 0);
    assert!(f.spool.pending().unwrap().is_empty());
}
