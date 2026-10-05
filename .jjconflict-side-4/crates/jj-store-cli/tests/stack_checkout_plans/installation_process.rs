use super::*;
use installation::{Hook, physical, private, published};
use jj_store_jj::local_execution::PhysicalInventoryV1;
use std::{path::Path, sync::atomic::AtomicBool};
const PHASES: [&str; 10] = [
    "prepared",
    "directory-intent",
    "directory-moved",
    "backup-intent",
    "backup-moved",
    "update-intent",
    "update-moved",
    "add-intent",
    "add-moved",
    "installed",
];
#[tokio::test]
async fn abrupt_loss_before_and_after_every_physical_move_replays_original_input() {
    use std::os::unix::fs::MetadataExt as _;
    for phase in PHASES {
        let f = Input::build(true).await;
        let inventory = f.inventory().await;
        let (recovery, _) = published(&f).await;
        drop(recovery);
        let checkout = fs::read(f.folder.root().join(".jj/working_copy/checkout")).unwrap();
        let tree = fs::read(f.folder.root().join(".jj/working_copy/tree_state")).unwrap();
        let input = f.folder.root().join(".jj/working_copy/install-test-input");
        private_write(
            &input,
            &serde_json::to_vec(&(f.execution.clone(), f.plan.clone(), inventory.clone())).unwrap(),
        );
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--ignored",
                "--exact",
                "installation_process::exit_at_physical_boundary",
                "--nocapture",
            ])
            .env("JJ_STORE_INSTALL_ROOT", f.folder.root())
            .env("JJ_STORE_INSTALL_PHASE", phase)
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(96),
            "{phase}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            fs::read(f.folder.root().join(".jj/working_copy/checkout")).unwrap(),
            checkout
        );
        assert_eq!(
            fs::read(f.folder.root().join(".jj/working_copy/tree_state")).unwrap(),
            tree
        );
        let original_stage = private(&f).join("f-0");
        let candidate_inode = fs::metadata(if original_stage.exists() {
            original_stage
        } else {
            f.folder.root().join("file.txt")
        })
        .unwrap()
        .ino();
        let (recovery, head) = published(&f).await;
        let (recovery, _) = recovery
            .install_tree_files(&head, f.plan.clone(), inventory, physical(&f))
            .await
            .unwrap();
        assert_eq!(
            fs::read(f.folder.root().join("file.txt")).unwrap(),
            b"candidate\n"
        );
        assert_eq!(fs::read(f.folder.root().join("src/new")).unwrap(), b"new\n");
        assert_eq!(
            fs::metadata(f.folder.root().join("file.txt"))
                .unwrap()
                .ino(),
            candidate_inode
        );
        assert_eq!(fs::read(private(&f).join("b-0")).unwrap(), b"original\n");
        assert_eq!(
            fs::read(f.folder.root().join(".jj/working_copy/checkout")).unwrap(),
            checkout
        );
        assert_eq!(
            fs::read(f.folder.root().join(".jj/working_copy/tree_state")).unwrap(),
            tree
        );
        drop(recovery);
    }
}
struct Access {
    path: PathBuf,
    binding: RecoveryBinding,
}
#[tokio::test]
async fn partial_move_acknowledgement_replays_only_exact_saved_next_prefix() {
    use std::os::unix::fs::MetadataExt as _;
    for valid in [true, false] {
        let f = Input::build(true).await;
        let inventory = f.inventory().await;
        let (recovery, _) = published(&f).await;
        drop(recovery);
        private_write(
            &f.folder.root().join(".jj/working_copy/install-test-input"),
            &serde_json::to_vec(&(f.execution.clone(), f.plan.clone(), inventory.clone())).unwrap(),
        );
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--ignored",
                "--exact",
                "installation_process::exit_at_physical_boundary",
                "--nocapture",
            ])
            .env("JJ_STORE_INSTALL_ROOT", f.folder.root())
            .env("JJ_STORE_INSTALL_PHASE", "backup-moved")
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(96));
        let before = fs::metadata(private(&f).join("b-0")).unwrap().ino();
        let state = fs::read_to_string(private(&f).join("progress")).unwrap();
        let after = state.replace(
            "\"phases\":[0,0],\"intent\":{\"backup\":0}",
            "\"phases\":[1,0],\"intent\":null",
        );
        assert_ne!(state, after);
        private_write(
            &private(&f).join("next"),
            if valid {
                &after.as_bytes()[..after.len() - 5]
            } else {
                b"corrupt-next"
            },
        );
        let (recovery, head) = published(&f).await;
        let result = recovery
            .install_tree_files(&head, f.plan.clone(), inventory, physical(&f))
            .await;
        if valid {
            let (recovery, _) = result.unwrap();
            assert_eq!(
                fs::read(f.folder.root().join("file.txt")).unwrap(),
                b"candidate\n"
            );
            assert!(!private(&f).join("next").exists());
            drop(recovery);
        } else {
            assert!(matches!(result, Err(local_browser::Error::Corrupt)));
            assert!(!f.folder.root().join("file.txt").exists());
            assert_eq!(fs::read(private(&f).join("f-0")).unwrap(), b"candidate\n");
            assert_eq!(
                fs::read_to_string(private(&f).join("progress")).unwrap(),
                state
            );
            assert_eq!(fs::read(private(&f).join("next")).unwrap(), b"corrupt-next");
        }
        assert_eq!(fs::metadata(private(&f).join("b-0")).unwrap().ino(), before);
        assert_eq!(fs::read(private(&f).join("b-0")).unwrap(), b"original\n");
    }
}
#[async_trait::async_trait]
impl local_browser::Access for Access {
    fn repository(&self) -> m::RepositoryId {
        self.binding.repository
    }
    fn generation(&self) -> u64 {
        self.binding.generation
    }
    fn metadata_path(&self) -> &Path {
        &self.path
    }
    async fn authorize(&self) -> local_browser::Result<()> {
        Ok(())
    }
}
#[tokio::test]
#[ignore = "isolated worker exits at a saved-intent physical boundary"]
async fn exit_at_physical_boundary() {
    let root = PathBuf::from(std::env::var_os("JJ_STORE_INSTALL_ROOT").unwrap());
    let phase = std::env::var("JJ_STORE_INSTALL_PHASE").unwrap();
    let (execution, plan, inventory): (LocalExecutionV1, TreeCheckoutPlanV1, PhysicalInventoryV1) =
        serde_json::from_slice(&fs::read(root.join(".jj/working_copy/install-test-input")).unwrap())
            .unwrap();
    let access: Arc<dyn local_browser::Access> = Arc::new(Access {
        path: root.join(".jj/repo"),
        binding: execution.witness.binding(),
    });
    let inner =
        jj_store_service::working_tree::Observer::new(local_browser::Executor::new(1).unwrap(), 1)
            .unwrap()
            .physical_guard(access.clone(), root.clone());
    let selected_plan = plan.id().unwrap();
    let hook = Arc::new(Hook {
        inner,
        root: root.clone(),
        plan: plan.clone(),
        fired: AtomicBool::new(false),
        action: Box::new(move |root, state| {
            let intent = &state["intent"];
            let moved = match phase.as_str() {
                "prepared" => {
                    intent.is_null() && state["dirs_done"] == 0 && state["phases"][0] == 0
                }
                "directory-intent" => {
                    intent.get("directory").is_some() && !root.join("src").exists()
                }
                "directory-moved" => intent.get("directory").is_some() && root.join("src").exists(),
                "backup-intent" => {
                    intent.get("backup") == Some(&serde_json::json!(0))
                        && root.join("file.txt").exists()
                }
                "backup-moved" => {
                    intent.get("backup") == Some(&serde_json::json!(0))
                        && !root.join("file.txt").exists()
                }
                "update-intent" => {
                    intent.get("install") == Some(&serde_json::json!(0))
                        && root
                            .join(format!(
                                ".jj/working_copy/.lapis-stack-tree-{selected_plan}/f-0"
                            ))
                            .exists()
                }
                "update-moved" => {
                    intent.get("install") == Some(&serde_json::json!(0))
                        && !root
                            .join(format!(
                                ".jj/working_copy/.lapis-stack-tree-{selected_plan}/f-0"
                            ))
                            .exists()
                }
                "add-intent" => {
                    intent.get("install") == Some(&serde_json::json!(1))
                        && !root.join("src/new").exists()
                }
                "add-moved" => {
                    intent.get("install") == Some(&serde_json::json!(1))
                        && root.join("src/new").exists()
                }
                "installed" => {
                    intent.is_null() && state["phases"][0] == 2 && state["phases"][1] == 2
                }
                _ => false,
            };
            if moved {
                std::process::exit(96);
            }
        }),
    });
    let recovery = Coordinator::new(1)
        .unwrap()
        .resume_publication(access, root, execution.witness, execution.key)
        .await
        .unwrap();
    let (recovery, head) = recovery.acknowledge_publication().await.unwrap();
    let _ = recovery
        .install_tree_files(&head, plan, inventory, hook)
        .await
        .unwrap();
    panic!("requested installation boundary was not reached");
}
