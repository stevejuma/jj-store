use super::*;
#[test]
fn shared_reader_adds_requirements_only_to_an_already_tracked_channel() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool.enqueue_from_automatic_reader(input(1, REPO)).unwrap();
    spool
        .enqueue_from_reader(input(2, RepositoryId::new([9; 16])))
        .unwrap();
    spool.enqueue_from_reader(input(3, REPO)).unwrap();
    let queue = spool.pending().unwrap();
    let middle = queue
        .iter()
        .find(|e| e.batch.batch_id == CaptureBatchId::new([2; 16]))
        .unwrap();
    ready(drain_capture_spool_entry(&mut spool, middle, &Publisher)).unwrap();
    let progress = spool
        .reader_publication_progress(IntegrationId::Codex, SOURCE, [1; 32])
        .unwrap()
        .unwrap();
    assert_eq!(progress.captured_byte_offset, 300);
    assert_eq!(progress.committed_byte_offset, 0);
    for sequence in [1, 3] {
        let entry = spool
            .pending()
            .unwrap()
            .into_iter()
            .find(|e| e.batch.batch_id == CaptureBatchId::new([sequence; 16]))
            .unwrap();
        ready(drain_capture_spool_entry(&mut spool, &entry, &Publisher)).unwrap();
    }
    let progress = spool
        .reader_publication_progress(IntegrationId::Codex, SOURCE, [1; 32])
        .unwrap()
        .unwrap();
    assert_eq!(progress.committed_byte_offset, 300);
    assert_eq!(progress.receipts, 3);
    let mut other = input(4, REPO);
    other.cursor.source_instance_id = jj_agent::SourceInstanceId::new([4; 16]);
    other.batch.source_instance_id = other.cursor.source_instance_id;
    spool.enqueue_from_reader(other).unwrap();
    assert!(
        spool
            .reader_publication_progress(
                IntegrationId::Codex,
                jj_agent::SourceInstanceId::new([4; 16]),
                [1; 32]
            )
            .unwrap()
            .is_none()
    );
}
