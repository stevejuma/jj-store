#![cfg(unix)]
use jj_store_capture::{
    codex_discovery::{ApprovedCodexRoot, CodexDirectoryScanner},
    codex_transcript::TranscriptPoll,
    profile::CaptureProfile,
    source_registry::{QualifiedCodexSource, SourceRegistry},
};
use serde_json::json;
use std::{fs, io::Write as _, path::Path, sync::Arc};
#[path = "source_parking/reservations.rs"]
mod reservations;

fn write_header(path: &Path, id: &str) {
    fs::write(
        path,
        format!(
            "{}\n",
            json!({"type":"session_meta","payload":{
                "id":id,"cwd":"/workspace/project","cli_version":"0.156.1",
                "source":"cli","originator":"codex_cli_rs"
            }})
        ),
    )
    .unwrap();
}
fn qualified(root: &Path, path: &Path) -> QualifiedCodexSource {
    let mut scanner =
        CodexDirectoryScanner::new(ApprovedCodexRoot::open(root).unwrap().unwrap()).unwrap();
    let path = fs::canonicalize(path).unwrap();
    let candidate = scanner
        .step(100, 100)
        .unwrap()
        .candidates
        .into_iter()
        .find(|c| c.path() == path)
        .unwrap();
    QualifiedCodexSource::inspect(CaptureProfile::CodexCliJsonlV1, candidate)
        .unwrap()
        .unwrap()
}

#[test]
fn parked_raw_batch_survives_append_and_transfers_across_overlapping_roots() {
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let nested = root.path().join("nested");
    fs::create_dir(&nested).unwrap();
    let path = nested.join("session.jsonl");
    let alias = root.path().join("alias.jsonl");
    write_header(&path, "00000000-0000-0000-0000-000000000001");
    fs::hard_link(&path, &alias).unwrap();
    let registry = SourceRegistry::open(&private.path().join("registry"), [8; 16], 4).unwrap();
    let mut reader = registry
        .acquire(qualified(root.path(), &path))
        .unwrap()
        .open_reader(None)
        .unwrap()
        .unwrap();
    let TranscriptPoll::Ready(original) = reader.poll().unwrap() else {
        panic!("missing initial batch");
    };
    assert!(!reader.compact_idle());
    let mut parked = reader.park();
    assert!(!parked.compact_idle());
    let competing = registry.acquire(qualified(root.path(), &alias)).unwrap();
    assert!(registry.acquire(qualified(&nested, &path)).is_err());
    drop(competing);
    fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .unwrap()
        .write_all(b"{\"later\":true}\n")
        .unwrap();
    let acquired = registry.acquire(qualified(&nested, &path)).unwrap();
    let mut reader = parked.resume(acquired).unwrap();
    let TranscriptPoll::Ready(retained) = reader.poll().unwrap() else {
        panic!("missing retained batch");
    };
    assert_eq!(retained.batch_id, original.batch_id);
    assert_eq!(retained.cursor, original.cursor);
    assert!(Arc::ptr_eq(&original.records, &retained.records));
    assert!(registry.acquire(qualified(root.path(), &alias)).is_err());
    drop(reader);
    assert!(
        parked
            .resume(registry.acquire(qualified(root.path(), &alias)).unwrap())
            .is_err()
    );
    assert!(registry.acquire(qualified(root.path(), &path)).is_ok());
    let durable = fs::read_to_string(private.path().join("registry/sources.json")).unwrap();
    assert!(!durable.contains("/workspace/project"));
    assert!(!durable.contains("session_meta"));
    assert!(!durable.contains("later"));
}

#[test]
fn compact_partial_eof_retains_exact_record_after_later_append() {
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    write_header(&path, "00000000-0000-0000-0000-000000000001");
    let registry = SourceRegistry::open(&private.path().join("registry"), [8; 16], 4).unwrap();
    let mut reader = registry
        .acquire(qualified(root.path(), &path))
        .unwrap()
        .open_reader(None)
        .unwrap()
        .unwrap();
    let TranscriptPoll::Ready(batch) = reader.poll().unwrap() else {
        panic!("header batch");
    };
    reader
        .acknowledge(&jj_store_capture::agent_capture::DurableCaptureAck {
            batch_id: batch.batch_id,
            position: batch.cursor.position,
        })
        .unwrap();
    let incomplete = format!(
        "{{\"type\":\"response_item\",\"payload\":{{\"type\":\"message\",\"text\":\"{}",
        "x".repeat(900_000)
    );
    fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .unwrap()
        .write_all(incomplete.as_bytes())
        .unwrap();
    assert!(matches!(reader.poll().unwrap(), TranscriptPoll::Waiting));
    assert!(reader.compact_idle());
    let mut parked = reader.park();
    assert!(parked.compact_idle());
    assert!(parked.idle_unchanged().unwrap());
    let mut reader = parked
        .resume(registry.acquire(qualified(root.path(), &path)).unwrap())
        .unwrap();
    assert!(matches!(reader.poll().unwrap(), TranscriptPoll::Waiting));
    let mut parked = reader.park();
    fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .unwrap()
        .write_all(b"\"}}\n")
        .unwrap();
    assert!(!parked.idle_unchanged().unwrap());
    let mut reader = parked
        .resume(registry.acquire(qualified(root.path(), &path)).unwrap())
        .unwrap();
    let TranscriptPoll::Ready(batch) = reader.poll().unwrap() else {
        panic!("completed partial");
    };
    assert_eq!(batch.records.len(), 1);
    assert_eq!(
        batch.records[0].bytes.as_deref().unwrap(),
        format!("{incomplete}\"}}}}\n").as_bytes()
    );
    assert!(!reader.compact_idle());
}

#[test]
fn foreign_registered_channel_cannot_consume_a_parked_reader() {
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let path = root.path().join("original.jsonl");
    let foreign = root.path().join("foreign.jsonl");
    write_header(&path, "00000000-0000-0000-0000-000000000001");
    write_header(&foreign, "00000000-0000-0000-0000-000000000002");
    let registry = SourceRegistry::open(&private.path().join("registry"), [8; 16], 4).unwrap();
    let reader = registry
        .acquire(qualified(root.path(), &path))
        .unwrap()
        .open_reader(None)
        .unwrap()
        .unwrap();
    let expected = reader.source_instance_id();
    let mut parked = reader.park();
    assert!(
        parked
            .resume(registry.acquire(qualified(root.path(), &foreign)).unwrap())
            .is_err()
    );
    let reader = parked
        .resume(registry.acquire(qualified(root.path(), &path)).unwrap())
        .unwrap();
    assert_eq!(reader.source_instance_id(), expected);
}

#[test]
fn an_identical_replacement_or_changed_header_cannot_reuse_raw_state() {
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    let retained = root.path().join("old");
    write_header(&path, "00000000-0000-0000-0000-000000000001");
    let header = fs::read(&path).unwrap();
    let registry = SourceRegistry::open(&private.path().join("registry"), [8; 16], 4).unwrap();
    let reader = registry
        .acquire(qualified(root.path(), &path))
        .unwrap()
        .open_reader(None)
        .unwrap()
        .unwrap();
    let mut parked = reader.park();
    fs::rename(&path, &retained).unwrap();
    fs::copy(&retained, &path).unwrap();
    assert!(
        parked
            .resume(registry.acquire(qualified(root.path(), &path)).unwrap())
            .is_err()
    );
    fs::remove_file(&path).unwrap();
    fs::rename(&retained, &path).unwrap();
    write_header(&path, "00000000-0000-0000-0000-000000000002");
    assert!(
        parked
            .resume(registry.acquire(qualified(root.path(), &path)).unwrap())
            .is_err()
    );
    fs::write(&path, header).unwrap();
    assert!(
        parked
            .resume(registry.acquire(qualified(root.path(), &path)).unwrap())
            .is_ok()
    );
}

#[test]
fn independent_generation_advancement_withholds_an_exact_parked_raw_allocation() {
    use jj_store_capture::reader_progress::ReaderSourceProgress;
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    write_header(&path, "00000000-0000-0000-0000-000000000001");
    let registry = SourceRegistry::open(&private.path().join("registry"), [8; 16], 4).unwrap();
    let bounds = ReaderSourceProgress {
        progress: None,
        generation_floor: 0,
        sequence_floor: 0,
    };
    let mut reader = registry
        .acquire(qualified(root.path(), &path))
        .unwrap()
        .open_reader_at(&bounds)
        .unwrap()
        .unwrap();
    let TranscriptPoll::Ready(original) = reader.poll().unwrap() else {
        panic!("initial batch")
    };
    let mut parked = reader.park();
    let superseded = ReaderSourceProgress {
        progress: None,
        generation_floor: 2,
        sequence_floor: 100,
    };
    assert!(
        parked
            .resume_at(
                registry.acquire(qualified(root.path(), &path)).unwrap(),
                &superseded
            )
            .is_err()
    );
    assert!(!parked.compact_idle());
    // A failed allocation check releases fresh leases and retains the original frame.
    let mut reader = parked
        .resume_at(
            registry.acquire(qualified(root.path(), &path)).unwrap(),
            &bounds,
        )
        .unwrap();
    let TranscriptPoll::Ready(retained) = reader.poll().unwrap() else {
        panic!("retained batch")
    };
    assert_eq!(retained.batch_id, original.batch_id);
    assert_eq!(retained.cursor, original.cursor);
    assert!(Arc::ptr_eq(&retained.records, &original.records));
    assert_eq!(retained.records[0].sequence, original.records[0].sequence);
}
