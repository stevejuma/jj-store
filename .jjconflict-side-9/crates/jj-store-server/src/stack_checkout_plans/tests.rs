use super::*;
mod fixture;
use fixture::{plan, scope};
#[test]
fn replaced_store_or_lock_is_unavailable_and_original_selection_is_unchanged() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("plans");
    let store = StackCheckoutPlans::create(&path, scope()).unwrap();
    let gate = store.root.gate().unwrap();
    let (bytes, stamp) = store.root.read(&gate).unwrap();
    let lock = path.join("checkout-plans.lock");
    let held = temp.path().join("held");
    std::fs::rename(&lock, &held).unwrap();
    assert_eq!(
        store
            .root
            .persist(&gate, b"replacement", Some(stamp))
            .map_err(Error::from),
        Err(Error::Unavailable)
    );
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
    std::fs::rename(held, lock).unwrap();
}
#[test]
fn unknown_plan_acknowledgement_keeps_exact_input_and_replay_syncs_it() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("plans");
    let store = StackCheckoutPlans::create(&path, scope()).unwrap();
    let original = plan();
    let gate = store.root.gate().unwrap();
    let (mut catalog, stamp) = store.read(&gate).unwrap();
    catalog.plans.push(SavedTreePlanV1 {
        reference: TreeCheckoutReferenceV1::from_plan(&original).unwrap(),
        plan: original.clone(),
        inventory: None,
    });
    let bytes = catalog.encode().unwrap();
    let held = temp.path().join("held");
    assert_eq!(
        store
            .root
            .persist_with(&gate, &bytes, Some(stamp), || {
                std::fs::rename(path.join("checkout-plans.lock"), &held).unwrap();
                Ok(())
            })
            .map_err(Error::from),
        Err(Error::OutcomeUnknown)
    );
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
    drop(gate);
    std::fs::rename(held, path.join("checkout-plans.lock")).unwrap();
    let reopened = StackCheckoutPlans::open(&path, scope()).unwrap();
    reopened
        .select(original.preparation().principal, &original)
        .unwrap();
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
}
#[test]
fn aggregate_capacity_fails_before_selection_without_evicting_original() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("plans");
    let store = StackCheckoutPlans::create(&path, scope()).unwrap();
    let original = plan();
    store
        .select(original.preparation().principal, &original)
        .unwrap();
    let gate = store.root.gate().unwrap();
    let (mut catalog, stamp) = store.read(&gate).unwrap();
    let before = std::fs::read(path.join("state.json")).unwrap();
    // Synthetic pressure data, never an engine plan or physical-install proof.
    let mut value = serde_json::to_value(&original).unwrap();
    let mut directories =
        vec![serde_json::json!({"path":"", "identity":m::RecordDigest::new([9;32])})];
    for n in 0..240_000 {
        directories.push(serde_json::json!({"path":format!("dir-{n:06}"), "identity":null}));
    }
    value["directories"] = directories.into();
    for id in 1..=8u8 {
        value["key"]["request"] = serde_json::to_value(m::RequestId::new([id; 16])).unwrap();
        let plan: TreeCheckoutPlanV1 = serde_json::from_value(value.clone()).unwrap();
        catalog.plans.push(SavedTreePlanV1 {
            reference: TreeCheckoutReferenceV1::from_plan(&plan).unwrap(),
            plan,
            inventory: None,
        });
    }
    assert_eq!(catalog.encode(), Err(Error::Capacity));
    // The primitive independently rejects oversized bytes before replacement.
    assert_eq!(
        store
            .root
            .persist(&gate, &vec![0; MAX_BYTES + 1], Some(stamp))
            .map_err(Error::from),
        Err(Error::Capacity)
    );
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), before);
}

#[test]
fn unknown_inventory_acknowledgement_reopens_exact_body_without_clearing_plan() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("plans");
    let store = StackCheckoutPlans::create(&path, scope()).unwrap();
    let (plan, inventory) = fixture::inventory_plan();
    let principal = plan.preparation().principal;
    store.select(principal, &plan).unwrap();
    let gate = store.root.gate().unwrap();
    let (mut catalog, stamp) = store.read(&gate).unwrap();
    catalog.plans[0].inventory = Some(SavedPhysicalInventoryV1 {
        reference: PhysicalInventoryReferenceV1::from_inventory(&plan, &inventory).unwrap(),
        inventory: inventory.clone(),
    });
    catalog.include_inventory();
    let bytes = catalog.encode().unwrap();
    let held = temp.path().join("held");
    assert_eq!(
        store
            .root
            .persist_with(&gate, &bytes, Some(stamp), || {
                std::fs::rename(path.join("checkout-plans.lock"), &held).unwrap();
                Ok(())
            })
            .map_err(Error::from),
        Err(Error::OutcomeUnknown)
    );
    drop(gate);
    std::fs::rename(held, path.join("checkout-plans.lock")).unwrap();
    let reopened = StackCheckoutPlans::open(&path, scope()).unwrap();
    reopened
        .select_inventory(principal, &plan, &inventory)
        .unwrap();
    reopened.select(principal, &plan).unwrap();
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
}
