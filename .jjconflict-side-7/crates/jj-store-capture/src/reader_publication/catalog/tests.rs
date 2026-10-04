use super::*;
use jj_agent::CaptureBatchId;
use jj_store_model::{PrincipalId, RepositoryId};
fn key() -> Key {
    Key {
        integration: IntegrationId::Codex,
        source: SourceInstanceId::new([2; 16]),
        generation: 1,
        digest: [1; 32],
    }
}
fn range(sequence: u64) -> Range {
    let mut id = [0; 16];
    id[..8].copy_from_slice(&sequence.to_be_bytes());
    Range {
        repository: RepositoryId::new([1; 16]),
        batch: CaptureBatchId::new(id),
        first: sequence,
        last: sequence,
        offset: sequence * 100,
        prefix: [1; 32],
        snapshot: None,
    }
}
#[test]
fn capacity_keeps_every_hole_and_folding_one_exact_receipt_releases_one_slot() {
    let mut catalog = Catalog::default();
    catalog.add(Channel::new(key(), range(1), true)).unwrap();
    for sequence in 1..=MAX_RANGES as u64 {
        catalog.register(key(), range(sequence)).unwrap()
    }
    let before = serde_json::to_vec(&catalog).unwrap();
    assert!(
        catalog
            .register(key(), range(MAX_RANGES as u64 + 1))
            .is_err()
    );
    assert_eq!(serde_json::to_vec(&catalog).unwrap(), before);
    let first = range(1);
    catalog
        .confirm(
            key(),
            &first,
            Evidence {
                principal: PrincipalId::new([3; 16]),
                request: crate::agent_publication::request_id(first.repository, first.batch),
                publication: crate::agent_publication::publication_id(
                    first.repository,
                    first.batch,
                ),
                receipt_digest: [9; 32],
            },
        )
        .unwrap();
    catalog
        .register(key(), range(MAX_RANGES as u64 + 1))
        .unwrap();
    let channel = &catalog.channels[0];
    assert_eq!(channel.pending.len(), MAX_RANGES);
    assert_eq!(channel.frontier.as_ref().unwrap().range, first);
    assert_eq!(channel.receipts, 1);
    assert_eq!(channel.latest, range(MAX_RANGES as u64 + 1));
}
#[test]
fn changed_digest_range_and_receipt_reuse_cannot_reauthor_selected_evidence() {
    let mut catalog = Catalog::default();
    catalog.add(Channel::new(key(), range(1), true)).unwrap();
    catalog.register(key(), range(1)).unwrap();
    let before = serde_json::to_vec(&catalog).unwrap();
    let mut changed = range(1);
    changed.prefix = [7; 32];
    assert!(catalog.register(key(), changed).is_err());
    assert_eq!(serde_json::to_vec(&catalog).unwrap(), before);
    let first = range(1);
    let mut evidence = Evidence {
        principal: PrincipalId::new([3; 16]),
        request: crate::agent_publication::request_id(first.repository, first.batch),
        publication: crate::agent_publication::publication_id(first.repository, first.batch),
        receipt_digest: [9; 32],
    };
    catalog.confirm(key(), &first, evidence.clone()).unwrap();
    let before = serde_json::to_vec(&catalog).unwrap();
    evidence.receipt_digest = [10; 32];
    assert!(catalog.confirm(key(), &first, evidence).is_err());
    assert_eq!(serde_json::to_vec(&catalog).unwrap(), before);
    let mut foreign = key();
    foreign.digest = [7; 32];
    assert!(catalog.register(foreign, range(2)).is_err());
}
#[test]
fn snapshot_capacity_keeps_status_holes_and_original_jsonl_encoding() {
    let original = range(1);
    let json = serde_json::to_value(&original).unwrap();
    assert!(json.get("snapshot").is_none());
    assert_eq!(serde_json::from_value::<Range>(json).unwrap(), original);
    let snapshot = |sequence| {
        let mut r = range(sequence);
        r.offset = 0;
        r.prefix = [0; 32];
        r.snapshot = Some(ranges::SnapshotPosition {
            digest: [1; 32],
            logical_records: 0,
        });
        r
    };
    let mut catalog = Catalog::default();
    catalog.add(Channel::new(key(), snapshot(1), true)).unwrap();
    for sequence in 1..=MAX_RANGES as u64 {
        catalog.register(key(), snapshot(sequence)).unwrap();
    }
    let before = serde_json::to_vec(&catalog).unwrap();
    assert!(
        catalog
            .register(key(), snapshot(MAX_RANGES as u64 + 1))
            .is_err()
    );
    assert_eq!(serde_json::to_vec(&catalog).unwrap(), before);
    let first = snapshot(1);
    catalog
        .confirm(
            key(),
            &first,
            Evidence {
                principal: PrincipalId::new([3; 16]),
                request: crate::agent_publication::request_id(first.repository, first.batch),
                publication: crate::agent_publication::publication_id(
                    first.repository,
                    first.batch,
                ),
                receipt_digest: [9; 32],
            },
        )
        .unwrap();
    catalog
        .register(key(), snapshot(MAX_RANGES as u64 + 1))
        .unwrap();
    assert_eq!(catalog.channels[0].pending.len(), MAX_RANGES);
    let mut invalid = first;
    invalid.offset = 1;
    assert!(invalid.validate().is_err());
}
