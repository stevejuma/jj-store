use super::*;

#[test]
fn physical_selection_and_channel_floors_span_partitions_pending_and_restart() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool.enqueue_from_reader(input(9, 1)).unwrap();
    let mut other = input(2, 3);
    other.repository = RepositoryId::new([7; 16]);
    spool.enqueue_from_reader(other).unwrap();
    let selected = spool
        .source_reader_progress(IntegrationId::Codex, SOURCE, [1; 32])
        .unwrap();
    assert_eq!(selected.progress.as_ref().unwrap().cursor.generation, 1);
    assert_eq!(selected.progress.as_ref().unwrap().last_sequence, 9);
    assert_eq!(selected.generation_floor, 3);
    assert_eq!(selected.sequence_floor, 9);
    let unseen = spool
        .source_reader_progress(IntegrationId::Codex, SOURCE, [88; 32])
        .unwrap();
    assert!(unseen.progress.is_none());
    assert_eq!(unseen.generation_floor, 3);
    assert_eq!(unseen.sequence_floor, 9);
    // A pending entry is sufficient even while its atomic journal needs repair.
    fs::remove_file(progress_path(root.path(), 1)).unwrap();
    assert_eq!(
        spool
            .source_reader_progress(IntegrationId::Codex, SOURCE, [1; 32])
            .unwrap(),
        selected
    );
    drop(spool);
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    let request = native_request();
    for sequence in [9, 2] {
        let batch = CaptureBatchId::new([sequence; 16]);
        spool.prepare(batch, request.clone()).unwrap();
        spool
            .complete(batch, request.request_id, request.publication_id)
            .unwrap();
    }
    assert!(spool.pending().unwrap().is_empty());
    drop(spool);
    let spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    assert_eq!(
        spool
            .source_reader_progress(IntegrationId::Codex, SOURCE, [1; 32])
            .unwrap(),
        selected
    );
}

#[test]
fn automatic_admission_refuses_stale_allocations_but_retains_exact_durable_retries() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 8 * 1024 * 1024).unwrap();
    let first = input(1, 1);
    spool.enqueue_from_automatic_reader(first).unwrap();
    let second = input(4, 2);
    spool.enqueue_from_automatic_reader(second).unwrap();
    let retained = spool.pending().unwrap();
    for stale in [input(3, 3), input(5, 1)] {
        assert!(spool.enqueue_from_automatic_reader(stale).is_err());
        assert_eq!(spool.pending().unwrap(), retained);
    }
    let mut collision = input(5, 2);
    collision.metadata.generation_digest = [99; 32];
    assert!(spool.enqueue_from_automatic_reader(collision).is_err());
    let mut independent = spool.fork_handle();
    independent
        .enqueue_from_automatic_reader(input(6, 3))
        .unwrap();
    let retained = spool.pending().unwrap();
    assert!(spool.enqueue_from_automatic_reader(input(5, 3)).is_err());
    assert_eq!(spool.pending().unwrap(), retained);
    // Receipt retries do not reinterpret accepted earlier generation/sequence bounds.
    spool.enqueue_from_automatic_reader(input(1, 1)).unwrap();
    assert_eq!(spool.pending().unwrap(), retained);
    spool.enqueue_from_automatic_reader(input(7, 3)).unwrap();
    // Unlinked sanitized pending intent still occupies this channel's allocation range.
    let generic = input(20, 5);
    spool
        .enqueue(
            generic.repository,
            generic.profile,
            generic.cursor,
            generic.batch,
        )
        .unwrap();
    let selected = spool
        .source_reader_progress(IntegrationId::Codex, SOURCE, [3; 32])
        .unwrap();
    assert_eq!(selected.progress.unwrap().last_sequence, 7);
    assert_eq!(selected.sequence_floor, 20);
    assert_eq!(selected.generation_floor, 5);
    let retained = spool.pending().unwrap();
    assert!(spool.enqueue_from_automatic_reader(input(19, 6)).is_err());
    assert_eq!(spool.pending().unwrap(), retained);
}
