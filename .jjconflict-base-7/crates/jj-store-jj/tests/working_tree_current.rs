use jj_lib::{
    config::StackedConfig, object_id::ObjectId as _, settings::UserSettings, workspace::Workspace,
};
use jj_store_jj::local_browser::{self as code, working_tree::*};
use jj_store_model as m;
use prost::Message as _;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};
struct Access(PathBuf);
struct HookAccess {
    metadata: PathBuf,
    calls: AtomicUsize,
    hook: Box<dyn Fn(usize) -> code::Result<()> + Send + Sync>,
}
#[async_trait::async_trait]
impl code::Access for HookAccess {
    fn repository(&self) -> m::RepositoryId {
        m::RepositoryId::new([1; 16])
    }
    fn generation(&self) -> u64 {
        7
    }
    fn metadata_path(&self) -> &Path {
        &self.metadata
    }
    async fn authorize(&self) -> code::Result<()> {
        (self.hook)(self.calls.fetch_add(1, Ordering::SeqCst) + 1)
    }
}
#[async_trait::async_trait]
impl code::Access for Access {
    fn repository(&self) -> m::RepositoryId {
        m::RepositoryId::new([1; 16])
    }
    fn generation(&self) -> u64 {
        7
    }
    fn metadata_path(&self) -> &Path {
        &self.0
    }
    async fn authorize(&self) -> code::Result<()> {
        Ok(())
    }
}
async fn fixture() -> (tempfile::TempDir, PathBuf, Arc<Access>, m::OperationId) {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path().canonicalize().unwrap().join("workspace");
    fs::create_dir(&root).unwrap();
    let settings = UserSettings::from_config(StackedConfig::with_defaults()).unwrap();
    let (workspace, repo) = Workspace::init_internal_git(&settings, &root, gix::hash::Kind::Sha1)
        .await
        .unwrap();
    let metadata = workspace.repo_path().to_owned();
    let ignore = fixture.path().canonicalize().unwrap().join("global-ignore");
    fs::write(&ignore, "global\n").unwrap();
    let git_config = metadata.join("store/git/config");
    let config = fs::read_to_string(&git_config).unwrap();
    fs::write(
        git_config,
        format!("{config}\n[core]\nexcludesFile = {}\n", ignore.display()),
    )
    .unwrap();
    fs::create_dir_all(metadata.join("store/git/info")).unwrap();
    fs::write(metadata.join("store/git/info/exclude"), "repo-exclude\n").unwrap();
    let operation = m::OperationId::from_slice(repo.operation().id().as_bytes()).unwrap();
    (fixture, root, Arc::new(Access(metadata)), operation)
}
#[tokio::test(flavor = "multi_thread")]
async fn current_read_proves_checkout_tree_sparse_and_captured_base_policy_without_writes() {
    let (_fixture, root, access, operation) = fixture().await;
    let reader = code::Executor::new(1).unwrap();
    let before_checkout = fs::read(root.join(".jj/working_copy/checkout")).unwrap();
    let before_tree = fs::read(root.join(".jj/working_copy/tree_state")).unwrap();
    let value = reader
        .working_tree_current(access.clone(), root.clone(), operation)
        .await
        .unwrap();
    assert_eq!(value.baseline.operation, operation);
    assert_eq!(value.checkout.trees, value.baseline.trees);
    assert_eq!(value.checkout.sparse, vec![String::new()]);
    let rules = IgnoreRules::new(value.base_ignores.clone(), vec![]).unwrap();
    assert!(rules.ignored("global").unwrap());
    assert!(rules.ignored("repo-exclude").unwrap());
    assert_eq!(
        reader
            .working_tree_current(access, root.clone(), operation)
            .await
            .unwrap(),
        value
    );
    assert_eq!(
        fs::read(root.join(".jj/working_copy/checkout")).unwrap(),
        before_checkout
    );
    assert_eq!(
        fs::read(root.join(".jj/working_copy/tree_state")).unwrap(),
        before_tree
    );
}
#[tokio::test(flavor = "multi_thread")]
async fn current_read_rejects_stale_missing_unsafe_and_unbounded_checkout_state() {
    let (_fixture, root, access, operation) = fixture().await;
    let reader = code::Executor::new(1).unwrap();
    assert!(matches!(
        reader
            .working_tree_current(access.clone(), root.clone(), m::OperationId::new([8; 64]))
            .await,
        Err(code::Error::EpochChanged)
    ));
    let tree_path = root.join(".jj/working_copy/tree_state");
    let bytes = fs::read(&tree_path).unwrap();
    fs::remove_file(&tree_path).unwrap();
    assert!(
        reader
            .working_tree_current(access.clone(), root.clone(), operation)
            .await
            .is_err()
    );
    assert!(
        !tree_path.exists(),
        "a read must not initialize missing state"
    );
    fs::write(&tree_path, vec![0; MAX_CHECKOUT_BYTES + 1]).unwrap();
    assert!(matches!(
        reader
            .working_tree_current(access.clone(), root.clone(), operation)
            .await,
        Err(code::Error::Limit)
    ));
    fs::remove_file(&tree_path).unwrap();
    std::os::unix::fs::symlink("/outside", &tree_path).unwrap();
    assert!(
        reader
            .working_tree_current(access.clone(), root.clone(), operation)
            .await
            .is_err()
    );
    fs::remove_file(&tree_path).unwrap();
    fs::write(&tree_path, &bytes).unwrap();
    let mut tree = jj_lib::protos::local_working_copy::TreeState::decode(bytes.as_slice()).unwrap();
    tree.tree_ids = vec![vec![7; 20]];
    fs::write(&tree_path, tree.encode_to_vec()).unwrap();
    assert!(matches!(
        reader.working_tree_current(access, root, operation).await,
        Err(code::Error::EpochChanged)
    ));
}
#[tokio::test(flavor = "multi_thread")]
async fn current_read_preserves_sparse_scope_and_detects_ignore_policy_edits() {
    let (_fixture, root, access, operation) = fixture().await;
    let tree_path = root.join(".jj/working_copy/tree_state");
    let mut tree = jj_lib::protos::local_working_copy::TreeState::decode(
        fs::read(&tree_path).unwrap().as_slice(),
    )
    .unwrap();
    tree.sparse_patterns = Some(jj_lib::protos::local_working_copy::SparsePatterns {
        prefixes: vec!["src".into()],
    });
    fs::write(&tree_path, tree.encode_to_vec()).unwrap();
    let reader = code::Executor::new(1).unwrap();
    let first = reader
        .working_tree_current(access.clone(), root.clone(), operation)
        .await
        .unwrap();
    let sparse = SparseRules::new(first.checkout.sparse.clone()).unwrap();
    assert!(sparse.matches("src/file").unwrap());
    assert!(!sparse.matches("other/file").unwrap());
    fs::write(access.0.join("store/git/info/exclude"), "new-policy\n").unwrap();
    let later = reader
        .working_tree_current(access, root, operation)
        .await
        .unwrap();
    assert_ne!(first.fingerprint, later.fingerprint);
}

#[tokio::test(flavor = "multi_thread")]
async fn current_read_rejects_metadata_and_policy_races_and_revoked_access() {
    for change in ["policy", "checkout", "denied"] {
        let (_fixture, root, access, operation) = fixture().await;
        let metadata = access.0.clone();
        let checkout = root.join(".jj/working_copy/checkout");
        let hook = Arc::new(HookAccess {
            metadata: metadata.clone(),
            calls: AtomicUsize::new(0),
            hook: Box::new(move |n| {
                // Initial admission and pre-head authorization are followed by
                // the baseline read, after the first metadata/policy capture.
                if n == 3 {
                    match change {
                        "policy" => {
                            fs::write(metadata.join("store/git/info/exclude"), "changed\n").unwrap()
                        }
                        "checkout" => {
                            let bytes = fs::read(&checkout).unwrap();
                            let mut value = jj_lib::protos::local_working_copy::Checkout::decode(
                                bytes.as_slice(),
                            )
                            .unwrap();
                            value.workspace_name = "replaced".into();
                            fs::write(&checkout, value.encode_to_vec()).unwrap();
                        }
                        _ => return Err(code::Error::Denied),
                    }
                }
                Ok(())
            }),
        });
        let value = code::Executor::new(1)
            .unwrap()
            .working_tree_current(hook, root, operation)
            .await;
        assert!(
            matches!(value, Err(code::Error::EpochChanged)) && change != "denied"
                || matches!(value, Err(code::Error::Denied)) && change == "denied",
            "{change}: {value:?}"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn current_read_bounds_repeated_state_and_rejects_conflict_label_mismatch() {
    let (_fixture, root, access, operation) = fixture().await;
    let tree_path = root.join(".jj/working_copy/tree_state");
    let bytes = fs::read(&tree_path).unwrap();
    let tree = jj_lib::protos::local_working_copy::TreeState::decode(bytes.as_slice()).unwrap();
    let mut excessive = tree.clone();
    excessive.sparse_patterns = Some(jj_lib::protos::local_working_copy::SparsePatterns {
        prefixes: vec!["src".into(); 513],
    });
    fs::write(&tree_path, excessive.encode_to_vec()).unwrap();
    let reader = code::Executor::new(1).unwrap();
    assert!(matches!(
        reader
            .working_tree_current(access.clone(), root.clone(), operation)
            .await,
        Err(code::Error::Limit)
    ));
    let mut mislabeled = tree;
    mislabeled.conflict_labels = vec!["different".into()];
    fs::write(&tree_path, mislabeled.encode_to_vec()).unwrap();
    assert!(matches!(
        reader.working_tree_current(access, root, operation).await,
        Err(code::Error::EpochChanged)
    ));
}
