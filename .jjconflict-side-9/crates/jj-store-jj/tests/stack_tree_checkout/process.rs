use super::*;
use jj_store_jj::local_execution::{RecoveryWitnessV1, TreeCheckoutPlanV1};
use std::{
    io::Write as _,
    path::Path,
    process::Command,
    time::{Duration, Instant},
};
struct Access(PathBuf);
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
#[tokio::test]
#[ignore = "isolated worker invoked by killed-process parent"]
async fn worker() {
    let root = PathBuf::from(std::env::var_os("STACK_TREE_PLAN_ROOT").unwrap());
    let artifacts = PathBuf::from(std::env::var_os("STACK_TREE_PLAN_ARTIFACTS").unwrap());
    let (witness, key): (RecoveryWitnessV1, StackPreparationKey) =
        serde_json::from_slice(&fs::read(artifacts.join("inputs.json")).unwrap()).unwrap();
    let access = Arc::new(Access(root.join(".jj/repo")));
    let recovery = Coordinator::new(1)
        .unwrap()
        .restore_history(access, root.clone(), witness, key)
        .await
        .unwrap();
    let bytes = fs::read(root.join("file.txt")).unwrap();
    let digest = content_digest(&bytes);
    let (recovery, plan) = recovery
        .prepare_tree_checkout(
            Arc::new(Files {
                root,
                original: bytes,
            }),
            digest,
        )
        .await
        .unwrap();
    let output = serde_json::to_vec(&(plan.id().unwrap(), plan.encode().unwrap())).unwrap();
    let mut saved = fs::File::create(artifacts.join("plan.json")).unwrap();
    saved.write_all(&output).unwrap();
    saved.sync_all().unwrap();
    fs::File::open(&artifacts).unwrap().sync_all().unwrap();
    fs::write(artifacts.join("ready"), []).unwrap();
    std::thread::sleep(Duration::from_secs(20));
    drop(recovery);
}
#[tokio::test]
async fn killed_worker_saved_plan_reopens_against_original_retained_tree_and_files() {
    let f = Fixture::new().await;
    fs::write(f.root().join("file.txt"), "original\n").unwrap();
    let (recovery, witness, key) = retained_with_inputs(
        &f,
        vec![
            ("file.txt", Some((b"candidate", false))),
            ("new", Some((b"added", false))),
        ],
    )
    .await;
    drop(recovery);
    let before = f.state();
    let artifacts = tempfile::tempdir().unwrap();
    fs::write(
        artifacts.path().join("inputs.json"),
        serde_json::to_vec(&(witness.clone(), key.clone())).unwrap(),
    )
    .unwrap();
    let mut child = Command::new(std::env::current_exe().unwrap())
        .args(["--ignored", "--exact", "process::worker", "--nocapture"])
        .env("STACK_TREE_PLAN_ROOT", f.root())
        .env("STACK_TREE_PLAN_ARTIFACTS", artifacts.path())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(12);
    while !artifacts.path().join("ready").exists() && Instant::now() < deadline {
        if child.try_wait().unwrap().is_some() {
            break;
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
    let ready = artifacts.path().join("ready").exists();
    if child.try_wait().unwrap().is_none() {
        child.kill().unwrap();
    }
    child.wait().unwrap();
    assert!(ready, "worker did not durably capture exact plan");
    let (id, bytes): (m::RecordDigest, Vec<u8>) =
        serde_json::from_slice(&fs::read(artifacts.path().join("plan.json")).unwrap()).unwrap();
    let plan = TreeCheckoutPlanV1::decode(id, &bytes).unwrap();
    let recovery = Coordinator::new(1)
        .unwrap()
        .restore_history(f.access(), f.root(), witness, key)
        .await
        .unwrap();
    let (physical, _) = guard(&f);
    let (_recovery, same) = recovery
        .revalidate_tree_checkout(plan, physical)
        .await
        .unwrap();
    assert_eq!(same.id().unwrap(), id);
    assert_eq!(f.state(), before);
    assert!(!f.root().join("new").exists());
}
