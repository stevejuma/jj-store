use super::*;
use jj_store_jj::{
    local_browser::{self, Access},
    query::workers::Workers,
};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

struct StopAfterView {
    base: Arc<dyn Access>,
    root: PathBuf,
    ready: PathBuf,
}
#[async_trait::async_trait]
impl Access for StopAfterView {
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
        if fs::read_dir(self.root.join(".jj/repo/op_store/views"))
            .ok()
            .is_some_and(|mut files| {
                files.any(|f| f.unwrap().file_name().as_encoded_bytes().len() == 128)
            })
        {
            fs::write(&self.ready, []).unwrap();
            std::future::pending::<()>().await;
        }
        Ok(())
    }
}
async fn wait_for(path: &Path) -> Result<(), tokio::time::error::Elapsed> {
    tokio::time::timeout(Duration::from_secs(10), async {
        while !path.exists() {
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
}

#[tokio::test]
async fn canceled_recovery_keeps_admission_until_worker_exits_and_exact_retry_finishes() {
    let f = Fixture::new().await;
    let (witness, key) = prepare(&f).await;
    let before = f.state();
    fs::remove_dir_all(f.root().join(".jj/repo/op_store")).unwrap();
    let side = tempfile::tempdir().unwrap();
    let ready = side.path().join("ready");
    let access = Arc::new(StopAfterView {
        base: f.access(),
        root: f.root(),
        ready: ready.clone(),
    });
    let workers = Workers::new(1).unwrap();
    let coordinator = Coordinator::with_workers(workers.clone(), Default::default()).unwrap();
    let root = f.root();
    let w = witness.clone();
    let k = key.clone();
    let task = tokio::spawn(async move { coordinator.restore_history(access, root, w, k).await });
    wait_for(&ready).await.unwrap();
    assert_eq!(workers.status().in_use, 1);
    task.abort();
    assert!(matches!(task.await,Err(error) if error.is_cancelled()));
    tokio::time::timeout(Duration::from_secs(5), async {
        while workers.status().in_use != 0 {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    Coordinator::new(1)
        .unwrap()
        .restore_history(f.access(), f.root(), witness, key)
        .await
        .unwrap();
    assert_eq!(f.state(), before);
}

#[tokio::test]
async fn process_loss_reopens_original_witness_and_retries_partial_immutable_restoration() {
    let f = Fixture::new().await;
    let (witness, key) = prepare(&f).await;
    let before = f.state();
    fs::remove_dir_all(f.root().join(".jj/repo/op_store")).unwrap();
    let side = tempfile::tempdir().unwrap();
    let payload = side.path().join("payload");
    let ready = side.path().join("ready");
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
            "lifecycle::source_recovery_process_helper",
        ])
        .env("JJ_STORE_STACK_SOURCE_RECOVERY", &payload)
        .env("JJ_STORE_STACK_SOURCE_RECOVERY_READY", &ready)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();
    let wait = wait_for(&ready).await;
    if wait.is_err() {
        let _ = child.kill();
        let _ = child.wait();
        panic!("recovery helper did not reach first selected view");
    }
    child.kill().unwrap();
    assert!(!child.wait().unwrap().success());
    assert_eq!(f.state(), before);
    Coordinator::new(1)
        .unwrap()
        .restore_history(f.access(), f.root(), witness, key)
        .await
        .unwrap();
    assert_eq!(f.state(), before);
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
#[tokio::test]
#[ignore = "invoked in an isolated subprocess by the process-loss parent"]
async fn source_recovery_process_helper() {
    let payload = std::env::var_os("JJ_STORE_STACK_SOURCE_RECOVERY").unwrap();
    let (root, id, bytes, key): (PathBuf, m::RecordDigest, Vec<u8>, StackPreparationKey) =
        serde_json::from_slice(&fs::read(payload).unwrap()).unwrap();
    let witness = RecoveryWitnessV1::decode(id, &bytes).unwrap();
    let ready = PathBuf::from(std::env::var_os("JJ_STORE_STACK_SOURCE_RECOVERY_READY").unwrap());
    let access = Arc::new(StopAfterView {
        base: Arc::new(Reopened(root.join(".jj/repo"))),
        root: root.clone(),
        ready,
    });
    Coordinator::new(1)
        .unwrap()
        .restore_history(access, root, witness, key)
        .await
        .unwrap();
    panic!("process fixture must be killed before recovery can finish");
}
