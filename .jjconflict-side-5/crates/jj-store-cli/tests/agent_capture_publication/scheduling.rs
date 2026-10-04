use super::*;

#[tokio::test]
async fn scheduler_rotates_unknown_partition_and_retries_exact_prepared_head() {
    let directory = tempfile::tempdir().unwrap();
    let other = RepositoryId::new([10; 16]);
    let mut spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    enqueue(&mut spool, capture(2, 2, REPOSITORY));
    enqueue(&mut spool, capture(1, 1, REPOSITORY));
    enqueue(&mut spool, capture(3, 3, other));
    let mut schedule = CaptureDrainScheduler::default();
    let queue = spool.pending().unwrap();
    let first = schedule.select(&queue).unwrap().unwrap();
    assert_eq!(first.batch.batch_id, CaptureBatchId::new([1; 16]));
    let publisher = FakePublisher::new([Reply::Unknown, Reply::Committed]);
    assert!(
        drain_capture_spool_entry(&mut spool, first, &publisher)
            .await
            .unwrap()
            .outcome_unknown
    );
    let queue = spool.pending().unwrap();
    let prepared = queue
        .iter()
        .find(|e| e.batch.batch_id == first.batch.batch_id)
        .unwrap();
    let selected = schedule.select(&queue).unwrap().unwrap();
    assert_eq!(selected.repository, other);
    let independent = FakePublisher::new([Reply::Committed]);
    assert_eq!(
        drain_capture_spool_entry(&mut spool, selected, &independent)
            .await
            .unwrap()
            .committed,
        1
    );
    // Restart resets only fairness position, never prepared intent or partition order.
    drop(spool);
    let mut spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    let mut schedule = CaptureDrainScheduler::default();
    let queue = spool.pending().unwrap();
    let retry = schedule.select(&queue).unwrap().unwrap();
    assert_eq!(retry, prepared);
    assert_eq!(
        drain_capture_spool_entry(&mut spool, retry, &publisher)
            .await
            .unwrap()
            .committed,
        1
    );
    assert_eq!(publisher.prepared.lock().unwrap().len(), 1);
    let requests = publisher.published.lock().unwrap();
    assert_eq!(requests[0], requests[1]);
    let queue = spool.pending().unwrap();
    assert_eq!(
        schedule.select(&queue).unwrap().unwrap().batch.batch_id,
        CaptureBatchId::new([2; 16])
    );
}

#[test]
fn scheduler_withheld_turns_dynamic_partitions_and_invalid_input_preserve_order() {
    let directory = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    for id in 1..=3 {
        enqueue(
            &mut spool,
            capture(id, id as u64, RepositoryId::new([id; 16])),
        );
    }
    let mut schedule = CaptureDrainScheduler::default();
    let mut queue = spool.pending().unwrap();
    queue.reverse();
    assert_eq!(
        schedule.select(&queue).unwrap().unwrap().repository,
        RepositoryId::new([1; 16])
    );
    // Withhold/cancel without delivery: the next selection still gets its turn.
    assert!(schedule.select(&[]).unwrap().is_none());
    let mut duplicate = queue.clone();
    duplicate.push(queue[0].clone());
    assert!(schedule.select(&duplicate).is_err());
    assert!(schedule.select(&vec![queue[0].clone(); 4097]).is_err());
    let mut invalid = queue.clone();
    invalid[0].schema_version = 2;
    assert!(schedule.select(&invalid).is_err());
    assert_eq!(
        schedule.select(&queue).unwrap().unwrap().repository,
        RepositoryId::new([2; 16])
    );
    enqueue(&mut spool, capture(4, 4, RepositoryId::new([4; 16])));
    let queue = spool.pending().unwrap();
    for id in [3, 4, 1, 2] {
        assert_eq!(
            schedule.select(&queue).unwrap().unwrap().repository,
            RepositoryId::new([id; 16])
        );
    }
}

#[tokio::test]
async fn local_prepared_intent_excludes_its_complete_native_partition() {
    let directory = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    enqueue(&mut spool, capture(1, 1, REPOSITORY));
    let later = capture(2, 2, REPOSITORY);
    let local = LocalAgentRequest {
        version: 1,
        request_id: RequestId::new([12; 16]),
        command: LocalAgentCommand::Capture {
            profile: later.profile.clone(),
            batch: Box::new(later.batch.clone()),
        },
    };
    enqueue(&mut spool, later);
    spool
        .prepare_local(CaptureBatchId::new([2; 16]), local)
        .unwrap();
    let mut independent = capture(3, 1, REPOSITORY);
    independent.batch.source_instance_id = SourceInstanceId::new([3; 16]);
    independent.cursor.source_instance_id = independent.batch.source_instance_id;
    enqueue(&mut spool, independent);
    let before = spool.pending().unwrap();
    let head = before
        .iter()
        .find(|e| e.batch.batch_id == CaptureBatchId::new([1; 16]))
        .unwrap();
    let publisher = FakePublisher::new([]);
    assert!(
        drain_capture_spool_entry(&mut spool, head, &publisher)
            .await
            .is_err()
    );
    assert!(publisher.prepared.lock().unwrap().is_empty());
    assert_eq!(spool.pending().unwrap(), before);
    let mut scheduler = CaptureDrainScheduler::default();
    for _ in 0..3 {
        assert_eq!(
            scheduler.select(&before).unwrap().unwrap().batch.batch_id,
            CaptureBatchId::new([3; 16])
        );
    }
    assert_eq!(spool.pending().unwrap(), before);
}

#[test]
fn scheduler_heads_keep_generation_sequence_and_integration_partitions_distinct() {
    let directory = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(directory.path(), 4 * 1024 * 1024).unwrap();
    let mut later_generation = capture(1, 1, REPOSITORY);
    later_generation.cursor.generation = 2;
    enqueue(&mut spool, later_generation);
    enqueue(&mut spool, capture(2, 20, REPOSITORY));
    enqueue(&mut spool, capture(3, 10, REPOSITORY));
    let queue = spool.pending().unwrap();
    let mut scheduler = CaptureDrainScheduler::default();
    assert_eq!(
        scheduler.select(&queue).unwrap().unwrap().batch.batch_id,
        CaptureBatchId::new([3; 16])
    );
    let mut foreign = queue[0].clone();
    foreign.batch.batch_id = CaptureBatchId::new([4; 16]);
    foreign.batch.integration = IntegrationId::Cursor;
    foreign.cursor.integration = IntegrationId::Cursor;
    foreign.profile = NormalizerProfileV1::cursor_3_22_7();
    let mut queue = queue;
    queue.push(foreign);
    assert_eq!(
        scheduler.select(&queue).unwrap().unwrap().batch.integration,
        IntegrationId::Cursor
    );
    assert_eq!(
        scheduler.select(&queue).unwrap().unwrap().batch.batch_id,
        CaptureBatchId::new([3; 16])
    );
}
