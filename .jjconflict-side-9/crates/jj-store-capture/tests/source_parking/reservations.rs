use super::*;
use jj_store_capture::{
    codex_activity::{ActivityPoll, CodexActivityProbe},
    reader_progress::ReaderSourceProgress,
    source_registry::{LeasedCodexTranscriptReader, ParkedCodexTranscriptReader},
};

fn bounds() -> ReaderSourceProgress {
    ReaderSourceProgress {
        progress: None,
        generation_floor: 0,
        sequence_floor: 0,
    }
}
fn active(registry: &SourceRegistry, root: &Path, path: &Path) -> LeasedCodexTranscriptReader {
    let mut probe = CodexActivityProbe::arm(qualified(root, path)).unwrap();
    assert_eq!(probe.poll().unwrap(), ActivityPoll::Waiting);
    writeln!(
        fs::OpenOptions::new().append(true).open(path).unwrap(),
        "{}",
        json!({"type":"event_msg","payload":{"type":"task_started"}})
    )
    .unwrap();
    for _ in 0..16 {
        if probe.poll().unwrap() == ActivityPoll::Active {
            break;
        }
    }
    registry
        .acquire_active(probe.proof().unwrap().unwrap())
        .unwrap()
        .open_reader_at(&bounds())
        .unwrap()
        .unwrap()
}
fn raw(
    reader: &mut LeasedCodexTranscriptReader,
) -> jj_store_capture::codex_transcript::CodexTranscriptBatch {
    let TranscriptPoll::Ready(batch) = reader.poll().unwrap() else {
        panic!("expected raw batch")
    };
    *batch
}
fn resume(
    parked: &mut ParkedCodexTranscriptReader,
    registry: &SourceRegistry,
) -> LeasedCodexTranscriptReader {
    let acquired = parked.acquire_reserved(registry).unwrap().unwrap();
    parked
        .resume_reserved_at(acquired, &bounds(), false)
        .unwrap()
        .unwrap()
}
fn channel_lock(state: &Path) -> std::path::PathBuf {
    fs::read_dir(state)
        .unwrap()
        .map(|e| e.unwrap().path())
        .find(|p| {
            p.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("source-")
        })
        .unwrap()
}

#[test]
fn raw_reservation_excludes_alias_and_rotated_readers_but_releases_physical_ownership() {
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    let archive = root.path().join("archive.jsonl");
    let unrelated = root.path().join("other.jsonl");
    write_header(&path, "00000000-0000-0000-0000-000000000001");
    let registry = SourceRegistry::open(&private.path().join("registry"), [8; 16], 4).unwrap();
    let source = active(&registry, root.path(), &path).source_instance_id();
    fs::rename(&path, &archive).unwrap();
    write_header(&path, "00000000-0000-0000-0000-000000000001");
    assert_eq!(
        registry
            .acquire_admitted(qualified(root.path(), &path))
            .unwrap()
            .unwrap()
            .source_instance_id(),
        source
    );
    let mut reader = registry
        .acquire_admitted(qualified(root.path(), &archive))
        .unwrap()
        .unwrap()
        .open_reader_at(&bounds())
        .unwrap()
        .unwrap();
    let original = raw(&mut reader);
    let alias = root.path().join("alias.jsonl");
    fs::hard_link(&archive, &alias).unwrap();
    let mut parked = reader.park_automatic();
    assert!(parked.has_channel_reservation());
    assert!(!parked.compact_idle());
    assert!(
        registry
            .acquire_admitted(qualified(root.path(), &path))
            .is_err()
    );
    assert!(registry.acquire(qualified(root.path(), &alias)).is_err());
    for entry in fs::read_dir(private.path().join("registry")).unwrap() {
        let path = entry.unwrap().path();
        if path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("physical-")
        {
            let lock = fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(path)
                .unwrap();
            lock.try_lock().expect("parked physical lease released");
            lock.unlock().unwrap();
        }
    }
    write_header(&unrelated, "00000000-0000-0000-0000-000000000002");
    drop(
        registry
            .acquire(qualified(root.path(), &unrelated))
            .unwrap(),
    );
    // Abandoned physical reacquisition is a cancelled handoff, not channel release.
    drop(parked.acquire_reserved(&registry).unwrap().unwrap());
    assert!(
        registry
            .acquire_admitted(qualified(root.path(), &path))
            .is_err()
    );
    let mut reader = resume(&mut parked, &registry);
    let retained = raw(&mut reader);
    assert_eq!(retained.batch_id, original.batch_id);
    assert_eq!(retained.cursor, original.cursor);
    assert!(Arc::ptr_eq(&retained.records, &original.records));
    assert!(Arc::ptr_eq(
        &retained.directory_context,
        &original.directory_context
    ));
    reader
        .acknowledge(&jj_store_capture::agent_capture::DurableCaptureAck {
            batch_id: retained.batch_id,
            position: retained.cursor.position,
        })
        .unwrap();
    let idle = reader.park_automatic();
    assert!(!idle.has_channel_reservation());
    assert!(idle.idle_unchanged().unwrap());
    drop(
        registry
            .acquire_admitted(qualified(root.path(), &path))
            .unwrap()
            .unwrap(),
    );
    assert!(
        parked.acquire_reserved(&registry).is_err(),
        "consumed exactly once"
    );
}

#[test]
fn changed_admission_or_allocation_retains_exact_raw_channel_for_retry() {
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    write_header(&path, "00000000-0000-0000-0000-000000000001");
    let state = private.path().join("registry");
    let registry = SourceRegistry::open(&state, [8; 16], 4).unwrap();
    let mut reader = active(&registry, root.path(), &path);
    let original = raw(&mut reader);
    let mut parked = reader.park_automatic();
    let catalog = fs::read(state.join("sources.json")).unwrap();
    let mut revoked: serde_json::Value = serde_json::from_slice(&catalog).unwrap();
    revoked["sources"][0]["admitted_sessions"] = json!([]);
    fs::write(
        state.join("sources.json"),
        serde_json::to_vec(&revoked).unwrap(),
    )
    .unwrap();
    assert!(parked.acquire_reserved(&registry).unwrap().is_none());
    fs::write(state.join("sources.json"), &catalog).unwrap();
    let acquired = parked.acquire_reserved(&registry).unwrap().unwrap();
    assert!(
        parked
            .resume_reserved_at(
                acquired,
                &ReaderSourceProgress {
                    progress: None,
                    generation_floor: 2,
                    sequence_floor: 100,
                },
                false
            )
            .is_err()
    );
    assert!(parked.has_channel_reservation());
    assert!(
        registry
            .acquire_admitted(qualified(root.path(), &path))
            .is_err()
    );
    let retained = raw(&mut resume(&mut parked, &registry));
    assert_eq!(retained.batch_id, original.batch_id);
    assert_eq!(retained.cursor, original.cursor);
    assert!(Arc::ptr_eq(&retained.records, &original.records));
}

#[test]
fn foreign_registry_or_replaced_channel_lock_cannot_transfer_a_raw_reservation() {
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    write_header(&path, "00000000-0000-0000-0000-000000000001");
    let state = private.path().join("registry");
    let foreign = private.path().join("foreign");
    let registry = SourceRegistry::open(&state, [8; 16], 4).unwrap();
    let other = SourceRegistry::open(&foreign, [8; 16], 4).unwrap();
    let mut reader = active(&registry, root.path(), &path);
    let original = raw(&mut reader);
    let mut parked = reader.park_automatic();
    fs::copy(state.join("sources.json"), foreign.join("sources.json")).unwrap();
    assert_eq!(
        other
            .lookup_admitted(&qualified(root.path(), &path))
            .unwrap(),
        Some(parked.source_instance_id())
    );
    assert!(parked.acquire_reserved(&other).is_err());
    let mut other_parked = other
        .acquire_admitted(qualified(root.path(), &path))
        .unwrap()
        .unwrap()
        .open_reader_at(&bounds())
        .unwrap()
        .unwrap();
    raw(&mut other_parked);
    let other_parked = other_parked.park_automatic();
    let foreign_physical = other_parked.acquire_reserved(&other).unwrap().unwrap();
    assert!(
        parked
            .resume_reserved_at(foreign_physical, &bounds(), false)
            .is_err()
    );
    let lock = channel_lock(&state);
    let retained_lock = state.join("retained-lock");
    fs::rename(&lock, &retained_lock).unwrap();
    let replacement = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(&lock)
        .unwrap();
    use std::os::unix::fs::PermissionsExt as _;
    replacement
        .set_permissions(fs::Permissions::from_mode(0o600))
        .unwrap();
    assert!(parked.acquire_reserved(&registry).is_err());
    fs::remove_file(&lock).unwrap();
    fs::rename(&retained_lock, &lock).unwrap();
    let reopened = SourceRegistry::open(&state, [8; 16], 4).unwrap();
    let retained = raw(&mut resume(&mut parked, &reopened));
    assert_eq!(retained.batch_id, original.batch_id);
    assert!(Arc::ptr_eq(&retained.records, &original.records));
}

#[test]
fn physical_exclusion_failed_reopen_and_source_replacement_retain_raw_reservation() {
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    write_header(&path, "00000000-0000-0000-0000-000000000001");
    let state = private.path().join("registry");
    let registry = SourceRegistry::open(&state, [8; 16], 4).unwrap();
    let mut reader = active(&registry, root.path(), &path);
    let original = raw(&mut reader);
    let mut parked = reader.park_automatic();
    let lock = fs::read_dir(&state)
        .unwrap()
        .map(|e| e.unwrap().path())
        .find(|p| {
            p.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("physical-")
        })
        .unwrap();
    let held = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(lock)
        .unwrap();
    held.try_lock().unwrap();
    assert!(parked.acquire_reserved(&registry).is_err());
    held.unlock().unwrap();
    let mut invalid = bounds();
    invalid.progress = Some(jj_store_capture::reader_progress::ReaderProgressV1 {
        schema_version: 0,
        repository: jj_store_model::RepositoryId::new([1; 16]),
        cursor: original.cursor.clone(),
        metadata: original.metadata.clone(),
        last_sequence: original.records.last().unwrap().sequence,
        last_batch: original.batch_id,
    });
    let acquired = parked.acquire_reserved(&registry).unwrap().unwrap();
    assert!(parked.resume_reserved_at(acquired, &invalid, true).is_err());
    let acquired = parked.acquire_reserved(&registry).unwrap().unwrap();
    let old = root.path().join("old");
    fs::rename(&path, &old).unwrap();
    fs::copy(&old, &path).unwrap();
    assert!(
        parked
            .resume_reserved_at(acquired, &bounds(), false)
            .is_err()
    );
    fs::remove_file(&path).unwrap();
    fs::rename(&old, &path).unwrap();
    let retained = raw(&mut resume(&mut parked, &registry));
    assert_eq!(retained.cursor, original.cursor);
    assert!(Arc::ptr_eq(&retained.records, &original.records));
}

#[test]
fn parked_owner_death_releases_ephemeral_channel_without_serializing_raw_state() {
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    write_header(&path, "00000000-0000-0000-0000-000000000001");
    let state = private.path().join("registry");
    let registry = SourceRegistry::open(&state, [8; 16], 4).unwrap();
    let mut reader = active(&registry, root.path(), &path);
    raw(&mut reader);
    let before = fs::read(state.join("sources.json")).unwrap();
    let parked = reader.park_automatic();
    assert!(
        registry
            .acquire_admitted(qualified(root.path(), &path))
            .is_err()
    );
    drop(parked);
    assert_eq!(fs::read(state.join("sources.json")).unwrap(), before);
    drop(
        registry
            .acquire_admitted(qualified(root.path(), &path))
            .unwrap()
            .unwrap(),
    );
    assert!(!String::from_utf8(before).unwrap().contains("session_meta"));
}

#[test]
fn automatic_partial_scan_parking_releases_both_leases_and_preserves_record_bytes() {
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    write_header(&path, "00000000-0000-0000-0000-000000000001");
    let registry = SourceRegistry::open(&private.path().join("registry"), [8; 16], 4).unwrap();
    let mut reader = active(&registry, root.path(), &path);
    let accepted = raw(&mut reader);
    reader
        .acknowledge(&jj_store_capture::agent_capture::DurableCaptureAck {
            batch_id: accepted.batch_id,
            position: accepted.cursor.position,
        })
        .unwrap();
    let incomplete = format!("{{\"partial\":\"{}", "x".repeat(900_000));
    fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .unwrap()
        .write_all(incomplete.as_bytes())
        .unwrap();
    assert!(matches!(reader.poll().unwrap(), TranscriptPoll::Waiting));
    let mut parked = reader.park_automatic();
    assert!(!parked.has_channel_reservation());
    assert!(parked.compact_idle());
    fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .unwrap()
        .write_all(b"\"}\n")
        .unwrap();
    let registered = registry
        .acquire_admitted(qualified(root.path(), &path))
        .unwrap()
        .unwrap();
    let mut reader = parked.resume(registered).unwrap();
    let retained = raw(&mut reader);
    assert_eq!(retained.records.len(), 1);
    assert_eq!(
        retained.records[0].sequence,
        accepted.records.last().unwrap().sequence + 1
    );
    assert_eq!(
        retained.records[0].bytes.as_deref().unwrap(),
        format!("{incomplete}\"}}\n").as_bytes()
    );
}

#[test]
fn durable_exact_raw_acceptance_reopens_with_prefix_verification_and_releases_channel_at_idle() {
    use jj_store_capture::agent_capture::AgentCaptureSpool;
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    write_header(&path, "00000000-0000-0000-0000-000000000001");
    let registry = SourceRegistry::open(&private.path().join("registry"), [8; 16], 4).unwrap();
    let mut reader = active(&registry, root.path(), &path);
    let original = raw(&mut reader);
    let source = reader.source_instance_id();
    let mut parked = reader.park_automatic();
    let mut spool =
        AgentCaptureSpool::open(&private.path().join("spool"), 4 * 1024 * 1024).unwrap();
    let capture = original
        .sanitize_omitted(
            jj_store_model::RepositoryId::new([1; 16]),
            &jj_agent::NormalizerProfileV1::codex_0_156_1(),
        )
        .unwrap();
    spool.enqueue_from_automatic_reader(capture).unwrap();
    let progress = spool
        .source_reader_progress(
            jj_agent::IntegrationId::Codex,
            source,
            original.metadata.generation_digest,
        )
        .unwrap();
    let acquired = parked.acquire_reserved(&registry).unwrap().unwrap();
    let mut reader = parked
        .resume_reserved_at(acquired, &progress, true)
        .unwrap()
        .unwrap();
    assert!(matches!(reader.poll().unwrap(), TranscriptPoll::Waiting));
    let idle = reader.park_automatic();
    assert!(!idle.has_channel_reservation());
    assert_eq!(spool.pending().unwrap().len(), 1);
    drop(
        registry
            .acquire_admitted(qualified(root.path(), &path))
            .unwrap()
            .unwrap(),
    );
}
