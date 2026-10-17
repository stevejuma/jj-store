//! A real Git-backed JJ consumer with no native storage feature.
use jj_store_jj::{
    local_browser as q,
    query::{Cancellation, cache},
};
use jj_store_model as m;
use q::Access as _;
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::{
        Arc,
        atomic::{AtomicU64, AtomicUsize, Ordering},
    },
};

struct Access {
    metadata: PathBuf,
    generation: AtomicU64,
    remaining: AtomicUsize,
}
#[async_trait::async_trait]
impl q::Access for Access {
    fn repository(&self) -> m::RepositoryId {
        m::RepositoryId::new([41; 16])
    }
    fn generation(&self) -> u64 {
        self.generation.load(Ordering::SeqCst)
    }
    fn metadata_path(&self) -> &Path {
        &self.metadata
    }
    async fn authorize(&self) -> q::Result<()> {
        self.remaining
            .try_update(Ordering::SeqCst, Ordering::SeqCst, |n| n.checked_sub(1))
            .map_err(|_| q::Error::Denied)?;
        if !self.metadata.is_dir() {
            return Err(q::Error::Denied);
        }
        Ok(())
    }
}
fn executable() -> PathBuf {
    if let Some(path) = std::env::var_os("JJ_STORE_TEST_STOCK_JJ") {
        let path = PathBuf::from(path);
        assert!(
            path.is_absolute(),
            "JJ_STORE_TEST_STOCK_JJ must be absolute"
        );
        return path;
    }
    std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())
        .map(|root| root.join("jj"))
        .find(|path| path.is_file())
        .expect("pinned stock JJ 0.45.1 is required")
        .canonicalize()
        .unwrap()
}
fn jj(root: &Path, args: &[&str]) -> String {
    let output = Command::new(executable())
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
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}
fn state(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    fn visit(root: &Path, path: &Path, result: &mut BTreeMap<PathBuf, Vec<u8>>) {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let kind = entry.file_type().unwrap();
            if kind.is_dir() {
                visit(root, &path, result);
            } else {
                assert!(kind.is_file(), "unexpected fixture symlink");
                result.insert(
                    path.strip_prefix(root).unwrap().to_owned(),
                    fs::read(path).unwrap(),
                );
            }
        }
    }
    let mut result = BTreeMap::new();
    visit(root, root, &mut result);
    result
}
fn log() -> q::Request {
    q::Request::Graph(
        q::GraphRequest::Log {
            revset: "all()".into(),
            workspace: None,
            user_email: "fixture@invalid".into(),
            at_time_millis: 0,
        },
        q::GraphWindow {
            limit: 32,
            after: None,
        },
    )
}

#[tokio::test]
async fn standalone_history_preserves_source_pins_diffs_cancellation_and_current_admission() {
    let source = tempfile::tempdir().unwrap();
    let root = source.path().canonicalize().unwrap();
    assert_eq!(jj(&root, &["--version"]), "jj 0.45.1");
    jj(&root, &["git", "init", "--colocate"]);
    fs::write(root.join("note.md"), "first\n").unwrap();
    jj(&root, &["describe", "-m", "First"]);
    let first: m::GitSha1Id = jj(&root, &["log", "-r", "@", "--no-graph", "-T", "commit_id"])
        .parse()
        .unwrap();
    jj(&root, &["new", "-m", "Second"]);
    fs::write(root.join("note.md"), "second\n").unwrap();
    jj(&root, &["describe", "-m", "Second"]);
    let second: m::GitSha1Id = jj(&root, &["log", "-r", "@", "--no-graph", "-T", "commit_id"])
        .parse()
        .unwrap();
    fs::write(root.join("note.md"), "dirty, not recorded\n").unwrap();
    let access = Arc::new(Access {
        metadata: root.join(".jj/repo"),
        generation: AtomicU64::new(7),
        remaining: AtomicUsize::new(usize::MAX),
    });
    let executor = q::Executor::new(2).unwrap();
    let before = state(&root);
    let operation = executor.heads(access.clone()).await.unwrap().heads[0];
    let scratch = tempfile::tempdir().unwrap();
    let policy = cache::Policy {
        max_entry_bytes: 32 * 1024 * 1024,
        ..Default::default()
    };
    let binding = cache::managed::Binding::Local {
        repository: access.repository(),
        generation: 7,
    };
    let namespace =
        || cache::managed::Namespace::acquire(scratch.path(), binding.clone(), policy).unwrap();
    let canceled = Cancellation::default();
    canceled.cancel();
    assert!(matches!(
        executor
            .prepare_managed_code_index(
                access.clone(),
                operation,
                namespace(),
                q::preparation::Limits::default(),
                canceled
            )
            .await,
        Err(q::Error::Canceled)
    ));
    assert_eq!(state(&root), before);
    let prepared = Arc::new(
        executor
            .prepare_managed_code_index(
                access.clone(),
                operation,
                namespace(),
                q::preparation::Limits::default(),
                Cancellation::default(),
            )
            .await
            .unwrap(),
    );
    assert_eq!(prepared.snapshot().operation, operation);
    let q::Response::Graph(page) = executor
        .read_code_index(access.clone(), prepared.clone(), log())
        .await
        .unwrap()
    else {
        panic!("expected graph");
    };
    assert_eq!(page.operation, operation);
    let q::Nodes::Log(nodes) = page.data else {
        panic!("expected log");
    };
    assert!(nodes.iter().any(|row| row.commit.id == first));
    assert!(nodes.iter().any(|row| row.commit.id == second));
    let q::Response::Diff(diff) = executor
        .read_code_index(
            access.clone(),
            prepared.clone(),
            q::Request::Diff(q::diff::Request {
                from: first,
                to: second,
                path: "note.md".into(),
                content: true,
            }),
        )
        .await
        .unwrap()
    else {
        panic!("expected diff");
    };
    assert_eq!(diff.changes.len(), 1);
    let q::diff::Content::Text { hunks } = &diff.changes[0].content else {
        panic!("expected text");
    };
    assert_eq!(
        hunks.iter().map(|h| h.before.as_str()).collect::<String>(),
        "first\n"
    );
    assert_eq!(
        hunks.iter().map(|h| h.after.as_str()).collect::<String>(),
        "second\n"
    );
    assert_eq!(
        state(&root),
        before,
        "reads repaired an index or changed dirty source files"
    );

    // An external writer advances the source; the retained selection stays old.
    jj(&root, &["new", "-m", "Third"]);
    let third: m::GitSha1Id = jj(&root, &["log", "-r", "@", "--no-graph", "-T", "commit_id"])
        .parse()
        .unwrap();
    fs::write(root.join("note.md"), "another dirty edit\n").unwrap();
    let advanced = state(&root);
    let q::Response::Graph(page) = executor
        .read_code_index(access.clone(), prepared.clone(), log())
        .await
        .unwrap()
    else {
        panic!("expected graph");
    };
    assert_eq!(page.operation, operation);
    let q::Nodes::Log(nodes) = page.data else {
        panic!("expected log");
    };
    assert!(!nodes.iter().any(|row| row.commit.id == third));
    assert!(matches!(
        executor
            .read_code_index(
                access.clone(),
                prepared.clone(),
                q::Request::CommitSummary { commit: third }
            )
            .await,
        Err(q::Error::NotCommitted)
    ));
    access.generation.store(8, Ordering::SeqCst);
    assert!(matches!(
        executor
            .read_code_index(access.clone(), prepared.clone(), log())
            .await,
        Err(q::Error::EpochChanged)
    ));
    access.generation.store(7, Ordering::SeqCst);
    access.remaining.store(1, Ordering::SeqCst);
    assert!(
        matches!(executor.heads(access.clone()).await, Err(q::Error::Denied)),
        "completion authorization was skipped"
    );
    assert_eq!(state(&root), advanced);
}
