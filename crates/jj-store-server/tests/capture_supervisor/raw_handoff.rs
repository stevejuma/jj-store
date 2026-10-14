use super::*;
use jj_store_service::local_management::{
    VerifiedCaptureWorkspace, VerifiedWorkspace, WorkspaceVerifier,
};
use std::sync::{
    Mutex,
    atomic::{AtomicUsize, Ordering},
    mpsc,
};

struct Gate {
    proof: VerifiedCaptureWorkspace,
    remaining: AtomicUsize,
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
        path: &str,
    ) -> Result<VerifiedCaptureWorkspace, jj_store_service::local_management::Error> {
        if path != self.proof.directory {
            return Err(jj_store_service::local_management::Error::Invalid);
        }
        if self
            .remaining
            .try_update(Ordering::SeqCst, Ordering::SeqCst, |n| n.checked_sub(1))
            == Ok(1)
        {
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
struct Release(mpsc::Sender<()>);
impl Drop for Release {
    fn drop(&mut self) {
        let _ = self.0.send(());
    }
}

#[tokio::test(flavor = "current_thread")]
async fn overdue_and_cancelled_admission_keeps_raw_channel_before_and_after_physical_acquisition() {
    // bind rechecks directory twice. The fifth check is after physical acquisition.
    for block_at in [1, 5] {
        let mut f = Fixture::new(1).await;
        f.message("raw", "immutable pending raw handoff");
        drop(f.sources.acquire_active(f.start()).unwrap());
        let (entered, mut receiver) = tokio::sync::oneshot::channel();
        let (release, wait) = mpsc::channel();
        let release = Release(release);
        let gate = Arc::new(Gate {
            proof: f
                .admission
                .inspect(f.workspace(A).to_str().unwrap())
                .await
                .unwrap(),
            remaining: AtomicUsize::new(0),
            entered: Mutex::new(Some(entered)),
            release: Mutex::new(wait),
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
        let mut settings = config(&f);
        settings.max_pending_bytes = 1;
        let mut worker = CodexCaptureSupervisor::prepare(
            settings,
            CodexSupervisorLocations {
                home: None,
                codex_home: None,
            },
            resources(&f),
            Duration::from_secs(2),
        )
        .unwrap()
        .unwrap();
        settle(&mut worker, |s| {
            s.ingress_capacity_refusals == 1 && s.resident_source_frames == 1
        })
        .await;
        worker.pause();
        quiesce(&mut worker).await;
        gate.remaining.store(block_at, Ordering::SeqCst);
        worker.resume().unwrap();
        tokio::time::timeout(Duration::from_secs(15), async {
            loop {
                worker.start_step().unwrap();
                worker.poll().await;
                if receiver.try_recv().is_ok() {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
        })
        .await
        .unwrap();
        assert!(f.sources.acquire_admitted(f.qualified()).is_err());
        let lock = fs::read_dir(f.root.path().join("sources"))
            .unwrap()
            .map(|e| e.unwrap().path())
            .find(|p| {
                p.file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .starts_with("physical-")
            })
            .unwrap();
        let physical = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(lock)
            .unwrap();
        if block_at == 1 {
            physical
                .try_lock()
                .expect("parked physical ownership released before handoff");
            physical.unlock().unwrap();
        } else {
            assert!(
                physical.try_lock().is_err(),
                "fresh physical ownership held through final admission"
            );
        }
        tokio::time::sleep(Duration::from_millis(2010)).await;
        assert_eq!(worker.phase(), CaptureWorkerPhase::Overdue);
        assert_eq!(worker.status().caught_up, 0);
        worker.pause();
        assert_eq!(worker.phase(), CaptureWorkerPhase::Draining);
        assert!(f.sources.acquire_admitted(f.qualified()).is_err());
        f.catalog(&[]); // final handoff must also fail current native admission
        drop(release);
        quiesce(&mut worker).await;
        assert_eq!(worker.phase(), CaptureWorkerPhase::Paused);
        physical
            .try_lock()
            .expect("failed handoff drops fresh physical lease");
        physical.unlock().unwrap();
        assert!(
            f.sources.acquire_admitted(f.qualified()).is_err(),
            "raw channel survives cancellation or completed admission refusal"
        );
        assert!(f.spool.pending().unwrap().is_empty());
        let source = f.sources.lookup_admitted(&f.qualified()).unwrap().unwrap();
        assert!(
            f.spool
                .latest_reader_progress(serde_json::from_value(json!("codex")).unwrap(), source)
                .unwrap()
                .is_none()
        );
        f.catalog(&[A, B]);
        worker.resume().unwrap();
        quantum(&mut worker).await;
        quiesce(&mut worker).await;
        settle(&mut worker, |s| s.ingress_capacity_refusals == 1).await;
        worker.pause();
        quiesce(&mut worker).await;
        assert!(f.sources.acquire_admitted(f.qualified()).is_err());
        worker.shutdown().await.unwrap();
        drop(f.sources.acquire_admitted(f.qualified()).unwrap().unwrap());
    }
}
