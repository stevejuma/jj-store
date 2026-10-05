use super::*;
fn folder() -> LocalFolder {
    LocalFolder {
        path: "/repo".into(),
        metadata: "/repo/.jj/repo".into(),
        repository: Some(RepositoryId::new([1; 16])),
    }
}
#[test]
fn repeated_checks_share_validation_but_boundaries_revalidate() {
    let a = FolderAdmission::default();
    let f = folder();
    a.validate(&f, 1, true, || Ok((f.clone(), 1))).unwrap();
    for _ in 0..3200 {
        assert!(a.cached(&f, 1));
    }
    assert_eq!(a.stats().0, 1);
    assert!(!a.cached(&f, 2));
    // An external change without any watcher event is rejected at the boundary.
    assert_eq!(
        a.validate(&f, 1, true, || Err(Error::IdentityChanged)),
        Err(Error::IdentityChanged)
    );
    assert!(!a.cached(&f, 1));
    a.validate(&f, 1, true, || Ok((f.clone(), 1))).unwrap();
    assert!(a.cached(&f, 1));
}
#[test]
fn invalidation_and_expiry_require_new_validation() {
    let a = FolderAdmission::default();
    let f = folder();
    a.validate(&f, 1, false, || Ok((f.clone(), 1))).unwrap();
    a.invalidate();
    assert!(!a.cached(&f, 1));
    a.validate(&f, 1, false, || Ok((f.clone(), 1))).unwrap();
    a.value.lock().unwrap().as_mut().unwrap().completed -= MAX_AGE;
    assert!(!a.cached(&f, 1));
    assert_eq!(
        a.validate(&f, 1, false, || Ok((f.clone(), 2))),
        Err(Error::IdentityChanged)
    );
    assert!(!a.cached(&f, 1));
}
#[test]
fn invalidation_during_validation_does_not_publish_a_stale_entry() {
    let a = FolderAdmission::default();
    let f = folder();
    assert_eq!(
        a.validate(&f, 1, false, || {
            a.invalidate();
            Ok((f.clone(), 1))
        }),
        Err(Error::Stale)
    );
    assert!(!a.cached(&f, 1));
}
#[test]
fn concurrent_cold_checks_share_one_physical_inspection() {
    let a = Arc::new(FolderAdmission::default());
    let barrier = Arc::new(std::sync::Barrier::new(8));
    let workers: Vec<_> = (0..8)
        .map(|_| {
            let a = a.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                let f = folder();
                a.validate(&f, 1, false, || {
                    std::thread::sleep(Duration::from_millis(10));
                    Ok((f.clone(), 1))
                })
                .unwrap();
            })
        })
        .collect();
    for worker in workers {
        worker.join().unwrap();
    }
    assert_eq!(a.stats().0, 1);
}
#[test]
fn capacity_preserves_live_entries_and_invalidation_reaches_them() {
    let cache = FolderAdmissions::default();
    let live: Vec<_> = (0..CAPACITY)
        .map(|i| {
            cache
                .get(RepositoryId::new((i as u128).to_be_bytes()))
                .unwrap()
        })
        .collect();
    let next = RepositoryId::new((CAPACITY as u128).to_be_bytes());
    assert!(cache.get(next).is_none());
    let f = folder();
    live[0]
        .validate(&f, 1, false, || Ok((f.clone(), 1)))
        .unwrap();
    cache.invalidate();
    assert!(!live[0].cached(&f, 1));
    drop(live);
    assert!(cache.get(next).is_some());
    assert_eq!(cache.0.lock().unwrap().len(), CAPACITY);
}
