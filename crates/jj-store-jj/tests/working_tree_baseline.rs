use jj_lib::{
    backend as b, config::StackedConfig, object_id::ObjectId as _, ref_name::WorkspaceNameBuf,
    repo::Repo as _, repo_path::RepoPath, settings::UserSettings, workspace::Workspace,
};
use jj_store_jj::local_browser::{self as local, working_tree::*};
use jj_store_model as m;
use std::{
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

struct Access {
    metadata: std::path::PathBuf,
    allowed: AtomicBool,
}
#[async_trait::async_trait]
impl local::Access for Access {
    fn repository(&self) -> m::RepositoryId {
        m::RepositoryId::new([1; 16])
    }
    fn generation(&self) -> u64 {
        7
    }
    fn metadata_path(&self) -> &Path {
        &self.metadata
    }
    async fn authorize(&self) -> local::Result<()> {
        if self.allowed.load(Ordering::SeqCst) {
            Ok(())
        } else {
            Err(local::Error::Denied)
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn baseline_reads_exact_workspace_at_selected_operation_without_publication() {
    let root = tempfile::tempdir().unwrap();
    let settings =
        UserSettings::from_config_and_home_dir(StackedConfig::with_defaults(), None).unwrap();
    let (workspace, repo) =
        Workspace::init_internal_git(&settings, root.path(), gix::hash::Kind::Sha1)
            .await
            .unwrap();
    let store = repo.store();
    let file = store
        .write_file(
            RepoPath::from_internal_string("binary").unwrap(),
            &mut futures::io::Cursor::new([0, 0xff, 4]),
        )
        .await
        .unwrap();
    let link = store
        .write_symlink(
            RepoPath::from_internal_string("link").unwrap(),
            "../outside",
        )
        .await
        .unwrap();
    let tree = b::Tree::from_sorted_entries(vec![
        (
            jj_lib::repo_path::RepoPathComponentBuf::new("binary").unwrap(),
            b::TreeValue::File {
                id: file,
                executable: true,
                copy_id: b::CopyId::placeholder(),
            },
        ),
        (
            jj_lib::repo_path::RepoPathComponentBuf::new("link").unwrap(),
            b::TreeValue::Symlink(link),
        ),
    ]);
    let tree = store.write_tree(RepoPath::root(), tree).await.unwrap();
    let mut tx = repo.start_transaction();
    let commit = tx
        .repo_mut()
        .new_commit(
            vec![store.root_commit_id().clone()],
            jj_lib::merged_tree::MergedTree::resolved(store.clone(), tree.id().clone()),
        )
        .write()
        .await
        .unwrap();
    tx.repo_mut()
        .set_wc_commit(WorkspaceNameBuf::from("default"), commit.id().clone())
        .unwrap();
    let repo = tx.commit("baseline fixture").await.unwrap();
    let operation = m::OperationId::from_slice(repo.operation().id().as_bytes()).unwrap();
    let mut later = repo.start_transaction();
    let other = later
        .repo_mut()
        .new_commit(vec![commit.id().clone()], store.root_commit().tree())
        .write()
        .await
        .unwrap();
    later
        .repo_mut()
        .set_wc_commit(WorkspaceNameBuf::from("default"), other.id().clone())
        .unwrap();
    let _later = later.commit("later fixture operation").await.unwrap();
    let access = Arc::new(Access {
        metadata: workspace.repo_path().to_owned(),
        allowed: AtomicBool::new(true),
    });
    let reader = local::Executor::new(2).unwrap();
    let before = reader.heads(access.clone()).await.unwrap().heads;
    let baseline = reader
        .working_tree_baseline(access.clone(), operation, "default".into())
        .await
        .unwrap();
    assert_eq!(baseline.commit.as_bytes(), commit.id().as_bytes());
    assert_eq!(baseline.operation, operation);
    assert_eq!(baseline.entries.len(), 2);
    assert_eq!(baseline.entries[0].kind, Kind::File { executable: true });
    assert_eq!(baseline.entries[0].content, content_digest(&[0, 0xff, 4]));
    assert_eq!(baseline.entries[1].kind, Kind::Symlink);
    assert_eq!(baseline.entries[1].content, content_digest(b"../outside"));
    assert_eq!(reader.heads(access.clone()).await.unwrap().heads, before);
    assert!(!root.path().join("binary").exists());
    let limited = local::Executor::with_workers(
        jj_store_jj::query::workers::Workers::new(1).unwrap(),
        local::Limits {
            max_work: 1,
            ..Default::default()
        },
    );
    assert!(matches!(
        limited
            .working_tree_baseline(access.clone(), operation, "default".into())
            .await,
        Err(local::Error::Limit)
    ));
    assert!(matches!(
        reader
            .working_tree_baseline(access.clone(), operation, "missing".into())
            .await,
        Err(local::Error::NotFound)
    ));
    assert!(matches!(
        reader
            .working_tree_baseline(
                access.clone(),
                m::OperationId::new([5; 64]),
                "default".into()
            )
            .await,
        Err(local::Error::NotCommitted)
    ));
    access.allowed.store(false, Ordering::SeqCst);
    assert!(matches!(
        reader
            .working_tree_baseline(access, operation, "default".into())
            .await,
        Err(local::Error::Denied)
    ));
}

#[tokio::test(flavor = "multi_thread")]
async fn baseline_limits_objects_and_rejects_submodules_without_partial_results() {
    let root = tempfile::tempdir().unwrap();
    let settings =
        UserSettings::from_config_and_home_dir(StackedConfig::with_defaults(), None).unwrap();
    let (workspace, repo) =
        Workspace::init_internal_git(&settings, root.path(), gix::hash::Kind::Sha1)
            .await
            .unwrap();
    let store = repo.store();
    let path = RepoPath::from_internal_string("oversized").unwrap();
    let file = store
        .write_file(
            path,
            &mut futures::io::Cursor::new(vec![5; MAX_FILE_BYTES + 1]),
        )
        .await
        .unwrap();
    let tree = store
        .write_tree(
            RepoPath::root(),
            b::Tree::from_sorted_entries(vec![(
                jj_lib::repo_path::RepoPathComponentBuf::new("oversized").unwrap(),
                b::TreeValue::File {
                    id: file,
                    executable: false,
                    copy_id: b::CopyId::placeholder(),
                },
            )]),
        )
        .await
        .unwrap();
    let mut tx = repo.start_transaction();
    let commit = tx
        .repo_mut()
        .new_commit(
            vec![store.root_commit_id().clone()],
            jj_lib::merged_tree::MergedTree::resolved(store.clone(), tree.id().clone()),
        )
        .write()
        .await
        .unwrap();
    tx.repo_mut()
        .set_wc_commit(WorkspaceNameBuf::from("default"), commit.id().clone())
        .unwrap();
    let repo = tx.commit("oversized fixture").await.unwrap();
    let access = Arc::new(Access {
        metadata: workspace.repo_path().to_owned(),
        allowed: AtomicBool::new(true),
    });
    let reader = local::Executor::new(1).unwrap();
    let operation = m::OperationId::from_slice(repo.operation().id().as_bytes()).unwrap();
    assert!(matches!(
        reader
            .working_tree_baseline(access.clone(), operation, "default".into())
            .await,
        Err(local::Error::Limit)
    ));
    let tree = store
        .write_tree(
            RepoPath::root(),
            b::Tree::from_sorted_entries(vec![(
                jj_lib::repo_path::RepoPathComponentBuf::new("submodule").unwrap(),
                b::TreeValue::GitSubmodule(commit.id().clone()),
            )]),
        )
        .await
        .unwrap();
    let mut tx = repo.start_transaction();
    let commit = tx
        .repo_mut()
        .new_commit(
            vec![commit.id().clone()],
            jj_lib::merged_tree::MergedTree::resolved(store.clone(), tree.id().clone()),
        )
        .write()
        .await
        .unwrap();
    tx.repo_mut()
        .set_wc_commit(WorkspaceNameBuf::from("default"), commit.id().clone())
        .unwrap();
    let repo = tx.commit("submodule fixture").await.unwrap();
    let operation = m::OperationId::from_slice(repo.operation().id().as_bytes()).unwrap();
    assert!(matches!(
        reader
            .working_tree_baseline(access.clone(), operation, "default".into())
            .await,
        Err(local::Error::Invalid)
    ));
    assert_eq!(reader.heads(access).await.unwrap().heads, vec![operation]);
}

#[test]
fn ignore_rules_use_upstream_precedence_and_do_not_reinclude_ignored_directories() {
    let rules = IgnoreRules::new(
        vec![b"*.tmp\nglobal\n".to_vec(), b"!global\ninfo\n".to_vec()],
        vec![
            IgnoreFile {
                path: ".gitignore".into(),
                bytes: b"build/\n!build/keep\n!info\n*.log\n".to_vec(),
            },
            IgnoreFile {
                path: "build/.gitignore".into(),
                bytes: b"!keep\n".to_vec(),
            },
            IgnoreFile {
                path: "src/.gitignore".into(),
                bytes: b"!keep.log\nlocal\n".to_vec(),
            },
        ],
    )
    .unwrap();
    for path in ["a.tmp", "build/keep", "src/a.log", "src/local"] {
        assert!(rules.ignored(path).unwrap(), "{path}");
    }
    for path in ["global", "info", "src/keep.log", "src/file", ".gitignore"] {
        assert!(!rules.ignored(path).unwrap(), "{path}");
    }
    assert!(
        IgnoreRules::new(
            vec![],
            vec![IgnoreFile {
                path: "../.gitignore".into(),
                bytes: vec![]
            }]
        )
        .is_err()
    );
    assert!(rules.ignored(".jj/secret").is_err());
}

#[test]
fn ignore_capture_limits_duplicate_sources_and_cancellation_fail_closed() {
    assert!(matches!(
        IgnoreRules::new(vec![vec![0; MAX_FILE_BYTES + 1]], vec![]),
        Err(local::Error::Limit)
    ));
    assert!(matches!(
        IgnoreRules::new(vec![vec![]; 3], vec![]),
        Err(local::Error::Limit)
    ));
    let file = IgnoreFile {
        path: ".gitignore".into(),
        bytes: b"ignored".to_vec(),
    };
    assert!(matches!(
        IgnoreRules::new(vec![], vec![file.clone(), file]),
        Err(local::Error::Invalid)
    ));
    assert!(matches!(
        IgnoreRules::with_guard(vec![], vec![], || Err(local::Error::Canceled)),
        Err(local::Error::Canceled)
    ));
}
