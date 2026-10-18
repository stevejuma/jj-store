use std::{
    path::{Path, PathBuf},
    process::Command,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
    },
};
use tauri_plugin_jj_store::{
    Admission, Config, Context, Error, Handle, History, Result, Status, local,
};
use tokio::sync::Notify;

struct Shared {
    metadata: PathBuf,
    enabled: AtomicBool,
    generation: AtomicU64,
    delay: AtomicUsize,
    entered: Notify,
    release: Notify,
}
#[derive(Clone)]
struct Policy(Arc<Shared>);
struct Access {
    shared: Arc<Shared>,
    generation: u64,
}
#[async_trait::async_trait]
impl Admission for Policy {
    async fn admit(&self, caller: &Context, vault: &str) -> Result<Arc<dyn local::Access>> {
        if !["main", "popup"].contains(&caller.window.as_str())
            || caller.url != "app://fixture/"
            || !["vault", "alias"].contains(&vault)
        {
            return Err(Error::Denied);
        }
        Ok(Arc::new(Access {
            shared: self.0.clone(),
            generation: self.0.generation.load(Ordering::SeqCst),
        }))
    }
}
#[async_trait::async_trait]
impl local::Access for Access {
    fn repository(&self) -> jj_store_model::RepositoryId {
        jj_store_model::RepositoryId::new([65; 16])
    }
    fn generation(&self) -> u64 {
        self.generation
    }
    fn metadata_path(&self) -> &Path {
        &self.shared.metadata
    }
    async fn authorize(&self) -> Result<()> {
        if self
            .shared
            .delay
            .try_update(Ordering::SeqCst, Ordering::SeqCst, |n| n.checked_sub(1))
            == Ok(1)
        {
            self.shared.entered.notify_one();
            self.shared.release.notified().await;
        }
        if !self.shared.enabled.load(Ordering::SeqCst) {
            return Err(Error::Denied);
        }
        if self.shared.generation.load(Ordering::SeqCst) != self.generation {
            return Err(Error::EpochChanged);
        }
        if !self.shared.metadata.is_dir() {
            return Err(Error::Denied);
        }
        Ok(())
    }
}
fn caller(window: &str) -> Context {
    Context {
        window: window.into(),
        url: "app://fixture/".into(),
    }
}
fn jj(root: &Path, args: &[&str]) -> String {
    let program = std::env::var_os("JJ_STORE_TEST_STOCK_JJ").unwrap_or_else(|| "jj".into());
    let output = Command::new(program)
        .current_dir(root)
        .args([
            "--no-pager",
            "--config",
            "user.name='Fixture'",
            "--config",
            "user.email='fixture@invalid'",
        ])
        .args(args)
        .env("JJ_CONFIG", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim().into()
}
fn fixture() -> (tempfile::TempDir, Policy) {
    let root = tempfile::tempdir().unwrap();
    let version = jj(root.path(), &["--version"]);
    assert!(
        version == "jj 0.45.1" || version == "jj 0.45.1-7c41cdeb16b6b321c64e789a966b6adf723816a5"
    );
    jj(root.path(), &["git", "init", "--colocate"]);
    std::fs::write(root.path().join("note.md"), "history\n").unwrap();
    jj(root.path(), &["describe", "-m", "Saved"]);
    let policy = Policy(Arc::new(Shared {
        metadata: root.path().canonicalize().unwrap().join(".jj/repo"),
        enabled: AtomicBool::new(true),
        generation: AtomicU64::new(7),
        delay: AtomicUsize::new(0),
        entered: Notify::new(),
        release: Notify::new(),
    }));
    (root, policy)
}
async fn ready(host: &History, caller: &Context, handle: Handle) -> local::Snapshot {
    tokio::time::timeout(std::time::Duration::from_secs(15), async {
        loop {
            match host.status(caller, "vault", handle).await.unwrap() {
                Status::Preparing => tokio::task::yield_now().await,
                Status::Ready(snapshot) => break snapshot,
                Status::Failed(error) => panic!("{error}"),
            }
        }
    })
    .await
    .unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn handles_bind_window_vault_origin_generation_and_exact_operation() {
    let (_root, policy) = fixture();
    let cache = tempfile::tempdir().unwrap();
    let config = Config {
        max_handles: 1,
        ..Default::default()
    };
    let host = History::new(policy.clone(), cache.path().into(), config).unwrap();
    let main = caller("main");
    let operation = host.heads(&main, "vault").await.unwrap().heads[0];
    let handle = host.open(&main, "vault", operation).await.unwrap();
    assert_eq!(ready(&host, &main, handle).await.operation, operation);
    assert!(matches!(
        host.open(&main, "vault", operation).await,
        Err(Error::Busy)
    ));
    assert!(matches!(
        host.status(&caller("popup"), "vault", handle).await,
        Err(Error::Denied)
    ));
    assert!(matches!(
        host.status(&main, "alias", handle).await,
        Err(Error::Denied)
    ));
    let mut foreign = main.clone();
    foreign.url = "https://outside.invalid/".into();
    assert!(matches!(
        host.status(&foreign, "vault", handle).await,
        Err(Error::Denied)
    ));
    assert!(matches!(
        host.close(&caller("popup"), "vault", handle).await,
        Err(Error::Denied)
    ));
    let query = local::Request::Graph(
        local::GraphRequest::Log {
            revset: "all()".into(),
            workspace: None,
            user_email: "fixture@invalid".into(),
            at_time_millis: 0,
        },
        local::GraphWindow {
            limit: 8,
            after: None,
        },
    );
    let local::Response::Graph(page) = host.read(&main, "vault", handle, query).await.unwrap()
    else {
        panic!()
    };
    assert_eq!(page.operation, operation);
    let local::Nodes::Log(nodes) = page.data else {
        panic!()
    };
    let saved = &nodes
        .iter()
        .find(|row| row.commit.description == "Saved\n")
        .unwrap()
        .commit;
    let local::Response::Diff(diff) = host
        .read(
            &main,
            "vault",
            handle,
            local::Request::Diff(local::diff::Request {
                from: saved.parents[0],
                to: saved.id,
                path: "note.md".into(),
                content: true,
            }),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    let local::diff::Content::Text { hunks } = &diff.changes[0].content else {
        panic!()
    };
    assert_eq!(
        hunks.iter().map(|h| h.after.as_str()).collect::<String>(),
        "history\n"
    );

    policy.0.generation.store(8, Ordering::SeqCst);
    assert!(matches!(
        host.status(&main, "vault", handle).await,
        Err(Error::EpochChanged)
    ));
    policy.0.generation.store(7, Ordering::SeqCst);
    policy.0.enabled.store(false, Ordering::SeqCst);
    assert!(matches!(
        host.status(&main, "vault", handle).await,
        Err(Error::Denied)
    ));
    // Withdrawn read authority must never prevent owned cleanup.
    host.close(&main, "vault", handle).await.unwrap();
    host.shutdown().await.unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn close_cancels_and_waits_for_actual_preparation_exit() {
    let (_root, policy) = fixture();
    let cache = tempfile::tempdir().unwrap();
    let host = History::new(
        policy.clone(),
        cache.path().join("private"),
        Config::default(),
    )
    .unwrap();
    let main = caller("main");
    let operation = host.heads(&main, "vault").await.unwrap().heads[0];
    policy.0.delay.store(2, Ordering::SeqCst);
    let handle = host.open(&main, "vault", operation).await.unwrap();
    policy.0.entered.notified().await;
    let closing = {
        let host = host.clone();
        let main = main.clone();
        tokio::spawn(async move { host.close(&main, "vault", handle).await })
    };
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            if matches!(
                host.status(&main, "vault", handle).await,
                Err(Error::Denied)
            ) {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    assert!(
        !closing.is_finished(),
        "cleanup returned while a preparation worker was blocked"
    );
    policy.0.release.notify_one();
    closing.await.unwrap().unwrap();
    assert!(
        !cache.path().join("private").exists(),
        "canceled preflight created a cache"
    );
    let next = host.open(&main, "vault", operation).await.unwrap();
    ready(&host, &main, next).await;
    host.shutdown().await.unwrap();
    assert!(matches!(
        host.heads(&main, "vault").await,
        Err(Error::Denied)
    ));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn revoke_waits_for_inflight_reads_and_denies_their_completion() {
    let (_root, policy) = fixture();
    let cache = tempfile::tempdir().unwrap();
    let host = History::new(policy.clone(), cache.path().into(), Config::default()).unwrap();
    let main = caller("main");
    let operation = host.heads(&main, "vault").await.unwrap().heads[0];
    let handle = host.open(&main, "vault", operation).await.unwrap();
    ready(&host, &main, handle).await;
    policy.0.delay.store(2, Ordering::SeqCst);
    let read = {
        let host = host.clone();
        let main = main.clone();
        tokio::spawn(async move { host.status(&main, "vault", handle).await })
    };
    policy.0.entered.notified().await;
    let revoke = {
        let host = host.clone();
        tokio::spawn(async move { host.revoke("main").await })
    };
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            if matches!(host.heads(&main, "vault").await, Err(Error::Denied)) {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    assert!(
        !revoke.is_finished(),
        "revocation failed to drain its admitted reader"
    );
    let shutdown = {
        let host = host.clone();
        tokio::spawn(async move { host.shutdown().await })
    };
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            if matches!(
                host.heads(&caller("popup"), "vault").await,
                Err(Error::Denied)
            ) {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    assert!(
        !shutdown.is_finished(),
        "shutdown skipped a revocation already draining readers"
    );
    policy.0.release.notify_one();
    assert!(matches!(read.await.unwrap(), Err(Error::Canceled)));
    revoke.await.unwrap().unwrap();
    shutdown.await.unwrap().unwrap();
    assert!(matches!(
        host.status(&main, "vault", handle).await,
        Err(Error::Denied)
    ));
    host.shutdown().await.unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn revocation_preserves_other_windows_and_failed_preparation_is_observable() {
    let (_root, policy) = fixture();
    let cache = tempfile::tempdir().unwrap();
    let host = History::new(policy, cache.path().into(), Config::default()).unwrap();
    let main = caller("main");
    let popup = caller("popup");
    let operation = host.heads(&main, "vault").await.unwrap().heads[0];
    let first = host.open(&main, "vault", operation).await.unwrap();
    ready(&host, &main, first).await;
    let second = host.open(&popup, "vault", operation).await.unwrap();
    ready(&host, &popup, second).await;
    host.revoke("main").await.unwrap();
    assert!(matches!(
        host.status(&main, "vault", first).await,
        Err(Error::Denied)
    ));
    assert!(matches!(
        host.status(&popup, "vault", second).await.unwrap(),
        Status::Ready(_)
    ));
    host.close(&popup, "vault", second).await.unwrap();
    let missing = tauri_plugin_jj_store::OperationId::new([99; 64]);
    let failed = host.open(&popup, "vault", missing).await.unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(15), async {
        loop {
            match host.status(&popup, "vault", failed).await.unwrap() {
                Status::Preparing => tokio::task::yield_now().await,
                Status::Failed(_) => break,
                Status::Ready(_) => panic!("admitted unknown operation"),
            }
        }
    })
    .await
    .unwrap();
    assert!(matches!(
        host.status(&popup, "vault", failed).await.unwrap(),
        Status::Failed(_)
    ));
    host.shutdown().await.unwrap();
}

mod blame;
mod file_history;
mod operations;
