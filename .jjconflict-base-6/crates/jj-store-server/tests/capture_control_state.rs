#![cfg(unix)]
use jj_store_model::{PrincipalId, RequestId};
use jj_store_server::capture_control_state::{CaptureControlChange, CaptureControlJournal, Error};

const INSTALLATION: [u8; 16] = [0xab; 16];
#[path = "capture_control_state/capacity.rs"]
mod capacity;
#[path = "capture_control_state/safety.rs"]
mod safety;
fn change(generation: [u8; 16], request: u8, revision: u64, paused: bool) -> CaptureControlChange {
    CaptureControlChange {
        generation,
        principal: PrincipalId::new([9; 16]),
        request: RequestId::new([request; 16]),
        expected_revision: revision,
        paused,
    }
}

#[test]
fn pause_receipts_survive_reopen_and_old_replay_cannot_reverse_newer_resume() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("controls");
    let journal = CaptureControlJournal::create(&path, INSTALLATION).unwrap();
    let initial = journal.current().unwrap();
    assert_eq!(initial.revision, 0);
    assert!(!initial.paused);
    let pause = change(initial.generation, 1, 0, true);
    let selected = journal.record(&pause).unwrap();
    assert!(selected.current.paused && !selected.replayed);
    assert_eq!(selected.receipt.revision, 1);
    let bytes = std::fs::read(path.join("state.json")).unwrap();
    drop(journal);
    let reopened = CaptureControlJournal::open(&path, INSTALLATION).unwrap();
    let replay = reopened.record(&pause).unwrap();
    assert!(replay.replayed);
    assert_eq!(replay.receipt, selected.receipt);
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
    let resumed = reopened
        .record(&change(initial.generation, 2, 1, false))
        .unwrap();
    assert!(!resumed.current.paused && resumed.current.revision == 2);
    let bytes = std::fs::read(path.join("state.json")).unwrap();
    let replay = reopened.record(&pause).unwrap();
    assert_eq!(replay.receipt, selected.receipt);
    assert_eq!(replay.current, resumed.current);
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
    let mut conflicting = pause;
    conflicting.paused = false;
    assert_eq!(reopened.record(&conflicting), Err(Error::Conflict));
    assert_eq!(
        reopened.record(&change(initial.generation, 3, 0, true)),
        Err(Error::StaleRevision)
    );
    conflicting.expected_revision = 2;
    conflicting.principal = PrincipalId::new([8; 16]);
    assert_eq!(reopened.record(&conflicting), Err(Error::Conflict));
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
}

#[test]
fn zero_identities_and_checkout_storage_are_rejected_before_selection() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("controls");
    assert!(matches!(
        CaptureControlJournal::create(&path, [0; 16]),
        Err(Error::Invalid)
    ));
    assert!(!path.exists());
    let journal = CaptureControlJournal::create(&path, INSTALLATION).unwrap();
    let initial = journal.current().unwrap();
    let mut intent = change(initial.generation, 1, 0, true);
    intent.principal = PrincipalId::new([0; 16]);
    assert_eq!(journal.record(&intent), Err(Error::Invalid));
    intent.principal = PrincipalId::new([9; 16]);
    intent.request = RequestId::new([0; 16]);
    assert_eq!(journal.record(&intent), Err(Error::Invalid));
    assert_eq!(journal.current().unwrap(), initial);
    let checkout = temporary.path().join("checkout");
    std::fs::create_dir(&checkout).unwrap();
    std::fs::create_dir(checkout.join(".git")).unwrap();
    assert!(matches!(
        CaptureControlJournal::create(&checkout.join("controls"), INSTALLATION),
        Err(Error::Unavailable)
    ));
    assert!(!checkout.join("controls").exists());
}
