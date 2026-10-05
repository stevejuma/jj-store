//! Cold physical/JJ comparison fixture; no registered HTTP or executor qualification.
use jj_store_jj::local_browser as code;
use jj_store_model as m;
use jj_store_service::working_tree::{self as tree, ChangeKind};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::Arc,
};

struct Access(PathBuf);
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
fn files(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    fn visit(root: &Path, path: &Path, out: &mut BTreeMap<PathBuf, Vec<u8>>) {
        for entry in fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            let metadata = fs::symlink_metadata(&path).unwrap();
            if metadata.is_dir() {
                visit(root, &path, out);
            } else if metadata.is_symlink() {
                out.insert(
                    path.strip_prefix(root).unwrap().into(),
                    fs::read_link(&path)
                        .unwrap()
                        .as_os_str()
                        .as_encoded_bytes()
                        .into(),
                );
            } else {
                out.insert(
                    path.strip_prefix(root).unwrap().into(),
                    fs::read(path).unwrap(),
                );
            }
        }
    }
    let mut out = BTreeMap::new();
    visit(root, root, &mut out);
    out
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires explicit existing pinned development JJ binary"]
async fn real_folder_comparison_preserves_operations_metadata_and_unsnapshotted_files() {
    let binary = PathBuf::from(
        std::env::var_os("JJ_STORE_WORKING_TREE_TEST_JJ").expect("explicit development executable"),
    );
    assert!(binary.is_absolute());
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path().join("repository");
    let config = fixture.path().join("fixture.toml");
    fs::write(
        &config,
        "[user]\nname = 'Working tree fixture'\nemail = 'working-tree@example.test'\n",
    )
    .unwrap();
    let command = |args: &[&str]| {
        let output = Command::new(&binary)
            .args(args)
            .env("JJ_CONFIG", &config)
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .current_dir(fixture.path())
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    };
    command(&["git", "init", "--colocate", root.to_str().unwrap()]);
    // Controlled fixture base policy: no global rules and verified empty repo exclude.
    fs::create_dir_all(root.join(".git/info")).unwrap();
    fs::write(root.join(".git/info/exclude"), b"").unwrap();
    fs::write(root.join("tracked.tmp"), b"before\n").unwrap();
    fs::write(root.join("deleted"), [0, 0xff, 4]).unwrap();
    command(&[
        "-R",
        root.to_str().unwrap(),
        "describe",
        "-m",
        "Exact baseline fixture",
    ]);
    // Ignore rules are added after tracking: tracked.tmp must still be reported.
    fs::write(root.join(".gitignore"), "*.tmp\n").unwrap();
    fs::write(root.join("tracked.tmp"), b"after\n").unwrap();
    fs::remove_file(root.join("deleted")).unwrap();
    fs::write(root.join("ignored.tmp"), "checkout obstacle").unwrap();
    fs::write(root.join("new"), "candidate addition").unwrap();
    let before = files(&root);
    let reader = code::Executor::new(1).unwrap();
    let access = Arc::new(Access(root.join(".jj/repo")));
    let heads = reader.heads(access.clone()).await.unwrap().heads;
    assert_eq!(heads.len(), 1);
    let baseline = reader
        .working_tree_baseline(access.clone(), heads[0], "default".into())
        .await
        .unwrap();
    let physical = tree::scan(
        &root,
        m::RepositoryId::new([1; 16]),
        7,
        tree::Limits::default(),
        |_| Ok(()),
    )
    .unwrap();
    let projection = tree::project(physical, baseline, vec![], || Ok(())).unwrap();
    assert_eq!(
        projection
            .changes
            .iter()
            .map(|c| (c.path.as_str(), c.kind))
            .collect::<Vec<_>>(),
        [
            (".gitignore", ChangeKind::Added),
            ("deleted", ChangeKind::Deleted),
            ("new", ChangeKind::Added),
            ("tracked.tmp", ChangeKind::Modified),
        ]
    );
    assert!(
        projection
            .obstacles
            .iter()
            .any(|o| o.entry.path == "ignored.tmp" && o.ignored)
    );
    assert_eq!(reader.heads(access).await.unwrap().heads, heads);
    assert_eq!(
        files(&root),
        before,
        "code/metadata/physical files must remain byte-identical"
    );
}
