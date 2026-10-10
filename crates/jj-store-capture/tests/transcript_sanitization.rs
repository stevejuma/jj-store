use jj_agent::*;
use jj_store_capture::{
    agent_capture::AgentCaptureSpool,
    codex_transcript::{CodexTranscriptReader, TranscriptPoll},
    profile::CaptureProfile,
    transcript_sanitization::{TRANSCRIPT_POLICY, TranscriptSanitizer},
};
use jj_store_model::RepositoryId;
use serde_json::json;
use std::{fs, io::Write as _, path::Path};
const SECRET: &str = "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";
const SESSION: &str = "00000000-0000-0000-0000-000000000001";
#[test]
fn streaming_paths_are_private_at_every_chunk_boundary() {
    let root = tempfile::tempdir().unwrap();
    let cache = std::env::var_os("JJ_STORE_TRANSCRIPT_CACHE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.path().join("cache"));
    let sanitizer = TranscriptSanitizer::new(
        Path::new("/Users/synthetic/project"),
        Path::new("/Users/synthetic"),
        Path::new("/tmp/synthetic"),
        &cache,
    )
    .unwrap();
    let mut output = String::new();
    let mut pending = String::new();
    for chunk in [
        "Updated [LICENSE](<",
        "/",
        "Users",
        "/synthetic",
        "/project",
        "/LICENSE:3>)",
    ] {
        let text = sanitizer
            .replace_stream_text(&mut pending, chunk, false)
            .unwrap();
        let safe = sanitizer.sanitize(json!({"text":text})).unwrap();
        let value: serde_json::Value = serde_json::from_slice(safe.bytes()).unwrap();
        output.push_str(value["text"].as_str().unwrap());
    }
    output.push_str(
        &sanitizer
            .replace_stream_text(&mut pending, "", true)
            .unwrap(),
    );
    assert_eq!(output, "Updated [LICENSE](<$WORKSPACE/LICENSE:3>)");
    let sanitizer = sanitizer
        .for_secrets(&["synthetic-secret-value".into()])
        .unwrap();
    for (raw, expected) in [
        (
            "🦀 /Users/synthetic/project/LICENSE:3",
            "🦀 $WORKSPACE/LICENSE:3",
        ),
        ("/Users/synthetic/config", "$HOME/config"),
        ("/tmp/synthetic/file", "$TEMP/file"),
        ("synthetic-secret-value", "$SECRET"),
    ] {
        for split in raw.char_indices().map(|(n, _)| n) {
            let mut pending = String::new();
            let mut output = sanitizer
                .replace_stream_text(&mut pending, &raw[..split], false)
                .unwrap();
            output.push_str(
                &sanitizer
                    .replace_stream_text(&mut pending, &raw[split..], false)
                    .unwrap(),
            );
            output.push_str(
                &sanitizer
                    .replace_stream_text(&mut pending, "", true)
                    .unwrap(),
            );
            assert_eq!(output, expected, "split {split}");
            assert!(pending.is_empty());
        }
    }
}
fn fixture(path: &Path) {
    let records = vec![
        json!({"type":"session_meta","payload":{"id":SESSION,"cwd":"/workspace/project","cli_version":"0.156.1","source":"cli","originator":"codex_cli_rs"}}),
        json!({"type":"response_item","payload":{"type":"message","id":"message-1","role":"assistant","content":[{"type":"output_text","text":format!("token={SECRET} /workspace/project/src/main.rs /Users/transcript-test/private /private/tmp/transcript-test/file")}],"metadata":{"/Users/transcript-test/private":"path-key"}}}),
    ];
    let mut file = fs::File::create(path).unwrap();
    for record in records {
        file.write_all(&serde_json::to_vec(&record).unwrap())
            .unwrap();
        file.write_all(b"\n").unwrap();
    }
}
#[test]
fn transcript_sanitization_precedes_durability_and_covers_path_keys_and_values() {
    let root = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    let state = tempfile::tempdir().unwrap();
    let path = root.path().join("rollout.jsonl");
    fixture(&path);
    let cache_path = std::env::var_os("JJ_STORE_TRANSCRIPT_CACHE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| cache.path().to_owned());
    let mut reader = CodexTranscriptReader::open(
        CaptureProfile::CodexCliJsonlV1,
        &path,
        SourceInstanceId::new([1; 16]),
        None,
    )
    .unwrap();
    let TranscriptPoll::Ready(batch) = reader.poll().unwrap() else {
        panic!("complete fixture")
    };
    let sanitizer = TranscriptSanitizer::new(
        Path::new("/workspace/project"),
        Path::new("/Users/transcript-test"),
        Path::new("/private/tmp/transcript-test"),
        &cache_path,
    )
    .unwrap();
    let first = batch
        .sanitize(
            RepositoryId::new([2; 16]),
            &NormalizerProfileV1::codex_0_156_1(),
            &sanitizer,
        )
        .unwrap();
    let retry = batch
        .sanitize(
            RepositoryId::new([2; 16]),
            &NormalizerProfileV1::codex_0_156_1(),
            &sanitizer,
        )
        .unwrap();
    assert_eq!(first.batch, retry.batch);
    assert_eq!(first.batch.sanitizer_version, TRANSCRIPT_POLICY);
    let text = first
        .batch
        .observations
        .iter()
        .map(|o| std::str::from_utf8(o.payload.bytes()).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    for raw in [
        SECRET,
        "/workspace/project",
        "/Users/transcript-test",
        "/private/tmp/transcript-test",
    ] {
        assert!(!text.contains(raw), "private input survived sanitization");
    }
    for tag in ["[REDACTED]", "$WORKSPACE", "$HOME", "$TEMP"] {
        assert!(text.contains(tag));
    }
    let normalized = normalize_batch(&first.profile, &first.batch).unwrap();
    assert!(
        normalized
            .records
            .iter()
            .any(|r| matches!(r, NormalizedActivityRecordV1::Message(_)))
    );
    let mut spool = AgentCaptureSpool::open(state.path(), 4 * 1024 * 1024).unwrap();
    let ack = spool.enqueue_from_reader(first).unwrap();
    reader.acknowledge(&ack).unwrap();
    for pending in spool.pending().unwrap() {
        for observation in pending.batch.observations {
            let text = std::str::from_utf8(observation.payload.bytes()).unwrap();
            for raw in [
                SECRET,
                "/workspace/project",
                "/Users/transcript-test",
                "/private/tmp/transcript-test",
            ] {
                assert!(!text.contains(raw));
            }
        }
    }
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "--ignored", "transcript_cached_child"])
        .env("JJ_STORE_TRANSCRIPT_CHILD_CACHE", &cache_path)
        .env("JJ_STORE_TRANSCRIPT_CHILD_FILE", &path)
        .status()
        .unwrap();
    assert!(status.success());
}
#[test]
#[ignore = "fresh-process fixture launched by the privacy acceptance test"]
fn transcript_cached_child() {
    let cache = std::env::var_os("JJ_STORE_TRANSCRIPT_CHILD_CACHE").unwrap();
    let path = std::env::var_os("JJ_STORE_TRANSCRIPT_CHILD_FILE").unwrap();
    let sanitizer = TranscriptSanitizer::new(
        Path::new("/workspace/project"),
        Path::new("/Users/transcript-test"),
        Path::new("/private/tmp/transcript-test"),
        Path::new(&cache),
    )
    .unwrap();
    let mut reader = CodexTranscriptReader::open(
        CaptureProfile::CodexCliJsonlV1,
        Path::new(&path),
        SourceInstanceId::new([1; 16]),
        None,
    )
    .unwrap();
    let TranscriptPoll::Ready(batch) = reader.poll().unwrap() else {
        panic!("fixture")
    };
    let input = batch
        .sanitize(
            RepositoryId::new([2; 16]),
            &NormalizerProfileV1::codex_0_156_1(),
            &sanitizer,
        )
        .unwrap();
    for observation in &input.batch.observations {
        let text = std::str::from_utf8(observation.payload.bytes()).unwrap();
        for raw in [
            SECRET,
            "/workspace/project",
            "/Users/transcript-test",
            "/private/tmp/transcript-test",
        ] {
            assert!(!text.contains(raw));
        }
    }
    assert_eq!(
        normalize_batch(&input.profile, &input.batch)
            .unwrap()
            .records
            .len(),
        1
    );
}
#[test]
fn omissions_key_collisions_and_invalid_identity_retain_safe_source_progress() {
    use jj_store_capture::codex_transcript::{CodexTranscriptBatch, RawTranscriptRecord};
    use jj_store_capture::reader_progress::ReaderCaptureMetadataV1;
    let root = tempfile::tempdir().unwrap();
    let cache = std::env::var_os("JJ_STORE_TRANSCRIPT_CACHE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.path().join("cache"));
    let sanitizer = TranscriptSanitizer::new(
        Path::new("/workspace/project"),
        Path::new("/Users/transcript-test"),
        Path::new("/private/tmp/transcript-test"),
        &cache,
    )
    .unwrap();
    let mut batch=CodexTranscriptBatch {
        directory_context:std::sync::Arc::new(jj_store_capture::codex_transcript::CodexDirectoryContext::Known("/workspace/project".into())),
        batch_id:CaptureBatchId::new([3;16]),
        cursor:CaptureCursorV1 {schema_version:1,integration:IntegrationId::Codex,source_instance_id:SourceInstanceId::new([1;16]),generation:1,
            parser_version:"codex-jsonl@0.156.1/v1".into(),native_session_key:Some(SESSION.into()),
            position:SourcePositionV1::JsonLines {complete_byte_offset:100,prefix_sha256:[1;32]}},
        metadata:ReaderCaptureMetadataV1 {schema_version:1,generation_digest:[2;32], opencode_snapshot:None},
        records:std::sync::Arc::new(vec![
            RawTranscriptRecord {record_key:None,sequence:1,bytes:Some(vec![0xff,b'\n'])},
            RawTranscriptRecord {record_key:None,sequence:2,bytes:None},
            RawTranscriptRecord {record_key:None,sequence:3,bytes:Some(br#"{"metadata":{"/Users/transcript-test/private":"private","$HOME/private":"already-templated"}}"#.to_vec())},
            RawTranscriptRecord {record_key:None,sequence:4,bytes:Some(br#"{"type":"response_item","payload":{"type":"message","id":"later","role":"assistant","content":[{"type":"output_text","text":"later valid message"}]}}"#.to_vec())},
        ]),
    };
    let input = batch
        .sanitize(
            RepositoryId::new([2; 16]),
            &NormalizerProfileV1::codex_0_156_1(),
            &sanitizer,
        )
        .unwrap();
    assert_eq!(input.batch.observations.len(), 4);
    for observation in &input.batch.observations[..3] {
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(observation.payload.bytes()).unwrap()["capture_omitted"],
            true
        );
    }
    let normalized = normalize_batch(&input.profile, &input.batch).unwrap();
    assert_eq!(normalized.records.len(), 1);
    assert_eq!(normalized.diagnostics.len(), 3);
    let mut spool = AgentCaptureSpool::open(&root.path().join("state"), 4 * 1024 * 1024).unwrap();
    spool.enqueue_from_reader(input).unwrap();
    assert_eq!(
        spool
            .reader_progress(
                RepositoryId::new([2; 16]),
                IntegrationId::Codex,
                SourceInstanceId::new([1; 16]),
                1
            )
            .unwrap()
            .unwrap()
            .last_sequence,
        4
    );
    batch.cursor.native_session_key = Some(SECRET.into());
    assert!(
        batch
            .sanitize(
                RepositoryId::new([2; 16]),
                &NormalizerProfileV1::codex_0_156_1(),
                &sanitizer
            )
            .is_err()
    );
}
#[test]
fn ambiguous_privacy_roots_fail_before_cache_or_state_creation() {
    let root = tempfile::tempdir().unwrap();
    let cache = root.path().join("cache");
    for workspace in [
        "/",
        "relative",
        "/workspace/../project",
        "/Users/transcript-test/",
    ] {
        assert!(
            TranscriptSanitizer::new(
                Path::new(workspace),
                Path::new("/Users/transcript-test"),
                Path::new("/private/tmp/transcript-test"),
                &cache
            )
            .is_err()
        );
        assert!(!cache.exists());
    }
}

#[test]
fn unknown_directory_never_sanitizes_native_bodies_and_safe_gaps_allow_later_repository() {
    let root = tempfile::tempdir().unwrap();
    let cache = std::env::var_os("JJ_STORE_TRANSCRIPT_CACHE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.path().join("cache"));
    let path = root.path().join("session.jsonl");
    let mut file = fs::File::create(&path).unwrap();
    for record in [
        json!({"type":"session_meta","payload":{"id":SESSION,"cwd":"/workspace/a","cli_version":"0.156.1","source":"cli","originator":"codex_cli_rs"}}),
        json!({"type":"turn_context","payload":{"cwd":"relative-unknown"}}),
        json!({"type":"response_item","payload":{"type":"message","id":"withheld","role":"assistant","content":[{"type":"output_text","text":format!("private uncertain body {SECRET}")} ]}}),
        json!({"type":"turn_context","payload":{"cwd":"/workspace/b/subdir"}}),
        json!({"type":"response_item","payload":{"type":"message","id":"later","role":"assistant","content":[{"type":"output_text","text":format!("{SECRET} /workspace/b/src/main.rs")} ]}}),
    ] {
        file.write_all(&serde_json::to_vec(&record).unwrap())
            .unwrap();
        file.write_all(b"\n").unwrap();
    }
    let mut reader = CodexTranscriptReader::open(
        CaptureProfile::CodexCliJsonlV1,
        &path,
        SourceInstanceId::new([1; 16]),
        None,
    )
    .unwrap();
    let profile = NormalizerProfileV1::codex_0_156_1();
    let sanitizer_a = TranscriptSanitizer::new(
        Path::new("/workspace/a"),
        Path::new("/Users/transcript-test"),
        Path::new("/private/tmp/transcript-test"),
        &cache,
    )
    .unwrap();
    let mut spool = AgentCaptureSpool::open(&root.path().join("state"), 4 * 1024 * 1024).unwrap();
    let TranscriptPoll::Ready(first) = reader.poll().unwrap() else {
        panic!("header")
    };
    let durable = spool
        .enqueue_from_reader(
            first
                .sanitize(RepositoryId::new([1; 16]), &profile, &sanitizer_a)
                .unwrap(),
        )
        .unwrap();
    reader.acknowledge(&durable).unwrap();
    let TranscriptPoll::Ready(gap) = reader.poll().unwrap() else {
        panic!("unknown context")
    };
    assert!(
        gap.sanitize(RepositoryId::new([1; 16]), &profile, &sanitizer_a)
            .is_err()
    );
    let omission = gap
        .sanitize_omitted(RepositoryId::new([1; 16]), &profile)
        .unwrap();
    let normalized = normalize_batch(&profile, &omission.batch).unwrap();
    assert!(normalized.records.is_empty());
    assert_eq!(normalized.diagnostics.len(), 2);
    let durable = spool.enqueue_from_reader(omission).unwrap();
    reader.acknowledge(&durable).unwrap();
    let TranscriptPoll::Ready(later) = reader.poll().unwrap() else {
        panic!("later context")
    };
    assert_eq!(later.directory(), Some(Path::new("/workspace/b/subdir")));
    let sanitizer_b = TranscriptSanitizer::new(
        Path::new("/workspace/b"),
        Path::new("/Users/transcript-test"),
        Path::new("/private/tmp/transcript-test"),
        &cache,
    )
    .unwrap();
    let input = later
        .sanitize(RepositoryId::new([2; 16]), &profile, &sanitizer_b)
        .unwrap();
    assert_eq!(
        normalize_batch(&profile, &input.batch)
            .unwrap()
            .records
            .len(),
        1
    );
    let durable = spool.enqueue_from_reader(input).unwrap();
    reader.acknowledge(&durable).unwrap();
    let bytes = serde_json::to_vec(&spool.pending().unwrap()).unwrap();
    let text = std::str::from_utf8(&bytes).unwrap();
    for forbidden in [
        SECRET,
        "private uncertain body",
        "relative-unknown",
        "/workspace/a",
        "/workspace/b",
    ] {
        assert!(!text.contains(forbidden));
    }
    assert_eq!(
        spool
            .latest_reader_progress(IntegrationId::Codex, SourceInstanceId::new([1; 16]))
            .unwrap()
            .unwrap()
            .last_sequence,
        5
    );
}

#[test]
fn oauth_jwts_are_redacted_at_their_own_bound_without_expanding_ordinary_secrets() {
    let root = tempfile::tempdir().unwrap();
    let cache = std::env::var_os("JJ_STORE_TRANSCRIPT_CACHE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.path().join("cache"));
    let sanitizer = TranscriptSanitizer::new(
        &root.path().join("workspace"),
        Path::new("/Users/synthetic-oauth"),
        Path::new("/tmp/synthetic-oauth"),
        &cache,
    )
    .unwrap();
    let token = "synthetic-jwt-".to_owned() + &"x".repeat(12000);
    assert!(sanitizer.for_secrets(std::slice::from_ref(&token)).is_err());
    let oauth = sanitizer
        .for_oauth_secrets(std::slice::from_ref(&token))
        .unwrap();
    let result = oauth
        .sanitize(json!({"text":format!("credential {token}"),token.clone():"key"}))
        .unwrap();
    let value: serde_json::Value = serde_json::from_slice(result.bytes()).unwrap();
    assert_eq!(value["text"], "credential $SECRET");
    assert_eq!(value["$SECRET"], "key");
    assert!(sanitizer.for_oauth_secrets(&["x".repeat(16385)]).is_err());
}
