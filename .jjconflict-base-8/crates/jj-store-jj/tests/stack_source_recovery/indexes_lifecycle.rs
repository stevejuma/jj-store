use super::*;
use jj_store_jj::{
    local_browser::{self, Access, Error},
    query::workers::Workers,
};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

struct PauseAfterAssociation {
    base: Arc<dyn Access>,
    root: PathBuf,
    ready: PathBuf,
}
#[async_trait::async_trait]
impl Access for PauseAfterAssociation {
    fn repository(&self) -> m::RepositoryId {
        self.base.repository()
    }
    fn generation(&self) -> u64 {
        self.base.generation()
    }
    fn metadata_path(&self) -> &Path {
        self.base.metadata_path()
    }
    async fn authorize(&self) -> local_browser::Result<()> {
        self.base.authorize().await?;
        let count = fs::read_dir(self.root.join(".jj/repo/index/op_links"))
            .ok()
            .map_or(0, |entries| {
                entries
                    .filter(|e| e.as_ref().unwrap().file_name().as_encoded_bytes().len() == 128)
                    .count()
            });
        if count == 1 {
            fs::write(&self.ready, []).unwrap();
            std::future::pending::<()>().await;
        }
        Ok(())
    }
}
async fn wait_for(path: &Path) -> Result<(), tokio::time::error::Elapsed> {
    tokio::time::timeout(Duration::from_secs(10), async {
        while !path.exists() {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
}
async fn retry(f: &Fixture, witness: RecoveryWitnessV1, key: StackPreparationKey) {
    drop(
        Coordinator::new(1)
            .unwrap()
            .restore_history(f.access(), f.root(), witness, key)
            .await
            .unwrap()
            .restore_indexes()
            .await
            .unwrap(),
    );
}

#[tokio::test]
async fn cancellation_after_one_index_association_retains_admission_and_exact_retry() {
    let f = Fixture::new().await;
    let (witness, key) = prepare(&f).await;
    let before = f.state();
    fs::remove_dir_all(f.root().join(".jj/repo/index")).unwrap();
    let side = tempfile::tempdir().unwrap();
    let ready = side.path().join("ready");
    let workers = Workers::new(1).unwrap();
    let coordinator = Coordinator::with_workers(workers.clone(), Default::default()).unwrap();
    let access = Arc::new(PauseAfterAssociation {
        base: f.access(),
        root: f.root(),
        ready: ready.clone(),
    });
    let recovery = coordinator
        .restore_history(access, f.root(), witness.clone(), key.clone())
        .await
        .unwrap();
    let task = tokio::spawn(async move { recovery.restore_indexes().await });
    wait_for(&ready).await.unwrap();
    let partial = indexes::index_state(&f.root());
    assert_eq!(
        partial
            .iter()
            .filter(|(n, _)| n.starts_with("op_links/"))
            .count(),
        1
    );
    assert_eq!(workers.status().in_use, 1);
    assert!(matches!(
        coordinator
            .restore_history(f.access(), f.root(), witness.clone(), key.clone())
            .await,
        Err(Error::Busy)
    ));
    assert_eq!(f.state(), before);
    task.abort();
    assert!(matches!(task.await, Err(e) if e.is_cancelled()));
    tokio::time::timeout(Duration::from_secs(5), async {
        while workers.status().in_use != 0 {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    retry(&f, witness.clone(), key.clone()).await;
    let complete = indexes::index_state(&f.root());
    for entry in partial {
        assert!(complete.contains(&entry));
    }
    assert_eq!(
        complete
            .iter()
            .filter(|(n, _)| n.starts_with("op_links/"))
            .count(),
        2
    );
    retry(&f, witness, key).await;
    assert_eq!(indexes::index_state(&f.root()), complete);
    assert_eq!(f.state(), before);
}

#[tokio::test]
async fn process_loss_after_one_index_association_reopens_saved_request_and_preserves_segments() {
    let f = Fixture::new().await;
    let (witness, key) = prepare(&f).await;
    let before = f.state();
    fs::remove_dir_all(f.root().join(".jj/repo/index")).unwrap();
    let side = tempfile::tempdir().unwrap();
    let ready = side.path().join("ready");
    let payload = side.path().join("payload");
    fs::write(
        &payload,
        serde_json::to_vec(&(
            f.root(),
            witness.id().unwrap(),
            witness.encode().unwrap(),
            key.clone(),
        ))
        .unwrap(),
    )
    .unwrap();
    let mut child = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--ignored",
            "--exact",
            "indexes_lifecycle::index_recovery_process_helper",
        ])
        .env("JJ_STORE_STACK_INDEX_RECOVERY", payload)
        .env("JJ_STORE_STACK_INDEX_READY", &ready)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();
    if wait_for(&ready).await.is_err() {
        let _ = child.kill();
        let _ = child.wait();
        panic!("index helper did not reach first association");
    }
    child.kill().unwrap();
    assert!(!child.wait().unwrap().success());
    let partial = indexes::index_state(&f.root());
    assert_eq!(
        partial
            .iter()
            .filter(|(n, _)| n.starts_with("op_links/"))
            .count(),
        1
    );
    retry(&f, witness, key).await;
    let complete = indexes::index_state(&f.root());
    for entry in partial {
        assert!(complete.contains(&entry));
    }
    assert_eq!(f.state(), before);
}

#[tokio::test]
#[ignore = "invoked by isolated process-loss parent"]
async fn index_recovery_process_helper() {
    let payload = std::env::var_os("JJ_STORE_STACK_INDEX_RECOVERY").unwrap();
    let (root, id, bytes, key): (PathBuf, m::RecordDigest, Vec<u8>, StackPreparationKey) =
        serde_json::from_slice(&fs::read(payload).unwrap()).unwrap();
    let witness = RecoveryWitnessV1::decode(id, &bytes).unwrap();
    let ready = PathBuf::from(std::env::var_os("JJ_STORE_STACK_INDEX_READY").unwrap());
    let access = Arc::new(PauseAfterAssociation {
        base: Arc::new(Reopened(root.join(".jj/repo"))),
        root: root.clone(),
        ready,
    });
    let recovery = Coordinator::new(1)
        .unwrap()
        .restore_history(access, root, witness, key)
        .await
        .unwrap();
    recovery.restore_indexes().await.unwrap();
    panic!("fixture must be killed before index restoration finishes");
}
struct Reopened(PathBuf);
#[async_trait::async_trait]
impl Access for Reopened {
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
