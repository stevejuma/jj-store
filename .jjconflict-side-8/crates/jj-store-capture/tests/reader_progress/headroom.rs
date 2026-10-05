use super::*;
use jj_store_model::{PrincipalId, PublicationId, RequestId};
use jj_store_service::local_repository::{LocalAgentCommand, LocalAgentRequest};
use jj_store_wal::publication::AgentDeltaRequest;

fn pending_path(root: &Path, batch: CaptureBatchId) -> std::path::PathBuf {
    root.join("pending").join(format!("{batch}.json"))
}
fn bound_input() -> ReaderCapture {
    let mut capture = input(1, 1);
    capture.thread_binding = Some(jj_collab::ThreadBindingV1 {
        thread_id: jj_collab::ThreadId::new([1; 16]),
        observed_thread: jj_collab::CollabObjectRef {
            kind: jj_collab::CollabObjectKind::ThreadVersion,
            id: jj_collab::CollabObjectId::new([2; 32]),
        },
        observed_assignment: Some(jj_collab::CollabObjectRef {
            kind: jj_collab::CollabObjectKind::ThreadLink,
            id: jj_collab::CollabObjectId::new([3; 32]),
        }),
    });
    capture
}
fn maximum_native(batch: CaptureBatchId) -> AgentDeltaRequest {
    let mut result = b"jj-store-agent-capture-v1\0".to_vec();
    result.extend_from_slice(batch.as_bytes());
    AgentDeltaRequest::new(
        PrincipalId::new([255; 16]),
        RequestId::new([255; 16]),
        PublicationId::new([255; 16]),
        AgentDeltaV1 {
            schema_version: 1,
            edits: (0_u64..4096)
                .map(|key| AgentCatalogEditV1 {
                    catalog: AgentCatalogKind::Projections,
                    key: AgentCatalogKey::derive("headroom", &key.to_le_bytes()).unwrap(),
                    expected: Some(AgentObjectRef::new(
                        AgentObjectKind::ProjectionCatalog,
                        AgentObjectId::new([254; 32]),
                    )),
                    replacement: Some(AgentObjectRef::new(
                        AgentObjectKind::ProjectionCatalog,
                        AgentObjectId::new([255; 32]),
                    )),
                })
                .collect(),
        },
        result,
    )
    .unwrap()
}

#[test]
fn admitted_reader_reserves_maximum_native_preparation_across_restart_and_shared_writers() {
    let root = tempfile::tempdir().unwrap();
    let capture = bound_input();
    let batch = capture.batch.batch_id;
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool.enqueue_from_reader(bound_input()).unwrap();
    let path = pending_path(root.path(), batch);
    let accepted = fs::read(&path).unwrap();
    let budget = accepted.len() as u64;
    let entry = spool.pending().unwrap().remove(0);
    let mut prepared = entry.clone();
    let mut request = maximum_native(batch);
    let session = &mut request.delta.edits[0];
    session.catalog = AgentCatalogKind::Sessions;
    session.expected.as_mut().unwrap().kind = AgentObjectKind::SessionSnapshot;
    session.replacement.as_mut().unwrap().kind = AgentObjectKind::SessionSnapshot;
    let request = request
        .with_thread_binding(capture.thread_binding.clone().unwrap(), i64::MAX)
        .unwrap();
    prepared.state = PendingCaptureStateV1::Prepared {
        request: request.clone(),
    };
    assert!(serde_json::to_vec(&prepared).unwrap().len() <= accepted.len());
    drop(spool);
    let mut spool = AgentCaptureSpool::open(root.path(), budget).unwrap();
    let mut fork = spool.fork_handle();
    let mut peer = AgentCaptureSpool::open(root.path(), budget).unwrap();
    // An independent hook-style writer counts reserved bytes too.
    let next = input(2, 1);
    assert!(
        peer.enqueue(REPO, next.profile, next.cursor, next.batch)
            .is_err()
    );
    assert!(fork.enqueue_from_reader(input(2, 1)).is_err());
    assert_eq!(
        spool
            .latest_reader_progress(IntegrationId::Codex, SOURCE)
            .unwrap()
            .unwrap()
            .last_sequence,
        1
    );
    spool.enqueue_from_reader(capture).unwrap();
    assert_eq!(fs::read(&path).unwrap(), accepted);
    assert_eq!(spool.pending().unwrap(), vec![entry]);
    fork.prepare(batch, request.clone()).unwrap();
    assert!(fs::metadata(&path).unwrap().len() <= budget);
    assert_eq!(peer.pending().unwrap()[0].state, prepared.state);
    let durable = fs::read(&path).unwrap();
    peer.prepare(batch, request.clone()).unwrap();
    assert_eq!(fs::read(&path).unwrap(), durable);
    peer.complete(batch, request.request_id, request.publication_id)
        .unwrap();
    assert!(spool.pending().unwrap().is_empty());
    spool.enqueue_from_reader(input(2, 1)).unwrap();
}

#[test]
fn admitted_reader_reserves_local_payload_duplication_with_binding() {
    let root = tempfile::tempdir().unwrap();
    let mut capture = input(1, 1);
    capture.batch.observations[0].payload =
        SanitizedPayloadV1::new(SanitizedMediaType::Text, &vec![b'x'; 512 * 1024]).unwrap();
    capture.thread_binding = Some(jj_collab::ThreadBindingV1 {
        thread_id: jj_collab::ThreadId::new([1; 16]),
        observed_thread: jj_collab::CollabObjectRef {
            kind: jj_collab::CollabObjectKind::ThreadVersion,
            id: jj_collab::CollabObjectId::new([2; 32]),
        },
        observed_assignment: Some(jj_collab::CollabObjectRef {
            kind: jj_collab::CollabObjectKind::ThreadLink,
            id: jj_collab::CollabObjectId::new([3; 32]),
        }),
    });
    let batch = capture.batch.batch_id;
    let request = LocalAgentRequest {
        version: 1,
        request_id: RequestId::new([4; 16]),
        command: LocalAgentCommand::BoundCapture {
            profile: capture.profile.clone(),
            batch: Box::new(capture.batch.clone()),
            binding: capture.thread_binding.clone().unwrap(),
        },
    };
    let mut spool = AgentCaptureSpool::open(root.path(), 8 * 1024 * 1024).unwrap();
    spool.enqueue_from_reader(capture).unwrap();
    let path = pending_path(root.path(), batch);
    let budget = fs::metadata(&path).unwrap().len();
    let mut entry = spool.pending().unwrap().remove(0);
    entry.state = PendingCaptureStateV1::LocalPrepared {
        request: request.clone(),
    };
    assert!(serde_json::to_vec(&entry).unwrap().len() <= budget as usize);
    drop(spool);
    let mut spool = AgentCaptureSpool::open(root.path(), budget).unwrap();
    spool.prepare_local(batch, request.clone()).unwrap();
    assert!(fs::metadata(&path).unwrap().len() <= budget);
    assert_eq!(spool.pending().unwrap(), vec![entry]);
    spool.complete_local(batch, request.request_id).unwrap();
    assert!(spool.pending().unwrap().is_empty());
}

#[test]
fn preparation_capacity_refusal_reports_exact_relief_without_mutating_intent() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    let capture = input(1, 1);
    let batch = capture.batch.batch_id;
    spool.enqueue_from_reader(capture).unwrap();
    let entry = spool.pending().unwrap().remove(0);
    let path = pending_path(root.path(), batch);
    // An existing compact spool record remains supported independently of
    // current-only AgentStore; diagnostic inspection must not rewrite it.
    let compact = serde_json::to_vec(&entry).unwrap();
    fs::write(&path, &compact).unwrap();
    let limit = compact.len() as u64;
    let mut prepared = entry.clone();
    let request = maximum_native(batch);
    prepared.state = PendingCaptureStateV1::Prepared {
        request: request.clone(),
    };
    let required = serde_json::to_vec(&prepared).unwrap().len() as u64;
    assert!(required > limit);
    drop(spool);
    let mut spool = AgentCaptureSpool::open(root.path(), limit).unwrap();
    let progress = spool
        .latest_reader_progress(IntegrationId::Codex, SOURCE)
        .unwrap();
    let error = spool.prepare(batch, request.clone()).unwrap_err();
    let refusal = error
        .downcast_ref::<jj_store_capture::agent_capture::CaptureCapacityRefusal>()
        .expect("typed capacity diagnosis missing");
    assert_eq!(refusal.limit_bytes, limit);
    assert_eq!(refusal.required_bytes, required);
    assert_eq!(refusal.to_string(), "capture spool byte limit exceeded");
    assert_eq!(fs::read(&path).unwrap(), compact);
    assert_eq!(spool.pending().unwrap(), vec![entry]);
    assert_eq!(
        spool
            .latest_reader_progress(IntegrationId::Codex, SOURCE)
            .unwrap(),
        progress
    );
    drop(spool);
    let mut spool = AgentCaptureSpool::open(root.path(), required).unwrap();
    spool.prepare(batch, request.clone()).unwrap();
    assert_eq!(fs::metadata(&path).unwrap().len(), required);
    spool
        .complete(batch, request.request_id, request.publication_id)
        .unwrap();
    assert!(spool.pending().unwrap().is_empty());
}
