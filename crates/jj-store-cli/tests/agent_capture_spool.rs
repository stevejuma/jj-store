use jj_agent::{
    AgentCatalogEditV1, AgentCatalogKey, AgentCatalogKind, AgentDeltaV1, AgentObjectId,
    AgentObjectKind, AgentObjectRef, CaptureBatchId, CaptureCursorV1, IntegrationId,
    NormalizerProfileV1, ObservationId, SanitizedMediaType, SanitizedObservationBatchV1,
    SanitizedObservationV1, SanitizedPayloadV1, SanitizedRepresentation, SourceInstanceId,
    SourceKind, SourcePositionV1, SourceTrust,
};
use jj_store_cli::agent_capture::{AgentCaptureSpool, PendingCaptureStateV1};
use jj_store_cli::agent_sanitization::KINGFISHER_POLICY;
use jj_store_model::{PrincipalId, PublicationId, RepositoryId, RequestId};
use jj_store_wal::publication::AgentDeltaRequest;

fn batch() -> SanitizedObservationBatchV1 {
    SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: CaptureBatchId::new([1; 16]),
        integration: IntegrationId::Codex,
        source_instance_id: SourceInstanceId::new([2; 16]),
        sanitizer_version: KINGFISHER_POLICY.into(),
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
    let peer = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    let lock =
        jj_store_server::policy::exclusive_lock(&directory.path().join("mutation.lock")).unwrap();
    assert!(AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).is_err());
    assert!(peer.pending().is_err());
    assert!(
        spool
            .enqueue(
                RepositoryId::new([9; 16]),
                NormalizerProfileV1::codex_0_156_1(),
                cursor(),
                batch()
            )
            .is_err()
    );
    drop(lock);

    spool
        .enqueue(
            RepositoryId::new([9; 16]),
            NormalizerProfileV1::codex_0_156_1(),
            cursor(),
            batch(),
        )
        .unwrap();
    assert_eq!(peer.pending().unwrap().len(), 1);
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

#[test]
fn capture_lock_protocol_excludes_legacy_writers_in_both_directions() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("capture.lock");
    let legacy = jj_store_server::policy::exclusive_lock(&path).unwrap();
    assert!(AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).is_err());
    assert!(!directory.path().join("pending").exists());
    drop(legacy);
    let first = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    let second = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    assert!(jj_store_server::policy::exclusive_lock(&path).is_err());
    drop(first);
    assert!(jj_store_server::policy::exclusive_lock(&path).is_err());
    drop(second);
    assert!(jj_store_server::policy::exclusive_lock(&path).is_ok());
}

#[test]
fn concurrent_handles_preserve_queue_byte_budget_and_prepared_identity() {
    let directory = tempfile::tempdir().unwrap();
    let mut first = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    let repository = RepositoryId::new([9; 16]);
    first
        .enqueue(
            repository,
            NormalizerProfileV1::codex_0_156_1(),
            cursor(),
            batch(),
        )
        .unwrap();
    first.prepare(batch().batch_id, request()).unwrap();
    let original = first.pending().unwrap();
    let bytes = std::fs::metadata(
        directory
            .path()
            .join("pending")
            .join(format!("{}.json", batch().batch_id)),
    )
    .unwrap()
    .len();
    let mut second = AgentCaptureSpool::open(directory.path(), bytes).unwrap();
    second
        .enqueue(
            repository,
            NormalizerProfileV1::codex_0_156_1(),
            cursor(),
            batch(),
        )
        .unwrap();
    let mut next = batch();
    next.batch_id = CaptureBatchId::new([20; 16]);
    assert!(
        second
            .enqueue(
                repository,
                NormalizerProfileV1::codex_0_156_1(),
                cursor(),
                next
            )
            .is_err()
    );
    let mut changed = request();
    changed.request_id = RequestId::new([20; 16]);
    assert!(second.prepare(batch().batch_id, changed).is_err());
    assert_eq!(first.pending().unwrap(), original);
}

#[test]
fn enqueue_enforces_entry_limit_before_acknowledging() {
    let directory = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(directory.path(), 64 * 1024 * 1024).unwrap();
    let repository = RepositoryId::new([9; 16]);
    let profile = NormalizerProfileV1::codex_0_156_1();
    spool
        .enqueue(repository, profile.clone(), cursor(), batch())
        .unwrap();
    let template = spool.pending().unwrap().remove(0);
    // Seed already durable valid records, avoiding 4096 redundant fsyncs in the fixture.
    for id in 2_u64..=4096 {
        let mut entry = template.clone();
        entry.batch.batch_id = CaptureBatchId::new(u128::from(id).to_le_bytes());
        std::fs::write(
            directory
                .path()
                .join("pending")
                .join(format!("{}.json", entry.batch.batch_id)),
            serde_json::to_vec(&entry).unwrap(),
        )
        .unwrap();
    }
    assert_eq!(spool.pending().unwrap().len(), 4096);
    let mut excess = batch();
    excess.batch_id = CaptureBatchId::new([20; 16]);
    assert!(
        spool
            .enqueue(repository, profile, cursor(), excess)
            .is_err()
    );
    assert_eq!(spool.pending().unwrap().len(), 4096);
}

#[cfg(unix)]
#[test]
fn capture_lock_and_pending_symlinks_fail_closed() {
    let directory = tempfile::tempdir().unwrap();
    let foreign = tempfile::tempdir().unwrap();
    std::os::unix::fs::symlink(foreign.path(), directory.path().join("pending")).unwrap();
    assert!(AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).is_err());
    assert!(std::fs::read_dir(foreign.path()).unwrap().next().is_none());
    std::fs::remove_file(directory.path().join("pending")).unwrap();
    std::fs::remove_file(directory.path().join("capture.lock")).unwrap();
    std::fs::write(foreign.path().join("lock"), b"unchanged").unwrap();
    std::os::unix::fs::symlink(
        foreign.path().join("lock"),
        directory.path().join("capture.lock"),
    )
    .unwrap();
    assert!(AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).is_err());
    assert_eq!(
        std::fs::read(foreign.path().join("lock")).unwrap(),
        b"unchanged"
    );
}

#[test]
fn prepared_thread_binding_survives_restart_and_invalid_binding_is_not_persisted() {
    use jj_collab::{CollabObjectId, CollabObjectKind, CollabObjectRef, ThreadBindingV1, ThreadId};
    let directory = tempfile::tempdir().unwrap();
    let repository = RepositoryId::new([9; 16]);
    let profile = NormalizerProfileV1::codex_0_156_1();
    let mut spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    spool
        .enqueue(repository, profile, cursor(), batch())
        .unwrap();
    let bound = request()
        .with_thread_binding(
            ThreadBindingV1 {
                thread_id: ThreadId::new([10; 16]),
                observed_thread: CollabObjectRef {
                    kind: CollabObjectKind::ThreadVersion,
                    id: CollabObjectId::new([11; 32]),
                },
                observed_assignment: None,
            },
            123,
        )
        .unwrap();
    let mut invalid = bound.clone();
    invalid.thread_binding.as_mut().unwrap().accepted_at_millis = -1;
    assert!(spool.prepare(batch().batch_id, invalid).is_err());
    assert!(matches!(
        spool.pending().unwrap()[0].state,
        PendingCaptureStateV1::Captured
    ));
    spool.prepare(batch().batch_id, bound.clone()).unwrap();
    drop(spool);
    let spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    assert!(
        matches!(&spool.pending().unwrap()[0].state,PendingCaptureStateV1::Prepared { request } if request==&bound)
    );
}
