use super::*;
use crate::{auth::Permission, native::create_repository_local_trusted};
use jj_store_jj::{objects::NativeObjects, roots};
use wal::inventory::DependencySource as _;

async fn selected() -> (SourceScope, ManifestSnapshot) {
    let repository = m::RepositoryId::new([6; 16]);
    let store = Arc::new(jj_store_object::memory::MemoryStore::default());
    let objects = Arc::new(NativeObjects::new(store.clone(), repository, 8 * 1024 * 1024).unwrap());
    create_repository_local_trusted(
        store.clone(),
        objects.clone(),
        Permission::Admin,
        m::EpochId::new([2; 16]),
        wal::publication::Limits::default(),
    )
    .await
    .unwrap();
    (
        objects.scope(),
        ManifestSnapshot::load(store, repository).await.unwrap(),
    )
}

#[tokio::test]
async fn expiry_eviction_and_active_owners_preserve_allocation_charges() {
    let (scope, selected) = selected().await;
    let cache = Cache {
        budget: Budget::new(HEADER),
        roots: 1,
        ttl: Duration::from_secs(10),
        ..Cache::default()
    };
    let key = Key::new(scope, &selected, false);
    let build = cache.build(None);
    assert!(build.lease.is_some());
    cache.finish(key, selected, build);
    let created = cache.entries.lock().unwrap()[0].created;
    let held = cache
        .lookup_at(key, 0, created + Duration::from_secs(9))
        .0
        .unwrap();
    assert!(
        cache
            .lookup_at(key, 0, created + Duration::from_secs(10))
            .0
            .is_none()
    );
    assert!(cache.entries.lock().unwrap().is_empty());
    assert_eq!(cache.budget.used(), HEADER);
    assert!(
        cache.reserve(HEADER).is_none(),
        "eviction must not free active data"
    );
    drop(held);
    assert_eq!(cache.budget.used(), 0);
    assert!(cache.reserve(HEADER).is_some());
}

#[tokio::test]
async fn exact_kind_data_is_isolated_by_source_epoch_and_kind() {
    let (scope, selected) = selected().await;
    let cache = Cache {
        roots: 1,
        ..Cache::default()
    };
    let key = Key::new(scope, &selected, false);
    let mut build = cache.build(None);
    build.insert(m::ObjectKey::Operation(roots::ROOT_OPERATION));
    cache.finish(key, selected.clone(), build);
    let held = cache.lookup(key, 0).0.unwrap();
    for foreign in [
        Key {
            scope: SourceScope::new(scope.repository()),
            ..key
        },
        Key {
            epoch: m::EpochId::new([3; 16]),
            ..key
        },
        Key {
            copies: true,
            ..key
        },
    ] {
        let (exact, base) = cache.lookup(foreign, 0);
        assert!(exact.is_none() && base.is_none());
    }
    let mut other = key;
    other.copies = true;
    cache.finish(other, selected, cache.build(None));
    assert!(cache.lookup(key, 0).0.is_none());
    assert_eq!(
        held.ids,
        BTreeSet::from([m::ObjectKey::Operation(roots::ROOT_OPERATION)])
    );
    cache.discard(other);
    assert_eq!(cache.budget.used(), allocation(1).unwrap());
    drop(held);
    assert_eq!(cache.budget.used(), 0);
}

#[tokio::test]
async fn growth_exhaustion_falls_back_without_retaining_incomplete_data() {
    let (scope, selected) = selected().await;
    let cache = Cache {
        budget: Budget::new(allocation(1).unwrap()),
        ..Cache::default()
    };
    let key = Key::new(scope, &selected, false);
    let mut build = cache.build(None);
    build.insert(m::ObjectKey::Operation(roots::ROOT_OPERATION));
    assert!(build.lease.is_some());
    build.insert(m::ObjectKey::Operation(m::OperationId::new([4; 64])));
    assert!(build.lease.is_none());
    assert_eq!(cache.finish(key, selected, build).len(), 2);
    assert_eq!(cache.budget.used(), 0);
    assert!(cache.entries.lock().unwrap().is_empty());
}
