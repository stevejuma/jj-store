#![cfg(unix)]
use jj_store_capture::{
    codex_activity::{ActivityPoll, CodexActivityProbe},
    codex_discovery::{ApprovedCodexRoot, CodexDirectoryScanner},
    profile::CaptureProfile,
    source_registry::{QualifiedCodexSource, SourceRegistry},
};
use serde_json::json;
use std::{fs, io::Write as _, path::Path};
const INSTALLATION: [u8; 16] = [8; 16];
fn record(value: serde_json::Value) -> Vec<u8> {
    let mut bytes = serde_json::to_vec(&value).unwrap();
    bytes.push(b'\n');
    bytes
}
fn header(desktop: bool, id: u8) -> Vec<u8> {
    record(json!({"type":"session_meta","payload":{
        "id":format!("00000000-0000-0000-0000-{id:012x}"),"cwd":"/workspace/a",
        "cli_version":if desktop { "0.159.2" } else { "0.156.1" },
        "source":if desktop { "vscode" } else { "cli" },
        "originator":if desktop { "Codex Desktop" } else { "codex_cli_rs" }
    }}))
}
fn event(kind: &str) -> Vec<u8> {
    record(json!({"type":"event_msg","payload":{"type":kind,"turn_id":"turn-1"}}))
}
fn append(path: &Path, bytes: &[u8]) {
    fs::OpenOptions::new()
        .append(true)
        .open(path)
        .unwrap()
        .write_all(bytes)
        .unwrap();
}
fn qualified(root: &Path, profile: CaptureProfile) -> QualifiedCodexSource {
    let mut scanner =
        CodexDirectoryScanner::new(ApprovedCodexRoot::open(root).unwrap().unwrap()).unwrap();
    let candidate = scanner
        .step(100, 100)
        .unwrap()
        .candidates
        .into_iter()
        .next()
        .unwrap();
    QualifiedCodexSource::inspect(profile, candidate)
        .unwrap()
        .unwrap()
}
fn poll(probe: &mut CodexActivityProbe) -> ActivityPoll {
    for _ in 0..100 {
        let result = probe.poll().unwrap();
        if matches!(result, ActivityPoll::Waiting | ActivityPoll::Active) {
            return result;
        }
    }
    panic!("bounded probe did not finish")
}

#[test]
fn compact_eof_does_not_admit_an_old_partial_start_and_renews_a_genuine_start() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    let start = event("task_started");
    fs::write(
        &path,
        [header(false, 1), start[..start.len() - 1].to_vec()].concat(),
    )
    .unwrap();
    let mut probe =
        CodexActivityProbe::arm(qualified(root.path(), CaptureProfile::CodexCliJsonlV1)).unwrap();
    assert_eq!(poll(&mut probe), ActivityPoll::Waiting);
    assert!(probe.is_compact());
    assert_eq!(probe.poll_idle().unwrap(), Some(ActivityPoll::Waiting));
    append(&path, b"\n");
    assert!(probe.poll_idle().unwrap().is_none());
    assert_eq!(poll(&mut probe), ActivityPoll::Waiting);
    assert!(probe.proof().unwrap().is_none());
    append(&path, &start);
    assert_eq!(poll(&mut probe), ActivityPoll::Active);
    let old = probe.proof().unwrap().unwrap();
    assert_eq!(probe.poll_idle().unwrap(), Some(ActivityPoll::Active));
    append(&path, &event("task_complete"));
    assert!(old.revalidate().is_err());
    assert!(probe.poll_idle().unwrap().is_none());
    assert_eq!(poll(&mut probe), ActivityPoll::Active);
    probe.proof().unwrap().unwrap().revalidate().unwrap();
}

#[test]
fn historical_sessions_and_growth_do_not_admit_or_allocate_source_state() {
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let state = private.path().join("registry");
    let path = root.path().join("session.jsonl");
    fs::write(&path, [header(false, 1), event("task_started")].concat()).unwrap();
    let mut probe =
        CodexActivityProbe::arm(qualified(root.path(), CaptureProfile::CodexCliJsonlV1)).unwrap();
    assert_eq!(poll(&mut probe), ActivityPoll::Waiting);
    append(
        &path,
        &record(json!({"type":"response_item","payload":{"type":"message","text":"task_started"}})),
    );
    append(&path, &event("task_complete"));
    assert_eq!(poll(&mut probe), ActivityPoll::Waiting);
    assert!(probe.proof().unwrap().is_none());
    assert!(
        SourceRegistry::open_existing(&state, INSTALLATION, 4)
            .unwrap()
            .is_none()
    );
    assert!(!state.exists());
    append(&path, &event("task_started"));
    assert_eq!(poll(&mut probe), ActivityPoll::Active);
    let proof = probe.proof().unwrap().unwrap();
    assert_eq!(proof.directory(), Path::new("/workspace/a"));
    let registry = SourceRegistry::open(&state, INSTALLATION, 4).unwrap();
    let registered = registry.acquire_active(proof).unwrap();
    assert!(registered.open_reader(None).unwrap().is_some());
    let durable = fs::read_to_string(state.join("sources.json")).unwrap();
    for private_value in [
        "/workspace",
        "session.jsonl",
        "task_started",
        "turn-1",
        "00000000-",
    ] {
        assert!(!durable.contains(private_value));
    }
}

#[test]
fn incomplete_baseline_and_invalid_context_wait_for_a_new_verified_start() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    let old = event("task_started");
    fs::write(
        &path,
        [header(false, 1), old[..old.len() - 2].to_vec()].concat(),
    )
    .unwrap();
    let mut probe =
        CodexActivityProbe::arm(qualified(root.path(), CaptureProfile::CodexCliJsonlV1)).unwrap();
    assert_eq!(poll(&mut probe), ActivityPoll::Waiting);
    append(&path, &old[old.len() - 2..]);
    assert_eq!(poll(&mut probe), ActivityPoll::Waiting);
    append(&path, b"invalid json\n");
    append(&path, &event("task_started"));
    assert_eq!(poll(&mut probe), ActivityPoll::Waiting);
    append(
        &path,
        &record(json!({"type":"turn_context","payload":{"cwd":"/workspace/b/nested"}})),
    );
    append(&path, &event("task_started"));
    assert_eq!(poll(&mut probe), ActivityPoll::Active);
    assert_eq!(
        probe.proof().unwrap().unwrap().directory(),
        Path::new("/workspace/b/nested")
    );
}

#[test]
fn bounded_prefix_verification_detects_rewrites_before_reusing_activity() {
    use std::io::Seek as _;
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    let initial = header(false, 1);
    let large = record(json!({"message":"x".repeat(64 * 1024)}));
    let mut bytes = initial.clone();
    for _ in 0..96 {
        bytes.extend_from_slice(&large);
    }
    fs::write(&path, &bytes).unwrap();
    let mut probe =
        CodexActivityProbe::arm(qualified(root.path(), CaptureProfile::CodexCliJsonlV1)).unwrap();
    assert_eq!(probe.poll().unwrap(), ActivityPoll::Scanning);
    assert!(probe.proof().unwrap().is_none());
    assert_eq!(poll(&mut probe), ActivityPoll::Waiting);
    append(&path, &event("task_started"));
    assert_eq!(probe.poll().unwrap(), ActivityPoll::Verifying);
    assert!(probe.proof().unwrap().is_none());
    assert_eq!(poll(&mut probe), ActivityPoll::Active);
    let old_proof = probe.proof().unwrap().unwrap();
    let mut file = fs::OpenOptions::new().write(true).open(&path).unwrap();
    file.seek(std::io::SeekFrom::Start(initial.len() as u64 + 15))
        .unwrap();
    file.write_all(b"y").unwrap();
    append(&path, &event("task_complete"));
    assert!(old_proof.revalidate().is_err());
    assert_eq!(probe.poll().unwrap(), ActivityPoll::Verifying);
    assert!(probe.poll().is_err());
    assert!(probe.requires_rearm());
    assert!(probe.proof().unwrap().is_none());
    assert!(probe.poll().is_err());
    let mut rearmed =
        CodexActivityProbe::arm(qualified(root.path(), CaptureProfile::CodexCliJsonlV1)).unwrap();
    assert_eq!(poll(&mut rearmed), ActivityPoll::Waiting);
    assert!(!rearmed.requires_rearm());
}

#[test]
fn retryable_header_failure_preserves_the_original_activity_boundary() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    let initial = [header(false, 1), event("task_started")].concat();
    fs::write(&path, &initial).unwrap();
    let mut probe =
        CodexActivityProbe::arm(qualified(root.path(), CaptureProfile::CodexCliJsonlV1)).unwrap();
    assert_eq!(poll(&mut probe), ActivityPoll::Waiting);
    assert!(!probe.requires_rearm());
    let changed =
        String::from_utf8(initial.clone())
            .unwrap()
            .replacen("session_meta", "session_meto", 1);
    fs::write(&path, changed).unwrap();
    assert!(probe.poll().is_err());
    assert!(!probe.requires_rearm());
    assert!(probe.proof().unwrap().is_none());
    // A new start after the original arm but before exact header recovery must
    // remain observable: a retryable error cannot silently move that boundary.
    let restored = [initial, event("task_started")].concat();
    fs::write(&path, restored).unwrap();
    assert_eq!(poll(&mut probe), ActivityPoll::Active);
    assert!(!probe.requires_rearm());
    assert!(probe.proof().unwrap().is_some());
}

#[test]
fn desktop_activity_partial_new_starts_and_append_races_are_independent() {
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    fs::write(&path, header(true, 1)).unwrap();
    let mut scanner =
        CodexDirectoryScanner::new(ApprovedCodexRoot::open(root.path()).unwrap().unwrap()).unwrap();
    assert!(
        QualifiedCodexSource::inspect(
            CaptureProfile::CodexCliJsonlV1,
            scanner.step(100, 100).unwrap().candidates.remove(0)
        )
        .is_err()
    );
    let mut probe =
        CodexActivityProbe::arm(qualified(root.path(), CaptureProfile::CodexDesktopJsonlV1))
            .unwrap();
    assert_eq!(poll(&mut probe), ActivityPoll::Waiting);
    let start = event("task_started");
    append(&path, &start[..start.len() - 2]);
    assert_eq!(poll(&mut probe), ActivityPoll::Waiting);
    append(&path, &start[start.len() - 2..]);
    assert_eq!(poll(&mut probe), ActivityPoll::Active);
    let stale = probe.proof().unwrap().unwrap();
    append(&path, &event("task_complete"));
    assert!(probe.proof().is_err());
    let state = private.path().join("registry");
    let registry = SourceRegistry::open(&state, INSTALLATION, 4).unwrap();
    let before = fs::read(state.join("sources.json")).unwrap();
    assert!(registry.acquire_active(stale).is_err());
    assert_eq!(fs::read(state.join("sources.json")).unwrap(), before);
    assert_eq!(fs::read_dir(&state).unwrap().count(), 2);
    assert_eq!(poll(&mut probe), ActivityPoll::Active);
    drop(
        registry
            .acquire_active(probe.proof().unwrap().unwrap())
            .unwrap(),
    );
    assert!(
        registry
            .acquire_admitted(qualified(root.path(), CaptureProfile::CodexDesktopJsonlV1))
            .unwrap()
            .is_some()
    );
}

#[test]
fn generic_registration_cannot_resume_history_and_new_sessions_need_new_activity() {
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let state = private.path().join("registry");
    let path = root.path().join("session.jsonl");
    fs::write(&path, header(false, 1)).unwrap();
    let registry = SourceRegistry::open(&state, INSTALLATION, 4).unwrap();
    drop(
        registry
            .acquire(qualified(root.path(), CaptureProfile::CodexCliJsonlV1))
            .unwrap(),
    );
    let legacy = fs::read(state.join("sources.json")).unwrap();
    assert!(!String::from_utf8_lossy(&legacy).contains("admitted_sessions"));
    assert!(
        registry
            .acquire_admitted(qualified(root.path(), CaptureProfile::CodexCliJsonlV1))
            .unwrap()
            .is_none()
    );
    assert_eq!(fs::read(state.join("sources.json")).unwrap(), legacy);
    let mut probe =
        CodexActivityProbe::arm(qualified(root.path(), CaptureProfile::CodexCliJsonlV1)).unwrap();
    assert_eq!(poll(&mut probe), ActivityPoll::Waiting);
    append(&path, &event("task_started"));
    assert_eq!(poll(&mut probe), ActivityPoll::Active);
    let source = registry
        .acquire_active(probe.proof().unwrap().unwrap())
        .unwrap()
        .source_instance_id();
    fs::write(&path, header(false, 2)).unwrap();
    assert!(
        registry
            .acquire_admitted(qualified(root.path(), CaptureProfile::CodexCliJsonlV1))
            .unwrap()
            .is_none()
    );
    let mut new =
        CodexActivityProbe::arm(qualified(root.path(), CaptureProfile::CodexCliJsonlV1)).unwrap();
    assert_eq!(poll(&mut new), ActivityPoll::Waiting);
    append(&path, &event("task_started"));
    assert_eq!(poll(&mut new), ActivityPoll::Active);
    assert_eq!(
        registry
            .acquire_active(new.proof().unwrap().unwrap())
            .unwrap()
            .source_instance_id(),
        source
    );
}

#[test]
fn admitted_offline_tail_resumes_in_a_fresh_process_after_sanitized_progress() {
    use jj_agent::{IntegrationId, NormalizerProfileV1};
    use jj_store_capture::{agent_capture::AgentCaptureSpool, codex_transcript::TranscriptPoll};
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let state = private.path().join("registry");
    let spool_path = private.path().join("spool");
    let path = root.path().join("session.jsonl");
    fs::write(&path, header(false, 1)).unwrap();
    let mut probe =
        CodexActivityProbe::arm(qualified(root.path(), CaptureProfile::CodexCliJsonlV1)).unwrap();
    assert_eq!(poll(&mut probe), ActivityPoll::Waiting);
    append(&path, &event("task_started"));
    assert_eq!(poll(&mut probe), ActivityPoll::Active);
    let registry = SourceRegistry::open(&state, INSTALLATION, 4).unwrap();
    let registered = registry
        .acquire_active(probe.proof().unwrap().unwrap())
        .unwrap();
    let source = registered.source_instance_id();
    let mut reader = registered.open_reader(None).unwrap().unwrap();
    let TranscriptPoll::Ready(batch) = reader.poll().unwrap() else {
        panic!("backfill")
    };
    assert_eq!(batch.records[0].sequence, 1);
    assert_eq!(batch.records.len(), 2);
    let mut spool = AgentCaptureSpool::open(&spool_path, 4 * 1024 * 1024).unwrap();
    let ack = spool
        .enqueue_from_reader(
            batch
                .sanitize_omitted(
                    jj_store_model::RepositoryId::new([1; 16]),
                    &NormalizerProfileV1::codex_0_156_1(),
                )
                .unwrap(),
        )
        .unwrap();
    reader.acknowledge(&ack).unwrap();
    assert_eq!(
        spool
            .latest_reader_progress(IntegrationId::Codex, source)
            .unwrap()
            .unwrap()
            .last_sequence,
        2
    );
    drop(reader);
    drop(spool);
    drop(registry);
    append(&path, &event("task_complete"));
    append(&path, &record(json!({"message":"offline final flush"})));
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "offline_admission_process_probe"])
        .env("JJ_STORE_ACTIVITY_ROOT", root.path())
        .env("JJ_STORE_ACTIVITY_STATE", private.path())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{} {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    // Same native session may rotate without a new start; the channel remains stable.
    fs::rename(&path, root.path().join("retained")).unwrap();
    fs::write(&path, header(false, 1)).unwrap();
    let reopened = SourceRegistry::open_existing(&state, INSTALLATION, 4)
        .unwrap()
        .unwrap();
    assert_eq!(
        reopened
            .acquire_admitted(qualified(root.path(), CaptureProfile::CodexCliJsonlV1))
            .unwrap()
            .unwrap()
            .source_instance_id(),
        source
    );
}

#[test]
fn offline_admission_process_probe() {
    use jj_agent::IntegrationId;
    use jj_store_capture::{agent_capture::AgentCaptureSpool, codex_transcript::TranscriptPoll};
    let Some(root) = std::env::var_os("JJ_STORE_ACTIVITY_ROOT") else {
        return;
    };
    let state = std::path::PathBuf::from(std::env::var_os("JJ_STORE_ACTIVITY_STATE").unwrap());
    let registry = SourceRegistry::open_existing(&state.join("registry"), INSTALLATION, 4)
        .unwrap()
        .unwrap();
    let registered = registry
        .acquire_admitted(qualified(Path::new(&root), CaptureProfile::CodexCliJsonlV1))
        .unwrap()
        .unwrap();
    let spool = AgentCaptureSpool::open(&state.join("spool"), 4 * 1024 * 1024).unwrap();
    let previous = spool
        .latest_reader_progress(IntegrationId::Codex, registered.source_instance_id())
        .unwrap()
        .unwrap();
    let mut reader = registered.open_reader(Some(previous)).unwrap().unwrap();
    for _ in 0..10 {
        if let TranscriptPoll::Ready(batch) = reader.poll().unwrap() {
            assert_eq!(batch.records[0].sequence, 3);
            assert_eq!(batch.records.len(), 2);
            return;
        }
    }
    panic!("offline tail unavailable")
}

#[test]
fn read_only_registry_lookup_rejects_missing_corrupt_unsafe_or_foreign_state() {
    use std::os::unix::fs::PermissionsExt as _;
    let private = tempfile::tempdir().unwrap();
    let state = private.path().join("registry");
    assert!(
        SourceRegistry::open_existing(&state, INSTALLATION, 1)
            .unwrap()
            .is_none()
    );
    assert!(!state.exists());
    drop(SourceRegistry::open(&state, INSTALLATION, 1).unwrap());
    let before = fs::read(state.join("sources.json")).unwrap();
    assert!(SourceRegistry::open_existing(&state, [9; 16], 1).is_err());
    assert_eq!(fs::read(state.join("sources.json")).unwrap(), before);
    fs::write(state.join(".tmpadmission"), b"abandoned").unwrap();
    fs::set_permissions(
        state.join(".tmpadmission"),
        fs::Permissions::from_mode(0o600),
    )
    .unwrap();
    assert!(
        SourceRegistry::open_existing(&state, INSTALLATION, 1)
            .unwrap()
            .is_some()
    );
    assert!(state.join(".tmpadmission").exists());
    fs::write(state.join(".tmpsecond"), b"abandoned").unwrap();
    fs::set_permissions(state.join(".tmpsecond"), fs::Permissions::from_mode(0o600)).unwrap();
    assert!(SourceRegistry::open_existing(&state, INSTALLATION, 1).is_err());
    assert!(state.join(".tmpadmission").exists());
    assert!(state.join(".tmpsecond").exists());
    fs::remove_file(state.join(".tmpsecond")).unwrap();
    fs::write(state.join("unexpected"), b"bad").unwrap();
    fs::set_permissions(state.join("unexpected"), fs::Permissions::from_mode(0o600)).unwrap();
    assert!(SourceRegistry::open_existing(&state, INSTALLATION, 1).is_err());
    fs::remove_file(state.join("unexpected")).unwrap();
    fs::set_permissions(
        state.join("sources.json"),
        fs::Permissions::from_mode(0o644),
    )
    .unwrap();
    assert!(SourceRegistry::open_existing(&state, INSTALLATION, 1).is_err());
    fs::set_permissions(
        state.join("sources.json"),
        fs::Permissions::from_mode(0o600),
    )
    .unwrap();
    fs::write(state.join("sources.json"), b"corrupt").unwrap();
    assert!(SourceRegistry::open_existing(&state, INSTALLATION, 1).is_err());
    fs::remove_file(state.join("sources.json")).unwrap();
    assert!(SourceRegistry::open_existing(&state, INSTALLATION, 1).is_err());
}

#[test]
fn oversized_records_foreign_headers_and_rebound_roots_withhold_activity() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    fs::write(&path, header(false, 1)).unwrap();
    let mut probe =
        CodexActivityProbe::arm(qualified(root.path(), CaptureProfile::CodexCliJsonlV1)).unwrap();
    assert_eq!(poll(&mut probe), ActivityPoll::Waiting);
    let mut oversized = vec![b'x'; 1024 * 1024 + 1];
    oversized.push(b'\n');
    append(&path, &oversized);
    append(&path, &event("task_started"));
    assert_eq!(probe.poll().unwrap(), ActivityPoll::Scanning);
    assert_eq!(poll(&mut probe), ActivityPoll::Waiting);
    append(
        &path,
        &record(json!({"type":"turn_context","payload":{"cwd":"/workspace/a"}})),
    );
    append(&path, &header(false, 2));
    append(
        &path,
        &record(json!({"type":"turn_context","payload":{"cwd":"/workspace/a"}})),
    );
    append(&path, &event("task_started"));
    assert_eq!(poll(&mut probe), ActivityPoll::Waiting);
    assert!(probe.proof().unwrap().is_none());
    let retained = root.path().with_extension("retained-activity");
    fs::rename(root.path(), &retained).unwrap();
    fs::create_dir(root.path()).unwrap();
    fs::write(&path, header(false, 1)).unwrap();
    assert!(probe.poll().is_err());
    assert!(probe.proof().unwrap().is_none());
    fs::remove_dir_all(&retained).unwrap();
}

#[test]
fn historical_record_budget_and_changes_during_arming_do_not_assert_activity() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    let mut bytes = header(false, 1);
    for _ in 0..300 {
        bytes.extend_from_slice(&event("task_started"));
    }
    fs::write(&path, bytes).unwrap();
    let mut probe =
        CodexActivityProbe::arm(qualified(root.path(), CaptureProfile::CodexCliJsonlV1)).unwrap();
    append(&path, &event("task_started"));
    assert_eq!(probe.poll().unwrap(), ActivityPoll::Scanning);
    assert!(probe.proof().unwrap().is_none());
    assert_eq!(poll(&mut probe), ActivityPoll::Waiting);
    append(&path, &event("task_started"));
    assert_eq!(probe.poll().unwrap(), ActivityPoll::Active);
}
