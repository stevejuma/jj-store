use jj_agent::{
    CaptureBatchId, IntegrationId, NormalizerProfileV1, SourceKind, SourcePositionV1, SourceTrust,
};
use jj_store_capture::{
    agent_capture::AgentCaptureSpool,
    agent_publication::drain_capture_spool_entry,
    opencode_checkpoint::{OpenCodeLifecycleV1, OpenCodeSnapshotStateV1},
    reader_progress::ReaderCapture,
};
use jj_store_model::RepositoryId;
use std::fs;
#[path = "reader_publication/mod.rs"]
mod fixture;
use fixture::*;
fn snapshot(sequence: u64, generation: u64, count: u64) -> ReaderCapture {
    let mut input = input(sequence, REPO);
    input.profile = NormalizerProfileV1::opencode_1_18_30();
    input.cursor.integration = IntegrationId::OpenCode;
    input.cursor.generation = generation;
    input.cursor.parser_version = "opencode-snapshot@1.18.30/v1".into();
    input.cursor.native_session_key = Some("ses_test".into());
    input.cursor.position = SourcePositionV1::Snapshot {
        generation_digest: [sequence as u8; 32],
        logical_records: count,
    };
    input.batch.integration = IntegrationId::OpenCode;
    input.batch.observations[0].source_kind = SourceKind::NativeApi;
    input.batch.observations[0].source_trust = SourceTrust::ApprovedHarnessApi;
    if count == 0 {
        let state = OpenCodeSnapshotStateV1::initial(
            [1; 32],
            if sequence % 2 == 1 {
                OpenCodeLifecycleV1::Busy
            } else {
                OpenCodeLifecycleV1::Idle
            },
        );
        input.cursor.position = SourcePositionV1::Snapshot {
            generation_digest: state.digest([1; 32]),
            logical_records: 0,
        };
        input.metadata.opencode_snapshot = Some(Box::new(state));
    }
    input
}
fn inspect(
    spool: &AgentCaptureSpool,
    boundary: u64,
) -> jj_store_capture::reader_publication::SnapshotPublicationInspection {
    spool
        .inspect_snapshot_publication(IntegrationId::OpenCode, SOURCE, [1; 32], boundary)
        .unwrap()
        .unwrap()
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
fn empty_status_acknowledgement_and_identity_completion_do_not_prove_publication() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    let ack = spool
        .enqueue_from_automatic_reader(snapshot(1, 1, 0))
        .unwrap();
    assert_eq!(inspect(&spool, 1).committed_sequence, 0);
    let entry = spool.pending().unwrap().remove(0);
    ready(drain_capture_spool_entry(
        &mut spool,
        &entry,
        &UnknownPublisher,
    ))
    .unwrap();
    let prepared = spool.pending().unwrap().remove(0);
    let jj_store_capture::agent_capture::PendingCaptureStateV1::Prepared { request } =
        &prepared.state
    else {
        panic!("exact prepared retry required");
    };
    spool
        .complete(ack.batch_id, request.request_id, request.publication_id)
        .unwrap();
    assert!(spool.pending().unwrap().is_empty());
    drop(spool);
    let spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    let p = inspect(&spool, 1);
    assert!(p.boundary_available && !p.completed);
    assert_eq!(p.tail.unwrap().committed_logical_records, None);
}
#[test]
fn unknown_or_mismatched_snapshot_receipts_keep_exact_retry_and_zero_frontier() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool
        .enqueue_from_automatic_reader(snapshot(1, 1, 1))
        .unwrap();
    let entry = spool.pending().unwrap().remove(0);
    ready(drain_capture_spool_entry(
        &mut spool,
        &entry,
        &UnknownPublisher,
    ))
    .unwrap();
    let prepared = spool.pending().unwrap().remove(0);
    assert!(
        ready(drain_capture_spool_entry(
            &mut spool,
            &prepared,
            &MismatchedPublisher
        ))
        .is_err()
    );
    assert_eq!(spool.pending().unwrap()[0], prepared);
    assert!(!inspect(&spool, 1).completed);
    drop(spool);
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    publish(&mut spool, 1);
    assert!(inspect(&spool, 1).completed);
}
#[test]
fn generation_replacement_can_reduce_count_but_cannot_skip_earlier_holes() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool
        .enqueue_from_automatic_reader(snapshot(1, 1, 5))
        .unwrap();
    let mut replacement = snapshot(2, 2, 1);
    replacement.repository = RepositoryId::new([9; 16]);
    spool.enqueue_from_automatic_reader(replacement).unwrap();
    publish(&mut spool, 2);
    assert_eq!(inspect(&spool, 2).committed_sequence, 0);
    assert_eq!(inspect(&spool, 2).tail.unwrap().outstanding_ranges, 1);
    drop(spool);
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    publish(&mut spool, 1);
    let p = inspect(&spool, 2);
    assert!(p.completed);
    let tail = p.tail.unwrap();
    assert_eq!(tail.committed_logical_records, Some(1));
    assert_eq!(tail.receipts, 2);
}
#[test]
fn old_acknowledged_history_cannot_be_promoted_by_a_later_snapshot_receipt() {
    for remove in [false, true] {
        let root = tempfile::tempdir().unwrap();
        let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
        if remove {
            spool
                .enqueue_from_automatic_reader(snapshot(1, 1, 1))
                .unwrap();
        } else {
            spool.enqueue_from_reader(snapshot(1, 1, 1)).unwrap();
        }
        publish(&mut spool, 1);
        spool
            .enqueue_from_automatic_reader(snapshot(2, 2, 2))
            .unwrap();
        publish(&mut spool, 2);
        if remove {
            let path = root.path().join("reader-publication/catalog.json");
            let mut value: serde_json::Value =
                serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
            value["channels"].as_array_mut().unwrap().remove(0);
            fs::write(&path, serde_json::to_vec(&value).unwrap()).unwrap();
        }
        drop(spool);
        let spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
        let p = inspect(&spool, 2);
        assert!(!p.completed && !p.boundary_available);
        assert_eq!(p.committed_sequence, 0);
        assert_eq!(p.tail.unwrap().committed_logical_records, None);
    }
}
#[test]
fn selected_snapshot_boundary_survives_unavailable_later_reader_history() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool
        .enqueue_from_automatic_reader(snapshot(1, 1, 0))
        .unwrap();
    publish(&mut spool, 1);
    spool
        .enqueue_from_automatic_reader(snapshot(2, 2, 0))
        .unwrap();
    fs::remove_file(root.path().join("reader-progress").join(format!(
        "{REPO}-{}-{SOURCE}-2.json",
        IntegrationId::OpenCode as u8
    )))
    .unwrap();
    let p = inspect(&spool, 1);
    assert!(p.completed);
    assert!(!p.tail.unwrap().history_available);
    assert!(!inspect(&spool, 2).boundary_available);
}
#[test]
fn snapshot_inspection_does_not_read_pending_bodies_or_repair_selected_files() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool
        .enqueue_from_automatic_reader(snapshot(1, 1, 1))
        .unwrap();
    let body = fs::read_dir(root.path().join("pending"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    fs::write(&body, b"BODY_MUST_NOT_BE_READ").unwrap();
    assert!(inspect(&spool, 1).boundary_available);
    assert_eq!(fs::read(&body).unwrap(), b"BODY_MUST_NOT_BE_READ");
    let temporary = root.path().join("reader-publication/.temporary");
    fs::write(&temporary, b"unfinished").unwrap();
    assert!(
        spool
            .inspect_snapshot_publication(IntegrationId::OpenCode, SOURCE, [1; 32], 1)
            .is_err()
    );
    assert_eq!(fs::read(&temporary).unwrap(), b"unfinished");
    fs::remove_file(&temporary).unwrap();
    fs::remove_file(root.path().join("mutation.lock")).unwrap();
    assert!(
        spool
            .inspect_snapshot_publication(IntegrationId::OpenCode, SOURCE, [1; 32], 1)
            .is_err()
    );
    assert!(!root.path().join("mutation.lock").exists());
}
#[test]
fn byte_and_snapshot_inspections_refuse_each_others_positions() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool.enqueue_from_automatic_reader(input(1, REPO)).unwrap();
    assert!(
        spool
            .inspect_snapshot_publication(IntegrationId::Codex, SOURCE, [1; 32], 1)
            .is_err()
    );
    spool
        .enqueue_from_automatic_reader(snapshot(2, 1, 1))
        .unwrap();
    assert!(
        spool
            .inspect_reader_publication(IntegrationId::OpenCode, SOURCE, [1; 32], 100)
            .is_err()
    );
}
#[test]
fn ingress_pressure_does_not_select_snapshot_publication_evidence() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 128).unwrap();
    assert!(
        spool
            .enqueue_from_automatic_reader(snapshot(1, 1, 0))
            .is_err()
    );
    assert!(!root.path().join("reader-publication").exists());
    drop(spool);
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool
        .enqueue_from_automatic_reader(snapshot(1, 1, 0))
        .unwrap();
    publish(&mut spool, 1);
    assert_eq!(
        inspect(&spool, 1).tail.unwrap().committed_logical_records,
        Some(0)
    );
}
#[test]
fn missing_corrupt_or_foreign_snapshot_catalog_never_repairs_into_publication() {
    for damage in [0, 1, 2] {
        let root = tempfile::tempdir().unwrap();
        let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
        spool
            .enqueue_from_automatic_reader(snapshot(1, 1, 1))
            .unwrap();
        publish(&mut spool, 1);
        let path = root.path().join("reader-publication/catalog.json");
        match damage {
            0 => fs::remove_dir_all(path.parent().unwrap()).unwrap(),
            1 => fs::write(&path, b"corrupt").unwrap(),
            _ => fs::write(path.parent().unwrap().join("foreign.json"), b"{}").unwrap(),
        }
        let before = fs::read(&path).ok();
        let result =
            spool.inspect_snapshot_publication(IntegrationId::OpenCode, SOURCE, [1; 32], 1);
        if damage == 0 {
            assert_eq!(result.unwrap(), None);
        } else {
            assert!(result.is_err());
        }
        assert_eq!(fs::read(&path).ok(), before);
    }
}
#[test]
fn page_inspection_is_bounded_and_keeps_independent_snapshot_witnesses() {
    use jj_store_capture::reader_publication::SnapshotPublicationQuery;
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool
        .enqueue_from_automatic_reader(snapshot(1, 1, 0))
        .unwrap();
    publish(&mut spool, 1);
    let mut replacement = snapshot(2, 2, 1);
    replacement.metadata.generation_digest = [2; 32];
    spool.enqueue_from_automatic_reader(replacement).unwrap();
    let selected = SnapshotPublicationQuery {
        integration: IntegrationId::OpenCode,
        source: SOURCE,
        generation_digest: [1; 32],
    };
    let mut other = selected;
    other.generation_digest = [2; 32];
    let page = spool
        .inspect_snapshot_publication_progress_batch(&[selected, other, selected])
        .unwrap();
    assert_eq!(page[0], page[2]);
    assert_eq!(page[0].as_ref().unwrap().committed_logical_records, Some(0));
    assert_eq!(page[1].as_ref().unwrap().committed_sequence, 0);
    assert_eq!(page[1].as_ref().unwrap().outstanding_ranges, 1);
    assert!(
        spool
            .inspect_snapshot_publication_progress_batch(&[selected; 101])
            .is_err()
    );
    other.generation_digest = [0; 32];
    assert!(
        spool
            .inspect_snapshot_publication_progress_batch(&[other])
            .is_err()
    );
    fs::remove_file(root.path().join("mutation.lock")).unwrap();
    assert!(
        spool
            .inspect_snapshot_publication_progress_batch(&[])
            .unwrap()
            .is_empty()
    );
    assert!(!root.path().join("mutation.lock").exists());
}
