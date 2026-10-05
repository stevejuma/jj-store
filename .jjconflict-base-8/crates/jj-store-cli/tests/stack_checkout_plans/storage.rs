use super::*;
#[tokio::test]
async fn open_lookup_and_duplicate_are_immutable_and_never_touch_checkout() {
    let f = Input::new().await;
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("plans");
    assert!(StackCheckoutPlans::open(&path, f.scope.clone()).is_err());
    assert!(!path.exists());
    let store = StackCheckoutPlans::create(&path, f.scope.clone()).unwrap();
    assert!(
        store
            .lookup(actor(), f.apply().request_id, f.apply().intent_digest)
            .unwrap()
            .is_none()
    );
    let before = f.folder.state();
    let reference = store.select(actor(), &f.plan).unwrap();
    let bytes = fs::read(path.join("state.json")).unwrap();
    let reopened = StackCheckoutPlans::open(&path, f.scope.clone()).unwrap();
    let saved = reopened
        .lookup(actor(), f.apply().request_id, f.apply().intent_digest)
        .unwrap()
        .unwrap();
    assert_eq!(saved.plan, f.plan);
    assert_eq!(saved.reference, reference);
    assert_eq!(reopened.select(actor(), &f.plan).unwrap(), reference);
    assert_eq!(fs::read(path.join("state.json")).unwrap(), bytes);
    assert_eq!(f.folder.state(), before);
    let mut scope = f.scope.clone();
    scope.installation = [7; 16];
    assert!(matches!(
        StackCheckoutPlans::open(&path, scope),
        Err(Error::ScopeMismatch)
    ));
}
#[tokio::test]
async fn scope_actor_intent_and_conflicting_request_reuse_fail_without_selection() {
    let f = Input::new().await;
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("plans");
    let store = StackCheckoutPlans::create(&path, f.scope.clone()).unwrap();
    store.select(actor(), &f.plan).unwrap();
    let before = fs::read(path.join("state.json")).unwrap();
    assert_eq!(
        store.select(m::PrincipalId::new([8; 16]), &f.plan),
        Err(Error::IntentMismatch)
    );
    assert_eq!(
        store.lookup(actor(), f.apply().request_id, m::IntentDigest::new([9; 32])),
        Err(Error::IntentMismatch)
    );
    for field in ["physical", "archive", "recovery"] {
        let mut value = serde_json::to_value(&f.plan).unwrap();
        value[field] = serde_json::to_value(m::RecordDigest::new([12; 32])).unwrap();
        let altered: TreeCheckoutPlanV1 = serde_json::from_value(value).unwrap();
        assert_eq!(store.select(actor(), &altered), Err(Error::IntentMismatch));
    }
    assert_eq!(fs::read(path.join("state.json")).unwrap(), before);
}
#[tokio::test]
async fn malformed_noncanonical_and_unsafe_store_state_is_not_repaired() {
    let f = Input::new().await;
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("plans");
    let store = StackCheckoutPlans::create(&path, f.scope.clone()).unwrap();
    store.select(actor(), &f.plan).unwrap();
    let before = fs::read(path.join("state.json")).unwrap();
    let mut bytes = before.clone();
    bytes.push(b' ');
    private_write(&path.join("state.json"), &bytes);
    assert!(StackCheckoutPlans::open(&path, f.scope.clone()).is_err());
    assert_eq!(fs::read(path.join("state.json")).unwrap(), bytes);
    private_write(&path.join("state.json"), &before);
    let mut value = json(&path);
    value["plans"][0]["reference"]["encoded_bytes"] = 1.into();
    private_write(
        &path.join("state.json"),
        &serde_json::to_vec(&value).unwrap(),
    );
    assert!(StackCheckoutPlans::open(&path, f.scope.clone()).is_err());
    private_write(&path.join("state.json"), &before);
    use std::os::unix::fs::{PermissionsExt as _, symlink};
    fs::set_permissions(path.join("state.json"), fs::Permissions::from_mode(0o644)).unwrap();
    assert!(StackCheckoutPlans::open(&path, f.scope.clone()).is_err());
    private_write(&path.join("state.json"), &before);
    let held = temp.path().join("held");
    fs::rename(&path, &held).unwrap();
    fs::create_dir(&path).unwrap();
    assert_eq!(
        store.lookup(actor(), f.apply().request_id, f.apply().intent_digest),
        Err(Error::Unavailable)
    );
    fs::remove_dir(&path).unwrap();
    symlink(&held, &path).unwrap();
    assert!(StackCheckoutPlans::open(&path, f.scope.clone()).is_err());
}
#[tokio::test]
async fn bounded_record_capacity_preserves_all_original_plans() {
    let f = Input::new().await;
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("plans");
    let store = StackCheckoutPlans::create(&path, f.scope.clone()).unwrap();
    // Synthetic identities isolate storage capacity; these altered plans are
    // deliberately not provider-revalidated or executed.
    for id in 1..=128u8 {
        let mut value = serde_json::to_value(&f.plan).unwrap();
        value["key"]["request"] = serde_json::to_value(m::RequestId::new([id; 16])).unwrap();
        store
            .select(actor(), &serde_json::from_value(value).unwrap())
            .unwrap();
    }
    let before = fs::read(path.join("state.json")).unwrap();
    let mut value = serde_json::to_value(&f.plan).unwrap();
    value["key"]["request"] = serde_json::to_value(m::RequestId::new([129; 16])).unwrap();
    assert_eq!(
        store.select(actor(), &serde_json::from_value(value).unwrap()),
        Err(Error::Capacity)
    );
    assert_eq!(fs::read(path.join("state.json")).unwrap(), before);
    assert_eq!(json(&path)["plans"].as_array().unwrap().len(), 128);
}
