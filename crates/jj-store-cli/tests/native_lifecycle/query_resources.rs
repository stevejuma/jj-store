use super::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn query_cancel_drops_pending_graph_authorization_and_releases_cache() {
    let f = Fixture::new().await;
    let root = tempfile::tempdir().unwrap();
    let paused = Arc::new(PausedAccess {
        inner: pin(&f, roots::ROOT_OPERATION).await,
        calls: Default::default(),
        entered: Default::default(),
        release: Default::default(),
    });
    let exec = q::GraphExecutor::new(root.path().to_owned(), q::Limits::default(), 1).unwrap();
    let job = {
        let exec = exec.clone();
        let access = paused.clone();
        let store = f.store.clone();
        tokio::spawn(async move {
            exec.execute(store, access, q::Request::Operations, page(10))
                .await
        })
    };
    tokio::time::timeout(Duration::from_secs(2), paused.entered.notified())
        .await
        .unwrap();
    job.abort();
    assert!(job.await.unwrap_err().is_cancelled());
    // No release notification: dropping a request must cancel pending I/O.
    let deadline = tokio::time::Instant::now() + Duration::from_secs(1);
    loop {
        match exec
            .execute(
                f.store.clone(),
                paused.clone(),
                q::Request::Operations,
                page(10),
            )
            .await
        {
            Ok(_) => break,
            Err(q::Error::Busy) => {
                assert!(
                    tokio::time::Instant::now() < deadline,
                    "canceled graph worker retained its permit/cache until the unrelated read deadline"
                );
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
            other => panic!("unexpected read result: {other:?}"),
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn query_cancel_drops_pending_content_authorization_and_releases_permit() {
    let f = Fixture::new().await;
    let paused = Arc::new(PausedAccess {
        inner: pin(&f, roots::ROOT_OPERATION).await,
        calls: Default::default(),
        entered: Default::default(),
        release: Default::default(),
    });
    let reader = q::content::ContentReader::new(q::content::Limits::default(), 1).unwrap();
    let job = {
        let reader = reader.clone();
        let access = paused.clone();
        let store = f.store.clone();
        tokio::spawn(async move {
            reader
                .object(store, access, m::ObjectKey::Commit(roots::ROOT_COMMIT))
                .await
        })
    };
    tokio::time::timeout(Duration::from_secs(2), paused.entered.notified())
        .await
        .unwrap();
    job.abort();
    assert!(job.await.unwrap_err().is_cancelled());
    let deadline = tokio::time::Instant::now() + Duration::from_secs(1);
    loop {
        match reader
            .object(
                f.store.clone(),
                paused.clone(),
                m::ObjectKey::Commit(roots::ROOT_COMMIT),
            )
            .await
        {
            Ok(_) => break,
            Err(q::Error::Busy) => {
                assert!(
                    tokio::time::Instant::now() < deadline,
                    "canceled content worker retained its permit until the unrelated read deadline"
                );
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
            other => panic!("unexpected read result: {other:?}"),
        }
    }
}

pub(super) struct BlockingAccess {
    inner: Arc<CommittedRead>,
    calls: std::sync::atomic::AtomicUsize,
    pub entered: tokio::sync::Notify,
    gate: (std::sync::Mutex<bool>, std::sync::Condvar),
}
impl BlockingAccess {
    pub fn new(inner: Arc<CommittedRead>) -> Arc<Self> {
        Arc::new(Self {
            inner,
            calls: Default::default(),
            entered: Default::default(),
            gate: Default::default(),
        })
    }
    pub fn release(&self) {
        *self.gate.0.lock().unwrap() = true;
        self.gate.1.notify_all();
    }
}
#[async_trait::async_trait]
impl q::ReadAccess for BlockingAccess {
    fn repository(&self) -> m::RepositoryId {
        self.inner.repository()
    }
    fn epoch(&self) -> m::EpochId {
        self.inner.epoch()
    }
    fn operation(&self) -> m::OperationId {
        self.inner.operation()
    }
    fn inventory(&self) -> &BTreeSet<m::ObjectKey> {
        self.inner.inventory()
    }
    async fn authorize(&self) -> q::Result<()> {
        if self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst) == 1 {
            self.entered.notify_one();
            let (_guard, timeout) = self
                .gate
                .1
                .wait_timeout_while(
                    self.gate.0.lock().unwrap(),
                    Duration::from_secs(5),
                    |released| !*released,
                )
                .unwrap();
            assert!(
                !timeout.timed_out(),
                "synchronous test gate was not released"
            );
        }
        q::ReadAccess::authorize(self.inner.as_ref()).await
    }
}

use jj_store_object::{
    ByteRange, Condition, Download, Key, Metadata, ObjectStore, StagedUpload, WriteOutcome,
};
struct StallingStore {
    inner: Arc<MemoryStore>,
    entered: tokio::sync::Notify,
    released: Arc<std::sync::atomic::AtomicBool>,
}
struct PendingBody(Arc<std::sync::atomic::AtomicBool>);
impl Drop for PendingBody {
    fn drop(&mut self) {
        self.0.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}
impl tokio::io::AsyncRead for PendingBody {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
        _: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        std::task::Poll::Pending
    }
}
#[async_trait::async_trait]
impl ObjectStore for StallingStore {
    async fn head(&self, key: &Key) -> jj_store_object::Result<Metadata> {
        self.inner.head(key).await
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> jj_store_object::Result<Download> {
        let mut download = self.inner.get(key, range).await?;
        download.reader = Box::pin(PendingBody(self.released.clone()));
        self.entered.notify_one();
        Ok(download)
    }
    async fn put(
        &self,
        _: &Key,
        _: Condition,
        _: &StagedUpload,
    ) -> jj_store_object::Result<WriteOutcome> {
        panic!("query attempted publication")
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn query_cancel_drops_remote_object_bodies_for_graphs_and_content() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let local = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let cache = tempfile::tempdir().unwrap();
            let (repo, commit) = initial(&f, local.path(), work.path()).await;
            let read = pin(&f, oid(repo.op_id())).await;
            let graph = executor(cache.path(), q::Limits::default());
            let content = q::content::ContentReader::new(q::content::Limits::default(), 1).unwrap();
            for use_graph in [true, false] {
                let store = Arc::new(StallingStore {
                    inner: f.store.clone(),
                    entered: Default::default(),
                    released: Default::default(),
                });
                let job = {
                    let store = store.clone();
                    let graph = graph.clone();
                    let content = content.clone();
                    let access = read.clone();
                    let commit = cid(commit.id());
                    tokio::spawn(async move {
                        if use_graph {
                            graph
                                .execute(store, access, log("all()"), page(10))
                                .await
                                .map(|_| ())
                        } else {
                            content
                                .object(store, access, m::ObjectKey::Commit(commit))
                                .await
                                .map(|_| ())
                        }
                    })
                };
                tokio::time::timeout(Duration::from_secs(2), store.entered.notified())
                    .await
                    .unwrap();
                assert!(!store.released.load(std::sync::atomic::Ordering::SeqCst));
                job.abort();
                assert!(job.await.unwrap_err().is_cancelled());
                let deadline = tokio::time::Instant::now() + Duration::from_secs(1);
                while !store.released.load(std::sync::atomic::Ordering::SeqCst) {
                    assert!(
                        tokio::time::Instant::now() < deadline,
                        "canceled query retained a stalled remote body"
                    );
                    tokio::time::sleep(Duration::from_millis(5)).await;
                }
            }
            assert_eq!(
                f.session.snapshot().await.unwrap().manifest().heads,
                BTreeSet::from([oid(repo.op_id())])
            );
        })
        .await;
}
