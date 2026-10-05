use super::*;
#[path = "../../../jj-store-jj/tests/stack_foreground/fixture.rs"]
mod fixture;
use jj_store_jj::{
    local_browser as code,
    local_execution::{PhysicalGuard, RecoveryBinding},
};
use jj_store_model as m;
use std::{
    fs,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
async fn selection(f: &fixture::Fixture) -> (RecoveryBinding, m::RecordDigest) {
    let state = f.witness().await;
    let physical = tree::scan(
        &f.root(),
        state.baseline.repository,
        state.baseline.generation,
        tree::Limits::default(),
        |_| Ok(()),
    )
    .unwrap();
    (
        RecoveryBinding {
            repository: state.baseline.repository,
            generation: state.baseline.generation,
            physical_workspace: state.physical_workspace,
            operation: state.baseline.operation,
            view: state.baseline.view,
            commit: state.baseline.commit,
            workspace: state.checkout.workspace,
        },
        physical.fingerprint,
    )
}
#[tokio::test]
async fn registered_access_guard_captures_complete_inventory_and_preserves_validation() {
    let f = fixture::Fixture::new().await;
    let (binding, expected) = selection(&f).await;
    let observer = tree::Observer::new(code::Executor::new(1).unwrap(), 1).unwrap();
    let guard = observer.physical_guard(f.access(), f.root());
    let before = f.state();
    let inventory = guard.inventory(&binding, expected).await.unwrap();
    inventory.check_selection(&binding, expected).unwrap();
    assert!(inventory.files().any(|row| row.path == "ignored.txt"));
    assert_eq!(f.state(), before);
    guard.validate(&binding, expected).await.unwrap();
    fs::write(f.root().join("ignored.txt"), "new editor bytes\n").unwrap();
    assert_eq!(
        guard.inventory(&binding, expected).await,
        Err(code::Error::EpochChanged)
    );
    assert_eq!(
        fs::read(f.root().join("ignored.txt")).unwrap(),
        b"new editor bytes\n"
    );
}
struct Access {
    inner: Arc<dyn code::Access>,
    denied: AtomicBool,
}
#[async_trait::async_trait]
impl code::Access for Access {
    fn repository(&self) -> m::RepositoryId {
        self.inner.repository()
    }
    fn generation(&self) -> u64 {
        self.inner.generation()
    }
    fn metadata_path(&self) -> &std::path::Path {
        self.inner.metadata_path()
    }
    async fn authorize(&self) -> code::Result<()> {
        if self.denied.load(Ordering::SeqCst) {
            Err(code::Error::Denied)
        } else {
            Ok(())
        }
    }
}
#[tokio::test]
async fn revoked_and_replaced_binding_cannot_capture_or_initialize_inventory() {
    let f = fixture::Fixture::new().await;
    let (binding, expected) = selection(&f).await;
    let observer = tree::Observer::new(code::Executor::new(1).unwrap(), 1).unwrap();
    let access = Arc::new(Access {
        inner: f.access(),
        denied: AtomicBool::new(false),
    });
    let guard = observer.physical_guard(access.clone(), f.root());
    let before = f.state();
    for case in [
        "repository",
        "generation",
        "workspace",
        "fingerprint",
        "revoked",
    ] {
        let mut bad = binding.clone();
        let mut digest = expected;
        match case {
            "repository" => bad.repository = m::RepositoryId::new([2; 16]),
            "generation" => bad.generation += 1,
            "workspace" => bad.physical_workspace = m::RecordDigest::new([2; 32]),
            "fingerprint" => digest = m::RecordDigest::new([2; 32]),
            "revoked" => access.denied.store(true, Ordering::SeqCst),
            _ => unreachable!(),
        }
        assert!(guard.inventory(&bad, digest).await.is_err(), "{case}");
        assert_eq!(f.state(), before);
    }
}
#[tokio::test]
async fn validation_only_guard_does_not_gain_inventory_capture() {
    struct Validate;
    #[async_trait::async_trait]
    impl PhysicalGuard for Validate {
        async fn validate(&self, _: &RecoveryBinding, _: m::RecordDigest) -> code::Result<()> {
            Ok(())
        }
    }
    let f = fixture::Fixture::new().await;
    let (binding, expected) = selection(&f).await;
    Validate.validate(&binding, expected).await.unwrap();
    assert_eq!(
        Validate.inventory(&binding, expected).await,
        Err(code::Error::Invalid)
    );
    assert_eq!(Validate.observe(&binding).await, Err(code::Error::Invalid));
}
#[tokio::test]
async fn fresh_observation_preserves_scope_authorization_and_original_validation() {
    let f = fixture::Fixture::new().await;
    let (binding, expected) = selection(&f).await;
    let observer = tree::Observer::new(code::Executor::new(1).unwrap(), 1).unwrap();
    let access = Arc::new(Access {
        inner: f.access(),
        denied: AtomicBool::new(false),
    });
    let guard = observer.physical_guard(access.clone(), f.root());
    let original = guard.observe(&binding).await.unwrap();
    assert_eq!(original.fingerprint(), expected);
    fs::write(f.root().join("ignored.txt"), b"fresh ignored change\n").unwrap();
    let before = f.state();
    let current = guard.observe(&binding).await.unwrap();
    assert_ne!(original.fingerprint(), current.fingerprint());
    assert_eq!(
        guard.validate(&binding, expected).await,
        Err(code::Error::EpochChanged)
    );
    assert_eq!(
        guard.inventory(&binding, expected).await,
        Err(code::Error::EpochChanged)
    );
    for case in ["repository", "generation", "workspace"] {
        let mut bad = binding.clone();
        match case {
            "repository" => bad.repository = m::RepositoryId::new([9; 16]),
            "generation" => bad.generation += 1,
            _ => bad.physical_workspace = m::RecordDigest::new([9; 32]),
        }
        assert_eq!(guard.observe(&bad).await, Err(code::Error::EpochChanged));
    }
    access.denied.store(true, Ordering::SeqCst);
    assert_eq!(guard.observe(&binding).await, Err(code::Error::Denied));
    assert_eq!(f.state(), before);
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn inventory_capture_cancellation_retains_shared_admission_until_worker_exits() {
    use jj_store_jj::local_browser as code;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    struct Paused {
        inner: Arc<dyn code::Access>,
        calls: AtomicUsize,
        entered: tokio::sync::Notify,
    }
    #[async_trait::async_trait]
    impl code::Access for Paused {
        fn repository(&self) -> m::RepositoryId {
            self.inner.repository()
        }
        fn generation(&self) -> u64 {
            self.inner.generation()
        }
        fn metadata_path(&self) -> &std::path::Path {
            self.inner.metadata_path()
        }
        async fn authorize(&self) -> code::Result<()> {
            if self.calls.fetch_add(1, Ordering::SeqCst) >= 2 {
                self.entered.notify_one();
                std::future::pending::<()>().await;
            }
            self.inner.authorize().await
        }
    }
    let f = fixture::Fixture::new().await;
    let (binding, expected) = selection(&f).await;
    let before = f.state();
    let observer = tree::Observer::new(code::Executor::new(1).unwrap(), 1).unwrap();
    let access = Arc::new(Paused {
        inner: f.access(),
        calls: AtomicUsize::new(0),
        entered: tokio::sync::Notify::new(),
    });
    let first = observer.physical_guard(access.clone(), f.root());
    let copy = binding.clone();
    let task = tokio::spawn(async move { first.inventory(&copy, expected).await });
    tokio::time::timeout(std::time::Duration::from_secs(3), access.entered.notified())
        .await
        .unwrap();
    let second = observer.physical_guard(f.access(), f.root());
    assert_eq!(
        second.inventory(&binding, expected).await.unwrap_err(),
        code::Error::Busy
    );
    task.abort();
    assert!(task.await.unwrap_err().is_cancelled());
    tokio::time::timeout(std::time::Duration::from_secs(3), async {
        loop {
            match second.validate(&binding, expected).await {
                Ok(()) => break,
                Err(code::Error::Busy) => tokio::task::yield_now().await,
                Err(error) => panic!("{error}"),
            }
        }
    })
    .await
    .unwrap();
    assert_eq!(f.state(), before);
}
