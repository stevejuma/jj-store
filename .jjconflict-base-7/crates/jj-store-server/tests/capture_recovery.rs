#![cfg(unix)]
#[path = "capture_source/fixture.rs"]
mod fixture;
use fixture::*;
use jj_store_capture::{
    CaptureResult,
    agent_capture::PendingCaptureStateV1,
    agent_publication::{AgentCapturePublisher, drain_capture_spool_partition},
    codex_transcript::TranscriptPoll,
    transcript_sanitization::TranscriptSanitizer,
};
use jj_store_model::{PublicationId, RequestId};
use jj_store_server::capture_source::{CaptureSourceStep, CodexCaptureSource};
use jj_store_service::transport::{
    AgentDeltaRequest, NormalizerProfileV1, PublishOutcome, SanitizedObservationBatchV1,
};
use serde_json::json;
use std::fs;

#[tokio::test]
async fn multiple_queued_partitions_recover_in_source_order_before_new_reading() {
    let mut fixture = Fixture::new(4 * 1024 * 1024).await;
    let active = fixture.start();
    let sources = fixture.sources.clone();
    let privacy = fixture.privacy();
    let mut source =
        CodexCaptureSource::acquire_active(sources, active, privacy, &mut fixture.context())
            .await
            .unwrap();
    assert_eq!(
        source.step(&mut fixture.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let partition = source.partition();
    fixture.append(json!({"type":"turn_context","payload":{"cwd":fixture.workspace(B)}}));
    fixture.message("b", "queued b");
    fixture.append(json!({"type":"turn_context","payload":{"cwd":fixture.workspace(A)}}));
    fixture.message("a", "queued a again");
    let previous = fixture
        .spool
        .latest_reader_progress(partition.integration, partition.source)
        .unwrap();
    let mut reader = jj_store_capture::codex_transcript::CodexTranscriptReader::open(
        jj_store_capture::profile::CaptureProfile::CodexCliJsonlV1,
        &fixture.transcript,
        partition.source,
        previous,
    )
    .unwrap();
    for repository in [B, A] {
        let batch = loop {
            if let TranscriptPoll::Ready(batch) = reader.poll().unwrap() {
                break batch;
            }
        };
        assert_eq!(
            batch.directory(),
            Some(fixture.workspace(repository).as_path())
        );
        let privacy = fixture.privacy();
        let sanitizer = TranscriptSanitizer::new(
            &fixture.workspace(repository),
            &privacy.home,
            &privacy.temporary,
            &privacy.cache,
        )
        .unwrap();
        let ack = fixture
            .spool
            .enqueue_from_reader(
                batch
                    .sanitize(
                        repository,
                        &NormalizerProfileV1::codex_0_156_1(),
                        &sanitizer,
                    )
                    .unwrap(),
            )
            .unwrap();
        reader.acknowledge(&ack).unwrap();
    }
    let previous = fixture
        .spool
        .latest_reader_progress(partition.integration, partition.source)
        .unwrap();
    drop(reader);
    fixture.message("tail", "new read waits for queued recovery");
    for (repository, remaining) in [(A, 2), (B, 1), (A, 0)] {
        assert!(
            matches!(source.step(&mut fixture.context()).await.unwrap(), CaptureSourceStep::Published(report) if report.committed == 1 && report.remaining == remaining)
        );
        assert_eq!(source.partition().repository, repository);
        assert_eq!(
            fixture
                .spool
                .latest_reader_progress(partition.integration, partition.source)
                .unwrap(),
            previous
        );
    }
    assert_eq!(
        source.step(&mut fixture.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    assert_eq!(fixture.spool.pending().unwrap().len(), 1);
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
async fn queued_other_repository_recovers_exact_context_after_restart() {
    let mut fixture = Fixture::new(4 * 1024 * 1024).await;
    let active = fixture.start();
    let sources = fixture.sources.clone();
    let privacy = fixture.privacy();
    let mut source =
        CodexCaptureSource::acquire_active(sources, active, privacy, &mut fixture.context())
            .await
            .unwrap();
    assert_eq!(
        source.step(&mut fixture.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    assert!(
        matches!(source.step(&mut fixture.context()).await.unwrap(), CaptureSourceStep::Published(report) if report.committed == 1)
    );
    fixture.append(json!({"type":"turn_context","payload":{"cwd":fixture.workspace(B)}}));
    fixture.message("b", &format!("queued second repository {SECRET}"));
    assert_eq!(
        source.step(&mut fixture.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let proof = fixture
        .admission
        .inspect(fixture.workspace(B).to_str().unwrap())
        .await
        .unwrap();
    let publisher = Unknown(
        fixture
            .authority
            .publisher(&fixture.host, &fixture.admission, &proof),
    );
    let mut partition = source.partition();
    partition.repository = B;
    assert!(
        drain_capture_spool_partition(&mut fixture.spool, partition, &publisher, 1)
            .await
            .unwrap()
            .outcome_unknown
    );
    let retained = fixture.spool.pending().unwrap();
    assert_eq!(retained.len(), 1);
    assert_eq!(retained[0].repository, B);
    let PendingCaptureStateV1::Prepared { request } = &retained[0].state else {
        panic!("exact prepared intent required")
    };
    assert_eq!(request.principal, fixture.authority.principal());
    let payloads = retained[0]
        .batch
        .observations
        .iter()
        .map(|v| std::str::from_utf8(v.payload.bytes()).unwrap())
        .collect::<Vec<_>>()
        .join(" ");
    assert!(!payloads.contains(SECRET));
    drop(source);
    let sources = fixture.sources.clone();
    let qualified = fixture.qualified();
    let privacy = fixture.privacy();
    let mut resumed =
        CodexCaptureSource::acquire_admitted(sources, qualified, privacy, &mut fixture.context())
            .await
            .unwrap()
            .expect("previously admitted source");
    assert_eq!(fixture.spool.pending().unwrap(), retained);
    fixture.catalog(&[A]);
    for _ in 0..4 {
        let _ = resumed.step(&mut fixture.context()).await;
    }
    assert_eq!(fixture.spool.pending().unwrap(), retained);
    fixture.catalog(&[A, B]);
    let mut committed = false;
    for _ in 0..20 {
        match resumed.step(&mut fixture.context()).await.unwrap() {
            CaptureSourceStep::Published(report) if report.committed == 1 => {
                committed = true;
                break;
            }
            _ => {}
        }
    }
    assert!(committed);
    assert!(fixture.spool.pending().unwrap().is_empty());
    assert_eq!(resumed.partition().repository, B);
    assert!(matches!(
        fixture
            .host
            .endpoint(B)
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
}

#[tokio::test]
async fn omitted_pending_range_requires_its_separately_authorized_prior_anchor() {
    let mut fixture = Fixture::new(4 * 1024 * 1024).await;
    let active = fixture.start();
    let sources = fixture.sources.clone();
    let privacy = fixture.privacy();
    let mut source =
        CodexCaptureSource::acquire_active(sources, active, privacy, &mut fixture.context())
            .await
            .unwrap();
    source.step(&mut fixture.context()).await.unwrap();
    source.step(&mut fixture.context()).await.unwrap();
    fixture.append(json!({"type":"turn_context","payload":{"cwd":fixture.workspace(B)}}));
    fixture.message("b", "bound b");
    source.step(&mut fixture.context()).await.unwrap();
    source.step(&mut fixture.context()).await.unwrap();
    fixture.append_bytes(b"invalid json\n");
    fixture.message("private", "withheld unknown context");
    assert_eq!(
        source.step(&mut fixture.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let retained = fixture.spool.pending().unwrap();
    assert_eq!(retained[0].repository, B);
    drop(source);
    let sources = fixture.sources.clone();
    let privacy = fixture.privacy();
    let qualified = fixture.qualified();
    let mut source =
        CodexCaptureSource::acquire_admitted(sources, qualified, privacy, &mut fixture.context())
            .await
            .unwrap()
            .unwrap();
    fixture.catalog(&[A]);
    assert_eq!(
        source.step(&mut fixture.context()).await.unwrap(),
        CaptureSourceStep::Withheld
    );
    assert_eq!(fixture.spool.pending().unwrap(), retained);
    fixture.catalog(&[A, B]);
    assert!(
        matches!(source.step(&mut fixture.context()).await.unwrap(), CaptureSourceStep::Published(report) if report.committed == 1)
    );
    assert_eq!(source.partition().repository, B);
    assert!(fixture.spool.pending().unwrap().is_empty());
}

#[tokio::test]
async fn changed_pending_source_and_lost_native_write_preserve_exact_intent() {
    let mut fixture = Fixture::new(4 * 1024 * 1024).await;
    let active = fixture.start();
    let sources = fixture.sources.clone();
    let privacy = fixture.privacy();
    let mut source =
        CodexCaptureSource::acquire_active(sources, active, privacy, &mut fixture.context())
            .await
            .unwrap();
    source.step(&mut fixture.context()).await.unwrap();
    let retained = fixture.spool.pending().unwrap();
    drop(source);
    let sources = fixture.sources.clone();
    let privacy = fixture.privacy();
    let qualified = fixture.qualified();
    let mut source =
        CodexCaptureSource::acquire_admitted(sources, qualified, privacy, &mut fixture.context())
            .await
            .unwrap()
            .unwrap();
    let original = fs::read(&fixture.transcript).unwrap();
    let mut changed = original.clone();
    *changed.iter_mut().rfind(|v| **v == b's').unwrap() = b'x';
    fs::write(&fixture.transcript, changed).unwrap();
    assert!(source.step(&mut fixture.context()).await.is_err());
    assert_eq!(fixture.spool.pending().unwrap(), retained);
    fs::write(&fixture.transcript, &original).unwrap();
    let old = fixture.transcript.with_extension("old");
    fs::rename(&fixture.transcript, &old).unwrap();
    fs::write(&fixture.transcript, &original).unwrap();
    assert_eq!(
        source.step(&mut fixture.context()).await.unwrap(),
        CaptureSourceStep::Withheld
    );
    assert_eq!(fixture.spool.pending().unwrap(), retained);
    fs::remove_file(&fixture.transcript).unwrap();
    fs::rename(old, &fixture.transcript).unwrap();
    fixture.without_native_authority();
    assert!(source.step(&mut fixture.context()).await.is_err());
    assert_eq!(fixture.spool.pending().unwrap(), retained);
}
