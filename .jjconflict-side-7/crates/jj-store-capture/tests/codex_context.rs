use jj_agent::*;
use jj_store_capture::{
    agent_capture::{AgentCaptureSpool, DurableCaptureAck},
    codex_transcript::{CodexTranscriptBatch, CodexTranscriptReader, TranscriptPoll},
    profile::CaptureProfile,
    reader_progress::{ReaderCapture, ReaderProgressV1},
};
use jj_store_model::RepositoryId;
use serde_json::json;
use std::{fs, io::Write as _, path::Path};
const SOURCE: SourceInstanceId = SourceInstanceId::new([7; 16]);
const A: RepositoryId = RepositoryId::new([1; 16]);
const B: RepositoryId = RepositoryId::new([2; 16]);
fn append(path: &Path, value: serde_json::Value) {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();
    file.write_all(&serde_json::to_vec(&value).unwrap())
        .unwrap();
    file.write_all(b"\n").unwrap();
}
fn header(path: &Path) {
    append(
        path,
        json!({"type":"session_meta","payload":{
            "id":"00000000-0000-0000-0000-000000000001","cwd":"/workspace/a",
            "cli_version":"0.156.1","source":"cli","originator":"codex_cli_rs"
        }}),
    );
}
fn reader(path: &Path, previous: Option<ReaderProgressV1>) -> CodexTranscriptReader {
    CodexTranscriptReader::open(CaptureProfile::CodexCliJsonlV1, path, SOURCE, previous).unwrap()
}
fn ready(reader: &mut CodexTranscriptReader) -> CodexTranscriptBatch {
    for _ in 0..100 {
        if let TranscriptPoll::Ready(batch) = reader.poll().unwrap() {
            return *batch;
        }
    }
    panic!("fixture did not become ready")
}
fn omitted(batch: &CodexTranscriptBatch, repository: RepositoryId) -> ReaderCapture {
    ReaderCapture {
        repository,
        profile: NormalizerProfileV1::codex_0_156_1(),
        cursor: batch.cursor.clone(),
        metadata: batch.metadata.clone(),
        thread_binding: None,
        batch: SanitizedObservationBatchV1 {
            schema_version: 1,
            batch_id: batch.batch_id,
            integration: IntegrationId::Codex,
            source_instance_id: SOURCE,
            sanitizer_version: "test-omission/v1".into(),
            representation: SanitizedRepresentation::SanitizedNative,
            observations: batch
                .records
                .iter()
                .map(|r| SanitizedObservationV1 {
                    observation_id: ObservationId::new((r.sequence as u128).to_be_bytes()),
                    source_sequence: r.sequence,
                    occurred_at_millis: None,
                    source_kind: SourceKind::Transcript,
                    source_trust: SourceTrust::LocalTranscript,
                    payload: SanitizedPayloadV1::new(
                        SanitizedMediaType::Json,
                        br#"{"capture_omitted":true}"#,
                    )
                    .unwrap(),
                })
                .collect(),
        },
    }
}
fn ack(reader: &mut CodexTranscriptReader, batch: &CodexTranscriptBatch) {
    reader
        .acknowledge(&DurableCaptureAck {
            batch_id: batch.batch_id,
            position: batch.cursor.position.clone(),
        })
        .unwrap();
}

#[test]
fn directory_changes_partition_exact_batches_and_global_resume_crosses_repositories() {
    let root = tempfile::tempdir().unwrap();
    let state = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    header(&path);
    append(&path, json!({"message":"a"}));
    let first_end = fs::metadata(&path).unwrap().len();
    append(
        &path,
        json!({"type":"turn_context","payload":{"cwd":"/workspace/b/subdir"}}),
    );
    append(&path, json!({"message":"b"}));
    let second_end = fs::metadata(&path).unwrap().len();
    append(
        &path,
        json!({"type":"turn_context","payload":{"cwd":"/workspace/a"}}),
    );
    append(&path, json!({"message":"a-again"}));
    let mut source = reader(&path, None);
    let mut spool = AgentCaptureSpool::open(state.path(), 4 * 1024 * 1024).unwrap();
    for (directory, repository, end, sequence) in [
        ("/workspace/a", A, first_end, 1),
        ("/workspace/b/subdir", B, second_end, 3),
    ] {
        let batch = ready(&mut source);
        assert_eq!(batch.directory(), Some(Path::new(directory)));
        assert_eq!(batch.records.len(), 2);
        assert_eq!(batch.records[0].sequence, sequence);
        assert_eq!(batch.batch_id, ready(&mut source).batch_id);
        assert!(
            matches!(batch.cursor.position, SourcePositionV1::JsonLines {complete_byte_offset,..} if complete_byte_offset == end)
        );
        let durable = spool
            .enqueue_from_reader(omitted(&batch, repository))
            .unwrap();
        source.acknowledge(&durable).unwrap();
    }
    let previous = spool
        .latest_reader_progress(IntegrationId::Codex, SOURCE)
        .unwrap()
        .unwrap();
    assert_eq!(previous.repository, B);
    assert_eq!(previous.last_sequence, 4);
    drop(source);
    let mut resumed = reader(&path, Some(previous));
    let last = ready(&mut resumed);
    assert_eq!(last.directory(), Some(Path::new("/workspace/a")));
    assert_eq!(last.records[0].sequence, 5);
    let durable = spool.enqueue_from_reader(omitted(&last, A)).unwrap();
    resumed.acknowledge(&durable).unwrap();
    assert_eq!(
        spool
            .reader_progress(B, IntegrationId::Codex, SOURCE, 1)
            .unwrap()
            .unwrap()
            .last_sequence,
        4
    );
    assert_eq!(
        spool
            .latest_reader_progress(IntegrationId::Codex, SOURCE)
            .unwrap()
            .unwrap()
            .last_sequence,
        6
    );
}

#[test]
fn malformed_context_withholds_later_content_until_explicit_valid_context() {
    for invalid in [
        json!(null),
        json!("relative"),
        json!("/"),
        json!("/private/../other"),
        json!("/private/\nunsafe"),
    ] {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("session.jsonl");
        header(&path);
        append(
            &path,
            json!({"type":"turn_context","payload":{"cwd":invalid}}),
        );
        append(&path, json!({"private":"unknown repository content"}));
        append(
            &path,
            json!({"type":"turn_context","payload":{"cwd":"/workspace/b"}}),
        );
        append(&path, json!({"message":"safe later record"}));
        let mut source = reader(&path, None);
        let first = ready(&mut source);
        assert_eq!(first.records.len(), 1);
        ack(&mut source, &first);
        let gap = ready(&mut source);
        assert_eq!(gap.directory(), None);
        assert_eq!(gap.records.len(), 2);
        let omission = gap
            .sanitize_omitted(A, &NormalizerProfileV1::codex_0_156_1())
            .unwrap();
        for record in omission.batch.observations {
            let text = std::str::from_utf8(record.payload.bytes()).unwrap();
            assert!(!text.contains("unknown repository content"));
            assert!(text.contains("unverified_directory_context"));
        }
        ack(&mut source, &gap);
        let later = ready(&mut source);
        assert_eq!(later.directory(), Some(Path::new("/workspace/b")));
        assert_eq!(later.records[0].sequence, 4);
    }
}

#[test]
fn restart_reconstructs_directory_across_bounded_prefix_steps_without_reemitting_history() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    header(&path);
    append(
        &path,
        json!({"type":"turn_context","payload":{"cwd":"/workspace/b"}}),
    );
    for _ in 0..7 {
        append(&path, json!({"padding":"x".repeat(800 * 1024)}));
    }
    let mut source = reader(&path, None);
    let previous = loop {
        let batch = ready(&mut source);
        ack(&mut source, &batch);
        if matches!(batch.cursor.position, SourcePositionV1::JsonLines {complete_byte_offset,..} if complete_byte_offset == fs::metadata(&path).unwrap().len())
        {
            break ReaderProgressV1 {
                schema_version: 1,
                repository: B,
                cursor: batch.cursor.clone(),
                metadata: batch.metadata.clone(),
                last_sequence: batch.records.last().unwrap().sequence,
                last_batch: batch.batch_id,
            };
        }
    };
    append(&path, json!({"message":"new-tail"}));
    let mut resumed = reader(&path, Some(previous));
    assert!(matches!(resumed.poll().unwrap(), TranscriptPoll::Verifying));
    let tail = ready(&mut resumed);
    assert_eq!(tail.directory(), Some(Path::new("/workspace/b")));
    assert_eq!(tail.records.len(), 1);
    assert_eq!(tail.records[0].sequence, 10);
}

#[test]
fn arbitrary_cwd_fields_do_not_rebind_and_repeated_session_headers_are_sticky_gaps() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    header(&path);
    append(
        &path,
        json!({"cwd":"/workspace/b","payload":{"cwd":"/workspace/b"},"type":"response_item"}),
    );
    header(&path);
    append(
        &path,
        json!({"type":"turn_context","payload":{"cwd":"/workspace/b"}}),
    );
    append(&path, json!({"message":"foreign session content"}));
    let mut source = reader(&path, None);
    let first = ready(&mut source);
    assert_eq!(first.directory(), Some(Path::new("/workspace/a")));
    assert_eq!(first.records.len(), 2);
    ack(&mut source, &first);
    let withheld = ready(&mut source);
    assert_eq!(withheld.directory(), None);
    assert_eq!(withheld.records.len(), 3);
    ack(&mut source, &withheld);
    assert!(matches!(source.poll().unwrap(), TranscriptPoll::Waiting));
}

#[test]
fn malformed_record_context_is_reconstructed_on_restart_until_a_valid_new_turn() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    header(&path);
    fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .unwrap()
        .write_all(&[255, b'\n'])
        .unwrap();
    append(&path, json!({"message":"uncertain historical content"}));
    let mut source = reader(&path, None);
    let first = ready(&mut source);
    ack(&mut source, &first);
    let gap = ready(&mut source);
    ack(&mut source, &gap);
    let previous = ReaderProgressV1 {
        schema_version: 1,
        repository: A,
        cursor: gap.cursor.clone(),
        metadata: gap.metadata.clone(),
        last_sequence: 3,
        last_batch: gap.batch_id,
    };
    append(
        &path,
        json!({"message":"uncertain new content","cwd":"/workspace/b"}),
    );
    append(
        &path,
        json!({"type":"turn_context","payload":{"cwd":"/workspace/b"}}),
    );
    append(&path, json!({"message":"verified later context"}));
    let mut resumed = reader(&path, Some(previous));
    let uncertain = ready(&mut resumed);
    assert_eq!(uncertain.directory(), None);
    assert_eq!(uncertain.records[0].sequence, 4);
    assert_eq!(uncertain.records.len(), 1);
    ack(&mut resumed, &uncertain);
    let later = ready(&mut resumed);
    assert_eq!(later.directory(), Some(Path::new("/workspace/b")));
    assert_eq!(later.records[0].sequence, 5);
}

#[test]
fn desktop_profile_independently_qualifies_directory_changes_without_cli_header_adoption() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("desktop.jsonl");
    append(
        &path,
        json!({"type":"session_meta","payload":{
            "id":"00000000-0000-0000-0000-000000000001","cwd":"/workspace/a",
            "cli_version":"0.159.2","source":"vscode","originator":"Codex Desktop"
        }}),
    );
    append(
        &path,
        json!({"type":"turn_context","payload":{"cwd":"/workspace/b"}}),
    );
    append(&path, json!({"message":"desktop second repository"}));
    assert!(
        CodexTranscriptReader::open(CaptureProfile::CodexCliJsonlV1, &path, SOURCE, None).is_err()
    );
    let mut source =
        CodexTranscriptReader::open(CaptureProfile::CodexDesktopJsonlV1, &path, SOURCE, None)
            .unwrap();
    let first = ready(&mut source);
    assert_eq!(first.directory(), Some(Path::new("/workspace/a")));
    ack(&mut source, &first);
    let second = ready(&mut source);
    assert_eq!(second.directory(), Some(Path::new("/workspace/b")));
    assert_eq!(second.records[0].sequence, 2);
    assert_eq!(second.cursor.parser_version, "codex-jsonl@0.159.2/v1");
}
