use super::*;

#[test]
fn lost_coordination_after_replacement_is_unknown_and_exact_receipt_recovers() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("controls");
    let journal = CaptureControlJournal::create(&path, [0xab; 16]).unwrap();
    let initial = journal.current().unwrap();
    let gate = journal.root.gate().unwrap();
    let (before, stamp) = journal.root.read(&gate).unwrap();
    let mut selected = Catalog::decode(&before, journal.installation).unwrap();
    let receipt = CaptureControlReceipt {
        principal: PrincipalId::new([9; 16]),
        request: RequestId::new([1; 16]),
        expected_revision: 0,
        revision: 1,
        paused: true,
    };
    selected.revision = receipt.revision;
    selected.paused = receipt.paused;
    selected.receipts.push(receipt);
    let bytes = selected.encode().unwrap();
    let held = temporary.path().join("held-lock");
    assert_eq!(
        journal
            .root
            .persist_with(&gate, &bytes, Some(stamp), || {
                // A real physical fault after replacement, before durability acknowledgement.
                std::fs::rename(path.join("control.lock"), &held).unwrap();
                Ok(())
            })
            .map_err(Error::from),
        Err(Error::OutcomeUnknown)
    );
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
    drop(gate);
    assert_eq!(journal.current(), Err(Error::Unavailable));
    std::fs::rename(held, path.join("control.lock")).unwrap();
    let reopened = CaptureControlJournal::open(&path, journal.installation).unwrap();
    let recovered = reopened
        .record(&CaptureControlChange {
            generation: initial.generation,
            principal: receipt.principal,
            request: receipt.request,
            expected_revision: receipt.expected_revision,
            paused: receipt.paused,
        })
        .unwrap();
    assert!(recovered.replayed && recovered.current.paused);
    assert_eq!(recovered.current.revision, 1);
    assert_eq!(recovered.receipt, receipt);
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
}
