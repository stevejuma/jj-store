use jj_agent::{
    AgentCatalogEditV1, AgentCatalogKey, AgentCatalogKind, AgentDeltaV1, AgentObjectId,
    AgentObjectKind, AgentObjectRef, CaptureBatchId, CaptureCursorV1, IntegrationId,
    NormalizerProfileV1, ObservationId, SanitizedMediaType, SanitizedObservationBatchV1,
    SanitizedObservationV1, SanitizedPayloadV1, SanitizedRepresentation, SourceInstanceId,
    SourceKind, SourcePositionV1, SourceTrust,
};
use jj_store_capture::agent_publication::{
    CaptureDrainPartition, CaptureDrainScheduler, drain_capture_spool_entry,
    drain_capture_spool_partition,
};
#[path = "agent_capture_publication/scheduling.rs"]
mod scheduling;
use jj_store_cli::{
    agent_capture::{AgentCaptureSpool, PendingCaptureStateV1},
    agent_publication::{AgentCapturePublisher, drain_capture_spool},
    agent_sanitization::KINGFISHER_POLICY,
};
use jj_store_jj::operation_stores::AuthorityResult;
use jj_store_model::{
    EpochId, IntentDigest, OperationId, PrincipalId, PublicationId, RepositoryId, RequestId,
    WalPosition,
};
use jj_store_service::local_repository::{LocalAgentCommand, LocalAgentRequest};
use jj_store_wal::publication::{AgentDeltaRequest, PublishOutcome, Receipt, Snapshot};
use std::{
    collections::{BTreeSet, VecDeque},
    sync::Mutex,
};

const REPOSITORY: RepositoryId = RepositoryId::new([9; 16]);
const PRINCIPAL: PrincipalId = PrincipalId::new([4; 16]);

#[tokio::test]
async fn exact_queued_entry_drain_preserves_other_batches_and_prepared_retry() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    enqueue(&mut spool, capture(1, 1, RepositoryId::new([10; 16])));
    enqueue(&mut spool, capture(2, 2, REPOSITORY));
    let queue = spool.pending().unwrap();
    let entry = queue
        .iter()
        .find(|entry| entry.batch.batch_id == CaptureBatchId::new([2; 16]))
        .unwrap();
    let publisher = FakePublisher::new([Reply::Unknown, Reply::Committed]);
    assert!(
        drain_capture_spool_entry(&mut spool, entry, &publisher)
            .await
            .unwrap()
            .outcome_unknown
    );
    let prepared = spool.pending().unwrap();
    let exact = prepared
        .iter()
        .find(|entry| entry.batch.batch_id == CaptureBatchId::new([2; 16]))
        .unwrap();
    let mut changed = exact.clone();
    changed.cursor.parser_version = "changed-parser".into();
    assert!(
        drain_capture_spool_entry(&mut spool, &changed, &publisher)
            .await
            .is_err()
    );
    assert_eq!(spool.pending().unwrap(), prepared);
    let report = drain_capture_spool_entry(&mut spool, exact, &publisher)
        .await
        .unwrap();
    assert_eq!(report.committed, 1);
    assert_eq!(report.remaining, 1);
    assert_eq!(
        *publisher.prepared.lock().unwrap(),
        vec![entry.batch.batch_id]
    );
    let published = publisher.published.lock().unwrap();
    assert_eq!(published.len(), 2);
    assert_eq!(published[0], published[1]);
    assert_eq!(
        spool.pending().unwrap(),
        vec![
            queue
                .iter()
                .find(|entry| entry.batch.batch_id == CaptureBatchId::new([1; 16]))
                .unwrap()
                .clone()
        ]
    );
}

#[tokio::test]
async fn exact_entry_cannot_bypass_an_earlier_same_partition_intent() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    enqueue(&mut spool, capture(2, 2, REPOSITORY));
    let selected = spool.pending().unwrap().remove(0);
    // Selection is only a hint: another handle can enqueue an earlier entry.
    let mut other = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    enqueue(&mut other, capture(1, 1, REPOSITORY));
    let before = spool.pending().unwrap();
    let publisher = FakePublisher::new([Reply::Committed]);
    assert!(
        drain_capture_spool_entry(&mut spool, &selected, &publisher)
            .await
            .is_err()
    );
    assert!(publisher.prepared.lock().unwrap().is_empty());
    assert!(publisher.published.lock().unwrap().is_empty());
    assert_eq!(spool.pending().unwrap(), before);
    let earlier = before
        .iter()
        .find(|entry| entry.batch.batch_id == CaptureBatchId::new([1; 16]))
        .unwrap();
    let unknown = FakePublisher::new([Reply::Unknown]);
    assert!(
        drain_capture_spool_entry(&mut spool, earlier, &unknown)
            .await
            .unwrap()
            .outcome_unknown
    );
    let prepared = spool.pending().unwrap();
    assert!(
        drain_capture_spool_entry(&mut spool, &selected, &publisher)
            .await
            .is_err()
    );
    assert!(publisher.prepared.lock().unwrap().is_empty());
    assert!(publisher.published.lock().unwrap().is_empty());
    assert_eq!(spool.pending().unwrap(), prepared);
}

#[derive(Clone, Copy)]
enum Reply {
    Committed,
    Unknown,
    Mismatched,
}

struct FakePublisher {
    replies: Mutex<VecDeque<Reply>>,
    prepared: Mutex<Vec<CaptureBatchId>>,
    published: Mutex<Vec<AgentDeltaRequest>>,
    drop_binding: bool,
    diagnostic_only: bool,
}

impl FakePublisher {
    fn new(replies: impl IntoIterator<Item = Reply>) -> Self {
        Self {
            replies: Mutex::new(replies.into_iter().collect()),
            prepared: Mutex::default(),
            published: Mutex::default(),
            drop_binding: false,
            diagnostic_only: false,
        }
    }
}

#[async_trait::async_trait]
impl AgentCapturePublisher for FakePublisher {
    async fn prepare_bound_agent_capture(
        &self,
        request_id: RequestId,
        publication_id: PublicationId,
        profile: &NormalizerProfileV1,
        batch: &SanitizedObservationBatchV1,
        result: Vec<u8>,
        binding: &jj_collab::ThreadBindingV1,
    ) -> AuthorityResult<AgentDeltaRequest> {
        let request = self
            .prepare_agent_capture(request_id, publication_id, profile, batch, result)
            .await?;
        if self.drop_binding {
            Ok(request)
        } else {
            Ok(request.with_thread_binding(binding.clone(), 123)?)
        }
    }
    async fn prepare_agent_capture(
        &self,
        request_id: RequestId,
        publication_id: PublicationId,
        _profile: &NormalizerProfileV1,
        batch: &SanitizedObservationBatchV1,
        result: Vec<u8>,
    ) -> AuthorityResult<AgentDeltaRequest> {
        self.prepared.lock().unwrap().push(batch.batch_id);
        Ok(AgentDeltaRequest::new(
            PRINCIPAL,
            request_id,
            publication_id,
            AgentDeltaV1 {
                schema_version: 1,
                edits: if self.diagnostic_only {
                    [
                        (AgentCatalogKind::Sources, AgentObjectKind::SourceState),
                        (
                            AgentCatalogKind::Projections,
                            AgentObjectKind::ProjectionCatalog,
                        ),
                    ]
                    .into_iter()
                    .enumerate()
                    .map(|(i, (catalog, kind))| {
                        Ok(AgentCatalogEditV1 {
                            catalog,
                            key: AgentCatalogKey::derive(
                                "capture-gap",
                                &[batch.batch_id.as_bytes()[0], i as u8],
                            )?,
                            expected: None,
                            replacement: Some(AgentObjectRef::new(
                                kind,
                                AgentObjectId::new([i as u8 + 1; 32]),
                            )),
                        })
                    })
                    .collect::<jj_agent::Result<Vec<_>>>()?
                } else {
                    vec![AgentCatalogEditV1 {
                        catalog: AgentCatalogKind::Sessions,
                        key: AgentCatalogKey::derive("capture", batch.batch_id.as_bytes())?,
                        expected: None,
                        replacement: Some(AgentObjectRef::new(
                            AgentObjectKind::SessionSnapshot,
                            AgentObjectId::new([batch.batch_id.as_bytes()[0]; 32]),
                        )),
                    }]
                },
            },
            result,
        )?)
    }

    async fn publish_agent_delta(
        &self,
        request: &AgentDeltaRequest,
    ) -> AuthorityResult<PublishOutcome> {
        self.published.lock().unwrap().push(request.clone());
        Ok(match self.replies.lock().unwrap().pop_front().unwrap() {
            Reply::Committed => PublishOutcome::Committed(receipt(request, request.result.clone())),
            Reply::Unknown => PublishOutcome::OutcomeUnknown {
                request_id: request.request_id,
                publication_id: request.publication_id,
            },
            Reply::Mismatched => PublishOutcome::Committed(receipt(request, b"wrong".to_vec())),
        })
    }
}

#[tokio::test]
async fn bound_capture_journals_before_prepare_and_retries_exact_assignment_after_restart() {
    bound_spool_acceptance(false).await;
}
#[tokio::test]
async fn bound_diagnostic_gap_drains_after_restart_without_a_session_edit() {
    bound_spool_acceptance(true).await;
}
async fn bound_spool_acceptance(diagnostic: bool) {
    use jj_collab::{CollabObjectId, CollabObjectKind, CollabObjectRef, ThreadBindingV1, ThreadId};
    let directory = tempfile::tempdir().unwrap();
    let mut pending = capture(7, 1, REPOSITORY);
    if diagnostic {
        pending.batch.observations[0].payload =
            SanitizedPayloadV1::new(SanitizedMediaType::Json, br#"{"future_record":true}"#)
                .unwrap();
    }
    let binding = ThreadBindingV1 {
        thread_id: ThreadId::new([10; 16]),
        observed_thread: CollabObjectRef {
            kind: CollabObjectKind::ThreadVersion,
            id: CollabObjectId::new([11; 32]),
        },
        observed_assignment: None,
    };
    let mut spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    spool
        .enqueue_with_binding(
            pending.repository,
            pending.profile.clone(),
            pending.cursor.clone(),
            pending.batch.clone(),
            Some(binding.clone()),
        )
        .unwrap();
    assert!(
        spool
            .enqueue(
                pending.repository,
                pending.profile.clone(),
                pending.cursor.clone(),
                pending.batch.clone()
            )
            .is_err()
    );
    drop(spool);
    let mut publisher = FakePublisher::new([Reply::Unknown, Reply::Committed]);
    publisher.diagnostic_only = diagnostic;
    let mut spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    assert_eq!(
        spool.pending().unwrap()[0].thread_binding,
        Some(binding.clone())
    );
    let mut dropped = FakePublisher::new([]);
    dropped.drop_binding = true;
    assert!(
        drain_capture_spool(&mut spool, REPOSITORY, &dropped)
            .await
            .is_err()
    );
    assert!(dropped.published.lock().unwrap().is_empty());
    assert!(matches!(
        spool.pending().unwrap()[0].state,
        PendingCaptureStateV1::Captured
    ));
    assert!(
        drain_capture_spool(&mut spool, REPOSITORY, &publisher)
            .await
            .unwrap()
            .outcome_unknown
    );
    drop(spool);
    let mut spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    let report = drain_capture_spool(&mut spool, REPOSITORY, &publisher)
        .await
        .unwrap();
    assert_eq!(report.committed, 1);
    assert!(spool.pending().unwrap().is_empty());
    assert_eq!(publisher.prepared.lock().unwrap().len(), 1);
    let requests = publisher.published.lock().unwrap();
    assert_eq!(requests[0], requests[1]);
    assert_eq!(
        requests[0]
            .delta
            .edits
            .iter()
            .filter(|e| e.catalog == AgentCatalogKind::Sessions)
            .count(),
        usize::from(!diagnostic)
    );
    assert_eq!(
        requests[0].thread_binding.as_ref().unwrap().binding,
        binding
    );
    assert_eq!(
        requests[0]
            .thread_binding
            .as_ref()
            .unwrap()
            .accepted_at_millis,
        123
    );
}

fn capture(batch: u8, sequence: u64, repository: RepositoryId) -> PendingCapture {
    let source = SourceInstanceId::new([2; 16]);
    let batch = SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: CaptureBatchId::new([batch; 16]),
        integration: IntegrationId::Codex,
        source_instance_id: source,
        sanitizer_version: KINGFISHER_POLICY.into(),
        representation: SanitizedRepresentation::SanitizedNative,
        observations: vec![SanitizedObservationV1 {
            observation_id: ObservationId::new([batch; 16]),
            source_sequence: sequence,
            occurred_at_millis: None,
            source_kind: SourceKind::Hook,
            source_trust: SourceTrust::LocalUserHook,
            payload: SanitizedPayloadV1::new(
                SanitizedMediaType::Json,
                br#"{"event":"SessionStart","session_id":"session-1"}"#,
            )
            .unwrap(),
        }],
    };
    PendingCapture {
        repository,
        profile: NormalizerProfileV1::codex_0_156_1(),
        cursor: CaptureCursorV1 {
            schema_version: 1,
            integration: IntegrationId::Codex,
            source_instance_id: source,
            generation: 1,
            parser_version: "codex-hooks@0.156.1".into(),
            native_session_key: None,
            position: SourcePositionV1::NativeSequence { sequence },
        },
        batch,
    }
}

struct PendingCapture {
    repository: RepositoryId,
    profile: NormalizerProfileV1,
    cursor: CaptureCursorV1,
    batch: SanitizedObservationBatchV1,
}

fn enqueue(spool: &mut AgentCaptureSpool, capture: PendingCapture) {
    spool
        .enqueue(
            capture.repository,
            capture.profile,
            capture.cursor,
            capture.batch,
        )
        .unwrap();
}

fn receipt(request: &AgentDeltaRequest, result: Vec<u8>) -> Receipt {
    Receipt {
        principal: request.principal,
        request_id: request.request_id,
        publication_id: request.publication_id,
        intent: IntentDigest::new([12; 32]),
        accepted: Snapshot {
            epoch: EpochId::new([13; 16]),
            position: WalPosition(14),
            code_state_version: 0,
            heads: BTreeSet::from([OperationId::new([15; 64])]),
        },
        result,
    }
}

#[tokio::test]
async fn bounded_partition_drain_retains_other_repositories_sources_and_unknown_intent() {
    let directory = tempfile::tempdir().unwrap();
    let other = RepositoryId::new([10; 16]);
    let mut spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    enqueue(&mut spool, capture(2, 2, REPOSITORY));
    enqueue(&mut spool, capture(1, 1, REPOSITORY));
    enqueue(&mut spool, capture(3, 3, other));
    let mut independent = capture(4, 1, REPOSITORY);
    independent.batch.source_instance_id = SourceInstanceId::new([3; 16]);
    independent.cursor.source_instance_id = independent.batch.source_instance_id;
    enqueue(&mut spool, independent);
    let partition = CaptureDrainPartition {
        repository: REPOSITORY,
        integration: IntegrationId::Codex,
        source: SourceInstanceId::new([2; 16]),
    };
    let publisher = FakePublisher::new([Reply::Unknown, Reply::Committed, Reply::Committed]);
    assert!(
        drain_capture_spool(&mut spool, REPOSITORY, &publisher)
            .await
            .is_err()
    );
    assert!(publisher.prepared.lock().unwrap().is_empty());
    let report = drain_capture_spool_partition(&mut spool, partition, &publisher, 1)
        .await
        .unwrap();
    assert!(report.outcome_unknown);
    assert_eq!((report.committed, report.remaining), (0, 4));
    let retained = spool
        .pending()
        .unwrap()
        .into_iter()
        .find(|e| e.batch.batch_id == CaptureBatchId::new([1; 16]))
        .unwrap();
    assert!(matches!(
        retained.state,
        PendingCaptureStateV1::Prepared { .. }
    ));
    let other_publisher = FakePublisher::new([Reply::Committed]);
    let other_report = drain_capture_spool_partition(
        &mut spool,
        CaptureDrainPartition {
            repository: other,
            ..partition
        },
        &other_publisher,
        1,
    )
    .await
    .unwrap();
    assert_eq!((other_report.committed, other_report.remaining), (1, 3));
    drop(spool);
    let mut spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    let report = drain_capture_spool_partition(&mut spool, partition, &publisher, 1)
        .await
        .unwrap();
    assert_eq!((report.committed, report.remaining), (1, 2));
    assert!(!report.outcome_unknown);
    {
        let published = publisher.published.lock().unwrap();
        assert_eq!(published[0], published[1]);
    }
    assert_eq!(publisher.prepared.lock().unwrap().len(), 1);
    let report = drain_capture_spool_partition(&mut spool, partition, &publisher, 1)
        .await
        .unwrap();
    assert_eq!((report.committed, report.remaining), (1, 1));
    assert_eq!(
        spool.pending().unwrap()[0].batch.batch_id,
        CaptureBatchId::new([4; 16])
    );
    let empty = FakePublisher::new([]);
    let report = drain_capture_spool_partition(&mut spool, partition, &empty, 1)
        .await
        .unwrap();
    assert_eq!((report.committed, report.remaining), (0, 1));
    assert!(empty.prepared.lock().unwrap().is_empty());
    for bound in [0, 257] {
        assert!(
            drain_capture_spool_partition(&mut spool, partition, &empty, bound)
                .await
                .is_err()
        );
    }
}

#[tokio::test]
async fn partition_preserves_foreign_local_intent_and_rejects_selected_local_before_preparation() {
    let directory = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    enqueue(&mut spool, capture(1, 1, REPOSITORY));
    let foreign = capture(2, 2, RepositoryId::new([10; 16]));
    let local = LocalAgentRequest {
        version: 1,
        request_id: RequestId::new([11; 16]),
        command: LocalAgentCommand::Capture {
            profile: foreign.profile.clone(),
            batch: Box::new(foreign.batch.clone()),
        },
    };
    enqueue(&mut spool, foreign);
    spool
        .prepare_local(CaptureBatchId::new([2; 16]), local.clone())
        .unwrap();
    let partition = CaptureDrainPartition {
        repository: REPOSITORY,
        integration: IntegrationId::Codex,
        source: SourceInstanceId::new([2; 16]),
    };
    let publisher = FakePublisher::new([Reply::Committed]);
    let report = drain_capture_spool_partition(&mut spool, partition, &publisher, 1)
        .await
        .unwrap();
    assert_eq!((report.committed, report.remaining), (1, 1));
    assert_eq!(
        spool.pending().unwrap()[0].state,
        PendingCaptureStateV1::LocalPrepared { request: local }
    );

    enqueue(&mut spool, capture(3, 3, REPOSITORY));
    let selected = capture(4, 4, REPOSITORY);
    let local = LocalAgentRequest {
        version: 1,
        request_id: RequestId::new([12; 16]),
        command: LocalAgentCommand::Capture {
            profile: selected.profile.clone(),
            batch: Box::new(selected.batch.clone()),
        },
    };
    enqueue(&mut spool, selected);
    spool
        .prepare_local(CaptureBatchId::new([4; 16]), local)
        .unwrap();
    let before = spool.pending().unwrap();
    let publisher = FakePublisher::new([]);
    // Even a later local entry outside this call's bound blocks the selected partition.
    assert!(
        drain_capture_spool_partition(&mut spool, partition, &publisher, 1)
            .await
            .is_err()
    );
    assert!(publisher.prepared.lock().unwrap().is_empty());
    assert!(publisher.published.lock().unwrap().is_empty());
    assert_eq!(spool.pending().unwrap(), before);

    for bound in [0, 257] {
        let error = drain_capture_spool_partition(&mut spool, partition, &publisher, bound)
            .await
            .unwrap_err();
        assert!(error.to_string().contains("batch bound"));
    }
}

#[tokio::test]
async fn partition_mismatched_receipt_retains_exact_intent_without_touching_other_source() {
    let directory = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    enqueue(&mut spool, capture(1, 1, REPOSITORY));
    let mut foreign = capture(2, 1, REPOSITORY);
    foreign.batch.source_instance_id = SourceInstanceId::new([3; 16]);
    foreign.cursor.source_instance_id = foreign.batch.source_instance_id;
    enqueue(&mut spool, foreign);
    let partition = CaptureDrainPartition {
        repository: REPOSITORY,
        integration: IntegrationId::Codex,
        source: SourceInstanceId::new([2; 16]),
    };
    let publisher = FakePublisher::new([Reply::Mismatched, Reply::Committed]);
    assert!(
        drain_capture_spool_partition(&mut spool, partition, &publisher, 1)
            .await
            .is_err()
    );
    let retained = spool
        .pending()
        .unwrap()
        .into_iter()
        .find(|e| e.batch.batch_id == CaptureBatchId::new([1; 16]))
        .unwrap();
    let PendingCaptureStateV1::Prepared { request } = retained.state else {
        panic!("durable request required")
    };
    drop(spool);
    let mut spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    let report = drain_capture_spool_partition(&mut spool, partition, &publisher, 1)
        .await
        .unwrap();
    assert_eq!((report.committed, report.remaining), (1, 1));
    assert_eq!(publisher.published.lock().unwrap()[1], request);
    assert_eq!(publisher.prepared.lock().unwrap().len(), 1);
    assert_eq!(
        spool.pending().unwrap()[0].state,
        PendingCaptureStateV1::Captured
    );
}

#[tokio::test]
async fn captured_entries_publish_in_source_sequence_order() {
    let directory = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    enqueue(&mut spool, capture(1, 20, REPOSITORY));
    enqueue(&mut spool, capture(2, 10, REPOSITORY));
    let publisher = FakePublisher::new([Reply::Committed, Reply::Committed]);

    let report = drain_capture_spool(&mut spool, REPOSITORY, &publisher)
        .await
        .unwrap();

    assert_eq!(report.committed, 2);
    assert_eq!(report.remaining, 0);
    assert!(!report.outcome_unknown);
    assert_eq!(
        *publisher.prepared.lock().unwrap(),
        vec![CaptureBatchId::new([2; 16]), CaptureBatchId::new([1; 16])]
    );
    assert!(spool.pending().unwrap().is_empty());
}

#[tokio::test]
async fn unknown_publication_survives_restart_and_retries_exact_request() {
    let directory = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    enqueue(&mut spool, capture(3, 1, REPOSITORY));
    let publisher = FakePublisher::new([Reply::Unknown, Reply::Committed]);

    let report = drain_capture_spool(&mut spool, REPOSITORY, &publisher)
        .await
        .unwrap();
    assert_eq!((report.committed, report.remaining), (0, 1));
    assert!(report.outcome_unknown);
    let prepared = spool.pending().unwrap()[0].clone();
    assert!(matches!(
        prepared.state,
        PendingCaptureStateV1::Prepared { .. }
    ));
    drop(spool);

    let mut spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    drain_capture_spool(&mut spool, REPOSITORY, &publisher)
        .await
        .unwrap();
    assert_eq!(publisher.prepared.lock().unwrap().len(), 1);
    let published = publisher.published.lock().unwrap();
    assert_eq!(published.len(), 2);
    assert_eq!(published[0], published[1]);
    assert!(spool.pending().unwrap().is_empty());
}

#[tokio::test]
async fn mismatched_commit_never_completes_prepared_capture() {
    let directory = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    enqueue(&mut spool, capture(4, 1, REPOSITORY));

    assert!(
        drain_capture_spool(
            &mut spool,
            REPOSITORY,
            &FakePublisher::new([Reply::Mismatched])
        )
        .await
        .is_err()
    );
    assert!(matches!(
        spool.pending().unwrap()[0].state,
        PendingCaptureStateV1::Prepared { .. }
    ));
}

#[tokio::test]
async fn repository_mismatch_fails_before_any_publication() {
    let directory = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    enqueue(&mut spool, capture(5, 1, REPOSITORY));
    enqueue(&mut spool, capture(6, 2, RepositoryId::new([8; 16])));
    let publisher = FakePublisher::new([]);

    assert!(
        drain_capture_spool(&mut spool, REPOSITORY, &publisher)
            .await
            .is_err()
    );
    assert!(publisher.prepared.lock().unwrap().is_empty());
    assert_eq!(spool.pending().unwrap().len(), 2);
}

struct PausedPublisher {
    inner: FakePublisher,
    preparing: tokio::sync::Notify,
    prepare_ready: tokio::sync::Notify,
    publishing: tokio::sync::Notify,
}

#[async_trait::async_trait]
impl AgentCapturePublisher for PausedPublisher {
    async fn prepare_agent_capture(
        &self,
        request_id: RequestId,
        publication_id: PublicationId,
        profile: &NormalizerProfileV1,
        batch: &SanitizedObservationBatchV1,
        result: Vec<u8>,
    ) -> AuthorityResult<AgentDeltaRequest> {
        self.preparing.notify_one();
        self.prepare_ready.notified().await;
        self.inner
            .prepare_agent_capture(request_id, publication_id, profile, batch, result)
            .await
    }

    async fn publish_agent_delta(
        &self,
        request: &AgentDeltaRequest,
    ) -> AuthorityResult<PublishOutcome> {
        self.inner.published.lock().unwrap().push(request.clone());
        self.publishing.notify_one();
        std::future::pending().await
    }
}

#[test]
#[ignore = "subprocess fixture invoked by publication_wait_allows_capture_from_another_process"]
fn enqueue_from_other_process() {
    let path = std::env::var_os("JJ_STORE_CAPTURE_TEST_SPOOL").expect("test spool");
    let id: u8 = std::env::var("JJ_STORE_CAPTURE_TEST_BATCH")
        .unwrap()
        .parse()
        .unwrap();
    let mut spool = AgentCaptureSpool::open(std::path::Path::new(&path), 4 * 1024 * 1024)
        .expect("publication must not hold the enqueue lock");
    enqueue(&mut spool, capture(id, u64::from(id), REPOSITORY));
}

async fn enqueue_child(path: &std::path::Path, id: u8) {
    let output = tokio::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "enqueue_from_other_process", "--ignored"])
        .env("JJ_STORE_CAPTURE_TEST_SPOOL", path)
        .env("JJ_STORE_CAPTURE_TEST_BATCH", id.to_string())
        .kill_on_drop(true)
        .output();
    let output = tokio::time::timeout(std::time::Duration::from_secs(10), output)
        .await
        .expect("enqueue child must finish without waiting for publication")
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
}

#[tokio::test]
async fn publication_wait_allows_capture_from_another_process() {
    let directory = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    enqueue(&mut spool, capture(1, 1, REPOSITORY));
    let publisher = std::sync::Arc::new(PausedPublisher {
        inner: FakePublisher::new([]),
        preparing: tokio::sync::Notify::new(),
        prepare_ready: tokio::sync::Notify::new(),
        publishing: tokio::sync::Notify::new(),
    });
    let worker = {
        let publisher = publisher.clone();
        tokio::spawn(async move {
            drain_capture_spool(&mut spool, REPOSITORY, publisher.as_ref()).await
        })
    };
    tokio::time::timeout(
        std::time::Duration::from_secs(10),
        publisher.preparing.notified(),
    )
    .await
    .unwrap();
    enqueue_child(directory.path(), 2).await;
    let mut observer = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    assert_eq!(observer.pending().unwrap().len(), 2);
    let competing = FakePublisher::new([]);
    assert!(
        drain_capture_spool(&mut observer, REPOSITORY, &competing)
            .await
            .is_err()
    );
    assert!(competing.prepared.lock().unwrap().is_empty());
    assert!(competing.published.lock().unwrap().is_empty());

    publisher.prepare_ready.notify_one();
    tokio::time::timeout(
        std::time::Duration::from_secs(10),
        publisher.publishing.notified(),
    )
    .await
    .unwrap();
    enqueue_child(directory.path(), 3).await;
    let prepared = observer
        .pending()
        .unwrap()
        .into_iter()
        .find(|e| e.batch.batch_id == CaptureBatchId::new([1; 16]))
        .unwrap();
    let PendingCaptureStateV1::Prepared { request } = prepared.state else {
        panic!("publication must use durable prepared intent")
    };
    worker.abort();
    assert!(worker.await.unwrap_err().is_cancelled());
    drop(observer);

    // A cold handle retries the exact request after cancellation released the lease.
    let mut restarted = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    let recovery = FakePublisher::new([Reply::Committed, Reply::Committed, Reply::Committed]);
    let report = drain_capture_spool(&mut restarted, REPOSITORY, &recovery)
        .await
        .unwrap();
    assert_eq!((report.committed, report.remaining), (3, 0));
    assert_eq!(recovery.published.lock().unwrap()[0], request);
    assert_eq!(
        *recovery.prepared.lock().unwrap(),
        vec![CaptureBatchId::new([2; 16]), CaptureBatchId::new([3; 16])]
    );
}

struct ProcessPausedPublisher {
    signal: std::path::PathBuf,
}
#[async_trait::async_trait]
impl AgentCapturePublisher for ProcessPausedPublisher {
    async fn prepare_agent_capture(
        &self,
        request_id: RequestId,
        publication_id: PublicationId,
        profile: &NormalizerProfileV1,
        batch: &SanitizedObservationBatchV1,
        result: Vec<u8>,
    ) -> AuthorityResult<AgentDeltaRequest> {
        FakePublisher::new([])
            .prepare_agent_capture(request_id, publication_id, profile, batch, result)
            .await
    }
    async fn publish_agent_delta(&self, _: &AgentDeltaRequest) -> AuthorityResult<PublishOutcome> {
        std::fs::write(&self.signal, b"prepared")?;
        std::future::pending().await
    }
}

#[tokio::test]
#[ignore = "subprocess fixture invoked by killed_drainer_preserves_prepared_capture_and_releases_lease"]
async fn hold_drain_in_other_process() {
    let path = std::path::PathBuf::from(
        std::env::var_os("JJ_STORE_CAPTURE_TEST_SPOOL").expect("test spool"),
    );
    let mut spool = AgentCaptureSpool::open(&path, 4 * 1024 * 1024).unwrap();
    drain_capture_spool(
        &mut spool,
        REPOSITORY,
        &ProcessPausedPublisher {
            signal: path.join("test-publishing"),
        },
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn killed_drainer_preserves_prepared_capture_and_releases_lease() {
    let directory = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    enqueue(&mut spool, capture(1, 1, REPOSITORY));
    drop(spool);
    let mut child = tokio::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "hold_drain_in_other_process", "--ignored"])
        .env("JJ_STORE_CAPTURE_TEST_SPOOL", directory.path())
        .stdout(std::process::Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(10), async {
        while !directory.path().join("test-publishing").exists() {
            assert!(
                child.try_wait().unwrap().is_none(),
                "drainer exited before publication"
            );
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }
    })
    .await
    .unwrap();
    enqueue_child(directory.path(), 2).await;
    let mut observer = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    assert!(
        drain_capture_spool(&mut observer, REPOSITORY, &FakePublisher::new([]))
            .await
            .is_err()
    );
    let entry = observer
        .pending()
        .unwrap()
        .into_iter()
        .find(|e| e.batch.batch_id == CaptureBatchId::new([1; 16]))
        .unwrap();
    let PendingCaptureStateV1::Prepared { request } = entry.state else {
        panic!("prepared request required")
    };
    drop(observer);
    child.kill().await.unwrap();
    assert!(!child.wait().await.unwrap().success());

    let mut restarted = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    let publisher = FakePublisher::new([Reply::Committed, Reply::Committed]);
    let report = drain_capture_spool(&mut restarted, REPOSITORY, &publisher)
        .await
        .unwrap();
    assert_eq!((report.committed, report.remaining), (2, 0));
    assert_eq!(publisher.published.lock().unwrap()[0], request);
    assert_eq!(
        *publisher.prepared.lock().unwrap(),
        vec![CaptureBatchId::new([2; 16])]
    );
}

#[tokio::test]
async fn cancelled_partition_releases_drain_lease_and_retries_exact_prepared_request() {
    let directory = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    enqueue(&mut spool, capture(1, 1, REPOSITORY));
    enqueue(&mut spool, capture(2, 2, RepositoryId::new([10; 16])));
    let partition = CaptureDrainPartition {
        repository: REPOSITORY,
        integration: IntegrationId::Codex,
        source: SourceInstanceId::new([2; 16]),
    };
    let publisher = std::sync::Arc::new(PausedPublisher {
        inner: FakePublisher::new([]),
        preparing: tokio::sync::Notify::new(),
        prepare_ready: tokio::sync::Notify::new(),
        publishing: tokio::sync::Notify::new(),
    });
    let worker = {
        let publisher = publisher.clone();
        tokio::spawn(async move {
            drain_capture_spool_partition(&mut spool, partition, publisher.as_ref(), 1).await
        })
    };
    tokio::time::timeout(
        std::time::Duration::from_secs(10),
        publisher.preparing.notified(),
    )
    .await
    .unwrap();
    publisher.prepare_ready.notify_one();
    tokio::time::timeout(
        std::time::Duration::from_secs(10),
        publisher.publishing.notified(),
    )
    .await
    .unwrap();
    let mut observer = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    let entry = observer
        .pending()
        .unwrap()
        .into_iter()
        .find(|e| e.batch.batch_id == CaptureBatchId::new([1; 16]))
        .unwrap();
    let PendingCaptureStateV1::Prepared { request } = entry.state else {
        panic!("durable request required")
    };
    assert!(
        drain_capture_spool_partition(&mut observer, partition, &FakePublisher::new([]), 1)
            .await
            .is_err()
    );
    for bound in [0, 257] {
        let error =
            drain_capture_spool_partition(&mut observer, partition, &FakePublisher::new([]), bound)
                .await
                .unwrap_err();
        assert!(error.to_string().contains("batch bound"));
    }
    worker.abort();
    assert!(worker.await.unwrap_err().is_cancelled());
    drop(observer);
    let mut restarted = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    let recovery = FakePublisher::new([Reply::Committed]);
    let report = drain_capture_spool_partition(&mut restarted, partition, &recovery, 1)
        .await
        .unwrap();
    assert_eq!((report.committed, report.remaining), (1, 1));
    assert_eq!(recovery.published.lock().unwrap()[0], request);
    assert!(recovery.prepared.lock().unwrap().is_empty());
    assert_eq!(
        restarted.pending().unwrap()[0].state,
        PendingCaptureStateV1::Captured
    );
}
