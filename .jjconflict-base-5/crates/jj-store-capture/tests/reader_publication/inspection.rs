use super::*;

#[test]
fn accepted_boundary_survives_unknown_later_history() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool.enqueue_from_automatic_reader(input(1, REPO)).unwrap();
    let entry = spool.pending().unwrap().remove(0);
    ready(drain_capture_spool_entry(&mut spool, &entry, &Publisher)).unwrap();
    let mut tail = input(2, REPO);
    tail.cursor.generation = 2;
    spool.enqueue_from_reader(tail).unwrap();
    // A selected boundary already has its receipts, even when later history is unknown.
    let progress = spool
        .inspect_reader_publication(IntegrationId::Codex, SOURCE, [1; 32], 100)
        .unwrap()
        .unwrap();
    assert!(progress.completed);
    assert_eq!(progress.committed_byte_offset, 100);
    assert!(progress.tail.is_none() || !progress.tail.unwrap().history_available);
}

#[test]
fn progress_inspection_does_not_open_pending_bodies() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool.enqueue_from_automatic_reader(input(1, REPO)).unwrap();
    let body = fs::read_dir(root.path().join("pending"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    fs::write(&body, b"BODY_MUST_NOT_BE_READ").unwrap();
    let progress = spool.inspect_reader_publication(IntegrationId::Codex, SOURCE, [1; 32], 100);
    assert!(
        progress.is_ok(),
        "metadata inspection opened a pending body: {progress:?}"
    );
    assert_eq!(fs::read(&body).unwrap(), b"BODY_MUST_NOT_BE_READ");
}

#[test]
fn inspection_keeps_earlier_holes_and_unknown_history_unqualified() {
    for automatic in [false, true] {
        let root = tempfile::tempdir().unwrap();
        let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
        if automatic {
            spool.enqueue_from_automatic_reader(input(1, REPO)).unwrap();
        } else {
            spool.enqueue_from_reader(input(1, REPO)).unwrap();
        }
        let mut tail = input(2, RepositoryId::new([9; 16]));
        tail.cursor.generation = 2;
        spool.enqueue_from_automatic_reader(tail).unwrap();
        let entry = spool
            .pending()
            .unwrap()
            .into_iter()
            .find(|e| e.batch.batch_id == CaptureBatchId::new([2; 16]))
            .unwrap();
        ready(drain_capture_spool_entry(&mut spool, &entry, &Publisher)).unwrap();
        let result = spool
            .inspect_reader_publication(IntegrationId::Codex, SOURCE, [1; 32], 200)
            .unwrap()
            .unwrap();
        assert!(!result.completed);
        assert_eq!(result.committed_byte_offset, 0);
        assert_eq!(result.boundary_available, automatic);
    }
}

#[test]
fn earlier_qualified_generation_proves_boundary_and_missing_tail_stays_separate() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool.enqueue_from_automatic_reader(input(1, REPO)).unwrap();
    let entry = spool.pending().unwrap().remove(0);
    ready(drain_capture_spool_entry(&mut spool, &entry, &Publisher)).unwrap();
    let mut tail = input(2, REPO);
    tail.cursor.generation = 7;
    spool.enqueue_from_automatic_reader(tail).unwrap();
    fs::remove_file(root.path().join("reader-progress").join(format!(
        "{REPO}-{}-{SOURCE}-7.json",
        IntegrationId::Codex as u8
    )))
    .unwrap();
    let result = spool
        .inspect_reader_publication(IntegrationId::Codex, SOURCE, [1; 32], 100)
        .unwrap()
        .unwrap();
    assert!(result.completed);
    assert!(result.tail.is_none());
    let result = spool
        .inspect_reader_publication(IntegrationId::Codex, SOURCE, [1; 32], 200)
        .unwrap()
        .unwrap();
    assert!(!result.completed && !result.boundary_available);
}

#[test]
fn inspection_neither_initializes_locks_nor_recovers_temporary_catalog_files() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool.enqueue_from_automatic_reader(input(1, REPO)).unwrap();
    let temporary = root.path().join("reader-publication/.temporary");
    fs::write(&temporary, b"unfinished").unwrap();
    assert!(
        spool
            .inspect_reader_publication(IntegrationId::Codex, SOURCE, [1; 32], 100)
            .is_err()
    );
    assert_eq!(fs::read(&temporary).unwrap(), b"unfinished");
    let lock = root.path().join("mutation.lock");
    fs::remove_file(&lock).unwrap();
    assert!(
        spool
            .inspect_reader_publication(IntegrationId::Codex, SOURCE, [1; 32], 100)
            .is_err()
    );
    assert!(!lock.exists());
}
