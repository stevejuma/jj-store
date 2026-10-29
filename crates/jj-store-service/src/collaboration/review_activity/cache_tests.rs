use super::Error;
use super::*;
use crate::collaboration::review_activity::tests::{REPO, Store, fixture, reference};
use jj_review::*;
use jj_store_wal::publication::ReviewSnapshotRead;
use std::{future::Future, task::Poll};

const ROOT: Option<m::RecordDigest> = Some(m::RecordDigest::new([86; 32]));

#[tokio::test]
async fn reuse_isolated_by_root_limit_profile_epoch_and_exact_window() {
    let cache = Cache::default();
    let (mut store, key, thread) = fixture(false);
    let pin = ReviewSelection {
        repository: REPO,
        store: &store,
    };
    let variants = [
        (Authority::Local, ROOT, 50, false),
        (
            Authority::Local,
            Some(m::RecordDigest::new([87; 32])),
            50,
            false,
        ),
        (Authority::Local, ROOT, 1, false),
        (Authority::Native(m::EpochId::new([1; 16])), ROOT, 50, false),
        (Authority::Native(m::EpochId::new([2; 16])), ROOT, 50, false),
        (Authority::Local, ROOT, 50, true),
    ];
    for (index, (authority, root, limit, partial)) in variants.into_iter().enumerate() {
        for _ in 0..2 {
            let result = cache
                .read(&pin, authority, root, limit, vec![(key, thread)], partial)
                .await
                .unwrap();
            assert_eq!(result.threads_partial, partial);
            assert_eq!(result.updated_at_millis, Some(900));
        }
        assert_eq!(store.reads.lock().unwrap().len(), (index + 1) * 5);
    }
    // Reference kind and ID and catalog identity are all part of the window;
    // changed invalid selections must not retrieve a previously admitted value.
    for changed in [
        (key, reference(ReviewObjectKind::ThreadSnapshot, 99)),
        (key, reference(ReviewObjectKind::Anchor, 7)),
        (
            ReviewCatalogKey::derive("thread", &[99; 16]).unwrap(),
            thread,
        ),
    ] {
        assert_eq!(
            cache
                .read(&pin, Authority::Local, ROOT, 50, vec![changed], false)
                .await
                .unwrap_err(),
            Error::Corrupt
        );
    }
    let empty = cache
        .read(&pin, Authority::Local, ROOT, 50, vec![], false)
        .await
        .unwrap();
    assert_eq!(empty.threads, 0);
    assert_eq!(empty.updated_at_millis, None);
    let before = store.reads.lock().unwrap().len();
    store.repository = m::RepositoryId::new([99; 16]);
    let foreign = ReviewSelection {
        repository: store.repository,
        store: &store,
    };
    cache
        .read(
            &foreign,
            Authority::Local,
            ROOT,
            50,
            vec![(key, thread)],
            false,
        )
        .await
        .unwrap();
    assert_eq!(
        store.reads.lock().unwrap().len(),
        before + 5,
        "another repository must derive its own facts"
    );
}

#[tokio::test]
async fn malformed_admission_is_rejected_even_after_warm_and_empty_reads() {
    let cache = Cache::default();
    let (store, key, thread) = fixture(false);
    let pin = ReviewSelection {
        repository: REPO,
        store: &store,
    };
    for _ in 0..2 {
        let empty = cache
            .read(&pin, Authority::Local, None, 50, vec![], false)
            .await
            .unwrap();
        assert_eq!(empty.threads, 0);
    }
    cache
        .read(&pin, Authority::Local, ROOT, 50, vec![(key, thread)], false)
        .await
        .unwrap();
    for limit in [0, MAX_PAGE + 1] {
        assert_eq!(
            cache
                .read(&pin, Authority::Local, ROOT, limit, vec![], false)
                .await
                .unwrap_err(),
            Error::Limit
        );
    }
    for (root, limit, rows) in [
        (None, 50, vec![(key, thread)]),
        (ROOT, 1, vec![(key, thread), (key, thread)]),
        (ROOT, 50, vec![(key, thread), (key, thread)]),
    ] {
        assert_eq!(
            cache
                .read(&pin, Authority::Local, root, limit, rows, false)
                .await
                .unwrap_err(),
            Error::Corrupt
        );
    }
    let foreign = ReviewSelection {
        repository: m::RepositoryId::new([99; 16]),
        store: &store,
    };
    assert_eq!(
        cache
            .read(
                &foreign,
                Authority::Local,
                ROOT,
                50,
                vec![(key, thread)],
                false
            )
            .await
            .unwrap_err(),
        Error::Corrupt
    );
    assert_eq!(store.reads.lock().unwrap().len(), 5);
}

#[tokio::test]
async fn failed_derivations_are_not_retained_and_repaired_evidence_can_retry() {
    let cache = Cache::default();
    let (mut store, key, thread) = fixture(false);
    let event = reference(ReviewObjectKind::ThreadEvent, 4);
    let removed = store.values.remove(&event).unwrap();
    assert_eq!(
        cache
            .read(
                &ReviewSelection {
                    repository: REPO,
                    store: &store
                },
                Authority::Local,
                ROOT,
                50,
                vec![(key, thread)],
                false
            )
            .await
            .unwrap_err(),
        Error::Corrupt
    );
    store.values.insert(event, removed);
    let pin = ReviewSelection {
        repository: REPO,
        store: &store,
    };
    let repaired = cache
        .read(&pin, Authority::Local, ROOT, 50, vec![(key, thread)], false)
        .await
        .unwrap();
    assert_eq!(repaired.updated_at_millis, Some(900));
    let reads = store.reads.lock().unwrap().len();
    assert_eq!(
        cache
            .read(&pin, Authority::Local, ROOT, 50, vec![(key, thread)], false)
            .await
            .unwrap(),
        repaired
    );
    assert_eq!(store.reads.lock().unwrap().len(), reads);
}

fn cache(capacity: usize, ttl: Duration) -> Cache {
    Cache {
        entries: Mutex::default(),
        capacity,
        ttl,
    }
}
fn key(byte: u8) -> Key {
    Key {
        repository: m::RepositoryId::new([byte; 16]),
        authority: Authority::Local,
        root: ROOT,
        limit: 50,
        window: [0; 32],
    }
}
fn facts() -> ReviewActivity {
    ReviewActivity {
        threads: 1,
        threads_partial: false,
        updated_at_millis: Some(900),
    }
}

#[tokio::test]
async fn retention_is_fixed_size_idle_lru_with_fixed_not_sliding_expiry() {
    let cache = cache(2, Duration::from_secs(10));
    let now = Instant::now();
    cache
        .slot(key(1), now)
        .unwrap()
        .unwrap()
        .set(facts())
        .unwrap();
    cache
        .slot(key(2), now + Duration::from_secs(1))
        .unwrap()
        .unwrap()
        .set(facts())
        .unwrap();
    drop(cache.slot(key(1), now + Duration::from_secs(2)).unwrap());
    cache
        .slot(key(3), now + Duration::from_secs(3))
        .unwrap()
        .unwrap()
        .set(facts())
        .unwrap();
    assert!(!cache.entries.lock().unwrap().contains_key(&key(2)));
    assert_eq!(cache.entries.lock().unwrap().len(), 2);
    let expired = cache
        .slot(key(1), now + Duration::from_secs(10))
        .unwrap()
        .unwrap();
    assert!(
        !expired.initialized(),
        "touching at two seconds must not extend fixed expiry"
    );
    drop(expired);
    let cache = Cache::default();
    for byte in 0..=255 {
        cache
            .slot(key(byte), now)
            .unwrap()
            .unwrap()
            .set(facts())
            .unwrap();
    }
    assert_eq!(cache.entries.lock().unwrap().len(), MAX_ENTRIES);
    // Neither keys nor numeric facts own heap data; include ample per-entry
    // allowance for BTree framing, Arc/cell and timestamps in this bound.
    assert!(
        std::mem::size_of::<Key>()
            + std::mem::size_of::<Entry>()
            + std::mem::size_of::<OnceCell<ReviewActivity>>()
            < 4096
    );
    assert_eq!(
        serde_json::to_value(facts()).unwrap(),
        serde_json::json!({"threads":1,"threads_partial":false,"updated_at_millis":900})
    );
}

#[tokio::test]
async fn active_saturation_expiry_and_disabled_retention_fall_back_without_query_loss() {
    let cache = cache(1, Duration::from_secs(1));
    let now = Instant::now();
    let active = cache.slot(key(1), now).unwrap().unwrap();
    active.set(facts()).unwrap();
    assert!(
        cache
            .slot(key(1), now + Duration::from_secs(1))
            .unwrap()
            .is_none()
    );
    assert!(cache.slot(key(2), now).unwrap().is_none());
    let (store, row, thread) = fixture(false);
    let pin = ReviewSelection {
        repository: REPO,
        store: &store,
    };
    assert_eq!(
        cache
            .read(&pin, Authority::Local, ROOT, 50, vec![(row, thread)], false)
            .await
            .unwrap(),
        facts()
    );
    assert_eq!(cache.entries.lock().unwrap().len(), 1);
    assert_eq!(active.get(), Some(&facts()));
    drop(active);
    let disabled = super::Cache {
        capacity: 0,
        ..Cache::default()
    };
    for _ in 0..2 {
        assert_eq!(
            disabled
                .read(&pin, Authority::Local, ROOT, 50, vec![(row, thread)], false)
                .await
                .unwrap(),
            facts()
        );
    }
    assert!(disabled.entries.lock().unwrap().is_empty());
    assert_eq!(store.reads.lock().unwrap().len(), 15);
}

struct GatedStore {
    inner: Store,
    thread: ReviewObjectRef,
    release: tokio::sync::Semaphore,
    calls: std::sync::atomic::AtomicUsize,
}
#[async_trait::async_trait]
impl ReviewSnapshotRead for GatedStore {
    fn repository(&self) -> m::RepositoryId {
        self.inner.repository()
    }
    fn root(&self) -> &ReviewRootV1 {
        self.inner.root()
    }
    async fn lookup(
        &self,
        _: ReviewCatalogKind,
        _: ReviewCatalogKey,
    ) -> jj_store_core::Result<Option<ReviewObjectRef>> {
        unreachable!()
    }
    async fn read_object(
        &self,
        reference: ReviewObjectRef,
    ) -> jj_store_core::Result<ReviewObjectV1> {
        if reference == self.thread {
            self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            self.release.acquire().await.unwrap().forget();
        }
        self.inner.read_object(reference).await
    }
}
async fn pending(future: &mut std::pin::Pin<Box<impl Future>>) {
    let result = std::future::poll_fn(|cx| Poll::Ready(future.as_mut().poll(cx))).await;
    assert!(result.is_pending());
}

#[tokio::test]
async fn concurrent_derivations_coalesce_and_canceled_initializer_releases_waiter() {
    for cancel in [false, true] {
        let cache = Cache::default();
        let (inner, key, thread) = fixture(false);
        let store = GatedStore {
            inner,
            thread,
            release: tokio::sync::Semaphore::new(0),
            calls: std::sync::atomic::AtomicUsize::new(0),
        };
        let pin = ReviewSelection {
            repository: REPO,
            store: &store,
        };
        let mut first =
            Box::pin(cache.read(&pin, Authority::Local, ROOT, 50, vec![(key, thread)], false));
        pending(&mut first).await;
        let mut waiter =
            Box::pin(cache.read(&pin, Authority::Local, ROOT, 50, vec![(key, thread)], false));
        pending(&mut waiter).await;
        assert_eq!(store.calls.load(std::sync::atomic::Ordering::SeqCst), 1);
        if cancel {
            drop(first);
            pending(&mut waiter).await;
            assert_eq!(store.calls.load(std::sync::atomic::Ordering::SeqCst), 2);
            store.release.add_permits(1);
            assert_eq!(waiter.await.unwrap(), facts());
        } else {
            store.release.add_permits(1);
            let (first, waiter) = tokio::join!(first, waiter);
            assert_eq!(first.unwrap(), facts());
            assert_eq!(waiter.unwrap(), facts());
        }
        assert_eq!(store.inner.reads.lock().unwrap().len(), 5);
        assert_eq!(
            cache
                .read(&pin, Authority::Local, ROOT, 50, vec![(key, thread)], false)
                .await
                .unwrap(),
            facts()
        );
        assert_eq!(store.inner.reads.lock().unwrap().len(), 5);
    }
}
