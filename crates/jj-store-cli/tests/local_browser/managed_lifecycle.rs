use super::{ReadBinding, jj};
use jj_store_jj::{local_browser as q, query as native};
use jj_store_service::code_indexes::{Indexes, State};
use std::{sync::Arc, time::Duration};

fn manager(root: &std::path::Path) -> Arc<Indexes> {
    let workers = native::workers::Workers::new(2).unwrap();
    Indexes::new(
        native::GraphExecutor::with_workers(
            root.into(),
            Default::default(),
            Default::default(),
            workers.clone(),
        )
        .unwrap(),
        q::Executor::with_workers(workers, Default::default()),
        root.into(),
        native::cache::Policy {
            max_entry_bytes: 16 * 1024 * 1024,
            max_repository_bytes: 32 * 1024 * 1024,
            max_total_bytes: 32 * 1024 * 1024,
            ..Default::default()
        },
    )
}
async fn settled(
    indexes: &Indexes,
    task: &jj_store_service::code_indexes::Task,
) -> jj_store_service::code_indexes::Task {
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            let report = indexes.inspect(&task.key, &task.id, false).unwrap();
            if !matches!(report.state, State::Running | State::Canceling) {
                return report;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap()
}
fn log() -> q::Request {
    q::Request::Graph(
        q::GraphRequest::Log {
            revset: "all()".into(),
            workspace: None,
            user_email: String::new(),
            at_time_millis: 0,
        },
        q::GraphWindow {
            limit: 5,
            after: None,
        },
    )
}
#[tokio::test]
async fn managed_owner_coalesces_resumes_and_keeps_old_selection_during_updates() {
    let source = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    jj(source.path(), &["new", "-m", "First"]);
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(source.path().join(".jj/repo")));
    let reader = q::Executor::new(1).unwrap();
    let first = reader.heads(access.clone()).await.unwrap().heads[0];
    let indexes = manager(cache.path());
    let task = indexes.start_local(access.clone(), first).await.unwrap();
    assert_eq!(
        indexes
            .maintain_local(access.clone(), first)
            .await
            .unwrap()
            .id,
        task.id
    );
    assert!(matches!(
        settled(&indexes, &task).await.state,
        State::Ready(_)
    ));
    let q::Response::Graph(before) = indexes
        .local_read(access.clone(), first, log())
        .await
        .unwrap()
    else {
        panic!()
    };
    jj(source.path(), &["new", "-m", "Second"]);
    let second = reader.heads(access.clone()).await.unwrap().heads[0];
    let next = indexes
        .maintain_local(access.clone(), second)
        .await
        .unwrap();
    let q::Response::Graph(during) = indexes
        .local_read(access.clone(), first, log())
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(before.data, during.data);
    assert!(matches!(
        settled(&indexes, &next).await.state,
        State::Ready(_)
    ));
    assert!(
        indexes
            .local_read(access.clone(), second, log())
            .await
            .is_ok()
    );
    assert_eq!(
        indexes.retained().await.unwrap().len(),
        1,
        "bookmarks/operations share one namespace"
    );
    drop(indexes);
    let restarted = manager(cache.path());
    assert_eq!(restarted.retained().await.unwrap().len(), 1);
    assert!(matches!(
        restarted
            .inspect_local(access.clone(), &task.id, false)
            .await,
        Err(jj_store_service::local_index_tasks::Error::Reset)
    ));
    let task = restarted.start_local(access.clone(), second).await.unwrap();
    let State::Ready(stats) = settled(&restarted, &task).await.state else {
        panic!()
    };
    assert!(
        stats.metadata_nodes < 16,
        "restart reuses JJ associations: {stats:?}"
    );
    restarted.invalidate(access.repository());
    assert!(matches!(
        restarted.inspect(&task.key, &task.id, false).unwrap().state,
        State::Canceled
    ));
}

#[tokio::test]
async fn managed_background_updates_coalesce_but_preserve_foreground_requests() {
    let source = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    let reader = q::Executor::new(1).unwrap();
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(source.path().join(".jj/repo")));
    let mut operations = Vec::new();
    for _ in 0..3 {
        jj(source.path(), &["new", "-m", "Update"]);
        operations.push(reader.heads(access.clone()).await.unwrap().heads[0]);
    }
    let indexes = manager(cache.path());
    let first = indexes
        .maintain_local(access.clone(), operations[0])
        .await
        .unwrap();
    let middle = indexes
        .maintain_local(access.clone(), operations[1])
        .await
        .unwrap();
    let last = indexes
        .maintain_local(access.clone(), operations[2])
        .await
        .unwrap();
    assert!(matches!(
        indexes.inspect(&first.key, &first.id, false).unwrap().state,
        State::Canceled
    ));
    assert!(matches!(
        indexes
            .inspect(&middle.key, &middle.id, false)
            .unwrap()
            .state,
        State::Canceled
    ));
    assert!(matches!(
        settled(&indexes, &last).await.state,
        State::Ready(_)
    ));
    let selected = indexes
        .start_local(access.clone(), operations[0])
        .await
        .unwrap();
    indexes
        .maintain_local(access.clone(), operations[1])
        .await
        .unwrap();
    assert!(matches!(
        settled(&indexes, &selected).await.state,
        State::Ready(_)
    ));
}

struct Gate {
    binding: ReadBinding,
    calls: std::sync::atomic::AtomicUsize,
    pending: std::sync::atomic::AtomicUsize,
    entered: tokio::sync::Notify,
}
#[async_trait::async_trait]
impl q::Access for Gate {
    fn repository(&self) -> jj_store_model::RepositoryId {
        q::Access::repository(&self.binding)
    }
    fn generation(&self) -> u64 {
        1
    }
    fn metadata_path(&self) -> &std::path::Path {
        q::Access::metadata_path(&self.binding)
    }
    async fn authorize(&self) -> q::Result<()> {
        use std::sync::atomic::Ordering::SeqCst;
        if self.calls.fetch_add(1, SeqCst) == 0 {
            return Ok(());
        }
        self.pending.fetch_add(1, SeqCst);
        struct Waiting<'a>(&'a std::sync::atomic::AtomicUsize);
        impl Drop for Waiting<'_> {
            fn drop(&mut self) {
                self.0.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
            }
        }
        let _waiting = Waiting(&self.pending);
        self.entered.notify_one();
        std::future::pending().await
    }
}
#[tokio::test]
async fn managed_cancel_and_server_teardown_release_pending_authorization() {
    let source = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    let metadata = source.path().join(".jj/repo");
    let operation = q::Executor::new(1)
        .unwrap()
        .heads(Arc::new(ReadBinding(metadata.clone())))
        .await
        .unwrap()
        .heads[0];
    for teardown in [false, true] {
        let cache = tempfile::tempdir().unwrap();
        let indexes = manager(cache.path());
        let gate = Arc::new(Gate {
            binding: ReadBinding(metadata.clone()),
            calls: 0.into(),
            pending: 0.into(),
            entered: Default::default(),
        });
        let task = indexes
            .maintain_local(gate.clone(), operation)
            .await
            .unwrap();
        tokio::time::timeout(Duration::from_secs(2), gate.entered.notified())
            .await
            .unwrap();
        if teardown {
            drop(indexes);
        } else {
            assert!(matches!(
                indexes.inspect(&task.key, &task.id, true).unwrap().state,
                State::Canceling
            ));
            assert!(matches!(
                settled(&indexes, &task).await.state,
                State::Canceled
            ));
        }
        tokio::time::timeout(Duration::from_secs(2), async {
            while gate.pending.load(std::sync::atomic::Ordering::SeqCst) != 0 {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        assert_eq!(std::fs::read_dir(cache.path()).unwrap().count(), 0);
    }
}

struct CurrentBinding {
    source: ReadBinding,
    allowed: std::sync::atomic::AtomicBool,
    generation: std::sync::atomic::AtomicU64,
}
#[async_trait::async_trait]
impl q::Access for CurrentBinding {
    fn repository(&self) -> jj_store_model::RepositoryId {
        q::Access::repository(&self.source)
    }
    fn generation(&self) -> u64 {
        self.generation.load(std::sync::atomic::Ordering::SeqCst)
    }
    fn metadata_path(&self) -> &std::path::Path {
        q::Access::metadata_path(&self.source)
    }
    async fn authorize(&self) -> q::Result<()> {
        if self.allowed.load(std::sync::atomic::Ordering::SeqCst) {
            Ok(())
        } else {
            Err(q::Error::Denied)
        }
    }
}
#[tokio::test]
async fn managed_reads_and_controls_reject_revoked_or_changed_registration() {
    use std::sync::atomic::Ordering::SeqCst;
    let source = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    let binding = Arc::new(CurrentBinding {
        source: ReadBinding(source.path().join(".jj/repo")),
        allowed: true.into(),
        generation: 1.into(),
    });
    let operation = q::Executor::new(1)
        .unwrap()
        .heads(binding.clone())
        .await
        .unwrap()
        .heads[0];
    let indexes = manager(cache.path());
    let task = indexes
        .start_local(binding.clone(), operation)
        .await
        .unwrap();
    assert!(matches!(
        settled(&indexes, &task).await.state,
        State::Ready(_)
    ));
    binding.allowed.store(false, SeqCst);
    assert!(matches!(
        indexes.local_read(binding.clone(), operation, log()).await,
        Err(q::Error::Denied)
    ));
    assert!(matches!(
        indexes.inspect_local(binding.clone(), &task.id, true).await,
        Err(jj_store_service::local_index_tasks::Error::Query(
            q::Error::Denied
        ))
    ));
    binding.allowed.store(true, SeqCst);
    binding.generation.store(2, SeqCst);
    assert!(matches!(
        indexes
            .inspect_local(binding.clone(), &task.id, false)
            .await,
        Err(jj_store_service::local_index_tasks::Error::Query(
            q::Error::EpochChanged
        ))
    ));
    let next = indexes
        .start_local(binding.clone(), operation)
        .await
        .unwrap();
    assert_ne!(next.key, task.key);
    assert!(matches!(
        indexes.inspect(&task.key, &task.id, false).unwrap().state,
        State::Canceled
    ));
    assert!(matches!(
        settled(&indexes, &next).await.state,
        State::Ready(_)
    ));
}

#[tokio::test]
async fn managed_stack_history_starts_its_operation_index() {
    let source = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    jj(source.path(), &["new", "-m", "Stack head"]);
    let head = jj(
        source.path(),
        &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
    )
    .parse()
    .unwrap();
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(source.path().join(".jj/repo")));
    let reader = q::Executor::new(1).unwrap();
    let operation = reader.heads(access.clone()).await.unwrap().heads[0];
    let indexes = manager(cache.path());
    let request = q::Request::StackHistory(q::stacks::Request {
        head,
        base: None,
        limit: 1,
        after: None,
    });
    assert!(matches!(
        indexes
            .local_read(access.clone(), operation, request.clone())
            .await,
        Err(q::Error::IndexRebuildRequired)
    ));
    let response = tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            match indexes
                .local_read(access.clone(), operation, request.clone())
                .await
            {
                Ok(response) => break response,
                Err(q::Error::IndexRebuildRequired | q::Error::Busy) => {
                    tokio::time::sleep(Duration::from_millis(10)).await
                }
                Err(error) => panic!("unexpected preparation error: {error:?}"),
            }
        }
    })
    .await
    .unwrap();
    let q::Response::StackHistory(page) = response else {
        panic!("stack history")
    };
    assert_eq!(page.commits[0].commit.id, head);
    assert!(page.next.is_some());
}
