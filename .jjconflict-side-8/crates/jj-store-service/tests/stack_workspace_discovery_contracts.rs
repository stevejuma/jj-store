use jj_store_model as m;
use jj_store_service::stack_workspace as s;

fn selected() -> s::Selection {
    s::Selection {
        version: 1,
        repository: m::RepositoryId::new([1; 16]),
        binding_generation: 7,
        physical_workspace: s::PhysicalWorkspaceId(m::RecordDigest::new([2; 32])),
        jj_workspace: "default".into(),
        code: s::CodeSelection::GitSha1Jj045 {
            operation: m::OperationId::new([3; 64]),
            view: m::ViewId::new([4; 64]),
            working_copy: m::GitSha1Id::new([5; 20]),
        },
        combination: None,
        working_tree: m::RecordDigest::new([6; 32]),
    }
}
fn combination() -> s::CombinationSummary {
    s::CombinationSummary {
        selection: s::CombinationSelection {
            id: s::CombinationId(m::RequestId::new([8; 16])),
            generation: 1,
        },
        name: "Daily work".into(),
        target: s::BookmarkTarget {
            name: "main".into(),
            remote: None,
            head: s::CommitIdentity::GitSha1(m::GitSha1Id::new([9; 20])),
        },
        merge: s::CommitIdentity::GitSha1(m::GitSha1Id::new([10; 20])),
        code: selected().code,
        stacks: vec![s::AppliedStackSummary {
            id: s::StackId(m::RequestId::new([11; 16])),
            bookmark: "feature/ui".into(),
            head: s::CommitIdentity::GitSha1(m::GitSha1Id::new([12; 20])),
            base: s::CommitIdentity::GitSha1(m::GitSha1Id::new([9; 20])),
        }],
    }
}
fn ready() -> s::WorkspaceState {
    let combination = combination();
    let mut selection = selected();
    selection.combination = Some(combination.selection.clone());
    s::WorkspaceState {
        version: 1,
        state: s::WorkspaceReadiness::Ready,
        catalog: Some(s::CombinationCatalog {
            version: 1,
            scope: s::RecoveryScope::from(&selection),
            revision: 1,
            active: Some(combination.selection.id),
            combinations: vec![combination],
            pending: false,
        }),
        selection,
    }
}
fn request() -> s::WorkspaceRequest {
    s::WorkspaceRequest {
        version: 1,
        repository: selected().repository,
        binding_generation: 7,
        operation: m::OperationId::new([3; 64]),
    }
}

#[test]
fn discovery_preserves_exact_current_input_separately_from_historical_outcomes() {
    let state = ready();
    state.validate_request(&request()).unwrap();
    for field in 0..3 {
        let mut wrong = request();
        match field {
            0 => wrong.repository = m::RepositoryId::new([99; 16]),
            1 => wrong.binding_generation += 1,
            _ => wrong.operation = m::OperationId::new([99; 64]),
        }
        assert_eq!(state.validate_request(&wrong), Err(s::Error::Stale));
    }
    let mut invalid = request();
    invalid.binding_generation = 0;
    assert_eq!(invalid.validate(), Err(s::Error::Invalid));
    let mut wrong = ready();
    wrong.catalog.as_mut().unwrap().scope.jj_workspace = "other".into();
    assert_eq!(wrong.validate(), Err(s::Error::IntentMismatch));
}

#[test]
fn readiness_distinguishes_absence_stale_code_and_original_pending_coordination() {
    let mut state = s::WorkspaceState {
        version: 1,
        state: s::WorkspaceReadiness::Absent,
        selection: selected(),
        catalog: None,
    };
    state.validate().unwrap();
    // Original preparation can remain pending before a catalog was staged.
    state.state = s::WorkspaceReadiness::Pending;
    state.validate().unwrap();
    state.state = s::WorkspaceReadiness::Ready;
    assert_eq!(state.validate(), Err(s::Error::Invalid));
    state = ready();
    state.selection.code = s::CodeSelection::GitSha1Jj045 {
        operation: m::OperationId::new([99; 64]),
        view: m::ViewId::new([99; 64]),
        working_copy: m::GitSha1Id::new([99; 20]),
    };
    assert_eq!(state.validate(), Err(s::Error::Invalid));
    state.state = s::WorkspaceReadiness::Stale;
    state.validate().unwrap();
    state.catalog.as_mut().unwrap().pending = true;
    assert_eq!(state.validate(), Err(s::Error::Invalid));
    state.state = s::WorkspaceReadiness::Pending;
    state.validate().unwrap();
    state = ready();
    state.state = s::WorkspaceReadiness::Absent;
    assert_eq!(state.validate(), Err(s::Error::Invalid));
    let catalog = state.catalog.as_mut().unwrap();
    catalog.combinations.clear();
    catalog.active = None;
    catalog.revision = 0;
    state.selection.combination = None;
    state.validate().unwrap();
    // Empty selected state may retain a later catalog revision; discovery does
    // not reset durable generation history or authorize first-use setup.
    state.catalog.as_mut().unwrap().revision = 3;
    state.validate().unwrap();
    state.catalog.as_mut().unwrap().pending = true;
    assert_eq!(state.validate(), Err(s::Error::Invalid));
    state.state = s::WorkspaceReadiness::Pending;
    state.validate().unwrap();
}

#[test]
fn native_discovery_contract_keeps_profile_identities_without_qualifying_provider() {
    let mut state = ready();
    let code = s::CodeSelection::Native {
        operation: request().operation,
        view: m::ViewId::new([4; 64]),
        working_copy: m::CommitId::new([5; 64]),
    };
    state.selection.code = code.clone();
    let catalog = state.catalog.as_mut().unwrap();
    catalog.scope.profile = s::CodeProfile::Native;
    let combination = &mut catalog.combinations[0];
    combination.code = code;
    combination.target.head = s::CommitIdentity::Native(m::CommitId::new([9; 64]));
    combination.merge = s::CommitIdentity::Native(m::CommitId::new([10; 64]));
    combination.stacks[0].head = s::CommitIdentity::Native(m::CommitId::new([12; 64]));
    combination.stacks[0].base = combination.target.head;
    state.validate_request(&request()).unwrap();
    state.catalog.as_mut().unwrap().combinations[0].stacks[0].base =
        s::CommitIdentity::GitSha1(m::GitSha1Id::new([9; 20]));
    assert_eq!(state.validate(), Err(s::Error::UnsupportedProfile));
}

#[test]
fn catalogs_bind_profiles_active_generation_membership_uniqueness_and_limits() {
    let original = ready();
    for field in 0..7 {
        let mut state = original.clone();
        let catalog = state.catalog.as_mut().unwrap();
        match field {
            0 => catalog.active = Some(s::CombinationId(m::RequestId::new([99; 16]))),
            1 => catalog.combinations[0].selection.generation = 0,
            2 => catalog.combinations.push(catalog.combinations[0].clone()),
            3 => {
                catalog.combinations[0].merge =
                    s::CommitIdentity::Native(m::CommitId::new([99; 64]))
            }
            4 => {
                let duplicate = catalog.combinations[0].stacks[0].clone();
                catalog.combinations[0].stacks.push(duplicate);
            }
            5 => catalog.combinations = vec![combination(); s::MAX_COMBINATIONS + 1],
            _ => {
                catalog.combinations[0].stacks =
                    vec![combination().stacks[0].clone(); s::MAX_COMBINATION_STACKS + 1]
            }
        }
        assert!(state.validate().is_err());
    }
    let mut wrong = original;
    wrong.selection.combination.as_mut().unwrap().generation += 1;
    assert_eq!(wrong.validate(), Err(s::Error::Invalid));
}

#[test]
fn discovery_wire_excludes_private_retention_and_proposed_state() {
    let original = ready();
    let encoded = serde_json::to_value(&original).unwrap();
    assert_eq!(
        serde_json::from_value::<s::WorkspaceState>(encoded.clone()).unwrap(),
        original
    );
    for field in ["retention", "principal", "after", "execution", "path"] {
        let mut changed = encoded.clone();
        changed["catalog"]["combinations"][0]
            .as_object_mut()
            .unwrap()
            .insert(field.into(), serde_json::json!({}));
        assert!(serde_json::from_value::<s::WorkspaceState>(changed).is_err());
    }
    let schema = serde_json::to_value(schemars::schema_for!(s::WorkspaceState)).unwrap();
    assert_eq!(schema["additionalProperties"], false);
    assert_eq!(
        schema["$defs"]["StackCombinationSummary"]["additionalProperties"],
        false
    );
    assert_eq!(
        schema["$defs"]["StackCombinationSummary"]["properties"]
            .as_object()
            .unwrap()
            .len(),
        6
    );
    assert_eq!(
        schema["$defs"]["StackCombinationSummary"]["properties"]["stacks"]["maxItems"],
        4096
    );
    assert_eq!(
        schema["$defs"]["StackCombinationCatalog"]["properties"]["combinations"]["maxItems"],
        64
    );
}

#[test]
fn complete_catalog_response_budget_rejects_valid_oversized_membership() {
    let mut state = ready();
    let catalog = state.catalog.as_mut().unwrap();
    catalog.combinations.clear();
    for n in 1..=4 {
        let mut summary = combination();
        summary.selection.id = s::CombinationId(m::RequestId::new([n; 16]));
        summary.name = format!("Workspace {n}");
        summary.stacks = (0u32..4096)
            .map(|i| {
                let mut id = [n; 16];
                id[..4].copy_from_slice(&i.to_be_bytes());
                s::AppliedStackSummary {
                    id: s::StackId(m::RequestId::new(id)),
                    bookmark: "x".repeat(1024),
                    ..combination().stacks[0].clone()
                }
            })
            .collect();
        catalog.combinations.push(summary);
    }
    catalog.active = Some(catalog.combinations[0].selection.id);
    state.selection.combination = Some(catalog.combinations[0].selection.clone());
    assert_eq!(state.validate(), Err(s::Error::Limit));
}
