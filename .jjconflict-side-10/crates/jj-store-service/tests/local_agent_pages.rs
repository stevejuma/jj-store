use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use jj_store_jj::local_metadata::{Domain, Limits, MetadataStore, Preparation, Update};
use jj_store_model as m;
use jj_store_service::local_repository::*;
use jj_store_wal::publication::*;

fn fixture() -> (tempfile::TempDir, LocalRepository) {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("repo");
    assert!(
        std::process::Command::new("/opt/homebrew/bin/jj")
            .args(["--no-pager", "git", "init", "--colocate"])
            .arg(&path)
            .env("JJ_CONFIG", "/dev/null")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .output()
            .unwrap()
            .status
            .success()
    );
    let store = MetadataStore::open(&path.join(".git"), Limits::default()).unwrap();
    (root, LocalRepository::new(&path.join(".jj/repo"), store))
}
async fn capture(repo: &LocalRepository, id: u8) {
    repo.apply_agent_command(&LocalAgentRequest {
        version: 1,
        request_id: m::RequestId::new([id; 16]),
        command: LocalAgentCommand::Capture {
            profile: NormalizerProfileV1::codex_0_156_1(),
            batch: Box::new(SanitizedObservationBatchV1 {
                schema_version: 1,
                batch_id: CaptureBatchId::new([id; 16]),
                integration: IntegrationId::Codex,
                source_instance_id: SourceInstanceId::new([id; 16]),
                sanitizer_version: "agent-redaction@0.1.0".into(),
                representation: SanitizedRepresentation::SanitizedNative,
                observations: vec![SanitizedObservationV1 {
                    observation_id: ObservationId::new([id; 16]),
                    source_sequence: 1,
                    occurred_at_millis: None,
                    source_kind: SourceKind::Hook,
                    source_trust: SourceTrust::LocalUserHook,
                    payload: SanitizedPayloadV1::new(
                        SanitizedMediaType::Json,
                        br#"{"hook_event_name":"SessionStart","session_id":"page-session"}"#,
                    )
                    .unwrap(),
                }],
            }),
        },
    })
    .await
    .unwrap();
}
#[tokio::test]
async fn local_session_cursor_survives_restart_and_later_capture() {
    let (root, repo) = fixture();
    capture(&repo, 1).await;
    capture(&repo, 2).await;
    let first = repo.agent_sessions(1, None).await.unwrap();
    let cursor = first.next_cursor.as_deref().unwrap();
    capture(&repo, 3).await;
    let path = root.path().join("repo");
    let restart = LocalRepository::new(
        &path.join(".jj/repo"),
        MetadataStore::open(&path.join(".git"), Limits::default()).unwrap(),
    );
    let next = restart.agent_sessions(1, Some(cursor)).await.unwrap();
    assert_eq!(first.snapshot, next.snapshot);
    assert_eq!(next.sessions.len(), 1);
    assert!(next.next_cursor.is_none());
    assert_ne!(
        first.sessions[0].session.session_id,
        next.sessions[0].session.session_id
    );
    assert_eq!(
        restart
            .agent_sessions(20, None)
            .await
            .unwrap()
            .sessions
            .len(),
        3
    );
    assert_eq!(
        restart.agent_sessions(2, Some(cursor)).await.unwrap_err(),
        LocalError::Invalid
    );
}
#[tokio::test]
async fn forged_agent_cursor_cannot_select_a_staged_metadata_candidate() {
    let (_root, repo) = fixture();
    capture(&repo, 1).await;
    capture(&repo, 2).await;
    let first = repo.agent_sessions(1, None).await.unwrap();
    let selected = repo.metadata_store().read().unwrap().unwrap();
    let update = Update {
        repository: selected.repository(),
        actor: repo.actor().unwrap().unwrap(),
        request: m::RequestId::new([9; 16]),
        intent: m::IntentDigest::new([9; 32]),
        domain: Domain::Agent,
        expected: selected.selection(Domain::Agent).cloned(),
        bytes: repo
            .metadata_store()
            .read_domain(&selected, Domain::Agent)
            .unwrap()
            .unwrap(),
        result: vec![],
        retained: Default::default(),
    };
    let Preparation::Ready(candidate) = repo
        .metadata_store()
        .prepare(Some(&selected), &update)
        .unwrap()
    else {
        panic!("staged candidate required")
    };
    let mut cursor: serde_json::Value =
        serde_json::from_slice(&URL_SAFE_NO_PAD.decode(first.next_cursor.unwrap()).unwrap())
            .unwrap();
    cursor["commit"] = serde_json::to_value(candidate.commit()).unwrap();
    // Preserve the canonical field order from the actual cursor.
    let cursor = format!(
        "{{\"version\":1,\"commit\":{},\"after\":{},\"limit\":1}}",
        cursor["commit"], cursor["after"]
    );
    let cursor = URL_SAFE_NO_PAD.encode(cursor);
    assert_eq!(
        repo.agent_sessions(1, Some(&cursor)).await.unwrap_err(),
        LocalError::Stale
    );
    assert_eq!(
        repo.metadata_store().read().unwrap().unwrap().commit(),
        selected.commit()
    );
}
