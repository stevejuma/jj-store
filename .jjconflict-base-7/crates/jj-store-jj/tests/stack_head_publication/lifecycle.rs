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
    PauseAfterRemoval(PathBuf, m::OperationId),
    Revoke,
    ReplaceLock,
    ReplaceDirectory,
    ChangePolicy,
    ChangeCheckout,
}
struct AfterCandidate {
    base: Arc<dyn Access>,
    root: PathBuf,
    candidate: m::OperationId,
    fault: Fault,
    fired: AtomicBool,
}
#[async_trait::async_trait]
impl Access for AfterCandidate {
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
        let heads = self.root.join(".jj/repo/op_heads/heads");
        if !heads.join(self.candidate.to_string()).exists() {
            return Ok(());
        }
        match &self.fault {
            Fault::Pause(ready) => {
                fs::write(ready, []).unwrap();
                std::future::pending::<()>().await;
            }
            Fault::PauseAfterRemoval(ready, original) => {
                if !heads.join(original.to_string()).exists() {
                    fs::write(ready, []).unwrap();
                    std::future::pending::<()>().await;
                }
            }
            Fault::Revoke => return Err(Error::Unavailable),
            _ if self.fired.swap(true, Ordering::SeqCst) => (),
            Fault::ReplaceLock => {
                fs::remove_file(heads.join("lock")).unwrap();
                fs::write(heads.join("lock"), []).unwrap();
            }
            Fault::ReplaceDirectory => {
                let old = heads.with_file_name("old-heads");
                fs::rename(&heads, &old).unwrap();
                fs::create_dir(&heads).unwrap();
                for entry in fs::read_dir(old).unwrap() {
                    let entry = entry.unwrap();
                    fs::copy(entry.path(), heads.join(entry.file_name())).unwrap();
                }
            }
            Fault::ChangePolicy => {
                fs::write(self.root.join(".git/info/exclude"), "changed-policy\n").unwrap();
            }
            Fault::ChangeCheckout => {
                fs::write(
                    self.root.join(".jj/working_copy/tree_state"),
                    "external checkout change",
                )
                .unwrap();
            }
        }
        Ok(())
    }
}
fn fault(f: &Fixture, candidate: m::OperationId, fault: Fault) -> Arc<dyn Access> {
    Arc::new(AfterCandidate {
        base: f.access(),
        root: f.root(),
        candidate,
        fault,
        fired: AtomicBool::new(false),
    })
}
async fn wait_for(path: &Path) -> Result<(), tokio::time::error::Elapsed> {
    tokio::time::timeout(Duration::from_secs(10), async {
        while !path.exists() {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
}

#[tokio::test]
async fn cancellation_after_selection_retains_admission_and_exact_retry_finishes_old_head_removal()
{
    let f = Fixture::new().await;
    let (witness, key, candidate) = prepare(&f).await;
    let side = tempfile::tempdir().unwrap();
    let ready = side.path().join("ready");
    let workers = Workers::new(1).unwrap();
    let coordinator = Coordinator::with_workers(workers.clone(), Default::default()).unwrap();
    let recovery = coordinator
        .resume_publication(
            fault(&f, candidate, Fault::Pause(ready.clone())),
            f.root(),
            witness.clone(),
            key.clone(),
        )
        .await
        .unwrap()
        .restore_extras()
        .await
        .unwrap()
        .restore_indexes()
        .await
        .unwrap();
    let before = f.state();
    let task = tokio::spawn(async move { recovery.publish_head().await });
    wait_for(&ready).await.unwrap();
    assert_eq!(heads(&f).len(), 2);
    assert_eq!(workers.status().in_use, 1);
    assert!(matches!(
        coordinator
            .resume_publication(f.access(), f.root(), witness.clone(), key.clone())
            .await,
        Err(Error::Busy)
    ));
    assert_eq!(&f.state()[..4], &before[..4]);
    task.abort();
    assert!(matches!(task.await, Err(e) if e.is_cancelled()));
    tokio::time::timeout(Duration::from_secs(5), async {
        while workers.status().in_use != 0 {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    let (_recovery, proof) = recover(&f, witness, key)
        .await
        .publish_head()
        .await
        .unwrap();
    assert_eq!(proof.operation, candidate);
    assert_eq!(heads(&f), vec![candidate.to_string()]);
    assert_eq!(&f.state()[..4], &before[..4]);
}

#[tokio::test]
async fn revocation_and_replaced_head_lock_after_candidate_selection_preserve_exact_retry() {
    for mode in [Fault::Revoke, Fault::ReplaceLock] {
        let f = Fixture::new().await;
        let (witness, key, candidate) = prepare(&f).await;
        let recovery = Coordinator::new(1)
            .unwrap()
            .resume_publication(
                fault(&f, candidate, mode),
                f.root(),
                witness.clone(),
                key.clone(),
            )
            .await
            .unwrap()
            .restore_extras()
            .await
            .unwrap()
            .restore_indexes()
            .await
            .unwrap();
        let before = f.state();
        assert!(recovery.publish_head().await.is_err());
        assert_eq!(heads(&f).len(), 2);
        assert_eq!(&f.state()[..4], &before[..4]);
        let (_recovery, proof) = recover(&f, witness, key)
            .await
            .publish_head()
            .await
            .unwrap();
        assert_eq!(proof.operation, candidate);
        assert_eq!(heads(&f), vec![candidate.to_string()]);
    }
}

#[tokio::test]
async fn changed_policy_checkout_and_replaced_head_directory_never_acknowledge_publication() {
    for mode in [
        Fault::ChangePolicy,
        Fault::ChangeCheckout,
        Fault::ReplaceDirectory,
    ] {
        let f = Fixture::new().await;
        let (witness, key, candidate) = prepare(&f).await;
        let recovery = Coordinator::new(1)
            .unwrap()
            .resume_publication(
                fault(&f, candidate, mode),
                f.root(),
                witness.clone(),
                key.clone(),
            )
            .await
            .unwrap()
            .restore_extras()
            .await
            .unwrap()
            .restore_indexes()
            .await
            .unwrap();
        assert!(recovery.publish_head().await.is_err());
        assert_eq!(heads(&f).len(), 2);
        let after = f.state();
        assert!(
            Coordinator::new(1)
                .unwrap()
                .resume_publication(f.access(), f.root(), witness, key)
                .await
                .is_err()
        );
        assert_eq!(f.state(), after);
    }
}

#[tokio::test]
async fn subprocess_loss_after_candidate_selection_reopens_same_witness_without_checkout() {
    let f = Fixture::new().await;
    let (witness, key, candidate) = prepare(&f).await;
    drop(recover(&f, witness.clone(), key.clone()).await);
    let before = f.state();
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
            candidate,
        ))
        .unwrap(),
    )
    .unwrap();
    let mut child = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--ignored",
            "--exact",
            "lifecycle::publication_process_helper",
        ])
        .env("JJ_STORE_STACK_HEAD_PAYLOAD", payload)
        .env("JJ_STORE_STACK_HEAD_READY", &ready)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();
    if wait_for(&ready).await.is_err() {
        let _ = child.kill();
        let _ = child.wait();
        panic!("publication helper did not select candidate");
    }
    child.kill().unwrap();
    assert!(!child.wait().unwrap().success());
    assert_eq!(heads(&f).len(), 2);
    assert_eq!(&f.state()[..4], &before[..4]);
    let (_recovery, proof) = recover(&f, witness, key)
        .await
        .publish_head()
        .await
        .unwrap();
    assert_eq!(proof.operation, candidate);
    assert_eq!(heads(&f), vec![candidate.to_string()]);
    assert_eq!(&f.state()[..4], &before[..4]);
}

#[tokio::test]
async fn cancellation_after_original_removal_retries_same_proof_without_reannouncing_heads() {
    let f = Fixture::new().await;
    let (witness, key, candidate) = prepare(&f).await;
    let side = tempfile::tempdir().unwrap();
    let ready = side.path().join("ready");
    let recovery = Coordinator::new(1)
        .unwrap()
        .resume_publication(
            fault(
                &f,
                candidate,
                Fault::PauseAfterRemoval(ready.clone(), witness.binding().operation),
            ),
            f.root(),
            witness.clone(),
            key.clone(),
        )
        .await
        .unwrap()
        .restore_extras()
        .await
        .unwrap()
        .restore_indexes()
        .await
        .unwrap();
    let before = f.state();
    let task = tokio::spawn(async move { recovery.publish_head().await });
    wait_for(&ready).await.unwrap();
    assert_eq!(heads(&f), vec![candidate.to_string()]);
    task.abort();
    assert!(matches!(task.await, Err(e) if e.is_cancelled()));
    let coordinator = Coordinator::new(1).unwrap();
    let recovery = tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            match coordinator
                .resume_publication(f.access(), f.root(), witness.clone(), key.clone())
                .await
            {
                Ok(recovery) => break recovery,
                Err(Error::Busy) => tokio::time::sleep(Duration::from_millis(10)).await,
                Err(e) => panic!("unexpected retry: {e:?}"),
            }
        }
    })
    .await
    .unwrap();
    let (_recovery, proof) = recovery.publish_head().await.unwrap();
    assert_eq!(proof.operation, candidate);
    assert_eq!(heads(&f), vec![candidate.to_string()]);
    assert_eq!(&f.state()[..4], &before[..4]);
}

#[tokio::test]
#[ignore = "invoked by isolated process-loss parent"]
async fn publication_process_helper() {
    let (root, id, bytes, key, candidate): (
        PathBuf,
        m::RecordDigest,
        Vec<u8>,
        StackPreparationKey,
        m::OperationId,
    ) = serde_json::from_slice(
        &fs::read(std::env::var_os("JJ_STORE_STACK_HEAD_PAYLOAD").unwrap()).unwrap(),
    )
    .unwrap();
    let witness = RecoveryWitnessV1::decode(id, &bytes).unwrap();
    let access = Arc::new(AfterCandidate {
        base: Arc::new(Reopened(root.join(".jj/repo"))),
        root: root.clone(),
        candidate,
        fault: Fault::Pause(PathBuf::from(
            std::env::var_os("JJ_STORE_STACK_HEAD_READY").unwrap(),
        )),
        fired: AtomicBool::new(false),
    });
    let recovery = Coordinator::new(1)
        .unwrap()
        .resume_publication(access, root, witness, key)
        .await
        .unwrap();
    recovery.publish_head().await.unwrap();
    panic!("helper must be killed before old-head removal");
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
