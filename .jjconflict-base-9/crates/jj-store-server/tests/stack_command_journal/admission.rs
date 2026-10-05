use super::*;
use jj_store_server::stack_command_journal::{MAX_RECORDS, Profile};
use std::os::unix::fs::{PermissionsExt as _, symlink};

#[test]
fn open_status_and_discovery_never_initialize_missing_state() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("missing");
    assert!(matches!(
        StackCommandJournal::open(&path, scope()),
        Err(Error::Unavailable)
    ));
    assert!(!path.exists());
    let journal = StackCommandJournal::create(&path, scope()).unwrap();
    let bytes = std::fs::read(path.join("state.json")).unwrap();
    assert_eq!(journal.pending(actor()), Ok(vec![]));
    assert_eq!(
        journal.status(
            actor(),
            m::RequestId::new([11; 16]),
            preview(11).plan.request.intent_digest
        ),
        Ok(None)
    );
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
    std::fs::remove_file(path.join("state.json")).unwrap();
    assert_eq!(journal.pending(actor()), Err(Error::Unavailable));
    assert!(matches!(
        StackCommandJournal::open(&path, scope()),
        Err(Error::Unavailable)
    ));
    assert!(!path.join("state.json").exists());
}

#[test]
fn installation_repository_physical_workspace_name_and_profile_are_bound() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("stacks");
    let journal = StackCommandJournal::create(&path, scope()).unwrap();
    for index in 0..5 {
        let mut changed = scope();
        match index {
            0 => changed.installation = [22; 16],
            1 => changed.repository = m::RepositoryId::new([22; 16]),
            2 => {
                changed.physical_workspace = s::PhysicalWorkspaceId(m::RecordDigest::new([22; 32]))
            }
            3 => changed.jj_workspace = "other".into(),
            _ => changed.profile = Profile::Native,
        }
        assert!(matches!(
            StackCommandJournal::open(&path, changed),
            Err(Error::ScopeMismatch)
        ));
    }
    let mut foreign = preview(11);
    foreign.plan.request.selection.physical_workspace =
        s::PhysicalWorkspaceId(m::RecordDigest::new([22; 32]));
    foreign.plan.request.intent_digest = foreign.plan.request.intent().unwrap();
    foreign.prepared = foreign.plan.identity().unwrap();
    assert_eq!(
        journal.record_prepared(actor(), &foreign),
        Err(Error::ScopeMismatch)
    );
    assert_eq!(
        journal.record_prepared(m::PrincipalId::new([0; 16]), &preview(11)),
        Err(Error::Invalid)
    );
    assert_eq!(
        journal.pending(m::PrincipalId::new([0; 16])),
        Err(Error::Invalid)
    );
    assert_eq!(
        journal.record_prepared(actor(), &preview(0)),
        Err(Error::Invalid)
    );
    let mut invalid = preview(11);
    invalid.prepared = s::PreparedIdentity(m::RecordDigest::new([22; 32]));
    assert_eq!(
        journal.record_prepared(actor(), &invalid),
        Err(Error::Invalid)
    );
    let mut invalid_scope = scope();
    invalid_scope.jj_workspace = " ".into();
    assert!(matches!(
        StackCommandJournal::create(&temporary.path().join("invalid"), invalid_scope),
        Err(Error::Invalid)
    ));
    assert!(!temporary.path().join("invalid").exists());
}

#[test]
fn record_capacity_never_evicts_existing_requests_and_replay_still_works() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("stacks");
    let journal = StackCommandJournal::create(&path, scope()).unwrap();
    for id in 1..=MAX_RECORDS {
        journal
            .record_prepared(actor(), &preview(id as u8))
            .unwrap();
    }
    let bytes = std::fs::read(path.join("state.json")).unwrap();
    assert_eq!(
        journal.record_prepared(actor(), &preview((MAX_RECORDS + 1) as u8)),
        Err(Error::Capacity)
    );
    assert_eq!(journal.pending(actor()).unwrap().len(), MAX_RECORDS);
    assert!(
        journal
            .record_prepared(actor(), &preview(1))
            .unwrap()
            .replayed
    );
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
}

#[test]
fn byte_capacity_failure_leaves_exact_prior_selection() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("stacks");
    let journal = StackCommandJournal::create(&path, scope()).unwrap();
    let mut large = preview(11);
    large.plan.checkout.paths = (0..4096)
        .map(|i| format!("src/{i:04}/{}", "a".repeat(1100)))
        .collect();
    large.prepared = large.plan.identity().unwrap();
    journal.record_prepared(actor(), &large).unwrap();
    let bytes = std::fs::read(path.join("state.json")).unwrap();
    let mut another = large.clone();
    another.plan.request.request_id = m::RequestId::new([12; 16]);
    another.prepared = another.plan.identity().unwrap();
    assert_eq!(
        journal.record_prepared(actor(), &another),
        Err(Error::Capacity)
    );
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
    assert!(journal.record_prepared(actor(), &large).unwrap().replayed);
}

#[test]
fn near_full_admission_reserves_space_for_blocked_checkout_and_retirement() {
    use jj_store_server::stack_command_journal::{MAX_BYTES, Record};
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("stacks");
    let journal = StackCommandJournal::create(&path, scope()).unwrap();
    let mut large = preview(11);
    large.plan.checkout.paths = (0..4096)
        .map(|i| format!("src/{i:04}/{}", "a".repeat(1100)))
        .collect();
    large.prepared = large.plan.identity().unwrap();
    journal.record_prepared(actor(), &large).unwrap();
    let bytes = std::fs::read(path.join("state.json")).unwrap();
    let mut another = preview(12);
    another.plan.checkout.paths = (0..4096)
        .map(|i| format!("src/{i:04}/{}", "a".repeat(917)))
        .collect();
    another.prepared = another.plan.identity().unwrap();
    let request = &another.plan.request;
    let record = Record {
        principal: actor(),
        preview: another.clone(),
        receipt: s::Receipt {
            version: 1,
            request_id: request.request_id,
            intent_digest: request.intent_digest,
            selection: request.selection.clone(),
            prepared: Some(another.prepared),
            state: s::ReceiptState::Prepared,
        },
    };
    let mut candidate: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    candidate["records"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::to_value(record).unwrap());
    let would_fit = serde_json::to_vec(&candidate).unwrap().len();
    assert!(would_fit < MAX_BYTES && would_fit > MAX_BYTES - 32 * 1024);
    assert_eq!(
        journal.record_prepared(actor(), &another),
        Err(Error::Capacity)
    );
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
    journal
        .transition(
            actor(),
            &apply(&large),
            &s::ReceiptState::Prepared,
            &s::ReceiptState::PublicationUnknown,
        )
        .unwrap();
    let required = published(&large, s::CheckoutState::Required);
    journal
        .transition(
            actor(),
            &apply(&large),
            &s::ReceiptState::PublicationUnknown,
            &required,
        )
        .unwrap();
    let blocked = published(
        &large,
        s::CheckoutState::Blocked {
            reason: "\\".repeat(4096),
        },
    );
    journal
        .transition(actor(), &apply(&large), &required, &blocked)
        .unwrap();
    let retired = s::ReceiptState::Retired {
        operation: large.plan.operation,
        reason: s::Retirement::Abandoned,
    };
    journal
        .transition(actor(), &apply(&large), &blocked, &retired)
        .unwrap();
    assert_eq!(
        journal
            .record_prepared(actor(), &large)
            .unwrap()
            .receipt
            .state,
        retired
    );
}

#[test]
fn private_path_modes_symlinks_and_replaced_roots_fail_closed() {
    let temporary = tempfile::tempdir().unwrap();
    let checkout = temporary.path().join("repo");
    std::fs::create_dir(&checkout).unwrap();
    std::fs::create_dir(checkout.join(".jj")).unwrap();
    assert!(matches!(
        StackCommandJournal::create(&checkout.join("stacks"), scope()),
        Err(Error::Unavailable)
    ));
    assert!(!checkout.join("stacks").exists());
    let path = temporary.path().join("stacks");
    let journal = StackCommandJournal::create(&path, scope()).unwrap();
    assert_eq!(
        std::fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o700
    );
    assert_eq!(
        std::fs::metadata(path.join("state.json"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o600
    );
    std::fs::set_permissions(
        path.join("state.json"),
        std::fs::Permissions::from_mode(0o644),
    )
    .unwrap();
    assert_eq!(journal.pending(actor()), Err(Error::Unavailable));
    std::fs::set_permissions(
        path.join("state.json"),
        std::fs::Permissions::from_mode(0o600),
    )
    .unwrap();
    let held = temporary.path().join("state-held");
    std::fs::rename(path.join("state.json"), &held).unwrap();
    symlink(&held, path.join("state.json")).unwrap();
    assert_eq!(journal.pending(actor()), Err(Error::Unavailable));
    std::fs::remove_file(path.join("state.json")).unwrap();
    std::fs::rename(held, path.join("state.json")).unwrap();
    std::fs::rename(&path, temporary.path().join("old-stacks")).unwrap();
    StackCommandJournal::create(&path, scope()).unwrap();
    assert_eq!(
        journal.record_prepared(actor(), &preview(11)),
        Err(Error::Unavailable)
    );
}

#[test]
fn corrupted_version_duplicates_preparation_and_multiple_pending_records_are_unavailable() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("stacks");
    let journal = StackCommandJournal::create(&path, scope()).unwrap();
    journal.record_prepared(actor(), &preview(11)).unwrap();
    journal.record_prepared(actor(), &preview(12)).unwrap();
    let original: serde_json::Value =
        serde_json::from_slice(&std::fs::read(path.join("state.json")).unwrap()).unwrap();
    for index in 0..5 {
        let mut value = original.clone();
        match index {
            0 => value["version"] = 2.into(),
            1 => value["extra"] = true.into(),
            2 => value["records"][1] = value["records"][0].clone(),
            3 => {
                value["records"][0]["preview"]["plan"]["checkout"]["paths"] =
                    serde_json::json!(["src/tampered"])
            }
            _ => {
                for record in value["records"].as_array_mut().unwrap() {
                    record["receipt"]["state"] =
                        serde_json::json!({"status": "publication_unknown"});
                }
            }
        }
        std::fs::write(path.join("state.json"), serde_json::to_vec(&value).unwrap()).unwrap();
        assert_eq!(journal.pending(actor()), Err(Error::Unavailable));
        assert!(matches!(
            StackCommandJournal::open(&path, scope()),
            Err(Error::Unavailable)
        ));
    }
}
