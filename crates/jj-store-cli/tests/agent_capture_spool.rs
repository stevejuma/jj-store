use jj_agent::{
    AgentCatalogEditV1, AgentCatalogKey, AgentCatalogKind, AgentDeltaV1, AgentObjectId,
    AgentObjectKind, AgentObjectRef, CaptureBatchId, CaptureCursorV1, IntegrationId,
    NormalizerProfileV1, ObservationId, SanitizedMediaType, SanitizedObservationBatchV1,
    SanitizedObservationV1, SanitizedPayloadV1, SanitizedRepresentation, SourceInstanceId,
    SourceKind, SourcePositionV1, SourceTrust,
};
use jj_store_cli::agent_capture::{AgentCaptureSpool, PendingCaptureStateV1};
use jj_store_model::{PrincipalId, PublicationId, RepositoryId, RequestId};
use jj_store_wal::publication::AgentDeltaRequest;

fn batch() -> SanitizedObservationBatchV1 {
    SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: CaptureBatchId::new([1; 16]),
        integration: IntegrationId::Codex,
        source_instance_id: SourceInstanceId::new([2; 16]),
        sanitizer_version: "agent-redaction@0.1.0".into(),
        representation: SanitizedRepresentation::SanitizedNative,
        observations: vec![SanitizedObservationV1 {
            observation_id: ObservationId::new([3; 16]),
            source_sequence: 7,
            occurred_at_millis: Some(1_725_000_000_000),
            source_kind: SourceKind::Hook,
            source_trust: SourceTrust::LocalUserHook,
            payload: SanitizedPayloadV1::new(
                SanitizedMediaType::Json,
                br#"{"event":"SessionStart","session_id":"session-1"}"#,
            )
            .unwrap(),
        }],
    }
}

fn cursor() -> CaptureCursorV1 {
    CaptureCursorV1 {
        schema_version: 1,
        integration: IntegrationId::Codex,
        source_instance_id: SourceInstanceId::new([2; 16]),
        generation: 1,
        parser_version: "codex-hooks@0.156.1".into(),
        native_session_key: Some("session-1".into()),
        position: SourcePositionV1::NativeSequence { sequence: 7 },
    }
}

fn request() -> AgentDeltaRequest {
    AgentDeltaRequest::new(
        PrincipalId::new([4; 16]),
        RequestId::new([5; 16]),
        PublicationId::new([6; 16]),
        AgentDeltaV1 {
            schema_version: 1,
            edits: vec![AgentCatalogEditV1 {
                catalog: AgentCatalogKind::Sessions,
                key: AgentCatalogKey::derive("session", &[7]).unwrap(),
                expected: None,
                replacement: Some(AgentObjectRef::new(
                    AgentObjectKind::SessionSnapshot,
                    AgentObjectId::new([8; 32]),
                )),
            }],
        },
        b"capture accepted".to_vec(),
    )
    .unwrap()
}

#[test]
fn sanitized_capture_and_prepared_publication_survive_restart() {
    let directory = tempfile::tempdir().unwrap();
    let repository = RepositoryId::new([9; 16]);
    let profile = NormalizerProfileV1::codex_0_156_1();
    let batch = batch();

    let mut spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    let acknowledgement = spool
        .enqueue(repository, profile.clone(), cursor(), batch.clone())
        .unwrap();
    assert_eq!(acknowledgement.batch_id, batch.batch_id);
    assert_eq!(acknowledgement.position, cursor().position);
    spool.prepare(batch.batch_id, request()).unwrap();
    drop(spool);

    let mut spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    let entries = spool.pending().unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].repository, repository);
    assert_eq!(entries[0].profile, profile);
    assert_eq!(entries[0].batch, batch);
    assert_eq!(entries[0].cursor, cursor());
    assert_eq!(
        entries[0].state,
        PendingCaptureStateV1::Prepared { request: request() }
    );

    // A retry cannot regress a prepared publication back to captured state.
    spool
        .enqueue(repository, profile, cursor(), entries[0].batch.clone())
        .unwrap();
    assert!(matches!(
        spool.pending().unwrap()[0].state,
        PendingCaptureStateV1::Prepared { .. }
    ));
    spool
        .complete(
            batch.batch_id,
            RequestId::new([5; 16]),
            PublicationId::new([6; 16]),
        )
        .unwrap();
    assert!(spool.pending().unwrap().is_empty());
}

#[test]
fn capture_spool_rejects_concurrent_owners_and_changed_retries() {
    let directory = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    assert!(AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).is_err());

    spool
        .enqueue(
            RepositoryId::new([9; 16]),
            NormalizerProfileV1::codex_0_156_1(),
            cursor(),
            batch(),
        )
        .unwrap();
    assert!(
        spool
            .complete(
                batch().batch_id,
                RequestId::new([5; 16]),
                PublicationId::new([6; 16]),
            )
            .is_err()
    );
    assert!(
        spool
            .enqueue(
                RepositoryId::new([10; 16]),
                NormalizerProfileV1::codex_0_156_1(),
                cursor(),
                batch(),
            )
            .is_err()
    );
}

#[test]
fn capture_spool_enforces_byte_bounds_and_rejects_corruption() {
    let directory = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(directory.path(), 64).unwrap();
    assert!(
        spool
            .enqueue(
                RepositoryId::new([9; 16]),
                NormalizerProfileV1::codex_0_156_1(),
                cursor(),
                batch(),
            )
            .is_err()
    );
    drop(spool);

    std::fs::write(
        directory.path().join("pending/.tmp-orphan"),
        b"unacknowledged",
    )
    .unwrap();
    let spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    assert!(!directory.path().join("pending/.tmp-orphan").exists());
    let corrupt = directory
        .path()
        .join("pending")
        .join(format!("{}.json", batch().batch_id));
    std::fs::write(corrupt, b"not json").unwrap();
    assert!(spool.pending().is_err());
}
