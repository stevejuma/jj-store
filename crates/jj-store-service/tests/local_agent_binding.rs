//! Real companion ref CAS and retained capture closures, independently of native WAL.
use jj_collab::*;
use jj_store_jj::local_metadata::{Domain, Limits, MetadataStore};
use jj_store_model as m;
use jj_store_service::local_repository::*;
use jj_store_wal::publication::{
    AgentObjectV1, CaptureBatchId, IntegrationId, NormalizerProfileV1, ObservationId,
    SanitizedMediaType, SanitizedObservationBatchV1, SanitizedObservationV1, SanitizedPayloadV1,
    SanitizedRepresentation, SourceInstanceId, SourceKind, SourceTrust,
};
use std::{path::Path, process::Command};
fn fixture() -> (tempfile::TempDir, LocalRepository) {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("repo");
    let output = Command::new("/opt/homebrew/bin/jj")
        .args(["--no-pager", "git", "init", "--colocate"])
        .arg(&path)
        .env("JJ_CONFIG", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    (root, reopen(&path))
}
fn reopen(path: &Path) -> LocalRepository {
    LocalRepository::new(
        &path.join(".jj/repo"),
        MetadataStore::open(&path.join(".git"), Limits::default()).unwrap(),
    )
}
async fn thread(repo: &LocalRepository, id: u8, command: ThreadCommandV1) -> ThreadCommandResultV1 {
    let outcome = repo
        .apply_collab_command(&LocalCollabRequest {
            version: 1,
            request_id: m::RequestId::new([id; 16]),
            command: LocalCollabCommand::Thread { intent: command },
        })
        .await
        .unwrap();
    let LocalCollabOutcome::Committed { receipt } = outcome else {
        panic!()
    };
    let LocalCollabResultV1::Thread { result } = receipt.result else {
        panic!()
    };
    result
}
async fn create(repo: &LocalRepository, id: u8) -> ThreadCommandResultV1 {
    thread(
        repo,
        id,
        ThreadCommandV1::Create {
            thread_id: ThreadId::new([id; 16]),
            title: format!("Thread {id}"),
            baseline: None,
        },
    )
    .await
}
fn batch() -> SanitizedObservationBatchV1 {
    let values: Vec<serde_json::Value> = serde_json::from_slice(include_bytes!(
        "../../jj-agent/tests/fixtures/codex-0.156.1-hooks.json"
    ))
    .unwrap();
    SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: CaptureBatchId::new([70; 16]),
        integration: IntegrationId::Codex,
        source_instance_id: SourceInstanceId::new([71; 16]),
        sanitizer_version: "agent-redaction@0.1.0".into(),
        representation: SanitizedRepresentation::SanitizedNative,
        observations: values
            .into_iter()
            .enumerate()
            .map(|(i, v)| SanitizedObservationV1 {
                observation_id: ObservationId::new([i as u8 + 1; 16]),
                source_sequence: i as u64 + 1,
                occurred_at_millis: Some(1_725_000_000_000 + i as i64),
                source_kind: SourceKind::Hook,
                source_trust: SourceTrust::LocalUserHook,
                payload: SanitizedPayloadV1::new(
                    SanitizedMediaType::Json,
                    &serde_json::to_vec(&v).unwrap(),
                )
                .unwrap(),
            })
            .collect(),
    }
}
fn bound(
    id: u8,
    target: &ThreadCommandResultV1,
    observed: Option<CollabObjectRef>,
    batch: SanitizedObservationBatchV1,
) -> LocalAgentRequest {
    LocalAgentRequest {
        version: 1,
        request_id: m::RequestId::new([id; 16]),
        command: LocalAgentCommand::BoundCapture {
            profile: NormalizerProfileV1::codex_0_156_1(),
            batch: Box::new(batch),
            binding: ThreadBindingV1 {
                thread_id: target.thread_id,
                observed_thread: target.version,
                observed_assignment: observed,
            },
        },
    }
}
#[tokio::test]
async fn bound_local_capture_selects_one_ref_retains_complete_session_and_replays_after_gc() {
    let (root, repo) = fixture();
    let first = create(&repo, 1).await;
    let active = thread(
        &repo,
        2,
        ThreadCommandV1::Lifecycle {
            thread_id: first.thread_id,
            expected: first.version,
            action: ThreadActionV1::Activate,
        },
    )
    .await;
    let archived = thread(
        &repo,
        3,
        ThreadCommandV1::Lifecycle {
            thread_id: first.thread_id,
            expected: active.version,
            action: ThreadActionV1::Archive,
        },
    )
    .await;
    let request = bound(40, &archived, None, batch());
    let before = repo.metadata_store().read().unwrap().unwrap();
    let original = repo.apply_agent_command(&request).await.unwrap();
    let selected = repo.metadata_store().read().unwrap().unwrap();
    assert_eq!(selected.sequence(), before.sequence() + 1);
    assert_eq!(
        selected.selection(Domain::Thread).unwrap().version,
        before.selection(Domain::Thread).unwrap().version + 1
    );
    assert_eq!(selected.selection(Domain::Agent).unwrap().version, 1);
    let actor = repo.actor().unwrap().unwrap();
    let receipt = selected.receipt(actor, request.request_id).unwrap();
    assert_eq!(receipt.domain, Domain::Agent);
    assert_eq!(receipt.selections.len(), 2);
    let agents = repo.agent_store().await.unwrap();
    let page = agents.reads().list_sessions(None, 10).await.unwrap();
    let snapshot = page.selections[0].1;
    let AgentObjectV1::SessionSnapshot(session) = agents.object(snapshot).await.unwrap() else {
        panic!()
    };
    assert_eq!(session.tools.len(), 3);
    let threads = repo.thread_store().await.unwrap();
    let current = threads.state.threads[&first.thread_id];
    let CollabObjectV1::ThreadVersion(value) = threads.object(current).unwrap() else {
        panic!()
    };
    assert_eq!(value.status, ThreadStatusV1::Archived);
    let assignment = threads.state.assignments[&session.session_id];
    let CollabObjectV1::ThreadLink(link) = threads.object(assignment).unwrap() else {
        panic!()
    };
    assert!(
        matches!(link.target, ThreadLinkTargetV1::Session { snapshot: v, primary: true, .. } if v == snapshot)
    );
    thread(
        &repo,
        4,
        ThreadCommandV1::Rename {
            thread_id: first.thread_id,
            expected: current,
            title: "After capture".into(),
        },
    )
    .await;
    let path = root.path().join("repo");
    let output = Command::new("git")
        .arg("--git-dir")
        .arg(path.join(".git"))
        .args(["gc", "--prune=now"])
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let fresh = reopen(&path);
    assert_eq!(fresh.apply_agent_command(&request).await.unwrap(), original);
    assert_eq!(
        fresh.agent_command_status(request.request_id).unwrap(),
        original
    );
    assert_eq!(
        fresh
            .agent_store()
            .await
            .unwrap()
            .object(snapshot)
            .await
            .unwrap(),
        AgentObjectV1::SessionSnapshot(session)
    );
    assert_eq!(
        fresh
            .thread_store()
            .await
            .unwrap()
            .state
            .assignments
            .values()
            .copied()
            .collect::<Vec<_>>(),
        vec![assignment]
    );
    let mut changed = request;
    let LocalAgentCommand::BoundCapture { binding, .. } = &mut changed.command else {
        panic!()
    };
    binding.observed_assignment = Some(assignment);
    assert_eq!(
        fresh.apply_agent_command(&changed).await.unwrap_err(),
        LocalError::IntentConflict
    );
}
#[tokio::test]
async fn bound_local_capture_rejects_stale_and_multi_session_intent_without_selecting_agents() {
    let (_root, repo) = fixture();
    let first = create(&repo, 1).await;
    let stale = bound(40, &first, None, batch());
    let renamed = thread(
        &repo,
        2,
        ThreadCommandV1::Rename {
            thread_id: first.thread_id,
            expected: first.version,
            title: "Renamed".into(),
        },
    )
    .await;
    let before = repo.metadata_store().read().unwrap().unwrap();
    assert_eq!(
        repo.apply_agent_command(&stale).await.unwrap_err(),
        LocalError::Stale
    );
    assert_eq!(
        repo.metadata_store().read().unwrap().unwrap().commit(),
        before.commit()
    );
    assert!(
        repo.metadata_store()
            .read()
            .unwrap()
            .unwrap()
            .selection(Domain::Agent)
            .is_none()
    );
    let mut multiple = batch();
    let mut another = multiple.observations[0].clone();
    another.source_sequence = 11;
    another.observation_id = ObservationId::new([50; 16]);
    another.payload = SanitizedPayloadV1::new(
        SanitizedMediaType::Json,
        br#"{"hook_event_name":"SessionStart","session_id":"another-session"}"#,
    )
    .unwrap();
    multiple.observations.push(another);
    assert_eq!(
        repo.apply_agent_command(&bound(41, &renamed, None, multiple))
            .await
            .unwrap_err(),
        LocalError::Invalid
    );
    assert_eq!(
        repo.metadata_store().read().unwrap().unwrap().commit(),
        before.commit()
    );
}
#[tokio::test]
async fn bound_local_capture_reassigns_one_session_and_retains_inactive_history() {
    let (_root, repo) = fixture();
    let first = create(&repo, 1).await;
    let second = create(&repo, 2).await;
    repo.apply_agent_command(&bound(40, &first, None, batch()))
        .await
        .unwrap();
    let initial = repo.thread_store().await.unwrap();
    let (&session_id, &assignment) = initial.state.assignments.first_key_value().unwrap();
    let mut late = batch();
    late.batch_id = CaptureBatchId::new([72; 16]);
    late.observations = vec![late.observations.last().unwrap().clone()];
    late.observations[0].source_sequence = 11;
    late.observations[0].observation_id = ObservationId::new([73; 16]);
    let request = bound(41, &second, Some(assignment), late);
    repo.apply_agent_command(&request).await.unwrap();
    let selected = repo.thread_store().await.unwrap();
    let new_assignment = selected.state.assignments[&session_id];
    assert_ne!(new_assignment, assignment);
    let CollabObjectV1::ThreadLink(old) = selected.object(assignment).unwrap() else {
        panic!()
    };
    let CollabObjectV1::ThreadLink(current) = selected.object(new_assignment).unwrap() else {
        panic!()
    };
    assert!(old.active);
    assert_eq!(old.thread_id, first.thread_id);
    assert_eq!(current.thread_id, second.thread_id);
    let CollabObjectV1::ThreadLink(inactive) =
        selected.object(selected.state.links[&old.id]).unwrap()
    else {
        panic!()
    };
    assert!(!inactive.active);
    assert_eq!(inactive.previous, Some(assignment));
    assert_ne!(
        selected.state.threads[&first.thread_id],
        initial.state.threads[&first.thread_id]
    );
}
