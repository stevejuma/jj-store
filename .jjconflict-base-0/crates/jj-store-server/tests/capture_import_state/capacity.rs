use super::*;

#[test]
fn byte_capacity_preserves_exact_replay_and_refuses_new_intent_without_eviction() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("imports");
    CaptureImportJournal::create(&path, INSTALLATION, GENERATION).unwrap();
    let mut catalog: serde_json::Value =
        serde_json::from_slice(&fs::read(path.join("state.json")).unwrap()).unwrap();
    let mut size = serde_json::to_vec(&catalog).unwrap().len();
    let mut receipts = vec![];
    loop {
        let ordinal = receipts.len() as u64 + 1;
        let receipt = CaptureImportReceipt {
            ordinal,
            intent: intent(ordinal as u128),
            source: source(),
        };
        let extra = serde_json::to_vec(&receipt).unwrap().len() + usize::from(!receipts.is_empty());
        if size + extra > MAX_BYTES {
            break;
        }
        size += extra;
        receipts.push(receipt);
    }
    assert!(receipts.len() < MAX_RECEIPTS);
    catalog["receipts"] = serde_json::to_value(&receipts).unwrap();
    let bytes = serde_json::to_vec(&catalog).unwrap();
    assert_eq!(bytes.len(), size);
    fs::write(path.join("state.json"), &bytes).unwrap();
    let reopened = CaptureImportJournal::open(&path, INSTALLATION, GENERATION).unwrap();
    assert_eq!(
        reopened.record(&intent(receipts.len() as u128 + 1), &source()),
        Err(Error::Capacity)
    );
    let mut invalid_offered_source = source();
    invalid_offered_source.prefix_digest = [0; 32];
    let replay = reopened
        .record(&receipts[0].intent, &invalid_offered_source)
        .unwrap();
    assert!(replay.replayed);
    assert_eq!(replay.receipt, receipts[0]);
    assert_eq!(reopened.entries().unwrap(), receipts);
    assert_eq!(fs::read(path.join("state.json")).unwrap(), bytes);
}

#[test]
fn malformed_import_catalogs_fail_closed_without_repair() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("imports");
    let journal = CaptureImportJournal::create(&path, INSTALLATION, GENERATION).unwrap();
    journal.record(&intent(1), &source()).unwrap();
    journal.record(&intent(2), &source()).unwrap();
    let bytes = fs::read(path.join("state.json")).unwrap();
    let catalog: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    for fault in 0..18 {
        let mut invalid = catalog.clone();
        match fault {
            0 => invalid["version"] = serde_json::json!(2),
            1 => invalid["installation"] = serde_json::json!("AB".repeat(16)),
            2 => invalid["control_generation"] = serde_json::json!("00".repeat(16)),
            3 => invalid["receipts"][1]["ordinal"] = serde_json::json!(1),
            4 => {
                invalid["receipts"][1]["intent"]["request"] =
                    invalid["receipts"][0]["intent"]["request"].clone()
            }
            5 => {
                invalid["receipts"][1]["intent"]["generation"] = serde_json::json!("ab".repeat(16))
            }
            6 => {
                invalid["receipts"][1]["source"]["physical_digest"] =
                    serde_json::json!("AB".repeat(32))
            }
            7 => {
                invalid["receipts"][1]["source"]["header_digest"] =
                    serde_json::json!("01".repeat(31))
            }
            8 => {
                invalid["receipts"][1]["source"]["session_digest"] =
                    serde_json::json!("00".repeat(32))
            }
            9 => invalid["receipts"][1]["source"]["complete_records"] = serde_json::json!(0),
            10 => invalid["receipts"][1]["source"]["product_version"] = serde_json::json!("other"),
            11 => invalid["unknown"] = serde_json::json!(true),
            12 => invalid["receipts"][1]["unknown"] = serde_json::json!(true),
            13 => invalid["receipts"][1]["intent"]["unknown"] = serde_json::json!(true),
            14 => invalid["receipts"][1]["source"]["unknown"] = serde_json::json!(true),
            15 => {
                invalid["receipts"][1]["intent"]["selector"] =
                    serde_json::json!("界".repeat(16) + ":")
            }
            16 => invalid["receipts"][1]["source"]["profile"] = serde_json::json!("unsupported"),
            _ => invalid["receipts"][1]["intent"]["version"] = serde_json::json!(2),
        }
        let bad = serde_json::to_vec(&invalid).unwrap();
        fs::write(path.join("state.json"), &bad).unwrap();
        assert_eq!(journal.entries(), Err(Error::Unavailable), "fault {fault}");
        assert!(matches!(
            CaptureImportJournal::open(&path, INSTALLATION, GENERATION),
            Err(Error::Unavailable)
        ));
        assert_eq!(fs::read(path.join("state.json")).unwrap(), bad);
    }
    fs::write(path.join("state.json"), &bytes).unwrap();
    assert_eq!(journal.entries().unwrap().len(), 2);
}
