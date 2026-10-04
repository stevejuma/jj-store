use jj_agent::*;
use jj_store_capture::{
    agent_capture::{AgentCaptureSpool, DurableCaptureAck},
    codex_transcript::{
        CodexTranscriptBatch, CodexTranscriptReader, MAX_RECORD_BYTES, TranscriptPoll,
    },
    profile::CaptureProfile,
    reader_progress::ReaderCapture,
};
use jj_store_model::RepositoryId;
use serde_json::json;
use std::{fs, io::Write as _, path::Path};
const SOURCE: SourceInstanceId = SourceInstanceId::new([7; 16]);
fn header(version: &str, desktop: bool) -> Vec<u8> {
    let mut bytes = serde_json::to_vec(&json!({
        "type":"session_meta", "payload":{
            "id":"00000000-0000-0000-0000-000000000001", "cwd":"/workspace/project",
            "cli_version":version, "source": if desktop { "vscode" } else { "cli" },
            "originator":if desktop { "Codex Desktop" } else { "codex_cli_rs" }
        }
    }))
    .unwrap();
    bytes.push(b'\n');
    bytes
}
#[test]
fn codex_transcript_reader_accepts_only_exact_bound_product_surface() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("rollout.jsonl");
    fs::write(&path, header("0.156.1", false)).unwrap();
    CodexTranscriptReader::open(CaptureProfile::CodexCliJsonlV1, &path, SOURCE, None).unwrap();
    assert!(
        CodexTranscriptReader::open(CaptureProfile::CodexDesktopJsonlV1, &path, SOURCE, None)
            .is_err()
    );
    fs::write(&path, header("0.159.2", true)).unwrap();
    CodexTranscriptReader::open(CaptureProfile::CodexDesktopJsonlV1, &path, SOURCE, None).unwrap();
    assert!(
        CodexTranscriptReader::open(CaptureProfile::CodexCliJsonlV1, &path, SOURCE, None).is_err()
    );
    fs::write(&path, header("0.999.0", true)).unwrap();
    assert!(
        CodexTranscriptReader::open(CaptureProfile::CodexDesktopJsonlV1, &path, SOURCE, None)
            .is_err()
    );
}
#[test]
fn codex_cli_exec_and_inherited_desktop_originator_keep_cli_surface_checks() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("rollout.jsonl");
    for (source, originator) in [
        ("exec", "codex_exec"),
        ("exec", "Codex Desktop"),
        ("cli", "Codex Desktop"),
    ] {
        let mut value: serde_json::Value =
            serde_json::from_slice(&header("0.156.1", false)).unwrap();
        value["payload"]["source"] = json!(source);
        value["payload"]["originator"] = json!(originator);
        fs::write(&path, format!("{value}\n")).unwrap();
        CodexTranscriptReader::open(CaptureProfile::CodexCliJsonlV1, &path, SOURCE, None).unwrap();
        assert!(
            CodexTranscriptReader::open(CaptureProfile::CodexDesktopJsonlV1, &path, SOURCE, None)
                .is_err()
        );
        value["payload"]["source"] = json!("vscode");
        fs::write(&path, format!("{value}\n")).unwrap();
        assert!(reader_open_fails(&path));
        value["payload"]["source"] = json!(source);
        value["payload"]["cli_version"] = json!("0.159.2");
        fs::write(&path, format!("{value}\n")).unwrap();
        assert!(reader_open_fails(&path));
        value["payload"]["cli_version"] = json!("0.156.1");
        value["payload"]["originator"] = json!("unqualified-originator");
        fs::write(&path, format!("{value}\n")).unwrap();
        assert!(reader_open_fails(&path));
    }
}
fn reader_open_fails(path: &Path) -> bool {
    CodexTranscriptReader::open(CaptureProfile::CodexCliJsonlV1, path, SOURCE, None).is_err()
}
fn append(path: &Path, bytes: &[u8]) {
    fs::OpenOptions::new()
        .append(true)
        .open(path)
        .unwrap()
        .write_all(bytes)
        .unwrap();
}
fn reader(
    path: &Path,
    previous: Option<jj_store_capture::reader_progress::ReaderProgressV1>,
) -> CodexTranscriptReader {
    CodexTranscriptReader::open(CaptureProfile::CodexCliJsonlV1, path, SOURCE, previous).unwrap()
}
fn ready(reader: &mut CodexTranscriptReader) -> CodexTranscriptBatch {
    for _ in 0..100 {
        match reader.poll().unwrap() {
            TranscriptPoll::Ready(batch) => return *batch,
            TranscriptPoll::Verifying
            | TranscriptPoll::Waiting
            | TranscriptPoll::ChangedDuringRead => {}
        }
    }
    panic!("bounded fixture never produced a complete record");
}
fn capture(
    spool: &mut AgentCaptureSpool,
    reader: &mut CodexTranscriptReader,
    batch: &CodexTranscriptBatch,
) -> DurableCaptureAck {
    // This source-only test uses sanitized omissions; native projection has a
    // separate gate. Raw records are never serialized into this test spool.
    let observations = batch
        .records
        .iter()
        .map(|record| SanitizedObservationV1 {
            observation_id: ObservationId::new((record.sequence as u128).to_be_bytes()),
            source_sequence: record.sequence,
            occurred_at_millis: None,
            source_kind: SourceKind::Transcript,
            source_trust: SourceTrust::LocalTranscript,
            payload: SanitizedPayloadV1::new(
                SanitizedMediaType::Json,
                br#"{"capture_omitted":true}"#,
            )
            .unwrap(),
        })
        .collect();
    let ack = spool
        .enqueue_from_reader(ReaderCapture {
            repository: RepositoryId::new([1; 16]),
            profile: NormalizerProfileV1::codex_0_156_1(),
            cursor: batch.cursor.clone(),
            metadata: batch.metadata.clone(),
            thread_binding: None,
            batch: SanitizedObservationBatchV1 {
                schema_version: 1,
                batch_id: batch.batch_id,
                integration: IntegrationId::Codex,
                source_instance_id: SOURCE,
                sanitizer_version: "test-sanitized-omission/v1".into(),
                representation: SanitizedRepresentation::SanitizedNative,
                observations,
            },
        })
        .unwrap();
    reader.acknowledge(&ack).unwrap();
    ack
}
#[test]
fn complete_tails_retry_exactly_and_resume_only_after_sanitized_durability() {
    let root = tempfile::tempdir().unwrap();
    let state = tempfile::tempdir().unwrap();
    let path = root.path().join("rollout.jsonl");
    let first = header("0.156.1", false);
    fs::write(&path, &first).unwrap();
    append(&path, b"{\"type\":\"response_item\",\"payload\":");
    let mut reader = reader(&path, None);
    let batch = ready(&mut reader);
    assert_eq!(batch.records.len(), 1);
    assert_eq!(batch.cursor.position, ready(&mut reader).cursor.position);
    assert_eq!(batch.batch_id, ready(&mut reader).batch_id);
    assert!(
        reader
            .acknowledge(&DurableCaptureAck {
                batch_id: CaptureBatchId::new([0; 16]),
                position: batch.cursor.position.clone()
            })
            .is_err()
    );
    let mut spool = AgentCaptureSpool::open(state.path(), 4 * 1024 * 1024).unwrap();
    capture(&mut spool, &mut reader, &batch);
    assert!(matches!(reader.poll().unwrap(), TranscriptPoll::Waiting));
    append(&path, b"{\"type\":\"message\",\"content\":\"hello\"}}\n");
    let second = ready(&mut reader);
    assert_eq!(second.records.len(), 1);
    assert_eq!(second.records[0].sequence, 2);
    capture(&mut spool, &mut reader, &second);
    let previous = spool
        .reader_progress(RepositoryId::new([1; 16]), IntegrationId::Codex, SOURCE, 1)
        .unwrap()
        .unwrap();
    drop(reader);
    let mut reopened = CodexTranscriptReader::open(
        CaptureProfile::CodexCliJsonlV1,
        &path,
        SOURCE,
        Some(previous),
    )
    .unwrap();
    assert!(matches!(reopened.poll().unwrap(), TranscriptPoll::Waiting));
    append(&path, b"{\"later\":true}\n");
    let last = ready(&mut reopened);
    assert_eq!(last.records.len(), 1);
    assert_eq!(last.records[0].sequence, 3);
    assert_eq!(last.cursor.generation, 1);
}
#[test]
fn partial_utf8_and_invalid_complete_records_preserve_exact_byte_positions() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("rollout.jsonl");
    fs::write(&path, header("0.156.1", false)).unwrap();
    let mut reader = reader(&path, None);
    let first = ready(&mut reader);
    reader
        .acknowledge(&DurableCaptureAck {
            batch_id: first.batch_id,
            position: first.cursor.position,
        })
        .unwrap();
    append(&path, &[b'"', 0xc3]);
    assert!(matches!(reader.poll().unwrap(), TranscriptPoll::Waiting));
    append(&path, &[0xa9, b'"', b'\n', 0xff, b'\n']);
    let batch = ready(&mut reader);
    assert_eq!(batch.records.len(), 2);
    assert_eq!(
        batch.records[0].bytes.as_deref().unwrap(),
        &[b'"', 0xc3, 0xa9, b'"', b'\n']
    );
    assert_eq!(batch.records[1].bytes.as_deref().unwrap(), &[0xff, b'\n']);
    let SourcePositionV1::JsonLines {
        complete_byte_offset,
        ..
    } = batch.cursor.position
    else {
        panic!("JSONL")
    };
    assert_eq!(complete_byte_offset, fs::metadata(&path).unwrap().len());
}
#[test]
fn oversized_records_are_bounded_omissions_and_later_records_remain_readable() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("rollout.jsonl");
    fs::write(&path, header("0.156.1", false)).unwrap();
    append(&path, &vec![b'x'; MAX_RECORD_BYTES * 3 + 1]);
    append(&path, b"\n{\"later\":true}\n");
    let mut reader = reader(&path, None);
    let first = ready(&mut reader);
    reader
        .acknowledge(&DurableCaptureAck {
            batch_id: first.batch_id,
            position: first.cursor.position,
        })
        .unwrap();
    let batch = ready(&mut reader);
    assert!(batch.records[0].bytes.is_none());
    assert_eq!(batch.records[0].sequence, 2);
    assert_eq!(batch.records[1].sequence, 3);
    assert_eq!(
        batch.records[1].bytes.as_deref().unwrap(),
        b"{\"later\":true}\n"
    );
}
#[test]
fn replacements_truncation_and_same_size_rewrites_create_new_generations() {
    for fault in 0..3 {
        let root = tempfile::tempdir().unwrap();
        let state = tempfile::tempdir().unwrap();
        let path = root.path().join("rollout.jsonl");
        fs::write(&path, header("0.156.1", false)).unwrap();
        append(&path, b"{\"value\":1}\n");
        let mut reader = reader(&path, None);
        let batch = ready(&mut reader);
        let mut spool = AgentCaptureSpool::open(state.path(), 4 * 1024 * 1024).unwrap();
        capture(&mut spool, &mut reader, &batch);
        let previous = spool
            .reader_progress(RepositoryId::new([1; 16]), IntegrationId::Codex, SOURCE, 1)
            .unwrap()
            .unwrap();
        match fault {
            0 => {
                fs::rename(&path, root.path().join("rotated.jsonl")).unwrap();
                fs::write(&path, header("0.156.1", false)).unwrap();
            }
            1 => fs::write(&path, header("0.156.1", false)).unwrap(),
            _ => {
                let mut bytes = header("0.156.1", false);
                bytes.extend_from_slice(b"{\"value\":2}\n");
                fs::write(&path, bytes).unwrap();
            }
        }
        let live = ready(&mut reader);
        assert_eq!(live.cursor.generation, 2);
        assert_eq!(live.records[0].sequence, 3);
        let mut reopened = CodexTranscriptReader::open(
            CaptureProfile::CodexCliJsonlV1,
            &path,
            SOURCE,
            Some(previous),
        )
        .unwrap();
        let restart = ready(&mut reopened);
        assert_eq!(restart.cursor, live.cursor);
        assert_eq!(restart.batch_id, live.batch_id);
        capture(&mut spool, &mut reader, &live);
        assert!(
            spool
                .reader_progress(RepositoryId::new([1; 16]), IntegrationId::Codex, SOURCE, 1)
                .unwrap()
                .is_some()
        );
        assert!(
            spool
                .reader_progress(RepositoryId::new([1; 16]), IntegrationId::Codex, SOURCE, 2)
                .unwrap()
                .is_some()
        );
    }
}
#[test]
fn resume_verifies_large_prefix_in_bounded_steps_and_does_not_reemit_it() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("rollout.jsonl");
    fs::write(&path, header("0.156.1", false)).unwrap();
    for _ in 0..7 {
        append(&path, &vec![b'x'; 1024 * 1024 - 1]);
        append(&path, b"\n");
    }
    let mut reader = reader(&path, None);
    let mut last = None;
    while !matches!(reader.poll().unwrap(), TranscriptPoll::Waiting) {
        let batch = ready(&mut reader);
        reader
            .acknowledge(&DurableCaptureAck {
                batch_id: batch.batch_id,
                position: batch.cursor.position.clone(),
            })
            .unwrap();
        last = Some(batch);
    }
    let last = last.unwrap();
    let previous = jj_store_capture::reader_progress::ReaderProgressV1 {
        schema_version: 1,
        repository: RepositoryId::new([1; 16]),
        cursor: last.cursor.clone(),
        metadata: last.metadata,
        last_sequence: last.records.last().unwrap().sequence,
        last_batch: last.batch_id,
    };
    let mut reopened = CodexTranscriptReader::open(
        CaptureProfile::CodexCliJsonlV1,
        &path,
        SOURCE,
        Some(previous),
    )
    .unwrap();
    assert!(matches!(
        reopened.poll().unwrap(),
        TranscriptPoll::Verifying
    ));
    assert!(matches!(reopened.poll().unwrap(), TranscriptPoll::Waiting));
    append(&path, b"{\"new\":true}\n");
    let batch = ready(&mut reopened);
    assert_eq!(batch.records.len(), 1);
    assert_eq!(batch.records[0].sequence, 9);
}
#[test]
fn missing_headers_unsafe_paths_and_changed_resume_identity_fail_closed() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("rollout.jsonl");
    fs::write(&path, b"{\"type\":\"session_meta\"").unwrap();
    assert!(
        CodexTranscriptReader::open(CaptureProfile::CodexCliJsonlV1, &path, SOURCE, None).is_err()
    );
    fs::write(&path, header("0.156.1", false)).unwrap();
    let link = root.path().join("alias.jsonl");
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(&path, &link).unwrap();
        assert!(
            CodexTranscriptReader::open(CaptureProfile::CodexCliJsonlV1, &link, SOURCE, None)
                .is_err()
        );
    }
    let mut reader = reader(&path, None);
    let batch = ready(&mut reader);
    let mut progress = jj_store_capture::reader_progress::ReaderProgressV1 {
        schema_version: 1,
        repository: RepositoryId::new([1; 16]),
        cursor: batch.cursor.clone(),
        metadata: batch.metadata.clone(),
        last_sequence: 1,
        last_batch: batch.batch_id,
    };
    progress.cursor.parser_version = "different-parser".into();
    assert!(
        CodexTranscriptReader::open(
            CaptureProfile::CodexCliJsonlV1,
            &path,
            SOURCE,
            Some(progress.clone())
        )
        .is_err()
    );
    progress.cursor = batch.cursor;
    progress.cursor.source_instance_id = SourceInstanceId::new([0; 16]);
    assert!(
        CodexTranscriptReader::open(
            CaptureProfile::CodexCliJsonlV1,
            &path,
            SOURCE,
            Some(progress)
        )
        .is_err()
    );
}
#[test]
fn rewrite_between_prefix_verification_steps_cannot_skip_changed_history() {
    use std::io::{Seek as _, SeekFrom};
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("rollout.jsonl");
    let metadata = header("0.156.1", false);
    fs::write(&path, &metadata).unwrap();
    for _ in 0..7 {
        append(&path, &vec![b'x'; 1024 * 1024 - 1]);
        append(&path, b"\n");
    }
    let mut original = reader(&path, None);
    let previous = loop {
        let batch = ready(&mut original);
        original
            .acknowledge(&DurableCaptureAck {
                batch_id: batch.batch_id,
                position: batch.cursor.position.clone(),
            })
            .unwrap();
        let SourcePositionV1::JsonLines {
            complete_byte_offset,
            ..
        } = batch.cursor.position
        else {
            panic!("JSONL")
        };
        if complete_byte_offset == fs::metadata(&path).unwrap().len() {
            break jj_store_capture::reader_progress::ReaderProgressV1 {
                schema_version: 1,
                repository: RepositoryId::new([1; 16]),
                cursor: batch.cursor,
                metadata: batch.metadata,
                last_sequence: batch.records.last().unwrap().sequence,
                last_batch: batch.batch_id,
            };
        }
    };
    let mut reopened = CodexTranscriptReader::open(
        CaptureProfile::CodexCliJsonlV1,
        &path,
        SOURCE,
        Some(previous),
    )
    .unwrap();
    assert!(matches!(
        reopened.poll().unwrap(),
        TranscriptPoll::Verifying
    ));
    let mut writer = fs::OpenOptions::new().write(true).open(&path).unwrap();
    writer
        .seek(SeekFrom::Start(metadata.len() as u64 + 1))
        .unwrap();
    writer.write_all(b"y").unwrap();
    writer.sync_all().unwrap();
    assert!(matches!(
        reopened.poll().unwrap(),
        TranscriptPoll::Verifying
    ));
    assert_eq!(ready(&mut reopened).cursor.generation, 2);
}
#[test]
fn batch_record_limit_preserves_every_later_record() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("rollout.jsonl");
    fs::write(&path, header("0.156.1", false)).unwrap();
    for _ in 0..258 {
        append(&path, b"{\"record\":true}\n");
    }
    let mut reader = reader(&path, None);
    let first = ready(&mut reader);
    assert_eq!(first.records.len(), 256);
    assert_eq!(first.records.last().unwrap().sequence, 256);
    reader
        .acknowledge(&DurableCaptureAck {
            batch_id: first.batch_id,
            position: first.cursor.position,
        })
        .unwrap();
    let second = ready(&mut reader);
    assert_eq!(second.records.len(), 3);
    assert_eq!(second.records[0].sequence, 257);
    assert_eq!(second.records.last().unwrap().sequence, 259);
    let SourcePositionV1::JsonLines {
        complete_byte_offset,
        ..
    } = second.cursor.position
    else {
        panic!("JSONL")
    };
    assert_eq!(complete_byte_offset, fs::metadata(path).unwrap().len());
}
