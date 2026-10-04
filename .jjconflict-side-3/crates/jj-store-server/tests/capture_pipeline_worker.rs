#![cfg(unix)]
#[path = "capture_source/fixture.rs"]
mod fixture;
use fixture::*;
use jj_store_server::{
    capture_source::CodexCaptureSource,
    capture_worker::{
        CapturePipelineWorker, CaptureWorkerCommand, CaptureWorkerPhase, CaptureWorkerPoll,
    },
};
use std::{sync::Arc, time::Duration};

async fn finish(worker: &mut CapturePipelineWorker) -> CaptureWorkerPoll {
    tokio::time::timeout(Duration::from_secs(30), async {
        loop {
            let outcome = worker.poll().await;
            if outcome != CaptureWorkerPoll::Running {
                return outcome;
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap()
}

#[tokio::test(flavor = "current_thread")]
async fn pipeline_work_runs_with_owned_resources_and_clean_shutdown() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    f.message("private", &format!("worker body {SECRET}"));
    let active = f.start();
    let sources = f.sources.clone();
    let privacy = f.privacy();
    let pipeline =
        CodexCaptureSource::acquire_active(sources.clone(), active, privacy, &mut f.context())
            .await
            .unwrap()
            .into_pipeline();
    let qualified = f.qualified();
    let spool_path = f.root.path().join("spool");
    let mut worker = CapturePipelineWorker::new(
        pipeline,
        Arc::new(f.authority),
        f.host,
        f.admission,
        f.spool,
        Duration::from_secs(30),
    )
    .unwrap();
    worker.pause();
    assert_eq!(worker.phase(), CaptureWorkerPhase::Paused);
    assert!(worker.start(CaptureWorkerCommand::Ingest).is_err());
    worker.resume().unwrap();
    worker.start(CaptureWorkerCommand::Ingest).unwrap();
    assert!(worker.start(CaptureWorkerCommand::Ingest).is_err());
    assert!(matches!(
        finish(&mut worker).await,
        CaptureWorkerPoll::Completed(jj_store_server::capture_source::CaptureSourceStep::Enqueued)
    ));
    let spool =
        jj_store_capture::agent_capture::AgentCaptureSpool::open(&spool_path, 4 * 1024 * 1024)
            .unwrap();
    let queued = spool.pending().unwrap();
    assert_eq!(queued.len(), 1);
    assert!(!serde_json::to_string(&queued).unwrap().contains(SECRET));
    assert!(sources.acquire_admitted(qualified.clone()).is_err());
    worker
        .start(CaptureWorkerCommand::Publish(Box::new(queued[0].clone())))
        .unwrap();
    assert!(
        matches!(finish(&mut worker).await, CaptureWorkerPoll::Completed(jj_store_server::capture_source::CaptureSourceStep::Published(report)) if report.committed == 1)
    );
    assert!(spool.pending().unwrap().is_empty());
    worker.shutdown().await.unwrap();
    assert_eq!(worker.phase(), CaptureWorkerPhase::Stopped);
    assert!(worker.resume().is_err());
    assert!(worker.start(CaptureWorkerCommand::Ingest).is_err());
    assert!(sources.acquire_admitted(qualified).unwrap().is_some());
}

#[tokio::test(flavor = "current_thread")]
async fn owned_step_failure_preserves_state_and_cannot_override_native_authority() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    let active = f.start();
    let sources = f.sources.clone();
    let privacy = f.privacy();
    let pipeline = CodexCaptureSource::acquire_active(sources, active, privacy, &mut f.context())
        .await
        .unwrap()
        .into_pipeline();
    let partition = pipeline.partition();
    let spool_path = f.root.path().join("spool");
    f.without_native_authority();
    let mut worker = CapturePipelineWorker::new(
        pipeline,
        Arc::new(f.authority),
        f.host,
        f.admission,
        f.spool,
        Duration::from_secs(30),
    )
    .unwrap();
    worker.start(CaptureWorkerCommand::Ingest).unwrap();
    assert_eq!(finish(&mut worker).await, CaptureWorkerPoll::Unavailable);
    assert_eq!(worker.phase(), CaptureWorkerPhase::Ready);
    worker.pause();
    worker.resume().unwrap();
    worker.start(CaptureWorkerCommand::Ingest).unwrap();
    assert_eq!(finish(&mut worker).await, CaptureWorkerPoll::Unavailable);
    let spool =
        jj_store_capture::agent_capture::AgentCaptureSpool::open(&spool_path, 4 * 1024 * 1024)
            .unwrap();
    assert!(spool.pending().unwrap().is_empty());
    assert!(
        spool
            .latest_reader_progress(partition.integration, partition.source)
            .unwrap()
            .is_none()
    );
    worker.shutdown().await.unwrap();
}
