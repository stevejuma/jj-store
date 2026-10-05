use super::*;
use jj_store_jj::local_execution::{PhysicalInventoryV1, PublishedHead, RecoveredHistory};
use std::{
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};
pub(super) fn physical(f: &Input) -> Arc<dyn PhysicalGuard> {
    jj_store_service::working_tree::Observer::new(local_browser::Executor::new(1).unwrap(), 1)
        .unwrap()
        .physical_guard(f.folder.access(), f.folder.root())
}
pub(super) async fn published(f: &Input) -> (RecoveredHistory, PublishedHead) {
    Coordinator::new(1)
        .unwrap()
        .resume_publication(
            f.folder.access(),
            f.folder.root(),
            f.execution.witness.clone(),
            f.execution.key.clone(),
        )
        .await
        .unwrap()
        .restore_extras()
        .await
        .unwrap()
        .restore_indexes()
        .await
        .unwrap()
        .publish_head()
        .await
        .unwrap()
}
pub(super) fn private(f: &Input) -> PathBuf {
    f.folder.root().join(format!(
        ".jj/working_copy/.lapis-stack-tree-{}",
        f.plan.id().unwrap()
    ))
}
pub(super) fn progress(root: &Path, plan: &TreeCheckoutPlanV1) -> Option<serde_json::Value> {
    fs::read(root.join(format!(
        ".jj/working_copy/.lapis-stack-tree-{}/progress",
        plan.id().unwrap()
    )))
    .ok()
    .map(|bytes| serde_json::from_slice(&bytes).unwrap())
}
type Action = Box<dyn Fn(&Path, &serde_json::Value) + Send + Sync>;
pub(super) struct Hook {
    pub inner: Arc<dyn PhysicalGuard>,
    pub root: PathBuf,
    pub plan: TreeCheckoutPlanV1,
    pub fired: AtomicBool,
    pub action: Action,
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
        if !self.fired.load(Ordering::SeqCst)
            && let Some(state) = progress(&self.root, &self.plan)
        {
            (self.action)(&self.root, &state);
        }
        self.inner.observe(binding).await
    }
}
#[tokio::test]
async fn retained_tree_files_install_with_durable_progress_and_unchanged_native_metadata() {
    let f = Input::build(true).await;
    let inventory = f.inventory().await;
    let observer =
        jj_store_service::working_tree::Observer::new(local_browser::Executor::new(1).unwrap(), 1)
            .unwrap();
    let guard = observer.physical_guard(f.folder.access(), f.folder.root());
    let recovery = Coordinator::new(1)
        .unwrap()
        .resume_publication(
            f.folder.access(),
            f.folder.root(),
            f.execution.witness.clone(),
            f.execution.key.clone(),
        )
        .await
        .unwrap()
        .restore_extras()
        .await
        .unwrap()
        .restore_indexes()
        .await
        .unwrap();
    let (recovery, published) = recovery.publish_head().await.unwrap();
    let checkout = fs::read(f.folder.root().join(".jj/working_copy/checkout")).unwrap();
    let state = fs::read(f.folder.root().join(".jj/working_copy/tree_state")).unwrap();
    let (recovery, installed) = recovery
        .install_tree_files(&published, f.plan.clone(), inventory, guard)
        .await
        .unwrap();
    assert_eq!(
        fs::read(f.folder.root().join("file.txt")).unwrap(),
        b"candidate\n"
    );
    assert_eq!(fs::read(f.folder.root().join("src/new")).unwrap(), b"new\n");
    assert_eq!(installed.plan(), f.plan.id().unwrap());
    assert_eq!(
        fs::read(f.folder.root().join(".jj/working_copy/checkout")).unwrap(),
        checkout
    );
    assert_eq!(
        fs::read(f.folder.root().join(".jj/working_copy/tree_state")).unwrap(),
        state
    );
    drop(recovery);
}
#[tokio::test]
async fn installed_tree_replay_retains_inodes_and_all_original_backups() {
    use std::os::unix::fs::MetadataExt as _;
    let f = Input::build(true).await;
    let inventory = f.inventory().await;
    let (recovery, head) = published(&f).await;
    let (recovery, proof) = recovery
        .install_tree_files(&head, f.plan.clone(), inventory.clone(), physical(&f))
        .await
        .unwrap();
    assert_eq!(fs::read(private(&f).join("b-0")).unwrap(), b"original\n");
    let before = [
        f.folder.root().join("file.txt"),
        f.folder.root().join("src/new"),
        private(&f).join("b-0"),
    ]
    .map(|p| fs::metadata(p).unwrap().ino());
    let state = fs::read(private(&f).join("progress")).unwrap();
    drop(recovery);
    let (recovery, head) = published(&f).await;
    let (recovery, again) = recovery
        .install_tree_files(&head, f.plan.clone(), inventory, physical(&f))
        .await
        .unwrap();
    assert_eq!(proof, again);
    assert_eq!(fs::read(private(&f).join("progress")).unwrap(), state);
    assert_eq!(
        before,
        [
            f.folder.root().join("file.txt"),
            f.folder.root().join("src/new"),
            private(&f).join("b-0")
        ]
        .map(|p| fs::metadata(p).unwrap().ino())
    );
    drop(recovery);
}
#[tokio::test]
async fn binary_executable_additions_deletions_and_nested_parents_preserve_other_files() {
    use std::os::unix::fs::PermissionsExt as _;
    let f = Input::build_effects(
        true,
        vec![
            ("file.txt", None),
            ("nested/deeper/binary", Some((b"\0\xff\x01\n", true))),
            ("nested/text", Some((b"plain\n", false))),
        ],
        |root| {
            fs::write(root.join("untracked"), b"editor draft\n").unwrap();
        },
    )
    .await;
    let inventory = f.inventory().await;
    let (recovery, head) = published(&f).await;
    let checkout = fs::read(f.folder.root().join(".jj/working_copy/checkout")).unwrap();
    let tree = fs::read(f.folder.root().join(".jj/working_copy/tree_state")).unwrap();
    let (recovery, _) = recovery
        .install_tree_files(&head, f.plan.clone(), inventory, physical(&f))
        .await
        .unwrap();
    assert!(!f.folder.root().join("file.txt").exists());
    assert_eq!(
        fs::read(f.folder.root().join("nested/deeper/binary")).unwrap(),
        b"\0\xff\x01\n"
    );
    assert_ne!(
        fs::metadata(f.folder.root().join("nested/deeper/binary"))
            .unwrap()
            .permissions()
            .mode()
            & 0o111,
        0
    );
    assert_eq!(
        fs::metadata(f.folder.root().join("nested/text"))
            .unwrap()
            .permissions()
            .mode()
            & 0o111,
        0
    );
    assert_eq!(
        fs::read(f.folder.root().join("untracked")).unwrap(),
        b"editor draft\n"
    );
    assert_eq!(
        fs::read(f.folder.root().join("ignored.txt")).unwrap(),
        b"ignored obstacle\n"
    );
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
#[tokio::test]
async fn editor_created_target_after_backup_is_never_replaced() {
    let f = Input::build(true).await;
    let inventory = f.inventory().await;
    let (recovery, head) = published(&f).await;
    let root = f.folder.root();
    let guard = Arc::new(Hook {
        inner: physical(&f),
        root: root.clone(),
        plan: f.plan.clone(),
        fired: AtomicBool::new(false),
        action: Box::new(|root, state| {
            if state["phases"][0] == 1 && !root.join("file.txt").exists() {
                fs::write(root.join("file.txt"), b"editor winner\n").unwrap();
            }
        }),
    });
    assert!(matches!(
        recovery
            .install_tree_files(&head, f.plan.clone(), inventory, guard)
            .await,
        Err(local_browser::Error::EpochChanged)
    ));
    assert_eq!(fs::read(root.join("file.txt")).unwrap(), b"editor winner\n");
    assert_eq!(fs::read(private(&f).join("b-0")).unwrap(), b"original\n");
    assert_eq!(fs::read(private(&f).join("f-0")).unwrap(), b"candidate\n");
}
#[tokio::test]
async fn changed_ignored_file_after_move_blocks_with_backups_and_native_state_retained() {
    let f = Input::build(true).await;
    let inventory = f.inventory().await;
    let (recovery, head) = published(&f).await;
    let root = f.folder.root();
    let tree = fs::read(root.join(".jj/working_copy/tree_state")).unwrap();
    let guard = Arc::new(Hook {
        inner: physical(&f),
        root: root.clone(),
        plan: f.plan.clone(),
        fired: AtomicBool::new(false),
        action: Box::new(|root, state| {
            if state["phases"][0] == 1 {
                fs::write(root.join("ignored.txt"), b"new ignored edit\n").unwrap();
            }
        }),
    });
    assert!(matches!(
        recovery
            .install_tree_files(&head, f.plan.clone(), inventory, guard)
            .await,
        Err(local_browser::Error::EpochChanged)
    ));
    assert_eq!(
        fs::read(root.join("ignored.txt")).unwrap(),
        b"new ignored edit\n"
    );
    assert_eq!(fs::read(private(&f).join("b-0")).unwrap(), b"original\n");
    assert_eq!(
        fs::read(root.join(".jj/working_copy/tree_state")).unwrap(),
        tree
    );
}
