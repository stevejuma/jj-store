use super::*;
use jj_store_server::capture_control_state::{CaptureControlReceipt, MAX_BYTES, MAX_RECEIPTS};
use std::fs;

#[test]
fn full_receipt_catalog_preserves_replay_and_refuses_new_requests_without_eviction() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("controls");
    let journal = CaptureControlJournal::create(&path, INSTALLATION).unwrap();
    let initial = journal.current().unwrap();
    let mut catalog: serde_json::Value =
        serde_json::from_slice(&fs::read(path.join("state.json")).unwrap()).unwrap();
    let receipts: Vec<_> = (0..MAX_RECEIPTS)
        .map(|i| CaptureControlReceipt {
            principal: PrincipalId::new([9; 16]),
            request: RequestId::new((i as u128 + 1).to_be_bytes()),
            expected_revision: i as u64,
            revision: i as u64 + 1,
            paused: i % 2 == 0,
        })
        .collect();
    catalog["revision"] = serde_json::json!(MAX_RECEIPTS);
    catalog["paused"] = serde_json::json!(receipts.last().unwrap().paused);
    catalog["receipts"] = serde_json::to_value(&receipts).unwrap();
    let bytes = serde_json::to_vec(&catalog).unwrap();
    assert!(bytes.len() <= MAX_BYTES);
    fs::write(path.join("state.json"), &bytes).unwrap();
    let reopened = CaptureControlJournal::open(&path, INSTALLATION).unwrap();
    let full = reopened.current().unwrap();
    let first = &receipts[0];
    let replay = reopened
        .record(&CaptureControlChange {
            generation: initial.generation,
            principal: first.principal,
            request: first.request,
            expected_revision: first.expected_revision,
            paused: first.paused,
        })
        .unwrap();
    assert!(replay.replayed);
    assert_eq!(replay.receipt, *first);
    assert_eq!(replay.current, full);
    assert!(!replay.current.paused && replay.receipt.paused);
    assert_eq!(
        reopened.record(&change(initial.generation, 8, full.revision, true)),
        Err(Error::Capacity)
    );
    assert_eq!(fs::read(path.join("state.json")).unwrap(), bytes);
}

#[test]
fn malformed_receipt_chains_generations_and_unknown_fields_fail_closed() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("controls");
    let journal = CaptureControlJournal::create(&path, INSTALLATION).unwrap();
    let initial = journal.current().unwrap();
    journal
        .record(&change(initial.generation, 1, 0, true))
        .unwrap();
    journal
        .record(&change(initial.generation, 2, 1, false))
        .unwrap();
    let bytes = fs::read(path.join("state.json")).unwrap();
    let catalog: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    for fault in 0..8 {
        let mut invalid = catalog.clone();
        match fault {
            0 => invalid["revision"] = serde_json::json!(3),
            1 => invalid["paused"] = serde_json::json!(true),
            2 => invalid["generation"] = serde_json::json!("00".repeat(16)),
            3 => invalid["generation"] = serde_json::json!("AB".repeat(16)),
            4 => invalid["receipts"][1]["expected_revision"] = serde_json::json!(0),
            5 => invalid["receipts"][1]["request"] = invalid["receipts"][0]["request"].clone(),
            6 => invalid["unknown"] = serde_json::json!(true),
            _ => invalid["receipts"][1]["unknown"] = serde_json::json!(true),
        }
        let bad = serde_json::to_vec(&invalid).unwrap();
        fs::write(path.join("state.json"), &bad).unwrap();
        assert_eq!(journal.current(), Err(Error::Unavailable), "fault {fault}");
        assert_eq!(fs::read(path.join("state.json")).unwrap(), bad);
    }
    fs::write(path.join("state.json"), &bytes).unwrap();
    assert_eq!(journal.current().unwrap().revision, 2);
}
