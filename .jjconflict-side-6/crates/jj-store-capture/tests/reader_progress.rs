use jj_agent::*;
use jj_store_capture::{
    agent_capture::{AgentCaptureSpool, PendingCaptureStateV1},
    reader_progress::{ReaderCapture, ReaderCaptureMetadataV1},
};
use jj_store_model::RepositoryId;
use std::{fs, path::Path};
#[path = "reader_progress/source.rs"]
mod source;
const REPO: RepositoryId = RepositoryId::new([1; 16]);
const SOURCE: SourceInstanceId = SourceInstanceId::new([2; 16]);
fn input(sequence: u64, generation: u64) -> ReaderCapture {
    ReaderCapture {
        repository: REPO,
        profile: NormalizerProfileV1::codex_0_156_1(),
        cursor: CaptureCursorV1 {
            schema_version: 1,
            integration: IntegrationId::Codex,
            source_instance_id: SOURCE,
            generation,
            parser_version: "codex-jsonl@0.156.1/v1".into(),
            native_session_key: Some("session".into()),
            position: SourcePositionV1::JsonLines {
                complete_byte_offset: sequence * 100,
                prefix_sha256: [sequence as u8; 32],
            },
        },
        batch: SanitizedObservationBatchV1 {
            schema_version: 1,
            batch_id: CaptureBatchId::new([sequence as u8; 16]),
            integration: IntegrationId::Codex,
            source_instance_id: SOURCE,
            sanitizer_version: jj_store_capture::agent_sanitization::KINGFISHER_POLICY.into(),
            representation: SanitizedRepresentation::SanitizedNative,
            observations: vec![SanitizedObservationV1 {
                observation_id: ObservationId::new([sequence as u8; 16]),
                source_sequence: sequence,
                occurred_at_millis: Some(1),
                source_kind: SourceKind::Transcript,
                source_trust: SourceTrust::LocalTranscript,
                payload: SanitizedPayloadV1::new(
                    SanitizedMediaType::Json,
                    br#"{"status":"omitted"}"#,
                )
                .unwrap(),
            }],
        },
        metadata: ReaderCaptureMetadataV1 {
            schema_version: 1,
            generation_digest: [generation as u8; 32],
            opencode_snapshot: None,
        },
        thread_binding: None,
    }
}

#[test]
fn repository_partitions_share_one_durable_source_cursor_and_cannot_replay_elsewhere() {
    let root = tempfile::tempdir().unwrap();
    let other = RepositoryId::new([9; 16]);
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool.enqueue_from_reader(input(1, 1)).unwrap();
    let mut second = input(2, 1);
    second.repository = other;
    spool.enqueue_from_reader(second).unwrap();
    let latest = spool
        .latest_reader_progress(IntegrationId::Codex, SOURCE)
        .unwrap()
        .unwrap();
    assert_eq!(latest.repository, other);
    assert_eq!(latest.last_sequence, 2);
    let mut wrong = input(3, 1);
    wrong.metadata.generation_digest = [44; 32];
    assert!(spool.enqueue_from_reader(wrong).is_err());
    let request = native_request();
    for sequence in [1, 2] {
        let batch = input(sequence, 1).batch.batch_id;
        spool.prepare(batch, request.clone()).unwrap();
        spool
            .complete(batch, request.request_id, request.publication_id)
            .unwrap();
    }
    assert!(spool.pending().unwrap().is_empty());
    let mut replay = input(2, 1);
    replay.repository = RepositoryId::new([10; 16]);
    assert!(spool.enqueue_from_reader(replay).is_err());
    drop(spool);
    let reopened = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    assert_eq!(
        reopened
            .latest_reader_progress(IntegrationId::Codex, SOURCE)
            .unwrap(),
        Some(latest)
    );
}

#[test]
fn partitioned_pending_recovery_repairs_all_repository_journals_and_rejects_conflicts() {
    let root = tempfile::tempdir().unwrap();
    let other = RepositoryId::new([9; 16]);
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool.enqueue_from_reader(input(1, 1)).unwrap();
    let mut second = input(2, 1);
    second.repository = other;
    spool.enqueue_from_reader(second).unwrap();
    let pending = spool.pending().unwrap();
    drop(spool);
    fs::remove_dir_all(root.path().join("reader-progress")).unwrap();
    let spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    assert_eq!(spool.pending().unwrap(), pending);
    assert_eq!(
        spool
            .reader_progress(REPO, IntegrationId::Codex, SOURCE, 1)
            .unwrap()
            .unwrap()
            .last_sequence,
        1
    );
    assert_eq!(
        spool
            .latest_reader_progress(IntegrationId::Codex, SOURCE)
            .unwrap()
            .unwrap()
            .last_sequence,
        2
    );
    let mut conflicting = spool
        .reader_progress(REPO, IntegrationId::Codex, SOURCE, 1)
        .unwrap()
        .unwrap();
    conflicting.repository = RepositoryId::new([10; 16]);
    let path = root.path().join("reader-progress").join(format!(
        "{}-{}-{SOURCE}-1.json",
        conflicting.repository,
        IntegrationId::Codex as u8
    ));
    fs::write(&path, serde_json::to_vec(&conflicting).unwrap()).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
    }
    assert!(
        spool
            .latest_reader_progress(IntegrationId::Codex, SOURCE)
            .is_err()
    );
    drop(spool);
    assert!(AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).is_err());
    assert!(path.exists());
}
#[test]
fn reader_cursor_is_durable_before_acknowledgement_and_survives_reopening() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    let candidate = input(1, 1);
    let cursor = candidate.cursor.clone();
    let ack = spool.enqueue_from_reader(candidate).unwrap();
    assert_eq!(ack.position, cursor.position);
    let progress = spool
        .reader_progress(REPO, IntegrationId::Codex, SOURCE, 1)
        .unwrap()
        .unwrap();
    assert_eq!(progress.cursor, cursor);
    assert_eq!(progress.last_sequence, 1);
    drop(spool);
    let reopened = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    assert_eq!(
        reopened
            .reader_progress(REPO, IntegrationId::Codex, SOURCE, 1)
            .unwrap(),
        Some(progress)
    );
    assert_eq!(reopened.pending().unwrap().len(), 1);
}
fn progress_path(root: &Path, generation: u64) -> std::path::PathBuf {
    root.join("reader-progress").join(format!(
        "{REPO}-{}-{SOURCE}-{generation}.json",
        IntegrationId::Codex as u8
    ))
}
fn native_request() -> jj_store_wal::publication::AgentDeltaRequest {
    use jj_store_model::{PrincipalId, PublicationId, RequestId};
    jj_store_wal::publication::AgentDeltaRequest::new(
        PrincipalId::new([3; 16]),
        RequestId::new([4; 16]),
        PublicationId::new([5; 16]),
        AgentDeltaV1 {
            schema_version: 1,
            edits: vec![AgentCatalogEditV1 {
                catalog: AgentCatalogKind::Sessions,
                key: AgentCatalogKey::derive("reader-test", b"session").unwrap(),
                expected: None,
                replacement: Some(AgentObjectRef::new(
                    AgentObjectKind::SessionSnapshot,
                    AgentObjectId::new([6; 32]),
                )),
            }],
        },
        vec![1],
    )
    .unwrap()
}
#[test]
fn native_and_local_prepared_intents_recover_missing_progress_and_keep_it_after_completion() {
    use jj_store_service::local_repository::{LocalAgentCommand, LocalAgentRequest};
    for local in [false, true] {
        let root = tempfile::tempdir().unwrap();
        let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
        let candidate = input(1, 1);
        let batch = candidate.batch.batch_id;
        let native = native_request();
        let local_request = LocalAgentRequest {
            version: 1,
            request_id: native.request_id,
            command: LocalAgentCommand::Capture {
                profile: candidate.profile.clone(),
                batch: Box::new(candidate.batch.clone()),
            },
        };
        spool.enqueue_from_reader(candidate).unwrap();
        if local {
            spool.prepare_local(batch, local_request).unwrap();
        } else {
            spool.prepare(batch, native.clone()).unwrap();
        }
        let prepared = spool.pending().unwrap();
        let progress = spool
            .reader_progress(REPO, IntegrationId::Codex, SOURCE, 1)
            .unwrap();
        drop(spool);
        fs::remove_file(progress_path(root.path(), 1)).unwrap();
        let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
        assert_eq!(spool.pending().unwrap(), prepared);
        assert_eq!(
            spool
                .reader_progress(REPO, IntegrationId::Codex, SOURCE, 1)
                .unwrap(),
            progress
        );
        // Completion repairs progress before removing the exact prepared request.
        fs::remove_file(progress_path(root.path(), 1)).unwrap();
        if local {
            spool.complete_local(batch, native.request_id).unwrap();
        } else {
            spool
                .complete(batch, native.request_id, native.publication_id)
                .unwrap();
        }
        assert!(spool.pending().unwrap().is_empty());
        drop(spool);
        let spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
        assert_eq!(
            spool
                .reader_progress(REPO, IntegrationId::Codex, SOURCE, 1)
                .unwrap(),
            progress
        );
    }
}
#[test]
fn cursor_failure_keeps_exact_prepared_native_and_local_requests() {
    use jj_store_service::local_repository::{LocalAgentCommand, LocalAgentRequest};
    for local in [false, true] {
        let root = tempfile::tempdir().unwrap();
        let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
        let candidate = input(1, 1);
        let batch = candidate.batch.batch_id;
        let native = native_request();
        let local_request = LocalAgentRequest {
            version: 1,
            request_id: native.request_id,
            command: LocalAgentCommand::Capture {
                profile: candidate.profile.clone(),
                batch: Box::new(candidate.batch.clone()),
            },
        };
        spool.enqueue_from_reader(candidate).unwrap();
        if local {
            spool.prepare_local(batch, local_request).unwrap();
        } else {
            spool.prepare(batch, native.clone()).unwrap();
        }
        let prepared = spool.pending().unwrap();
        let path = progress_path(root.path(), 1);
        fs::remove_file(&path).unwrap();
        fs::create_dir(&path).unwrap();
        let result = if local {
            spool.complete_local(batch, native.request_id)
        } else {
            spool.complete(batch, native.request_id, native.publication_id)
        };
        assert!(result.is_err());
        assert_eq!(spool.pending().unwrap(), prepared);
        fs::remove_dir(&path).unwrap();
        drop(spool);
        let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
        assert_eq!(spool.pending().unwrap(), prepared);
        if local {
            spool.complete_local(batch, native.request_id).unwrap();
        } else {
            spool
                .complete(batch, native.request_id, native.publication_id)
                .unwrap();
        }
        assert!(spool.pending().unwrap().is_empty());
    }
}
#[test]
fn inconsistent_or_regressive_readers_never_admit_another_pending_batch() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool.enqueue_from_reader(input(1, 1)).unwrap();
    for change in 0..4 {
        let mut candidate = input(2, 1);
        match change {
            0 => candidate.metadata.generation_digest = [9; 32],
            1 => candidate.cursor.parser_version = "other-parser".into(),
            2 => candidate.cursor.position = input(1, 1).cursor.position,
            _ => candidate.batch.observations[0].source_sequence = 1,
        }
        assert!(spool.enqueue_from_reader(candidate).is_err());
        assert_eq!(spool.pending().unwrap().len(), 1);
    }
    spool.enqueue_from_reader(input(2, 1)).unwrap();
    // Missing progress must not let a live handle bypass already durable identity.
    fs::remove_file(progress_path(root.path(), 1)).unwrap();
    let mut candidate = input(3, 1);
    candidate.cursor.parser_version = "other-parser".into();
    assert!(spool.enqueue_from_reader(candidate).is_err());
    assert_eq!(spool.pending().unwrap().len(), 2);
}
#[test]
fn generations_and_older_retained_retries_do_not_regress_progress() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool.enqueue_from_reader(input(1, 1)).unwrap();
    spool.enqueue_from_reader(input(2, 1)).unwrap();
    spool.enqueue_from_reader(input(1, 1)).unwrap();
    let mut second_generation = input(1, 2);
    second_generation.batch.batch_id = CaptureBatchId::new([20; 16]);
    spool.enqueue_from_reader(second_generation).unwrap();
    drop(spool);
    let spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    assert_eq!(
        spool
            .reader_progress(REPO, IntegrationId::Codex, SOURCE, 1)
            .unwrap()
            .unwrap()
            .last_sequence,
        2
    );
    assert_eq!(
        spool
            .reader_progress(REPO, IntegrationId::Codex, SOURCE, 2)
            .unwrap()
            .unwrap()
            .last_sequence,
        1
    );
}
#[test]
fn live_handles_clean_unacknowledged_temporaries_and_legacy_bytes_stay_unchanged() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    let candidate = input(1, 1);
    spool
        .enqueue(REPO, candidate.profile, candidate.cursor, candidate.batch)
        .unwrap();
    let pending = spool.pending().unwrap()[0].clone();
    let pending_bytes = fs::read(
        root.path()
            .join("pending")
            .join(format!("{}.json", pending.batch.batch_id)),
    )
    .unwrap();
    let value: serde_json::Value = serde_json::from_slice(&pending_bytes).unwrap();
    assert!(value.get("reader_metadata").is_none());
    assert!(!root.path().join("reader-progress").exists());
    assert_eq!(serde_json::to_vec(&pending).unwrap(), pending_bytes);
    spool.enqueue_from_reader(input(2, 1)).unwrap();
    fs::write(
        root.path().join("reader-progress/.tmpinterrupted"),
        b"unacknowledged",
    )
    .unwrap();
    assert_eq!(
        spool
            .reader_progress(REPO, IntegrationId::Codex, SOURCE, 1)
            .unwrap()
            .unwrap()
            .last_sequence,
        2
    );
    assert!(!root.path().join("reader-progress/.tmpinterrupted").exists());
    spool.enqueue_from_reader(input(3, 1)).unwrap();
}
#[test]
fn corrupt_oversized_and_symlinked_progress_fails_closed() {
    for fault in 0..3 {
        let root = tempfile::tempdir().unwrap();
        let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
        spool.enqueue_from_reader(input(1, 1)).unwrap();
        let path = progress_path(root.path(), 1);
        let bytes = fs::read(&path).unwrap();
        match fault {
            0 => fs::write(&path, b"invalid").unwrap(),
            1 => fs::write(&path, vec![b' '; 8193]).unwrap(),
            _ => {
                fs::remove_file(&path).unwrap();
                #[cfg(unix)]
                std::os::unix::fs::symlink(root.path().join("outside"), &path).unwrap();
                #[cfg(not(unix))]
                fs::create_dir(&path).unwrap();
            }
        }
        assert!(spool.enqueue_from_reader(input(2, 1)).is_err());
        assert_eq!(spool.pending().unwrap().len(), 1);
        drop(spool);
        assert!(AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).is_err());
        if fault == 2 {
            fs::remove_file(&path).unwrap();
        }
        fs::write(&path, bytes).unwrap();
    }
}
#[test]
fn full_progress_journal_retains_old_generations_and_allows_existing_updates() {
    use std::io::Write as _;
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool.enqueue_from_reader(input(1, 1)).unwrap();
    let template = spool
        .reader_progress(REPO, IntegrationId::Codex, SOURCE, 1)
        .unwrap()
        .unwrap();
    for generation in 2..=4096 {
        let mut progress = template.clone();
        progress.cursor.generation = generation;
        let mut options = fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            options.mode(0o600);
        }
        options
            .open(progress_path(root.path(), generation))
            .unwrap()
            .write_all(&serde_json::to_vec(&progress).unwrap())
            .unwrap();
    }
    spool.enqueue_from_reader(input(2, 1)).unwrap();
    assert!(spool.enqueue_from_reader(input(3, 4097)).is_err());
    assert_eq!(spool.pending().unwrap().len(), 2);
    assert_eq!(
        fs::read_dir(root.path().join("reader-progress"))
            .unwrap()
            .count(),
        4096
    );
    drop(spool);
    let spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    assert_eq!(
        spool
            .reader_progress(REPO, IntegrationId::Codex, SOURCE, 4096)
            .unwrap()
            .unwrap()
            .last_sequence,
        1
    );
    assert_eq!(
        spool
            .reader_progress(REPO, IntegrationId::Codex, SOURCE, 1)
            .unwrap()
            .unwrap()
            .last_sequence,
        2
    );
}
#[test]
fn concurrent_reader_handles_share_mutation_and_pending_limits() {
    use jj_store_capture::state_lease::StateLease;
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool.enqueue_from_reader(input(1, 1)).unwrap();
    let pending = spool.pending().unwrap()[0].clone();
    let limit = fs::metadata(
        root.path()
            .join("pending")
            .join(format!("{}.json", pending.batch.batch_id)),
    )
    .unwrap()
    .len()
        + 4;
    let request = native_request();
    spool
        .prepare(pending.batch.batch_id, request.clone())
        .unwrap();
    spool
        .complete(
            pending.batch.batch_id,
            request.request_id,
            request.publication_id,
        )
        .unwrap();
    drop(spool);
    let mut first = AgentCaptureSpool::open(root.path(), limit).unwrap();
    let mut second = AgentCaptureSpool::open(root.path(), limit).unwrap();
    let lease = StateLease::exclusive(&root.path().join("mutation.lock")).unwrap();
    assert!(first.enqueue_from_reader(input(2, 1)).is_err());
    assert!(second.enqueue_from_reader(input(3, 2)).is_err());
    drop(lease);
    let barrier = std::sync::Barrier::new(2);
    let (first_ok, second_ok) = std::thread::scope(|scope| {
        let a = scope.spawn(|| {
            barrier.wait();
            first.enqueue_from_reader(input(2, 1)).is_ok()
        });
        let b = scope.spawn(|| {
            barrier.wait();
            second.enqueue_from_reader(input(3, 2)).is_ok()
        });
        (a.join().unwrap(), b.join().unwrap())
    });
    assert_ne!(first_ok, second_ok);
    assert_eq!(first.pending().unwrap().len(), 1);
    assert_eq!(
        first
            .reader_progress(REPO, IntegrationId::Codex, SOURCE, 1)
            .unwrap()
            .unwrap()
            .last_sequence,
        if first_ok { 2 } else { 1 }
    );
    assert_eq!(
        first
            .reader_progress(REPO, IntegrationId::Codex, SOURCE, 2)
            .unwrap()
            .is_some(),
        second_ok
    );
}

#[test]
fn forked_spool_handles_keep_compatibility_lease_and_shared_durable_progress() {
    let root = tempfile::tempdir().unwrap();
    let spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    let mut forked = spool.fork_handle();
    let read = forked.fork_handle();
    drop(spool);
    assert!(
        jj_store_capture::state_lease::StateLease::exclusive(&root.path().join("capture.lock"))
            .is_err()
    );
    forked.enqueue_from_reader(input(1, 1)).unwrap();
    assert_eq!(read.pending().unwrap().len(), 1);
    assert_eq!(
        read.latest_reader_progress(IntegrationId::Codex, SOURCE)
            .unwrap()
            .unwrap()
            .last_sequence,
        1
    );
    let request = native_request();
    let batch = read.pending().unwrap()[0].batch.batch_id;
    forked.prepare(batch, request.clone()).unwrap();
    assert_eq!(
        read.pending().unwrap()[0].state,
        PendingCaptureStateV1::Prepared {
            request: request.clone()
        }
    );
    forked
        .complete(batch, request.request_id, request.publication_id)
        .unwrap();
    assert!(read.pending().unwrap().is_empty());
    drop(forked);
    assert!(
        jj_store_capture::state_lease::StateLease::exclusive(&root.path().join("capture.lock"))
            .is_err()
    );
    drop(read);
    assert!(
        jj_store_capture::state_lease::StateLease::exclusive(&root.path().join("capture.lock"))
            .is_ok()
    );
}

#[path = "reader_progress/headroom.rs"]
mod headroom;
