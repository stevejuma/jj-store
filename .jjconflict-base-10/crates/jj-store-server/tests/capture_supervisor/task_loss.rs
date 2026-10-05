use super::*;
use jj_store_server::capture_source::{CaptureSourceStep, CodexCaptureSource};
use jj_store_service::local_management::{
    VerifiedCaptureWorkspace, VerifiedWorkspace, WorkspaceVerifier,
};
use std::sync::atomic::{AtomicBool, Ordering};

struct PanicOnce {
    a: VerifiedCaptureWorkspace,
    b: VerifiedCaptureWorkspace,
    fired: Arc<AtomicBool>,
}
impl WorkspaceVerifier for PanicOnce {
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
            Ok(self.a.clone())
        } else if directory == self.b.directory {
            if !self.fired.swap(true, Ordering::SeqCst) {
                panic!("injected source owner loss");
            }
            Ok(self.b.clone())
        } else {
            Err(jj_store_service::local_management::Error::Invalid)
        }
    }
}

#[tokio::test(flavor = "current_thread")]
async fn source_task_loss_retains_exact_queue_and_requires_renewed_current_admission() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    f.message("first", &format!("retained A before task loss {SECRET}"));
    let active = f.start();
    let mut pipeline = CodexCaptureSource::acquire_active(
        f.sources.clone(),
        active,
        f.privacy(),
        &mut f.context(),
    )
    .await
    .unwrap()
    .into_pipeline();
    assert_eq!(
        pipeline.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let partition = pipeline.partition();
    drop(pipeline);
    let queue = f.spool.pending().unwrap();
    let cursor = f
        .spool
        .latest_reader_progress(partition.integration, partition.source)
        .unwrap();
    let path = f
        .root
        .path()
        .join("spool/pending")
        .join(format!("{}.json", queue[0].batch.batch_id));
    let bytes = fs::read(&path).unwrap();
    let fired = Arc::new(AtomicBool::new(false));
    let verifier = PanicOnce {
        a: f.admission
            .inspect(f.workspace(A).to_str().unwrap())
            .await
            .unwrap(),
        b: f.admission
            .inspect(f.workspace(B).to_str().unwrap())
            .await
            .unwrap(),
        fired: fired.clone(),
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
        Arc::new(verifier),
    );
    f.catalog(&[B]);
    f.append(json!({"type":"turn_context","payload":{"cwd":f.workspace(B)}}));
    f.message(
        "tail",
        &format!("unacknowledged B survives owner loss {SECRET}"),
    );
    let mut worker = supervisor(&f, config(&f));
    settle(&mut worker, |s| {
        fired.load(Ordering::SeqCst)
            && s.tracked == 1
            && s.running_sources == 0
            && s.resident_source_frames == 0
            && s.unavailable > 0
    })
    .await;
    for _ in 0..8 {
        quantum(&mut worker).await;
        quiesce(&mut worker).await;
        assert_eq!(worker.status().caught_up, 0);
        assert_eq!(worker.status().running_sources, 0);
        assert_eq!(worker.status().resident_source_frames, 0);
        assert_eq!(f.spool.pending().unwrap(), queue);
        assert_eq!(fs::read(&path).unwrap(), bytes);
        assert_eq!(
            f.spool
                .latest_reader_progress(partition.integration, partition.source)
                .unwrap(),
            cursor
        );
    }
    // This closed owner cannot borrow a new reader on resume.
    worker.pause();
    worker.resume().unwrap();
    for _ in 0..8 {
        quantum(&mut worker).await;
        quiesce(&mut worker).await;
        assert_eq!(worker.status().caught_up, 0);
        assert_eq!(worker.status().running_sources, 0);
        assert_eq!(worker.status().resident_source_frames, 0);
        assert_eq!(f.spool.pending().unwrap(), queue);
        assert_eq!(fs::read(&path).unwrap(), bytes);
        assert_eq!(
            f.spool
                .latest_reader_progress(partition.integration, partition.source)
                .unwrap(),
            cursor
        );
    }
    worker.shutdown().await.unwrap();
    drop(worker);
    f.catalog(&[A, B]);
    // An explicit replacement reconstructs only admitted native-session progress
    // and independently verifies current B/Write before reading its offline tail.
    let mut worker = supervisor(&f, config(&f));
    settle(&mut worker, |s| s.caught_up == 1).await;
    assert!(f.spool.pending().unwrap().is_empty());
    let current = f
        .spool
        .latest_reader_progress(partition.integration, partition.source)
        .unwrap()
        .unwrap();
    assert_eq!(current.repository, B);
    assert!(current.last_sequence > cursor.unwrap().last_sequence);
    let pager = jj_store_service::collaboration::AgentSessionPager::new(
        Default::default(),
        Arc::new(jj_store_service::auth::SystemClock),
    )
    .unwrap();
    for (repository, accepted, absent) in [
        (
            A,
            "retained A before task loss",
            "unacknowledged B survives owner loss",
        ),
        (
            B,
            "unacknowledged B survives owner loss",
            "retained A before task loss",
        ),
    ] {
        let session = f
            .host
            .endpoint(repository)
            .unwrap()
            .bind_local_trusted(
                f.authority.principal(),
                jj_store_service::auth::Permission::Read,
            )
            .unwrap()
            .into_session();
        let page = pager.begin(session.clone(), 10).await.unwrap();
        assert_eq!(page.sessions.len(), 1);
        let detail = pager
            .detail(session, page.sessions[0].session_id)
            .await
            .unwrap();
        let body = serde_json::to_string(&detail).unwrap();
        assert!(body.contains(accepted));
        assert!(!body.contains(absent));
        assert!(!body.contains(SECRET));
    }
    worker.shutdown().await.unwrap();
}
