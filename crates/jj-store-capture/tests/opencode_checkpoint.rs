use jj_agent::{IntegrationId, SessionState, SourceInstanceId};
use jj_store_capture::{
    agent_capture::AgentCaptureSpool,
    opencode_checkpoint::{OpenCodeLifecycleV1, OpenCodeSnapshotStateV1},
    opencode_ingress::{OpenCodeEvidence, OpenCodeRecordPosition},
    opencode_snapshot::{OpenCodeActivity, OpenCodeSessionSnapshot},
    transcript_sanitization::TranscriptSanitizer,
};
use jj_store_model::RepositoryId;
use serde_json::json;
use std::path::Path;
#[test]
fn shared_message_prefix_preserves_the_original_snapshot_hash_bytes() {
    use jj_store_capture::opencode_checkpoint::next_message_prefix;
    use sha2::{Digest as _, Sha256};
    let page = session().messages(&serde_json::to_vec(&json!([
        {"info":{"id":"msg_hash","sessionID":"ses_checkpoint","role":"user"},
         "parts":[{"id":"prt_hash","sessionID":"ses_checkpoint","messageID":"msg_hash","type":"text","text":"recorded"}]}
    ])).unwrap()).unwrap();
    let mut original = Sha256::new_with_prefix(b"jj-store/opencode-snapshot-record/v1\0");
    original.update([7; 32]);
    let bytes = serde_json::to_vec(page.records[0].envelope()).unwrap();
    original.update((bytes.len() as u64).to_be_bytes());
    original.update(bytes);
    assert_eq!(
        next_message_prefix([7; 32], &page.records[0]).unwrap(),
        <[u8; 32]>::from(original.finalize())
    );
}
fn session() -> OpenCodeSessionSnapshot {
    OpenCodeSessionSnapshot::from_api_info(
        &serde_json::to_vec(
            &json!({"id":"ses_checkpoint","version":"1.18.30","directory":"/workspace/a"}),
        )
        .unwrap(),
        "ses_checkpoint",
    )
    .unwrap()
}
fn capture(
    state: OpenCodeSnapshotStateV1,
    sequence: u64,
) -> jj_store_capture::reader_progress::ReaderCapture {
    let root = tempfile::tempdir().unwrap();
    let cache = std::env::var_os("JJ_STORE_TRANSCRIPT_CACHE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.path().join("cache"));
    let sanitizer = TranscriptSanitizer::new(
        Path::new("/workspace/a"),
        Path::new("/Users/checkpoint"),
        Path::new("/private/tmp/checkpoint"),
        &cache,
    )
    .unwrap();
    session()
        .status_record(match state.lifecycle {
            OpenCodeLifecycleV1::Busy => OpenCodeActivity::Busy,
            OpenCodeLifecycleV1::Retry => OpenCodeActivity::Retry,
            OpenCodeLifecycleV1::Idle => OpenCodeActivity::Idle,
        })
        .unwrap()
        .sanitize(
            RepositoryId::new([2; 16]),
            &OpenCodeRecordPosition {
                source: SourceInstanceId::new([3; 16]),
                generation: 1,
                session_witness: [8; 32],
                sequence,
                logical_records: state.message_records,
                prefix_digest: state.digest([8; 32]),
                evidence: OpenCodeEvidence::Api,
            },
            Path::new("/workspace/a"),
            &sanitizer,
        )
        .unwrap()
        .with_opencode_state(state)
        .unwrap()
}
#[test]
fn empty_lifecycle_advances_without_fictional_messages_and_survives_reopen() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    let busy = OpenCodeSnapshotStateV1::initial([8; 32], OpenCodeLifecycleV1::Busy);
    let input = capture(busy.clone(), 1);
    assert!(
        matches!(&jj_agent::normalize_batch(&input.profile,&input.batch).unwrap().records[..],[jj_agent::NormalizedActivityRecordV1::SessionState(s)] if s.state==SessionState::Active)
    );
    let expected = input.cursor.clone();
    let ack = spool.enqueue_from_automatic_reader(input).unwrap();
    assert_eq!(ack.position, expected.position);
    drop(spool);
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    assert_eq!(
        spool
            .latest_reader_progress(IntegrationId::OpenCode, SourceInstanceId::new([3; 16]))
            .unwrap()
            .unwrap()
            .metadata
            .opencode_snapshot,
        Some(Box::new(busy.clone()))
    );
    let idle = OpenCodeSnapshotStateV1 {
        lifecycle: OpenCodeLifecycleV1::Idle,
        ..busy
    };
    let input = capture(idle.clone(), 2);
    assert_eq!(
        input
            .metadata
            .opencode_snapshot
            .as_ref()
            .unwrap()
            .message_records,
        0
    );
    assert!(
        matches!(&jj_agent::normalize_batch(&input.profile,&input.batch).unwrap().records[..],[jj_agent::NormalizedActivityRecordV1::SessionState(s)] if s.state==SessionState::Idle)
    );
    spool.enqueue_from_automatic_reader(input).unwrap();
    assert_eq!(spool.pending().unwrap().len(), 2);
    assert_eq!(
        spool
            .latest_reader_progress(IntegrationId::OpenCode, SourceInstanceId::new([3; 16]))
            .unwrap()
            .unwrap()
            .metadata
            .opencode_snapshot,
        Some(Box::new(idle))
    );
}
#[test]
fn invalid_composite_cursor_context_or_duplicate_state_never_advances() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    let busy = OpenCodeSnapshotStateV1::initial([8; 32], OpenCodeLifecycleV1::Busy);
    spool
        .enqueue_from_automatic_reader(capture(busy.clone(), 1))
        .unwrap();
    assert!(
        spool
            .enqueue_from_automatic_reader(capture(busy.clone(), 2))
            .is_err()
    );
    let idle = OpenCodeSnapshotStateV1 {
        lifecycle: OpenCodeLifecycleV1::Idle,
        ..busy
    };
    for mutation in 0..6 {
        let mut input = capture(idle.clone(), 2);
        match mutation {
            0 => input.metadata.generation_digest = [9; 32],
            1 => input.cursor.integration = IntegrationId::Codex,
            2 => {
                input
                    .metadata
                    .opencode_snapshot
                    .as_mut()
                    .unwrap()
                    .message_records = 1
            }
            3 => input.cursor.parser_version = "foreign".into(),
            4 => input.metadata.opencode_snapshot = None,
            _ => input.batch.observations[0].source_trust = jj_agent::SourceTrust::LocalTranscript,
        };
        assert!(spool.enqueue_from_automatic_reader(input).is_err());
    }
    assert_eq!(spool.pending().unwrap().len(), 1);
}
