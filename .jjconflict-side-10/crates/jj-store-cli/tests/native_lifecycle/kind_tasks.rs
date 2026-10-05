use super::*;
use jj_store_service::{index_tasks as graph_tasks, kind_tasks as tasks};

async fn terminal(
    manager: &tasks::KindTasks,
    session: Arc<NativeSession>,
    id: &str,
) -> tasks::Task {
    tokio::time::timeout(Duration::from_secs(3), async {
        loop {
            let task = manager.status(session.clone(), id).await.unwrap();
            if !matches!(task.state, tasks::State::Running | tasks::State::Canceling) {
                return task;
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_kind_tasks_prepare_exact_selection_and_report_bounded_progress() {
    let f = ContentFixture::new().await;
    let workers = q::workers::Workers::new(1).unwrap();
    let manager = tasks::KindTasks::new(workers.clone(), tasks::Limits::default()).unwrap();
    let snapshot = f.read.snapshot().await.unwrap().clone();
    let before = f.f.session.snapshot().await.unwrap().manifest().clone();
    let task = manager
        .start(
            f.f.session.clone(),
            snapshot.operation_id,
            tasks::Kind::Operations,
        )
        .await
        .unwrap();
    assert_eq!(task.snapshot, snapshot);
    let done = terminal(&manager, f.f.session.clone(), &task.id).await;
    assert!(
        matches!(done.state, tasks::State::Ready),
        "{:?}",
        done.state
    );
    assert!(done.progress.complete);
    assert!(done.progress.matching_ids >= 2);
    assert!(done.progress.stats.windows >= 1);
    assert_eq!(
        done.progress.stats.scanned,
        done.progress.stats.descriptor_work
    );
    assert_eq!(workers.status().in_use, 0);
    assert_eq!(*f.f.session.snapshot().await.unwrap().manifest(), before);
    let restarted = tasks::KindTasks::new(workers, tasks::Limits::default()).unwrap();
    assert_eq!(
        restarted
            .status(f.f.session.clone(), &task.id)
            .await
            .unwrap_err(),
        graph_tasks::Error::Reset
    );
    let report = serde_json::to_value(done).unwrap();
    assert!(report.get("ids").is_none() && report.get("credentials").is_none());
}

struct GatedStore {
    inner: Arc<MemoryStore>,
    target: String,
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
impl jj_store_object::ObjectStore for GatedStore {
    async fn head(&self, key: &Key) -> jj_store_object::Result<jj_store_object::Metadata> {
        self.inner.head(key).await
    }
    async fn get(
        &self,
        key: &Key,
        range: Option<ByteRange>,
    ) -> jj_store_object::Result<jj_store_object::Download> {
        let mut result = self.inner.get(key, range).await?;
        if key.as_str() == self.target {
            result.reader = Box::pin(PendingBody(self.released.clone()));
            self.entered.notify_one();
        }
        Ok(result)
    }
    async fn put(
        &self,
        _: &Key,
        _: Condition,
        _: &StagedUpload,
    ) -> jj_store_object::Result<WriteOutcome> {
        panic!("kind task published native state")
    }
}
async fn gated(f: &ContentFixture) -> (Arc<GatedStore>, Arc<NativeSession>) {
    use sha2::{Digest as _, Sha256};
    let key = m::ObjectKey::File(f.file);
    let mut hash = Sha256::new_with_prefix(b"jj-store/object/v1");
    hash.update([key.kind() as u8]);
    hash.update(key.as_bytes());
    let root = f.f.session.snapshot().await.unwrap().manifest().inventory;
    let reference = wal::catalog::Catalog::new(wal::records::Records::new(f.f.store.clone(), REPO))
        .get(root, m::RecordDigest::new(hash.finalize().into()))
        .await
        .unwrap()
        .unwrap();
    let gate = Arc::new(GatedStore {
        inner: f.f.store.clone(),
        target: reference.key(REPO).unwrap().as_str().to_owned(),
        entered: Default::default(),
        released: Default::default(),
    });
    let session = Arc::new(
        NativeSession::new(
            gate.clone(),
            Arc::new(NativeObjects::new(gate.clone(), REPO, 4 * 1024 * 1024).unwrap()),
            f.f.reader.clone(),
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
        )
        .unwrap(),
    );
    (gate, session)
}
async fn entered(gate: &GatedStore) {
    tokio::time::timeout(Duration::from_secs(2), gate.entered.notified())
        .await
        .unwrap();
}
async fn released(workers: &q::workers::Workers, gate: &GatedStore) {
    tokio::time::timeout(Duration::from_secs(2), async {
        while workers.status().in_use != 0 {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    assert!(gate.released.load(std::sync::atomic::Ordering::SeqCst));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_kind_tasks_share_graph_capacity_coalesce_cancel_and_drop() {
    let f = ContentFixture::new().await;
    let root = tempfile::tempdir().unwrap();
    let shared = q::workers::Executors::new(
        root.path().to_owned(),
        q::Limits::default(),
        c::Limits::default(),
        q::cache::Policy::default(),
        1,
    )
    .unwrap();
    let manager =
        tasks::KindTasks::new(shared.workers().clone(), tasks::Limits::default()).unwrap();
    let (gate, session) = gated(&f).await;
    let operation = f.read.snapshot().await.unwrap().operation_id;
    let task = manager
        .start(session.clone(), operation, tasks::Kind::Operations)
        .await
        .unwrap();
    entered(&gate).await;
    assert_eq!(shared.workers().status().in_use, 1);
    assert_eq!(
        manager
            .start(session.clone(), operation, tasks::Kind::Operations)
            .await
            .unwrap()
            .id,
        task.id
    );
    assert_eq!(
        manager
            .start(session.clone(), operation, tasks::Kind::Copies)
            .await
            .unwrap_err(),
        graph_tasks::Error::Busy
    );
    assert_eq!(
        f.read
            .prepare_index(shared.graph(), q::Cancellation::default())
            .await
            .unwrap_err(),
        q::Error::Busy
    );
    let file = f.read.file(shared.content(), f.file, None).await.unwrap();
    assert_eq!(shared.workers().status().in_use, 2);
    drop(file);
    assert_eq!(
        manager
            .cancel(f.f.session.clone(), &task.id)
            .await
            .unwrap_err(),
        graph_tasks::Error::Denied
    );
    assert!(matches!(
        manager
            .cancel(session.clone(), &task.id)
            .await
            .unwrap()
            .state,
        tasks::State::Canceling
    ));
    let done = terminal(&manager, session.clone(), &task.id).await;
    assert!(matches!(done.state, tasks::State::Canceled));
    assert!(!done.progress.complete);
    assert_eq!(done.progress.stats.scanned, 0);
    assert_eq!(done.progress.stats.descriptor_work, 256);
    released(shared.workers(), &gate).await;
    manager
        .start(session, operation, tasks::Kind::Operations)
        .await
        .unwrap();
    entered(&gate).await;
    drop(manager);
    released(shared.workers(), &gate).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_kind_tasks_expire_pending_io_without_polling_and_bound_retention() {
    let f = ContentFixture::new().await;
    let workers = q::workers::Workers::new(1).unwrap();
    let manager = tasks::KindTasks::new(
        workers.clone(),
        tasks::Limits {
            ttl: Duration::from_millis(20),
            preparation: jj_store_service::native_inventory::preparation::Limits {
                timeout: Duration::from_millis(250),
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .unwrap();
    let (gate, session) = gated(&f).await;
    let operation = f.read.snapshot().await.unwrap().operation_id;
    let task = manager
        .start(session.clone(), operation, tasks::Kind::Copies)
        .await
        .unwrap();
    entered(&gate).await;
    tokio::time::sleep(Duration::from_millis(50)).await;
    assert!(matches!(
        manager
            .status(session.clone(), &task.id)
            .await
            .unwrap()
            .state,
        tasks::State::Running
    ));
    released(&workers, &gate).await;
    tokio::time::sleep(Duration::from_millis(30)).await;
    assert_eq!(
        manager.status(session, &task.id).await.unwrap_err(),
        graph_tasks::Error::Reset
    );

    let limited = tasks::KindTasks::new(
        workers.clone(),
        tasks::Limits {
            retained: 1,
            ttl: Duration::from_millis(100),
            preparation: jj_store_service::native_inventory::preparation::Limits {
                max_record_reads: 1,
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .unwrap();
    let task = limited
        .start(f.f.session.clone(), operation, tasks::Kind::Operations)
        .await
        .unwrap();
    assert!(matches!(
        terminal(&limited, f.f.session.clone(), &task.id)
            .await
            .state,
        tasks::State::Failed {
            error: q::Error::IndexBudget
        }
    ));
    assert_eq!(
        limited
            .start(f.f.session.clone(), operation, tasks::Kind::Copies)
            .await
            .unwrap_err(),
        graph_tasks::Error::Busy
    );
    tokio::time::sleep(Duration::from_millis(150)).await;
    assert_eq!(
        limited
            .status(f.f.session.clone(), &task.id)
            .await
            .unwrap_err(),
        graph_tasks::Error::Reset
    );
    assert_eq!(workers.status().in_use, 0);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_kind_task_reports_require_current_access_after_revocation() {
    use jj_store_service::administration::{TokenChange, TokenRequest};
    let f = ContentFixture::new().await;
    let workers = q::workers::Workers::new(1).unwrap();
    let manager = tasks::KindTasks::new(workers.clone(), tasks::Limits::default()).unwrap();
    let (gate, session) = gated(&f).await;
    let operation = f.read.snapshot().await.unwrap().operation_id;
    let task = manager
        .start(session.clone(), operation, tasks::Kind::Operations)
        .await
        .unwrap();
    entered(&gate).await;
    let before = f.f.session.snapshot().await.unwrap().manifest().clone();
    f.f.session
        .change_token(&TokenRequest {
            request_id: m::RequestId::new([207; 16]),
            expected: before.snapshot(),
            change: TokenChange::Revoke {
                token: f.f.reader.id(),
            },
        })
        .await
        .unwrap();
    assert_eq!(
        manager.status(session.clone(), &task.id).await.unwrap_err(),
        graph_tasks::Error::Denied
    );
    assert_eq!(
        manager.cancel(session, &task.id).await.unwrap_err(),
        graph_tasks::Error::Denied
    );
    drop(manager);
    released(&workers, &gate).await;
}
