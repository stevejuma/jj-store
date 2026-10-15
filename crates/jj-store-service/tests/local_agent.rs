use jj_store_jj::local_metadata::{Domain, Limits, MetadataStore};
use jj_store_model as m;
use jj_store_object::{ObjectStore, memory::MemoryStore};
use jj_store_service::{local_repository::*, work::WorkCommandV1};
use jj_store_wal::publication::*;
use jj_store_wal::records::{RecordKind, Records};
use std::{process::Command, sync::Arc};

#[path = "local_agent/native_results.rs"]
mod native_results;
#[path = "local_agent/opencode_aliases.rs"]
mod opencode_aliases;
#[path = "local_agent/portable_review.rs"]
mod portable_review;
#[path = "local_agent/timing.rs"]
mod timing;

fn fixture() -> (tempfile::TempDir, LocalRepository) {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("repo");
    let output = Command::new("jj")
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
fn reopen(path: &std::path::Path) -> LocalRepository {
    LocalRepository::new(
        &path.join(".jj/repo"),
        MetadataStore::open(&path.join(".git"), Limits::default()).unwrap(),
    )
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
            .map(|(i, value)| SanitizedObservationV1 {
                observation_id: ObservationId::new([i as u8 + 1; 16]),
                source_sequence: i as u64 + 1,
                occurred_at_millis: Some(1_725_000_000_000 + i as i64),
                source_kind: SourceKind::Hook,
                source_trust: SourceTrust::LocalUserHook,
                payload: SanitizedPayloadV1::new(
                    SanitizedMediaType::Json,
                    &serde_json::to_vec(&value).unwrap(),
                )
                .unwrap(),
            })
            .collect(),
    }
}
fn request(id: u8, batch: SanitizedObservationBatchV1) -> LocalAgentRequest {
    LocalAgentRequest {
        version: 1,
        request_id: m::RequestId::new([id; 16]),
        command: LocalAgentCommand::Capture {
            profile: NormalizerProfileV1::codex_0_156_1(),
            batch: Box::new(batch),
        },
    }
}

#[tokio::test]
async fn browser_agent_presentation_pins_metadata_and_transcript_membership() {
    let (_root, repo) = fixture();
    repo.apply_agent_command(&request(1, batch()))
        .await
        .unwrap();
    let pin = repo.metadata_store().read().unwrap().unwrap().commit();
    let page = repo.browser_agent_sessions(pin, 20, None).await.unwrap();
    assert_eq!(page.sessions.len(), 1);
    let id = page.sessions[0].session_id;
    let detail = repo.browser_agent_session(pin, id).await.unwrap();
    assert!(!detail.activity.is_empty());
    assert_eq!(detail.tools.len(), 3);
    let stream = &detail.transcripts[0];
    let query = LocalBrowserTranscriptRequest {
        session_id: id,
        transcript_id: stream.transcript_id,
        source_instance_id: stream.source_instance_id,
        source_generation: stream.source_generation,
        limit: 10,
        max_bytes: 1024,
    };
    let transcript = repo
        .browser_agent_transcript(pin, query.clone(), None)
        .await
        .unwrap();
    assert_eq!(transcript.chunks[0].content, "Update the parser");
    let mut wrong = query;
    wrong.transcript_id = AgentObjectId::new([9; 32]);
    assert!(matches!(
        repo.browser_agent_transcript(pin, wrong, None).await,
        Err(LocalError::NotFound)
    ));
    repo.apply_agent_command(&LocalAgentRequest {
        version: 1,
        request_id: m::RequestId::new([8; 16]),
        command: LocalAgentCommand::Initialize,
    })
    .await
    .unwrap();
    assert_eq!(
        repo.browser_agent_sessions(pin, 20, None)
            .await
            .unwrap()
            .snapshot,
        page.snapshot
    );
    assert!(matches!(
        repo.browser_agent_sessions(
            "0909090909090909090909090909090909090909".parse().unwrap(),
            20,
            None
        )
        .await,
        Err(LocalError::Stale)
    ));
    assert!(matches!(
        repo.browser_agent_sessions(pin, 0, None).await,
        Err(LocalError::Invalid)
    ));
}
async fn session(store: &LocalAgentStore) -> (AgentObjectRef, AgentSessionSnapshotV1) {
    let page = store.reads().list_sessions(None, 20).await.unwrap();
    assert_eq!(page.selections.len(), 1);
    let reference = page.selections[0].1;
    let AgentObjectV1::SessionSnapshot(value) = store.object(reference).await.unwrap() else {
        panic!("expected session");
    };
    (reference, value)
}
#[tokio::test]
async fn local_agent_reads_do_not_initialize_metadata_or_actor() {
    let (root, repo) = fixture();
    let store = repo.agent_store().await.unwrap();
    assert!(store.snapshot.repo_id.is_none());
    assert_eq!(store.snapshot.agent_state_version, 0);
    assert!(
        store
            .reads()
            .list_sessions(None, 20)
            .await
            .unwrap()
            .selections
            .is_empty()
    );
    assert!(matches!(
        repo.agent_command_status(m::RequestId::new([1; 16]))
            .unwrap(),
        LocalAgentOutcome::NotKnown { .. }
    ));
    assert!(!root.path().join("repo/.jj/repo/lapis").exists());
    assert!(repo.metadata_store().read().unwrap().is_none());
}
#[tokio::test]
async fn local_capture_replays_original_receipt_after_restart_and_git_gc() {
    let (root, repo) = fixture();
    let initial = LocalAgentRequest {
        version: 1,
        request_id: m::RequestId::new([1; 16]),
        command: LocalAgentCommand::Initialize,
    };
    let initialized = repo.apply_agent_command(&initial).await.unwrap();
    assert!(matches!(initialized, LocalAgentOutcome::Committed { .. }));
    let command = request(2, batch());
    let original = repo.apply_agent_command(&command).await.unwrap();
    let selected = repo.metadata_store().read().unwrap().unwrap();
    let current = repo.agent_store().await.unwrap();
    let (reference, value) = session(&current).await;
    assert_eq!(value.tools.len(), 3);
    let path = root.path().join("repo");
    assert!(
        Command::new("git")
            .arg("--git-dir")
            .arg(path.join(".git"))
            .args(["gc", "--prune=now"])
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .status()
            .unwrap()
            .success()
    );
    let restarted = reopen(&path);
    assert_eq!(
        restarted.apply_agent_command(&command).await.unwrap(),
        original
    );
    assert_eq!(
        restarted.agent_command_status(command.request_id).unwrap(),
        original
    );
    assert_eq!(
        restarted.metadata_store().read().unwrap().unwrap().commit(),
        selected.commit()
    );
    assert_eq!(
        restarted
            .agent_store()
            .await
            .unwrap()
            .object(reference)
            .await
            .unwrap(),
        AgentObjectV1::SessionSnapshot(value)
    );
    assert_eq!(
        restarted.apply_agent_command(&initial).await.unwrap(),
        initialized
    );
    let mut collision = command.clone();
    collision.command = LocalAgentCommand::Initialize;
    assert_eq!(
        restarted.apply_agent_command(&collision).await.unwrap_err(),
        LocalError::IntentConflict
    );
}
#[tokio::test]
async fn local_transcripts_are_cumulative_and_old_reads_remain_pinned() {
    let (_root, repo) = fixture();
    let batch = batch();
    repo.apply_agent_command(&request(1, batch.clone()))
        .await
        .unwrap();
    let before = repo.agent_store().await.unwrap();
    let (_, first) = session(&before).await;
    let range = before
        .reads()
        .transcript_range(
            first.transcripts[0],
            batch.source_instance_id,
            1,
            None,
            10,
            1024,
        )
        .await
        .unwrap();
    assert_eq!(range.chunks.len(), 1);
    assert_eq!(range.chunks[0].payload.bytes(), b"Update the parser");
    let mut followup = batch.clone();
    followup.batch_id = CaptureBatchId::new([75; 16]);
    followup.observations = vec![SanitizedObservationV1 {
        observation_id: ObservationId::new([75; 16]),
        source_sequence: 11,
        occurred_at_millis: Some(1_725_000_000_011),
        source_kind: SourceKind::Hook,
        source_trust: SourceTrust::LocalUserHook,
        payload: SanitizedPayloadV1::new(SanitizedMediaType::Json,
            br#"{"hook_event_name":"UserPromptSubmit","session_id":"thr_demo","turn_id":"turn_2","prompt":"Explain the failure"}"#).unwrap(),
    }];
    repo.apply_agent_command(&request(2, followup))
        .await
        .unwrap();
    let after = repo.agent_store().await.unwrap();
    let (new_ref, second) = session(&after).await;
    assert_eq!(first.session_id, second.session_id);
    assert_eq!(second.evidence.len(), 2);
    assert_eq!(second.event_batches.len(), 2);
    assert_eq!(first.tools, second.tools);
    let range = after
        .reads()
        .transcript_range(
            second.transcripts[0],
            batch.source_instance_id,
            1,
            None,
            10,
            1024,
        )
        .await
        .unwrap();
    assert_eq!(range.chunks.len(), 2);
    assert_eq!(range.chunks[1].payload.bytes(), b"Explain the failure");
    assert_eq!(session(&before).await.1, first);
    assert_eq!(before.object(new_ref).await.unwrap_err(), LocalError::Stale);
    assert_eq!(
        repo.apply_agent_command(&request(3, batch))
            .await
            .unwrap_err(),
        LocalError::Stale
    );
    assert_eq!(repo.agent_store().await.unwrap().snapshot, after.snapshot);
}
#[tokio::test]
async fn agent_and_work_publications_preserve_each_other() {
    let (_root, repo) = fixture();
    repo.apply_agent_command(&request(1, batch()))
        .await
        .unwrap();
    let first = repo.agent_store().await.unwrap();
    let command = jj_store_service::work::Request {
        version: 1,
        request_id: m::RequestId::new([2; 16]),
        command: WorkCommandV1::CreateProject {
            id: jj_store_service::work::ProjectId::new([1; 16]),
            title: "Local agents".into(),
            body: "".into(),
        },
    };
    repo.apply_work_command(&command).unwrap();
    let selected = repo.metadata_store().read().unwrap().unwrap();
    let work = selected.selection(Domain::Work).unwrap().clone();
    assert_eq!(
        repo.agent_store().await.unwrap().snapshot.agent_root,
        first.snapshot.agent_root
    );
    repo.apply_agent_command(&LocalAgentRequest {
        version: 1,
        request_id: m::RequestId::new([3; 16]),
        command: LocalAgentCommand::Initialize,
    })
    .await
    .unwrap();
    assert_eq!(
        repo.metadata_store()
            .read()
            .unwrap()
            .unwrap()
            .selection(Domain::Work),
        Some(&work)
    );
    assert_eq!(
        repo.work_page(&Default::default()).unwrap().projects.len(),
        1
    );
    assert!(matches!(
        repo.agent_command_status(command.request_id),
        Err(LocalError::IntentConflict)
    ));
}
#[tokio::test]
async fn invalid_or_unsupported_capture_never_selects_partial_metadata() {
    let (_root, repo) = fixture();
    let mut unsupported = request(1, batch());
    if let LocalAgentCommand::Capture { profile, .. } = &mut unsupported.command {
        profile.product_version = "future".into();
    }
    assert_eq!(
        repo.apply_agent_command(&unsupported).await.unwrap_err(),
        LocalError::Unsupported
    );
    let mut invalid = request(2, batch());
    if let LocalAgentCommand::Capture { batch, .. } = &mut invalid.command {
        batch.observations[1].source_sequence = batch.observations[0].source_sequence;
    }
    assert_eq!(
        repo.apply_agent_command(&invalid).await.unwrap_err(),
        LocalError::Invalid
    );
    assert!(repo.metadata_store().read().unwrap().is_none());
    assert!(repo.actor().unwrap().is_none());
}
#[test]
fn concurrent_disjoint_captures_preserve_both_sources() {
    let (root, _) = fixture();
    let path = root.path().join("repo");
    let barrier = Arc::new(std::sync::Barrier::new(2));
    let workers: Vec<_> = (1..=2)
        .map(|id| {
            let repo = reopen(&path);
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                let mut batch = batch();
                batch.batch_id = CaptureBatchId::new([id; 16]);
                batch.source_instance_id = SourceInstanceId::new([id; 16]);
                barrier.wait();
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(repo.apply_agent_command(&request(id, batch)))
            })
        })
        .collect();
    for worker in workers {
        assert!(matches!(
            worker.join().unwrap().unwrap(),
            LocalAgentOutcome::Committed { .. }
        ));
    }
    let repo = reopen(&path);
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let store = repo.agent_store().await.unwrap();
        assert_eq!(
            store
                .reads()
                .list_catalog(AgentCatalogKind::Sources, None, 20)
                .await
                .unwrap()
                .selections
                .len(),
            2
        );
        assert_eq!(store.snapshot.agent_state_version, 2);
    });
}

#[tokio::test]
async fn git_carrier_preserves_native_record_bytes_and_repository_bound_ids() {
    let (_root, repo) = fixture();
    repo.apply_agent_command(&request(1, batch()))
        .await
        .unwrap();
    let snapshot = repo.agent_store().await.unwrap();
    let repository = snapshot.snapshot.repo_id.unwrap();
    let (reference, value) = session(&snapshot).await;
    let memory = Arc::new(MemoryStore::default());
    let native_records = Records::new(memory.clone(), repository);
    let native_ref = native_records
        .put(
            RecordKind::AgentObject,
            None,
            &AgentObjectV1::SessionSnapshot(value),
        )
        .await
        .unwrap();
    assert_eq!(native_ref.digest.as_bytes(), reference.id.as_bytes());
    let selected = repo.metadata_store().read().unwrap().unwrap();
    let descriptor: serde_json::Value = serde_json::from_slice(
        &repo
            .metadata_store()
            .read_domain(&selected, Domain::Agent)
            .unwrap()
            .unwrap(),
    )
    .unwrap();
    let key = native_ref.key(repository).unwrap();
    let blob = descriptor["records"][key.as_str()]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();
    let native_bytes = memory
        .get(&key, None)
        .await
        .unwrap()
        .read_bounded(16 * 1024 * 1024)
        .await
        .unwrap();
    assert_eq!(
        repo.metadata_store()
            .read_retained_blob(&selected, blob)
            .unwrap(),
        native_bytes
    );
    let root = native_records
        .put(
            RecordKind::AgentRoot,
            None,
            AgentSnapshotRead::root(&snapshot),
        )
        .await
        .unwrap();
    assert_eq!(Some(root.digest), snapshot.snapshot.agent_root);
    let other_repository = Records::new(memory, m::RepositoryId::new([99; 16]));
    let other = other_repository
        .put(
            RecordKind::AgentRoot,
            None,
            AgentSnapshotRead::root(&snapshot),
        )
        .await
        .unwrap();
    assert_ne!(root.digest, other.digest);
}

#[tokio::test]
async fn diagnostic_and_partial_tool_captures_survive_local_restart_without_state_loss() {
    let (root, repo) = fixture();
    let mut capture = batch();
    capture.observations.truncate(1);
    capture.observations[0].payload =
        SanitizedPayloadV1::new(SanitizedMediaType::Json, br#"{"status":"omitted"}"#).unwrap();
    let omitted = request(90, capture.clone());
    let original = repo.apply_agent_command(&omitted).await.unwrap();
    assert!(
        repo.agent_store()
            .await
            .unwrap()
            .reads()
            .list_sessions(None, 20)
            .await
            .unwrap()
            .selections
            .is_empty()
    );
    let restarted = reopen(&root.path().join("repo"));
    assert_eq!(
        restarted.apply_agent_command(&omitted).await.unwrap(),
        original
    );
    for (id, sequence, payload) in [
        (
            91,
            2,
            serde_json::json!({"hook_event_name":"PostToolUse", "session_id":"partial_session", "tool_use_id":"call_1", "tool_name":"Bash", "tool_response":{"exit_code":1,"output":"failure"}}),
        ),
        (
            92,
            3,
            serde_json::json!({"hook_event_name":"PreToolUse", "session_id":"partial_session", "tool_use_id":"call_1", "tool_name":"Bash", "tool_input":{"command":"test"}}),
        ),
    ] {
        capture.batch_id = CaptureBatchId::new([id; 16]);
        capture.observations[0].observation_id = ObservationId::new([id; 16]);
        capture.observations[0].source_sequence = sequence;
        capture.observations[0].payload = SanitizedPayloadV1::new(
            SanitizedMediaType::Json,
            &serde_json::to_vec(&payload).unwrap(),
        )
        .unwrap();
        restarted
            .apply_agent_command(&request(id, capture.clone()))
            .await
            .unwrap();
    }
    let store = reopen(&root.path().join("repo"))
        .agent_store()
        .await
        .unwrap();
    let (_, selected) = session(&store).await;
    let AgentObjectV1::ToolSnapshot(tool) = store
        .object(*selected.tools.values().next().unwrap())
        .await
        .unwrap()
    else {
        panic!("tool");
    };
    assert_eq!(tool.state, ToolState::Failed);
    assert!(tool.input.is_some());
    assert!(tool.output.is_some());
    assert_eq!(tool.evidence.len(), 2);
    let sources = store
        .reads()
        .list_catalog(AgentCatalogKind::Sources, None, 20)
        .await
        .unwrap();
    let AgentObjectV1::SourceState(source) = store.object(sources.selections[0].1).await.unwrap()
    else {
        panic!("source");
    };
    assert_eq!(source.coverage.gaps, 1);
    assert_eq!(source.receipts.len(), 3);
}

#[tokio::test]
async fn transcript_tail_rollover_preserves_pinned_history_after_local_restart() {
    let (root, repo) = fixture();
    let mut capture = batch();
    let template = capture.observations[0].clone();
    capture.batch_id = CaptureBatchId::new([93; 16]);
    capture.observations = (1u64..=256).map(|sequence| {
        let mut value = template.clone();
        let mut id = [0;16]; id[..8].copy_from_slice(&sequence.to_be_bytes());
        value.observation_id = ObservationId::new(id);
        value.source_sequence = sequence;
        value.payload = SanitizedPayloadV1::new(SanitizedMediaType::Json,
            &serde_json::to_vec(&serde_json::json!({"hook_event_name":"UserPromptSubmit", "session_id":"thr_demo", "turn_id":format!("t_{sequence}"), "prompt":format!("message {sequence}")})).unwrap()).unwrap();
        value
    }).collect();
    repo.apply_agent_command(&request(93, capture.clone()))
        .await
        .unwrap();
    let old = repo.agent_store().await.unwrap();
    let (_, old_session) = session(&old).await;
    let AgentObjectV1::TranscriptManifest(old_manifest) =
        old.object(old_session.transcripts[0]).await.unwrap()
    else {
        panic!("manifest")
    };
    let old_tail = old_manifest.streams[0].ordered_chunk_root;
    let AgentObjectV1::TranscriptTailPage(page) = old.object(old_tail).await.unwrap() else {
        panic!("tail")
    };
    assert_eq!(page.chunks.len(), 256);
    let mut next = template;
    next.source_sequence = 257;
    next.observation_id = ObservationId::new([94; 16]);
    next.payload = SanitizedPayloadV1::new(SanitizedMediaType::Json, br#"{"hook_event_name":"UserPromptSubmit","session_id":"thr_demo","turn_id":"t_257","prompt":"after rollover"}"#).unwrap();
    capture.batch_id = CaptureBatchId::new([94; 16]);
    capture.observations = vec![next];
    let intent = request(94, capture.clone());
    let receipt = repo.apply_agent_command(&intent).await.unwrap();
    let cold = reopen(&root.path().join("repo"));
    assert_eq!(cold.apply_agent_command(&intent).await.unwrap(), receipt);
    let current = cold.agent_store().await.unwrap();
    let (_, current_session) = session(&current).await;
    let AgentObjectV1::TranscriptManifest(manifest) = current
        .object(current_session.transcripts[0])
        .await
        .unwrap()
    else {
        panic!("manifest")
    };
    let AgentObjectV1::TranscriptTailPage(page) = current
        .object(manifest.streams[0].ordered_chunk_root)
        .await
        .unwrap()
    else {
        panic!("tail")
    };
    assert_eq!(page.prefix, Some(old_tail));
    assert_eq!(page.chunks.len(), 1);
    let range = current
        .reads()
        .transcript_range(
            current_session.transcripts[0],
            capture.source_instance_id,
            1,
            Some(255),
            10,
            1024,
        )
        .await
        .unwrap();
    assert_eq!(
        range.chunks.iter().map(|c| c.sequence).collect::<Vec<_>>(),
        vec![256, 257]
    );
    let pinned = old
        .reads()
        .transcript_range(
            old_session.transcripts[0],
            capture.source_instance_id,
            1,
            Some(255),
            10,
            1024,
        )
        .await
        .unwrap();
    assert_eq!(
        pinned.chunks.iter().map(|c| c.sequence).collect::<Vec<_>>(),
        vec![256]
    );
}
