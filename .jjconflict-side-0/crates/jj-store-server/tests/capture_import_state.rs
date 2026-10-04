#![cfg(unix)]
use jj_store_model::{PrincipalId, RequestId};
use jj_store_server::capture_import_state::*;
use std::fs;
#[path = "capture_import_state/capacity.rs"]
mod capacity;
#[path = "capture_import_state/cold.rs"]
mod cold;
#[path = "capture_import_state/safety.rs"]
mod safety;
const INSTALLATION: [u8; 16] = [0xab; 16];
const GENERATION: [u8; 16] = [0xcd; 16];
fn intent(request: u128) -> CaptureImportIntent {
    CaptureImportIntent {
        version: 1,
        generation: GENERATION,
        principal: PrincipalId::new([9; 16]),
        request: RequestId::new(request.to_be_bytes()),
        expected_revision: 4,
        selector: format!("{}:0000000000000001", "ef".repeat(16)),
    }
}
fn source() -> CaptureImportSourceBinding {
    use jj_store_capture::profile::CaptureProfile;
    CaptureImportSourceBinding {
        profile: CaptureProfile::CodexCliJsonlV1,
        product_version: "0.156.1".into(),
        physical_digest: [1; 32],
        header_digest: [2; 32],
        session_digest: [3; 32],
        prefix_digest: [4; 32],
        complete_byte_offset: 128,
        complete_records: 2,
    }
}

#[test]
fn import_receipt_survives_reopen_and_exact_replay_never_retargets_its_source() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("imports");
    let journal = CaptureImportJournal::create(&path, INSTALLATION, GENERATION).unwrap();
    let command = intent(1);
    assert!(journal.resolve(&command).unwrap().is_none());
    let accepted = journal.record(&command, &source()).unwrap();
    assert!(!accepted.replayed && accepted.receipt.ordinal == 1);
    let bytes = fs::read(path.join("state.json")).unwrap();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(
        json["receipts"][0]["source"]["physical_digest"],
        "01".repeat(32)
    );
    drop(journal);
    let reopened = CaptureImportJournal::open(&path, INSTALLATION, GENERATION).unwrap();
    assert_eq!(
        reopened.resolve(&command).unwrap(),
        Some(accepted.receipt.clone())
    );
    let mut replacement = source();
    replacement.physical_digest = [8; 32];
    replacement.prefix_digest = [7; 32];
    replacement.complete_byte_offset = 4096;
    let replay = reopened.record(&command, &replacement).unwrap();
    assert!(replay.replayed);
    assert_eq!(replay.receipt, accepted.receipt);
    assert_eq!(fs::read(path.join("state.json")).unwrap(), bytes);
    reopened.record(&intent(2), &replacement).unwrap();
    let selected = fs::read(path.join("state.json")).unwrap();
    assert_eq!(
        reopened.record(&command, &source()).unwrap().receipt,
        accepted.receipt
    );
    for field in 0..3 {
        let mut changed = command.clone();
        match field {
            0 => changed.selector = format!("{}:0000000000000002", "ef".repeat(16)),
            1 => changed.expected_revision = 5,
            _ => changed.principal = PrincipalId::new([8; 16]),
        }
        assert_eq!(reopened.resolve(&changed), Err(Error::Conflict));
        assert_eq!(reopened.record(&changed, &source()), Err(Error::Conflict));
    }
    assert_eq!(fs::read(path.join("state.json")).unwrap(), selected);
}

#[test]
fn invalid_import_inputs_select_nothing_and_foreign_generations_do_not_inherit_receipts() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("imports");
    assert!(matches!(
        CaptureImportJournal::create(&path, [0; 16], GENERATION),
        Err(Error::Invalid)
    ));
    assert!(!path.exists());
    let journal = CaptureImportJournal::create(&path, INSTALLATION, GENERATION).unwrap();
    let bytes = fs::read(path.join("state.json")).unwrap();
    for field in 0..8 {
        let mut bad = intent(1);
        match field {
            0 => bad.version = 2,
            1 => bad.principal = PrincipalId::new([0; 16]),
            2 => bad.request = RequestId::new([0; 16]),
            3 => bad.generation = [0; 16],
            4 => bad.selector = format!("{}:0000000000000000", "ef".repeat(16)),
            5 => bad.selector = format!("{}:0000000000000001", "EF".repeat(16)),
            6 => bad.selector = "界".repeat(16) + ":",
            _ => bad.selector = format!("{}:0000000000000001", "00".repeat(16)),
        }
        assert_eq!(journal.record(&bad, &source()), Err(Error::Invalid));
        assert_eq!(journal.resolve(&bad), Err(Error::Invalid));
    }
    for field in 0..4 {
        let mut bad = source();
        match field {
            0 => bad.complete_records = 0,
            1 => bad.complete_byte_offset = 1,
            2 => bad.prefix_digest = [0; 32],
            _ => bad.product_version = "other".into(),
        }
        assert_eq!(journal.record(&intent(1), &bad), Err(Error::Invalid));
    }
    let mut foreign = intent(1);
    foreign.generation = [1; 16];
    assert_eq!(
        journal.record(&foreign, &source()),
        Err(Error::ForeignGeneration)
    );
    assert_eq!(journal.resolve(&foreign), Err(Error::ForeignGeneration));
    assert_eq!(fs::read(path.join("state.json")).unwrap(), bytes);
    assert!(journal.entries().unwrap().is_empty());
}
