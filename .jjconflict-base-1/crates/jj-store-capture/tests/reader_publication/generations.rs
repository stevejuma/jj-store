use super::*;

fn continued(sequence: u64, generation: u64) -> jj_store_capture::reader_progress::ReaderCapture {
    let mut capture = input(sequence, REPO);
    capture.cursor.generation = generation;
    capture
}
fn publish(spool: &mut AgentCaptureSpool, sequence: u8) {
    let entry = spool
        .pending()
        .unwrap()
        .into_iter()
        .find(|e| e.batch.batch_id == CaptureBatchId::new([sequence; 16]))
        .unwrap();
    ready(drain_capture_spool_entry(spool, &entry, &Publisher)).unwrap();
}
#[test]
fn later_numeric_generation_cannot_hide_an_earlier_physical_history_hole() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool.enqueue_from_automatic_reader(input(1, REPO)).unwrap();
    let mut other = continued(2, 2);
    other.metadata.generation_digest = [2; 32];
    spool.enqueue_from_automatic_reader(other).unwrap();
    let mut resumed = continued(3, 3);
    resumed.repository = RepositoryId::new([9; 16]);
    spool.enqueue_from_automatic_reader(resumed).unwrap();
    publish(&mut spool, 3);
    let progress = spool
        .reader_publication_progress(IntegrationId::Codex, SOURCE, [1; 32])
        .unwrap()
        .unwrap();
    assert_eq!(progress.generation, 3);
    assert_eq!(progress.captured_byte_offset, 300);
    assert_eq!(progress.committed_byte_offset, 0);
    assert_eq!(progress.outstanding_ranges, 1);
    drop(spool);
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    assert_eq!(
        spool
            .reader_publication_progress(IntegrationId::Codex, SOURCE, [1; 32])
            .unwrap(),
        Some(progress)
    );
    publish(&mut spool, 1);
    let progress = spool
        .reader_publication_progress(IntegrationId::Codex, SOURCE, [1; 32])
        .unwrap()
        .unwrap();
    assert_eq!(progress.committed_byte_offset, 300);
    assert_eq!(progress.receipts, 2);
    assert!(progress.history_available);
    // The unrelated physical file's withheld range is not part of this history.
    assert_eq!(spool.pending().unwrap().len(), 1);
}
#[test]
fn unknown_old_generation_is_not_promoted_by_a_new_generation_receipt() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool.enqueue_from_reader(input(1, REPO)).unwrap();
    publish(&mut spool, 1);
    spool
        .enqueue_from_automatic_reader(continued(2, 2))
        .unwrap();
    publish(&mut spool, 2);
    let progress = spool
        .reader_publication_progress(IntegrationId::Codex, SOURCE, [1; 32])
        .unwrap()
        .unwrap();
    assert!(!progress.history_available);
    assert_eq!(progress.committed_byte_offset, 0);
}
#[test]
fn a_pending_continuation_preserves_the_prior_committed_frontier() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool.enqueue_from_automatic_reader(input(1, REPO)).unwrap();
    publish(&mut spool, 1);
    spool
        .enqueue_from_automatic_reader(continued(2, 2))
        .unwrap();
    let progress = spool
        .reader_publication_progress(IntegrationId::Codex, SOURCE, [1; 32])
        .unwrap()
        .unwrap();
    assert_eq!(progress.committed_byte_offset, 100);
    assert_eq!(progress.outstanding_ranges, 1);
    publish(&mut spool, 2);
    let progress = spool
        .reader_publication_progress(IntegrationId::Codex, SOURCE, [1; 32])
        .unwrap()
        .unwrap();
    assert_eq!(progress.committed_byte_offset, 200);
    assert_eq!(progress.receipts, 2);
}

#[test]
fn loss_of_an_old_numeric_channel_keeps_retained_physical_history_unqualified() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool.enqueue_from_automatic_reader(input(1, REPO)).unwrap();
    publish(&mut spool, 1);
    spool
        .enqueue_from_automatic_reader(continued(2, 2))
        .unwrap();
    publish(&mut spool, 2);
    let path = root.path().join("reader-publication/catalog.json");
    let mut value: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    value["channels"].as_array_mut().unwrap().remove(0);
    fs::write(&path, serde_json::to_vec(&value).unwrap()).unwrap();
    drop(spool);
    let spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    let progress = spool
        .reader_publication_progress(IntegrationId::Codex, SOURCE, [1; 32])
        .unwrap()
        .unwrap();
    assert!(!progress.history_available);
    assert_eq!(progress.committed_byte_offset, 0);
}
