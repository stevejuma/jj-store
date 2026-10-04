// Included by the pagination integration test; exercises the production pager.
use super::*;
use jj_store_object::{
    ByteRange, Condition, Download, Key, Metadata, ObjectStore, StagedUpload, WriteOutcome,
};
use std::collections::BTreeMap;

#[derive(Default)]
pub(super) struct CountingStore {
    inner: MemoryStore,
    reads: Mutex<BTreeMap<String, usize>>,
    poisoned: Mutex<BTreeSet<String>>,
    block: AtomicBool,
    entered: tokio::sync::Notify,
}
impl CountingStore {
    fn take(&self) -> BTreeMap<String, usize> {
        std::mem::take(&mut *self.reads.lock().unwrap())
    }
}
#[async_trait::async_trait]
impl ObjectStore for CountingStore {
    async fn head(&self, key: &Key) -> jj_store_object::Result<Metadata> {
        self.inner.head(key).await
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> jj_store_object::Result<Download> {
        *self
            .reads
            .lock()
            .unwrap()
            .entry(key.as_str().into())
            .or_default() += 1;
        // Yield so concurrent requests genuinely overlap while loading a root.
        tokio::task::yield_now().await;
        if self.poisoned.lock().unwrap().contains(key.as_str()) {
            let mut download = self.inner.get(key, range).await?;
            download.metadata.len = 3;
            download.reader = Box::pin(std::io::Cursor::new(b"bad".to_vec()));
            return Ok(download);
        }
        if key.as_str().contains("/thread-roots/") && self.block.load(Ordering::SeqCst) {
            self.entered.notify_one();
            std::future::pending::<()>().await;
        }
        self.inner.get(key, range).await
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> jj_store_object::Result<WriteOutcome> {
        self.inner.put(key, condition, upload).await
    }
}
fn domain_reads(reads: &BTreeMap<String, usize>) -> usize {
    reads
        .iter()
        .filter(|(k, _)| {
            k.contains("/thread-roots/")
                || k.contains("/reaction-roots/")
                || k.contains("/collab-objects/")
        })
        .map(|(_, n)| n)
        .sum()
}

#[tokio::test]
async fn scoped_materializations_reuse_unchanged_roots_and_refresh_envelopes() {
    let f = Fixture::new().await;
    f.create(10).await;
    let target = ReactionTargetV1::Thread {
        thread_id: ThreadId::new([10; 16]),
    };
    f.react(20, target.clone()).await;
    let pager = f.pager(Limits::default());
    f.store.take();
    let first = pager
        .page(f.session(ACTOR), Query::Threads { limit: 50 }, None)
        .await
        .unwrap();
    let cold = f.store.take();
    assert!(domain_reads(&cold) > 0);
    assert!(
        !cold.keys().any(|k| k.contains("/reaction-roots/")),
        "directory loaded Reaction state"
    );
    pager
        .page(f.session(ACTOR), Query::Threads { limit: 50 }, None)
        .await
        .unwrap();
    let warm = f.store.take();
    assert_eq!(domain_reads(&warm), 0, "warm root was rematerialized");
    assert!(
        !warm.is_empty(),
        "authorization/manifest must still be read"
    );
    f.react(21, target.clone()).await;
    f.store.take();
    let changed = pager
        .page(f.session(ACTOR), Query::Threads { limit: 50 }, None)
        .await
        .unwrap();
    assert_ne!(
        first.snapshot.manifest_digest,
        changed.snapshot.manifest_digest
    );
    assert_eq!(first.snapshot.thread_root, changed.snapshot.thread_root);
    assert_eq!(
        domain_reads(&f.store.take()),
        0,
        "unrelated publication evicted Thread data"
    );
    let reaction = pager
        .page(
            f.session(ACTOR),
            Query::Reactions { target, limit: 50 },
            None,
        )
        .await
        .unwrap();
    let Items::Reactions { reactions, .. } = reaction.items else {
        panic!()
    };
    assert_eq!(reactions[0].count, 2);
}

#[tokio::test]
async fn concurrent_cold_reads_share_one_validated_materialization() {
    let f = Fixture::new().await;
    f.create(10).await;
    let query = Query::Threads { limit: 50 };
    let single = f.pager(Limits::default());
    f.store.take();
    single
        .page(f.session(ACTOR), query.clone(), None)
        .await
        .unwrap();
    let once = domain_reads(&f.store.take());
    let pager = f.pager(Limits::default());
    let (a, b, c) = tokio::join!(
        pager.page(f.session(ACTOR), query.clone(), None),
        pager.page(f.session(ACTOR), query.clone(), None),
        pager.page(f.session(ACTOR), query, None)
    );
    assert_eq!(a.unwrap().snapshot, b.unwrap().snapshot);
    c.unwrap();
    assert_eq!(
        domain_reads(&f.store.take()),
        once,
        "concurrent roots loaded repeatedly"
    );
}

#[tokio::test]
async fn expiry_eviction_and_failed_validation_rematerialize_from_storage() {
    let f = Fixture::new().await;
    f.create(10).await;
    f.react(
        20,
        ReactionTargetV1::Thread {
            thread_id: ThreadId::new([10; 16]),
        },
    )
    .await;
    let pager = f.pager(Limits {
        snapshots: 1,
        ttl: Duration::from_secs(10),
        ..Limits::default()
    });
    let query = Query::Threads { limit: 50 };
    f.store.take();
    let first = pager
        .page(f.session(ACTOR), query.clone(), None)
        .await
        .unwrap();
    let cold = f.store.take();
    let root = cold
        .keys()
        .find(|k| k.contains("/thread-roots/"))
        .unwrap()
        .clone();
    // The other domain evicts the sole cached root, without consuming a cursor.
    pager
        .page(f.session(ACTOR), Query::Emojis { limit: 50 }, None)
        .await
        .unwrap();
    f.store.take();
    assert_eq!(
        first.snapshot,
        pager
            .page(f.session(ACTOR), query.clone(), None)
            .await
            .unwrap()
            .snapshot
    );
    assert!(domain_reads(&f.store.take()) > 0);
    f.store.poisoned.lock().unwrap().insert(root.clone());
    // An immutable warm copy remains valid; cold reads must detect corruption.
    pager
        .page(f.session(ACTOR), query.clone(), None)
        .await
        .unwrap();
    f.clock.set(1010);
    assert_eq!(
        pager
            .page(f.session(ACTOR), query.clone(), None)
            .await
            .unwrap_err(),
        Error::Read(jj_store_service::collab_mutations::Error::Corrupt)
    );
    assert!(
        pager
            .page(f.session(ACTOR), query.clone(), None)
            .await
            .is_err()
    );
    f.store.poisoned.lock().unwrap().remove(&root);
    pager.page(f.session(ACTOR), query, None).await.unwrap();
}

#[tokio::test]
async fn epoch_changes_reset_cursors_and_invalidate_materializations() {
    let f = Fixture::new().await;
    f.create(10).await;
    f.create(11).await;
    let pager = f.pager(Limits::default());
    let first = pager
        .page(f.session(ACTOR), Query::Threads { limit: 1 }, None)
        .await
        .unwrap();
    // Simulate an externally adopted authority epoch in deterministic storage.
    // This is test-only; serving/promotion behavior is not changed.
    let key = Key::new(format!("repos/{REPO}/manifest.json")).unwrap();
    let download = f.store.get(&key, None).await.unwrap();
    let version = download.metadata.version.clone();
    let bytes = download.read_bounded(8 * 1024 * 1024).await.unwrap();
    let mut manifest: jj_store_wal::publication::Manifest = serde_json::from_slice(&bytes).unwrap();
    manifest.epoch = m::EpochId::new([9; 16]);
    let bytes = serde_json::to_vec(&manifest).unwrap();
    let upload = StagedUpload::from_bytes(&bytes, 8 * 1024 * 1024)
        .await
        .unwrap();
    f.store
        .put(&key, Condition::Matches(version), &upload)
        .await
        .unwrap();
    assert_eq!(
        pager
            .page(
                f.session(ACTOR),
                Query::Threads { limit: 1 },
                first.next_cursor.as_deref()
            )
            .await
            .unwrap_err(),
        Error::CursorReset
    );
    f.store.take();
    let next = pager
        .page(f.session(ACTOR), Query::Threads { limit: 50 }, None)
        .await
        .unwrap();
    assert_ne!(first.snapshot.epoch, next.snapshot.epoch);
    assert_eq!(first.snapshot.thread_root, next.snapshot.thread_root);
    assert!(
        domain_reads(&f.store.take()) > 0,
        "epoch reused old validation"
    );
}

#[tokio::test]
async fn cached_reactions_apply_viewer_policy_and_changed_roots_load_fresh_data() {
    let f = Fixture::new().await;
    f.create(10).await;
    let target = ReactionTargetV1::Thread {
        thread_id: ThreadId::new([10; 16]),
    };
    f.react(20, target.clone()).await;
    let pager = f.pager(Limits::default());
    let query = Query::Reactions {
        target: target.clone(),
        limit: 50,
    };
    pager
        .page(f.session(ACTOR), query.clone(), None)
        .await
        .unwrap();
    f.store.take();
    let page = pager
        .page(
            f.session(m::PrincipalId::new([20; 16])),
            query.clone(),
            None,
        )
        .await
        .unwrap();
    let Items::Reactions { reactions, .. } = page.items else {
        panic!()
    };
    assert!(reactions[0].viewer_reacted);
    assert_eq!(domain_reads(&f.store.take()), 0);
    f.react(21, target).await;
    f.store.take();
    let page = pager.page(f.session(ACTOR), query, None).await.unwrap();
    let Items::Reactions { reactions, .. } = page.items else {
        panic!()
    };
    assert_eq!(reactions[0].count, 2);
    assert!(!reactions[0].viewer_reacted);
    assert!(domain_reads(&f.store.take()) > 0);
}

#[tokio::test]
async fn cancelled_initialization_is_not_admitted_and_a_later_request_retries() {
    let f = Fixture::new().await;
    f.create(10).await;
    let pager = Arc::new(f.pager(Limits::default()));
    f.store.block.store(true, Ordering::SeqCst);
    let task = tokio::spawn({
        let pager = pager.clone();
        let session = f.session(ACTOR);
        async move {
            pager
                .page(session, Query::Threads { limit: 50 }, None)
                .await
        }
    });
    tokio::time::timeout(Duration::from_secs(2), f.store.entered.notified())
        .await
        .unwrap();
    task.abort();
    assert!(task.await.unwrap_err().is_cancelled());
    f.store.block.store(false, Ordering::SeqCst);
    let page = pager
        .page(f.session(ACTOR), Query::Threads { limit: 50 }, None)
        .await
        .unwrap();
    assert_eq!(ids(page.items), vec![ThreadId::new([10; 16])]);
}

#[tokio::test]
async fn one_entry_cache_still_serves_queries_that_need_two_domain_roots() {
    let f = Fixture::new().await;
    f.create(10).await;
    let target = ReactionTargetV1::Thread {
        thread_id: ThreadId::new([10; 16]),
    };
    f.react(20, target.clone()).await;
    let pager = f.pager(Limits {
        snapshots: 1,
        ..Limits::default()
    });
    let page = pager
        .page(
            f.session(ACTOR),
            Query::Reactions { target, limit: 50 },
            None,
        )
        .await
        .unwrap();
    let Items::Reactions { reactions, .. } = page.items else {
        panic!()
    };
    assert_eq!(reactions[0].count, 1);
}
