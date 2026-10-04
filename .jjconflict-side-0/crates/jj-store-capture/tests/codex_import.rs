#![cfg(unix)]
use jj_store_capture::{
    codex_discovery::{ApprovedCodexRoot, CodexDirectoryScanner},
    codex_import::{CodexImportBinding, CodexImportPoll, CodexImportProbe, CodexImportSnapshot},
    profile::CaptureProfile,
    source_registry::{QualifiedCodexSource, SourceRegistry},
};
use serde_json::{Value, json};
use std::{fs, io::Write as _, path::Path};
#[path = "codex_import/boundaries.rs"]
mod boundaries;
#[path = "codex_import/identity.rs"]
mod identity;
#[path = "codex_import/reader.rs"]
mod reader;
fn append(path: &Path, value: Value) {
    writeln!(
        fs::OpenOptions::new().append(true).open(path).unwrap(),
        "{value}"
    )
    .unwrap();
}
fn header(path: &Path, desktop: bool) {
    let profile = if desktop {
        CaptureProfile::CodexDesktopJsonlV1
    } else {
        CaptureProfile::CodexCliJsonlV1
    };
    fs::write(path, format!("{}\n", json!({"type":"session_meta", "payload":{
            "id":"00000000-0000-0000-0000-000000000001", "cwd":"/workspace/a", "cli_version":profile.product_version(),
        "source":if desktop {"vscode"} else {"cli"}, "originator":if desktop {"Codex Desktop"} else {"codex_cli_rs"}
    }}))).unwrap();
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
fn select(path: &Path) -> CodexImportProbe {
    CodexImportProbe::select(qualified(path, CaptureProfile::CodexCliJsonlV1)).unwrap()
}
fn ready(probe: &mut CodexImportProbe) -> CodexImportSnapshot {
    for _ in 0..20 {
        match probe.poll().unwrap() {
            CodexImportPoll::Ready(snapshot) => return snapshot,
            CodexImportPoll::Verifying | CodexImportPoll::ChangedDuringRead => {}
        }
    }
    panic!("bounded fixture did not settle");
}

#[test]
fn inactive_preparation_selects_complete_history_without_native_enrollment_or_raw_output() {
    let root = tempfile::tempdir().unwrap();
    let transcripts = root.path().join("transcripts");
    fs::create_dir(&transcripts).unwrap();
    let path = transcripts.join("session.jsonl");
    header(&path, false);
    append(
        &path,
        json!({"type":"event_msg","payload":{"type":"task_started","text":"private inactive body"}}),
    );
    append(
        &path,
        json!({"type":"turn_context","payload":{"cwd":"/workspace/b"}}),
    );
    let complete = fs::metadata(&path).unwrap().len();
    fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .unwrap()
        .write_all(br#"{"type":"turn_context","payload":{"cwd":"/private/partial"#)
        .unwrap();
    let sources_path = root.path().join("sources");
    let sources = SourceRegistry::open(&sources_path, [1; 16], 4).unwrap();
    let before = fs::read(sources_path.join("sources.json")).unwrap();
    let source = qualified(&path, CaptureProfile::CodexCliJsonlV1);
    let mut probe = CodexImportProbe::select(source.clone()).unwrap();
    assert!(!probe.is_compact() && !probe.revalidate().unwrap());
    let snapshot = ready(&mut probe);
    assert_eq!(snapshot.directory(), Some(Path::new("/workspace/b")));
    assert_eq!(snapshot.binding().complete_byte_offset, complete);
    assert_eq!(snapshot.binding().complete_records, 3);
    assert!(probe.is_compact() && probe.revalidate().unwrap());
    assert!(sources.lookup_admitted(&source).unwrap().is_none());
    assert_eq!(fs::read(sources_path.join("sources.json")).unwrap(), before);
    assert_eq!(fs::read_dir(sources_path).unwrap().count(), 2);
    let displayed = format!("{:?}", snapshot.binding());
    for private in [
        "private inactive body",
        "/workspace/b",
        "/private/partial",
        "00000000-0000-0000-0000-000000000001",
    ] {
        assert!(!displayed.contains(private));
    }
}

#[test]
fn partial_or_ambiguous_context_grants_no_current_directory() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    header(&path, false);
    append(
        &path,
        json!({"type":"turn_context","payload":{"cwd":"relative"}}),
    );
    let mut probe = select(&path);
    let snapshot = ready(&mut probe);
    assert!(snapshot.directory().is_none());
    assert_eq!(snapshot.prior_directory(), Some(Path::new("/workspace/a")));
    append(
        &path,
        json!({"type":"session_meta","payload":{"id":"foreign"}}),
    );
    append(
        &path,
        json!({"type":"turn_context","payload":{"cwd":"/foreign/session"}}),
    );
    let snapshot = ready(&mut probe);
    assert!(snapshot.directory().is_none());
    assert_eq!(snapshot.prior_directory(), Some(Path::new("/workspace/a")));
}
