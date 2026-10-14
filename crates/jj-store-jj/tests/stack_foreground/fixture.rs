use jj_store_jj::local_browser::{self, working_tree::CurrentState};
use jj_store_model as m;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::Arc,
};

pub struct Fixture {
    _temp: tempfile::TempDir,
    root: PathBuf,
    operation: m::OperationId,
}
impl Fixture {
    pub async fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().canonicalize().unwrap();
        stock(&root, &["git", "init", "--colocate"]);
        fs::write(root.join("file.txt"), "original\n").unwrap();
        fs::write(root.join(".gitignore"), "ignored.txt\n").unwrap();
        stock(&root, &["describe", "-m", "Fixture"]);
        let operation = stock(&root, &["op", "log", "--no-graph", "-n", "1", "-T", "id"])
            .parse()
            .unwrap();
        fs::write(root.join("file.txt"), "dirty external edit\n").unwrap();
        fs::write(root.join("ignored.txt"), "ignored obstacle\n").unwrap();
        Self {
            _temp: temp,
            root,
            operation,
        }
    }
    pub fn root(&self) -> PathBuf {
        self.root.clone()
    }
    pub fn access(&self) -> Arc<dyn local_browser::Access> {
        Arc::new(Access(self.root.join(".jj/repo")))
    }
    pub async fn witness(&self) -> CurrentState {
        local_browser::Executor::new(1)
            .unwrap()
            .working_tree_current(self.access(), self.root(), self.operation)
            .await
            .unwrap()
    }
    pub fn state(&self) -> Vec<Vec<u8>> {
        let mut state: Vec<_> = [
            "file.txt",
            "ignored.txt",
            ".jj/working_copy/checkout",
            ".jj/working_copy/tree_state",
        ]
        .into_iter()
        .map(|p| fs::read(self.root.join(p)).unwrap())
        .collect();
        let mut heads: Vec<_> = fs::read_dir(self.root.join(".jj/repo/op_heads/heads"))
            .unwrap()
            .map(|e| e.unwrap().file_name().as_encoded_bytes().to_vec())
            .collect();
        heads.sort();
        state.extend(heads);
        state
    }
}
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
pub fn stock(root: &Path, args: &[&str]) -> String {
    let output = Command::new("jj")
        .current_dir(root)
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
