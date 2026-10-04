use super::*;
use jj_store_capture::profile::CaptureProfile;
use std::fs;
mod reader;
mod reader_verification;
const INSTALLATION: [u8; 16] = [0xab; 16];
const GENERATION: [u8; 16] = [0xcd; 16];
fn intent() -> CaptureImportIntent {
    CaptureImportIntent {
        version: 1,
        generation: GENERATION,
        principal: PrincipalId::new([9; 16]),
        request: RequestId::new([1; 16]),
        expected_revision: 4,
        selector: format!("{}:0000000000000001", "ef".repeat(16)),
    }
}
fn source() -> CaptureImportSourceBinding {
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
fn lost_coordination_after_import_replacement_is_unknown_and_exact_receipt_recovers() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("imports");
    let journal = CaptureImportJournal::create(&path, INSTALLATION, GENERATION).unwrap();
    let gate = journal.root.gate().unwrap();
    let (before, stamp) = journal.root.read(&gate).unwrap();
    let mut catalog = journal.decode(&before).unwrap();
    let receipt = CaptureImportReceipt {
        ordinal: 1,
        intent: intent(),
        source: source(),
    };
    catalog.receipts.push(receipt.clone());
    let bytes = catalog.encode().unwrap();
    let held = temporary.path().join("held-lock");
    assert_eq!(
        journal
            .root
            .persist_with(&gate, &bytes, Some(stamp), || {
                fs::rename(path.join("import.lock"), &held).unwrap();
                Ok(())
            })
            .map_err(Error::from),
        Err(Error::OutcomeUnknown)
    );
    assert_eq!(fs::read(path.join("state.json")).unwrap(), bytes);
    drop(gate);
    assert_eq!(journal.resolve(&intent()), Err(Error::Unavailable));
    fs::rename(held, path.join("import.lock")).unwrap();
    let reopened = CaptureImportJournal::open(&path, INSTALLATION, GENERATION).unwrap();
    let recovered = reopened.record(&intent(), &source()).unwrap();
    assert!(recovered.replayed);
    assert_eq!(recovered.receipt, receipt);
    assert_eq!(fs::read(path.join("state.json")).unwrap(), bytes);
}

#[test]
fn import_callback_retains_gate_until_settlement() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("imports");
    let journal = CaptureImportJournal::create(&path, INSTALLATION, GENERATION).unwrap();
    let second = CaptureImportJournal::open(&path, INSTALLATION, GENERATION).unwrap();
    let (selected, result) = journal
        .record_then(&intent(), &source(), |selected, check| {
            assert!(!selected.replayed);
            assert_eq!(second.entries(), Err(Error::Unavailable));
            check()
        })
        .unwrap();
    assert_eq!(result, Ok(()));
    assert_eq!(second.resolve(&intent()).unwrap(), Some(selected.receipt));
    let (_, result) = journal
        .record_then(&intent(), &source(), |selected, check| {
            assert!(selected.replayed);
            assert_eq!(second.resolve(&intent()), Err(Error::Unavailable));
            check()
        })
        .unwrap();
    assert_eq!(result, Ok(()));
    assert_eq!(second.entries().unwrap().len(), 1);
}

#[test]
fn import_callback_rejects_byte_identical_selected_file_replacement() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("imports");
    let journal = CaptureImportJournal::create(&path, INSTALLATION, GENERATION).unwrap();
    let (_, checked) = journal
        .record_then(&intent(), &source(), |_, check| {
            check().unwrap();
            let replacement = path.join(".capture-import-replacement");
            fs::copy(path.join("state.json"), &replacement).unwrap();
            fs::rename(replacement, path.join("state.json")).unwrap();
            check()
        })
        .unwrap();
    assert_eq!(checked, Err(Error::Unavailable));
    assert!(journal.record(&intent(), &source()).unwrap().replayed);
}
