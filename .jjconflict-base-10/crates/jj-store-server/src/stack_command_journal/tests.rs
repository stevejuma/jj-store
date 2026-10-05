use super::*;
#[path = "../../tests/stack_command_journal/fixture.rs"]
mod fixture;
use fixture::*;

#[test]
fn lost_coordination_after_atomic_replacement_is_unknown_and_replay_syncs_original() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("stacks");
    let journal = StackCommandJournal::create(&path, scope()).unwrap();
    let preview = preview(11);
    journal.record_prepared(actor(), &preview).unwrap();
    let gate = journal.root.gate().unwrap();
    let (mut catalog, stamp) = journal.read(&gate).unwrap();
    catalog.records[0].receipt.state = s::ReceiptState::PublicationUnknown;
    let bytes = catalog.encode().unwrap();
    let held = temporary.path().join("held-lock");
    assert_eq!(
        journal
            .root
            .persist_with(&gate, &bytes, Some(stamp), || {
                std::fs::rename(path.join("stacks.lock"), &held).unwrap();
                Ok(())
            })
            .map_err(Error::from),
        Err(Error::OutcomeUnknown)
    );
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
    drop(gate);
    assert_eq!(journal.pending(actor()), Err(Error::Unavailable));
    std::fs::rename(held, path.join("stacks.lock")).unwrap();
    let reopened = StackCommandJournal::open(&path, scope()).unwrap();
    let replay = reopened.record_prepared(actor(), &preview).unwrap();
    assert!(replay.replayed);
    assert_eq!(replay.receipt.state, s::ReceiptState::PublicationUnknown);
    assert!(!replay.receipt.may_execute());
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
}

#[test]
fn gate_and_selected_identity_prevent_concurrent_or_replaced_state_updates() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("stacks");
    let journal = StackCommandJournal::create(&path, scope()).unwrap();
    let second = StackCommandJournal::open(&path, scope()).unwrap();
    let gate = journal.root.gate().unwrap();
    let (catalog, stamp) = journal.read(&gate).unwrap();
    assert_eq!(
        second.record_prepared(actor(), &preview(11)),
        Err(Error::Unavailable)
    );
    let replacement = path.join(".stack-command-replacement");
    std::fs::write(&replacement, catalog.encode().unwrap()).unwrap();
    use std::os::unix::fs::PermissionsExt as _;
    std::fs::set_permissions(&replacement, std::fs::Permissions::from_mode(0o600)).unwrap();
    std::fs::rename(replacement, path.join("state.json")).unwrap();
    assert_eq!(
        journal
            .root
            .persist(&gate, &catalog.encode().unwrap(), Some(stamp))
            .map_err(Error::from),
        Err(Error::Unavailable)
    );
}

#[test]
fn pre_replacement_failure_never_selects_new_receipt() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("stacks");
    let journal = StackCommandJournal::create(&path, scope()).unwrap();
    let gate = journal.root.gate().unwrap();
    let (bytes, stamp) = journal.root.read(&gate).unwrap();
    let held = temporary.path().join("held-lock");
    std::fs::rename(path.join("stacks.lock"), &held).unwrap();
    assert_eq!(
        journal
            .root
            .persist(&gate, b"replacement", Some(stamp))
            .map_err(Error::from),
        Err(Error::Unavailable)
    );
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
    std::fs::rename(held, path.join("stacks.lock")).unwrap();
}

#[test]
fn process_loss_after_replacement_recovers_exact_unresolved_receipt() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("stacks");
    let journal = StackCommandJournal::create(&path, scope()).unwrap();
    let original = preview(11);
    journal.record_prepared(actor(), &original).unwrap();
    drop(journal);
    let child = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--ignored",
            "--exact",
            "stack_command_journal::tests::exit_after_selected_replacement",
            "--nocapture",
        ])
        .env("JJ_STORE_STACK_JOURNAL_LOSS_PATH", &path)
        .output()
        .unwrap();
    assert_eq!(
        child.status.code(),
        Some(93),
        "{}",
        String::from_utf8_lossy(&child.stderr)
    );
    let bytes = std::fs::read(path.join("state.json")).unwrap();
    let reopened = StackCommandJournal::open(&path, scope()).unwrap();
    let replay = reopened.record_prepared(actor(), &original).unwrap();
    assert!(replay.replayed);
    assert_eq!(replay.receipt.state, s::ReceiptState::PublicationUnknown);
    let status = reopened
        .status(
            actor(),
            original.plan.request.request_id,
            original.plan.request.intent_digest,
        )
        .unwrap()
        .unwrap();
    assert_eq!(status.preview, original);
    assert!(!status.receipt.may_execute());
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
    // Real code publication still requires provider evidence; metadata cannot
    // skip straight to completed physical checkout.
    assert_eq!(
        reopened.transition(
            actor(),
            &apply(&original),
            &s::ReceiptState::PublicationUnknown,
            &published(
                &original,
                s::CheckoutState::Verified {
                    fingerprint: m::RecordDigest::new([44; 32])
                }
            )
        ),
        Err(Error::InvalidTransition)
    );
}

#[test]
#[ignore = "subprocess fixture exits after atomic replacement before sync/ack"]
fn exit_after_selected_replacement() {
    let path =
        std::path::PathBuf::from(std::env::var_os("JJ_STORE_STACK_JOURNAL_LOSS_PATH").unwrap());
    let journal = StackCommandJournal::open(&path, scope()).unwrap();
    let gate = journal.root.gate().unwrap();
    let (mut catalog, stamp) = journal.read(&gate).unwrap();
    catalog.records[0].receipt.state = s::ReceiptState::PublicationUnknown;
    let bytes = catalog.encode().unwrap();
    let _ = journal
        .root
        .persist_with(&gate, &bytes, Some(stamp), || std::process::exit(93));
    panic!("subprocess must exit within the after-replacement boundary");
}
