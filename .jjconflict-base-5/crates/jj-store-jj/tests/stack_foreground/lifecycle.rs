use super::*;
use jj_store_jj::{local_browser, query::workers::Workers};
use jj_store_model as m;
use std::{
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

#[tokio::test]
async fn external_heads_and_replaced_working_copy_directory_invalidate_the_lease() {
    let fixture = Fixture::new().await;
    let coordinator = Coordinator::new(1).unwrap();
    let witness = fixture.witness().await;
    let lease = coordinator
        .acquire(fixture.access(), fixture.root(), witness.clone())
        .await
        .unwrap();
    stock(
        &fixture.root(),
        &[
            "--ignore-working-copy",
            "bookmark",
            "create",
            "external",
            "-r",
            "@",
        ],
    );
    assert!(matches!(
        lease.validate_metadata().await,
        Err(Error::EpochChanged)
    ));
    drop(lease);
    assert!(matches!(
        coordinator
            .acquire(fixture.access(), fixture.root(), witness)
            .await,
        Err(Error::EpochChanged)
    ));

    let fixture = Fixture::new().await;
    let lease = coordinator
        .acquire(fixture.access(), fixture.root(), fixture.witness().await)
        .await
        .unwrap();
    let path = fixture.root().join(".jj/working_copy");
    let old = fixture.root().join(".jj/old-working-copy");
    fs::rename(&path, &old).unwrap();
    fs::create_dir(&path).unwrap();
    assert!(matches!(
        lease.validate_metadata().await,
        Err(Error::EpochChanged)
    ));
    drop(lease);
    assert!(path.is_dir());
}

struct GatedAccess {
    inner: Arc<dyn local_browser::Access>,
    lock: std::path::PathBuf,
    entered: Arc<AtomicBool>,
    gate: Arc<AtomicBool>,
}
#[async_trait::async_trait]
impl local_browser::Access for GatedAccess {
    fn repository(&self) -> m::RepositoryId {
        self.inner.repository()
    }
    fn generation(&self) -> u64 {
        self.inner.generation()
    }
    fn metadata_path(&self) -> &Path {
        self.inner.metadata_path()
    }
    async fn authorize(&self) -> local_browser::Result<()> {
        self.inner.authorize().await?;
        if self.lock.exists() && self.gate.load(Ordering::SeqCst) {
            self.entered.store(true, Ordering::SeqCst);
            std::future::pending().await
        } else {
            Ok(())
        }
    }
}

#[tokio::test]
async fn cancellation_after_lock_releases_foreground_admission_and_preserves_state() {
    let fixture = Fixture::new().await;
    let workers = Workers::new(1).unwrap();
    let coordinator = Coordinator::with_workers(workers.clone(), Default::default()).unwrap();
    let gate = Arc::new(AtomicBool::new(true));
    let entered = Arc::new(AtomicBool::new(false));
    let access = Arc::new(GatedAccess {
        inner: fixture.access(),
        lock: fixture.root().join(".jj/working_copy/working_copy.lock"),
        gate: gate.clone(),
        entered: entered.clone(),
    });
    let expected = fixture.witness().await;
    let before = fixture.state();
    let task = tokio::spawn({
        let coordinator = coordinator.clone();
        let root = fixture.root();
        let expected = expected.clone();
        async move { coordinator.acquire(access, root, expected).await }
    });
    tokio::time::timeout(Duration::from_secs(5), async {
        while !entered.load(Ordering::SeqCst) {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    assert!(workers.status().in_use > 0);
    task.abort();
    assert!(matches!(task.await, Err(error) if error.is_cancelled()));
    gate.store(false, Ordering::SeqCst);
    tokio::time::timeout(Duration::from_secs(5), async {
        while workers.status().in_use != 0 {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    coordinator
        .acquire(fixture.access(), fixture.root(), expected)
        .await
        .unwrap();
    assert_eq!(fixture.state(), before);
}

#[tokio::test]
async fn foreground_process_loss_releases_lock_without_changing_checkout() {
    let fixture = Fixture::new().await;
    let expected = fixture.witness().await;
    let before = fixture.state();
    let output = Command::new(std::env::current_exe().unwrap())
        .args(["--ignored", "--exact", "lifecycle::foreground_loss_helper"])
        .env("JJ_STORE_FOREGROUND_TEST_ROOT", fixture.root())
        .env(
            "JJ_STORE_FOREGROUND_TEST_OPERATION",
            expected.baseline.operation.to_string(),
        )
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(73),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(fixture.state(), before);
    Coordinator::new(1)
        .unwrap()
        .acquire(fixture.access(), fixture.root(), expected)
        .await
        .unwrap();
}

#[tokio::test]
#[ignore = "isolated subprocess process-loss helper; invoked by parent regression"]
async fn foreground_loss_helper() {
    let root: std::path::PathBuf = std::env::var_os("JJ_STORE_FOREGROUND_TEST_ROOT")
        .unwrap()
        .into();
    let operation = std::env::var("JJ_STORE_FOREGROUND_TEST_OPERATION")
        .unwrap()
        .parse()
        .unwrap();
    struct Access(std::path::PathBuf);
    #[async_trait::async_trait]
    impl local_browser::Access for Access {
        fn repository(&self) -> m::RepositoryId {
            m::RepositoryId::new([81; 16])
        }
        fn generation(&self) -> u64 {
            1
        }
        fn metadata_path(&self) -> &Path {
            &self.0
        }
        async fn authorize(&self) -> local_browser::Result<()> {
            Ok(())
        }
    }
    let access = Arc::new(Access(root.join(".jj/repo")));
    let expected = local_browser::Executor::new(1)
        .unwrap()
        .working_tree_current(access.clone(), root.clone(), operation)
        .await
        .unwrap();
    let _lease = Coordinator::new(1)
        .unwrap()
        .acquire(access, root, expected)
        .await
        .unwrap();
    std::process::exit(73);
}
