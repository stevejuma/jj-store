use super::*;
use installation::{physical, published};
use jj_store_jj::local_execution::{InstalledTreeFiles, PhysicalInventoryV1, RecoveredHistory};
use std::os::unix::fs::MetadataExt as _;
fn private(f: &Input) -> PathBuf {
    f.folder.root().join(format!(
        ".jj/working_copy/.lapis-stack-native-{}",
        f.plan.id().unwrap()
    ))
}
async fn install(f: &Input) -> (RecoveredHistory, InstalledTreeFiles, PhysicalInventoryV1) {
    let inventory = f.inventory().await;
    let (recovery, head) = published(f).await;
    let (recovery, proof) = recovery
        .install_tree_files(&head, f.plan.clone(), inventory.clone(), physical(f))
        .await
        .unwrap();
    (recovery, proof, inventory)
}
async fn resume(
    f: &Input,
    inventory: PhysicalInventoryV1,
) -> local_browser::Result<RecoveredHistory> {
    Coordinator::new(1)
        .unwrap()
        .resume_tree_checkout(
            f.folder.access(),
            f.folder.root(),
            f.execution.witness.clone(),
            f.execution.key.clone(),
            f.plan.clone(),
            inventory,
        )
        .await
}
#[tokio::test]
async fn native_tree_completion_and_exact_replay_preserve_installed_inodes() {
    let f = Input::build(true).await;
    let (recovery, files, inventory) = install(&f).await;
    let old = fs::read(f.folder.root().join(".jj/working_copy/tree_state")).unwrap();
    let inode = fs::metadata(f.folder.root().join("file.txt"))
        .unwrap()
        .ino();
    let (recovery, proof) = recovery
        .complete_tree_checkout(files, f.plan.clone(), inventory.clone(), physical(&f))
        .await
        .unwrap();
    assert_eq!(proof.plan(), f.plan.id().unwrap());
    assert_eq!(proof.inventory(), inventory.id().unwrap());
    assert_eq!(proof.operation, f.plan.candidate().0);
    assert_ne!(
        old,
        fs::read(f.folder.root().join(".jj/working_copy/tree_state")).unwrap()
    );
    let native = ["tree_state", "checkout"].map(|name| {
        let path = f.folder.root().join(".jj/working_copy").join(name);
        (fs::metadata(&path).unwrap().ino(), fs::read(path).unwrap())
    });
    recovery.validate_binding().await.unwrap();
    drop(recovery);
    let state = local_browser::Executor::new(1)
        .unwrap()
        .working_tree_current(f.folder.access(), f.folder.root(), proof.operation)
        .await
        .unwrap();
    assert_eq!(state.checkout.trees, [f.plan.trees().1]);
    assert_eq!(state.checkout.operation, proof.operation);
    assert_eq!(state.baseline.commit, f.plan.commits().1);
    let (recovery, repeated) = resume(&f, inventory)
        .await
        .unwrap()
        .finish_tree_checkout(physical(&f))
        .await
        .unwrap();
    assert_eq!(proof, repeated);
    assert_eq!(
        inode,
        fs::metadata(f.folder.root().join("file.txt"))
            .unwrap()
            .ino()
    );
    assert_eq!(
        fs::read(installation::private(&f).join("b-0")).unwrap(),
        b"original\n"
    );
    assert_eq!(
        native,
        ["tree_state", "checkout"].map(|name| {
            let path = f.folder.root().join(".jj/working_copy").join(name);
            (fs::metadata(&path).unwrap().ino(), fs::read(path).unwrap())
        })
    );
    drop(recovery);
    // Stock JJ consumes our exact metadata without a spurious snapshot rewrite.
    assert_eq!(
        fixture::stock(
            &f.folder.root(),
            &["log", "--no-graph", "-r", "@", "-T", "commit_id"]
        ),
        f.plan.commits().1.to_string()
    );
}
fn file_states(f: &Input) -> Vec<(String, jj_lib::local_working_copy::FileState)> {
    use jj_lib::{
        config::StackedConfig, default_backend_factories::default_backend_factories,
        settings::UserSettings,
    };
    let settings =
        UserSettings::from_config_and_home_dir(StackedConfig::with_defaults(), None).unwrap();
    let loader = jj_lib::repo::RepoLoader::init_from_file_system(
        &settings,
        &f.folder.root().join(".jj/repo"),
        &default_backend_factories(),
    )
    .unwrap();
    // Test-only upstream decoding, after the provider foreground owner exited.
    let wc = jj_lib::local_working_copy::LocalWorkingCopy::load(
        loader.store().clone(),
        f.folder.root(),
        f.folder.root().join(".jj/working_copy"),
        &settings,
    )
    .unwrap();
    wc.file_states()
        .unwrap()
        .iter()
        .map(|(path, state)| (path.as_internal_file_string().to_owned(), state))
        .collect()
}
#[tokio::test]
async fn native_file_states_use_component_order_and_preserve_unaffected_records() {
    let f = Input::build_effects(
        true,
        vec![
            ("file.txt", None),
            ("a.b", Some((b"flat", false))),
            ("a/z", Some((b"\0\xff", true))),
        ],
        |_| {},
    )
    .await;
    let before = file_states(&f);
    let (recovery, files, inventory) = install(&f).await;
    let (recovery, _) = recovery
        .complete_tree_checkout(files, f.plan.clone(), inventory, physical(&f))
        .await
        .unwrap();
    drop(recovery);
    let after = file_states(&f);
    assert_eq!(
        after
            .iter()
            .map(|(path, _)| path.as_str())
            .collect::<Vec<_>>(),
        [".gitignore", "a/z", "a.b"]
    );
    assert_eq!(after[0], before[0]);
    assert!(!f.folder.root().join("file.txt").exists());
    assert_eq!(after[1].1.size, 2);
    assert!(matches!(
        after[1].1.file_type,
        jj_lib::local_working_copy::FileType::Normal { .. }
    ));
    assert_ne!(
        fs::metadata(f.folder.root().join("a/z")).unwrap().mode() & 0o111,
        0
    );
    assert_eq!(
        fs::read(f.folder.root().join("ignored.txt")).unwrap(),
        b"ignored obstacle\n"
    );
}
type Action = Box<dyn Fn(&PathBuf) + Send + Sync>;
struct Hook {
    inner: Arc<dyn PhysicalGuard>,
    root: PathBuf,
    action: Action,
}
#[async_trait::async_trait]
impl PhysicalGuard for Hook {
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
        (self.action)(&self.root);
        self.inner.observe(binding).await
    }
}
#[tokio::test]
async fn changed_physical_file_blocks_before_native_frame_or_selection() {
    let f = Input::build(true).await;
    let (recovery, files, inventory) = install(&f).await;
    let before = ["tree_state", "checkout"]
        .map(|p| fs::read(f.folder.root().join(".jj/working_copy").join(p)).unwrap());
    fs::write(f.folder.root().join("ignored.txt"), b"new editor bytes").unwrap();
    assert!(matches!(
        recovery
            .complete_tree_checkout(files, f.plan.clone(), inventory, physical(&f))
            .await,
        Err(local_browser::Error::EpochChanged)
    ));
    assert_eq!(
        before,
        ["tree_state", "checkout"]
            .map(|p| fs::read(f.folder.root().join(".jj/working_copy").join(p)).unwrap())
    );
    assert!(!private(&f).exists());
}

fn exit_worker(f: &Input, inventory: &PhysicalInventoryV1, phase: &str) {
    private_write(
        &f.folder
            .root()
            .join(".jj/working_copy/completion-test-input"),
        &serde_json::to_vec(&(f.execution.clone(), f.plan.clone(), inventory.clone())).unwrap(),
    );
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--ignored",
            "--exact",
            "completion::exit_at_native_boundary",
            "--nocapture",
        ])
        .env("JJ_STORE_NATIVE_ROOT", f.folder.root())
        .env("JJ_STORE_NATIVE_PHASE", phase)
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(97),
        "{phase}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
#[tokio::test]
async fn checkout_selected_before_tree_and_missing_completion_evidence_are_rejected() {
    for fault in [
        "checkout-first",
        "missing-frame",
        "missing-install-progress",
    ] {
        let f = Input::build(true).await;
        let (recovery, _, inventory) = install(&f).await;
        drop(recovery);
        exit_worker(
            &f,
            &inventory,
            if fault == "checkout-first" {
                "frame"
            } else {
                "tree"
            },
        );
        match fault {
            "checkout-first" => fs::rename(
                private(&f).join("checkout"),
                f.folder.root().join(".jj/working_copy/checkout"),
            )
            .unwrap(),
            "missing-frame" => fs::remove_file(private(&f).join("progress")).unwrap(),
            "missing-install-progress" => {
                fs::remove_file(installation::private(&f).join("progress")).unwrap()
            }
            _ => unreachable!(),
        }
        let native = ["tree_state", "checkout"]
            .map(|p| fs::read(f.folder.root().join(".jj/working_copy").join(p)).unwrap());
        assert!(resume(&f, inventory).await.is_err(), "{fault}");
        assert_eq!(
            native,
            ["tree_state", "checkout"]
                .map(|p| fs::read(f.folder.root().join(".jj/working_copy").join(p)).unwrap())
        );
    }
}
#[tokio::test]
async fn process_loss_before_frame_and_after_each_native_selection_resumes_exact_input() {
    for phase in ["prepared", "frame", "tree", "checkout"] {
        let f = Input::build(true).await;
        let (recovery, _, inventory) = install(&f).await;
        drop(recovery);
        let inode = fs::metadata(f.folder.root().join("file.txt"))
            .unwrap()
            .ino();
        exit_worker(&f, &inventory, phase);
        let recovery = if phase == "prepared" {
            assert!(matches!(
                resume(&f, inventory.clone()).await,
                Err(local_browser::Error::NotFound)
            ));
            let (recovery, head) = published(&f).await;
            let (recovery, files) = recovery
                .install_tree_files(&head, f.plan.clone(), inventory.clone(), physical(&f))
                .await
                .unwrap();
            recovery
                .complete_tree_checkout(files, f.plan.clone(), inventory, physical(&f))
                .await
                .unwrap()
                .0
        } else {
            resume(&f, inventory)
                .await
                .unwrap()
                .finish_tree_checkout(physical(&f))
                .await
                .unwrap()
                .0
        };
        assert_eq!(
            inode,
            fs::metadata(f.folder.root().join("file.txt"))
                .unwrap()
                .ino()
        );
        assert_eq!(
            fs::read(f.folder.root().join("file.txt")).unwrap(),
            b"candidate\n"
        );
        assert_eq!(
            fs::read(installation::private(&f).join("b-0")).unwrap(),
            b"original\n"
        );
        assert!(!private(&f).join("tree_state").exists());
        assert!(!private(&f).join("checkout").exists());
        drop(recovery);
        let state = local_browser::Executor::new(1)
            .unwrap()
            .working_tree_current(f.folder.access(), f.folder.root(), f.plan.candidate().0)
            .await
            .unwrap();
        assert_eq!(state.checkout.trees, [f.plan.trees().1]);
    }
}
#[tokio::test]
async fn partial_initial_frame_accepts_only_exact_retained_prefix() {
    for valid in [true, false] {
        let f = Input::build(true).await;
        let (recovery, _, inventory) = install(&f).await;
        drop(recovery);
        exit_worker(&f, &inventory, "frame");
        let frame = fs::read(private(&f).join("progress")).unwrap();
        fs::rename(private(&f).join("progress"), private(&f).join("next")).unwrap();
        private_write(
            &private(&f).join("next"),
            if valid {
                &frame[..frame.len() - 5]
            } else {
                b"corrupt"
            },
        );
        let native = ["tree_state", "checkout"]
            .map(|name| fs::metadata(private(&f).join(name)).unwrap().ino());
        let (recovery, head) = published(&f).await;
        let (recovery, files) = recovery
            .install_tree_files(&head, f.plan.clone(), inventory.clone(), physical(&f))
            .await
            .unwrap();
        let result = recovery
            .complete_tree_checkout(files, f.plan.clone(), inventory, physical(&f))
            .await;
        if valid {
            let (recovery, _) = result.unwrap();
            assert_eq!(frame, fs::read(private(&f).join("progress")).unwrap());
            assert_eq!(
                native,
                ["tree_state", "checkout"].map(|name| fs::metadata(
                    f.folder.root().join(".jj/working_copy").join(name)
                )
                .unwrap()
                .ino())
            );
            drop(recovery);
        } else {
            assert!(matches!(result, Err(local_browser::Error::Corrupt)));
            assert_eq!(fs::read(private(&f).join("next")).unwrap(), b"corrupt");
            assert!(!private(&f).join("progress").exists());
        }
    }
}
#[tokio::test]
async fn native_replacement_policy_and_backup_changes_block_remaining_checkout() {
    for fault in [
        "native-inode",
        "native-mode",
        "policy",
        "backup",
        "frame",
        "private-directory",
        "physical",
    ] {
        let f = Input::build(true).await;
        let (recovery, files, inventory) = install(&f).await;
        let root = f.folder.root();
        let path = private(&f);
        let before_checkout = fs::read(root.join(".jj/working_copy/checkout")).unwrap();
        let once = std::sync::atomic::AtomicBool::new(false);
        let hook = Arc::new(Hook {
            inner: physical(&f),
            root: root.clone(),
            action: Box::new(move |root| {
                if path.join("progress").exists()
                    && !path.join("tree_state").exists()
                    && !once.swap(true, std::sync::atomic::Ordering::SeqCst)
                {
                    match fault {
                        "native-inode" => {
                            let native = root.join(".jj/working_copy/tree_state");
                            let bytes = fs::read(&native).unwrap();
                            fs::rename(&native, root.join(".jj/working_copy/editor-tree")).unwrap();
                            fs::write(native, bytes).unwrap();
                        }
                        "native-mode" => {
                            use std::os::unix::fs::PermissionsExt as _;
                            fs::set_permissions(
                                root.join(".jj/working_copy/tree_state"),
                                fs::Permissions::from_mode(0o644),
                            )
                            .unwrap();
                        }
                        "policy" => {
                            fs::write(root.join(".git/info/exclude"), b"new-policy\n").unwrap();
                        }
                        "backup" => {
                            let stages = fs::read_dir(root.join(".jj/working_copy"))
                                .unwrap()
                                .filter_map(|p| p.ok())
                                .find(|p| {
                                    p.file_name()
                                        .to_str()
                                        .unwrap()
                                        .starts_with(".lapis-stack-tree-")
                                })
                                .unwrap();
                            fs::write(stages.path().join("b-0"), b"changed-original").unwrap();
                        }
                        "frame" => {
                            fs::write(path.join("progress"), b"corrupt").unwrap();
                        }
                        "private-directory" => {
                            fs::rename(&path, path.with_extension("replaced")).unwrap();
                            fs::create_dir(&path).unwrap();
                        }
                        "physical" => {
                            fs::write(root.join("ignored.txt"), b"editor-after-tree").unwrap();
                        }
                        _ => unreachable!(),
                    }
                }
            }),
        });
        assert!(
            recovery
                .complete_tree_checkout(files, f.plan.clone(), inventory.clone(), hook)
                .await
                .is_err(),
            "{fault}"
        );
        assert_eq!(
            fs::read(root.join(".jj/working_copy/checkout")).unwrap(),
            before_checkout,
            "{fault}"
        );
        let result = resume(&f, inventory).await;
        if let Ok(recovery) = result {
            assert!(
                recovery.finish_tree_checkout(physical(&f)).await.is_err(),
                "{fault}"
            );
        }
    }
}
struct Access {
    metadata: PathBuf,
    binding: RecoveryBinding,
    denied: Option<Arc<std::sync::atomic::AtomicBool>>,
}
#[async_trait::async_trait]
impl local_browser::Access for Access {
    fn repository(&self) -> m::RepositoryId {
        self.binding.repository
    }
    fn generation(&self) -> u64 {
        self.binding.generation
    }
    fn metadata_path(&self) -> &std::path::Path {
        &self.metadata
    }
    async fn authorize(&self) -> local_browser::Result<()> {
        if self
            .denied
            .as_ref()
            .is_some_and(|denied| denied.load(std::sync::atomic::Ordering::SeqCst))
        {
            Err(local_browser::Error::Denied)
        } else {
            Ok(())
        }
    }
}
#[tokio::test]
#[ignore = "isolated worker exits at provider-owned native metadata boundaries"]
async fn exit_at_native_boundary() {
    let root = PathBuf::from(std::env::var_os("JJ_STORE_NATIVE_ROOT").unwrap());
    let phase = std::env::var("JJ_STORE_NATIVE_PHASE").unwrap();
    let (execution, plan, inventory): (LocalExecutionV1, TreeCheckoutPlanV1, PhysicalInventoryV1) =
        serde_json::from_slice(
            &fs::read(root.join(".jj/working_copy/completion-test-input")).unwrap(),
        )
        .unwrap();
    let access: Arc<dyn local_browser::Access> = Arc::new(Access {
        metadata: root.join(".jj/repo"),
        binding: execution.witness.binding(),
        denied: None,
    });
    let physical =
        jj_store_service::working_tree::Observer::new(local_browser::Executor::new(1).unwrap(), 1)
            .unwrap()
            .physical_guard(access.clone(), root.clone());
    let recovery = Coordinator::new(1)
        .unwrap()
        .resume_publication(access, root.clone(), execution.witness, execution.key)
        .await
        .unwrap();
    let (recovery, head) = recovery.acknowledge_publication().await.unwrap();
    let (recovery, files) = recovery
        .install_tree_files(&head, plan.clone(), inventory.clone(), physical.clone())
        .await
        .unwrap();
    let private = root.join(format!(
        ".jj/working_copy/.lapis-stack-native-{}",
        plan.id().unwrap()
    ));
    let hook = Arc::new(Hook {
        inner: physical,
        root,
        action: Box::new(move |_| {
            let frame = private.join("progress").exists();
            let tree = private.join("tree_state").exists();
            let checkout = private.join("checkout").exists();
            if match phase.as_str() {
                "prepared" => !frame && tree && checkout,
                "frame" => frame && tree && checkout,
                "tree" => frame && !tree && checkout,
                "checkout" => frame && !tree && !checkout,
                _ => false,
            } {
                std::process::exit(97);
            }
        }),
    });
    recovery
        .complete_tree_checkout(files, plan, inventory, hook)
        .await
        .unwrap();
    panic!("requested native boundary was not reached");
}

#[tokio::test]
async fn authorization_loss_after_tree_selection_retains_original_checkout_and_resumes() {
    use std::sync::atomic::{AtomicBool, Ordering};
    let f = Input::build(true).await;
    let (recovery, _, inventory) = install(&f).await;
    drop(recovery);
    let denied = Arc::new(AtomicBool::new(false));
    let access: Arc<dyn local_browser::Access> = Arc::new(Access {
        metadata: f.folder.root().join(".jj/repo"),
        binding: f.execution.witness.binding(),
        denied: Some(denied.clone()),
    });
    let guard =
        jj_store_service::working_tree::Observer::new(local_browser::Executor::new(1).unwrap(), 1)
            .unwrap()
            .physical_guard(access.clone(), f.folder.root());
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
    let (recovery, files) = recovery
        .install_tree_files(&head, f.plan.clone(), inventory.clone(), guard.clone())
        .await
        .unwrap();
    let path = private(&f);
    let gate = denied.clone();
    let hook = Arc::new(Hook {
        inner: guard,
        root: f.folder.root(),
        action: Box::new(move |_| {
            if path.join("progress").exists() && !path.join("tree_state").exists() {
                gate.store(true, Ordering::SeqCst);
            }
        }),
    });
    let before = fs::read(f.folder.root().join(".jj/working_copy/checkout")).unwrap();
    assert!(matches!(
        recovery
            .complete_tree_checkout(files, f.plan.clone(), inventory.clone(), hook)
            .await,
        Err(local_browser::Error::Denied)
    ));
    assert_eq!(
        before,
        fs::read(f.folder.root().join(".jj/working_copy/checkout")).unwrap()
    );
    assert!(private(&f).join("checkout").exists());
    denied.store(false, Ordering::SeqCst);
    let (recovery, _) = resume(&f, inventory)
        .await
        .unwrap()
        .finish_tree_checkout(physical(&f))
        .await
        .unwrap();
    drop(recovery);
}
struct Blocking {
    inner: Arc<dyn PhysicalGuard>,
    private: PathBuf,
    entered: tokio::sync::Notify,
    released: std::sync::Mutex<bool>,
    wake: std::sync::Condvar,
}
#[async_trait::async_trait]
impl PhysicalGuard for Blocking {
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
        if self.private.join("progress").exists() {
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
async fn canceled_native_worker_retains_admission_and_foreground_until_exit() {
    let f = Input::build(true).await;
    let (recovery, _, inventory) = install(&f).await;
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
    let (recovery, files) = recovery
        .install_tree_files(&head, f.plan.clone(), inventory.clone(), physical(&f))
        .await
        .unwrap();
    let guard = Arc::new(Blocking {
        inner: physical(&f),
        private: private(&f),
        entered: tokio::sync::Notify::new(),
        released: std::sync::Mutex::new(false),
        wake: std::sync::Condvar::new(),
    });
    let worker_guard = guard.clone();
    let plan = f.plan.clone();
    let saved_inventory = inventory.clone();
    let task = tokio::spawn(async move {
        recovery
            .complete_tree_checkout(files, plan, saved_inventory, worker_guard)
            .await
    });
    tokio::time::timeout(std::time::Duration::from_secs(10), guard.entered.notified())
        .await
        .unwrap();
    task.abort();
    assert!(task.await.err().unwrap().is_cancelled());
    assert!(matches!(
        coordinator
            .resume_tree_checkout(
                f.folder.access(),
                f.folder.root(),
                f.execution.witness.clone(),
                f.execution.key.clone(),
                f.plan.clone(),
                inventory.clone()
            )
            .await,
        Err(local_browser::Error::Busy)
    ));
    assert!(matches!(
        resume(&f, inventory.clone()).await,
        Err(local_browser::Error::Busy)
    ));
    *guard.released.lock().unwrap() = true;
    guard.wake.notify_all();
    let resumed = tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            match resume(&f, inventory.clone()).await {
                Ok(recovery) => break recovery,
                Err(local_browser::Error::Busy) => tokio::task::yield_now().await,
                Err(error) => panic!("unexpected recovery result: {error:?}"),
            }
        }
    })
    .await
    .unwrap();
    let (recovery, _) = resumed.finish_tree_checkout(physical(&f)).await.unwrap();
    drop(recovery);
}
