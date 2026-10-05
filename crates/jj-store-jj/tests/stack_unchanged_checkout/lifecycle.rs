use super::*;
use jj_store_jj::{
    local_browser::Access, local_execution::CheckoutWitnessV1, query::workers::Workers,
};
use prost::Message as _;
use std::{
    path::{Path, PathBuf},
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};
enum Fault {
    Pause(PathBuf),
    Revoke,
    ReplaceLock,
    ChangePhysical,
}
struct AfterPointer {
    base: Arc<dyn PhysicalGuard>,
    root: PathBuf,
    operation: m::OperationId,
    fault: Fault,
    fired: AtomicBool,
}
#[async_trait::async_trait]
impl PhysicalGuard for AfterPointer {
    async fn validate(
        &self,
        binding: &jj_store_jj::local_execution::RecoveryBinding,
        expected: m::RecordDigest,
    ) -> local_browser::Result<()> {
        self.base.validate(binding, expected).await?;
        let bytes = fs::read(self.root.join(".jj/working_copy/checkout")).unwrap();
        let checkout =
            jj_lib::protos::local_working_copy::Checkout::decode(bytes.as_slice()).unwrap();
        if checkout.operation_id != self.operation.as_bytes() {
            return Ok(());
        }
        match &self.fault {
            Fault::Pause(ready) => {
                fs::write(ready, []).unwrap();
                std::future::pending::<()>().await;
            }
            Fault::Revoke => return Err(local_browser::Error::Denied),
            _ if self.fired.swap(true, Ordering::SeqCst) => (),
            Fault::ReplaceLock => {
                let path = self.root.join(".jj/working_copy/working_copy.lock");
                fs::remove_file(&path).unwrap();
                fs::write(path, []).unwrap();
            }
            Fault::ChangePhysical => {
                fs::write(self.root.join("ignored.txt"), "new ignored obstacle\n").unwrap();
                return Err(local_browser::Error::EpochChanged);
            }
        }
        Ok(())
    }
}
fn fault(f: &Fixture, operation: m::OperationId, fault: Fault) -> Arc<dyn PhysicalGuard> {
    Arc::new(AfterPointer {
        base: files(f),
        root: f.root(),
        operation,
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
async fn cancellation_after_pointer_selection_retains_admission_until_exit_and_retries_exactly() {
    let f = Fixture::new().await;
    let (witness, key, _) = prepare(&f).await;
    let workers = Workers::new(1).unwrap();
    let coordinator = Coordinator::with_workers(workers.clone(), Default::default()).unwrap();
    let recovery = coordinator
        .resume_publication(f.access(), f.root(), witness.clone(), key.clone())
        .await
        .unwrap()
        .restore_extras()
        .await
        .unwrap()
        .restore_indexes()
        .await
        .unwrap();
    let (recovery, published) = recovery.publish_head().await.unwrap();
    let side = tempfile::tempdir().unwrap();
    let ready = side.path().join("ready");
    let guard = fault(&f, published.operation, Fault::Pause(ready.clone()));
    let (recovery, checkout) = recovery
        .prepare_unchanged_checkout(&published, guard.clone(), m::RecordDigest::new([82; 32]))
        .await
        .unwrap();
    let before = f.state();
    let saved = checkout.clone();
    let task =
        tokio::spawn(async move { recovery.complete_unchanged_checkout(checkout, guard).await });
    wait_for(&ready).await.unwrap();
    assert_eq!(workers.status().in_use, 1);
    assert_eq!(f.state()[0], before[0]);
    assert_eq!(f.state()[1], before[1]);
    assert_eq!(f.state()[3], before[3]);
    assert_ne!(f.state()[2], before[2]);
    assert!(matches!(
        coordinator
            .resume_unchanged_checkout(
                f.access(),
                f.root(),
                witness.clone(),
                key.clone(),
                saved.clone()
            )
            .await,
        Err(local_browser::Error::Busy)
    ));
    task.abort();
    assert!(matches!(task.await,Err(e) if e.is_cancelled()));
    tokio::time::timeout(Duration::from_secs(5), async {
        while workers.status().in_use != 0 {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    let selected = f.state();
    let recovery = coordinator
        .resume_unchanged_checkout(f.access(), f.root(), witness, key, saved.clone())
        .await
        .unwrap();
    let (_recovery, proof) = recovery
        .complete_unchanged_checkout(saved, files(&f))
        .await
        .unwrap();
    assert_eq!(proof.operation, published.operation);
    assert_eq!(f.state(), selected);
}
#[tokio::test]
async fn denied_and_replaced_lock_after_pointer_selection_cannot_acknowledge_but_exact_retry_preserves_bytes()
 {
    for mode in [Fault::Revoke, Fault::ReplaceLock] {
        let f = Fixture::new().await;
        let (recovery, published, witness, key) = published(&f).await;
        let guard = fault(&f, published.operation, mode);
        let (recovery, checkout) = recovery
            .prepare_unchanged_checkout(&published, guard.clone(), m::RecordDigest::new([82; 32]))
            .await
            .unwrap();
        assert!(
            recovery
                .complete_unchanged_checkout(checkout.clone(), guard)
                .await
                .is_err()
        );
        let selected = f.state();
        let recovery = Coordinator::new(1)
            .unwrap()
            .resume_unchanged_checkout(f.access(), f.root(), witness, key, checkout.clone())
            .await
            .unwrap();
        let (_recovery, proof) = recovery
            .complete_unchanged_checkout(checkout, files(&f))
            .await
            .unwrap();
        assert_eq!(proof.operation, published.operation);
        assert_eq!(f.state(), selected);
    }
}
#[tokio::test]
async fn physical_change_after_pointer_selection_is_retained_and_requires_original_guard_recovery()
{
    let f = Fixture::new().await;
    let (recovery, published, witness, key) = published(&f).await;
    let original = files(&f);
    let guard = fault(&f, published.operation, Fault::ChangePhysical);
    let (recovery, checkout) = recovery
        .prepare_unchanged_checkout(&published, guard.clone(), m::RecordDigest::new([82; 32]))
        .await
        .unwrap();
    assert!(matches!(
        recovery
            .complete_unchanged_checkout(checkout.clone(), guard)
            .await,
        Err(local_browser::Error::EpochChanged)
    ));
    let selected = f.state();
    let recovery = Coordinator::new(1)
        .unwrap()
        .resume_unchanged_checkout(f.access(), f.root(), witness, key, checkout.clone())
        .await
        .unwrap();
    assert!(matches!(
        recovery
            .complete_unchanged_checkout(checkout, original)
            .await,
        Err(local_browser::Error::EpochChanged)
    ));
    assert_eq!(f.state(), selected);
    assert_eq!(
        fs::read(f.root().join("ignored.txt")).unwrap(),
        b"new ignored obstacle\n"
    );
}
#[tokio::test]
async fn subprocess_loss_after_pointer_selection_reopens_original_saved_witness() {
    let f = Fixture::new().await;
    let (recovery, published, witness, key) = published(&f).await;
    let (recovery, checkout) = recovery
        .prepare_unchanged_checkout(&published, files(&f), m::RecordDigest::new([82; 32]))
        .await
        .unwrap();
    drop(recovery);
    let before = f.state();
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
            checkout.id().unwrap(),
            checkout.encode().unwrap(),
            published.operation,
        ))
        .unwrap(),
    )
    .unwrap();
    let mut child = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--ignored", "--exact", "lifecycle::checkout_process_helper"])
        .env("JJ_STORE_STACK_CHECKOUT_PAYLOAD", payload)
        .env("JJ_STORE_STACK_CHECKOUT_READY", &ready)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();
    if wait_for(&ready).await.is_err() {
        let _ = child.kill();
        let _ = child.wait();
        panic!("helper did not select pointer");
    }
    child.kill().unwrap();
    assert!(!child.wait().unwrap().success());
    let selected = f.state();
    assert_eq!(selected[0], before[0]);
    assert_eq!(selected[1], before[1]);
    assert_eq!(selected[3], before[3]);
    assert_ne!(selected[2], before[2]);
    let recovery = Coordinator::new(1)
        .unwrap()
        .resume_unchanged_checkout(f.access(), f.root(), witness, key, checkout.clone())
        .await
        .unwrap();
    let (_recovery, proof) = recovery
        .complete_unchanged_checkout(checkout, files(&f))
        .await
        .unwrap();
    assert_eq!(proof.operation, published.operation);
    assert_eq!(f.state(), selected);
}
#[tokio::test]
#[ignore = "invoked by isolated process-loss parent"]
async fn checkout_process_helper() {
    let (root, id, bytes, key, cid, cbytes, operation): (
        PathBuf,
        m::RecordDigest,
        Vec<u8>,
        StackPreparationKey,
        m::RecordDigest,
        Vec<u8>,
        m::OperationId,
    ) = serde_json::from_slice(
        &fs::read(std::env::var_os("JJ_STORE_STACK_CHECKOUT_PAYLOAD").unwrap()).unwrap(),
    )
    .unwrap();
    let witness = RecoveryWitnessV1::decode(id, &bytes).unwrap();
    let checkout = CheckoutWitnessV1::decode(cid, &cbytes).unwrap();
    let ready = PathBuf::from(std::env::var_os("JJ_STORE_STACK_CHECKOUT_READY").unwrap());
    let expected = ["file.txt", "ignored.txt", ".gitignore"]
        .into_iter()
        .map(|p| fs::read(root.join(p)).unwrap())
        .collect();
    let guard = Arc::new(AfterPointer {
        base: Arc::new(Files {
            root: root.clone(),
            expected,
        }),
        root: root.clone(),
        operation,
        fault: Fault::Pause(ready),
        fired: AtomicBool::new(false),
    });
    let recovery = Coordinator::new(1)
        .unwrap()
        .resume_unchanged_checkout(
            Arc::new(Reopened(root.join(".jj/repo"))),
            root,
            witness,
            key,
            checkout.clone(),
        )
        .await
        .unwrap();
    recovery
        .complete_unchanged_checkout(checkout, guard)
        .await
        .unwrap();
    panic!("helper must be killed before acknowledgement");
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
