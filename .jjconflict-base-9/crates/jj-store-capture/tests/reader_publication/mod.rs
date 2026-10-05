use jj_agent::*;
use jj_store_capture::{
    CaptureResult,
    agent_publication::AgentCapturePublisher,
    reader_progress::{ReaderCapture, ReaderCaptureMetadataV1},
};
use jj_store_model::{
    EpochId, IntentDigest, PrincipalId, PublicationId, RepositoryId, RequestId, WalPosition,
};
use jj_store_wal::publication::{AgentDeltaRequest, PublishOutcome, Receipt, Snapshot};
pub const REPO: RepositoryId = RepositoryId::new([1; 16]);
pub const SOURCE: SourceInstanceId = SourceInstanceId::new([2; 16]);
pub fn input(sequence: u64, repository: RepositoryId) -> ReaderCapture {
    ReaderCapture {
        repository,
        profile: NormalizerProfileV1::codex_0_156_1(),
        cursor: CaptureCursorV1 {
            schema_version: 1,
            integration: IntegrationId::Codex,
            source_instance_id: SOURCE,
            generation: 1,
            parser_version: "codex-jsonl@0.156.1/v1".into(),
            native_session_key: Some("session".into()),
            position: SourcePositionV1::JsonLines {
                complete_byte_offset: sequence * 100,
                prefix_sha256: [sequence as u8; 32],
            },
        },
        batch: SanitizedObservationBatchV1 {
            schema_version: 1,
            batch_id: CaptureBatchId::new([sequence as u8; 16]),
            integration: IntegrationId::Codex,
            source_instance_id: SOURCE,
            sanitizer_version: jj_store_capture::agent_sanitization::KINGFISHER_POLICY.into(),
            representation: SanitizedRepresentation::SanitizedNative,
            observations: vec![SanitizedObservationV1 {
                observation_id: ObservationId::new([sequence as u8; 16]),
                source_sequence: sequence,
                occurred_at_millis: Some(1),
                source_kind: SourceKind::Transcript,
                source_trust: SourceTrust::LocalTranscript,
                payload: SanitizedPayloadV1::new(
                    SanitizedMediaType::Json,
                    br#"{"status":"omitted"}"#,
                )
                .unwrap(),
            }],
        },
        metadata: ReaderCaptureMetadataV1 {
            schema_version: 1,
            generation_digest: [1; 32],
            opencode_snapshot: None,
        },
        thread_binding: None,
    }
}

pub struct Publisher;
pub struct UnknownPublisher;
pub struct MismatchedPublisher;
#[async_trait::async_trait]
impl AgentCapturePublisher for Publisher {
    async fn prepare_agent_capture(
        &self,
        request_id: RequestId,
        publication_id: PublicationId,
        _profile: &NormalizerProfileV1,
        _batch: &SanitizedObservationBatchV1,
        result: Vec<u8>,
    ) -> CaptureResult<AgentDeltaRequest> {
        Ok(AgentDeltaRequest::new(
            PrincipalId::new([3; 16]),
            request_id,
            publication_id,
            AgentDeltaV1 {
                schema_version: 1,
                edits: vec![AgentCatalogEditV1 {
                    catalog: AgentCatalogKind::Sessions,
                    key: AgentCatalogKey::derive("reader-test", b"session")?,
                    expected: None,
                    replacement: Some(AgentObjectRef::new(
                        AgentObjectKind::SessionSnapshot,
                        AgentObjectId::new([6; 32]),
                    )),
                }],
            },
            result,
        )?)
    }
    async fn publish_agent_delta(
        &self,
        request: &AgentDeltaRequest,
    ) -> CaptureResult<PublishOutcome> {
        Ok(PublishOutcome::Committed(receipt(request)))
    }
}
pub fn receipt(request: &AgentDeltaRequest) -> Receipt {
    Receipt {
        principal: request.principal,
        request_id: request.request_id,
        publication_id: request.publication_id,
        intent: IntentDigest::new([8; 32]),
        accepted: Snapshot {
            epoch: EpochId::new([7; 16]),
            position: WalPosition(1),
            code_state_version: 0,
            heads: Default::default(),
        },
        result: request.result.clone(),
    }
}

pub fn ready<T>(future: impl std::future::Future<Output = T>) -> T {
    let mut future = std::pin::pin!(future);
    match future
        .as_mut()
        .poll(&mut std::task::Context::from_waker(std::task::Waker::noop()))
    {
        std::task::Poll::Ready(value) => value,
        std::task::Poll::Pending => panic!("synthetic publisher unexpectedly suspended"),
    }
}

macro_rules! uncertain_publisher {
    ($name:ident, $mismatch:expr) => {
        #[async_trait::async_trait]
        impl AgentCapturePublisher for $name {
            async fn prepare_agent_capture(
                &self,
                request: RequestId,
                publication: PublicationId,
                profile: &NormalizerProfileV1,
                batch: &SanitizedObservationBatchV1,
                result: Vec<u8>,
            ) -> CaptureResult<AgentDeltaRequest> {
                Publisher
                    .prepare_agent_capture(request, publication, profile, batch, result)
                    .await
            }
            async fn publish_agent_delta(
                &self,
                request: &AgentDeltaRequest,
            ) -> CaptureResult<PublishOutcome> {
                if $mismatch {
                    let mut receipt = receipt(request);
                    receipt.result = b"wrong".to_vec();
                    Ok(PublishOutcome::Committed(receipt))
                } else {
                    Ok(PublishOutcome::OutcomeUnknown {
                        request_id: request.request_id,
                        publication_id: request.publication_id,
                    })
                }
            }
        }
    };
}
uncertain_publisher!(UnknownPublisher, false);
uncertain_publisher!(MismatchedPublisher, true);
