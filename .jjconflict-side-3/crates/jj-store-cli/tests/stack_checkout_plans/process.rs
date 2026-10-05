use super::*;
#[tokio::test]
async fn process_loss_after_plan_before_reference_reuses_original_on_reopen() {
    let f = Input::new().await;
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("journal");
    let plans = temp.path().join("plans");
    let journal = f.journal(&path);
    StackCheckoutPlans::create(&plans, f.scope.clone()).unwrap();
    let input = temp.path().join("input.json");
    private_write(
        &input,
        &serde_json::to_vec(&(f.scope.clone(), f.plan.clone())).unwrap(),
    );
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--ignored",
            "--exact",
            "process::exit_after_plan",
            "--nocapture",
        ])
        .env("JJ_STORE_CHECKOUT_PLAN_INPUT", &input)
        .env("JJ_STORE_CHECKOUT_PLAN_STORE", &plans)
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(93),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let before = fs::read(plans.join("state.json")).unwrap();
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
    let store = StackCheckoutPlans::open(&plans, f.scope.clone()).unwrap();
    journal
        .save_local_tree_checkout(&store, actor(), &f.apply(), &f.plan)
        .unwrap();
    assert_eq!(fs::read(plans.join("state.json")).unwrap(), before);
    assert_eq!(
        journal
            .load_local_tree_checkout(&store, actor(), &f.apply())
            .unwrap(),
        f.plan
    );
}
#[test]
#[ignore = "isolated worker exits after durable plan selection before journal reference"]
fn exit_after_plan() {
    let input = PathBuf::from(std::env::var_os("JJ_STORE_CHECKOUT_PLAN_INPUT").unwrap());
    let plans = PathBuf::from(std::env::var_os("JJ_STORE_CHECKOUT_PLAN_STORE").unwrap());
    let (scope, plan): (Scope, TreeCheckoutPlanV1) =
        serde_json::from_slice(&fs::read(input).unwrap()).unwrap();
    StackCheckoutPlans::open(&plans, scope)
        .unwrap()
        .select(actor(), &plan)
        .unwrap();
    std::process::exit(93);
}
#[tokio::test]
async fn process_loss_after_reference_selection_preserves_original_receipt() {
    let f = Input::new().await;
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("journal");
    let plans = temp.path().join("plans");
    let journal = f.journal(&path);
    StackCheckoutPlans::create(&plans, f.scope.clone()).unwrap();
    let input = temp.path().join("input.json");
    private_write(
        &input,
        &serde_json::to_vec(&(f.scope.clone(), f.plan.clone(), f.apply())).unwrap(),
    );
    let before = f.folder.state();
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--ignored",
            "--exact",
            "process::exit_after_reference",
            "--nocapture",
        ])
        .env("JJ_STORE_CHECKOUT_PLAN_INPUT", &input)
        .env("JJ_STORE_CHECKOUT_PLAN_STORE", &plans)
        .env("JJ_STORE_CHECKOUT_PLAN_JOURNAL", &path)
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(94),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let before_journal = fs::read(path.join("state.json")).unwrap();
    let store = StackCheckoutPlans::open(&plans, f.scope.clone()).unwrap();
    let replay = journal
        .save_local_tree_checkout(&store, actor(), &f.apply(), &f.plan)
        .unwrap();
    assert!(replay.replayed);
    assert_eq!(replay.receipt.state, s::ReceiptState::Prepared);
    assert_eq!(
        journal
            .load_local_tree_checkout(&store, actor(), &f.apply())
            .unwrap(),
        f.plan
    );
    assert_eq!(fs::read(path.join("state.json")).unwrap(), before_journal);
    assert_eq!(f.folder.state(), before);
}
#[test]
#[ignore = "isolated worker exits after reference selection before caller acknowledgement"]
fn exit_after_reference() {
    let input = PathBuf::from(std::env::var_os("JJ_STORE_CHECKOUT_PLAN_INPUT").unwrap());
    let plans = PathBuf::from(std::env::var_os("JJ_STORE_CHECKOUT_PLAN_STORE").unwrap());
    let path = PathBuf::from(std::env::var_os("JJ_STORE_CHECKOUT_PLAN_JOURNAL").unwrap());
    let (scope, plan, apply): (Scope, TreeCheckoutPlanV1, s::ApplyRequest) =
        serde_json::from_slice(&fs::read(input).unwrap()).unwrap();
    StackCommandJournal::open(&path, scope.clone())
        .unwrap()
        .save_local_tree_checkout(
            &StackCheckoutPlans::open(&plans, scope).unwrap(),
            actor(),
            &apply,
            &plan,
        )
        .unwrap();
    std::process::exit(94);
}
