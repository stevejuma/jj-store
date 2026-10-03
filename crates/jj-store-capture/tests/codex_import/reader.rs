use super::*;
use jj_agent::{
    CaptureBatchId, CaptureCursorV1, IntegrationId, SourceInstanceId, SourcePositionV1,
};
use jj_store_capture::reader_progress::{ReaderCaptureMetadataV1, ReaderProgressV1};
use jj_store_model::RepositoryId;
use sha2::{Digest as _, Sha256};
fn progress(path: &Path) -> ReaderProgressV1 {
    let bytes = fs::read(path).unwrap();
    ReaderProgressV1 {
        schema_version: 1,
        repository: RepositoryId::new([1; 16]),
        metadata: ReaderCaptureMetadataV1 {
            schema_version: 1,
            generation_digest: qualified(path, CaptureProfile::CodexCliJsonlV1).generation_digest(),
            opencode_snapshot: None,
        },
        cursor: CaptureCursorV1 {
            schema_version: 1,
            integration: IntegrationId::Codex,
            source_instance_id: SourceInstanceId::new([2; 16]),
            generation: 1,
            parser_version: "codex-jsonl@0.156.1/v1".into(),
            native_session_key: None,
            position: SourcePositionV1::JsonLines {
                complete_byte_offset: bytes.len() as u64,
                prefix_sha256: Sha256::digest(bytes).into(),
            },
        },
        last_sequence: 1,
        last_batch: CaptureBatchId::new([3; 16]),
    }
}
#[test]
fn accepted_probe_rejects_a_retained_cursor_from_different_bytes_of_the_same_file() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    header(&path, false);
    append(
        &path,
        json!({"type":"event_msg","payload":{"text":"BEFORE"}}),
    );
    let old = progress(&path);
    let mut probe = select(&path);
    let accepted = ready(&mut probe).binding();
    probe.retain_accepted(accepted).unwrap();
    assert!(probe.verify_reader_progress(Some(&old)).unwrap());
    fs::write(
        &path,
        fs::read_to_string(&path)
            .unwrap()
            .replace("BEFORE", "AFTER!"),
    )
    .unwrap();
    let mut probe = select(&path);
    ready(&mut probe);
    assert!(probe.verify_reader_progress(Some(&old)).is_err());
    assert!(
        probe
            .verify_reader_progress(Some(&progress(&path)))
            .unwrap()
    );
}
#[test]
fn cursor_verification_shares_prefix_budget_and_retains_selected_boundary_on_append() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    header(&path, false);
    for _ in 0..5 {
        append(&path, json!({"unknown":"x".repeat(1024*1024)}));
    }
    let prior = progress(&path);
    let mut probe = select(&path);
    assert!(matches!(probe.poll().unwrap(), CodexImportPoll::Verifying));
    let CodexImportPoll::Ready(snapshot) = probe.poll().unwrap() else {
        panic!("second prefix step")
    };
    let accepted = snapshot.binding();
    probe.retain_accepted(accepted).unwrap();
    assert!(
        !probe.verify_reader_progress(Some(&prior)).unwrap(),
        "two scans escaped the shared four-MiB step budget"
    );
    assert!(matches!(probe.poll().unwrap(), CodexImportPoll::Ready(_)));
    assert!(probe.verify_reader_progress(Some(&prior)).unwrap());
    append(&path, json!({"unknown":"later tail"}));
    assert!(matches!(
        probe.poll().unwrap(),
        CodexImportPoll::ChangedDuringRead
    ));
    assert_eq!(ready(&mut probe).binding(), accepted);
    while !probe.verify_reader_progress(Some(&prior)).unwrap() {
        probe.poll().unwrap();
    }
}

#[test]
fn same_snapshot_reader_verification_extends_prefix_without_rescanning_earlier_bytes() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    header(&path, false);
    for _ in 0..3 {
        append(&path, json!({"unknown":"x".repeat(1024*1024)}));
    }
    let earlier_bytes = fs::read(&path).unwrap();
    for _ in 0..2 {
        append(&path, json!({"unknown":"x".repeat(1024*1024)}));
    }
    let full = progress(&path);
    let mut earlier = full.clone();
    earlier.cursor.position = SourcePositionV1::JsonLines {
        complete_byte_offset: earlier_bytes.len() as u64,
        prefix_sha256: Sha256::digest(earlier_bytes).into(),
    };
    let mut probe = select(&path);
    ready(&mut probe);
    probe.poll().unwrap();
    assert!(probe.verify_reader_progress(Some(&earlier)).unwrap());
    probe.poll().unwrap();
    assert!(
        probe.verify_reader_progress(Some(&full)).unwrap(),
        "same-snapshot advancement rescanned the verified prefix and starved publication"
    );
    let SourcePositionV1::JsonLines { prefix_sha256, .. } = &mut earlier.cursor.position else {
        unreachable!()
    };
    *prefix_sha256 = [9; 32];
    probe.poll().unwrap();
    assert!(probe.verify_reader_progress(Some(&earlier)).is_err());
}
