use jj_agent::*;
use jj_store_cli::{
    agent_capture::{AgentCaptureSpool, PendingCaptureStateV1},
    local_agent_publication::{LocalAgentCapturePublisher, drain_local_capture_spool},
};
use jj_store_jj::operation_stores::AuthorityResult;
use jj_store_model::{PrincipalId, RepositoryId};
use jj_store_service::local_repository::*;
use std::sync::Mutex;

const REPO: RepositoryId = RepositoryId::new([1; 16]);
const ACTOR: PrincipalId = PrincipalId::new([2; 16]);
struct Publisher {
    replies: Mutex<std::collections::VecDeque<bool>>,
    requests: Mutex<Vec<LocalAgentRequest>>,
    mismatch: bool,
}
#[async_trait::async_trait]
impl LocalAgentCapturePublisher for Publisher {
    fn binding(&self) -> AuthorityResult<(RepositoryId, PrincipalId)> {
        Ok((REPO, ACTOR))
    }
    async fn apply(&self, request: &LocalAgentRequest) -> AuthorityResult<LocalAgentOutcome> {
        self.requests.lock().unwrap().push(request.clone());
        if !self.replies.lock().unwrap().pop_front().unwrap() {
            return Ok(LocalAgentOutcome::OutcomeUnknown {
                request_id: request.request_id,
            });
        }
        let LocalAgentCommand::Capture { batch, .. } = &request.command else {
            panic!("capture required")
        };
        Ok(LocalAgentOutcome::Committed {
            receipt: Box::new(LocalAgentReceipt {
                version: 2,
                profile: LocalProfile::LocalJjGit,
                repository: REPO,
                actor: ACTOR,
                request_id: request.request_id,
                intent: request.intent_for(ACTOR)?,
                metadata_sequence: 1,
                agent_state_version: 1,
                agent_root: jj_store_model::RecordDigest::new([3; 32]),
                accepted_at_millis: 1,
                batch: if self.mismatch {
                    None
                } else {
                    Some(batch.batch_id)
                },
            }),
        })
    }
}
fn enqueue(spool: &mut AgentCaptureSpool, id: u8, sequence: u64, repository: RepositoryId) {
    let source = SourceInstanceId::new([4; 16]);
    spool
        .enqueue(
            repository,
            NormalizerProfileV1::codex_0_156_1(),
            CaptureCursorV1 {
                schema_version: 1,
                integration: IntegrationId::Codex,
                source_instance_id: source,
                generation: 1,
                parser_version: "codex-hooks@0.156.1".into(),
                native_session_key: None,
                position: SourcePositionV1::NativeSequence { sequence },
            },
            SanitizedObservationBatchV1 {
                schema_version: 1,
                batch_id: CaptureBatchId::new([id; 16]),
                integration: IntegrationId::Codex,
                source_instance_id: source,
                sanitizer_version: "agent-redaction@0.1.0".into(),
                representation: SanitizedRepresentation::SanitizedNative,
                observations: vec![SanitizedObservationV1 {
                    observation_id: ObservationId::new([id; 16]),
                    source_sequence: sequence,
                    occurred_at_millis: None,
                    source_kind: SourceKind::Hook,
                    source_trust: SourceTrust::LocalUserHook,
                    payload: SanitizedPayloadV1::new(
                        SanitizedMediaType::Json,
                        br#"{"hook_event_name":"SessionStart","session_id":"recovery-session"}"#,
                    )
                    .unwrap(),
                }],
            },
        )
        .unwrap();
}
#[tokio::test]
async fn unknown_local_capture_restarts_with_exact_intent_before_later_batches() {
    let root = tempfile::tempdir().unwrap();
    let publisher = Publisher {
        replies: Mutex::new([false, true, true].into()),
        requests: Mutex::default(),
        mismatch: false,
    };
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    enqueue(&mut spool, 1, 20, REPO);
    enqueue(&mut spool, 2, 10, REPO);
    let report = drain_local_capture_spool(&mut spool, &publisher)
        .await
        .unwrap();
    assert!(report.outcome_unknown);
    assert_eq!(report.committed, 0);
    assert_eq!(report.remaining, 2);
    let prepared = spool
        .pending()
        .unwrap()
        .into_iter()
        .find(|e| e.batch.batch_id == CaptureBatchId::new([2; 16]))
        .unwrap();
    let PendingCaptureStateV1::LocalPrepared { request } = prepared.state else {
        panic!("durable exact local request required")
    };
    assert!(
        spool
            .complete_local(
                prepared.batch.batch_id,
                jj_store_model::RequestId::new([9; 16])
            )
            .is_err()
    );
    drop(spool);
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    let report = drain_local_capture_spool(&mut spool, &publisher)
        .await
        .unwrap();
    assert_eq!(report.committed, 2);
    assert_eq!(report.remaining, 0);
    assert!(!report.outcome_unknown);
    let requests = publisher.requests.lock().unwrap();
    assert_eq!(requests[0], request);
    assert_eq!(requests[1], request);
    assert_ne!(requests[1].request_id, requests[2].request_id);
}
#[tokio::test]
async fn mismatched_receipts_and_repository_binding_never_complete_local_captures() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    enqueue(&mut spool, 1, 1, REPO);
    let publisher = Publisher {
        replies: Mutex::new([true].into()),
        requests: Mutex::default(),
        mismatch: true,
    };
    assert!(
        drain_local_capture_spool(&mut spool, &publisher)
            .await
            .is_err()
    );
    assert!(matches!(
        spool.pending().unwrap()[0].state,
        PendingCaptureStateV1::LocalPrepared { .. }
    ));
    enqueue(&mut spool, 2, 2, RepositoryId::new([9; 16]));
    let count = publisher.requests.lock().unwrap().len();
    assert!(
        drain_local_capture_spool(&mut spool, &publisher)
            .await
            .is_err()
    );
    assert_eq!(publisher.requests.lock().unwrap().len(), count);
}
