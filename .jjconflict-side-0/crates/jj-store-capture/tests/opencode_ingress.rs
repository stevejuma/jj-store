use jj_agent::{IntegrationId, SourceInstanceId};
use jj_store_capture::{
    agent_capture::AgentCaptureSpool,
    opencode_ingress::{OpenCodeEvidence, OpenCodeRecordPosition},
    opencode_snapshot::{OpenCodeActivity, OpenCodeSessionSnapshot},
    source_registry::{OpenCodeSourceBinding, SourceRegistry},
    transcript_sanitization::TranscriptSanitizer,
};
use jj_store_model::RepositoryId;
use serde_json::json;
use std::{fs, path::Path};
const SECRET: &str = "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";
fn session(directory: &str) -> OpenCodeSessionSnapshot {
    OpenCodeSessionSnapshot::from_api_info(
        &serde_json::to_vec(&json!({"id":"ses_first","version":"1.18.30","directory":directory}))
            .unwrap(),
        "ses_first",
    )
    .unwrap()
}
fn record(
    cwd: Option<&str>,
    text: &str,
) -> jj_store_capture::opencode_snapshot::OpenCodeMessageSnapshot {
    let session = session("/workspace/a");
    session.messages(&serde_json::to_vec(&json!([{"info":{"id":"msg_first","sessionID":"ses_first","role":"assistant","path":{"cwd":cwd}},"parts":[{"id":"prt_first","sessionID":"ses_first","messageID":"msg_first","type":"text","text":text}]}])).unwrap()).unwrap().records.remove(0)
}
fn position(source: SourceInstanceId, sequence: u64, prefix: u8) -> OpenCodeRecordPosition {
    OpenCodeRecordPosition {
        source,
        generation: 1,
        session_witness: [8; 32],
        sequence,
        logical_records: sequence,
        prefix_digest: [prefix; 32],
        evidence: OpenCodeEvidence::Api,
    }
}
#[test]
fn api_source_identity_is_private_exclusive_installation_bound_and_resumable() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("sources");
    let registry = SourceRegistry::open(&path, [7; 16], 2).unwrap();
    let binding = OpenCodeSourceBinding::new([4; 32], &session("/workspace/a")).unwrap();
    assert!(
        registry
            .lookup_opencode_admitted(&binding)
            .unwrap()
            .is_none()
    );
    assert!(
        registry
            .acquire_opencode_active(binding.clone(), OpenCodeActivity::Idle)
            .is_err()
    );
    assert!(
        registry
            .acquire_opencode_admitted(binding.clone())
            .unwrap()
            .is_none()
    );
    let lease = registry
        .acquire_opencode_active(binding.clone(), OpenCodeActivity::Busy)
        .unwrap();
    let id = lease.source_instance_id();
    assert_eq!(
        registry.lookup_opencode_admitted(&binding).unwrap(),
        Some(id)
    );
    assert!(
        registry
            .acquire_opencode_active(binding.clone(), OpenCodeActivity::Retry)
            .is_err()
    );
    assert!(registry.acquire_opencode_admitted(binding.clone()).is_err());
    let bytes = fs::read(path.join("sources.json")).unwrap();
    assert!(!String::from_utf8(bytes).unwrap().contains("/workspace"));
    drop(lease);
    drop(registry);
    let registry = SourceRegistry::open_existing(&path, [7; 16], 2)
        .unwrap()
        .unwrap();
    let lease = registry
        .acquire_opencode_admitted(binding)
        .unwrap()
        .unwrap();
    assert_eq!(lease.source_instance_id(), id);
    assert!(SourceRegistry::open_existing(&path, [9; 16], 2).is_err());
    let rebound = OpenCodeSourceBinding::new([4; 32], &session("/workspace/b")).unwrap();
    assert!(
        registry
            .lookup_opencode_admitted(&rebound)
            .unwrap()
            .is_none()
    );
    drop(lease);
    let rebound = registry
        .acquire_opencode_active(rebound, OpenCodeActivity::Busy)
        .unwrap();
    assert_eq!(rebound.source_instance_id(), id);
    let separate = OpenCodeSourceBinding::new([5; 32], &session("/workspace/a")).unwrap();
    let other = registry
        .acquire_opencode_active(separate, OpenCodeActivity::Busy)
        .unwrap();
    assert_ne!(other.source_instance_id(), id);
    assert!(OpenCodeSourceBinding::new([0; 32], &session("/workspace/a")).is_err());
}
#[test]
fn sanitized_api_records_are_durable_before_ack_and_keep_exact_retry_after_restart() {
    let root = tempfile::tempdir().unwrap();
    let cache = std::env::var_os("JJ_STORE_TRANSCRIPT_CACHE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.path().join("cache"));
    let sanitizer = TranscriptSanitizer::new(
        Path::new("/workspace/b"),
        Path::new("/Users/opencode-test"),
        Path::new("/private/tmp/opencode-test"),
        &cache,
    )
    .unwrap();
    let native = record(
        Some("/workspace/b"),
        &format!(
            "{SECRET} /workspace/b/src /Users/opencode-test/private /private/tmp/opencode-test/file"
        ),
    );
    let repository = RepositoryId::new([2; 16]);
    let source = SourceInstanceId::new([3; 16]);
    assert!(
        native
            .sanitize(
                repository,
                &position(source, 1, 1),
                Path::new("/workspace/a"),
                &sanitizer
            )
            .is_err()
    );
    let input = native
        .sanitize(
            repository,
            &position(source, 1, 1),
            Path::new("/workspace/b"),
            &sanitizer,
        )
        .unwrap();
    let retry = native
        .sanitize(
            repository,
            &position(source, 1, 1),
            Path::new("/workspace/b"),
            &sanitizer,
        )
        .unwrap();
    assert_eq!(input.batch, retry.batch);
    let text = std::str::from_utf8(input.batch.observations[0].payload.bytes()).unwrap();
    for raw in [
        SECRET,
        "/workspace/b",
        "/Users/opencode-test",
        "/private/tmp/opencode-test",
    ] {
        assert!(!text.contains(raw));
    }
    for tag in ["[REDACTED]", "$WORKSPACE", "$HOME", "$TEMP"] {
        assert!(text.contains(tag));
    }
    let normalized = jj_agent::normalize_batch(&input.profile, &input.batch).unwrap();
    assert_eq!(normalized.records.len(), 1);
    let mut export_position = position(source, 1, 1);
    export_position.evidence = OpenCodeEvidence::Export;
    let export = native
        .sanitize(
            repository,
            &export_position,
            Path::new("/workspace/b"),
            &sanitizer,
        )
        .unwrap();
    assert_eq!(
        export.batch.observations[0].source_kind,
        jj_agent::SourceKind::SessionExport
    );
    assert_eq!(
        export.batch.observations[0].source_trust,
        jj_agent::SourceTrust::LocalTranscript
    );
    assert_ne!(export.batch.batch_id, input.batch.batch_id);
    assert_eq!(
        jj_agent::normalize_batch(&export.profile, &export.batch)
            .unwrap()
            .records,
        normalized.records
    );

    let mut spool = AgentCaptureSpool::open(&root.path().join("spool"), 4 * 1024 * 1024).unwrap();
    let ack = spool.enqueue_from_automatic_reader(input).unwrap();
    assert_eq!(spool.enqueue_from_automatic_reader(retry).unwrap(), ack);
    let progress = spool
        .latest_reader_progress(IntegrationId::OpenCode, source)
        .unwrap()
        .unwrap();
    assert_eq!(progress.last_sequence, 1);
    assert_eq!(progress.cursor.position, ack.position);
    drop(spool);
    let mut spool = AgentCaptureSpool::open(&root.path().join("spool"), 4 * 1024 * 1024).unwrap();
    assert_eq!(
        spool
            .latest_reader_progress(IntegrationId::OpenCode, source)
            .unwrap(),
        Some(progress)
    );
    assert_eq!(
        spool
            .enqueue_from_automatic_reader(
                native
                    .sanitize(
                        repository,
                        &position(source, 1, 1),
                        Path::new("/workspace/b"),
                        &sanitizer
                    )
                    .unwrap()
            )
            .unwrap(),
        ack
    );
    let mut other = position(source, 2, 2);
    other.session_witness = [9; 32];
    assert!(
        spool
            .enqueue_from_automatic_reader(
                native
                    .sanitize(repository, &other, Path::new("/workspace/b"), &sanitizer)
                    .unwrap()
            )
            .is_err()
    );
    assert_eq!(spool.pending().unwrap().len(), 1);
}
#[test]
fn missing_context_keeps_only_safe_omission_and_does_not_skip_later_records() {
    let root = tempfile::tempdir().unwrap();
    let source = SourceInstanceId::new([3; 16]);
    let repository = RepositoryId::new([2; 16]);
    let native = record(None, SECRET);
    let input = native
        .sanitize_omitted(repository, &position(source, 1, 1))
        .unwrap();
    let text = std::str::from_utf8(input.batch.observations[0].payload.bytes()).unwrap();
    assert!(!text.contains(SECRET));
    assert!(!text.contains("msg_first"));
    assert!(text.contains("unverified_directory_context"));
    let normalized = jj_agent::normalize_batch(&input.profile, &input.batch).unwrap();
    assert!(normalized.records.is_empty());
    assert_eq!(normalized.diagnostics.len(), 1);
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool.enqueue_from_automatic_reader(input).unwrap();
    spool
        .enqueue_from_automatic_reader(
            native
                .sanitize_omitted(repository, &position(source, 2, 2))
                .unwrap(),
        )
        .unwrap();
    assert_eq!(
        spool
            .latest_reader_progress(IntegrationId::OpenCode, source)
            .unwrap()
            .unwrap()
            .last_sequence,
        2
    );
    for invalid in [0, 1, 2] {
        let mut p = position(source, 3, 3);
        match invalid {
            0 => p.logical_records = 0,
            1 => p.prefix_digest = [0; 32],
            _ => p.sequence = 0,
        };
        assert!(native.sanitize_omitted(repository, &p).is_err());
    }
}
