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
    a: VerifiedCaptureWorkspace,
    b: VerifiedCaptureWorkspace,
    once: AtomicBool,
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
        if directory == self.a.directory {
            if self.once.swap(false, Ordering::SeqCst) {
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
                    .recv_timeout(Duration::from_secs(20))
                    .unwrap();
            }
            Ok(self.a.clone())
        } else if directory == self.b.directory {
            Ok(self.b.clone())
        } else {
            Err(jj_store_service::local_management::Error::Invalid)
        }
    }
}
struct Release(mpsc::Sender<()>);
impl Drop for Release {
    fn drop(&mut self) {
        let _ = self.0.send(());
    }
}

#[tokio::test(flavor = "current_thread")]
async fn blocked_directory_keeps_its_owner_while_another_source_delivers() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    let b_path = f.transcript.with_file_name("b.jsonl");
    fs::write(
        &b_path,
        fs::read_to_string(&f.transcript)
            .unwrap()
            .replace("000000000001", "000000000002")
            .replace(
                f.workspace(A).to_str().unwrap(),
                f.workspace(B).to_str().unwrap(),
            ),
    )
    .unwrap();
    let (entered, mut receiver) = tokio::sync::oneshot::channel();
    let (release, wait) = mpsc::channel();
    let release = Release(release);
    let gate = Gate {
        a: f.admission
            .inspect(f.workspace(A).to_str().unwrap())
            .await
            .unwrap(),
        b: f.admission
            .inspect(f.workspace(B).to_str().unwrap())
            .await
            .unwrap(),
        once: AtomicBool::new(true),
        entered: Mutex::new(Some(entered)),
        release: Mutex::new(wait),
    };
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
        Arc::new(gate),
    );
    let mut settings = config(&f);
    settings.max_readers = 2;
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
        s.observed_sources == 2 && s.resident_source_frames == 0
    })
    .await;
    f.append(json!({"type":"event_msg","payload":{"type":"task_started"}}));
    f.message("a", "blocked a body");
    use std::io::Write as _;
    writeln!(fs::OpenOptions::new().append(true).open(&b_path).unwrap(), "{}\n{}", json!({"type":"event_msg","payload":{"type":"task_started"}}), json!({"type":"response_item","payload":{"type":"message","id":"b","role":"assistant","content":[{"type":"output_text","text":"independent b body"}]}})).unwrap();
    tokio::time::timeout(Duration::from_secs(10), async {
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
    let pager = jj_store_service::collaboration::AgentSessionPager::new(
        Default::default(),
        Arc::new(jj_store_service::auth::SystemClock),
    )
    .unwrap();
    let session = f
        .host
        .endpoint(B)
        .unwrap()
        .bind_local_trusted(
            f.authority.principal(),
            jj_store_service::auth::Permission::Read,
        )
        .unwrap()
        .into_session();
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            worker.start_step().unwrap();
            worker.poll().await;
            let s = worker.status();
            assert!(s.running_sources <= 2);
            assert!(s.resident_source_frames <= 4);
            if !pager
                .begin(session.clone(), 10)
                .await
                .unwrap()
                .sessions
                .is_empty()
            {
                break;
            }
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    })
    .await
    .unwrap();
    tokio::time::sleep(Duration::from_millis(2010)).await;
    assert_eq!(worker.phase(), CaptureWorkerPhase::Overdue);
    assert_eq!(worker.status().caught_up, 0);
    worker.pause();
    assert_eq!(worker.phase(), CaptureWorkerPhase::Draining);
    assert!(worker.start_step().is_err());
    drop(release);
    quiesce(&mut worker).await;
    assert_eq!(worker.phase(), CaptureWorkerPhase::Paused);
    worker.resume().unwrap();
    settle(&mut worker, |s| s.caught_up == 2).await;
    worker.shutdown().await.unwrap();
}
