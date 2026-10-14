use super::{ReadBinding, jj};
use jj_store_jj::{
    local_browser as q,
    query::{cache, workers::Workers},
};
use jj_store_service::local_index_tasks::{self as tasks, IndexTasks, LocalIndexTask, State};
use std::{
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    time::Duration,
};

fn operation(path: &Path) -> jj_store_model::OperationId {
    jj(path, &["op", "log", "--no-graph", "-n", "1", "-T", "id"])
        .parse()
        .unwrap()
}
fn limits() -> tasks::Limits {
    tasks::Limits {
        cache: cache::Policy {
            max_entry_bytes: 16 * 1024 * 1024,
            max_repository_bytes: 16 * 1024 * 1024,
            max_total_bytes: 16 * 1024 * 1024,
            ..Default::default()
        },
        ..Default::default()
    }
}
fn request() -> q::GraphRequest {
    q::GraphRequest::Log {
        revset: "all()".into(),
        workspace: None,
        user_email: String::new(),
        at_time_millis: 0,
    }
}
fn window() -> q::GraphWindow {
    q::GraphWindow {
        limit: 1,
        after: None,
    }
}

#[tokio::test]
async fn local_interactive_authorization_waits_obey_initial_and_final_deadlines() {
    let source = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    for skip in [0, 1] {
        let workers = Workers::new(1).unwrap();
        let executor = q::Executor::with_workers(
            workers.clone(),
            q::Limits {
                timeout: Duration::from_millis(100),
                ..Default::default()
            },
        );
        let gate = Arc::new(Gate::new(source.path().join(".jj/repo"), 1));
        gate.skip.store(skip, Ordering::SeqCst);
        gate.armed.store(true, Ordering::SeqCst);
        let result = tokio::time::timeout(Duration::from_secs(2), executor.heads(gate))
            .await
            .unwrap();
        assert!(matches!(result, Err(q::Error::Limit)));
        assert_eq!(workers.status().in_use, 0);
    }
}
async fn settled(tasks: &IndexTasks, access: Arc<dyn q::Access>, id: &str) -> LocalIndexTask {
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            let task = tasks.status(access.clone(), id).await.unwrap();
            if !matches!(task.state, State::Running | State::Canceling) {
                return task;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap()
}

#[tokio::test]
async fn local_tasks_retain_exact_history_evict_under_quota_and_reset_after_restart() {
    let source = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    jj(source.path(), &["describe", "-m", "First"]);
    let first_op = operation(source.path());
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(source.path().join(".jj/repo")));
    let executor = q::Executor::new(1).unwrap();
    let tasks = IndexTasks::new(executor.clone(), cache.path().into(), limits()).unwrap();
    assert!(matches!(
        tasks
            .graph(access.clone(), first_op, request(), window())
            .await,
        Err(tasks::Error::Query(q::Error::IndexRebuildRequired))
    ));
    let first = tasks.start(access.clone(), first_op).await.unwrap();
    assert_eq!(
        tasks.start(access.clone(), first_op).await.unwrap().id,
        first.id
    );
    assert!(matches!(
        settled(&tasks, access.clone(), &first.id).await.state,
        State::Ready { .. }
    ));
    assert_eq!(
        tasks.start(access.clone(), first_op).await.unwrap().id,
        first.id
    );
    jj(source.path(), &["new", "-m", "Later"]);
    let later_op = operation(source.path());
    let page = tasks
        .graph(access.clone(), first_op, request(), window())
        .await
        .unwrap();
    assert_eq!(page.operation, first_op);
    let q::Nodes::Log(rows) = page.data else {
        panic!()
    };
    assert_eq!(rows[0].commit.description, "First\n");
    let mut next = window();
    next.after = page.next;
    assert!(next.after.is_some());
    tasks
        .graph(access.clone(), first_op, request(), next)
        .await
        .unwrap();
    // Only one generation fits this deliberately small repository quota.
    let later = tasks.start(access.clone(), later_op).await.unwrap();
    assert!(matches!(
        settled(&tasks, access.clone(), &later.id).await.state,
        State::Ready { .. }
    ));
    assert!(matches!(
        tasks.status(access.clone(), &first.id).await,
        Err(tasks::Error::Reset)
    ));
    let q::Nodes::Log(rows) = tasks
        .graph(access.clone(), later_op, request(), window())
        .await
        .unwrap()
        .data
    else {
        panic!()
    };
    assert_eq!(rows[0].commit.description, "Later\n");
    let denied = Arc::new(Gate::new(access.metadata_path().into(), 1));
    denied.denied.store(true, Ordering::SeqCst);
    assert!(matches!(
        tasks.status(denied, &later.id).await,
        Err(tasks::Error::Query(q::Error::Denied))
    ));
    let changed = Arc::new(Gate::new(access.metadata_path().into(), 2));
    assert!(matches!(
        tasks.status(changed, &later.id).await,
        Err(tasks::Error::Query(q::Error::EpochChanged))
    ));
    drop(tasks);
    let restarted = IndexTasks::new(executor, cache.path().into(), limits()).unwrap();
    assert!(matches!(
        restarted.status(access.clone(), &later.id).await,
        Err(tasks::Error::Reset)
    ));
    assert!(matches!(
        restarted.graph(access, later_op, request(), window()).await,
        Err(tasks::Error::Query(q::Error::IndexRebuildRequired))
    ));
}

struct Gate {
    path: PathBuf,
    generation: u64,
    armed: AtomicBool,
    skip: AtomicUsize,
    denied: AtomicBool,
    entered: tokio::sync::Notify,
    release: tokio::sync::Notify,
}
impl Gate {
    fn new(path: PathBuf, generation: u64) -> Self {
        Self {
            path,
            generation,
            armed: false.into(),
            skip: 0.into(),
            denied: false.into(),
            entered: Default::default(),
            release: Default::default(),
        }
    }
}
#[async_trait::async_trait]
impl q::Access for Gate {
    fn repository(&self) -> jj_store_model::RepositoryId {
        jj_store_model::RepositoryId::new([20; 16])
    }
    fn generation(&self) -> u64 {
        self.generation
    }
    fn metadata_path(&self) -> &Path {
        &self.path
    }
    async fn authorize(&self) -> q::Result<()> {
        if self.armed.load(Ordering::SeqCst)
            && self
                .skip
                .try_update(Ordering::SeqCst, Ordering::SeqCst, |n| n.checked_sub(1))
                .is_err()
            && self.armed.swap(false, Ordering::SeqCst)
        {
            self.entered.notify_one();
            self.release.notified().await;
        }
        if self.denied.load(Ordering::SeqCst) {
            return Err(q::Error::Denied);
        }
        Ok(())
    }
}

#[tokio::test]
async fn local_task_cancellation_and_expiry_release_preparation_without_blocking_reads() {
    let source = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    let operation = operation(source.path());
    let path = source.path().join(".jj/repo");
    let plain: Arc<dyn q::Access> = Arc::new(ReadBinding(path.clone()));
    let workers = Workers::new(1).unwrap();
    let executor = q::Executor::with_workers(workers.clone(), Default::default());
    for expires in [false, true] {
        let mut policy = limits();
        if expires {
            policy.preparation.timeout = Duration::from_millis(150);
        }
        let tasks = IndexTasks::new(executor.clone(), cache.path().into(), policy).unwrap();
        let gate = Arc::new(Gate::new(path.clone(), 1));
        let task = tasks.start(gate.clone(), operation).await.unwrap();
        gate.armed.store(true, Ordering::SeqCst);
        tokio::time::timeout(Duration::from_secs(5), gate.entered.notified())
            .await
            .unwrap();
        assert_eq!(workers.status().in_use, 1);
        executor.heads(plain.clone()).await.unwrap();
        assert_eq!(
            tasks.start(plain.clone(), operation).await.unwrap().id,
            task.id
        );
        if expires {
            tokio::time::sleep(Duration::from_millis(250)).await;
            assert!(matches!(
                tasks.status(plain.clone(), &task.id).await,
                Err(tasks::Error::Reset)
            ));
        } else {
            assert!(matches!(
                tasks.cancel(plain.clone(), &task.id).await.unwrap().state,
                State::Canceling
            ));
            assert!(matches!(
                settled(&tasks, plain.clone(), &task.id).await.state,
                State::Canceled
            ));
        }
        tokio::time::timeout(Duration::from_secs(3), async {
            while workers.status().in_use != 0 {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .unwrap();
        drop(tasks);
        // Its full disk reservation is available again after actual exit.
        let reservation =
            cache::Reservation::acquire(cache.path(), plain.repository(), limits().cache).unwrap();
        drop(reservation);
    }
}

#[tokio::test]
async fn local_task_ready_expiry_releases_cache_without_polling() {
    let source = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    let operation = operation(source.path());
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(source.path().join(".jj/repo")));
    let mut policy = limits();
    policy.completed_ttl = Duration::from_millis(200);
    let tasks = IndexTasks::new(q::Executor::new(1).unwrap(), cache.path().into(), policy).unwrap();
    let task = tasks.start(access.clone(), operation).await.unwrap();
    assert!(matches!(
        settled(&tasks, access.clone(), &task.id).await.state,
        State::Ready { .. }
    ));
    assert!(matches!(
        cache::Reservation::acquire(cache.path(), access.repository(), policy.cache),
        Err(q::Error::Limit)
    ));
    tokio::time::sleep(Duration::from_millis(300)).await;
    let reservation =
        cache::Reservation::acquire(cache.path(), access.repository(), policy.cache).unwrap();
    assert!(matches!(
        tasks.status(access, &task.id).await,
        Err(tasks::Error::Reset)
    ));
    drop(reservation);
}

#[tokio::test]
async fn local_task_eviction_keeps_reservation_until_active_reader_exits() {
    let source = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    let first_op = operation(source.path());
    let path = source.path().join(".jj/repo");
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(path.clone()));
    let workers = Workers::new(2).unwrap();
    let executor = q::Executor::with_workers(workers.clone(), Default::default());
    let tasks = Arc::new(IndexTasks::new(executor, cache.path().into(), limits()).unwrap());
    let first = tasks.start(access.clone(), first_op).await.unwrap();
    assert!(matches!(
        settled(&tasks, access.clone(), &first.id).await.state,
        State::Ready { .. }
    ));
    jj(source.path(), &["new", "-m", "Later"]);
    let later_op = operation(source.path());
    let gate = Arc::new(Gate::new(path, 1));
    gate.skip.store(2, Ordering::SeqCst); // service and initial executor checks
    gate.armed.store(true, Ordering::SeqCst);
    let reader = {
        let tasks = tasks.clone();
        let gate = gate.clone();
        tokio::spawn(async move { tasks.graph(gate, first_op, request(), window()).await })
    };
    tokio::time::timeout(Duration::from_secs(3), gate.entered.notified())
        .await
        .unwrap();
    let later = tasks.start(access.clone(), later_op).await.unwrap();
    assert!(matches!(
        settled(&tasks, access.clone(), &later.id).await.state,
        State::Failed {
            error: q::Error::Limit
        }
    ));
    assert!(matches!(
        tasks.status(access.clone(), &first.id).await,
        Err(tasks::Error::Reset)
    ));
    assert!(matches!(
        cache::Reservation::acquire(cache.path(), access.repository(), limits().cache),
        Err(q::Error::Limit)
    ));
    reader.abort();
    let _ = reader.await;
    tokio::time::timeout(Duration::from_secs(3), async {
        while workers.status().in_use != 0 {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    let retry = tasks.start(access.clone(), later_op).await.unwrap();
    assert!(matches!(
        settled(&tasks, access, &retry.id).await.state,
        State::Ready { .. }
    ));
}
