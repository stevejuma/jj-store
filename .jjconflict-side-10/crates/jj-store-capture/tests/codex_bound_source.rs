use jj_agent::SourceInstanceId;
use jj_store_capture::{
    codex_discovery::{ApprovedCodexRoot, CodexDirectoryScanner, CodexSourceCandidate},
    codex_source::BoundCodexTranscriptReader,
    codex_transcript::TranscriptPoll,
    profile::CaptureProfile,
};
use serde_json::json;
use std::{fs, path::Path};
const SOURCE: SourceInstanceId = SourceInstanceId::new([7; 16]);
fn header(desktop: bool) -> Vec<u8> {
    let mut bytes = serde_json::to_vec(&json!({"type":"session_meta","payload":{
        "id":"00000000-0000-0000-0000-000000000001","cwd":"/workspace/project",
        "cli_version":if desktop { "0.159.2" } else { "0.156.1" },
        "source":if desktop { "vscode" } else { "cli" },
        "originator":if desktop { "Codex Desktop" } else { "codex_cli_rs" },
    }}))
    .unwrap();
    bytes.push(b'\n');
    bytes
}
fn discover(path: &Path) -> CodexSourceCandidate {
    let mut scanner =
        CodexDirectoryScanner::new(ApprovedCodexRoot::open(path).unwrap().unwrap()).unwrap();
    let mut step = scanner.step(10, 10).unwrap();
    assert!(step.complete);
    assert_eq!(step.candidates.len(), 1);
    step.candidates.pop().unwrap()
}

#[test]
fn discovery_candidate_qualifies_exact_surface_before_any_source_poll() {
    for (desktop, profile, other) in [
        (
            false,
            CaptureProfile::CodexCliJsonlV1,
            CaptureProfile::CodexDesktopJsonlV1,
        ),
        (
            true,
            CaptureProfile::CodexDesktopJsonlV1,
            CaptureProfile::CodexCliJsonlV1,
        ),
    ] {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("misleading-name.jsonl");
        fs::write(&path, header(desktop)).unwrap();
        assert!(
            BoundCodexTranscriptReader::open(other, discover(root.path()), SOURCE, None).is_err()
        );
        let mut reader =
            BoundCodexTranscriptReader::open(profile, discover(root.path()), SOURCE, None)
                .unwrap()
                .unwrap();
        assert_eq!(reader.directory(), Path::new("/workspace/project"));
        assert_eq!(
            reader.native_session_key(),
            "00000000-0000-0000-0000-000000000001"
        );
        assert!(reader.revalidate().unwrap());
        let TranscriptPoll::Ready(batch) = reader.poll().unwrap() else {
            panic!("header")
        };
        assert_eq!(batch.records.len(), 1);
        assert_eq!(batch.records[0].bytes.as_ref().unwrap(), &header(desktop));
        let TranscriptPoll::Ready(retry) = reader.poll().unwrap() else {
            panic!("retry")
        };
        assert_eq!(retry.batch_id, batch.batch_id);
    }
}

#[test]
fn replaced_candidate_is_unavailable_before_header_inspection() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    fs::write(&path, header(false)).unwrap();
    let candidate = discover(root.path());
    fs::rename(&path, root.path().join("retained.old")).unwrap();
    fs::write(&path, b"private replacement is not a valid header\n").unwrap();
    assert!(
        BoundCodexTranscriptReader::open(CaptureProfile::CodexCliJsonlV1, candidate, SOURCE, None)
            .unwrap()
            .is_none()
    );
}

#[test]
fn bound_poll_withholds_replaced_file_even_with_a_pending_exact_batch() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    fs::write(&path, header(false)).unwrap();
    let mut reader = BoundCodexTranscriptReader::open(
        CaptureProfile::CodexCliJsonlV1,
        discover(root.path()),
        SOURCE,
        None,
    )
    .unwrap()
    .unwrap();
    let TranscriptPoll::Ready(_) = reader.poll().unwrap() else {
        panic!("pending header")
    };
    fs::rename(&path, root.path().join("retained.old")).unwrap();
    fs::write(&path, header(false)).unwrap();
    assert!(!reader.revalidate().unwrap());
    assert_eq!(
        reader.poll().err().unwrap().to_string(),
        "approved Codex source unavailable"
    );
}

#[cfg(unix)]
#[test]
fn bound_poll_withholds_rebound_root_and_symlinked_file() {
    use std::os::unix::fs::symlink;
    let root = tempfile::tempdir().unwrap();
    let approved = root.path().join("approved");
    let outside = root.path().join("outside");
    fs::create_dir_all(&approved).unwrap();
    fs::create_dir_all(&outside).unwrap();
    let path = approved.join("session.jsonl");
    fs::write(&path, header(false)).unwrap();
    let mut reader = BoundCodexTranscriptReader::open(
        CaptureProfile::CodexCliJsonlV1,
        discover(&approved),
        SOURCE,
        None,
    )
    .unwrap()
    .unwrap();
    fs::rename(&approved, root.path().join("retained")).unwrap();
    symlink(&outside, &approved).unwrap();
    assert_eq!(
        reader.poll().err().unwrap().to_string(),
        "approved Codex source unavailable"
    );
    fs::remove_file(&approved).unwrap();
    fs::rename(root.path().join("retained"), &approved).unwrap();
    fs::rename(&path, outside.join("session.jsonl")).unwrap();
    symlink(outside.join("session.jsonl"), &path).unwrap();
    assert_eq!(
        reader.poll().err().unwrap().to_string(),
        "approved Codex source unavailable"
    );
}

#[test]
fn durable_pending_ack_survives_unavailability_and_rediscovery_preserves_generation_progress() {
    use jj_agent::*;
    use jj_store_capture::{agent_capture::AgentCaptureSpool, reader_progress::ReaderCapture};
    use jj_store_model::RepositoryId;
    let root = tempfile::tempdir().unwrap();
    let state = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    fs::write(&path, header(false)).unwrap();
    let profile = CaptureProfile::CodexCliJsonlV1;
    let mut reader = BoundCodexTranscriptReader::open(profile, discover(root.path()), SOURCE, None)
        .unwrap()
        .unwrap();
    let TranscriptPoll::Ready(raw) = reader.poll().unwrap() else {
        panic!("header")
    };
    let mut spool = AgentCaptureSpool::open(state.path(), 4 * 1024 * 1024).unwrap();
    let ack = spool
        .enqueue_from_reader(ReaderCapture {
            repository: RepositoryId::new([1; 16]),
            profile: NormalizerProfileV1::codex_0_156_1(),
            cursor: raw.cursor.clone(),
            metadata: raw.metadata.clone(),
            thread_binding: None,
            batch: SanitizedObservationBatchV1 {
                schema_version: 1,
                batch_id: raw.batch_id,
                integration: IntegrationId::Codex,
                source_instance_id: SOURCE,
                sanitizer_version: "test-sanitized-omission/v1".into(),
                representation: SanitizedRepresentation::SanitizedNative,
                observations: vec![SanitizedObservationV1 {
                    observation_id: ObservationId::new([1; 16]),
                    source_sequence: 1,
                    occurred_at_millis: None,
                    source_kind: SourceKind::Transcript,
                    source_trust: SourceTrust::LocalTranscript,
                    payload: SanitizedPayloadV1::new(
                        SanitizedMediaType::Json,
                        br#"{"capture_omitted":true}"#,
                    )
                    .unwrap(),
                }],
            },
        })
        .unwrap();
    fs::rename(&path, root.path().join("retained.old")).unwrap();
    fs::write(&path, header(false)).unwrap();
    assert!(reader.poll().is_err());
    reader.acknowledge(&ack).unwrap();
    let previous = spool
        .reader_progress(RepositoryId::new([1; 16]), IntegrationId::Codex, SOURCE, 1)
        .unwrap()
        .unwrap();
    drop(reader);
    let mut resumed =
        BoundCodexTranscriptReader::open(profile, discover(root.path()), SOURCE, Some(previous))
            .unwrap()
            .unwrap();
    assert_eq!(resumed.generation(), 2);
    let TranscriptPoll::Ready(next) = resumed.poll().unwrap() else {
        panic!("rediscovered header")
    };
    assert_eq!(next.records[0].sequence, 2);
    assert_ne!(
        next.metadata.generation_digest,
        raw.metadata.generation_digest
    );
    assert_ne!(next.batch_id, raw.batch_id);
}
