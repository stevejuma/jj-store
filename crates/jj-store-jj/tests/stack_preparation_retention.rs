//! Retention of real unpublished JJ code, independent of command execution.
use jj_lib::{
    config::StackedConfig,
    default_backend_factories::default_backend_factories,
    object_id::ObjectId as _,
    repo::{Repo as _, RepoLoader},
    settings::UserSettings,
};
use jj_store_jj::{
    local_evidence,
    local_metadata::{Limits, MetadataStore, StackPreparationKey, StackRetentionOutcome},
};
use jj_store_model as m;
use std::{fs, path::Path, process::Command};

const REPOSITORY: m::RepositoryId = m::RepositoryId::new([71; 16]);
fn settings() -> UserSettings {
    UserSettings::from_config(StackedConfig::with_defaults()).unwrap()
}
fn stock(path: &Path, args: &[&str]) -> String {
    let output = Command::new("jj")
        .current_dir(path)
        .args([
            "--no-pager",
            "--config",
            "user.name='Fixture'",
            "--config",
            "user.email='fixture@invalid'",
        ])
        .args(args)
        .env("JJ_CONFIG", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}
fn key() -> StackPreparationKey {
    StackPreparationKey {
        repository: REPOSITORY,
        physical_workspace: m::RecordDigest::new([72; 32]),
        principal: m::PrincipalId::new([73; 16]),
        request: m::RequestId::new([74; 16]),
        intent: m::IntentDigest::new([75; 32]),
        prepared: m::RecordDigest::new([76; 32]),
    }
}
fn heads(path: &Path) -> std::collections::BTreeSet<std::ffi::OsString> {
    fs::read_dir(path.join("op_heads/heads"))
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect()
}
async fn fixture() -> (
    tempfile::TempDir,
    MetadataStore,
    local_evidence::GitEvidenceCapsuleV1,
    std::collections::BTreeSet<std::ffi::OsString>,
) {
    let root = tempfile::tempdir().unwrap();
    stock(root.path(), &["git", "init", "--colocate"]);
    fs::write(root.path().join("file.txt"), "one\n").unwrap();
    stock(root.path(), &["describe", "-m", "Before"]);
    stock(root.path(), &["bookmark", "create", "main", "-r", "@"]);
    let id: m::OperationId = stock(
        root.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    let repo_path = root.path().join(".jj/repo");
    let before = heads(&repo_path);
    let loader =
        RepoLoader::init_from_file_system(&settings(), &repo_path, &default_backend_factories())
            .unwrap();
    let operation = loader
        .load_operation(&jj_lib::op_store::OperationId::from_bytes(id.as_bytes()))
        .await
        .unwrap();
    let repo = loader.load_at(&operation).await.unwrap();
    let old = repo
        .store()
        .get_commit(
            repo.view()
                .get_wc_commit_id(jj_lib::ref_name::WorkspaceName::DEFAULT)
                .unwrap(),
        )
        .unwrap();
    let mut tx = repo.start_transaction();
    tx.repo_mut()
        .rewrite_commit(&old)
        .set_description("Exact unpublished reword")
        .write()
        .await
        .unwrap();
    tx.repo_mut().rebase_descendants().await.unwrap();
    let prepared = tx
        .write("Stack preparation fixture")
        .await
        .unwrap()
        .leave_unpublished();
    let prepared_id = m::OperationId::from_slice(prepared.operation().id().as_bytes()).unwrap();
    let archive = local_evidence::inspect(&settings(), &repo_path, REPOSITORY, prepared_id)
        .await
        .unwrap();
    fs::write(root.path().join("unsnapshotted.txt"), "must stay dirty\n").unwrap();
    let store = MetadataStore::open(&root.path().join(".git"), Limits::default()).unwrap();
    drop(prepared);
    drop(repo);
    drop(loader);
    (root, store, archive, before)
}
#[tokio::test]
async fn retained_unpublished_stack_operation_survives_gc_and_source_state_loss() {
    let (root, store, archive, before) = fixture().await;
    let repo_path = root.path().join(".jj/repo");
    assert!(store.read_stack_prepared(&key()).unwrap().is_none());
    let StackRetentionOutcome::Retained(selected) =
        store.retain_stack_prepared(&key(), &archive).unwrap()
    else {
        panic!("retention must be acknowledged")
    };
    assert_eq!(selected.capsule, archive);
    assert_eq!(heads(&repo_path), before);
    assert!(
        store.read().unwrap().is_none(),
        "must not initialize companion metadata"
    );
    fs::remove_dir_all(repo_path.join("op_store")).unwrap();
    fs::remove_dir_all(repo_path.join("index")).unwrap();
    let output = Command::new("git")
        .arg("--git-dir")
        .arg(root.path().join(".git"))
        .args(["gc", "--prune=now"])
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let reopened = MetadataStore::open(&root.path().join(".git"), Limits::default()).unwrap();
    assert_eq!(
        reopened.read_stack_prepared(&key()).unwrap().unwrap(),
        *selected
    );
    let StackRetentionOutcome::Retained(replay) =
        reopened.retain_stack_prepared(&key(), &archive).unwrap()
    else {
        panic!("packed-ref replay must sync")
    };
    assert_eq!(replay, selected);
    assert_eq!(fs::read(root.path().join("file.txt")).unwrap(), b"one\n");
    assert_eq!(
        fs::read(root.path().join("unsnapshotted.txt")).unwrap(),
        b"must stay dirty\n"
    );
}

fn reference(key: &StackPreparationKey) -> String {
    format!(
        "{}{}/{}",
        jj_store_jj::local_metadata::STACK_PREPARED_REF_PREFIX,
        key.physical_workspace,
        key.request
    )
}

#[tokio::test]
async fn lookup_never_initializes_and_retry_keeps_original_slot_and_user_branches() {
    let (root, store, archive, before) = fixture().await;
    let repo_path = root.path().join(".jj/repo");
    let checkout = fs::read(root.path().join(".jj/working_copy/checkout")).unwrap();
    let tree_state = fs::read(root.path().join(".jj/working_copy/tree_state")).unwrap();
    assert!(store.read_stack_prepared(&key()).unwrap().is_none());
    assert!(!store.git_dir().join("refs/lapis").exists());
    let branches = stock(root.path(), &["--ignore-working-copy", "bookmark", "list"]);
    let StackRetentionOutcome::Retained(original) =
        store.retain_stack_prepared(&key(), &archive).unwrap()
    else {
        panic!()
    };
    let bytes = fs::read(store.git_dir().join(reference(&key()))).unwrap();
    let StackRetentionOutcome::Retained(replay) =
        store.retain_stack_prepared(&key(), &archive).unwrap()
    else {
        panic!()
    };
    assert_eq!(replay, original);
    assert_eq!(
        fs::read(store.git_dir().join(reference(&key()))).unwrap(),
        bytes
    );
    assert_eq!(
        stock(root.path(), &["--ignore-working-copy", "bookmark", "list"]),
        branches
    );
    assert_eq!(heads(&repo_path), before);
    assert_eq!(
        fs::read(root.path().join(".jj/working_copy/checkout")).unwrap(),
        checkout
    );
    assert_eq!(
        fs::read(root.path().join(".jj/working_copy/tree_state")).unwrap(),
        tree_state
    );
    assert!(store.read().unwrap().is_none());
}

#[tokio::test]
async fn original_principal_intent_preparation_and_repository_bind_immutable_request_slot() {
    use jj_store_jj::local_metadata::Error;
    let (_root, store, archive, _) = fixture().await;
    store.retain_stack_prepared(&key(), &archive).unwrap();
    let bytes = fs::read(store.git_dir().join(reference(&key()))).unwrap();
    for index in 0..4 {
        let mut changed = key();
        match index {
            0 => changed.principal = m::PrincipalId::new([22; 16]),
            1 => changed.intent = m::IntentDigest::new([22; 32]),
            2 => changed.prepared = m::RecordDigest::new([22; 32]),
            _ => changed.repository = m::RepositoryId::new([22; 16]),
        }
        assert!(matches!(
            store.read_stack_prepared(&changed),
            Err(Error::IntentConflict)
        ));
        assert!(store.retain_stack_prepared(&changed, &archive).is_err());
        assert_eq!(
            fs::read(store.git_dir().join(reference(&key()))).unwrap(),
            bytes
        );
    }
    let mut changed = key();
    changed.physical_workspace = m::RecordDigest::new([23; 32]);
    assert!(store.read_stack_prepared(&changed).unwrap().is_none());
    let mut invalid = key();
    invalid.request = m::RequestId::new([0; 16]);
    assert!(store.read_stack_prepared(&invalid).is_err());
    assert!(store.retain_stack_prepared(&invalid, &archive).is_err());
}

#[tokio::test]
async fn unknown_ref_selection_retries_the_exact_retention_candidate() {
    let (_root, store, archive, _) = fixture().await;
    let path = store.git_dir().join(reference(&key()));
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    let lock = path.with_extension("lock");
    fs::write(&lock, b"held competing ref lock").unwrap();
    let StackRetentionOutcome::Unknown { commit } =
        store.retain_stack_prepared(&key(), &archive).unwrap()
    else {
        panic!("held ref lock must not select")
    };
    assert!(store.read_stack_prepared(&key()).unwrap().is_none());
    fs::remove_file(lock).unwrap();
    let StackRetentionOutcome::Retained(replay) =
        store.retain_stack_prepared(&key(), &archive).unwrap()
    else {
        panic!()
    };
    assert_eq!(replay.commit, commit);
    assert_eq!(replay.capsule, archive);
    assert!(store.read().unwrap().is_none());
}

#[tokio::test]
async fn reserved_ref_collisions_symbolic_refs_and_incomplete_closures_fail_closed() {
    let (_root, store, archive, _) = fixture().await;
    let path = store.git_dir().join(reference(&key()));
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, "ref: refs/heads/main\n").unwrap();
    assert!(store.read_stack_prepared(&key()).is_err());
    assert!(store.retain_stack_prepared(&key(), &archive).is_err());
    assert_eq!(fs::read_to_string(&path).unwrap(), "ref: refs/heads/main\n");
    let unrelated = store.stage_blob(b"unexpected reserved object").unwrap();
    fs::write(&path, format!("{unrelated}\n")).unwrap();
    assert!(store.read_stack_prepared(&key()).is_err());
    assert!(store.retain_stack_prepared(&key(), &archive).is_err());
    assert_eq!(fs::read_to_string(&path).unwrap(), format!("{unrelated}\n"));
    fs::remove_file(&path).unwrap();
    let tree = archive
        .commits
        .values()
        .flat_map(|c| &c.root_tree)
        .find(|id| {
            if **id == m::GitSha1Id::new([0; 20]) {
                return false;
            }
            let git = gix::open(store.git_dir()).unwrap();
            let oid = gix::ObjectId::from_hex(id.to_string().as_bytes()).unwrap();
            !git.find_object(oid).unwrap().data.is_empty()
        })
        .unwrap()
        .to_string();
    fs::remove_file(
        store
            .git_dir()
            .join("objects")
            .join(&tree[..2])
            .join(&tree[2..]),
    )
    .unwrap();
    assert!(store.retain_stack_prepared(&key(), &archive).is_err());
    assert!(!path.exists());
    assert!(store.read().unwrap().is_none());
}

#[tokio::test]
async fn strict_object_and_closure_limits_select_nothing() {
    use jj_store_jj::local_metadata::Error;
    let (_root, store, archive, _) = fixture().await;
    for limits in [
        Limits {
            object_bytes: 4,
            ..Limits::default()
        },
        Limits {
            root_bytes: 4,
            ..Limits::default()
        },
        Limits {
            closure_bytes: 4,
            ..Limits::default()
        },
        Limits {
            objects: 1,
            ..Limits::default()
        },
    ] {
        let tiny = MetadataStore::open(store.git_dir(), limits).unwrap();
        assert!(matches!(
            tiny.retain_stack_prepared(&key(), &archive),
            Err(Error::Limit)
        ));
        assert!(store.read_stack_prepared(&key()).unwrap().is_none());
    }
}
