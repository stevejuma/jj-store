use super::*;
use jj_store_object::{
    ByteRange, Condition, Download, Key, Metadata, ObjectStore, StagedUpload, WriteOutcome,
};
use std::{collections::BTreeMap, sync::Mutex};

#[derive(Default)]
struct CountingStore {
    inner: MemoryStore,
    reads: Mutex<BTreeMap<String, usize>>,
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
        tokio::task::yield_now().await;
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
fn reader(store: Arc<CountingStore>, permission: Permission, nodes: usize) -> Arc<NativeSession> {
    Arc::new(
        NativeSession::new_local_trusted(
            store.clone(),
            Arc::new(NativeObjects::new(store, REPO, 2 * 1024 * 1024).unwrap()),
            ACTOR,
            permission,
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits {
                inventory_nodes: nodes,
                ..Default::default()
            },
        )
        .unwrap(),
    )
}
fn domain_reads(reads: &BTreeMap<String, usize>) -> usize {
    reads
        .iter()
        .filter(|(key, _)| key.contains("/work-"))
        .map(|(_, count)| count)
        .sum()
}
#[tokio::test]
async fn work_first_pages_share_selected_roots_and_enforce_fresh_limits() {
    let store = Arc::new(CountingStore::default());
    let objects = Arc::new(NativeObjects::new(store.clone(), REPO, 2 * 1024 * 1024).unwrap());
    create_repository_local_trusted(
        store.clone(),
        objects,
        Permission::Admin,
        m::EpochId::new([3; 16]),
        Default::default(),
    )
    .await
    .unwrap();
    let writer = reader(store.clone(), Permission::Write, 100_000);
    create(writer.clone(), 10).await;
    create(writer.clone(), 11).await;
    let pager = WorkPager::new(Arc::new(SystemClock));
    let query = WorkBrowseQuery {
        limit: 1,
        ..Default::default()
    };
    store.take();
    let first = pager.browse(writer.clone(), query.clone()).await.unwrap();
    let cold_reads = store.take();
    assert!(domain_reads(&cold_reads) > 0, "{cold_reads:?}");
    let (again, details) = tokio::join!(
        pager.browse(writer.clone(), query.clone()),
        pager.page(writer.clone(), WorkQuery::default()),
    );
    assert_eq!(again.unwrap().snapshot, first.snapshot);
    assert_eq!(details.unwrap().snapshot, first.snapshot);
    assert_eq!(
        domain_reads(&store.take()),
        0,
        "unchanged roots must not be reconstructed"
    );
    let low = reader(store.clone(), Permission::Read, 1);
    assert_eq!(
        pager.browse(low.clone(), query.clone()).await.unwrap_err(),
        Error::Limit
    );
    let mut next = query.clone();
    next.cursor = first.next_cursor;
    assert_eq!(
        pager.browse(low, next.clone()).await.unwrap_err(),
        Error::Limit
    );
    create(writer.clone(), 12).await;
    store.take();
    let fresh = pager.browse(writer.clone(), query).await.unwrap();
    assert_eq!(fresh.total_matches, 3);
    assert_ne!(fresh.snapshot.work_root, first.snapshot.work_root);
    assert!(domain_reads(&store.take()) > 0);
    store.take();
    let historical = pager.browse(writer, next).await.unwrap();
    assert_eq!(historical.snapshot, first.snapshot);
    assert_eq!(historical.total_matches, 2);
    assert_eq!(historical.projects[0].id, ProjectId::new([11; 16]));
    assert_eq!(domain_reads(&store.take()), 0);
}
