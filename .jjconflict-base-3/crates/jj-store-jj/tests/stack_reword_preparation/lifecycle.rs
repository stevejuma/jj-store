use super::*;
use jj_store_jj::{local_browser, query::workers::Workers};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
struct Access {
    inner: Arc<dyn local_browser::Access>,
    operations: PathBuf,
    count: usize,
    entered: Arc<AtomicBool>,
    deny: bool,
}
#[async_trait::async_trait]
impl local_browser::Access for Access {
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
        if fs::read_dir(&self.operations).unwrap().count() > self.count {
            self.entered.store(true, Ordering::SeqCst);
            if self.deny {
                Err(Error::Denied)
            } else {
                std::future::pending().await
            }
        } else {
            Ok(())
        }
    }
}
#[tokio::test]
async fn cancellation_after_candidate_write_releases_lease_only_when_worker_exits() {
    let fixture = Fixture::new().await;
    let expected = fixture.witness().await;
    let before = fixture.state();
    let workers = Workers::new(1).unwrap();
    let coordinator = Coordinator::with_workers(workers.clone(), Default::default()).unwrap();
    let operations = fixture.root().join(".jj/repo/op_store/operations");
    let entered = Arc::new(AtomicBool::new(false));
    let access = Arc::new(Access {
        inner: fixture.access(),
        count: fs::read_dir(&operations).unwrap().count(),
        operations,
        entered: entered.clone(),
        deny: false,
    });
    let lease = coordinator
        .acquire(access, fixture.root(), expected.clone())
        .await
        .unwrap();
    let task = tokio::spawn(async move {
        lease
            .prepare_reword(Reword {
                principal: m::PrincipalId::new([85; 16]),
                commit: expected.baseline.commit,
                message: "Unacknowledged candidate".into(),
            })
            .await
    });
    tokio::time::timeout(Duration::from_secs(5), async {
        while !entered.load(Ordering::SeqCst) {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    assert!(workers.status().in_use >= 1);
    task.abort();
    assert!(matches!(task.await, Err(e) if e.is_cancelled()));
    tokio::time::timeout(Duration::from_secs(5), async {
        while workers.status().in_use != 0 {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    coordinator
        .acquire(fixture.access(), fixture.root(), fixture.witness().await)
        .await
        .unwrap();
    assert_eq!(fixture.state(), before);
}
#[tokio::test]
async fn authorization_loss_after_candidate_write_exposes_no_preparation_or_publication() {
    let fixture = Fixture::new().await;
    let expected = fixture.witness().await;
    let before = fixture.state();
    let operations = fixture.root().join(".jj/repo/op_store/operations");
    let access = Arc::new(Access {
        inner: fixture.access(),
        count: fs::read_dir(&operations).unwrap().count(),
        operations,
        entered: Arc::new(AtomicBool::new(false)),
        deny: true,
    });
    let lease = Coordinator::new(1)
        .unwrap()
        .acquire(access, fixture.root(), expected.clone())
        .await
        .unwrap();
    assert!(matches!(
        lease
            .prepare_reword(Reword {
                principal: m::PrincipalId::new([85; 16]),
                commit: expected.baseline.commit,
                message: "Denied candidate".into()
            })
            .await,
        Err(Error::Denied)
    ));
    assert_eq!(fixture.state(), before);
}
