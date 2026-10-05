use super::*;
use jj_store_service::stack_workspace as s;

fn scope(f: &Prepared) -> s::RecoveryScope {
    s::RecoveryScope::from(&f.preview.plan.request.selection)
}
#[tokio::test]
async fn public_catalog_preserves_selected_state_during_pending_and_strips_private_data() {
    let f = Prepared::new().await;
    let path = f.private.path().join("combinations");
    let store = CombinationStore::create(&path, f.scope.clone(), f.actor).unwrap();
    let before = f.folder.state();
    let empty = store.public_catalog(f.actor, &scope(&f)).unwrap();
    empty.validate().unwrap();
    assert!(empty.combinations.is_empty());
    assert!(!empty.pending);
    store
        .stage(f.actor, 0, &f.preview, &f.after, &f.retained)
        .unwrap();
    let pending = store.public_catalog(f.actor, &scope(&f)).unwrap();
    pending.validate().unwrap();
    assert!(pending.pending);
    assert!(pending.combinations.is_empty());
    assert_eq!(pending.revision, 0);
    f.mark_completed(); // Selected metadata fixture, not actual setup execution.
    store.finish(&f.journal, f.actor, &f.apply()).unwrap();
    let catalog = store.public_catalog(f.actor, &scope(&f)).unwrap();
    catalog.validate().unwrap();
    assert!(!catalog.pending);
    assert_eq!(catalog.revision, 1);
    assert_eq!(catalog.combinations.len(), 1);
    assert_eq!(
        catalog.active().unwrap().selection,
        f.after.combinations[0].selection
    );
    assert_eq!(catalog.active().unwrap().code, f.after.combinations[0].code);
    let encoded = serde_json::to_value(&catalog).unwrap();
    let fields = encoded["combinations"][0].as_object().unwrap();
    assert_eq!(fields.len(), 6);
    for key in [
        "retention",
        "immutable_heads",
        "principal",
        "after",
        "execution",
    ] {
        assert!(!fields.contains_key(key));
    }
    let bytes = std::fs::read(path.join("state.json")).unwrap();
    drop(store);
    let reopened = CombinationStore::open(&path, f.scope.clone(), f.actor).unwrap();
    assert_eq!(
        reopened.public_catalog(f.actor, &scope(&f)).unwrap(),
        catalog
    );
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
    assert_eq!(f.folder.state(), before);
}

#[tokio::test]
async fn public_catalog_refuses_foreign_scope_actor_and_corrupted_state() {
    let f = Prepared::new().await;
    let path = f.private.path().join("combinations");
    assert!(CombinationStore::open(&path, f.scope.clone(), f.actor).is_err());
    assert!(!path.exists());
    let store = CombinationStore::create(&path, f.scope.clone(), f.actor).unwrap();
    assert_eq!(
        store.public_catalog(jj_store_model::PrincipalId::new([99; 16]), &scope(&f)),
        Err(c::Error::Forbidden)
    );
    let mut wrong = scope(&f);
    wrong.physical_workspace = s::PhysicalWorkspaceId(jj_store_model::RecordDigest::new([99; 32]));
    assert_eq!(
        store.public_catalog(f.actor, &wrong),
        Err(c::Error::Forbidden)
    );
    let bytes = std::fs::read(path.join("state.json")).unwrap();
    assert_eq!(
        store.public_catalog(f.actor, &scope(&f)).unwrap().revision,
        0
    );
    assert_eq!(std::fs::read(path.join("state.json")).unwrap(), bytes);
    std::fs::write(path.join("state.json"), b"{}").unwrap();
    assert_eq!(
        store.public_catalog(f.actor, &scope(&f)),
        Err(c::Error::Unavailable)
    );
}
