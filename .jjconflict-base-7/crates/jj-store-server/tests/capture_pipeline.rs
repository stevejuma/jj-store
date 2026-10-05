#![cfg(unix)]
mod capture_source {
    pub mod fixture;
}
use capture_source::fixture::*;
use jj_store_capture::{
    CaptureResult,
    agent_capture::{AgentCaptureSpool, PendingCaptureStateV1},
    agent_publication::{AgentCapturePublisher, drain_capture_spool_entry},
};
use jj_store_model::{PublicationId, RequestId};
use jj_store_server::capture_source::{
    CaptureResumeStep, CaptureSourceStep, CodexCaptureResume, CodexCaptureSource,
};
use jj_store_service::transport::{
    AgentDeltaRequest, NormalizerProfileV1, PublishOutcome, SanitizedObservationBatchV1,
};
use serde_json::json;
use std::fs;
#[path = "capture_source/progress_capacity.rs"]
mod progress_capacity;

#[tokio::test]
async fn revoked_pending_repository_does_not_block_later_independent_ingress_and_publication() {
    let mut fixture = Fixture::new(4 * 1024 * 1024).await;
    let active = fixture.start();
    let sources = fixture.sources.clone();
    let privacy = fixture.privacy();
    let source =
        CodexCaptureSource::acquire_active(sources, active, privacy, &mut fixture.context())
            .await
            .unwrap();
    let mut pipeline = source.into_pipeline();
    assert_eq!(
        pipeline.ingest_step(&mut fixture.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let retained = fixture.spool.pending().unwrap();
    assert_eq!(retained.len(), 1);
    fixture.catalog(&[B]);
    assert_eq!(
        pipeline
            .publish_step(&retained[0], &mut fixture.context())
            .await
            .unwrap(),
        CaptureSourceStep::Withheld
    );
    assert_eq!(fixture.spool.pending().unwrap(), retained);
    fixture.append(json!({"type":"turn_context","payload":{"cwd":fixture.workspace(B)}}));
    fixture.append(json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"b"}}));
    fixture.message("b", "independently ingested b body");
    assert_eq!(
        pipeline.ingest_step(&mut fixture.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let queue = fixture.spool.pending().unwrap();
    let next = queue.iter().find(|entry| entry.repository == B).unwrap();
    assert!(
        matches!(pipeline.publish_step(next, &mut fixture.context()).await.unwrap(), CaptureSourceStep::Published(report) if report.committed == 1 && report.remaining == 1)
    );
    assert_eq!(fixture.spool.pending().unwrap(), retained);
    assert_eq!(pipeline.partition().repository, B);
    fixture.catalog(&[A, B]);
    assert!(
        matches!(pipeline.publish_step(&retained[0], &mut fixture.context()).await.unwrap(), CaptureSourceStep::Published(report) if report.committed == 1)
    );
    assert_eq!(pipeline.partition().repository, B);
    assert!(fixture.spool.pending().unwrap().is_empty());
}

struct Unknown<P>(P);
#[async_trait::async_trait]
impl<P: AgentCapturePublisher> AgentCapturePublisher for Unknown<P> {
    async fn prepare_agent_capture(
        &self,
        request: RequestId,
        publication: PublicationId,
        profile: &NormalizerProfileV1,
        batch: &SanitizedObservationBatchV1,
        result: Vec<u8>,
    ) -> CaptureResult<AgentDeltaRequest> {
        self.0
            .prepare_agent_capture(request, publication, profile, batch, result)
            .await
    }
    async fn publish_agent_delta(
        &self,
        request: &AgentDeltaRequest,
    ) -> CaptureResult<PublishOutcome> {
        Ok(PublishOutcome::OutcomeUnknown {
            request_id: request.request_id,
            publication_id: request.publication_id,
        })
    }
}

#[tokio::test]
async fn resumed_pipeline_ingests_offline_tail_past_unknown_intent_without_rewinding_anchor() {
    let mut fixture = Fixture::new(4 * 1024 * 1024).await;
    let active = fixture.start();
    let sources = fixture.sources.clone();
    let privacy = fixture.privacy();
    let mut pipeline =
        CodexCaptureSource::acquire_active(sources, active, privacy, &mut fixture.context())
            .await
            .unwrap()
            .into_pipeline();
    pipeline.ingest_step(&mut fixture.context()).await.unwrap();
    let queue = fixture.spool.pending().unwrap();
    let proof = fixture
        .admission
        .inspect(fixture.workspace(A).to_str().unwrap())
        .await
        .unwrap();
    let publisher = Unknown(
        fixture
            .authority
            .publisher(&fixture.host, &fixture.admission, &proof),
    );
    assert!(
        drain_capture_spool_entry(&mut fixture.spool, &queue[0], &publisher)
            .await
            .unwrap()
            .outcome_unknown
    );
    let retained = fixture.spool.pending().unwrap();
    let PendingCaptureStateV1::Prepared { request } = &retained[0].state else {
        panic!("prepared intent required")
    };
    drop(pipeline);
    fixture.catalog(&[B]);
    fixture.append(json!({"type":"turn_context","payload":{"cwd":fixture.workspace(B)}}));
    fixture.append(json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"b"}}));
    fixture.message("offline", &format!("offline b {SECRET}"));
    let mut resume = CodexCaptureResume::prepare(
        fixture.sources.clone(),
        fixture.qualified(),
        fixture.privacy(),
        &fixture.spool,
    )
    .unwrap()
    .unwrap();
    let CaptureResumeStep::Ready(source) = resume.step(&mut fixture.context()).await.unwrap()
    else {
        panic!("current B must resume")
    };
    let mut pipeline = source.into_pipeline();
    assert_eq!(
        pipeline.ingest_step(&mut fixture.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let queue = fixture.spool.pending().unwrap();
    let b = queue.iter().find(|entry| entry.repository == B).unwrap();
    assert!(!serde_json::to_string(b).unwrap().contains(SECRET));
    let partition = pipeline.partition();
    let progress = fixture
        .spool
        .latest_reader_progress(partition.integration, partition.source)
        .unwrap();
    assert!(
        matches!(pipeline.publish_step(b, &mut fixture.context()).await.unwrap(), CaptureSourceStep::Published(report) if report.committed == 1)
    );
    assert_eq!(fixture.spool.pending().unwrap(), retained);
    fixture.catalog(&[A, B]);
    assert!(
        matches!(pipeline.publish_step(&retained[0], &mut fixture.context()).await.unwrap(), CaptureSourceStep::Published(report) if report.committed == 1)
    );
    assert_eq!(pipeline.partition().repository, B);
    assert_eq!(
        fixture
            .spool
            .latest_reader_progress(partition.integration, partition.source)
            .unwrap(),
        progress
    );
    assert!(matches!(
        fixture
            .host
            .endpoint(A)
            .unwrap()
            .bind_local_trusted(
                fixture.authority.principal(),
                jj_store_service::auth::Permission::Write
            )
            .unwrap()
            .publish_agent_delta(request)
            .await
            .unwrap(),
        PublishOutcome::Committed(_)
    ));
    fixture.append_bytes(b"invalid json\n");
    fixture.message("unknown", "private unknown body after old A publication");
    assert_eq!(
        pipeline.ingest_step(&mut fixture.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let gaps = fixture.spool.pending().unwrap();
    assert_eq!(gaps.len(), 1);
    assert_eq!(gaps[0].repository, B);
    assert!(
        !serde_json::to_string(&gaps)
            .unwrap()
            .contains("private unknown body")
    );
}

#[tokio::test]
async fn independent_ingress_preserves_same_partition_order_across_a_b_a() {
    let mut fixture = Fixture::new(4 * 1024 * 1024).await;
    let active = fixture.start();
    let sources = fixture.sources.clone();
    let privacy = fixture.privacy();
    let mut pipeline =
        CodexCaptureSource::acquire_active(sources, active, privacy, &mut fixture.context())
            .await
            .unwrap()
            .into_pipeline();
    pipeline.ingest_step(&mut fixture.context()).await.unwrap();
    let a = fixture.spool.pending().unwrap().remove(0);
    fixture.append(json!({"type":"turn_context","payload":{"cwd":fixture.workspace(B)}}));
    fixture.append(json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"b"}}));
    fixture.message("b", "b only");
    pipeline.ingest_step(&mut fixture.context()).await.unwrap();
    let b = fixture
        .spool
        .pending()
        .unwrap()
        .into_iter()
        .find(|entry| entry.repository == B)
        .unwrap();
    fixture.append(json!({"type":"turn_context","payload":{"cwd":fixture.workspace(A)}}));
    fixture.message("a", "later a");
    pipeline.ingest_step(&mut fixture.context()).await.unwrap();
    let before = fixture.spool.pending().unwrap();
    let later = before
        .iter()
        .find(|entry| entry.repository == A && entry.batch.batch_id != a.batch.batch_id)
        .unwrap();
    assert!(
        pipeline
            .publish_step(later, &mut fixture.context())
            .await
            .is_err()
    );
    assert_eq!(fixture.spool.pending().unwrap(), before);
    assert!(
        matches!(pipeline.publish_step(&b, &mut fixture.context()).await.unwrap(), CaptureSourceStep::Published(report) if report.committed == 1)
    );
    assert_eq!(pipeline.partition().repository, A);
    assert!(
        matches!(pipeline.publish_step(&a, &mut fixture.context()).await.unwrap(), CaptureSourceStep::Published(report) if report.committed == 1)
    );
    assert!(
        matches!(pipeline.publish_step(later, &mut fixture.context()).await.unwrap(), CaptureSourceStep::Published(report) if report.committed == 1)
    );
    assert!(fixture.spool.pending().unwrap().is_empty());
}

#[tokio::test]
async fn pipeline_backpressure_keeps_exact_batch_and_requires_current_write_and_same_spool() {
    let mut fixture = Fixture::new(4 * 1024 * 1024).await;
    let active = fixture.start();
    let sources = fixture.sources.clone();
    let privacy = fixture.privacy();
    let mut pipeline =
        CodexCaptureSource::acquire_active(sources, active, privacy, &mut fixture.context())
            .await
            .unwrap()
            .into_pipeline();
    pipeline.ingest_step(&mut fixture.context()).await.unwrap();
    let retained = fixture.spool.pending().unwrap();
    let partition = pipeline.partition();
    let original_progress = fixture
        .spool
        .latest_reader_progress(partition.integration, partition.source)
        .unwrap();
    let bytes = fs::read_dir(fixture.root.path().join("spool/pending"))
        .unwrap()
        .map(|entry| entry.unwrap().metadata().unwrap().len())
        .sum();
    fixture.spool = AgentCaptureSpool::open(&fixture.root.path().join("spool"), bytes).unwrap();
    fixture.append(json!({"type":"turn_context","payload":{"cwd":fixture.workspace(B)}}));
    fixture.message("first-b", "exact pending b");
    let ending = fs::metadata(&fixture.transcript).unwrap().len();
    assert!(pipeline.ingest_step(&mut fixture.context()).await.is_err());
    assert_eq!(fixture.spool.pending().unwrap(), retained);
    assert_eq!(
        fixture
            .spool
            .latest_reader_progress(partition.integration, partition.source)
            .unwrap(),
        original_progress
    );
    fixture.message("second-b", "later pending b");
    fixture.spool =
        AgentCaptureSpool::open(&fixture.root.path().join("spool"), 4 * 1024 * 1024).unwrap();
    assert_eq!(
        pipeline.ingest_step(&mut fixture.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let progress = fixture
        .spool
        .latest_reader_progress(partition.integration, partition.source)
        .unwrap()
        .unwrap();
    assert_eq!(
        serde_json::to_value(&progress.cursor.position).unwrap()["complete_byte_offset"],
        json!(ending)
    );
    let queue = fixture.spool.pending().unwrap();
    let b = queue.iter().find(|entry| entry.repository == B).unwrap();
    assert!(
        !serde_json::to_string(b)
            .unwrap()
            .contains("later pending b")
    );
    let replacement =
        AgentCaptureSpool::open(&fixture.root.path().join("other-spool"), 4 * 1024 * 1024).unwrap();
    let original = std::mem::replace(&mut fixture.spool, replacement);
    assert!(pipeline.ingest_step(&mut fixture.context()).await.is_err());
    assert!(
        pipeline
            .publish_step(b, &mut fixture.context())
            .await
            .is_err()
    );
    assert_eq!(original.pending().unwrap(), queue);
    fixture.spool = original;
    fixture.without_native_authority();
    assert!(pipeline.ingest_step(&mut fixture.context()).await.is_err());
    assert!(
        pipeline
            .publish_step(b, &mut fixture.context())
            .await
            .is_err()
    );
    assert_eq!(fixture.spool.pending().unwrap(), queue);
    assert_eq!(
        fixture
            .spool
            .latest_reader_progress(partition.integration, partition.source)
            .unwrap()
            .unwrap(),
        progress
    );
}
