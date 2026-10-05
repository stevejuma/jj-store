use jj_store_model as m;
use jj_store_service::stack_workspace as s;

fn command() -> s::CommandResult {
    let selection = s::Selection {
        version: 1,
        repository: m::RepositoryId::new([1; 16]),
        binding_generation: 2,
        physical_workspace: s::PhysicalWorkspaceId(m::RecordDigest::new([3; 32])),
        jj_workspace: "default".into(),
        code: s::CodeSelection::GitSha1Jj045 {
            operation: m::OperationId::new([4; 64]),
            view: m::ViewId::new([5; 64]),
            working_copy: m::GitSha1Id::new([6; 20]),
        },
        combination: None,
        working_tree: m::RecordDigest::new([7; 32]),
    };
    let request = s::PrepareRequest::new(
        m::RequestId::new([8; 16]),
        selection,
        s::Command::Setup {
            name: "Daily work".into(),
            target: s::BookmarkTarget {
                name: "trunk".into(),
                remote: Some("origin".into()),
                head: s::CommitIdentity::GitSha1(m::GitSha1Id::new([9; 20])),
            },
        },
    )
    .unwrap();
    let plan = s::PreparedPlan {
        version: 1,
        request,
        operation: m::OperationId::new([10; 64]),
        view: m::ViewId::new([11; 64]),
        checkout: s::CheckoutEffect {
            before: s::TreeIdentity::GitSha1(m::GitSha1Id::new([12; 20])).into(),
            after: s::TreeIdentity::GitSha1(m::GitSha1Id::new([12; 20])).into(),
            paths: vec![],
        },
        combination: s::CombinationSelection {
            id: s::CombinationId(m::RequestId::new([13; 16])),
            generation: 1,
        },
        rewrites: vec![],
        bookmarks: vec![],
        conflicts: vec![],
    };
    let prepared = plan.identity().unwrap();
    let receipt = s::Receipt {
        version: 1,
        request_id: plan.request.request_id,
        intent_digest: plan.request.intent_digest,
        selection: plan.request.selection.clone(),
        prepared: Some(prepared),
        state: s::ReceiptState::Prepared,
    };
    s::CommandResult {
        version: 1,
        preview: s::Preview { prepared, plan },
        receipt,
        replayed: true,
    }
}
fn lookup(command: &s::CommandResult) -> s::StatusRequest {
    let request = &command.preview.plan.request;
    s::StatusRequest {
        version: 1,
        scope: s::RecoveryScope::from(&request.selection),
        request_id: request.request_id,
        intent_digest: request.intent_digest,
    }
}

#[test]
fn recovery_scope_omits_live_generations_but_binds_physical_profile() {
    let original = command();
    let request = lookup(&original);
    request.validate().unwrap();
    let mut current = original.receipt.selection.clone();
    current.binding_generation += 1;
    current.working_tree = m::RecordDigest::new([99; 32]);
    current.combination = Some(original.preview.plan.combination.clone());
    if let s::CodeSelection::GitSha1Jj045 { operation, .. } = &mut current.code {
        *operation = m::OperationId::new([99; 64]);
    }
    request.scope.check(&current).unwrap();
    for field in 0..4 {
        let mut wrong = request.scope.clone();
        match field {
            0 => wrong.repository = m::RepositoryId::new([99; 16]),
            1 => wrong.physical_workspace = s::PhysicalWorkspaceId(m::RecordDigest::new([99; 32])),
            2 => wrong.jj_workspace = "other".into(),
            _ => wrong.profile = s::CodeProfile::Native,
        }
        assert_eq!(wrong.check(&current), Err(s::Error::IntentMismatch));
    }
    let mut invalid = request;
    invalid.request_id = m::RequestId::new([0; 16]);
    assert_eq!(invalid.validate(), Err(s::Error::Invalid));
}

#[test]
fn public_results_validate_original_receipt_and_lookup_identity() {
    let original = command();
    original.validate().unwrap();
    let status = s::CommandStatus {
        version: 1,
        request: lookup(&original),
        result: Some(original.clone()),
    };
    status.validate().unwrap();
    for field in 0..4 {
        let mut changed = status.clone();
        match field {
            0 => changed.request.intent_digest = m::IntentDigest::new([99; 32]),
            1 => changed.request.request_id = m::RequestId::new([99; 16]),
            2 => {
                changed.result.as_mut().unwrap().receipt.prepared =
                    Some(s::PreparedIdentity(m::RecordDigest::new([99; 32])))
            }
            _ => {
                changed.result.as_mut().unwrap().receipt.state = s::ReceiptState::Published {
                    operation: m::OperationId::new([99; 64]),
                    view: original.preview.plan.view,
                    checkout: s::CheckoutState::Required,
                }
            }
        }
        assert!(changed.validate().is_err());
    }
    let mut absent = status;
    absent.result = None;
    absent.validate().unwrap();
}

#[test]
fn pending_results_reject_terminal_duplicate_foreign_and_oversized_records() {
    let original = command();
    let page = s::PendingCommands {
        version: 1,
        request: s::PendingRequest {
            version: 1,
            scope: lookup(&original).scope,
        },
        commands: vec![original.clone()],
    };
    page.validate().unwrap();
    let mut changed = page.clone();
    changed.commands.push(original.clone());
    assert_eq!(changed.validate(), Err(s::Error::Invalid));
    changed.commands = vec![original.clone(); s::MAX_PENDING_COMMANDS + 1];
    assert_eq!(changed.validate(), Err(s::Error::Limit));
    changed = page.clone();
    changed.request.scope.jj_workspace = "other".into();
    assert_eq!(changed.validate(), Err(s::Error::IntentMismatch));
    for state in [
        s::ReceiptState::Rejected {
            reason: "Canceled".into(),
        },
        s::ReceiptState::Retired {
            operation: original.preview.plan.operation,
            reason: s::Retirement::Undone,
        },
        s::ReceiptState::Published {
            operation: original.preview.plan.operation,
            view: original.preview.plan.view,
            checkout: s::CheckoutState::Verified {
                fingerprint: m::RecordDigest::new([99; 32]),
            },
        },
    ] {
        changed = page.clone();
        changed.commands[0].receipt.state = state;
        assert_eq!(changed.validate(), Err(s::Error::Invalid));
    }
    for state in [
        s::ReceiptState::PublicationUnknown,
        s::ReceiptState::Published {
            operation: original.preview.plan.operation,
            view: original.preview.plan.view,
            checkout: s::CheckoutState::Required,
        },
        s::ReceiptState::Published {
            operation: original.preview.plan.operation,
            view: original.preview.plan.view,
            checkout: s::CheckoutState::Blocked {
                reason: "Editor changed a file".into(),
            },
        },
    ] {
        changed = page.clone();
        changed.commands[0].receipt.state = state;
        changed.validate().unwrap();
    }
}

#[test]
fn cancellation_names_only_original_identity_and_preparation() {
    let original = command();
    let request = s::CancelRequest {
        version: 1,
        request: lookup(&original),
        prepared: original.preview.prepared,
    };
    request.validate_prepared(&original.preview).unwrap();
    let mut changed = request.clone();
    changed.prepared = s::PreparedIdentity(m::RecordDigest::new([99; 32]));
    assert_eq!(
        changed.validate_prepared(&original.preview),
        Err(s::Error::IntentMismatch)
    );
    let encoded = serde_json::to_value(&request).unwrap();
    for field in ["selection", "command", "principal", "path"] {
        let mut changed = encoded.clone();
        changed
            .as_object_mut()
            .unwrap()
            .insert(field.into(), serde_json::json!("untrusted"));
        assert!(serde_json::from_value::<s::CancelRequest>(changed).is_err());
    }
}

#[test]
fn public_schema_excludes_private_execution_and_current_snapshots() {
    let original = command();
    let encoded = serde_json::to_value(&original).unwrap();
    assert_eq!(
        serde_json::from_value::<s::CommandResult>(encoded.clone()).unwrap(),
        original
    );
    for field in ["execution", "retention", "principal", "snapshot"] {
        let mut changed = encoded.clone();
        changed
            .as_object_mut()
            .unwrap()
            .insert(field.into(), serde_json::json!({}));
        assert!(serde_json::from_value::<s::CommandResult>(changed).is_err());
    }
    let schema = serde_json::to_value(schemars::schema_for!(s::CommandStatus)).unwrap();
    let result = &schema["$defs"]["StackCommandResult"];
    assert_eq!(result["additionalProperties"], false);
    assert_eq!(result["properties"].as_object().unwrap().len(), 4);
}

#[test]
fn public_command_response_budget_covers_valid_large_previews() {
    let mut original = command();
    original.preview.plan.checkout.paths = (0..4096)
        .map(|i| format!("{i}/{}", "x".repeat(2048)))
        .collect();
    original.preview.prepared = original.preview.plan.identity().unwrap();
    original.receipt.prepared = Some(original.preview.prepared);
    // Every path and the preparation are valid; aggregate output still exceeds
    // the response budget and cannot be presented as a partial public result.
    original.preview.validate().unwrap();
    assert_eq!(original.validate(), Err(s::Error::Limit));
}
