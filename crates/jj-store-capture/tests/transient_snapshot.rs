//! Headless snapshot acceptance; no spool, repository binding or acknowledgement.
#![cfg(unix)]
use jj_agent::{SourceInstanceId, SourcePositionV1};
use jj_store_capture::{
    codex_discovery::{ApprovedCodexRoot, CodexDirectoryScanner, CodexSourceCandidate},
    codex_transcript::{CodexSnapshotReader, CodexTranscriptReader, SnapshotPoll, TranscriptPoll},
    profile::CaptureProfile,
    transcript_sanitization::TranscriptSanitizer,
};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{fs, io::Write, path::Path};

const SOURCE: SourceInstanceId = SourceInstanceId::new([7; 16]);
const PROFILE: CaptureProfile = CaptureProfile::CodexCliJsonlV1;
fn fixture(path: &Path, count: usize, desktop: bool) -> Vec<u8> {
    let mut bytes = format!(
        "{}\n",
        json!({"type":"session_meta","payload":{
            "id":"12345678-1234-1234-1234-123456789abc","cwd":"/workspace/project",
            "cli_version":if desktop {"0.159.2"} else {"0.156.1"},
            "source":if desktop {"vscode"} else {"cli"},
            "originator":if desktop {"Codex Desktop"} else {"codex_cli_rs"},
        }})
    )
    .into_bytes();
    for index in 0..count {
        writeln!(&mut bytes, "{}", json!({"type":"response_item","payload":{
            "type":"message","id":format!("m-{index}"),"role":"assistant",
            "content":[{"type":"output_text","text":format!("private-secret /workspace/project/note {index}")}]
        }})).unwrap();
    }
    fs::write(path, &bytes).unwrap();
    bytes
}
fn discover(directory: &Path) -> CodexSourceCandidate {
    let mut scanner =
        CodexDirectoryScanner::new(ApprovedCodexRoot::open(directory).unwrap().unwrap()).unwrap();
    let mut step = scanner.step(4096, 256).unwrap();
    assert!(step.complete);
    assert_eq!(step.candidates.len(), 1);
    step.candidates.pop().unwrap()
}
fn open(directory: &Path) -> CodexSnapshotReader {
    CodexSnapshotReader::open(PROFILE, discover(directory), SOURCE, 64 * 1024 * 1024).unwrap()
}
#[test]
fn transient_snapshot_spans_batches_without_durable_ack_and_preserves_exact_bytes() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("rollout.jsonl");
    let bytes = fixture(&path, 700, false);
    let mut snapshot = open(root.path());
    let mut sequences = Vec::new();
    let mut batches = 0;
    let summary = loop {
        match snapshot.poll().unwrap() {
            SnapshotPoll::Ready(batch) => {
                batches += 1;
                sequences.extend(batch.records.iter().map(|r| r.sequence));
                assert!(batch.records.len() <= 256);
            }
            SnapshotPoll::Reading => {}
            SnapshotPoll::Complete(summary) => break summary,
        }
    };
    assert_eq!(batches, 3);
    assert_eq!(sequences, (1..=701).collect::<Vec<_>>());
    assert_eq!(summary.records, 701);
    assert_eq!(summary.complete_bytes, bytes.len() as u64);
    assert_eq!(summary.sha256, <[u8; 32]>::from(Sha256::digest(&bytes)));
    let SnapshotPoll::Complete(repeated) = snapshot.poll().unwrap() else {
        panic!("complete")
    };
    assert!(summary == repeated);
    assert_eq!(fs::read(&path).unwrap(), bytes);
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 1);
    assert_eq!(snapshot.directory(), Path::new("/workspace/project"));
    assert_eq!(
        snapshot.native_session_key(),
        "12345678-1234-1234-1234-123456789abc"
    );
}
#[test]
fn snapshot_progress_does_not_acknowledge_an_independent_durable_reader() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("rollout.jsonl");
    fixture(&path, 600, false);
    let mut durable = CodexTranscriptReader::open(PROFILE, &path, SOURCE, None).unwrap();
    let TranscriptPoll::Ready(first) = durable.poll().unwrap() else {
        panic!("pending")
    };
    let mut snapshot = open(root.path());
    loop {
        if matches!(snapshot.poll().unwrap(), SnapshotPoll::Complete(_)) {
            break;
        }
    }
    let TranscriptPoll::Ready(retry) = durable.poll().unwrap() else {
        panic!("pending")
    };
    assert_eq!(retry.batch_id, first.batch_id);
    assert_eq!(retry.cursor, first.cursor);
    assert!(matches!(
        retry.cursor.position,
        SourcePositionV1::JsonLines { .. }
    ));
}
#[test]
fn long_record_scan_yields_bounded_reading_steps_and_an_explicit_safe_omission() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("rollout.jsonl");
    fixture(&path, 0, false);
    let mut file = fs::OpenOptions::new().append(true).open(&path).unwrap();
    file.write_all(&vec![b'q'; 3 * 1024 * 1024]).unwrap();
    file.write_all(b"\n").unwrap();
    let mut snapshot = open(root.path());
    let mut reading = 0;
    let mut omitted = 0;
    let mut complete = false;
    for _ in 0..10 {
        match snapshot.poll().unwrap() {
            SnapshotPoll::Reading => reading += 1,
            SnapshotPoll::Ready(batch) => {
                omitted += batch.records.iter().filter(|r| r.bytes.is_none()).count()
            }
            SnapshotPoll::Complete(summary) => {
                assert_eq!(summary.records, 2);
                complete = true;
                break;
            }
        }
    }
    assert!(complete);
    assert!(reading > 0);
    assert_eq!(omitted, 1);
}
#[test]
fn complete_snapshot_requires_complete_tail_and_stays_failed_after_restoration() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("rollout.jsonl");
    let bytes = fixture(&path, 1, false);
    fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .unwrap()
        .write_all(b"{partial")
        .unwrap();
    let mut snapshot = open(root.path());
    assert!(matches!(snapshot.poll().unwrap(), SnapshotPoll::Ready(_)));
    assert!(snapshot.poll().is_err());
    fs::write(&path, bytes).unwrap();
    assert!(snapshot.poll().is_err());
}
#[test]
fn source_growth_rewrite_and_truncation_invalidate_the_whole_tentative_snapshot() {
    for change in ["append", "rewrite", "truncate"] {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("rollout.jsonl");
        let bytes = fixture(&path, 600, false);
        let mut snapshot = open(root.path());
        assert!(matches!(snapshot.poll().unwrap(), SnapshotPoll::Ready(_)));
        match change {
            "append" => fs::OpenOptions::new()
                .append(true)
                .open(&path)
                .unwrap()
                .write_all(b"{}\n")
                .unwrap(),
            "rewrite" => {
                let mut changed = bytes;
                let end = changed.len() - 3;
                changed[end] = b'X';
                fs::write(&path, changed).unwrap();
            }
            _ => fs::write(&path, b"short\n").unwrap(),
        }
        assert!(snapshot.poll().is_err(), "{change}");
    }
}
#[test]
fn replacement_and_symlink_rebinding_cannot_complete_a_prior_snapshot() {
    for symlink in [false, true] {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("rollout.jsonl");
        fixture(&path, 300, false);
        let mut snapshot = open(root.path());
        assert!(matches!(snapshot.poll().unwrap(), SnapshotPoll::Ready(_)));
        let retained = root.path().join("original");
        fs::rename(&path, &retained).unwrap();
        if symlink {
            std::os::unix::fs::symlink(retained, &path).unwrap();
        } else {
            fixture(&path, 300, false);
        }
        assert!(snapshot.poll().is_err());
    }
}
#[test]
fn explicit_bounds_and_exact_surface_are_checked_before_polling() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("rollout.jsonl");
    fixture(&path, 1, true);
    let candidate = discover(root.path());
    for bound in [0, 1, 64 * 1024 * 1024 + 1] {
        assert!(
            CodexSnapshotReader::open(
                CaptureProfile::CodexDesktopJsonlV1,
                candidate.clone(),
                SOURCE,
                bound
            )
            .is_err()
        );
    }
    assert!(
        CodexSnapshotReader::open(PROFILE, candidate.clone(), SOURCE, 64 * 1024 * 1024).is_err()
    );
    assert!(
        CodexSnapshotReader::open(
            CaptureProfile::CursorDesktopTranscriptV1,
            candidate.clone(),
            SOURCE,
            64 * 1024 * 1024
        )
        .is_err()
    );
    let mut accepted = CodexSnapshotReader::open(
        CaptureProfile::CodexDesktopJsonlV1,
        candidate,
        SOURCE,
        64 * 1024 * 1024,
    )
    .unwrap();
    assert!(matches!(accepted.poll().unwrap(), SnapshotPoll::Ready(_)));
    assert!(matches!(
        accepted.poll().unwrap(),
        SnapshotPoll::Complete(_)
    ));
}
#[test]
fn neutral_native_sanitization_requires_no_repository_and_matches_the_publication_adapter() {
    let root = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    let path = root.path().join("rollout.jsonl");
    fixture(&path, 1, false);
    let sanitizer = TranscriptSanitizer::new(
        Path::new("/workspace/project"),
        Path::new("/home/synthetic"),
        Path::new("/tmp/synthetic"),
        cache.path(),
    )
    .unwrap()
    .for_secrets(&["private-secret".into()])
    .unwrap();
    let mut snapshot = open(root.path());
    let SnapshotPoll::Ready(raw) = snapshot.poll().unwrap() else {
        panic!("records")
    };
    let sanitized = raw
        .sanitized_observations(&PROFILE.normalizer(), &sanitizer)
        .unwrap();
    let encoded = serde_json::to_vec(&sanitized).unwrap();
    // Domain payloads serialize byte arrays; inspect actual JSON payload bytes.
    let payloads = sanitized
        .observations
        .iter()
        .map(|o| String::from_utf8(o.payload.bytes().to_vec()).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(!payloads.contains("private-secret"));
    assert!(!payloads.contains("/workspace/project"));
    assert!(payloads.contains("$WORKSPACE"));
    // The existing adapter needs a real selection; only the equivalence test supplies one.
    let selected = jj_store_model::RepositoryId::new([8; 16]);
    let capture = raw
        .sanitize(selected, &PROFILE.normalizer(), &sanitizer)
        .unwrap();
    assert_eq!(serde_json::to_vec(&capture.batch).unwrap(), encoded);
    assert_eq!(capture.cursor, raw.cursor);
    assert_eq!(capture.metadata, raw.metadata);
}

#[test]
fn lapis_acp_cli_snapshot_retains_exact_version_and_source_surface_checks() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("rollout.jsonl");
    let bytes = fixture(&path, 2, false);
    let header_end = bytes.iter().position(|b| *b == b'\n').unwrap();
    let mut header: serde_json::Value = serde_json::from_slice(&bytes[..header_end]).unwrap();
    header["payload"]["source"] = json!("vscode");
    header["payload"]["originator"] = json!("lapis-acp");
    let mut accepted = format!("{header}\n").into_bytes();
    accepted.extend(&bytes[header_end + 1..]);
    fs::write(&path, &accepted).unwrap();
    let mut reader = open(root.path());
    let summary = loop {
        if let SnapshotPoll::Complete(summary) = reader.poll().unwrap() {
            break summary;
        }
    };
    assert_eq!(summary.records, 3);
    assert_eq!(summary.sha256, <[u8; 32]>::from(Sha256::digest(&accepted)));
    assert_eq!(fs::read(&path).unwrap(), accepted);
    for (originator, version) in [
        ("other-adapter", "0.156.1"),
        ("lapis-acp", "0.159.2"),
        ("Codex Desktop", "0.163.0-alpha.10.2"),
    ] {
        header["payload"]["originator"] = json!(originator);
        header["payload"]["cli_version"] = json!(version);
        fs::write(&path, format!("{header}\n")).unwrap();
        assert!(
            CodexSnapshotReader::open(PROFILE, discover(root.path()), SOURCE, 64 * 1024 * 1024)
                .is_err()
        );
    }
}
