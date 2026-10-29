use super::*;
use jj_lib::{config::StackedConfig, settings::UserSettings};
use jj_store_jj::{
    local_evidence::capture_operation_archive, local_execution::Reword,
    local_metadata::StackRetentionOutcome,
};
use jj_store_server::stack_combinations::{self as c, CombinationStore};

#[tokio::test]
async fn actual_reword_checkout_requires_combination_before_final_acknowledgement() {
    let folder = Fixture::new().await;
    fs::write(folder.root().join("file.txt"), b"original\n").unwrap();
    let current = folder.witness().await;
    let observer =
        jj_store_service::working_tree::Observer::new(local_browser::Executor::new(1).unwrap(), 1)
            .unwrap();
    let physical = jj_store_service::working_tree::scan(
        &folder.root(),
        current.baseline.repository,
        1,
        Default::default(),
        |_| Ok(()),
    )
    .unwrap()
    .fingerprint;
    let scope = Scope {
        installation: [1; 16],
        repository: current.baseline.repository,
        physical_workspace: s::PhysicalWorkspaceId(current.physical_workspace),
        jj_workspace: "default".into(),
        profile: Profile::GitSha1Jj045,
    };
    let source_code = s::CodeSelection::GitSha1Jj045 {
        operation: current.baseline.operation,
        view: current.baseline.view,
        working_copy: current.baseline.commit,
    };
    let selection = s::Selection {
        version: 1,
        repository: scope.repository,
        binding_generation: 1,
        physical_workspace: scope.physical_workspace,
        jj_workspace: "default".into(),
        code: source_code.clone(),
        combination: None,
        working_tree: physical,
    };
    let combination = s::CombinationSelection {
        id: s::CombinationId(m::RequestId::new([2; 16])),
        generation: 1,
    };
    let target = s::BookmarkTarget {
        name: "main".into(),
        remote: None,
        head: s::CommitIdentity::GitSha1(m::GitSha1Id::new([0; 20])),
    };
    let initial = s::PreparedPlan {
        version: 1,
        request: s::PrepareRequest::new(
            m::RequestId::new([9; 16]),
            selection.clone(),
            s::Command::Setup {
                target: target.clone(),
                name: "Existing fixture".into(),
            },
        )
        .unwrap(),
        operation: current.baseline.operation,
        view: current.baseline.view,
        checkout: s::CheckoutEffect {
            before: s::TreeIdentity::GitSha1(current.baseline.trees[0]).into(),
            after: s::TreeIdentity::GitSha1(current.baseline.trees[0]).into(),
            paths: vec![],
        },
        combination: combination.clone(),
        rewrites: vec![],
        bookmarks: vec![],
        conflicts: vec![],
    };
    let initial = s::Preview {
        prepared: initial.identity().unwrap(),
        plan: initial,
    };
    let coordinator = Coordinator::new(1).unwrap();
    let lease = coordinator
        .acquire(folder.access(), folder.root(), current.clone())
        .await
        .unwrap();
    let (lease, witness) = lease.recovery_witness().await.unwrap();
    drop(lease);
    let settings =
        UserSettings::from_config_and_home_dir(StackedConfig::with_defaults(), None).unwrap();
    let source = capture_operation_archive(
        &settings,
        &folder.root().join(".jj/repo"),
        scope.repository,
        current.baseline.operation,
        Default::default(),
    )
    .await
    .unwrap();
    let metadata = MetadataStore::open(&folder.root().join(".git"), Limits::default()).unwrap();
    let StackRetentionOutcome::Retained(retained) = metadata
        .retain_stack_execution(&preparation_key(actor(), &initial), &source)
        .unwrap()
    else {
        panic!()
    };
    let private = tempfile::tempdir().unwrap();
    let journal =
        StackCommandJournal::create(&private.path().join("journal"), scope.clone()).unwrap();
    journal
        .record_local_prepared(
            actor(),
            &initial,
            &LocalExecutionV1::from_retention(actor(), &initial, witness.clone(), &retained)
                .unwrap(),
        )
        .unwrap();
    let store =
        CombinationStore::create(&private.path().join("combinations"), scope, actor()).unwrap();
    let before = c::Snapshot {
        version: 1,
        revision: 1,
        active: Some(combination.id),
        combinations: vec![c::Combination {
            selection: combination.clone(),
            name: "Existing fixture".into(),
            target,
            immutable_heads: vec![m::GitSha1Id::new([0; 20])],
            merge: m::GitSha1Id::new([0; 20]),
            code: source_code,
            stacks: vec![],
            retention: c::RetainedCode {
                key: retained.key.clone(),
                commit: retained.commit,
                archive: source.id().unwrap(),
            },
        }],
    };
    store
        .stage(actor(), 0, &initial, &before, &retained)
        .unwrap();
    // Historical metadata bootstrap only, to provide exact existing source
    // authority. This does not qualify the Setup engine or physical switching.
    let path = private.path().join("journal/state.json");
    let mut bytes: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    bytes["records"][0]["receipt"]["state"] = serde_json::to_value(s::ReceiptState::Published {
        operation: current.baseline.operation,
        view: current.baseline.view,
        checkout: s::CheckoutState::Verified {
            fingerprint: physical,
        },
    })
    .unwrap();
    private_write(&path, &serde_json::to_vec(&bytes).unwrap());
    let initial_apply = s::ApplyRequest {
        version: 1,
        request_id: initial.plan.request.request_id,
        intent_digest: initial.plan.request.intent_digest,
        prepared: initial.prepared,
        selection: selection.clone(),
    };
    store.finish(&journal, actor(), &initial_apply).unwrap();
    let mut selected = selection;
    selected.combination = Some(combination.clone());
    let request = s::PrepareRequest::new(
        m::RequestId::new([10; 16]),
        selected,
        s::Command::Reword {
            commit: s::CommitIdentity::GitSha1(current.baseline.commit),
            message: "Reworded through the coordinated join".into(),
        },
    )
    .unwrap();
    let lease = coordinator
        .acquire(folder.access(), folder.root(), current.clone())
        .await
        .unwrap();
    let (lease, candidate) = lease
        .prepare_reword(Reword {
            principal: actor(),
            commit: current.baseline.commit,
            message: "Reworded through the coordinated join".into(),
            immutable_heads: vec![m::GitSha1Id::new([0; 20])],
        })
        .await
        .unwrap();
    let capsule = candidate.history.capsule().unwrap();
    let plan = s::PreparedPlan {
        version: 1,
        request,
        operation: capsule.operation_id,
        view: capsule.view_id,
        checkout: initial.plan.checkout.clone(),
        combination: s::CombinationSelection {
            generation: 2,
            ..combination
        },
        rewrites: vec![],
        bookmarks: vec![],
        conflicts: vec![],
    };
    let preview = s::Preview {
        prepared: plan.identity().unwrap(),
        plan,
    };
    let key = preparation_key(actor(), &preview);
    let (lease, retained) = lease
        .retain_history(witness.clone(), key.clone(), candidate.history)
        .await
        .unwrap();
    let StackRetentionOutcome::Retained(retained) = retained else {
        panic!()
    };
    let execution =
        LocalExecutionV1::from_retention(actor(), &preview, witness.clone(), &retained).unwrap();
    journal
        .record_local_prepared(actor(), &preview, &execution)
        .unwrap();
    let apply = s::ApplyRequest {
        version: 1,
        request_id: preview.plan.request.request_id,
        intent_digest: preview.plan.request.intent_digest,
        prepared: preview.prepared,
        selection: preview.plan.request.selection.clone(),
    };
    let mut after = before;
    after.revision = 2;
    after.combinations[0].selection = preview.plan.combination.clone();
    after.combinations[0].code = s::CodeSelection::GitSha1Jj045 {
        operation: capsule.operation_id,
        view: capsule.view_id,
        working_copy: capsule.view.wc_commit_ids["default"],
    };
    after.combinations[0].retention = c::RetainedCode {
        key: key.clone(),
        commit: retained.commit,
        archive: execution.archive,
    };
    journal
        .save_local_combination(&store, actor(), &apply, 1, &after, &retained)
        .unwrap();
    drop(lease);
    let recovery = coordinator
        .resume_publication(folder.access(), folder.root(), witness, key)
        .await
        .unwrap()
        .restore_extras()
        .await
        .unwrap()
        .restore_indexes()
        .await
        .unwrap();
    journal
        .transition(
            actor(),
            &apply,
            &s::ReceiptState::Prepared,
            &s::ReceiptState::PublicationUnknown,
        )
        .unwrap();
    let (recovery, head) = recovery.publish_head().await.unwrap();
    journal
        .acknowledge_local_publication(actor(), &apply, &head)
        .unwrap();
    let guard = observer.physical_guard(folder.access(), folder.root());
    let (recovery, saved) = recovery
        .prepare_unchanged_checkout(&head, guard.clone(), physical)
        .await
        .unwrap();
    journal
        .save_local_checkout(actor(), &apply, &saved)
        .unwrap();
    let (recovery, proof) = recovery
        .complete_unchanged_checkout(saved, guard)
        .await
        .unwrap();
    assert_eq!(
        journal.acknowledge_local_checkout(actor(), &apply, &proof),
        Err(Error::InvalidTransition)
    );
    journal
        .save_local_unchanged_completion(actor(), &apply, &proof)
        .unwrap();
    assert!(
        !journal
            .status(actor(), apply.request_id, apply.intent_digest)
            .unwrap()
            .unwrap()
            .receipt
            .completed()
    );
    assert!(
        journal
            .finish_local_unchanged_combination(&store, actor(), &apply, &proof)
            .unwrap()
            .receipt
            .completed()
    );
    assert_eq!(store.snapshot().unwrap(), after);
    assert!(
        journal
            .finish_local_unchanged_combination(&store, actor(), &apply, &proof)
            .unwrap()
            .replayed
    );
    assert_eq!(
        fs::read(folder.root().join("file.txt")).unwrap(),
        b"original\n"
    );
    drop(recovery);
    assert_eq!(
        fixture::stock(
            &folder.root(),
            &["log", "--no-graph", "-r", "@", "-T", "description"]
        ),
        "Reworded through the coordinated join"
    );
}
