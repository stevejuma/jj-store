use super::*;
use jj_store_jj::local_browser as q;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
struct Gate {
    inner: Arc<dyn q::Access>,
    denied: AtomicBool,
    blocked: AtomicBool,
    entered: tokio::sync::Notify,
}
impl Gate {
    fn new(f: &Fixture) -> Self {
        Self {
            inner: f.access(),
            denied: AtomicBool::new(false),
            blocked: AtomicBool::new(false),
            entered: tokio::sync::Notify::new(),
        }
    }
}
#[async_trait::async_trait]
impl q::Access for Gate {
    fn repository(&self) -> m::RepositoryId {
        self.inner.repository()
    }
    fn generation(&self) -> u64 {
        self.inner.generation()
    }
    fn metadata_path(&self) -> &std::path::Path {
        self.inner.metadata_path()
    }
    async fn authorize(&self) -> q::Result<()> {
        self.inner.authorize().await?;
        if self.denied.load(Ordering::SeqCst) {
            return Err(q::Error::Denied);
        }
        if self.blocked.load(Ordering::SeqCst) {
            self.entered.notify_one();
            std::future::pending().await
        } else {
            Ok(())
        }
    }
}
#[tokio::test]
async fn current_policy_authorization_loss_releases_original_admission_without_writes() {
    let f = Fixture::new().await;
    let current = f.witness().await;
    let (witness, key) = prepare(&f).await;
    let coordinator = Coordinator::new(1).unwrap();
    let access = Arc::new(Gate::new(&f));
    let lease = coordinator
        .resume_publication(access.clone(), f.root(), witness, key)
        .await
        .unwrap();
    let before = f.state();
    access.denied.store(true, Ordering::SeqCst);
    assert!(matches!(
        lease
            .validate_reword_policy(current.baseline.commit, vec![m::GitSha1Id::new([0; 20])])
            .await,
        Err(q::Error::Denied)
    ));
    access.denied.store(false, Ordering::SeqCst);
    let _lease = coordinator
        .acquire(f.access(), f.root(), current)
        .await
        .unwrap();
    assert_eq!(f.state(), before);
}
#[tokio::test]
async fn canceled_policy_worker_retains_original_lock_and_admission_until_exit() {
    let f = Fixture::new().await;
    let current = f.witness().await;
    let (witness, key) = prepare(&f).await;
    let coordinator = Coordinator::new(1).unwrap();
    let access = Arc::new(Gate::new(&f));
    let lease = coordinator
        .resume_publication(access.clone(), f.root(), witness, key)
        .await
        .unwrap();
    let before = f.state();
    access.blocked.store(true, Ordering::SeqCst);
    let task = tokio::spawn(
        lease.validate_reword_policy(current.baseline.commit, vec![m::GitSha1Id::new([0; 20])]),
    );
    tokio::time::timeout(std::time::Duration::from_secs(8), access.entered.notified())
        .await
        .unwrap();
    assert!(matches!(
        coordinator
            .acquire(f.access(), f.root(), current.clone())
            .await,
        Err(q::Error::Busy)
    ));
    task.abort();
    assert!(matches!(task.await, Err(error) if error.is_cancelled()));
    access.blocked.store(false, Ordering::SeqCst);
    let _lease = tokio::time::timeout(std::time::Duration::from_secs(3), async {
        loop {
            match coordinator
                .acquire(f.access(), f.root(), current.clone())
                .await
            {
                Ok(lease) => break lease,
                Err(q::Error::Busy) => {
                    tokio::time::sleep(std::time::Duration::from_millis(10)).await
                }
                Err(error) => panic!("unexpected recovery error: {error}"),
            }
        }
    })
    .await
    .unwrap();
    assert_eq!(f.state(), before);
}
