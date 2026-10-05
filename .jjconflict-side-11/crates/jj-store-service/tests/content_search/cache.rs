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
    fn take(&self) -> usize {
        std::mem::take(&mut *self.reads.lock().unwrap())
            .iter()
            .filter(|(key, _)| {
                key.contains("/work-")
                    || key.contains("/thread-roots/")
                    || key.contains("/collab-objects/")
            })
            .map(|(_, n)| n)
            .sum()
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
fn reader(store: Arc<CountingStore>, nodes: usize) -> Arc<NativeSession> {
    Arc::new(
        NativeSession::new_local_trusted(
            store.clone(),
            Arc::new(NativeObjects::new(store, REPO, 8 * 1024 * 1024).unwrap()),
            ACTOR,
            Permission::Admin,
            Arc::new(auth::SystemClock),
            Protection::default(),
            SessionLimits {
                inventory_nodes: nodes,
                ..Default::default()
            },
        )
        .unwrap(),
    )
}
#[tokio::test]
async fn content_search_reuses_unchanged_roots_across_filters_and_rechecks_limits() {
    let store = Arc::new(CountingStore::default());
    let objects = Arc::new(NativeObjects::new(store.clone(), REPO, 8 * 1024 * 1024).unwrap());
    create_repository_local_trusted(
        store.clone(),
        objects,
        Permission::Admin,
        m::EpochId::new([2; 16]),
        Default::default(),
    )
    .await
    .unwrap();
    let writer = reader(store.clone(), 100_000);
    for n in 20..=22 {
        work::WorkCommandService
            .apply(
                writer.clone(),
                &work::Request {
                    version: 1,
                    request_id: m::RequestId::new([n; 16]),
                    command: work::WorkCommandV1::CreateProject {
                        id: work::ProjectId::new([n; 16]),
                        title: format!("Project {n}"),
                        body: "needle Other İ".into(),
                    },
                },
            )
            .await
            .unwrap();
    }
    let pager = ContentPager::new(Arc::new(auth::SystemClock));
    let query = ContentQuery {
        q: "needle".into(),
        limit: 1,
        ..Default::default()
    };
    store.take();
    let first = pager.page(writer.clone(), query.clone()).await.unwrap();
    assert!(store.take() > 0);
    let again = pager
        .page(
            writer.clone(),
            ContentQuery {
                q: "other".into(),
                ..query.clone()
            },
        )
        .await
        .unwrap();
    assert_eq!(again.results.len(), 1);
    assert_eq!(
        store.take(),
        0,
        "a new filter must not reconstruct unchanged domains"
    );
    let low = reader(store.clone(), 1);
    assert!(matches!(
        pager.page(low.clone(), query.clone()).await,
        Err(work::Error::Limit)
    ));
    let next = ContentQuery {
        cursor: first.next_cursor,
        ..query.clone()
    };
    assert!(matches!(
        pager.page(low, next.clone()).await,
        Err(work::Error::Limit)
    ));
    work::WorkCommandService
        .apply(
            writer.clone(),
            &work::Request {
                version: 1,
                request_id: m::RequestId::new([23; 16]),
                command: work::WorkCommandV1::CreateProject {
                    id: work::ProjectId::new([23; 16]),
                    title: "Later".into(),
                    body: "needle".into(),
                },
            },
        )
        .await
        .unwrap();
    store.take();
    let new = pager.page(writer.clone(), query).await.unwrap();
    assert_ne!(new.snapshot.work_root, first.snapshot.work_root);
    assert!(store.take() > 0);
    let old = pager.page(writer, next).await.unwrap();
    assert_eq!(old.snapshot.work_root, first.snapshot.work_root);
    assert_ne!(old.results[0].id, first.results[0].id);
    assert_eq!(store.take(), 0);
}
