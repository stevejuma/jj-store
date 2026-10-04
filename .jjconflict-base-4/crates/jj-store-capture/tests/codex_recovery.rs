#![cfg(unix)]
use jj_agent::{IntegrationId, NormalizerProfileV1};
use jj_store_capture::{
    agent_capture::{AgentCaptureSpool, PendingCaptureV1},
    codex_discovery::{ApprovedCodexRoot, CodexDirectoryScanner},
    codex_transcript::{CodexRecoveryPoll, CodexTranscriptBatch, TranscriptPoll},
    profile::CaptureProfile,
    source_registry::{LeasedCodexTranscriptReader, QualifiedCodexSource, SourceRegistry},
};
use jj_store_model::RepositoryId;
use serde_json::{Value, json};
use sha2::{Digest as _, Sha256};
use std::{fs, io::Write as _, path::Path, process::Command, sync::Arc};
const A: RepositoryId = RepositoryId::new([1; 16]);
const B: RepositoryId = RepositoryId::new([2; 16]);
fn append(path: &Path, value: Value) {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();
    writeln!(file, "{value}").unwrap();
}
fn header(path: &Path) {
    append(
        path,
        json!({"type":"session_meta","payload":{"id":"00000000-0000-0000-0000-000000000001","cwd":"/workspace/a","cli_version":"0.156.1","source":"cli","originator":"codex_cli_rs"}}),
    );
}
fn qualified(path: &Path) -> QualifiedCodexSource {
    let mut scanner = CodexDirectoryScanner::new(
        ApprovedCodexRoot::open(path.parent().unwrap())
            .unwrap()
            .unwrap(),
    )
    .unwrap();
    QualifiedCodexSource::inspect(
        CaptureProfile::CodexCliJsonlV1,
        scanner.step(100, 100).unwrap().candidates.remove(0),
    )
    .unwrap()
    .unwrap()
}
fn ready(reader: &mut LeasedCodexTranscriptReader) -> CodexTranscriptBatch {
    for _ in 0..100 {
        if let TranscriptPoll::Ready(batch) = reader.poll().unwrap() {
            return *batch;
        }
    }
    panic!("bounded reader did not produce fixture batch");
}
fn enqueue(
    spool: &mut AgentCaptureSpool,
    reader: &mut LeasedCodexTranscriptReader,
    batch: &CodexTranscriptBatch,
    repository: RepositoryId,
) -> PendingCaptureV1 {
    let ack = spool
        .enqueue_from_reader(
            batch
                .sanitize_omitted(repository, &NormalizerProfileV1::codex_0_156_1())
                .unwrap(),
        )
        .unwrap();
    reader.acknowledge(&ack).unwrap();
    spool
        .pending()
        .unwrap()
        .into_iter()
        .find(|entry| entry.batch.batch_id == batch.batch_id)
        .unwrap()
}

#[test]
fn desktop_recovery_rejects_cli_profile_and_incomplete_ending() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("transcripts/session.jsonl");
    fs::create_dir(path.parent().unwrap()).unwrap();
    append(
        &path,
        json!({"type":"session_meta","payload":{"id":"00000000-0000-0000-0000-000000000001","cwd":"/workspace/a","cli_version":"0.159.2","source":"vscode","originator":"Codex Desktop"}}),
    );
    append(
        &path,
        json!({"type":"turn_context","payload":{"cwd":"/workspace/b"}}),
    );
    append(&path, json!({"text":"desktop b"}));
    let mut scanner = CodexDirectoryScanner::new(
        ApprovedCodexRoot::open(path.parent().unwrap())
            .unwrap()
            .unwrap(),
    )
    .unwrap();
    let candidate = scanner.step(100, 100).unwrap().candidates.remove(0);
    assert!(
        QualifiedCodexSource::inspect(CaptureProfile::CodexCliJsonlV1, candidate.clone()).is_err()
    );
    let qualified = QualifiedCodexSource::inspect(CaptureProfile::CodexDesktopJsonlV1, candidate)
        .unwrap()
        .unwrap();
    let sources = SourceRegistry::open(&root.path().join("sources"), [1; 16], 4).unwrap();
    let mut reader = sources
        .acquire(qualified)
        .unwrap()
        .open_reader(None)
        .unwrap()
        .unwrap();
    let mut spool = AgentCaptureSpool::open(&root.path().join("spool"), 4 * 1024 * 1024).unwrap();
    let profile = NormalizerProfileV1::codex_desktop_0_159_2();
    let first = ready(&mut reader);
    let ack = spool
        .enqueue_from_reader(first.sanitize_omitted(A, &profile).unwrap())
        .unwrap();
    reader.acknowledge(&ack).unwrap();
    let mut batch = ready(&mut reader);
    let ack = spool
        .enqueue_from_reader(batch.sanitize_omitted(B, &profile).unwrap())
        .unwrap();
    reader.acknowledge(&ack).unwrap();
    let mut entry = spool
        .pending()
        .unwrap()
        .into_iter()
        .find(|entry| entry.repository == B)
        .unwrap();
    let mut proof = reader.recover_batch(&entry).unwrap();
    assert!(
        matches!(proof.poll().unwrap(),CodexRecoveryPoll::Ready(context) if context.directory() == Some(Path::new("/workspace/b")))
    );
    let bytes = fs::read(&path).unwrap();
    let offset = bytes.len() as u64 - 1;
    let prefix: [u8; 32] = Sha256::digest(&bytes[..offset as usize]).into();
    batch.cursor.position = jj_agent::SourcePositionV1::JsonLines {
        complete_byte_offset: offset,
        prefix_sha256: prefix,
    };
    let mut id = Sha256::new_with_prefix(b"jj-store/codex-transcript-batch/v1\0");
    id.update(batch.cursor.source_instance_id.as_bytes());
    id.update(batch.cursor.generation.to_be_bytes());
    id.update(batch.metadata.generation_digest);
    id.update(offset.to_be_bytes());
    id.update(prefix);
    batch.batch_id = jj_agent::CaptureBatchId::new(id.finalize()[..16].try_into().unwrap());
    let capture = batch.sanitize_omitted(B, &profile).unwrap();
    entry.batch = capture.batch;
    entry.cursor = capture.cursor;
    assert!(reader.recover_batch(&entry).unwrap().poll().is_err());
}

#[test]
fn exact_range_reconstructs_changed_context_and_rejects_mixed_or_changed_identity() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("transcripts/session.jsonl");
    fs::create_dir(path.parent().unwrap()).unwrap();
    header(&path);
    append(&path, json!({"type":"message","text":"a"}));
    append(
        &path,
        json!({"type":"turn_context","payload":{"cwd":"/workspace/b"}}),
    );
    append(&path, json!({"type":"message","text":"b"}));
    let sources = SourceRegistry::open(&root.path().join("sources"), [1; 16], 4).unwrap();
    let mut reader = sources
        .acquire(qualified(&path))
        .unwrap()
        .open_reader(None)
        .unwrap()
        .unwrap();
    let mut spool = AgentCaptureSpool::open(&root.path().join("spool"), 4 * 1024 * 1024).unwrap();
    let first = ready(&mut reader);
    enqueue(&mut spool, &mut reader, &first, A);
    let second = ready(&mut reader);
    let entry = enqueue(&mut spool, &mut reader, &second, B);
    let mut proof = reader.recover_batch(&entry).unwrap();
    let CodexRecoveryPoll::Ready(directory) = proof.poll().unwrap() else {
        panic!("small prefix")
    };
    assert_eq!(directory.directory(), Some(Path::new("/workspace/b")));
    assert_eq!(directory.omission_anchor(), Some(Path::new("/workspace/a")));
    assert!(directory.omission_only());
    assert!(proof.revalidate().unwrap());
    let mut mixed = second.clone();
    mixed.records = Arc::new(
        first
            .records
            .iter()
            .chain(second.records.iter())
            .map(
                |row| jj_store_capture::codex_transcript::RawTranscriptRecord {
                    record_key: None,
                    sequence: row.sequence,
                    bytes: None,
                },
            )
            .collect(),
    );
    let capture = mixed.sanitize_omitted(B, &entry.profile).unwrap();
    let mut invalid = entry.clone();
    invalid.batch = capture.batch;
    assert!(reader.recover_batch(&invalid).unwrap().poll().is_err());
    let mut invalid = entry.clone();
    invalid.reader_metadata.as_mut().unwrap().generation_digest = [8; 32];
    assert!(reader.recover_batch(&invalid).is_err());
    let mut invalid = entry.clone();
    invalid.batch.observations[0].source_sequence += 1;
    assert!(reader.recover_batch(&invalid).is_err());
    let mut invalid = entry.clone();
    invalid.cursor.parser_version = "codex-jsonl@0.159.2/v1".into();
    assert!(reader.recover_batch(&invalid).is_err());
}

#[test]
fn long_prefix_recovery_is_bounded_and_restarts_after_append_or_rewrite() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("transcripts/session.jsonl");
    fs::create_dir(path.parent().unwrap()).unwrap();
    header(&path);
    for _ in 0..7 {
        append(&path, json!({"padding":"x".repeat(800*1024)}));
    }
    append(
        &path,
        json!({"type":"turn_context","payload":{"cwd":"/workspace/b"}}),
    );
    append(&path, json!({"text":"b"}));
    let sources = SourceRegistry::open(&root.path().join("sources"), [1; 16], 4).unwrap();
    let mut reader = sources
        .acquire(qualified(&path))
        .unwrap()
        .open_reader(None)
        .unwrap()
        .unwrap();
    let mut spool = AgentCaptureSpool::open(&root.path().join("spool"), 16 * 1024 * 1024).unwrap();
    let entry = loop {
        let batch = ready(&mut reader);
        let entry = enqueue(
            &mut spool,
            &mut reader,
            &batch,
            if batch.directory() == Some(Path::new("/workspace/b")) {
                B
            } else {
                A
            },
        );
        if entry.repository == B {
            break entry;
        }
    };
    let mut proof = reader.recover_batch(&entry).unwrap();
    assert!(matches!(
        proof.poll().unwrap(),
        CodexRecoveryPoll::Verifying
    ));
    append(&path, json!({"text":"later tail"}));
    assert!(matches!(
        proof.poll().unwrap(),
        CodexRecoveryPoll::Verifying
    ));
    assert!(matches!(proof.poll().unwrap(), CodexRecoveryPoll::Ready(_)));
    let original = fs::read(&path).unwrap();
    let mut changed = original.clone();
    let header_end = changed.iter().position(|v| *v == b'\n').unwrap() + 1;
    *changed[header_end..]
        .iter_mut()
        .find(|v| **v == b'x')
        .unwrap() = b'y';
    fs::write(&path, changed).unwrap();
    assert!(!proof.revalidate().unwrap());
    assert!(matches!(
        proof.poll().unwrap(),
        CodexRecoveryPoll::Verifying
    ));
    assert!(proof.poll().is_err());
    fs::write(&path, original).unwrap();
    assert!(matches!(
        proof.poll().unwrap(),
        CodexRecoveryPoll::Verifying
    ));
    assert!(matches!(proof.poll().unwrap(), CodexRecoveryPoll::Ready(_)));
    assert!(proof.revalidate().unwrap());
}

#[test]
fn unknown_context_body_cannot_gain_an_omission_anchor() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("transcripts/session.jsonl");
    fs::create_dir(path.parent().unwrap()).unwrap();
    header(&path);
    append(&path, json!({"type":"turn_context","payload":{"cwd":null}}));
    append(&path, json!({"text":"private"}));
    let sources = SourceRegistry::open(&root.path().join("sources"), [1; 16], 4).unwrap();
    let mut reader = sources
        .acquire(qualified(&path))
        .unwrap()
        .open_reader(None)
        .unwrap()
        .unwrap();
    let mut spool = AgentCaptureSpool::open(&root.path().join("spool"), 4 * 1024 * 1024).unwrap();
    let first = ready(&mut reader);
    enqueue(&mut spool, &mut reader, &first, A);
    let gap = ready(&mut reader);
    let mut entry = enqueue(&mut spool, &mut reader, &gap, A);
    entry.batch.observations[0].payload = jj_agent::SanitizedPayloadV1::new(
        jj_agent::SanitizedMediaType::Json,
        br#"{"record":"body evidence"}"#,
    )
    .unwrap();
    let mut proof = reader.recover_batch(&entry).unwrap();
    let CodexRecoveryPoll::Ready(directory) = proof.poll().unwrap() else {
        panic!("small prefix")
    };
    assert!(directory.directory().is_none());
    assert!(directory.omission_anchor().is_none());
    assert!(!directory.omission_only());
}

#[test]
fn fresh_process_recovers_exact_queued_range_from_retained_admitted_channel() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("transcripts/session.jsonl");
    fs::create_dir(path.parent().unwrap()).unwrap();
    header(&path);
    let mut probe =
        jj_store_capture::codex_activity::CodexActivityProbe::arm(qualified(&path)).unwrap();
    assert_eq!(
        probe.poll().unwrap(),
        jj_store_capture::codex_activity::ActivityPoll::Waiting
    );
    append(
        &path,
        json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"t"}}),
    );
    assert_eq!(
        probe.poll().unwrap(),
        jj_store_capture::codex_activity::ActivityPoll::Active
    );
    let sources = SourceRegistry::open(&root.path().join("sources"), [1; 16], 4).unwrap();
    let mut reader = sources
        .acquire_active(probe.proof().unwrap().unwrap())
        .unwrap()
        .open_reader(None)
        .unwrap()
        .unwrap();
    let mut spool = AgentCaptureSpool::open(&root.path().join("spool"), 4 * 1024 * 1024).unwrap();
    let first = ready(&mut reader);
    enqueue(&mut spool, &mut reader, &first, A);
    append(
        &path,
        json!({"type":"turn_context","payload":{"cwd":"/workspace/b"}}),
    );
    append(&path, json!({"text":"queued b"}));
    let batch = ready(&mut reader);
    let entry = enqueue(&mut spool, &mut reader, &batch, B);
    let edits = vec![jj_agent::AgentCatalogEditV1 {
        catalog: jj_agent::AgentCatalogKind::Sessions,
        key: jj_agent::AgentCatalogKey::derive("test", b"session").unwrap(),
        expected: None,
        replacement: Some(jj_agent::AgentObjectRef::new(
            jj_agent::AgentObjectKind::SessionSnapshot,
            jj_agent::AgentObjectId::new([8; 32]),
        )),
    }];
    let request = jj_store_wal::publication::AgentDeltaRequest::new(
        jj_store_model::PrincipalId::new([4; 16]),
        jj_store_model::RequestId::new([5; 16]),
        jj_store_model::PublicationId::new([6; 16]),
        jj_agent::AgentDeltaV1 {
            schema_version: 1,
            edits,
        },
        vec![7],
    )
    .unwrap();
    spool.prepare(entry.batch.batch_id, request).unwrap();
    let retained = spool.pending().unwrap();
    drop(reader);
    drop(spool);
    let output = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "queued_recovery_child", "--ignored"])
        .env("JJ_STORE_CODEX_RECOVERY_TEST_ROOT", root.path())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        AgentCaptureSpool::open(&root.path().join("spool"), 4 * 1024 * 1024)
            .unwrap()
            .pending()
            .unwrap(),
        retained
    );
}

#[test]
#[ignore = "fresh process fixture invoked by fresh_process_recovers_exact_queued_range_from_retained_admitted_channel"]
fn queued_recovery_child() {
    let root =
        std::path::PathBuf::from(std::env::var_os("JJ_STORE_CODEX_RECOVERY_TEST_ROOT").unwrap());
    let spool = AgentCaptureSpool::open(&root.join("spool"), 4 * 1024 * 1024).unwrap();
    let sources = SourceRegistry::open_existing(&root.join("sources"), [1; 16], 4)
        .unwrap()
        .unwrap();
    let source = sources
        .acquire_admitted(qualified(&root.join("transcripts/session.jsonl")))
        .unwrap()
        .unwrap();
    let progress = spool
        .latest_reader_progress(IntegrationId::Codex, source.source_instance_id())
        .unwrap();
    let reader = source.open_reader(progress).unwrap().unwrap();
    let entry = spool
        .pending()
        .unwrap()
        .into_iter()
        .find(|entry| entry.repository == B)
        .unwrap();
    assert!(matches!(
        entry.state,
        jj_store_capture::agent_capture::PendingCaptureStateV1::Prepared { .. }
    ));
    let mut probe = reader.recover_batch(&entry).unwrap();
    let CodexRecoveryPoll::Ready(directory) = probe.poll().unwrap() else {
        panic!("small prefix")
    };
    assert_eq!(reader.directory(), Path::new("/workspace/a"));
    assert_eq!(directory.directory(), Some(Path::new("/workspace/b")));
    assert!(reader.revalidate().unwrap() && probe.revalidate().unwrap());
}
