use jj_agent::{CaptureBatchId, IntegrationId};
use jj_store_capture::{
    agent_capture::AgentCaptureSpool, agent_publication::drain_capture_spool_entry,
};
use jj_store_model::RepositoryId;
#[path = "reader_publication/mod.rs"]
mod reader_publication;
use reader_publication::*;
use std::fs;
#[path = "reader_publication/generations.rs"]
mod generations;
#[path = "reader_publication/inspection.rs"]
mod inspection;
#[path = "reader_publication/mixed.rs"]
mod mixed;
#[path = "reader_publication/paths.rs"]
mod paths;
#[path = "reader_publication/process.rs"]
mod process;

#[test]
fn later_repository_receipt_cannot_cover_an_earlier_withheld_range_after_restart() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool.enqueue_from_automatic_reader(input(1, REPO)).unwrap();
    spool
        .enqueue_from_automatic_reader(input(2, RepositoryId::new([9; 16])))
        .unwrap();
    let entry = spool
        .pending()
        .unwrap()
        .into_iter()
        .find(|e| e.batch.batch_id == CaptureBatchId::new([2; 16]))
        .unwrap();
    ready(drain_capture_spool_entry(&mut spool, &entry, &Publisher)).unwrap();
    let progress = spool
        .reader_publication_progress(IntegrationId::Codex, SOURCE, [1; 32])
        .unwrap()
        .unwrap();
    assert_eq!(progress.captured_byte_offset, 200);
    assert_eq!(progress.committed_byte_offset, 0);
    assert_eq!(progress.outstanding_ranges, 2);
    drop(spool);
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    assert_eq!(
        spool
            .reader_publication_progress(IntegrationId::Codex, SOURCE, [1; 32])
            .unwrap(),
        Some(progress)
    );
    let entry = spool.pending().unwrap().remove(0);
    ready(drain_capture_spool_entry(&mut spool, &entry, &Publisher)).unwrap();
    let progress = spool
        .reader_publication_progress(IntegrationId::Codex, SOURCE, [1; 32])
        .unwrap()
        .unwrap();
    assert!(progress.history_available);
    assert_eq!(progress.committed_byte_offset, 200);
    assert_eq!(progress.outstanding_ranges, 0);
    assert_eq!(progress.receipts, 2);
    spool.enqueue_from_automatic_reader(input(3, REPO)).unwrap();
    let progress = spool
        .reader_publication_progress(IntegrationId::Codex, SOURCE, [1; 32])
        .unwrap()
        .unwrap();
    assert_eq!(progress.captured_byte_offset, 300);
    assert_eq!(progress.committed_byte_offset, 200);
    assert_eq!(progress.outstanding_ranges, 1);
}

#[test]
fn identity_only_completion_and_empty_queue_never_prove_publication() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool.enqueue_from_automatic_reader(input(1, REPO)).unwrap();
    let entry = spool.pending().unwrap().remove(0);
    ready(drain_capture_spool_entry(
        &mut spool,
        &entry,
        &UnknownPublisher,
    ))
    .unwrap();
    let entry = spool.pending().unwrap().remove(0);
    let jj_store_capture::agent_capture::PendingCaptureStateV1::Prepared { request } = &entry.state
    else {
        panic!("prepared intent required")
    };
    spool
        .complete(
            entry.batch.batch_id,
            request.request_id,
            request.publication_id,
        )
        .unwrap();
    spool.enqueue_from_automatic_reader(input(2, REPO)).unwrap();
    let entry = spool.pending().unwrap().remove(0);
    ready(drain_capture_spool_entry(&mut spool, &entry, &Publisher)).unwrap();
    assert!(spool.pending().unwrap().is_empty());
    drop(spool);
    let spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    let progress = spool
        .reader_publication_progress(IntegrationId::Codex, SOURCE, [1; 32])
        .unwrap()
        .unwrap();
    assert_eq!(progress.committed_byte_offset, 0);
    assert_eq!(progress.outstanding_ranges, 2);
}

#[test]
fn unknown_and_mismatched_receipts_keep_the_exact_prepared_retry_and_frontier() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool.enqueue_from_automatic_reader(input(1, REPO)).unwrap();
    let entry = spool.pending().unwrap().remove(0);
    let report = ready(drain_capture_spool_entry(
        &mut spool,
        &entry,
        &UnknownPublisher,
    ))
    .unwrap();
    assert!(report.outcome_unknown);
    let prepared = spool.pending().unwrap().remove(0);
    let bytes = fs::read(root.path().join("reader-publication/catalog.json")).unwrap();
    assert!(
        ready(drain_capture_spool_entry(
            &mut spool,
            &prepared,
            &MismatchedPublisher
        ))
        .is_err()
    );
    assert_eq!(spool.pending().unwrap(), vec![prepared.clone()]);
    assert_eq!(
        fs::read(root.path().join("reader-publication/catalog.json")).unwrap(),
        bytes
    );
    drop(spool);
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    ready(drain_capture_spool_entry(&mut spool, &prepared, &Publisher)).unwrap();
    let progress = spool
        .reader_publication_progress(IntegrationId::Codex, SOURCE, [1; 32])
        .unwrap()
        .unwrap();
    assert_eq!(progress.committed_byte_offset, 100);
    assert_eq!(progress.receipts, 1);
}

#[test]
fn confirmed_evidence_precedes_body_removal_and_retry_does_not_count_it_twice() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool.enqueue_from_automatic_reader(input(1, REPO)).unwrap();
    let entry = spool.pending().unwrap().remove(0);
    ready(drain_capture_spool_entry(
        &mut spool,
        &entry,
        &UnknownPublisher,
    ))
    .unwrap();
    let prepared = spool.pending().unwrap().remove(0);
    let progress_path = root.path().join("reader-progress").join(format!(
        "{REPO}-{}-{SOURCE}-1.json",
        IntegrationId::Codex as u8
    ));
    fs::remove_file(&progress_path).unwrap();
    fs::create_dir(&progress_path).unwrap();
    assert!(ready(drain_capture_spool_entry(&mut spool, &prepared, &Publisher)).is_err());
    assert_eq!(spool.pending().unwrap(), vec![prepared.clone()]);
    let selected = fs::read(root.path().join("reader-publication/catalog.json")).unwrap();
    assert!(String::from_utf8_lossy(&selected).contains("\"receipts\":1"));
    fs::remove_dir(progress_path).unwrap();
    drop(spool);
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    ready(drain_capture_spool_entry(&mut spool, &prepared, &Publisher)).unwrap();
    assert_eq!(
        fs::read(root.path().join("reader-publication/catalog.json")).unwrap(),
        selected
    );
    assert!(spool.pending().unwrap().is_empty());
    assert_eq!(
        spool
            .reader_publication_progress(IntegrationId::Codex, SOURCE, [1; 32])
            .unwrap()
            .unwrap()
            .receipts,
        1
    );
}

#[test]
fn preexisting_or_lost_publication_history_is_not_promoted_by_a_later_receipt() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool.enqueue_from_reader(input(1, REPO)).unwrap();
    let entry = spool.pending().unwrap().remove(0);
    ready(drain_capture_spool_entry(&mut spool, &entry, &Publisher)).unwrap();
    assert!(
        spool
            .reader_publication_progress(IntegrationId::Codex, SOURCE, [1; 32])
            .unwrap()
            .is_none()
    );
    spool.enqueue_from_automatic_reader(input(2, REPO)).unwrap();
    let entry = spool.pending().unwrap().remove(0);
    ready(drain_capture_spool_entry(&mut spool, &entry, &Publisher)).unwrap();
    let progress = spool
        .reader_publication_progress(IntegrationId::Codex, SOURCE, [1; 32])
        .unwrap()
        .unwrap();
    assert!(!progress.history_available);
    assert_eq!(progress.committed_byte_offset, 0);
    fs::remove_dir_all(root.path().join("reader-publication")).unwrap();
    spool.enqueue_from_automatic_reader(input(3, REPO)).unwrap();
    let entry = spool.pending().unwrap().remove(0);
    ready(drain_capture_spool_entry(&mut spool, &entry, &Publisher)).unwrap();
    assert!(
        !spool
            .reader_publication_progress(IntegrationId::Codex, SOURCE, [1; 32])
            .unwrap()
            .unwrap()
            .history_available
    );
}

#[test]
fn corrupt_or_missing_selected_catalog_withholds_enqueue_and_is_never_repaired() {
    for missing in [false, true] {
        let root = tempfile::tempdir().unwrap();
        let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
        spool.enqueue_from_automatic_reader(input(1, REPO)).unwrap();
        let pending = spool.pending().unwrap();
        let path = root.path().join("reader-publication/catalog.json");
        if missing {
            fs::remove_file(&path).unwrap()
        } else {
            fs::write(&path, b"corrupt").unwrap()
        }
        assert!(spool.enqueue_from_automatic_reader(input(2, REPO)).is_err());
        assert_eq!(spool.pending().unwrap(), pending);
        assert!(
            spool
                .reader_publication_progress(IntegrationId::Codex, SOURCE, [1; 32])
                .is_err()
        );
        drop(spool);
        assert!(AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).is_err());
        if missing {
            assert!(!path.exists())
        } else {
            assert_eq!(fs::read(path).unwrap(), b"corrupt")
        }
    }
}

#[test]
fn selected_catalog_contains_no_body_path_or_native_session_key() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    let mut candidate = input(1, REPO);
    candidate.cursor.native_session_key = Some("PRIVATE_NATIVE_SESSION_SENTINEL".into());
    spool.enqueue_from_automatic_reader(candidate).unwrap();
    let entry = spool.pending().unwrap().remove(0);
    ready(drain_capture_spool_entry(&mut spool, &entry, &Publisher)).unwrap();
    let text = fs::read_to_string(root.path().join("reader-publication/catalog.json")).unwrap();
    for denied in [
        "PRIVATE_NATIVE_SESSION_SENTINEL",
        "omitted",
        root.path().to_str().unwrap(),
        "native_session_key",
        "payload",
        "parser_version",
    ] {
        assert!(!text.contains(denied));
    }
    assert!(text.contains("receipt_digest"));
    assert!(text.contains("chain"));
}
