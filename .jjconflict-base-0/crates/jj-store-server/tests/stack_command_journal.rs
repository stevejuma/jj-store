#![cfg(unix)]
use jj_store_model as m;
use jj_store_server::stack_command_journal::{Error, StackCommandJournal};
use jj_store_service::stack_workspace as s;

#[path = "stack_command_journal/fixture.rs"]
mod fixture;
use fixture::*;
#[path = "stack_command_journal/admission.rs"]
mod admission;
#[path = "stack_command_journal/public.rs"]
mod public;

#[test]
fn retained_preparation_survives_reopen_and_conflicting_request_reuse() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("stacks");
    let journal = StackCommandJournal::create(&path, scope()).unwrap();
    let original = preview(11);
    let saved = journal.record_prepared(actor(), &original).unwrap();
    assert_eq!(saved.receipt.state, s::ReceiptState::Prepared);
    assert!(!saved.replayed);
    let bytes = std::fs::read(path.join("state.json")).unwrap();
    drop(journal);
    let reopened = StackCommandJournal::open(&path, scope()).unwrap();
    let replay = reopened.record_prepared(actor(), &original).unwrap();
    assert!(replay.replayed);
    assert_eq!(replay.preview, original);
    assert_eq!(replay.receipt, saved.receipt);
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
    let mut changed = original.clone();
    changed.plan.checkout.paths = vec!["src/other".into()];
    changed.prepared = changed.plan.identity().unwrap();
    assert_eq!(
        reopened.record_prepared(actor(), &changed),
        Err(Error::IntentMismatch)
    );
    assert_eq!(
        reopened.record_prepared(m::PrincipalId::new([12; 16]), &original),
        Err(Error::IntentMismatch)
    );
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
}

#[test]
fn exact_status_precedes_live_generations_and_discovery_is_actor_scoped() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("stacks");
    let journal = StackCommandJournal::create(&path, scope()).unwrap();
    let original = preview(11);
    journal.record_prepared(actor(), &original).unwrap();
    let other_actor = m::PrincipalId::new([12; 16]);
    journal.record_prepared(other_actor, &preview(12)).unwrap();
    let stored = journal
        .status(
            actor(),
            original.plan.request.request_id,
            original.plan.request.intent_digest,
        )
        .unwrap()
        .unwrap();
    assert_eq!(stored.preview, original);
    assert_eq!(journal.pending(actor()).unwrap(), vec![stored]);
    assert_eq!(journal.pending(other_actor).unwrap().len(), 1);
    assert_eq!(
        journal.status(
            other_actor,
            original.plan.request.request_id,
            original.plan.request.intent_digest
        ),
        Err(Error::IntentMismatch)
    );
    assert_eq!(
        journal.status(
            actor(),
            original.plan.request.request_id,
            m::IntentDigest::new([44; 32])
        ),
        Err(Error::IntentMismatch)
    );
    assert_eq!(
        journal.status(
            actor(),
            m::RequestId::new([13; 16]),
            original.plan.request.intent_digest
        ),
        Ok(None)
    );
    // Changing live pins in a new proposal cannot replace this original.
    let mut changed = original.clone();
    changed.plan.request.selection.binding_generation += 1;
    changed
        .plan
        .request
        .selection
        .combination
        .as_mut()
        .unwrap()
        .generation += 1;
    changed.plan.request.intent_digest = changed.plan.request.intent().unwrap();
    changed.prepared = changed.plan.identity().unwrap();
    assert_eq!(
        journal.record_prepared(actor(), &changed),
        Err(Error::IntentMismatch)
    );
    assert_eq!(
        journal
            .status(
                actor(),
                original.plan.request.request_id,
                original.plan.request.intent_digest
            )
            .unwrap()
            .unwrap()
            .preview,
        original
    );
}

#[test]
fn publication_checkout_and_retirement_are_monotonic_across_reopen() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("stacks");
    let original = preview(11);
    let apply = apply(&original);
    let journal = StackCommandJournal::create(&path, scope()).unwrap();
    journal.record_prepared(actor(), &original).unwrap();
    let required = published(&original, s::CheckoutState::Required);
    assert_eq!(
        journal.transition(actor(), &apply, &s::ReceiptState::Prepared, &required),
        Err(Error::InvalidTransition)
    );
    journal
        .transition(
            actor(),
            &apply,
            &s::ReceiptState::Prepared,
            &s::ReceiptState::PublicationUnknown,
        )
        .unwrap();
    let bytes = std::fs::read(path.join("state.json")).unwrap();
    drop(journal);
    let journal = StackCommandJournal::open(&path, scope()).unwrap();
    let replay = journal
        .transition(
            actor(),
            &apply,
            &s::ReceiptState::Prepared,
            &s::ReceiptState::PublicationUnknown,
        )
        .unwrap();
    assert!(replay.replayed);
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
    assert_eq!(
        journal.transition(
            actor(),
            &apply,
            &s::ReceiptState::PublicationUnknown,
            &s::ReceiptState::Prepared
        ),
        Err(Error::InvalidTransition)
    );
    assert_eq!(
        journal.transition(
            actor(),
            &apply,
            &s::ReceiptState::PublicationUnknown,
            &s::ReceiptState::Rejected {
                reason: "Not proven absent".into()
            }
        ),
        Err(Error::InvalidTransition)
    );
    journal
        .transition(
            actor(),
            &apply,
            &s::ReceiptState::PublicationUnknown,
            &required,
        )
        .unwrap();
    let blocked = published(
        &original,
        s::CheckoutState::Blocked {
            reason: "An editor changed src/one".into(),
        },
    );
    journal
        .transition(actor(), &apply, &required, &blocked)
        .unwrap();
    let verified = published(
        &original,
        s::CheckoutState::Verified {
            fingerprint: m::RecordDigest::new([55; 32]),
        },
    );
    let complete = journal
        .transition(actor(), &apply, &blocked, &verified)
        .unwrap();
    assert!(complete.receipt.completed());
    assert!(!complete.receipt.may_execute());
    assert!(journal.pending(actor()).unwrap().is_empty());
    assert_eq!(
        journal.transition(actor(), &apply, &verified, &required),
        Err(Error::InvalidTransition)
    );
    let retired = s::ReceiptState::Retired {
        operation: original.plan.operation,
        reason: s::Retirement::Undone,
    };
    journal
        .transition(actor(), &apply, &verified, &retired)
        .unwrap();
    drop(journal);
    let journal = StackCommandJournal::open(&path, scope()).unwrap();
    let retry = journal.record_prepared(actor(), &original).unwrap();
    assert!(retry.replayed);
    assert_eq!(retry.receipt.state, retired);
    assert!(!retry.receipt.may_execute());
    assert_eq!(
        journal.transition(
            actor(),
            &apply,
            &retired,
            &s::ReceiptState::PublicationUnknown
        ),
        Err(Error::InvalidTransition)
    );
}

#[test]
fn pending_checkout_blocks_competing_publication_until_verified() {
    let temporary = tempfile::tempdir().unwrap();
    let journal = StackCommandJournal::create(&temporary.path().join("stacks"), scope()).unwrap();
    let first = preview(11);
    let second = preview(12);
    journal.record_prepared(actor(), &first).unwrap();
    journal.record_prepared(actor(), &second).unwrap();
    journal
        .transition(
            actor(),
            &apply(&first),
            &s::ReceiptState::Prepared,
            &s::ReceiptState::PublicationUnknown,
        )
        .unwrap();
    assert_eq!(
        journal.transition(
            actor(),
            &apply(&second),
            &s::ReceiptState::Prepared,
            &s::ReceiptState::PublicationUnknown
        ),
        Err(Error::Pending)
    );
    let required = published(&first, s::CheckoutState::Required);
    journal
        .transition(
            actor(),
            &apply(&first),
            &s::ReceiptState::PublicationUnknown,
            &required,
        )
        .unwrap();
    assert_eq!(
        journal.transition(
            actor(),
            &apply(&second),
            &s::ReceiptState::Prepared,
            &s::ReceiptState::PublicationUnknown
        ),
        Err(Error::Pending)
    );
    let verified = published(
        &first,
        s::CheckoutState::Verified {
            fingerprint: m::RecordDigest::new([44; 32]),
        },
    );
    journal
        .transition(actor(), &apply(&first), &required, &verified)
        .unwrap();
    journal
        .transition(
            actor(),
            &apply(&second),
            &s::ReceiptState::Prepared,
            &s::ReceiptState::PublicationUnknown,
        )
        .unwrap();
}

#[test]
fn transition_requires_exact_preparation_and_expected_historical_state() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("stacks");
    let journal = StackCommandJournal::create(&path, scope()).unwrap();
    let original = preview(11);
    let exact = apply(&original);
    journal.record_prepared(actor(), &original).unwrap();
    let bytes = std::fs::read(path.join("state.json")).unwrap();
    let mut tampered = exact.clone();
    tampered.prepared = s::PreparedIdentity(m::RecordDigest::new([77; 32]));
    assert_eq!(
        journal.transition(
            actor(),
            &tampered,
            &s::ReceiptState::Prepared,
            &s::ReceiptState::PublicationUnknown
        ),
        Err(Error::IntentMismatch)
    );
    tampered = exact.clone();
    tampered.selection.binding_generation += 1;
    assert_eq!(
        journal.transition(
            actor(),
            &tampered,
            &s::ReceiptState::Prepared,
            &s::ReceiptState::PublicationUnknown
        ),
        Err(Error::IntentMismatch)
    );
    assert_eq!(
        journal.transition(
            actor(),
            &exact,
            &s::ReceiptState::PublicationUnknown,
            &published(&original, s::CheckoutState::Required)
        ),
        Err(Error::StaleState)
    );
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
    journal
        .transition(
            actor(),
            &exact,
            &s::ReceiptState::Prepared,
            &s::ReceiptState::PublicationUnknown,
        )
        .unwrap();
    let wrong = s::ReceiptState::Published {
        operation: m::OperationId::new([77; 64]),
        view: original.plan.view,
        checkout: s::CheckoutState::Required,
    };
    assert_eq!(
        journal.transition(
            actor(),
            &exact,
            &s::ReceiptState::PublicationUnknown,
            &wrong
        ),
        Err(Error::Invalid)
    );
    let wrong_view = s::ReceiptState::Published {
        operation: original.plan.operation,
        view: m::ViewId::new([77; 64]),
        checkout: s::CheckoutState::Required,
    };
    assert_eq!(
        journal.transition(
            actor(),
            &exact,
            &s::ReceiptState::PublicationUnknown,
            &wrong_view
        ),
        Err(Error::Invalid)
    );
}

#[test]
fn rejected_and_abandoned_records_never_execute_on_retry() {
    let temporary = tempfile::tempdir().unwrap();
    let journal = StackCommandJournal::create(&temporary.path().join("stacks"), scope()).unwrap();
    let original = preview(11);
    journal.record_prepared(actor(), &original).unwrap();
    let rejected = s::ReceiptState::Rejected {
        reason: "Physical checkout changed".into(),
    };
    journal
        .transition(
            actor(),
            &apply(&original),
            &s::ReceiptState::Prepared,
            &rejected,
        )
        .unwrap();
    assert_eq!(
        journal.transition(
            actor(),
            &apply(&original),
            &rejected,
            &s::ReceiptState::Prepared
        ),
        Err(Error::InvalidTransition)
    );
    let retry = journal.record_prepared(actor(), &original).unwrap();
    assert_eq!(retry.receipt.state, rejected);
    assert!(!retry.receipt.may_execute());
    let abandoned = preview(12);
    journal.record_prepared(actor(), &abandoned).unwrap();
    journal
        .transition(
            actor(),
            &apply(&abandoned),
            &s::ReceiptState::Prepared,
            &s::ReceiptState::PublicationUnknown,
        )
        .unwrap();
    let retired = s::ReceiptState::Retired {
        operation: abandoned.plan.operation,
        reason: s::Retirement::Abandoned,
    };
    journal
        .transition(
            actor(),
            &apply(&abandoned),
            &s::ReceiptState::PublicationUnknown,
            &retired,
        )
        .unwrap();
    assert_eq!(
        journal
            .record_prepared(actor(), &abandoned)
            .unwrap()
            .receipt
            .state,
        retired
    );
    assert!(journal.pending(actor()).unwrap().is_empty());
}
