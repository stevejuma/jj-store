use super::*;
use installation::{Hook, physical, private, progress, published};
use jj_store_jj::local_execution::PhysicalInventoryV1;
use std::{
    path::Path,
    sync::{
        Condvar, Mutex,
        atomic::{AtomicBool, Ordering},
    },
};
#[tokio::test]
async fn replaced_staging_files_and_private_directory_fail_before_physical_changes() {
    for fault in ["candidate", "directory", "progress-mode", "next"] {
        let f = Input::build(true).await;
        let inventory = f.inventory().await;
        let (recovery, head) = published(&f).await;
        let private_path = private(&f);
        let once = AtomicBool::new(false);
        let guard = Arc::new(Hook {
            inner: physical(&f),
            root: f.folder.root(),
            plan: f.plan.clone(),
            fired: AtomicBool::new(false),
            action: Box::new(move |_, state| {
                if state["intent"].is_null() && !once.swap(true, Ordering::SeqCst) {
                    match fault {
                        "candidate" => {
                            fs::rename(
                                private_path.join("f-0"),
                                private_path.join("saved-editor-copy"),
                            )
                            .unwrap();
                            fs::write(private_path.join("f-0"), b"candidate\n").unwrap();
                        }
                        "directory" => {
                            fs::rename(&private_path, private_path.with_extension("replaced"))
                                .unwrap();
                            fs::create_dir(&private_path).unwrap();
                        }
                        "progress-mode" => {
                            use std::os::unix::fs::PermissionsExt as _;
                            fs::set_permissions(
                                private_path.join("progress"),
                                fs::Permissions::from_mode(0o644),
                            )
                            .unwrap();
                        }
                        _ => unreachable!(),
                    }
                }
            }),
        });
        assert!(
            recovery
                .install_tree_files(&head, f.plan.clone(), inventory, guard)
                .await
                .is_err(),
            "{fault}"
        );
        assert_eq!(
            fs::read(f.folder.root().join("file.txt")).unwrap(),
            b"original\n"
        );
        assert!(!f.folder.root().join("src").exists());
    }
}
#[tokio::test]
async fn replaced_parent_backup_symlink_and_changed_mode_never_acknowledge_installation() {
    for fault in ["parent", "backup", "symlink", "candidate-mode"] {
        let f = Input::build(true).await;
        let inventory = f.inventory().await;
        let (recovery, head) = published(&f).await;
        let private_path = private(&f);
        let once = AtomicBool::new(false);
        let guard = Arc::new(Hook {
            inner: physical(&f),
            root: f.folder.root(),
            plan: f.plan.clone(),
            fired: AtomicBool::new(false),
            action: Box::new(move |root, state| {
                if state["phases"][0] == 1 && !once.swap(true, Ordering::SeqCst) {
                    match fault {
                        "parent" => {
                            fs::rename(root.join("src"), root.join("editor-src")).unwrap();
                            fs::create_dir(root.join("src")).unwrap();
                        }
                        "backup" => {
                            fs::write(private_path.join("b-0"), b"changed backup\n").unwrap();
                        }
                        "symlink" => {
                            std::os::unix::fs::symlink(
                                private_path.join("f-0"),
                                root.join("file.txt"),
                            )
                            .unwrap();
                        }
                        "candidate-mode" => {
                            use std::os::unix::fs::PermissionsExt as _;
                            fs::set_permissions(
                                private_path.join("f-0"),
                                fs::Permissions::from_mode(0o644),
                            )
                            .unwrap();
                        }
                        _ => unreachable!(),
                    }
                }
            }),
        });
        assert!(
            recovery
                .install_tree_files(&head, f.plan.clone(), inventory, guard)
                .await
                .is_err(),
            "{fault}"
        );
        assert!(private(&f).join("b-0").exists());
        assert_eq!(fs::read(private(&f).join("f-0")).unwrap(), b"candidate\n");
        assert!(!f.folder.root().join("src/new").exists());
    }
}
struct Access {
    inner: Arc<dyn local_browser::Access>,
    denied: Arc<AtomicBool>,
}
struct StopPrepared(Arc<dyn PhysicalGuard>, PathBuf);
#[async_trait::async_trait]
impl PhysicalGuard for StopPrepared {
    async fn validate(
        &self,
        binding: &RecoveryBinding,
        expected: m::RecordDigest,
    ) -> local_browser::Result<()> {
        self.0.validate(binding, expected).await
    }
    async fn observe(
        &self,
        binding: &RecoveryBinding,
    ) -> local_browser::Result<PhysicalInventoryV1> {
        if self.1.join("progress").exists() {
            return Err(local_browser::Error::Unavailable);
        }
        self.0.observe(binding).await
    }
}
#[tokio::test]
async fn interrupted_initial_progress_prefix_reuses_exact_staged_file_identities() {
    use std::os::unix::fs::MetadataExt as _;
    let f = Input::build(true).await;
    let inventory = f.inventory().await;
    let (recovery, head) = published(&f).await;
    assert!(matches!(
        recovery
            .install_tree_files(
                &head,
                f.plan.clone(),
                inventory.clone(),
                Arc::new(StopPrepared(physical(&f), private(&f)))
            )
            .await,
        Err(local_browser::Error::Unavailable)
    ));
    let before = fs::metadata(private(&f).join("f-0")).unwrap();
    let bytes = fs::read(private(&f).join("progress")).unwrap();
    fs::rename(private(&f).join("progress"), private(&f).join("next")).unwrap();
    fs::OpenOptions::new()
        .write(true)
        .open(private(&f).join("next"))
        .unwrap()
        .set_len((bytes.len() - 20) as u64)
        .unwrap();
    let (recovery, head) = published(&f).await;
    let (recovery, _) = recovery
        .install_tree_files(&head, f.plan.clone(), inventory, physical(&f))
        .await
        .unwrap();
    assert_eq!(
        fs::metadata(f.folder.root().join("file.txt"))
            .unwrap()
            .ino(),
        before.ino()
    );
    assert_eq!(
        fs::metadata(f.folder.root().join("file.txt"))
            .unwrap()
            .mtime(),
        before.mtime()
    );
    assert_eq!(
        fs::read(f.folder.root().join("file.txt")).unwrap(),
        b"candidate\n"
    );
    drop(recovery);
}
#[tokio::test]
async fn partial_candidate_prefix_is_resumed_but_corrupt_prefix_is_preserved() {
    use std::os::unix::fs::{MetadataExt as _, PermissionsExt as _};
    for valid in [true, false] {
        let f = Input::build(true).await;
        let inventory = f.inventory().await;
        let (recovery, head) = published(&f).await;
        fs::create_dir(private(&f)).unwrap();
        fs::set_permissions(private(&f), fs::Permissions::from_mode(0o700)).unwrap();
        private_write(
            &private(&f).join("f-0"),
            if valid { b"candi" } else { b"wrong" },
        );
        let inode = fs::metadata(private(&f).join("f-0")).unwrap().ino();
        let result = recovery
            .install_tree_files(&head, f.plan.clone(), inventory, physical(&f))
            .await;
        if valid {
            let (recovery, _) = result.unwrap();
            assert_eq!(
                fs::metadata(f.folder.root().join("file.txt"))
                    .unwrap()
                    .ino(),
                inode
            );
            drop(recovery);
        } else {
            assert!(matches!(result, Err(local_browser::Error::Corrupt)));
            assert_eq!(fs::read(private(&f).join("f-0")).unwrap(), b"wrong");
            assert_eq!(
                fs::read(f.folder.root().join("file.txt")).unwrap(),
                b"original\n"
            );
        }
    }
}
#[tokio::test]
async fn missing_completed_progress_never_reconstructs_an_installation_from_new_files() {
    let f = Input::build(true).await;
    let inventory = f.inventory().await;
    let (recovery, head) = published(&f).await;
    let (recovery, _) = recovery
        .install_tree_files(&head, f.plan.clone(), inventory.clone(), physical(&f))
        .await
        .unwrap();
    drop(recovery);
    fs::remove_file(private(&f).join("progress")).unwrap();
    let (recovery, head) = published(&f).await;
    assert!(matches!(
        recovery
            .install_tree_files(&head, f.plan.clone(), inventory, physical(&f))
            .await,
        Err(local_browser::Error::EpochChanged)
    ));
    assert!(!private(&f).join("progress").exists());
    assert_eq!(fs::read(private(&f).join("b-0")).unwrap(), b"original\n");
    assert_eq!(
        fs::read(f.folder.root().join("file.txt")).unwrap(),
        b"candidate\n"
    );
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
        if self.denied.load(Ordering::SeqCst) {
            Err(local_browser::Error::Denied)
        } else {
            self.inner.authorize().await
        }
    }
}
#[tokio::test]
async fn authorization_loss_between_moves_retains_pending_progress_and_backups() {
    let f = Input::build(true).await;
    let inventory = f.inventory().await;
    let (recovery, _) = published(&f).await;
    drop(recovery);
    let denied = Arc::new(AtomicBool::new(false));
    let access: Arc<dyn local_browser::Access> = Arc::new(Access {
        inner: f.folder.access(),
        denied: denied.clone(),
    });
    let inner =
        jj_store_service::working_tree::Observer::new(local_browser::Executor::new(1).unwrap(), 1)
            .unwrap()
            .physical_guard(access.clone(), f.folder.root());
    let guard = Arc::new(Hook {
        inner,
        root: f.folder.root(),
        plan: f.plan.clone(),
        fired: AtomicBool::new(false),
        action: Box::new(move |_, state| {
            if state["phases"][0] == 1 {
                denied.store(true, Ordering::SeqCst);
            }
        }),
    });
    let recovery = Coordinator::new(1)
        .unwrap()
        .resume_publication(
            access,
            f.folder.root(),
            f.execution.witness.clone(),
            f.execution.key.clone(),
        )
        .await
        .unwrap();
    let (recovery, head) = recovery.acknowledge_publication().await.unwrap();
    assert!(matches!(
        recovery
            .install_tree_files(&head, f.plan.clone(), inventory, guard)
            .await,
        Err(local_browser::Error::Denied)
    ));
    assert_eq!(fs::read(private(&f).join("b-0")).unwrap(), b"original\n");
    assert!(!f.folder.root().join("file.txt").exists());
    assert_eq!(progress(&f.folder.root(), &f.plan).unwrap()["phases"][0], 1);
}
struct Blocked {
    inner: Arc<dyn PhysicalGuard>,
    root: PathBuf,
    plan: TreeCheckoutPlanV1,
    entered: tokio::sync::Notify,
    released: Mutex<bool>,
    wake: Condvar,
}
#[async_trait::async_trait]
impl PhysicalGuard for Blocked {
    async fn validate(
        &self,
        binding: &RecoveryBinding,
        expected: m::RecordDigest,
    ) -> local_browser::Result<()> {
        self.inner.validate(binding, expected).await
    }
    async fn observe(
        &self,
        binding: &RecoveryBinding,
    ) -> local_browser::Result<PhysicalInventoryV1> {
        if progress(&self.root, &self.plan).is_some() {
            self.entered.notify_one();
            let mut released = self.released.lock().unwrap();
            while !*released {
                released = self.wake.wait(released).unwrap();
            }
        }
        self.inner.observe(binding).await
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn canceled_worker_keeps_provider_admission_and_foreground_lock_until_exit() {
    let f = Input::build(true).await;
    let inventory = f.inventory().await;
    let (recovery, _) = published(&f).await;
    drop(recovery);
    let coordinator = Coordinator::new(1).unwrap();
    let recovery = coordinator
        .resume_publication(
            f.folder.access(),
            f.folder.root(),
            f.execution.witness.clone(),
            f.execution.key.clone(),
        )
        .await
        .unwrap();
    let (recovery, head) = recovery.acknowledge_publication().await.unwrap();
    let guard = Arc::new(Blocked {
        inner: physical(&f),
        root: f.folder.root(),
        plan: f.plan.clone(),
        entered: tokio::sync::Notify::new(),
        released: Mutex::new(false),
        wake: Condvar::new(),
    });
    let plan = f.plan.clone();
    let worker_guard = guard.clone();
    let task = tokio::spawn(async move {
        recovery
            .install_tree_files(&head, plan, inventory, worker_guard)
            .await
    });
    tokio::time::timeout(std::time::Duration::from_secs(10), guard.entered.notified())
        .await
        .unwrap();
    task.abort();
    assert!(task.await.err().unwrap().is_cancelled());
    assert!(matches!(
        coordinator
            .resume_publication(
                f.folder.access(),
                f.folder.root(),
                f.execution.witness.clone(),
                f.execution.key.clone()
            )
            .await,
        Err(local_browser::Error::Busy)
    ));
    assert!(matches!(
        Coordinator::new(1)
            .unwrap()
            .resume_publication(
                f.folder.access(),
                f.folder.root(),
                f.execution.witness.clone(),
                f.execution.key.clone()
            )
            .await,
        Err(local_browser::Error::Busy)
    ));
    *guard.released.lock().unwrap() = true;
    guard.wake.notify_all();
    let resumed = tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            match coordinator
                .resume_publication(
                    f.folder.access(),
                    f.folder.root(),
                    f.execution.witness.clone(),
                    f.execution.key.clone(),
                )
                .await
            {
                Ok(recovery) => break recovery,
                Err(local_browser::Error::Busy) => {
                    tokio::time::sleep(std::time::Duration::from_millis(20)).await
                }
                Err(error) => panic!("{error:?}"),
            }
        }
    })
    .await
    .unwrap();
    assert_eq!(
        fs::read(f.folder.root().join("file.txt")).unwrap(),
        b"original\n"
    );
    assert!(!f.folder.root().join("src").exists());
    drop(resumed);
}
