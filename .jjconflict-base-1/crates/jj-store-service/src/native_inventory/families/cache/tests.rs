use super::*;
use crate::native_inventory::cache::Budget;
use wal::inventory::SourceScope;
fn fixture() -> Cache {
    Cache {
        entries: Mutex::new(Vec::new()),
        budget: Budget::new(4 * HEADER),
        shared_ids: false,
        roots: 2,
        ttl: Duration::from_secs(300),
    }
}
fn key(scope: SourceScope, n: u8) -> Key {
    Key {
        scope,
        epoch: m::EpochId::new([n; 16]),
        root: None,
    }
}
#[test]
fn copy_slots_coalesce_by_source_epoch_and_keep_busy_gates() {
    let cache = fixture();
    let source = SourceScope::new(m::RepositoryId::new([1; 16]));
    let a = key(source, 1);
    let (_, first) = cache.entry(a).unwrap();
    let (_, second) = cache.entry(a).unwrap();
    assert!(Arc::ptr_eq(&first, &second));
    let (_, other) = cache.entry(key(source, 2)).unwrap();
    assert!(!Arc::ptr_eq(&first, &other));
    assert!(matches!(
        cache.entry(key(SourceScope::new(m::RepositoryId::new([1; 16])), 1)),
        Err(query::Error::IndexBudget)
    ));
    drop(other);
    let (_, next) = cache.entry(key(source, 3)).unwrap();
    assert!(!Arc::ptr_eq(&first, &next));
    let (_, same) = cache.entry(a).unwrap();
    assert!(Arc::ptr_eq(&first, &same));
}
#[test]
fn copy_growth_and_eviction_keep_live_allocation_charged() {
    let cache = fixture();
    let mut lease = cache.reserve(HEADER).unwrap();
    assert_eq!(cache.budget.used(), HEADER);
    cache.grow(&mut lease, 4 * HEADER).unwrap();
    cache.clear_idle();
    assert_eq!(cache.budget.used(), 4 * HEADER);
    assert!(cache.reserve(1).is_err());
    assert!(cache.grow(&mut lease, 4 * HEADER + 1).is_err());
    assert_eq!(cache.budget.used(), 4 * HEADER);
    drop(lease);
    assert_eq!(cache.budget.used(), 0);
}
#[test]
fn copy_slot_expiry_is_fixed_and_does_not_replace_a_busy_gate() {
    let cache = fixture();
    let source = SourceScope::new(m::RepositoryId::new([1; 16]));
    let a = key(source, 1);
    let (_, first) = cache.entry(a).unwrap();
    cache.entries.lock().unwrap()[0].created = Instant::now() - Duration::from_secs(301);
    let (_, same) = cache.entry(a).unwrap();
    assert!(Arc::ptr_eq(&first, &same));
    drop(first);
    drop(same);
    let (_, fresh) = cache.entry(a).unwrap();
    assert_eq!(cache.entries.lock().unwrap().len(), 1);
    assert_eq!(Arc::strong_count(&fresh), 2);
    assert!(cache.entries.lock().unwrap()[0].created.elapsed() < Duration::from_secs(1));
}
