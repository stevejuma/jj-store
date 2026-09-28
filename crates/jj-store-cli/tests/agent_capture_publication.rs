use jj_agent::{
    AgentCatalogEditV1, AgentCatalogKey, AgentCatalogKind, AgentDeltaV1, AgentObjectId,
    AgentObjectKind, AgentObjectRef, CaptureBatchId, CaptureCursorV1, IntegrationId,
    NormalizerProfileV1, ObservationId, SanitizedMediaType, SanitizedObservationBatchV1,
    SanitizedObservationV1, SanitizedPayloadV1, SanitizedRepresentation, SourceInstanceId,
    SourceKind, SourcePositionV1, SourceTrust,
};
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
use jj_store_wal::publication::{AgentDeltaRequest, PublishOutcome, Receipt, Snapshot};
use std::{
    collections::{BTreeSet, VecDeque},
    sync::Mutex,
};

const REPOSITORY: RepositoryId = RepositoryId::new([9; 16]);
const PRINCIPAL: PrincipalId = PrincipalId::new([4; 16]);

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
}

impl FakePublisher {
    fn new(replies: impl IntoIterator<Item = Reply>) -> Self {
        Self {
            replies: Mutex::new(replies.into_iter().collect()),
            prepared: Mutex::default(),
            published: Mutex::default(),
        }
    }
}

#[async_trait::async_trait]
impl AgentCapturePublisher for FakePublisher {
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
                edits: vec![AgentCatalogEditV1 {
                    catalog: AgentCatalogKind::Sessions,
                    key: AgentCatalogKey::derive("capture", batch.batch_id.as_bytes())?,
                    expected: None,
                    replacement: Some(AgentObjectRef::new(
                        AgentObjectKind::SessionSnapshot,
                        AgentObjectId::new([batch.batch_id.as_bytes()[0]; 32]),
                    )),
                }],
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
