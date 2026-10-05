use super::*;
#[tokio::test]
async fn publication_requires_saved_original_plan_and_reopen_resolves_it() {
    let f = Input::new().await;
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("journal");
    let plans = temp.path().join("plans");
    let journal = f.journal(&path);
    let store = StackCheckoutPlans::create(&plans, f.scope.clone()).unwrap();
    let original = fs::read(path.join("state.json")).unwrap();
    assert_eq!(json(&path)["version"], 2);
    assert!(
        json(&path)["records"][0]["execution"]
            .get("tree_checkout")
            .is_none()
    );
    assert_eq!(
        journal.transition(
            actor(),
            &f.apply(),
            &s::ReceiptState::Prepared,
            &s::ReceiptState::PublicationUnknown
        ),
        Err(Error::InvalidTransition)
    );
    assert_eq!(fs::read(path.join("state.json")).unwrap(), original);
    let before = f.folder.state();
    let saved = journal
        .save_local_tree_checkout(&store, actor(), &f.apply(), &f.plan)
        .unwrap();
    assert_eq!(saved.receipt.state, s::ReceiptState::Prepared);
    assert_eq!(json(&path)["version"], 4);
    let encoded = fs::read(path.join("state.json")).unwrap();
    assert!(encoded.len() < original.len() + 256); // reference, no body
    let reopened = StackCommandJournal::open(&path, f.scope.clone()).unwrap();
    assert_eq!(
        reopened
            .load_local_tree_checkout(&store, actor(), &f.apply())
            .unwrap(),
        f.plan
    );
    assert!(
        reopened
            .save_local_tree_checkout(&store, actor(), &f.apply(), &f.plan)
            .unwrap()
            .replayed
    );
    assert_eq!(fs::read(path.join("state.json")).unwrap(), encoded);
    assert_eq!(
        reopened.transition(
            actor(),
            &f.apply(),
            &s::ReceiptState::Prepared,
            &s::ReceiptState::PublicationUnknown,
        ),
        Err(Error::InvalidTransition)
    ); // complete inventory is also required
    assert_eq!(reopened.pending(actor()).unwrap().len(), 1);
    assert!(
        reopened
            .save_local_tree_checkout(&store, actor(), &f.apply(), &f.plan)
            .unwrap()
            .replayed
    );
    assert_eq!(f.folder.state(), before); // metadata admission never publishes
    fs::remove_file(plans.join("state.json")).unwrap();
    assert!(
        reopened
            .load_local_tree_checkout(&store, actor(), &f.apply())
            .is_err()
    );
    assert!(!plans.join("state.json").exists());
}
#[tokio::test]
async fn original_auth_and_linkage_precede_any_store_access() {
    let f = Input::new().await;
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("journal");
    let plans = temp.path().join("plans");
    let journal = f.journal(&path);
    let store = StackCheckoutPlans::create(&plans, f.scope.clone()).unwrap();
    let original = fs::read(plans.join("state.json")).unwrap();
    assert_eq!(
        journal.save_local_tree_checkout(&store, m::PrincipalId::new([8; 16]), &f.apply(), &f.plan),
        Err(Error::IntentMismatch)
    );
    for field in [
        "physical",
        "archive",
        "recovery",
        "before_commit",
        "before_tree",
        "after_tree",
        "view",
        "retained",
    ] {
        let mut value = serde_json::to_value(&f.plan).unwrap();
        value[field] = match field {
            "before_commit" | "before_tree" | "after_tree" | "retained" => {
                serde_json::to_value(m::GitSha1Id::new([8; 20])).unwrap()
            }
            "view" => serde_json::to_value(m::ViewId::new([8; 64])).unwrap(),
            _ => serde_json::to_value(m::RecordDigest::new([8; 32])).unwrap(),
        };
        let plan = serde_json::from_value(value).unwrap();
        assert!(
            journal
                .save_local_tree_checkout(&store, actor(), &f.apply(), &plan)
                .is_err(),
            "{field}"
        );
        assert_eq!(fs::read(plans.join("state.json")).unwrap(), original);
    }
    let mut wrong = f.apply();
    wrong.prepared = s::PreparedIdentity(m::RecordDigest::new([8; 32]));
    assert_eq!(
        journal.save_local_tree_checkout(&store, actor(), &wrong, &f.plan),
        Err(Error::IntentMismatch)
    );
    fs::remove_file(plans.join("state.json")).unwrap();
    assert_eq!(
        journal.save_local_tree_checkout(&store, m::PrincipalId::new([8; 16]), &f.apply(), &f.plan),
        Err(Error::IntentMismatch)
    );
}
#[tokio::test]
async fn terminal_request_never_acquires_new_plan_and_legacy_bytes_stay_unchanged() {
    let f = Input::new().await;
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("journal");
    let plans = temp.path().join("plans");
    let journal = f.journal(&path);
    let store = StackCheckoutPlans::create(&plans, f.scope.clone()).unwrap();
    journal
        .transition(
            actor(),
            &f.apply(),
            &s::ReceiptState::Prepared,
            &s::ReceiptState::Rejected {
                reason: "Cancelled".into(),
            },
        )
        .unwrap();
    let before = fs::read(path.join("state.json")).unwrap();
    let plan_before = fs::read(plans.join("state.json")).unwrap();
    assert_eq!(
        journal.save_local_tree_checkout(&store, actor(), &f.apply(), &f.plan),
        Err(Error::InvalidTransition)
    );
    StackCommandJournal::open(&path, f.scope.clone())
        .unwrap()
        .status(actor(), f.apply().request_id, f.apply().intent_digest)
        .unwrap();
    assert_eq!(fs::read(path.join("state.json")).unwrap(), before);
    assert_eq!(fs::read(plans.join("state.json")).unwrap(), plan_before);
}
#[tokio::test]
async fn saved_input_is_provider_revalidated_under_original_lease() {
    let f = Input::new().await;
    let temp = tempfile::tempdir().unwrap();
    let journal = f.journal(&temp.path().join("journal"));
    let store = StackCheckoutPlans::create(&temp.path().join("plans"), f.scope.clone()).unwrap();
    journal
        .save_local_tree_checkout(&store, actor(), &f.apply(), &f.plan)
        .unwrap();
    let saved = journal
        .load_local_tree_checkout(&store, actor(), &f.apply())
        .unwrap();
    let recovery = Coordinator::new(1)
        .unwrap()
        .restore_history(
            f.folder.access(),
            f.folder.root(),
            f.execution.witness.clone(),
            f.execution.key.clone(),
        )
        .await
        .unwrap();
    let before = f.folder.state();
    let (recovery, checked) = recovery
        .revalidate_tree_checkout(
            saved,
            Arc::new(Files {
                root: f.folder.root(),
            }),
        )
        .await
        .unwrap();
    assert_eq!(checked, f.plan);
    drop(recovery);
    assert_eq!(f.folder.state(), before);
}
#[tokio::test]
async fn malformed_reference_and_missing_body_never_repair_from_supplied_plan() {
    let f = Input::new().await;
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("journal");
    let plans = temp.path().join("plans");
    let journal = f.journal(&path);
    let store = StackCheckoutPlans::create(&plans, f.scope.clone()).unwrap();
    journal
        .save_local_tree_checkout(&store, actor(), &f.apply(), &f.plan)
        .unwrap();
    let original = fs::read(path.join("state.json")).unwrap();
    for field in ["encoded_bytes", "id"] {
        let mut value = json(&path);
        value["records"][0]["execution"]["tree_checkout"][field] = if field == "encoded_bytes" {
            1.into()
        } else {
            serde_json::to_value(m::RecordDigest::new([19; 32])).unwrap()
        };
        private_write(
            &path.join("state.json"),
            &serde_json::to_vec(&value).unwrap(),
        );
        let reopened = StackCommandJournal::open(&path, f.scope.clone()).unwrap();
        assert_eq!(
            reopened.load_local_tree_checkout(&store, actor(), &f.apply()),
            Err(Error::IntentMismatch)
        );
        assert_eq!(
            reopened.save_local_tree_checkout(&store, actor(), &f.apply(), &f.plan),
            Err(Error::IntentMismatch)
        );
        private_write(&path.join("state.json"), &original);
    }
    // Produce the canonical typed field order by editing original bytes.
    let empty = format!(
        "{{\"version\":1,\"scope\":{},\"plans\":[]}}",
        serde_json::to_string(&f.scope).unwrap()
    );
    private_write(&plans.join("state.json"), empty.as_bytes());
    assert_eq!(
        journal.save_local_tree_checkout(&store, actor(), &f.apply(), &f.plan),
        Err(Error::Unavailable)
    );
    assert_eq!(
        fs::read(plans.join("state.json")).unwrap(),
        empty.as_bytes()
    );
    assert_eq!(fs::read(path.join("state.json")).unwrap(), original);
}
#[tokio::test]
async fn acknowledged_retention_replay_preserves_saved_plan_reference() {
    let f = Input::new().await;
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("journal");
    let journal = f.journal(&path);
    let store = StackCheckoutPlans::create(&temp.path().join("plans"), f.scope.clone()).unwrap();
    journal
        .save_local_tree_checkout(&store, actor(), &f.apply(), &f.plan)
        .unwrap();
    let original = fs::read(path.join("state.json")).unwrap();
    let retained = MetadataStore::open(&f.folder.root().join(".git"), Default::default())
        .unwrap()
        .read_stack_prepared(&f.execution.key)
        .unwrap()
        .unwrap();
    assert!(
        journal
            .acknowledge_local_retention(actor(), &f.preview, &retained)
            .unwrap()
            .replayed
    );
    assert_eq!(fs::read(path.join("state.json")).unwrap(), original);
}
#[tokio::test]
async fn plan_contention_releases_journal_and_cannot_bypass_original_authentication() {
    let f = Input::new().await;
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("journal");
    let plans = temp.path().join("plans");
    let journal = f.journal(&path);
    let store = StackCheckoutPlans::create(&plans, f.scope.clone()).unwrap();
    let before = fs::read(path.join("state.json")).unwrap();
    let lock = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(plans.join("checkout-plans.lock"))
        .unwrap();
    lock.try_lock().unwrap();
    assert_eq!(
        journal.save_local_tree_checkout(
            &store,
            m::PrincipalId::new([99; 16]),
            &f.apply(),
            &f.plan
        ),
        Err(Error::IntentMismatch)
    );
    assert_eq!(
        journal.save_local_tree_checkout(&store, actor(), &f.apply(), &f.plan),
        Err(Error::Unavailable)
    );
    assert_eq!(fs::read(path.join("state.json")).unwrap(), before);
    assert!(
        journal
            .status(actor(), f.apply().request_id, f.apply().intent_digest)
            .unwrap()
            .is_some()
    );
    drop(lock);
    journal
        .save_local_tree_checkout(&store, actor(), &f.apply(), &f.plan)
        .unwrap();
    let execution = journal
        .status(actor(), f.apply().request_id, f.apply().intent_digest)
        .unwrap()
        .unwrap()
        .execution
        .unwrap();
    let fresh = StackCommandJournal::create(&temp.path().join("fresh"), f.scope.clone()).unwrap();
    assert_eq!(
        fresh.record_local_prepared(actor(), &f.preview, &execution),
        Err(Error::InvalidTransition)
    );
    let mut value = json(&path);
    value["records"][0]["execution"]["checkout"] = serde_json::json!({"id":m::RecordDigest::new([1;32]), "witness":{
        "version":1, "key":execution.key, "recovery":execution.witness_id, "operation":f.preview.plan.operation,
        "view":f.preview.plan.view, "physical":f.plan.physical(), "before":[],"before_fingerprint":m::RecordDigest::new([2;32]), "after":[]}});
    private_write(
        &path.join("state.json"),
        &serde_json::to_vec(&value).unwrap(),
    );
    assert!(matches!(
        StackCommandJournal::open(&path, f.scope.clone()),
        Err(Error::Unavailable)
    ));
}
