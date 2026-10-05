use super::*;
#[tokio::test]
async fn saved_complete_inventory_is_required_for_changed_tree_publication() {
    let f = Input::build(true).await;
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("journal");
    let plans = temp.path().join("plans");
    let journal = f.journal(&path);
    let store = StackCheckoutPlans::create(&plans, f.scope.clone()).unwrap();
    journal
        .save_local_tree_checkout(&store, actor(), &f.apply(), &f.plan)
        .unwrap();
    assert_eq!(
        journal.transition(
            actor(),
            &f.apply(),
            &s::ReceiptState::Prepared,
            &s::ReceiptState::PublicationUnknown
        ),
        Err(Error::InvalidTransition)
    );
    let inventory = f.inventory().await;
    let before = f.folder.state();
    journal
        .save_local_physical_inventory(&store, actor(), &f.apply(), &inventory)
        .unwrap();
    assert_eq!(json(&path)["version"], 5);
    assert_eq!(json(&plans)["version"], 2);
    assert_eq!(
        journal
            .load_local_physical_inventory(&store, actor(), &f.apply())
            .unwrap(),
        inventory
    );
    journal
        .transition(
            actor(),
            &f.apply(),
            &s::ReceiptState::Prepared,
            &s::ReceiptState::PublicationUnknown,
        )
        .unwrap();
    assert_eq!(f.folder.state(), before);
}

#[tokio::test]
async fn immutable_inventory_reopen_replays_without_scanning_new_editor_state() {
    let f = Input::build(true).await;
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("journal");
    let plans = temp.path().join("plans");
    let journal = f.journal(&path);
    let store = StackCheckoutPlans::create(&plans, f.scope.clone()).unwrap();
    journal
        .save_local_tree_checkout(&store, actor(), &f.apply(), &f.plan)
        .unwrap();
    let inventory = f.inventory().await;
    let original = fs::read(path.join("state.json")).unwrap();
    journal
        .save_local_physical_inventory(&store, actor(), &f.apply(), &inventory)
        .unwrap();
    let encoded = fs::read(path.join("state.json")).unwrap();
    assert!(encoded.len() < original.len() + 512);
    let saved = fs::read(plans.join("state.json")).unwrap();
    fs::write(f.folder.root().join("ignored.txt"), "new editor work\n").unwrap();
    let before = f.folder.state();
    let journal = StackCommandJournal::open(&path, f.scope.clone()).unwrap();
    let store = StackCheckoutPlans::open(&plans, f.scope.clone()).unwrap();
    assert_eq!(
        journal
            .load_local_physical_inventory(&store, actor(), &f.apply())
            .unwrap(),
        inventory
    );
    assert!(
        journal
            .save_local_physical_inventory(&store, actor(), &f.apply(), &inventory)
            .unwrap()
            .replayed
    );
    assert!(
        journal
            .save_local_tree_checkout(&store, actor(), &f.apply(), &f.plan)
            .unwrap()
            .replayed
    );
    store.select(actor(), &f.plan).unwrap(); // never clears the attached inventory
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
    assert_eq!(fs::read(path.join("state.json")).unwrap(), encoded);
    assert_eq!(fs::read(plans.join("state.json")).unwrap(), saved);
    assert_eq!(f.folder.state(), before); // loading original input is not a live physical proof
}

#[tokio::test]
async fn inventory_binding_and_original_authentication_precede_store_access() {
    use jj_store_jj::local_execution::PhysicalInventoryV1;
    let f = Input::build(true).await;
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("journal");
    let plans = temp.path().join("plans");
    let journal = f.journal(&path);
    let store = StackCheckoutPlans::create(&plans, f.scope.clone()).unwrap();
    journal
        .save_local_tree_checkout(&store, actor(), &f.apply(), &f.plan)
        .unwrap();
    let inventory = f.inventory().await;
    let before = fs::read(path.join("state.json")).unwrap();
    let catalog = fs::read(plans.join("state.json")).unwrap();
    let binding = f.execution.witness.binding();
    for scope in ["repository", "generation", "workspace"] {
        let wrong = PhysicalInventoryV1::new(
            if scope == "repository" {
                m::RepositoryId::new([99; 16])
            } else {
                binding.repository
            },
            binding.generation + u64::from(scope == "generation"),
            if scope == "workspace" {
                m::RecordDigest::new([99; 32])
            } else {
                binding.physical_workspace
            },
            inventory.directories().cloned().collect(),
            inventory.files().cloned().collect(),
        )
        .unwrap();
        assert_eq!(
            journal.save_local_physical_inventory(&store, actor(), &f.apply(), &wrong),
            Err(Error::IntentMismatch)
        );
        assert_eq!(fs::read(plans.join("state.json")).unwrap(), catalog);
    }
    fs::remove_file(plans.join("state.json")).unwrap();
    assert_eq!(
        journal.save_local_physical_inventory(
            &store,
            m::PrincipalId::new([99; 16]),
            &f.apply(),
            &inventory
        ),
        Err(Error::IntentMismatch)
    );
    assert_eq!(
        journal.load_local_physical_inventory(&store, m::PrincipalId::new([99; 16]), &f.apply()),
        Err(Error::IntentMismatch)
    );
    let mut wrong = f.apply();
    wrong.selection.binding_generation += 1;
    assert_eq!(
        journal.save_local_physical_inventory(&store, actor(), &wrong, &inventory),
        Err(Error::IntentMismatch)
    );
    assert!(!plans.join("state.json").exists());
    assert_eq!(fs::read(path.join("state.json")).unwrap(), before);
}

#[tokio::test]
async fn missing_inventory_and_tampered_reference_are_never_repaired_from_input() {
    let f = Input::build(true).await;
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("journal");
    let plans = temp.path().join("plans");
    let journal = f.journal(&path);
    let store = StackCheckoutPlans::create(&plans, f.scope.clone()).unwrap();
    journal
        .save_local_tree_checkout(&store, actor(), &f.apply(), &f.plan)
        .unwrap();
    let legacy = fs::read(plans.join("state.json")).unwrap();
    let inventory = f.inventory().await;
    journal
        .save_local_physical_inventory(&store, actor(), &f.apply(), &inventory)
        .unwrap();
    let original = fs::read(path.join("state.json")).unwrap();
    for field in ["id", "encoded_bytes", "plan"] {
        let mut value = json(&path);
        value["records"][0]["execution"]["physical_inventory"][field] = if field == "encoded_bytes"
        {
            1.into()
        } else {
            serde_json::to_value(m::RecordDigest::new([99; 32])).unwrap()
        };
        private_write(
            &path.join("state.json"),
            &serde_json::to_vec(&value).unwrap(),
        );
        if field == "plan" {
            assert!(matches!(
                StackCommandJournal::open(&path, f.scope.clone()),
                Err(Error::Unavailable)
            ));
        } else {
            let opened = StackCommandJournal::open(&path, f.scope.clone()).unwrap();
            assert_eq!(
                opened.load_local_physical_inventory(&store, actor(), &f.apply()),
                Err(Error::IntentMismatch)
            );
            assert_eq!(
                opened.save_local_physical_inventory(&store, actor(), &f.apply(), &inventory),
                Err(Error::IntentMismatch)
            );
        }
        private_write(&path.join("state.json"), &original);
    }
    private_write(&plans.join("state.json"), &legacy); // exact canonical version-1 catalog, body missing
    assert_eq!(
        journal.load_local_physical_inventory(&store, actor(), &f.apply()),
        Err(Error::Unavailable)
    );
    assert_eq!(
        journal.save_local_physical_inventory(&store, actor(), &f.apply(), &inventory),
        Err(Error::Unavailable)
    );
    assert_eq!(fs::read(plans.join("state.json")).unwrap(), legacy);
    assert_eq!(fs::read(path.join("state.json")).unwrap(), original);
}

#[tokio::test]
async fn terminal_legacy_request_never_acquires_inventory_or_changes_bytes() {
    let f = Input::build(true).await;
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("journal");
    let plans = temp.path().join("plans");
    let journal = f.journal(&path);
    let store = StackCheckoutPlans::create(&plans, f.scope.clone()).unwrap();
    journal
        .save_local_tree_checkout(&store, actor(), &f.apply(), &f.plan)
        .unwrap();
    let inventory = f.inventory().await;
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
    let catalog = fs::read(plans.join("state.json")).unwrap();
    assert_eq!(json(&path)["version"], 4);
    assert_eq!(json(&plans)["version"], 1);
    assert_eq!(
        journal.save_local_physical_inventory(&store, actor(), &f.apply(), &inventory),
        Err(Error::InvalidTransition)
    );
    assert_eq!(
        journal.load_local_physical_inventory(&store, actor(), &f.apply()),
        Err(Error::NotFound)
    );
    StackCommandJournal::open(&path, f.scope.clone())
        .unwrap()
        .status(actor(), f.apply().request_id, f.apply().intent_digest)
        .unwrap();
    assert_eq!(fs::read(path.join("state.json")).unwrap(), before);
    assert_eq!(fs::read(plans.join("state.json")).unwrap(), catalog);
}

#[tokio::test]
async fn inventory_contention_releases_journal_and_rejects_wrong_actor_first() {
    let f = Input::build(true).await;
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("journal");
    let plans = temp.path().join("plans");
    let journal = f.journal(&path);
    let store = StackCheckoutPlans::create(&plans, f.scope.clone()).unwrap();
    journal
        .save_local_tree_checkout(&store, actor(), &f.apply(), &f.plan)
        .unwrap();
    let inventory = f.inventory().await;
    let before = fs::read(path.join("state.json")).unwrap();
    let lock = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(plans.join("checkout-plans.lock"))
        .unwrap();
    lock.try_lock().unwrap();
    assert_eq!(
        journal.save_local_physical_inventory(
            &store,
            m::PrincipalId::new([99; 16]),
            &f.apply(),
            &inventory
        ),
        Err(Error::IntentMismatch)
    );
    assert_eq!(
        journal.save_local_physical_inventory(&store, actor(), &f.apply(), &inventory),
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
        .save_local_physical_inventory(&store, actor(), &f.apply(), &inventory)
        .unwrap();
}
