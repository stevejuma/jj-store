use jj_store_capture::{
    codex_discovery::{ApprovedCodexRoot, CodexDirectoryScanner},
    profile::CaptureProfile,
    source_registry::QualifiedCodexSource,
};
#[test]
fn cursor_hook_binding_qualifies_both_products_independently() {
    for profile in [
        CaptureProfile::CursorDesktopTranscriptV1,
        CaptureProfile::CursorCliTranscriptV1,
    ] {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().canonicalize().unwrap().join("session.jsonl");
        std::fs::write(&path, b"{\"role\":\"user\",\"message\":{\"content\":[{\"type\":\"text\",\"text\":\"fixture prompt\"}]}}\n").unwrap();
        std::fs::write(
            path.with_extension("jj-store.json"),
            serde_json::to_vec(&serde_json::json!({
                "schema_version":1,"profile":profile,"product_version":profile.product_version(),
                "session":"12345678-1234-1234-1234-123456789abc","directory":"/fixture/workspace",
                "transcript":path,"active":true,"observed_at_millis":1
            }))
            .unwrap(),
        )
        .unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(
                path.with_extension("jj-store.json"),
                std::fs::Permissions::from_mode(0o600),
            )
            .unwrap();
        }
        let mut scan =
            CodexDirectoryScanner::new(ApprovedCodexRoot::open(root.path()).unwrap().unwrap())
                .unwrap();
        let candidate = scan.step(64, 1).unwrap().candidates.pop().unwrap();
        assert!(
            QualifiedCodexSource::inspect(profile, candidate)
                .unwrap()
                .is_some()
        );
    }
}

use jj_agent::{IntegrationId, SourceInstanceId};
use jj_store_capture::{
    agent_capture::AgentCaptureSpool,
    codex_activity::{ActivityPoll, CodexActivityProbe},
    codex_transcript::{CodexTranscriptReader, TranscriptPoll},
    cursor_binding,
    source_registry::SourceRegistry,
};
use std::{fs, io::Write, path::Path};
const SESSION: &str = "12345678-1234-1234-1234-123456789abc";
fn bind(path: &Path, profile: CaptureProfile, event: &str) {
    cursor_binding::write_hook(&serde_json::to_vec(&serde_json::json!({"cursor_version":profile.product_version(),"conversation_id":SESSION,"workspace_roots":["/fixture/workspace"],"hook_event_name":event,"transcript_path":path,"prompt":"MUST NOT BE IN BINDING","text":"MUST NOT BE IN BINDING"})).unwrap(),Path::new("/fixture/workspace"),&[path.parent().unwrap().to_owned()]).unwrap();
}
fn qualified(path: &Path, profile: CaptureProfile) -> QualifiedCodexSource {
    let mut s = CodexDirectoryScanner::new(
        ApprovedCodexRoot::open(path.parent().unwrap())
            .unwrap()
            .unwrap(),
    )
    .unwrap();
    QualifiedCodexSource::inspect(profile, s.step(64, 1).unwrap().candidates.pop().unwrap())
        .unwrap()
        .unwrap()
}
fn ready(
    r: &mut CodexTranscriptReader,
) -> jj_store_capture::codex_transcript::CodexTranscriptBatch {
    for _ in 0..100 {
        if let TranscriptPoll::Ready(b) = r.poll().unwrap() {
            return *b;
        }
    }
    panic!("no complete fixture record")
}
#[test]
fn cursor_hook_handoff_is_private_exact_and_withholds_inactive_or_wrong_profile() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().canonicalize().unwrap().join("s.jsonl");
    fs::write(&path, "{}\n").unwrap();
    let p = CaptureProfile::CursorCliTranscriptV1;
    bind(&path, p, "stop");
    let mut probe = CodexActivityProbe::arm(qualified(&path, p)).unwrap();
    assert_eq!(probe.poll().unwrap(), ActivityPoll::Waiting);
    let registry = SourceRegistry::open(&root.path().join("sources"), [2; 16], 4).unwrap();
    assert!(!registry.previously_admitted(&qualified(&path, p)).unwrap());
    bind(&path, p, "beforeSubmitPrompt");
    assert_eq!(probe.poll_idle().unwrap(), None);
    assert_eq!(probe.poll().unwrap(), ActivityPoll::Active);
    let active = probe.proof().unwrap().unwrap();
    let source = registry.acquire_active(active).unwrap();
    drop(source);
    bind(&path, p, "sessionEnd");
    assert!(registry.previously_admitted(&qualified(&path, p)).unwrap());
    assert!(cursor_binding::read(CaptureProfile::CursorDesktopTranscriptV1, &path).is_err());
    let hint = fs::read_to_string(path.with_extension("jj-store.json")).unwrap();
    assert!(!hint.contains("MUST NOT"));
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(
            fs::metadata(path.with_extension("jj-store.json"))
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
    }
    let changed = hint.replace("/fixture/workspace", "/fixture/other");
    fs::write(path.with_extension("jj-store.json"), changed).unwrap();
    assert!(!registry.previously_admitted(&qualified(&path, p)).unwrap());
}
#[test]
fn cursor_complete_prefix_retries_partial_flush_restart_and_rewrite_without_resetting_ids() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().canonicalize().unwrap().join("s.jsonl");
    let first =
        "{\"role\":\"user\",\"message\":{\"content\":[{\"type\":\"text\",\"text\":\"one\"}]}}\n";
    fs::write(&path, first).unwrap();
    let p = CaptureProfile::CursorCliTranscriptV1;
    bind(&path, p, "sessionStart");
    let source = SourceInstanceId::new([8; 16]);
    let mut r = CodexTranscriptReader::open(p, &path, source, None).unwrap();
    let b = ready(&mut r);
    assert_eq!(b.cursor.integration, IntegrationId::Cursor);
    let key = b.records[0].record_key;
    assert_eq!(ready(&mut r).batch_id, b.batch_id);
    let mut spool = AgentCaptureSpool::open(&root.path().join("spool"), 4 * 1024 * 1024).unwrap();
    let ack = spool
        .enqueue_from_reader(
            b.sanitize_omitted(jj_store_model::RepositoryId::new([1; 16]), &p.normalizer())
                .unwrap(),
        )
        .unwrap();
    r.acknowledge(&ack).unwrap();
    let previous = spool
        .latest_reader_progress(IntegrationId::Cursor, source)
        .unwrap()
        .unwrap();
    drop(r);
    let mut r = CodexTranscriptReader::open(p, &path, source, Some(previous)).unwrap();
    loop {
        match r.poll().unwrap() {
            TranscriptPoll::Verifying => {}
            TranscriptPoll::Waiting => break,
            _ => panic!("unchanged restart duplicated records"),
        }
    }
    let mut f = fs::OpenOptions::new().append(true).open(&path).unwrap();
    f.write_all(b"{\"type\":\"turn_ended\",\"status\":\"success\"}")
        .unwrap();
    assert!(!matches!(r.poll().unwrap(), TranscriptPoll::Ready(_)));
    f.write_all(b"\n").unwrap();
    let tail = ready(&mut r);
    assert_eq!(tail.records.len(), 1);
    let ack = spool
        .enqueue_from_reader(
            tail.sanitize_omitted(jj_store_model::RepositoryId::new([1; 16]), &p.normalizer())
                .unwrap(),
        )
        .unwrap();
    r.acknowledge(&ack).unwrap();
    fs::write(
        &path,
        format!("{first}{{\"type\":\"turn_ended\",\"status\":\"error\"}}\n"),
    )
    .unwrap();
    let rewrite = ready(&mut r);
    assert!(rewrite.cursor.generation > b.cursor.generation);
    assert!(rewrite.records[0].sequence > tail.records[0].sequence);
    assert_eq!(rewrite.records[0].record_key, key);
}
#[test]
fn cursor_normalization_keeps_missing_timing_and_tool_state_unknown() {
    use jj_agent::*;
    let profile = CaptureProfile::CursorCliTranscriptV1.normalizer();
    let observation=SanitizedObservationV1 {observation_id:ObservationId::new([2;16]),source_sequence:1,occurred_at_millis:None,source_kind:SourceKind::Transcript,source_trust:SourceTrust::LocalTranscript,payload:SanitizedPayloadV1::new(SanitizedMediaType::Json,&serde_json::to_vec(&serde_json::json!({"capture_format":"cursor-jsonl/v1","product_version":profile.product_version,"session_id":SESSION,"record_key":"ab".repeat(32),"record":{"role":"assistant","message":{"content":[{"type":"text","text":"fixture answer"},{"type":"tool_use","name":"Shell","input":{"command":"pwd"}}]}}})).unwrap()).unwrap()};
    let batch = SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: CaptureBatchId::new([3; 16]),
        integration: IntegrationId::Cursor,
        source_instance_id: SourceInstanceId::new([4; 16]),
        sanitizer_version: "fixture/v1".into(),
        representation: SanitizedRepresentation::SanitizedNative,
        observations: vec![observation],
    };
    let out = normalize_batch(&profile, &batch).unwrap();
    assert_eq!(out.records.len(), 2);
    assert!(out.diagnostics.is_empty());
    let tool = out
        .records
        .iter()
        .find_map(|r| {
            if let NormalizedActivityRecordV1::Tool(t) = r {
                Some(t)
            } else {
                None
            }
        })
        .unwrap();
    assert_eq!(tool.state, ToolState::Unknown);
    assert!(tool.output.is_none());
    assert!(
        out.aliases
            .iter()
            .all(|a| a.native_kind == NativeEntityKind::Session)
    );
    assert_eq!(
        normalize_batch(&profile, &batch).unwrap().records,
        out.records
    );
}

#[test]
fn cursor_default_discovery_skips_generated_project_payloads() {
    let root = tempfile::tempdir().unwrap();
    let projects = root.path().canonicalize().unwrap().join("projects");
    let project = projects.join("qualified-project");
    let transcripts = project.join("agent-transcripts").join(SESSION);
    fs::create_dir_all(&transcripts).unwrap();
    let transcript = transcripts.join(format!("{SESSION}.jsonl"));
    fs::write(&transcript, "{}\n").unwrap();
    fs::write(transcript.with_extension("jj-store.json"), "{}").unwrap();
    fs::write(transcripts.join("unbound.jsonl"), "{}\n").unwrap();
    for name in ["mcps", "canvases"] {
        let unrelated = project.join(name).join("node_modules");
        fs::create_dir_all(&unrelated).unwrap();
        for i in 0..128 {
            fs::write(unrelated.join(format!("unrelated-{i}.jsonl")), "{}\n").unwrap();
        }
    }
    let mut scanner =
        CodexDirectoryScanner::cursor(ApprovedCodexRoot::open(&projects).unwrap().unwrap())
            .unwrap();
    let mut examined = 0;
    let mut candidates = Vec::new();
    loop {
        let step = scanner.step(64, 16).unwrap();
        examined += step.entries_examined;
        candidates.extend(step.candidates);
        if step.complete {
            break;
        }
    }
    assert_eq!(
        candidates.len(),
        1,
        "generated project files must not compete with transcript discovery"
    );
    assert_eq!(candidates[0].path(), transcript);
    assert!(
        examined < 16,
        "only the supported transcript layout is traversed"
    );
}

#[test]
fn cursor_recent_completion_retains_active_witness_for_discovery() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().canonicalize().unwrap().join("short.jsonl");
    fs::write(&path, "{}\n").unwrap();
    let profile = CaptureProfile::CursorCliTranscriptV1;
    bind(&path, profile, "stop");
    assert!(
        !cursor_binding::read(profile, &path)
            .unwrap()
            .fresh_activity()
    );
    bind(&path, profile, "afterAgentResponse");
    bind(&path, profile, "stop");
    let binding = cursor_binding::read(profile, &path).unwrap();
    assert!(!binding.active);
    assert!(
        binding.fresh_activity(),
        "terminal hook must retain the recent exact active witness"
    );
    let mut value: serde_json::Value =
        serde_json::from_slice(&fs::read(path.with_extension("jj-store.json")).unwrap()).unwrap();
    value["last_active_at_millis"] = 1.into();
    fs::write(
        path.with_extension("jj-store.json"),
        serde_json::to_vec(&value).unwrap(),
    )
    .unwrap();
    assert!(
        !cursor_binding::read(profile, &path)
            .unwrap()
            .fresh_activity()
    );
}

#[test]
fn cursor_nullable_start_hands_off_only_to_exact_recent_terminal_binding() {
    for variant in 0..5 {
        let root = tempfile::tempdir().unwrap();
        let root = root.path().canonicalize().unwrap();
        let path = root.join("delayed.jsonl");
        let cli = CaptureProfile::CursorCliTranscriptV1;
        let start = serde_json::json!({"cursor_version":cli.product_version(),"conversation_id":SESSION,
            "workspace_roots":["/fixture/workspace"],"hook_event_name":"sessionStart","transcript_path":null});
        assert!(
            !cursor_binding::write_hook(
                &serde_json::to_vec(&start).unwrap(),
                Path::new("/fixture/workspace"),
                std::slice::from_ref(&root)
            )
            .unwrap()
        );
        let pending = fs::read_dir(root.join(".jj-store-bindings"))
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .path();
        if variant >= 3 {
            let mut v: serde_json::Value =
                serde_json::from_slice(&fs::read(&pending).unwrap()).unwrap();
            v["started_at_millis"] = if variant == 3 {
                0.into()
            } else {
                u64::MAX.into()
            };
            fs::write(&pending, serde_json::to_vec(&v).unwrap()).unwrap();
        }
        fs::write(&path, "{}\n").unwrap();
        let profile = if variant == 2 {
            CaptureProfile::CursorDesktopTranscriptV1
        } else {
            cli
        };
        let workspace = if variant == 1 {
            "/fixture/other"
        } else {
            "/fixture/workspace"
        };
        let finish = serde_json::json!({"cursor_version":profile.product_version(),"conversation_id":SESSION,
            "workspace_roots":[workspace],"hook_event_name":"sessionEnd","transcript_path":path});
        assert!(
            cursor_binding::write_hook(
                &serde_json::to_vec(&finish).unwrap(),
                Path::new(workspace),
                std::slice::from_ref(&root)
            )
            .unwrap()
        );
        let binding = cursor_binding::read(profile, &path).unwrap();
        assert!(!binding.active);
        assert_eq!(binding.fresh_activity(), variant == 0, "variant {variant}");
        let hint = fs::read_to_string(path.with_extension("jj-store.json")).unwrap();
        assert!(!hint.contains("prompt"));
        if variant == 0 {
            assert_eq!(
                fs::read_dir(root.join(".jj-store-bindings"))
                    .unwrap()
                    .count(),
                0
            );
        }
    }
}
