use super::*;
use jj_store_service::index_tasks as tasks;

async fn terminal(
    manager: &tasks::IndexTasks,
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
fn shared(root: &std::path::Path) -> q::workers::Executors {
    q::workers::Executors::new(
        root.to_owned(),
        q::Limits {
            max_inline_rebuild_commits: 0,
            ..q::Limits::default()
        },
        c::Limits::default(),
        q::cache::Policy::default(),
        1,
    )
    .unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn index_tasks_report_pinned_completion_restart_and_current_revocation() {
    use jj_store_service::administration::{TokenChange, TokenRequest};
    let f = ContentFixture::new().await;
    let root = tempfile::tempdir().unwrap();
    let shared = shared(root.path());
    let manager = tasks::IndexTasks::new(shared.graph().clone(), tasks::Limits::default()).unwrap();
    let session = f.f.session(f.f.reader.clone(), Protection::default());
    let snapshot = f.read.snapshot().await.unwrap().clone();
    let before = f.f.session.snapshot().await.unwrap().manifest().clone();
    let task = manager
        .start(session.clone(), snapshot.operation_id)
        .await
        .unwrap();
    assert_eq!(task.snapshot, snapshot);
    let done = terminal(&manager, session.clone(), &task.id).await;
    let tasks::State::Ready { index } = done.state else {
        panic!("not ready: {:?}", done.state);
    };
    assert_eq!(index.operation, snapshot.operation_id);
    assert_eq!(index.cache.state, q::cache::State::Built);
    let result = f
        .read
        .graph(
            shared.graph(),
            q::Request::Log {
                revset: "all()".into(),
                workspace: None,
                user_email: String::new(),
                at_time_millis: 0,
            },
            q::Page {
                limit: 10,
                after: None,
            },
        )
        .await
        .unwrap();
    assert_eq!(result.cache.state, q::cache::State::Reused);
    assert_eq!(*f.f.session.snapshot().await.unwrap().manifest(), before);
    // A valid token for a different repository cannot inspect/cancel this task,
    // even when it authenticates the same principal as the original reader.
    let other_repository = m::RepositoryId::new([219; 16]);
    let objects =
        Arc::new(NativeObjects::new(f.f.store.clone(), other_repository, 4 * 1024 * 1024).unwrap());
    let mut policy = TokenPolicy::new(Scope::Repository(other_repository));
    let token = policy
        .issue(m::PrincipalId::new([3; 16]), Permission::Admin, None)
        .unwrap();
    let mut operator = TokenPolicy::new(Scope::Operator);
    let operator_token = operator
        .issue(ACTOR, Permission::CreateRepository, None)
        .unwrap();
    create_repository(
        f.f.store.clone(),
        objects.clone(),
        &operator,
        &operator_token,
        &policy,
        m::EpochId::new([220; 16]),
        &SystemClock,
        Limits::default(),
    )
    .await
    .unwrap();
    let other = Arc::new(
        NativeSession::new(
            f.f.store.clone(),
            objects,
            token,
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
        )
        .unwrap(),
    );
    assert_eq!(
        manager.status(other.clone(), &task.id).await.unwrap_err(),
        tasks::Error::Denied
    );
    assert_eq!(
        manager.cancel(other, &task.id).await.unwrap_err(),
        tasks::Error::Denied
    );
    let restarted =
        tasks::IndexTasks::new(shared.graph().clone(), tasks::Limits::default()).unwrap();
    assert_eq!(
        restarted
            .status(session.clone(), &task.id)
            .await
            .unwrap_err(),
        tasks::Error::Reset
    );
    f.f.session
        .change_token(&TokenRequest {
            request_id: m::RequestId::new([222; 16]),
            expected: before.snapshot(),
            change: TokenChange::Revoke {
                token: f.f.reader.id(),
            },
        })
        .await
        .unwrap();
    assert_eq!(
        manager.status(session.clone(), &task.id).await.unwrap_err(),
        tasks::Error::Denied
    );
    assert_eq!(
        manager.cancel(session, &task.id).await.unwrap_err(),
        tasks::Error::Denied
    );
}

struct GatedStore {
    inner: Arc<MemoryStore>,
    target: String,
    entered: tokio::sync::Notify,
    released: Arc<std::sync::atomic::AtomicBool>,
}
struct WaitingBody(Arc<std::sync::atomic::AtomicBool>);
impl Drop for WaitingBody {
    fn drop(&mut self) {
        self.0.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}
impl tokio::io::AsyncRead for WaitingBody {
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
            result.reader = Box::pin(WaitingBody(self.released.clone()));
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
        panic!("index task published native state")
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn index_preparation_reserves_interactive_capacity() {
    let f = ContentFixture::new().await;
    let root = tempfile::tempdir().unwrap();
    let shared = shared(root.path());
    let gate = Arc::new(GatedStore {
        inner: f.f.store.clone(),
        target: f.commit.key().storage_path(REPO),
        entered: Default::default(),
        released: Default::default(),
    });
    let cancellation = q::Cancellation::default();
    let job = {
        let graph = shared.graph().clone();
        let access = f.read.clone();
        let gate = gate.clone();
        let cancellation = cancellation.clone();
        tokio::spawn(async move { graph.prepare_index(gate, access, cancellation).await })
    };
    tokio::time::timeout(Duration::from_secs(2), gate.entered.notified())
        .await
        .unwrap();
    // A second cache namespace must not acquire another background slot.
    let other_root = tempfile::tempdir().unwrap();
    let other = q::GraphExecutor::with_workers(
        other_root.path().to_owned(),
        q::Limits::default(),
        q::cache::Policy::default(),
        shared.workers().clone(),
    )
    .unwrap();
    let second = f
        .read
        .prepare_index(&other, q::Cancellation::default())
        .await;
    let file = f.read.file(shared.content(), f.file, None).await;
    let active = shared.workers().status().in_use;
    let file_ok = file.is_ok();
    drop(file);
    let operation = f
        .read
        .graph(
            shared.graph(),
            q::Request::Operations,
            q::Page {
                limit: 1,
                after: None,
            },
        )
        .await;
    // Always release the blocked worker before assertions, including the baseline.
    cancellation.cancel();
    let stopped = job.await.unwrap();
    assert_eq!(second.unwrap_err(), q::Error::Busy);
    assert!(file_ok, "preparation starved the interactive file reader");
    assert_eq!(active, 2);
    operation.unwrap();
    assert_eq!(stopped.unwrap_err(), q::Error::Canceled);
    assert!(gate.released.load(std::sync::atomic::Ordering::SeqCst));
    assert_eq!(shared.workers().status().in_use, 0);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn index_tasks_deduplicate_bound_cancel_and_drop_running_jobs() {
    let f = ContentFixture::new().await;
    let root = tempfile::tempdir().unwrap();
    let shared = shared(root.path());
    let manager = tasks::IndexTasks::new(
        shared.graph().clone(),
        tasks::Limits {
            running: 1,
            ..tasks::Limits::default()
        },
    )
    .unwrap();
    let gate = Arc::new(GatedStore {
        inner: f.f.store.clone(),
        target: f.commit.key().storage_path(REPO),
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
    let operation = f.read.snapshot().await.unwrap().operation_id;
    let task = manager.start(session.clone(), operation).await.unwrap();
    tokio::time::timeout(Duration::from_secs(2), gate.entered.notified())
        .await
        .unwrap();
    assert_eq!(shared.workers().status().in_use, 1);
    assert_eq!(
        manager.start(session.clone(), operation).await.unwrap().id,
        task.id
    );
    assert_eq!(
        manager
            .start(session.clone(), roots::ROOT_OPERATION)
            .await
            .unwrap_err(),
        tasks::Error::Busy
    );
    assert_eq!(
        manager
            .cancel(f.f.session.clone(), &task.id)
            .await
            .unwrap_err(),
        tasks::Error::Denied
    );
    assert!(matches!(
        manager
            .cancel(session.clone(), &task.id)
            .await
            .unwrap()
            .state,
        tasks::State::Canceling
    ));
    assert!(matches!(
        terminal(&manager, session.clone(), &task.id).await.state,
        tasks::State::Canceled
    ));
    assert!(gate.released.load(std::sync::atomic::Ordering::SeqCst));
    assert_eq!(shared.workers().status().in_use, 0);
    manager.start(session, operation).await.unwrap();
    tokio::time::timeout(Duration::from_secs(2), gate.entered.notified())
        .await
        .unwrap();
    drop(manager);
    tokio::time::timeout(Duration::from_secs(1), async {
        while shared.workers().status().in_use != 0 {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn index_tasks_retention_expiry_and_failures_are_explicit() {
    let f = ContentFixture::new().await;
    let root = tempfile::tempdir().unwrap();
    let shared = shared(root.path());
    let manager = tasks::IndexTasks::new(
        shared.graph().clone(),
        tasks::Limits {
            retained: 1,
            ttl: Duration::from_secs(2),
            ..tasks::Limits::default()
        },
    )
    .unwrap();
    let operation = f.read.snapshot().await.unwrap().operation_id;
    let task = manager.start(f.f.session.clone(), operation).await.unwrap();
    terminal(&manager, f.f.session.clone(), &task.id).await;
    assert_eq!(
        manager
            .start(f.f.session.clone(), operation)
            .await
            .unwrap_err(),
        tasks::Error::Busy
    );
    tokio::time::sleep(Duration::from_millis(2100)).await;
    assert_eq!(
        manager
            .status(f.f.session.clone(), &task.id)
            .await
            .unwrap_err(),
        tasks::Error::Reset
    );
    manager.start(f.f.session.clone(), operation).await.unwrap();
    let small = q::GraphExecutor::new(
        root.path().to_owned(),
        q::Limits {
            max_index_commits: 1,
            ..q::Limits::default()
        },
        1,
    )
    .unwrap();
    let limited = tasks::IndexTasks::new(small, tasks::Limits::default()).unwrap();
    let task = limited.start(f.f.session.clone(), operation).await.unwrap();
    assert!(matches!(
        terminal(&limited, f.f.session.clone(), &task.id)
            .await
            .state,
        tasks::State::Failed {
            error: q::Error::IndexBudget
        }
    ));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn index_tasks_expiry_cancels_pending_storage_without_status_polling() {
    let f = ContentFixture::new().await;
    let root = tempfile::tempdir().unwrap();
    let shared = shared(root.path());
    let manager = tasks::IndexTasks::new(
        shared.graph().clone(),
        tasks::Limits {
            ttl: Duration::from_secs(1),
            ..tasks::Limits::default()
        },
    )
    .unwrap();
    let gate = Arc::new(GatedStore {
        inner: f.f.store.clone(),
        target: f.commit.key().storage_path(REPO),
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
    let operation = f.read.snapshot().await.unwrap().operation_id;
    let task = manager.start(session.clone(), operation).await.unwrap();
    tokio::time::timeout(Duration::from_secs(2), gate.entered.notified())
        .await
        .unwrap();
    tokio::time::timeout(Duration::from_secs(2), async {
        while shared.workers().status().in_use != 0 {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    assert!(gate.released.load(std::sync::atomic::Ordering::SeqCst));
    assert_eq!(
        manager.status(session, &task.id).await.unwrap_err(),
        tasks::Error::Reset
    );
}
