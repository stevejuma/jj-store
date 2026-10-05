use super::*;

fn lookup(original: &s::Preview) -> s::StatusRequest {
    let request = &original.plan.request;
    s::StatusRequest {
        version: 1,
        scope: s::RecoveryScope::from(&request.selection),
        request_id: request.request_id,
        intent_digest: request.intent_digest,
    }
}
#[test]
fn public_status_keeps_original_scope_identity_and_excludes_private_records() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("stacks");
    let journal = StackCommandJournal::create(&path, scope()).unwrap();
    let original = preview(11);
    journal.record_prepared(actor(), &original).unwrap();
    let request = lookup(&original);
    let result = journal.public_status(actor(), &request).unwrap();
    result.validate().unwrap();
    assert!(result.result.as_ref().unwrap().replayed);
    assert_eq!(result.result.as_ref().unwrap().preview, original);
    let encoded = serde_json::to_value(&result).unwrap();
    let projected = encoded["result"].as_object().unwrap();
    assert_eq!(projected.len(), 4);
    for key in ["principal", "execution", "retention", "snapshot"] {
        assert!(!projected.contains_key(key));
    }
    let bytes = std::fs::read(path.join("state.json")).unwrap();
    assert_eq!(
        journal.public_status(m::PrincipalId::new([99; 16]), &request),
        Err(Error::IntentMismatch)
    );
    let mut wrong = request.clone();
    wrong.intent_digest = m::IntentDigest::new([99; 32]);
    assert_eq!(
        journal.public_status(actor(), &wrong),
        Err(Error::IntentMismatch)
    );
    wrong = request.clone();
    wrong.scope.physical_workspace = s::PhysicalWorkspaceId(m::RecordDigest::new([99; 32]));
    assert_eq!(
        journal.public_status(actor(), &wrong),
        Err(Error::ScopeMismatch)
    );
    wrong = request.clone();
    wrong.request_id = m::RequestId::new([99; 16]);
    assert!(
        journal
            .public_status(actor(), &wrong)
            .unwrap()
            .result
            .is_none()
    );
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
    drop(journal);
    let reopened = StackCommandJournal::open(&path, scope()).unwrap();
    assert_eq!(reopened.public_status(actor(), &request).unwrap(), result);
}

#[test]
fn public_pending_is_actor_scoped_and_retirement_does_not_restore_commands() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("stacks");
    let journal = StackCommandJournal::create(&path, scope()).unwrap();
    let original = preview(11);
    let other = preview(12);
    let other_actor = m::PrincipalId::new([99; 16]);
    journal.record_prepared(actor(), &original).unwrap();
    journal.record_prepared(other_actor, &other).unwrap();
    let request = s::PendingRequest {
        version: 1,
        scope: lookup(&original).scope,
    };
    let selected = journal.public_pending(actor(), &request).unwrap();
    selected.validate().unwrap();
    assert_eq!(selected.commands.len(), 1);
    assert_eq!(selected.commands[0].preview, original);
    assert_eq!(
        journal
            .public_pending(other_actor, &request)
            .unwrap()
            .commands[0]
            .preview,
        other
    );
    let apply = apply(&original);
    journal
        .transition(
            actor(),
            &apply,
            &s::ReceiptState::Prepared,
            &s::ReceiptState::PublicationUnknown,
        )
        .unwrap();
    let retirement = s::ReceiptState::Retired {
        operation: original.plan.operation,
        reason: s::Retirement::Undone,
    };
    journal
        .transition(
            actor(),
            &apply,
            &s::ReceiptState::PublicationUnknown,
            &retirement,
        )
        .unwrap();
    let bytes = std::fs::read(path.join("state.json")).unwrap();
    assert!(
        journal
            .public_pending(actor(), &request)
            .unwrap()
            .commands
            .is_empty()
    );
    assert_eq!(
        journal
            .public_status(actor(), &lookup(&original))
            .unwrap()
            .result
            .unwrap()
            .receipt
            .state,
        retirement
    );
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
}

#[test]
fn public_result_validation_rejects_mismatched_preparation_and_receipt() {
    let temporary = tempfile::tempdir().unwrap();
    let journal = StackCommandJournal::create(&temporary.path().join("stacks"), scope()).unwrap();
    let original = preview(11);
    let mut selected = journal.record_prepared(actor(), &original).unwrap();
    selected.public_result().unwrap().validate().unwrap();
    selected.receipt.prepared = Some(s::PreparedIdentity(m::RecordDigest::new([99; 32])));
    assert_eq!(selected.public_result(), Err(Error::Invalid));
}
