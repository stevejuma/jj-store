use super::*;
use jj_store_jj::{
    local_browser::{self, Access, Error},
    query::workers::Workers,
};
use std::{
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

enum Fault {
    Pause(PathBuf),
    ReplaceLock,
    Revoke,
}
struct AfterSegment {
    base: Arc<dyn Access>,
    root: PathBuf,
    fault: Fault,
    fired: AtomicBool,
}
#[async_trait::async_trait]
impl Access for AfterSegment {
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
        let extra = self.root.join(".jj/repo/store/extra");
        let selected = fs::read_dir(&extra).ok().is_some_and(|mut entries| {
            entries.any(|e| e.unwrap().file_name().as_encoded_bytes().len() == 128)
        });
        if !selected {
            return Ok(());
        }
        match &self.fault {
            Fault::Pause(ready) => {
                fs::write(ready, []).unwrap();
                std::future::pending::<()>().await;
            }
            Fault::ReplaceLock if !self.fired.swap(true, Ordering::SeqCst) => {
                fs::remove_file(extra.join("lock")).unwrap();
                fs::write(extra.join("lock"), []).unwrap();
            }
            Fault::Revoke => return Err(Error::Unavailable),
            _ => (),
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
fn fault(f: &Fixture, fault: Fault) -> Arc<dyn Access> {
    Arc::new(AfterSegment {
        base: f.access(),
        root: f.root(),
        fault,
        fired: AtomicBool::new(false),
    })
}
fn remove_extras(f: &Fixture) {
    fs::remove_dir_all(f.root().join(".jj/repo/store/extra")).unwrap();
}
async fn retry(f: &Fixture, witness: RecoveryWitnessV1, key: StackPreparationKey) {
    drop(
        Coordinator::new(1)
            .unwrap()
            .restore_history(f.access(), f.root(), witness, key)
            .await
            .unwrap()
            .restore_extras()
            .await
            .unwrap(),
    );
}

#[tokio::test]
async fn cancellation_after_segment_retains_admission_until_exit_and_retry_preserves_bytes() {
    let f = Fixture::new().await;
    let (witness, key) = prepare(&f).await;
    let before = f.state();
    remove_extras(&f);
    let side = tempfile::tempdir().unwrap();
    let ready = side.path().join("ready");
    let access = fault(&f, Fault::Pause(ready.clone()));
    let workers = Workers::new(1).unwrap();
    let coordinator = Coordinator::with_workers(workers.clone(), Default::default()).unwrap();
    let recovery = coordinator
        .restore_history(access, f.root(), witness.clone(), key.clone())
        .await
        .unwrap();
    let task = tokio::spawn(async move { recovery.restore_extras().await });
    wait_for(&ready).await.unwrap();
    assert_eq!(workers.status().in_use, 1);
    let partial = extras::extra_state(&f.root());
    assert!(!partial.is_empty());
    assert!(partial.iter().all(|(n, _)| !n.starts_with("heads/")));
    assert_eq!(f.state(), before);
    assert!(matches!(
        coordinator
            .restore_history(f.access(), f.root(), witness.clone(), key.clone())
            .await,
        Err(Error::Busy)
    ));
    task.abort();
    assert!(matches!(task.await, Err(error) if error.is_cancelled()));
    tokio::time::timeout(Duration::from_secs(5), async {
        while workers.status().in_use != 0 {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    retry(&f, witness.clone(), key.clone()).await;
    let complete = extras::extra_state(&f.root());
    for entry in partial {
        assert!(complete.contains(&entry));
    }
    assert!(complete.iter().any(|(n, _)| n.starts_with("heads/")));
    retry(&f, witness, key).await;
    assert_eq!(extras::extra_state(&f.root()), complete);
    assert_eq!(f.state(), before);
}

#[tokio::test]
async fn replaced_extra_lock_and_authorization_loss_leave_no_head_and_exact_retry_finishes() {
    for mode in [Fault::ReplaceLock, Fault::Revoke] {
        let f = Fixture::new().await;
        let (witness, key) = prepare(&f).await;
        let before = f.state();
        remove_extras(&f);
        let recovery = Coordinator::new(1)
            .unwrap()
            .restore_history(fault(&f, mode), f.root(), witness.clone(), key.clone())
            .await
            .unwrap();
        assert!(recovery.restore_extras().await.is_err());
        let partial = extras::extra_state(&f.root());
        assert!(!partial.is_empty());
        assert!(partial.iter().all(|(n, _)| !n.starts_with("heads/")));
        assert_eq!(f.state(), before);
        retry(&f, witness, key).await;
        let complete = extras::extra_state(&f.root());
        for entry in partial {
            assert!(complete.contains(&entry));
        }
        assert_eq!(f.state(), before);
    }
}

#[tokio::test]
async fn process_loss_after_extra_segment_reopens_saved_witness_and_exact_retry() {
    let f = Fixture::new().await;
    let (witness, key) = prepare(&f).await;
    let before = f.state();
    remove_extras(&f);
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
            "extras_lifecycle::extra_recovery_process_helper",
        ])
        .env("JJ_STORE_STACK_EXTRA_RECOVERY", payload)
        .env("JJ_STORE_STACK_EXTRA_READY", &ready)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();
    if wait_for(&ready).await.is_err() {
        let _ = child.kill();
        let _ = child.wait();
        panic!("extra helper did not reach immutable segment selection");
    }
    child.kill().unwrap();
    assert!(!child.wait().unwrap().success());
    let partial = extras::extra_state(&f.root());
    assert!(!partial.is_empty());
    assert!(partial.iter().all(|(n, _)| !n.starts_with("heads/")));
    retry(&f, witness, key).await;
    let complete = extras::extra_state(&f.root());
    for entry in partial {
        assert!(complete.contains(&entry));
    }
    assert_eq!(f.state(), before);
}

#[tokio::test]
#[ignore = "invoked by isolated process-loss parent"]
async fn extra_recovery_process_helper() {
    let payload = std::env::var_os("JJ_STORE_STACK_EXTRA_RECOVERY").unwrap();
    let (root, id, bytes, key): (PathBuf, m::RecordDigest, Vec<u8>, StackPreparationKey) =
        serde_json::from_slice(&fs::read(payload).unwrap()).unwrap();
    let witness = RecoveryWitnessV1::decode(id, &bytes).unwrap();
    let ready = PathBuf::from(std::env::var_os("JJ_STORE_STACK_EXTRA_READY").unwrap());
    let access = Arc::new(AfterSegment {
        base: Arc::new(Reopened(root.join(".jj/repo"))),
        root: root.clone(),
        fault: Fault::Pause(ready),
        fired: AtomicBool::new(false),
    });
    let recovery = Coordinator::new(1)
        .unwrap()
        .restore_history(access, root, witness, key)
        .await
        .unwrap();
    recovery.restore_extras().await.unwrap();
    panic!("fixture must be killed before extra selection completes");
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
