//! Exact production inputs remain independent of catalog revisions and patch text.
use jj_store_model as m;
use jj_store_service::stack_workspace as stacks;

fn selection() -> stacks::Selection {
    stacks::Selection {
        version: 1,
        repository: m::RepositoryId::new([1; 16]),
        binding_generation: 3,
        physical_workspace: stacks::PhysicalWorkspaceId(m::RecordDigest::new([2; 32])),
        jj_workspace: "default".into(),
        code: stacks::CodeSelection::GitSha1Jj045 {
            operation: m::OperationId::new([3; 64]),
            view: m::ViewId::new([4; 64]),
            working_copy: m::GitSha1Id::new([5; 20]),
        },
        combination: Some(stacks::CombinationSelection {
            id: stacks::CombinationId(m::RequestId::new([6; 16])),
            generation: 7,
        }),
        working_tree: m::RecordDigest::new([8; 32]),
    }
}
fn request() -> stacks::PrepareRequest {
    stacks::PrepareRequest::new(
        m::RequestId::new([9; 16]),
        selection(),
        stacks::Command::Reword {
            commit: stacks::CommitIdentity::GitSha1(m::GitSha1Id::new([10; 20])),
            message: "Retain exact intent".into(),
        },
    )
    .unwrap()
}
#[test]
fn local_stack_intent_binds_all_execution_preconditions() {
    let original = request();
    for field in 0..8 {
        let mut changed = original.clone();
        match field {
            0 => changed.selection.repository = m::RepositoryId::new([11; 16]),
            1 => changed.selection.binding_generation += 1,
            2 => {
                changed.selection.physical_workspace =
                    stacks::PhysicalWorkspaceId(m::RecordDigest::new([12; 32]))
            }
            3 => changed.selection.jj_workspace = "other".into(),
            4 => {
                if let stacks::CodeSelection::GitSha1Jj045 { operation, .. } =
                    &mut changed.selection.code
                {
                    *operation = m::OperationId::new([13; 64]);
                }
            }
            5 => {
                if let stacks::CodeSelection::GitSha1Jj045 { view, .. } =
                    &mut changed.selection.code
                {
                    *view = m::ViewId::new([14; 64]);
                }
            }
            6 => changed.selection.combination.as_mut().unwrap().generation += 1,
            _ => changed.selection.working_tree = m::RecordDigest::new([15; 32]),
        }
        assert_eq!(changed.validate(), Err(stacks::Error::IntentMismatch));
        assert_ne!(changed.intent().unwrap(), original.intent_digest);
    }
    let mut duplicate = original.clone();
    duplicate.request_id = m::RequestId::new([16; 16]);
    duplicate.validate().unwrap();
    assert_eq!(duplicate.intent_digest, original.intent_digest);
}
#[test]
fn local_stack_wire_rejects_foreign_fields_and_identity_profiles() {
    let original = request();
    let encoded = serde_json::to_value(&original).unwrap();
    assert_eq!(
        serde_json::from_value::<stacks::PrepareRequest>(encoded.clone()).unwrap(),
        original
    );
    let mut nested = encoded.clone();
    nested["command"]
        .as_object_mut()
        .unwrap()
        .insert("patch".into(), serde_json::json!("client patch"));
    assert!(serde_json::from_value::<stacks::PrepareRequest>(nested).is_err());
    for key in ["actor", "path", "revision", "patch"] {
        let mut tampered = encoded.clone();
        tampered
            .as_object_mut()
            .unwrap()
            .insert(key.into(), serde_json::json!("untrusted"));
        assert!(serde_json::from_value::<stacks::PrepareRequest>(tampered).is_err());
    }
    let mut changed = original.clone();
    changed.command = stacks::Command::Reword {
        commit: stacks::CommitIdentity::Native(m::CommitId::new([17; 64])),
        message: "Wrong profile".into(),
    };
    assert_eq!(changed.intent(), Err(stacks::Error::UnsupportedProfile));
}
#[test]
fn local_stack_exact_selections_bind_source_parent_and_file_state() {
    let expected = selection();
    let token = stacks::SelectionToken(m::RecordDigest::new([18; 32]));
    let claims = stacks::ExactSelection {
        version: 1,
        token,
        selection: expected.clone(),
        source: stacks::ContentSource::WorkingCopy,
        source_tree: stacks::TreeIdentity::GitSha1(m::GitSha1Id::new([19; 20])).into(),
        parent_tree: stacks::TreeIdentity::GitSha1(m::GitSha1Id::new([20; 20])).into(),
        path: "src/settings.rs".into(),
        file_state: m::RecordDigest::new([21; 32]),
        scope: stacks::SelectionScope::Hunks {
            diff_version: 1,
            hunks: vec![m::RecordDigest::new([22; 32])],
        },
    };
    claims.validate(&expected).unwrap();
    let mut stale = expected.clone();
    stale.working_tree = m::RecordDigest::new([23; 32]);
    assert_eq!(claims.validate(&stale), Err(stacks::Error::Stale));
    for path in ["../a", ".jj/state", "a/.git/config", "/tmp/a", "a\\b", ""] {
        let mut changed = claims.clone();
        changed.path = path.into();
        assert_eq!(changed.validate(&expected), Err(stacks::Error::Invalid));
    }
    let mut changed = claims.clone();
    changed.parent_tree = stacks::TreeIdentity::Native(m::TreeId::new([24; 64])).into();
    assert_eq!(
        changed.validate(&expected),
        Err(stacks::Error::UnsupportedProfile)
    );
    changed = claims.clone();
    changed.scope = stacks::SelectionScope::Hunks {
        diff_version: 1,
        hunks: vec![m::RecordDigest::new([22; 32]); 2],
    };
    assert_eq!(changed.validate(&expected), Err(stacks::Error::Invalid));
    assert_ne!(claims.fingerprint().unwrap(), {
        let mut c = claims.clone();
        c.file_state = m::RecordDigest::new([25; 32]);
        c.fingerprint().unwrap()
    });
}
#[test]
fn local_stack_apply_names_exact_preparation_not_a_new_command() {
    let original = request();
    let prepared = stacks::PreparedIdentity(m::RecordDigest::new([26; 32]));
    let apply = stacks::ApplyRequest {
        version: 1,
        request_id: original.request_id,
        intent_digest: original.intent_digest,
        prepared,
        selection: original.selection.clone(),
    };
    apply.validate(&original, prepared).unwrap();
    let mut changed = apply.clone();
    changed.prepared = stacks::PreparedIdentity(m::RecordDigest::new([27; 32]));
    assert_eq!(
        changed.validate(&original, prepared),
        Err(stacks::Error::Stale)
    );
    changed = apply.clone();
    changed.request_id = m::RequestId::new([28; 16]);
    assert_eq!(
        changed.validate(&original, prepared),
        Err(stacks::Error::IntentMismatch)
    );
    changed = apply.clone();
    changed.selection.binding_generation += 1;
    assert_eq!(
        changed.validate(&original, prepared),
        Err(stacks::Error::Stale)
    );
}
#[test]
fn local_stack_receipt_separates_publication_checkout_and_retirement() {
    let request = request();
    let mut receipt = stacks::Receipt {
        version: 1,
        request_id: request.request_id,
        intent_digest: request.intent_digest,
        selection: request.selection.clone(),
        prepared: Some(stacks::PreparedIdentity(m::RecordDigest::new([29; 32]))),
        state: stacks::ReceiptState::Published {
            operation: m::OperationId::new([30; 64]),
            view: m::ViewId::new([31; 64]),
            checkout: stacks::CheckoutState::Required,
        },
    };
    receipt.validate(&request).unwrap();
    assert!(!receipt.completed());
    assert!(!receipt.may_execute());
    receipt.state = stacks::ReceiptState::Published {
        operation: m::OperationId::new([30; 64]),
        view: m::ViewId::new([31; 64]),
        checkout: stacks::CheckoutState::Verified {
            fingerprint: m::RecordDigest::new([32; 32]),
        },
    };
    assert!(receipt.completed());
    receipt.state = stacks::ReceiptState::Retired {
        operation: m::OperationId::new([30; 64]),
        reason: stacks::Retirement::Undone,
    };
    assert!(!receipt.may_execute());
    assert!(!receipt.completed());
    let mut wrong = request.clone();
    wrong.request_id = m::RequestId::new([33; 16]);
    assert_eq!(receipt.validate(&wrong), Err(stacks::Error::IntentMismatch));
}

#[test]
fn local_stack_preparation_identity_binds_exact_published_output() {
    let request = request();
    let plan = stacks::PreparedPlan {
        version: 1,
        request: request.clone(),
        operation: m::OperationId::new([34; 64]),
        view: m::ViewId::new([35; 64]),
        checkout: stacks::CheckoutEffect {
            before: stacks::TreeIdentity::GitSha1(m::GitSha1Id::new([36; 20])).into(),
            after: stacks::TreeIdentity::GitSha1(m::GitSha1Id::new([37; 20])).into(),
            paths: vec!["src/settings.rs".into()],
        },
        combination: request.selection.combination.clone().unwrap(),
        rewrites: vec![stacks::Rewrite {
            change: m::ChangeId::new([38; 16]),
            before: vec![stacks::CommitIdentity::GitSha1(m::GitSha1Id::new([10; 20]))],
            after: vec![stacks::CommitIdentity::GitSha1(m::GitSha1Id::new([39; 20]))],
        }],
        bookmarks: vec![],
        conflicts: vec![],
    };
    let preview = stacks::Preview {
        prepared: plan.identity().unwrap(),
        plan,
    };
    preview.validate().unwrap();
    let mut changed = preview.clone();
    changed.plan.operation = m::OperationId::new([40; 64]);
    assert_eq!(changed.validate(), Err(stacks::Error::IntentMismatch));
    let mut receipt = stacks::Receipt {
        version: 1,
        request_id: request.request_id,
        intent_digest: request.intent_digest,
        selection: request.selection.clone(),
        prepared: Some(preview.prepared),
        state: stacks::ReceiptState::Published {
            operation: preview.plan.operation,
            view: preview.plan.view,
            checkout: stacks::CheckoutState::Required,
        },
    };
    receipt.validate_prepared(&preview).unwrap();
    receipt.state = stacks::ReceiptState::Published {
        operation: m::OperationId::new([40; 64]),
        view: preview.plan.view,
        checkout: stacks::CheckoutState::Required,
    };
    assert_eq!(
        receipt.validate_prepared(&preview),
        Err(stacks::Error::IntentMismatch)
    );
}
#[test]
fn local_stack_setup_is_explicit_and_commands_are_bounded() {
    let mut before_setup = selection();
    before_setup.combination = None;
    assert_eq!(
        stacks::PrepareRequest::new(
            m::RequestId::new([41; 16]),
            before_setup.clone(),
            request().command
        ),
        Err(stacks::Error::Invalid)
    );
    let setup = stacks::Command::Setup {
        target: stacks::BookmarkTarget {
            name: "trunk".into(),
            remote: Some("origin".into()),
            head: stacks::CommitIdentity::GitSha1(m::GitSha1Id::new([42; 20])),
        },
        name: "Local workspace".into(),
    };
    stacks::PrepareRequest::new(m::RequestId::new([43; 16]), before_setup, setup.clone()).unwrap();
    assert_eq!(
        stacks::PrepareRequest::new(m::RequestId::new([44; 16]), selection(), setup),
        Err(stacks::Error::Invalid)
    );
    let mut request = request();
    request.command = stacks::Command::Transfer {
        selections: vec![stacks::SelectionToken(m::RecordDigest::new([45; 32])); 257],
        destination: stacks::ContentSource::WorkingCopy,
    };
    assert_eq!(request.intent(), Err(stacks::Error::Limit));
    request.command = stacks::Command::Reword {
        commit: stacks::CommitIdentity::GitSha1(m::GitSha1Id::new([10; 20])),
        message: "x".repeat(65 * 1024),
    };
    assert_eq!(request.intent(), Err(stacks::Error::Limit));
    request.version = 2;
    assert_eq!(request.intent(), Err(stacks::Error::Invalid));
}
#[test]
fn local_stack_wire_schema_keeps_separate_code_profiles() {
    let schema = schemars::schema_for!(stacks::PrepareRequest);
    let value = serde_json::to_value(schema).unwrap();
    assert_eq!(value["additionalProperties"], false);
    let definitions = value["$defs"].as_object().unwrap();
    assert!(definitions.contains_key("StackWorkspaceSelection"));
    assert!(definitions.contains_key("StackCodeSelection"));
    let selection = &definitions["StackWorkspaceSelection"];
    assert_eq!(selection["additionalProperties"], false);
    assert!(selection["properties"].get("revision").is_none());
    assert!(
        definitions["StackWorkspaceCommand"]
            .to_string()
            .contains("delete_bookmark")
    );
    assert!(
        definitions["StackWorkspaceCommand"]
            .to_string()
            .contains("abandon_segment")
    );
}

#[test]
fn local_stack_native_tree_selection_retains_ordered_conflict_terms() {
    let mut native = selection();
    native.code = stacks::CodeSelection::Native {
        operation: m::OperationId::new([46; 64]),
        view: m::ViewId::new([47; 64]),
        working_copy: m::CommitId::new([48; 64]),
    };
    let mut terms = stacks::TreeSelection {
        terms: vec![
            stacks::TreeIdentity::Native(m::TreeId::new([49; 64])),
            stacks::TreeIdentity::Native(m::TreeId::new([50; 64])),
            stacks::TreeIdentity::Native(m::TreeId::new([51; 64])),
        ],
    };
    terms.validate(&native.code).unwrap();
    let first = serde_json::to_string(&terms).unwrap();
    terms.terms.swap(0, 2);
    assert_ne!(first, serde_json::to_string(&terms).unwrap());
    terms.terms.pop();
    assert_eq!(terms.validate(&native.code), Err(stacks::Error::Invalid));
    let command = stacks::Command::Reword {
        commit: stacks::CommitIdentity::Native(m::CommitId::new([52; 64])),
        message: "Native profile".into(),
    };
    stacks::PrepareRequest::new(m::RequestId::new([53; 16]), native, command)
        .unwrap()
        .validate()
        .unwrap();
}
