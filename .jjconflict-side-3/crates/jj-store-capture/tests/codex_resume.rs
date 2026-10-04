#![cfg(unix)]
use jj_store_capture::{
    codex_activity::{ActivityPoll, CodexActivityProbe},
    codex_discovery::{ApprovedCodexRoot, CodexDirectoryScanner},
    codex_resume::{CodexResumePoll, CodexResumeProbe},
    profile::CaptureProfile,
    source_registry::{QualifiedCodexSource, SourceRegistry},
};
use serde_json::{Value, json};
use std::{fs, io::Write as _, path::Path, process::Command};
fn append(path: &Path, value: Value) {
    let mut file = fs::OpenOptions::new().append(true).open(path).unwrap();
    writeln!(file, "{value}").unwrap();
}
fn header(path: &Path) {
    fs::write(path,format!("{}\n",json!({"type":"session_meta","payload":{"id":"00000000-0000-0000-0000-000000000001","cwd":"/workspace/a","cli_version":"0.156.1","source":"cli","originator":"codex_cli_rs"}}))).unwrap();
}
fn qualified(path: &Path, profile: CaptureProfile) -> QualifiedCodexSource {
    let mut scanner = CodexDirectoryScanner::new(
        ApprovedCodexRoot::open(path.parent().unwrap())
            .unwrap()
            .unwrap(),
    )
    .unwrap();
    QualifiedCodexSource::inspect(
        profile,
        scanner.step(100, 100).unwrap().candidates.remove(0),
    )
    .unwrap()
    .unwrap()
}
fn probe(path: &Path) -> CodexResumeProbe {
    CodexResumeProbe::new(qualified(path, CaptureProfile::CodexCliJsonlV1)).unwrap()
}

#[test]
fn complete_context_only_and_sticky_unknown_history_have_separate_prior_anchors() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    header(&path);
    append(
        &path,
        json!({"type":"turn_context","payload":{"cwd":"/workspace/b"}}),
    );
    append(
        &path,
        json!({"type":"message","cwd":"/foreign/body","text":"untrusted body"}),
    );
    fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .unwrap()
        .write_all(br#"{"type":"turn_context","payload":{"cwd":"/partial"#)
        .unwrap();
    let mut current = probe(&path);
    let CodexResumePoll::Ready(directory) = current.poll().unwrap() else {
        panic!("small prefix")
    };
    assert_eq!(directory.directory(), Some(Path::new("/workspace/b")));
    assert!(current.revalidate().unwrap());
    fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .unwrap()
        .write_all(b"\n")
        .unwrap();
    assert!(!current.revalidate().unwrap());
    let CodexResumePoll::Ready(directory) = current.poll().unwrap() else {
        panic!("small prefix")
    };
    assert!(directory.directory().is_none());
    assert_eq!(directory.prior_directory(), Some(Path::new("/workspace/b")));
    append(
        &path,
        json!({"type":"session_meta","payload":{"id":"foreign"}}),
    );
    append(
        &path,
        json!({"type":"turn_context","payload":{"cwd":"/foreign/repeated-session"}}),
    );
    let CodexResumePoll::Ready(directory) = current.poll().unwrap() else {
        panic!("small prefix")
    };
    assert!(directory.directory().is_none());
    assert_eq!(directory.prior_directory(), Some(Path::new("/workspace/b")));
}

#[test]
fn bounded_snapshot_restarts_on_append_rewrite_and_rejects_replacement() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    header(&path);
    for _ in 0..7 {
        append(&path, json!({"padding":"x".repeat(800*1024)}));
    }
    append(
        &path,
        json!({"type":"turn_context","payload":{"cwd":"/workspace/b"}}),
    );
    let mut current = probe(&path);
    assert!(matches!(
        current.poll().unwrap(),
        CodexResumePoll::Verifying
    ));
    append(
        &path,
        json!({"type":"turn_context","payload":{"cwd":"/workspace/c"}}),
    );
    assert!(matches!(
        current.poll().unwrap(),
        CodexResumePoll::Verifying
    ));
    assert!(
        matches!(current.poll().unwrap(),CodexResumePoll::Ready(directory) if directory.directory() == Some(Path::new("/workspace/c")))
    );
    let bytes = fs::read(&path).unwrap();
    let mut text = String::from_utf8(bytes).unwrap();
    text = text.replace("/workspace/c", "/workspace/d");
    fs::write(&path, &text).unwrap();
    assert!(!current.revalidate().unwrap());
    assert!(matches!(
        current.poll().unwrap(),
        CodexResumePoll::Verifying
    ));
    assert!(
        matches!(current.poll().unwrap(),CodexResumePoll::Ready(directory) if directory.directory() == Some(Path::new("/workspace/d")))
    );
    fs::rename(&path, root.path().join("retained-old")).unwrap();
    fs::write(&path, text).unwrap();
    assert!(current.poll().is_err());
    assert!(current.revalidate().is_err());
}

#[test]
fn lookup_is_read_only_and_only_retained_native_session_markers_resume() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("transcripts/session.jsonl");
    fs::create_dir(path.parent().unwrap()).unwrap();
    header(&path);
    let state = root.path().join("sources");
    let sources = SourceRegistry::open(&state, [1; 16], 4).unwrap();
    let before = fs::read(state.join("sources.json")).unwrap();
    assert!(
        sources
            .lookup_admitted(&qualified(&path, CaptureProfile::CodexCliJsonlV1))
            .unwrap()
            .is_none()
    );
    assert_eq!(fs::read(state.join("sources.json")).unwrap(), before);
    assert_eq!(fs::read_dir(&state).unwrap().count(), 2);
    let generic = sources
        .acquire(qualified(&path, CaptureProfile::CodexCliJsonlV1))
        .unwrap();
    drop(generic);
    assert!(
        sources
            .lookup_admitted(&qualified(&path, CaptureProfile::CodexCliJsonlV1))
            .unwrap()
            .is_none()
    );
    let mut activity =
        CodexActivityProbe::arm(qualified(&path, CaptureProfile::CodexCliJsonlV1)).unwrap();
    assert_eq!(activity.poll().unwrap(), ActivityPoll::Waiting);
    append(
        &path,
        json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"t"}}),
    );
    assert_eq!(activity.poll().unwrap(), ActivityPoll::Active);
    let active = sources
        .acquire_active(activity.proof().unwrap().unwrap())
        .unwrap();
    let source = active.source_instance_id();
    drop(active);
    let before = fs::read(state.join("sources.json")).unwrap();
    let locks = fs::read_dir(&state).unwrap().count();
    assert_eq!(
        sources
            .lookup_admitted(&qualified(&path, CaptureProfile::CodexCliJsonlV1))
            .unwrap(),
        Some(source)
    );
    assert_eq!(fs::read(state.join("sources.json")).unwrap(), before);
    assert_eq!(fs::read_dir(&state).unwrap().count(), locks);
    let text = fs::read_to_string(&path).unwrap().replace(
        "00000000-0000-0000-0000-000000000001",
        "00000000-0000-0000-0000-000000000002",
    );
    fs::write(&path, text).unwrap();
    assert!(
        sources
            .lookup_admitted(&qualified(&path, CaptureProfile::CodexCliJsonlV1))
            .unwrap()
            .is_none()
    );
    assert_eq!(fs::read(state.join("sources.json")).unwrap(), before);
}

#[test]
fn desktop_snapshot_qualifies_independently_from_cli() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("desktop.jsonl");
    fs::write(&path,format!("{}\n",json!({"type":"session_meta","payload":{"id":"00000000-0000-0000-0000-000000000001","cwd":"/workspace/a","cli_version":"0.159.2","source":"vscode","originator":"Codex Desktop"}}))).unwrap();
    append(
        &path,
        json!({"type":"turn_context","payload":{"cwd":"/workspace/b"}}),
    );
    let mut scanner =
        CodexDirectoryScanner::new(ApprovedCodexRoot::open(root.path()).unwrap().unwrap()).unwrap();
    let candidate = scanner.step(100, 100).unwrap().candidates.remove(0);
    assert!(
        QualifiedCodexSource::inspect(CaptureProfile::CodexCliJsonlV1, candidate.clone()).is_err()
    );
    let qualified = QualifiedCodexSource::inspect(CaptureProfile::CodexDesktopJsonlV1, candidate)
        .unwrap()
        .unwrap();
    let mut current = CodexResumeProbe::new(qualified).unwrap();
    assert!(
        matches!(current.poll().unwrap(),CodexResumePoll::Ready(directory) if directory.directory() == Some(Path::new("/workspace/b")))
    );
}

#[test]
fn compact_eof_rebuilds_partial_context_after_append_and_same_length_rewrite() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    header(&path);
    let record = format!(
        "{}\n",
        json!({"type":"turn_context","payload":{"cwd":"/workspace/b","padding":"x".repeat(900 * 1024)}})
    );
    fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .unwrap()
        .write_all(&record.as_bytes()[..record.len() - 2])
        .unwrap();
    let mut current = probe(&path);
    assert!(!current.is_compact());
    assert!(
        matches!(current.poll().unwrap(), CodexResumePoll::Ready(context) if context.directory() == Some(Path::new("/workspace/a")))
    );
    assert!(current.is_compact());
    fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .unwrap()
        .write_all(&record.as_bytes()[record.len() - 2..])
        .unwrap();
    assert!(!current.revalidate().unwrap());
    assert!(
        matches!(current.poll().unwrap(), CodexResumePoll::Ready(context) if context.directory() == Some(Path::new("/workspace/b")))
    );
    assert!(current.is_compact());
    let text = fs::read_to_string(&path)
        .unwrap()
        .replace("/workspace/b", "/workspace/c");
    fs::write(&path, text).unwrap();
    assert!(!current.revalidate().unwrap());
    assert!(
        matches!(current.poll().unwrap(), CodexResumePoll::Ready(context) if context.directory() == Some(Path::new("/workspace/c")))
    );
    assert!(current.is_compact());
}

#[test]
fn fresh_process_reconstructs_current_complete_context_from_read_only_admitted_lookup() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("transcripts/session.jsonl");
    fs::create_dir(path.parent().unwrap()).unwrap();
    header(&path);
    let state = root.path().join("sources");
    let sources = SourceRegistry::open(&state, [1; 16], 4).unwrap();
    let mut activity =
        CodexActivityProbe::arm(qualified(&path, CaptureProfile::CodexCliJsonlV1)).unwrap();
    assert_eq!(activity.poll().unwrap(), ActivityPoll::Waiting);
    append(
        &path,
        json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"t"}}),
    );
    assert_eq!(activity.poll().unwrap(), ActivityPoll::Active);
    let active = sources
        .acquire_active(activity.proof().unwrap().unwrap())
        .unwrap();
    drop(active);
    append(
        &path,
        json!({"type":"turn_context","payload":{"cwd":"/workspace/b"}}),
    );
    append(
        &path,
        json!({"type":"event_msg","payload":{"type":"task_complete","turn_id":"t"}}),
    );
    let before = fs::read(state.join("sources.json")).unwrap();
    let locks = fs::read_dir(&state).unwrap().count();
    let output = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "resume_context_child", "--ignored"])
        .env("JJ_STORE_CODEX_RESUME_TEST_ROOT", root.path())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(fs::read(state.join("sources.json")).unwrap(), before);
    assert_eq!(fs::read_dir(&state).unwrap().count(), locks);
}

#[test]
#[ignore = "fresh-process fixture invoked by fresh_process_reconstructs_current_complete_context_from_read_only_admitted_lookup"]
fn resume_context_child() {
    let root =
        std::path::PathBuf::from(std::env::var_os("JJ_STORE_CODEX_RESUME_TEST_ROOT").unwrap());
    let sources = SourceRegistry::open_existing(&root.join("sources"), [1; 16], 4)
        .unwrap()
        .unwrap();
    let qualified = qualified(
        &root.join("transcripts/session.jsonl"),
        CaptureProfile::CodexCliJsonlV1,
    );
    assert!(sources.lookup_admitted(&qualified).unwrap().is_some());
    let mut current = CodexResumeProbe::new(qualified).unwrap();
    assert!(
        matches!(current.poll().unwrap(),CodexResumePoll::Ready(directory) if directory.directory() == Some(Path::new("/workspace/b")))
    );
    assert!(current.revalidate().unwrap());
}
