use super::*;
use jj_store_jj::local_execution::PhysicalInventoryV1;
#[tokio::test]
async fn process_loss_on_each_side_of_inventory_reference_retains_exact_original() {
    for phase in ["body", "reference"] {
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
        let input = temp.path().join("input.json");
        private_write(
            &input,
            &serde_json::to_vec(&(
                f.scope.clone(),
                f.plan.clone(),
                f.apply(),
                inventory.clone(),
            ))
            .unwrap(),
        );
        let before = f.folder.state();
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--ignored",
                "--exact",
                "inventory_process::exit_after_inventory_selection",
                "--nocapture",
            ])
            .env("JJ_STORE_CHECKOUT_INVENTORY_INPUT", &input)
            .env("JJ_STORE_CHECKOUT_INVENTORY_STORE", &plans)
            .env("JJ_STORE_CHECKOUT_INVENTORY_JOURNAL", &path)
            .env("JJ_STORE_CHECKOUT_INVENTORY_PHASE", phase)
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(95),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let saved_body = fs::read(plans.join("state.json")).unwrap();
        let saved_journal = fs::read(path.join("state.json")).unwrap();
        let journal = StackCommandJournal::open(&path, f.scope.clone()).unwrap();
        let store = StackCheckoutPlans::open(&plans, f.scope.clone()).unwrap();
        assert_eq!(json(&path)["version"], if phase == "body" { 4 } else { 5 });
        if phase == "body" {
            assert_eq!(
                journal.load_local_physical_inventory(&store, actor(), &f.apply()),
                Err(Error::NotFound)
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
        }
        let selection = journal
            .save_local_physical_inventory(&store, actor(), &f.apply(), &inventory)
            .unwrap();
        assert_eq!(selection.replayed, phase == "reference");
        assert_eq!(selection.receipt.state, s::ReceiptState::Prepared);
        assert_eq!(
            journal
                .load_local_physical_inventory(&store, actor(), &f.apply())
                .unwrap(),
            inventory
        );
        assert_eq!(fs::read(plans.join("state.json")).unwrap(), saved_body);
        if phase == "reference" {
            assert_eq!(fs::read(path.join("state.json")).unwrap(), saved_journal);
        }
        assert_eq!(f.folder.state(), before);
    }
}
#[test]
#[ignore = "isolated worker exits after body/reference selection before acknowledgement"]
fn exit_after_inventory_selection() {
    let input = PathBuf::from(std::env::var_os("JJ_STORE_CHECKOUT_INVENTORY_INPUT").unwrap());
    let plans = PathBuf::from(std::env::var_os("JJ_STORE_CHECKOUT_INVENTORY_STORE").unwrap());
    let path = PathBuf::from(std::env::var_os("JJ_STORE_CHECKOUT_INVENTORY_JOURNAL").unwrap());
    let (scope, plan, apply, inventory): (
        Scope,
        TreeCheckoutPlanV1,
        s::ApplyRequest,
        PhysicalInventoryV1,
    ) = serde_json::from_slice(&fs::read(input).unwrap()).unwrap();
    let store = StackCheckoutPlans::open(&plans, scope.clone()).unwrap();
    if std::env::var("JJ_STORE_CHECKOUT_INVENTORY_PHASE").unwrap() == "body" {
        store.select_inventory(actor(), &plan, &inventory).unwrap();
    } else {
        StackCommandJournal::open(&path, scope)
            .unwrap()
            .save_local_physical_inventory(&store, actor(), &apply, &inventory)
            .unwrap();
    }
    std::process::exit(95);
}
