use super::*;
use jj_store_capture::{
    CaptureResult,
    agent_capture::PendingCaptureStateV1,
    agent_publication::{AgentCapturePublisher, drain_capture_spool_entry},
};
use jj_store_model::{PublicationId, RepositoryId, RequestId};
use jj_store_server::capture_source::{CaptureSourceStep, CodexCaptureSource};
use jj_store_service::collaboration::{AgentReadSnapshot, AgentSessionPager};
use jj_store_service::transport::{
    AgentDeltaRequest, NormalizerProfileV1, PublishOutcome, Receipt, SanitizedObservationBatchV1,
};
use std::sync::Mutex;

// Lose only the acknowledgement after actual native authority has selected its receipt.
struct LostAcknowledgement<P> {
    native: P,
    receipt: Arc<Mutex<Option<Receipt>>>,
}
#[async_trait::async_trait]
impl<P: AgentCapturePublisher> AgentCapturePublisher for LostAcknowledgement<P> {
    async fn prepare_agent_capture(
        &self,
        request: RequestId,
        publication: PublicationId,
        profile: &NormalizerProfileV1,
        batch: &SanitizedObservationBatchV1,
        result: Vec<u8>,
    ) -> CaptureResult<AgentDeltaRequest> {
        self.native
            .prepare_agent_capture(request, publication, profile, batch, result)
            .await
    }
    async fn publish_agent_delta(
        &self,
        request: &AgentDeltaRequest,
    ) -> CaptureResult<PublishOutcome> {
        let PublishOutcome::Committed(receipt) = self.native.publish_agent_delta(request).await?
        else {
            panic!("fixture native publication did not commit")
        };
        *self.receipt.lock().unwrap() = Some(receipt);
        Ok(PublishOutcome::OutcomeUnknown {
            request_id: request.request_id,
            publication_id: request.publication_id,
        })
    }
}
async fn read(f: &Fixture, repository: RepositoryId) -> (AgentReadSnapshot, String) {
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
    let pager = AgentSessionPager::new(
        Default::default(),
        Arc::new(jj_store_service::auth::SystemClock),
    )
    .unwrap();
    let page = pager.begin(session.clone(), 10).await.unwrap();
    let mut body = String::new();
    for item in page.sessions {
        body.push_str(
            &serde_json::to_string(
                &pager
                    .detail(session.clone(), item.session_id)
                    .await
                    .unwrap(),
            )
            .unwrap(),
        );
    }
    (page.snapshot, body)
}
fn capacity(f: &Fixture) -> u64 {
    fs::read_dir(f.root.path().join("spool/pending"))
        .unwrap()
        .map(|entry| entry.unwrap().metadata().unwrap().len())
        .sum()
}

#[tokio::test(flavor = "current_thread")]
async fn full_prepared_queue_retains_unknown_receipt_and_recovers_independent_b_after_restart() {
    let mut f = Fixture::new(8 * 1024 * 1024).await;
    f.message(
        "a",
        &format!("{} {SECRET}", "durable A evidence ".repeat(32768)),
    );
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
    let partition = pipeline.partition();
    assert_eq!(
        pipeline.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    drop(pipeline);
    let captured = f.spool.pending().unwrap().remove(0);
    let proof = f
        .admission
        .inspect(f.workspace(A).to_str().unwrap())
        .await
        .unwrap();
    let receipt = Arc::new(Mutex::new(None));
    let lost = LostAcknowledgement {
        native: f.authority.publisher(&f.host, &f.admission, &proof),
        receipt: receipt.clone(),
    };
    let report = drain_capture_spool_entry(&mut f.spool, &captured, &lost)
        .await
        .unwrap();
    assert!(report.outcome_unknown);
    assert_eq!(report.committed, 0);
    assert_eq!(report.remaining, 1);
    drop(lost);
    let retained = f.spool.pending().unwrap();
    let PendingCaptureStateV1::Prepared { request } = &retained[0].state else {
        panic!("exact prepared intent missing")
    };
    assert_eq!(request.principal, f.authority.principal());
    let original = receipt.lock().unwrap().clone().unwrap();
    assert_eq!(original.request_id, request.request_id);
    assert_eq!(original.publication_id, request.publication_id);
    assert_eq!(original.principal, request.principal);
    assert_eq!(original.result, request.result);
    let native_before = read(&f, A).await;
    assert!(native_before.1.contains("durable A evidence"));
    assert!(!native_before.1.contains(SECRET));
    let durable_before = f
        .spool
        .latest_reader_progress(partition.integration, partition.source)
        .unwrap()
        .unwrap();
    let budget = capacity(&f);
    f.spool = AgentCaptureSpool::open(&f.root.path().join("spool"), budget).unwrap();
    f.append(json!({"type":"turn_context","payload":{"cwd":f.workspace(B)}}));
    f.message("b", &format!("backpressured offline B body {SECRET}"));
    f.append(json!({"type":"event_msg","payload":{"type":"task_complete"}}));
    f.catalog(&[B]);
    let mut settings = config(&f);
    settings.max_pending_bytes = budget;
    let mut worker = supervisor(&f, settings.clone());
    settle(&mut worker, |s| {
        s.tracked == 1
            && s.awaiting_activity == 0
            && s.pending_batches == 1
            && s.unavailable > 0
            && s.resident_source_frames == 1
    })
    .await;
    for _ in 0..16 {
        quantum(&mut worker).await;
        quiesce(&mut worker).await;
        assert_eq!(f.spool.pending().unwrap(), retained);
        assert_eq!(capacity(&f), budget);
        assert_eq!(
            f.spool
                .latest_reader_progress(partition.integration, partition.source)
                .unwrap(),
            Some(durable_before.clone())
        );
        assert_eq!(worker.status().caught_up, 0);
        assert!(worker.status().resident_source_frames <= 2);
        assert!(worker.status().resident_delivery_frames <= 2);
    }
    worker.pause();
    f.message("later-b", "append while full must survive exact restart");
    worker.shutdown().await.unwrap();
    drop(worker);
    f.spool = AgentCaptureSpool::open(&f.root.path().join("spool"), budget).unwrap();
    assert_eq!(f.spool.pending().unwrap(), retained);
    let mut worker = supervisor(&f, settings);
    settle(&mut worker, |s| {
        s.awaiting_activity == 0
            && s.pending_batches == 1
            && s.resident_source_frames == 1
            && s.unavailable > 0
    })
    .await;
    assert_eq!(f.spool.pending().unwrap(), retained);
    assert_eq!(
        f.spool
            .latest_reader_progress(partition.integration, partition.source)
            .unwrap(),
        Some(durable_before)
    );
    assert_eq!(read(&f, B).await.1, "");
    f.catalog(&[A, B]);
    settle(&mut worker, |s| s.caught_up == 1).await;
    assert!(f.spool.pending().unwrap().is_empty());
    assert_eq!(capacity(&f), 0);
    // The unknown A request resolves its exact native receipt without reauthoring A.
    assert_eq!(read(&f, A).await, native_before);
    let body = read(&f, B).await.1;
    assert!(body.contains("backpressured offline B body"));
    assert!(body.contains("append while full must survive exact restart"));
    assert!(!body.contains("durable A evidence"));
    assert!(!body.contains(SECRET));
    assert!(
        f.spool
            .latest_reader_progress(partition.integration, partition.source)
            .unwrap()
            .unwrap()
            .last_sequence
            > retained[0]
                .batch
                .observations
                .last()
                .unwrap()
                .source_sequence
    );
    worker.shutdown().await.unwrap();
}

#[tokio::test]
async fn captured_preparation_growth_withholds_exact_intent_until_explicit_capacity_relief() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    f.message("a", &format!("captured preparation budget {SECRET}"));
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
    let entry = f.spool.pending().unwrap().remove(0);
    // A pre-headroom binary durably accepted this same compact V1 record.
    // Preserve the legacy admission case rather than stripping live reservations.
    fs::write(
        f.root
            .path()
            .join("spool/pending")
            .join(format!("{}.json", entry.batch.batch_id)),
        serde_json::to_vec(&entry).unwrap(),
    )
    .unwrap();
    let before = read(&f, A).await;
    assert_eq!(before.1, "");
    let budget = capacity(&f);
    f.spool = AgentCaptureSpool::open(&f.root.path().join("spool"), budget).unwrap();
    let error = pipeline
        .publish_step(&entry, &mut f.context())
        .await
        .unwrap_err();
    let jj_store_server::Error::CaptureCapacity(refusal) = error else {
        panic!("preparation capacity diagnosis lost at source boundary");
    };
    assert_eq!(refusal.limit_bytes, budget);
    assert!(refusal.required_bytes > budget);
    assert_eq!(f.spool.pending().unwrap(), vec![entry.clone()]);
    assert_eq!(capacity(&f), budget);
    // Preparation may stage immutable objects, but cannot select a root or receipt
    // before its complete prepared intent is durably within the queue byte bound.
    assert_eq!(read(&f, A).await, before);
    f.spool = AgentCaptureSpool::open(&f.root.path().join("spool"), 4 * 1024 * 1024).unwrap();
    assert!(
        matches!(pipeline.publish_step(&entry,&mut f.context()).await.unwrap(),CaptureSourceStep::Published(report) if report.committed==1)
    );
    assert!(f.spool.pending().unwrap().is_empty());
    let body = read(&f, A).await.1;
    assert!(body.contains("captured preparation budget"));
    assert!(!body.contains(SECRET));
}
