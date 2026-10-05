//! Explicit disposable CLI fixtures, never installed-service or HTTP acceptance.
use jj_store_jj::local_browser as code;
use jj_store_model as m;
use jj_store_service::working_tree::{self as tree, ChangeKind, Error};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

struct ReleaseGate(Arc<(std::sync::Mutex<bool>, std::sync::Condvar)>);
impl Drop for ReleaseGate {
    fn drop(&mut self) {
        let (lock, wake) = &*self.0;
        *lock.lock().unwrap() = true;
        wake.notify_all();
    }
}

struct Access {
    metadata: PathBuf,
    calls: AtomicUsize,
    hook: Box<dyn Fn(usize) -> code::Result<()> + Send + Sync>,
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
        &self.metadata
    }
    async fn authorize(&self) -> code::Result<()> {
        (self.hook)(self.calls.fetch_add(1, Ordering::SeqCst) + 1)
    }
}
struct Fixture {
    _temp: tempfile::TempDir,
    root: PathBuf,
    binary: PathBuf,
    config: PathBuf,
}
impl Fixture {
    fn new() -> Self {
        let binary = PathBuf::from(
            std::env::var_os("JJ_STORE_WORKING_TREE_TEST_JJ").expect("explicit pinned executable"),
        );
        assert!(binary.is_absolute());
        let temp = tempfile::tempdir().unwrap();
        let parent = temp.path().canonicalize().unwrap();
        let root = parent.join("repository");
        let config = parent.join("fixture.toml");
        fs::write(
            &config,
            "[user]\nname = 'Observation fixture'\nemail = 'observation@example.test'\n",
        )
        .unwrap();
        let fixture = Self {
            _temp: temp,
            root,
            binary,
            config,
        };
        fixture.run(&["git", "init", "--colocate", fixture.root.to_str().unwrap()]);
        fs::write(fixture.root.join("tracked.tmp"), "before\n").unwrap();
        fs::write(fixture.root.join("excluded"), "outside sparse\n").unwrap();
        fs::create_dir(fixture.root.join("src")).unwrap();
        fs::write(fixture.root.join("src/tracked"), "before\n").unwrap();
        fixture.run(&[
            "-R",
            fixture.root.to_str().unwrap(),
            "describe",
            "-m",
            "Observation baseline",
        ]);
        let global = parent.join("global-ignore");
        fs::write(&global, "global-ignore\n").unwrap();
        let path = fixture.root.join(".git/config");
        let bytes = fs::read_to_string(&path).unwrap();
        fs::write(
            path,
            format!("{bytes}\n[core]\nexcludesFile = {}\n", global.display()),
        )
        .unwrap();
        fs::create_dir_all(fixture.root.join(".git/info")).unwrap();
        fs::write(fixture.root.join(".git/info/exclude"), "repo-ignore\n").unwrap();
        fixture
    }
    fn run(&self, args: &[&str]) {
        let output = Command::new(&self.binary)
            .args(args)
            .env("JJ_CONFIG", &self.config)
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .current_dir(self._temp.path())
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    fn access(
        &self,
        hook: impl Fn(usize) -> code::Result<()> + Send + Sync + 'static,
    ) -> Arc<Access> {
        Arc::new(Access {
            metadata: self.root.join(".jj/repo"),
            calls: AtomicUsize::new(0),
            hook: Box::new(hook),
        })
    }
    async fn operation(&self, reader: &code::Executor) -> m::OperationId {
        let heads = reader.heads(self.access(|_| Ok(()))).await.unwrap().heads;
        assert_eq!(heads.len(), 1);
        heads[0]
    }
    async fn pre_scan_calls(&self, reader: &code::Executor, operation: m::OperationId) -> usize {
        let access = self.access(|_| Ok(()));
        reader
            .working_tree_current(access.clone(), self.root.clone(), operation)
            .await
            .unwrap();
        access.calls.load(Ordering::SeqCst)
    }
}
fn files(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    fn visit(root: &Path, path: &Path, out: &mut BTreeMap<PathBuf, Vec<u8>>) {
        for entry in fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            let metadata = fs::symlink_metadata(&path).unwrap();
            if metadata.is_dir() {
                visit(root, &path, out);
            } else {
                let bytes = if metadata.is_symlink() {
                    fs::read_link(&path)
                        .unwrap()
                        .as_os_str()
                        .as_encoded_bytes()
                        .to_vec()
                } else {
                    fs::read(&path).unwrap()
                };
                out.insert(path.strip_prefix(root).unwrap().into(), bytes);
            }
        }
    }
    let mut out = BTreeMap::new();
    visit(root, root, &mut out);
    out
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires explicit existing pinned development JJ binary"]
async fn observation_reads_current_dirty_files_and_complete_ignore_policy_without_writes() {
    let fixture = Fixture::new();
    fs::write(fixture.root.join(".gitignore"), "*.tmp\n").unwrap();
    fs::write(fixture.root.join("tracked.tmp"), "after\n").unwrap();
    fs::write(fixture.root.join("global-ignore"), "obstacle").unwrap();
    fs::write(fixture.root.join("repo-ignore"), "obstacle").unwrap();
    fs::write(fixture.root.join("new"), [0, 0xff]).unwrap();
    let reader = code::Executor::new(1).unwrap();
    let operation = fixture.operation(&reader).await;
    let before = files(&fixture.root);
    let observer = tree::Observer::new(reader.clone(), 1).unwrap();
    let value = observer
        .observe(fixture.access(|_| Ok(())), fixture.root.clone(), operation)
        .await
        .unwrap();
    assert_eq!(
        value
            .projection
            .changes
            .iter()
            .map(|c| (c.path.as_str(), c.kind))
            .collect::<Vec<_>>(),
        [
            (".gitignore", ChangeKind::Added),
            ("new", ChangeKind::Added),
            ("tracked.tmp", ChangeKind::Modified),
        ]
    );
    assert!(
        value
            .projection
            .obstacles
            .iter()
            .any(|o| o.entry.path == "global-ignore" && o.ignored)
    );
    assert!(
        value
            .projection
            .obstacles
            .iter()
            .any(|o| o.entry.path == "repo-ignore" && o.ignored)
    );
    assert_eq!(
        value.current.physical_workspace,
        value.projection.physical.workspace.0
    );
    assert_eq!(files(&fixture.root), before);
    assert_eq!(fixture.operation(&reader).await, operation);
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires explicit existing pinned development JJ binary"]
async fn observation_respects_real_sparse_checkout_and_outside_obstacles() {
    let fixture = Fixture::new();
    fixture.run(&[
        "-R",
        fixture.root.to_str().unwrap(),
        "sparse",
        "set",
        "--clear",
        "--add",
        "src",
    ]);
    assert!(!fixture.root.join("excluded").exists());
    fs::write(fixture.root.join("outside"), "checkout obstacle").unwrap();
    fs::write(fixture.root.join("src/tracked"), "after\n").unwrap();
    let reader = code::Executor::new(1).unwrap();
    let operation = fixture.operation(&reader).await;
    let before = files(&fixture.root);
    let value = tree::Observer::new(reader, 1)
        .unwrap()
        .observe(fixture.access(|_| Ok(())), fixture.root.clone(), operation)
        .await
        .unwrap();
    assert_eq!(value.current.checkout.sparse, vec!["src"]);
    assert_eq!(
        value
            .projection
            .changes
            .iter()
            .map(|c| c.path.as_str())
            .collect::<Vec<_>>(),
        ["src/tracked"]
    );
    assert!(
        value
            .projection
            .obstacles
            .iter()
            .any(|o| o.entry.path == "outside" && o.outside_sparse)
    );
    assert_eq!(files(&fixture.root), before);
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires explicit existing pinned development JJ binary"]
async fn observation_rejects_changed_policy_metadata_and_revoked_access_during_scan() {
    for change in ["policy", "metadata", "denied"] {
        let fixture = Fixture::new();
        let reader = code::Executor::new(1).unwrap();
        let operation = fixture.operation(&reader).await;
        let at = fixture.pre_scan_calls(&reader, operation).await + 1;
        let root = fixture.root.clone();
        let access = fixture.access(move |n| {
            if n == at {
                match change {
                    "policy" => {
                        fs::write(root.join(".git/info/exclude"), "replacement-policy\n").unwrap()
                    }
                    "metadata" => {
                        let path = root.join(".jj/working_copy/type");
                        fs::remove_file(&path).unwrap();
                        fs::write(path, "local").unwrap();
                    }
                    _ => return Err(code::Error::Denied),
                }
            }
            Ok(())
        });
        let value = tree::Observer::new(reader, 1)
            .unwrap()
            .observe(access, fixture.root.clone(), operation)
            .await;
        assert_eq!(
            value.unwrap_err(),
            if change == "denied" {
                Error::Denied
            } else {
                Error::Stale
            }
        );
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "requires explicit existing pinned development JJ binary"]
async fn observation_cancellation_retains_admission_until_scan_worker_exits() {
    let fixture = Fixture::new();
    let reader = code::Executor::new(1).unwrap();
    let operation = fixture.operation(&reader).await;
    let at = fixture.pre_scan_calls(&reader, operation).await + 1;
    let entered = Arc::new(tokio::sync::Notify::new());
    let gate = Arc::new((std::sync::Mutex::new(false), std::sync::Condvar::new()));
    let _release = ReleaseGate(gate.clone());
    let worker_gate = gate.clone();
    let worker_entered = entered.clone();
    let access = fixture.access(move |n| {
        if n == at {
            worker_entered.notify_one();
            let (lock, wake) = &*worker_gate;
            let mut released = lock.lock().unwrap();
            while !*released {
                released = wake.wait(released).unwrap();
            }
        }
        Ok(())
    });
    let observer = tree::Observer::new(reader.clone(), 1).unwrap();
    let owner = observer.clone();
    let root = fixture.root.clone();
    let task = tokio::spawn(async move { owner.observe(access, root, operation).await });
    tokio::time::timeout(std::time::Duration::from_secs(5), entered.notified())
        .await
        .unwrap();
    task.abort();
    assert!(task.await.unwrap_err().is_cancelled());
    let busy = observer
        .observe(fixture.access(|_| Ok(())), fixture.root.clone(), operation)
        .await;
    let (lock, wake) = &*gate;
    *lock.lock().unwrap() = true;
    wake.notify_all();
    assert_eq!(busy.unwrap_err(), Error::Busy);
    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(5);
    loop {
        match observer
            .observe(fixture.access(|_| Ok(())), fixture.root.clone(), operation)
            .await
        {
            Ok(_) => break,
            Err(Error::Busy) if tokio::time::Instant::now() < deadline => {
                tokio::task::yield_now().await
            }
            value => panic!("scan did not release capacity: {value:?}"),
        }
    }
}
