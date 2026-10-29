use jj_agent::{CaptureBatchId, SourceInstanceId};
use jj_store_capture::{
    agent_capture::DurableCaptureAck,
    codex_transcript::{CodexTranscriptBatch, CodexTranscriptReader, TranscriptPoll},
    profile::CaptureProfile,
};
use jj_store_model::RepositoryId;
use std::fs;

fn ready(reader: &mut CodexTranscriptReader) -> Box<CodexTranscriptBatch> {
    for _ in 0..8 {
        match reader.poll().unwrap() {
            TranscriptPoll::Ready(batch) => return batch,
            TranscriptPoll::Verifying => continue,
            _ => panic!("expected exact pending batch"),
        }
    }
    panic!("bounded source did not become ready");
}

#[test]
fn headless_reader_retains_exact_pending_identity_until_host_acknowledgement() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("session.jsonl");
    let profile = CaptureProfile::CodexCliJsonlV1;
    fs::write(&path, concat!(
        "{\"type\":\"session_meta\",\"payload\":{\"id\":\"12345678-1234-1234-1234-123456789abc\",\"cwd\":\"/private/repo\",\"source\":\"cli\",\"originator\":\"codex_cli_rs\",\"cli_version\":\"0.156.1\"}}\n",
        "{\"type\":\"event_msg\",\"payload\":{\"type\":\"agent_message\",\"message\":\"private raw text\"}}\n",
    )).unwrap();
    let mut reader =
        CodexTranscriptReader::open(profile, &path, SourceInstanceId::new([7; 16]), None).unwrap();
    let first = ready(&mut reader);
    let repeated = ready(&mut reader);
    assert_eq!(first.batch_id, repeated.batch_id);
    assert_eq!(first.cursor, repeated.cursor);
    // The safe omission path retains position without copying any source body.
    let capture = first
        .sanitize_omitted(RepositoryId::new([8; 16]), &profile.normalizer())
        .unwrap();
    let encoded = serde_json::to_string(&capture.batch).unwrap();
    assert!(!encoded.contains("private raw text"));
    assert!(!encoded.contains("/private/repo"));
    let acknowledgement = DurableCaptureAck {
        batch_id: capture.batch.batch_id,
        position: capture.cursor.position,
    };
    let mut wrong = acknowledgement.clone();
    wrong.batch_id = CaptureBatchId::new([9; 16]);
    assert!(reader.acknowledge(&wrong).is_err());
    assert_eq!(ready(&mut reader).batch_id, first.batch_id);
    reader.acknowledge(&acknowledgement).unwrap();
    assert!(matches!(reader.poll().unwrap(), TranscriptPoll::Waiting));
}
