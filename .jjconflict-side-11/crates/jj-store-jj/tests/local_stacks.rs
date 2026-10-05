//! Real stock-JJ fixtures; implementation exercises jj-lib, not subprocess commands.
use jj_store_jj::{
    local_browser,
    local_stacks::{self, Command, Request, State},
};
use jj_store_model as m;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command as Process,
    sync::Arc,
};
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
fn stock(root: &Path, args: &[&str]) -> String {
    let result = Process::new("/opt/homebrew/bin/jj")
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
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    String::from_utf8(result.stdout).unwrap()
}
fn fixture() -> tempfile::TempDir {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    stock(root, &["git", "init", "--colocate"]);
    fs::write(root.join(".gitignore"), "ignored.txt\n").unwrap();
    fs::write(root.join("base.txt"), "base\n").unwrap();
    stock(root, &["commit", "-m", "base"]);
    stock(root, &["bookmark", "create", "main", "-r", "@-"]);
    stock(root, &["new", "main", "-m", "topic one"]);
    fs::write(root.join("one.txt"), "one\n").unwrap();
    stock(root, &["bookmark", "create", "topic-one"]);
    stock(root, &["new", "main", "-m", "topic two"]);
    fs::write(root.join("two.txt"), "two\n").unwrap();
    stock(root, &["bookmark", "create", "topic-two"]);
    stock(root, &["new", "main"]);
    temp
}
async fn run(root: &Path, state: Option<&State>, command: Command) -> local_stacks::State {
    local_stacks::execute(
        Arc::new(Access(root.join(".jj/repo"))),
        root.canonicalize().unwrap(),
        Request {
            version: 1,
            binding_generation: 1,
            operation: state.map(|s| s.operation),
            revision: state.map(|s| s.revision),
            command,
        },
    )
    .await
    .unwrap()
}
async fn refresh(root: &Path) -> State {
    run(root, None, Command::Refresh).await
}
#[tokio::test]
async fn megamerge_membership_preserves_dirty_work_and_named_workspace_switching() {
    let temp = fixture();
    let root = temp.path();
    fs::write(root.join("draft.txt"), "draft one\n").unwrap();
    fs::write(root.join("ignored.txt"), "ignored\n").unwrap();
    let mut state = refresh(root).await;
    assert!(state.active.is_none());
    state = run(
        root,
        Some(&state),
        Command::Setup {
            name: "First".into(),
            target: "main".into(),
        },
    )
    .await;
    let first = state.active.clone().unwrap();
    assert_eq!(
        fs::read_to_string(root.join("draft.txt")).unwrap(),
        "draft one\n"
    );
    state = run(
        root,
        Some(&state),
        Command::Apply {
            branch: "topic-one".into(),
        },
    )
    .await;
    state = run(
        root,
        Some(&state),
        Command::Apply {
            branch: "topic-two".into(),
        },
    )
    .await;
    assert!(root.join("one.txt").exists() && root.join("two.txt").exists());
    assert_eq!(
        stock(
            root,
            &["log", "-r", "@-", "--no-graph", "-T", "parents.len()"]
        ),
        "2"
    );
    assert!(stock(root, &["file", "list"]).contains("draft.txt"));
    assert!(!stock(root, &["file", "list"]).contains("ignored.txt"));
    assert!(!stock(root, &["file", "list"]).contains(".jj/"));
    state = run(
        root,
        Some(&state),
        Command::Unapply {
            branch: "topic-one".into(),
        },
    )
    .await;
    assert!(!root.join("one.txt").exists() && root.join("two.txt").exists());
    state = run(
        root,
        Some(&state),
        Command::Create {
            name: "Second".into(),
        },
    )
    .await;
    let second = state.active.clone().unwrap();
    assert!(!root.join("draft.txt").exists());
    fs::write(root.join("draft.txt"), "draft two\n").unwrap();
    state = refresh(root).await;
    state = run(
        root,
        Some(&state),
        Command::Switch {
            workspace: first.clone(),
        },
    )
    .await;
    assert_eq!(
        fs::read_to_string(root.join("draft.txt")).unwrap(),
        "draft one\n"
    );
    state = run(root, Some(&state), Command::Switch { workspace: second }).await;
    assert_eq!(
        fs::read_to_string(root.join("draft.txt")).unwrap(),
        "draft two\n"
    );
    assert_eq!(state.workspaces.len(), 2);
    state = refresh(root).await;
    assert_eq!(state.workspaces.len(), 2);
    let config = fs::read_to_string(root.join(".jj/jj-store/stacks.toml")).unwrap();
    assert!(config.contains("[[workspaces]]") && config.contains("target = \"main\""));
}
#[tokio::test]
async fn stale_commands_are_rejected_and_refresh_tracks_external_edits() {
    let temp = fixture();
    let root = temp.path();
    let state = refresh(root).await;
    let state = run(
        root,
        Some(&state),
        Command::Setup {
            name: "Work".into(),
            target: "main".into(),
        },
    )
    .await;
    fs::write(root.join("base.txt"), "external edit\n").unwrap();
    let error = local_stacks::execute(
        Arc::new(Access(root.join(".jj/repo"))),
        root.canonicalize().unwrap(),
        Request {
            version: 1,
            binding_generation: 1,
            operation: Some(state.operation),
            revision: Some(state.revision),
            command: Command::Apply {
                branch: "topic-one".into(),
            },
        },
    )
    .await
    .unwrap_err();
    assert!(matches!(error, local_stacks::Error::Stale));
    let next = refresh(root).await;
    assert_ne!(state.operation, next.operation);
    assert!(next.workspaces[0].applied.is_empty());
    assert_eq!(
        fs::read_to_string(root.join("base.txt")).unwrap(),
        "external edit\n"
    );
    assert!(
        run(
            root,
            Some(&next),
            Command::Apply {
                branch: "topic-one".into()
            }
        )
        .await
        .warning
        .is_none()
    );
}
#[tokio::test]
async fn malformed_toml_is_an_error_and_does_not_replace_saved_settings() {
    let temp = fixture();
    let root = temp.path();
    let _ = refresh(root).await;
    let config = root.join(".jj/jj-store/stacks.toml");
    fs::write(&config, "version = 900\n").unwrap();
    assert!(
        local_stacks::execute(
            Arc::new(Access(root.join(".jj/repo"))),
            root.canonicalize().unwrap(),
            Request {
                version: 1,
                binding_generation: 1,
                operation: None,
                revision: None,
                command: Command::Refresh
            }
        )
        .await
        .is_err()
    );
    assert_eq!(fs::read_to_string(config).unwrap(), "version = 900\n");
}

#[tokio::test]
async fn cli_branch_advancement_refreshes_merge_and_retains_wip() {
    let temp = fixture();
    let root = temp.path();
    let state = refresh(root).await;
    let state = run(
        root,
        Some(&state),
        Command::Setup {
            name: "Work".into(),
            target: "main".into(),
        },
    )
    .await;
    let state = run(
        root,
        Some(&state),
        Command::Apply {
            branch: "topic-one".into(),
        },
    )
    .await;
    assert_eq!(state.workspaces[0].applied, vec!["topic-one"]);
    fs::write(
        root.join("draft.txt"),
        "retained
",
    )
    .unwrap();
    let state = refresh(root).await;
    // Advance a bookmark to an existing different tip without changing the checkout.
    stock(
        root,
        &[
            "bookmark",
            "set",
            "topic-one",
            "-r",
            "topic-two",
            "--allow-backwards",
        ],
    );
    let next = refresh(root).await;
    assert_ne!(state.operation, next.operation);
    assert!(root.join("two.txt").exists());
    assert!(!root.join("one.txt").exists());
    assert_eq!(
        fs::read_to_string(root.join("draft.txt")).unwrap(),
        "retained
"
    );
    assert_eq!(next.workspaces[0].applied, vec!["topic-one"]);
    stock(root, &["new", "main"]);
    let detached = refresh(root).await;
    assert!(detached.active.is_none());
    assert_eq!(detached.workspaces.len(), 1);
    let restored = run(
        root,
        Some(&detached),
        Command::Switch {
            workspace: state.active.unwrap(),
        },
    )
    .await;
    assert!(restored.active.is_some());
    assert_eq!(
        fs::read_to_string(root.join("draft.txt")).unwrap(),
        "retained
"
    );
}
#[tokio::test]
async fn megamerge_conflicts_remain_native_jj_conflicts() {
    let temp = fixture();
    let root = temp.path();
    stock(root, &["new", "main"]);
    fs::write(
        root.join("base.txt"),
        "left
",
    )
    .unwrap();
    stock(root, &["bookmark", "create", "left"]);
    stock(root, &["new", "main"]);
    fs::write(
        root.join("base.txt"),
        "right
",
    )
    .unwrap();
    stock(root, &["bookmark", "create", "right"]);
    stock(root, &["new", "main"]);
    let state = refresh(root).await;
    let state = run(
        root,
        Some(&state),
        Command::Setup {
            name: "Conflicts".into(),
            target: "main".into(),
        },
    )
    .await;
    let state = run(
        root,
        Some(&state),
        Command::Apply {
            branch: "left".into(),
        },
    )
    .await;
    let state = run(
        root,
        Some(&state),
        Command::Apply {
            branch: "right".into(),
        },
    )
    .await;
    assert!(state.warning.is_none());
    assert!(stock(root, &["resolve", "--list"]).contains("base.txt"));
    assert!(state.workspaces[0].conflicted);
    assert_eq!(state.workspaces[0].applied, vec!["left", "right"]);
}
struct FailSaveAccess {
    root: PathBuf,
    calls: std::sync::atomic::AtomicUsize,
}
#[async_trait::async_trait]
impl local_browser::Access for FailSaveAccess {
    fn repository(&self) -> m::RepositoryId {
        m::RepositoryId::new([81; 16])
    }
    fn generation(&self) -> u64 {
        1
    }
    fn metadata_path(&self) -> &Path {
        &self.root
    }
    async fn authorize(&self) -> local_browser::Result<()> {
        if self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst) == 1 {
            fs::create_dir(self.root.parent().unwrap().join("jj-store/stacks.toml")).unwrap();
        }
        Ok(())
    }
}
#[tokio::test]
async fn settings_save_failure_reports_the_applied_jj_operation_without_replay() {
    let temp = fixture();
    let root = temp.path();
    let before = refresh(root).await;
    let result = local_stacks::execute(
        Arc::new(FailSaveAccess {
            root: root.join(".jj/repo"),
            calls: Default::default(),
        }),
        root.canonicalize().unwrap(),
        Request {
            version: 1,
            binding_generation: 1,
            operation: Some(before.operation),
            revision: Some(before.revision),
            command: Command::Setup {
                name: "Retained".into(),
                target: "main".into(),
            },
        },
    )
    .await
    .unwrap();
    assert_ne!(before.operation, result.operation);
    assert!(result.warning.unwrap().contains("could not be saved"));
    assert_eq!(result.workspaces.len(), 1);
    assert_eq!(
        stock(root, &["op", "log", "-n", "1", "--no-graph", "-T", "id"]).trim(),
        result.operation.to_string()
    );
}

#[tokio::test]
async fn linked_physical_workspaces_keep_separate_toml_and_honor_native_sparse_checkout() {
    let source = fixture();
    let root = source.path();
    let state = refresh(root).await;
    let _ = run(
        root,
        Some(&state),
        Command::Setup {
            name: "Source".into(),
            target: "main".into(),
        },
    )
    .await;
    let before_config = fs::read(root.join(".jj/jj-store/stacks.toml")).unwrap();
    let before_wc = stock(root, &["log", "-r", "@", "--no-graph", "-T", "commit_id"]);
    let linked = tempfile::tempdir().unwrap();
    let folder = linked.path().join("workspace");
    stock(
        root,
        &[
            "workspace",
            "add",
            folder.to_str().unwrap(),
            "--name",
            "linked",
            "-r",
            "main",
        ],
    );
    stock(&folder, &["sparse", "set", "--clear", "--add", "base.txt"]);
    fs::write(folder.join("outside.txt"), "outside sparse checkout\n").unwrap();
    let execute = async |state: Option<State>, command| {
        local_stacks::execute(
            Arc::new(Access(root.join(".jj/repo"))),
            folder.canonicalize().unwrap(),
            Request {
                version: 1,
                binding_generation: 1,
                operation: state.as_ref().map(|s| s.operation),
                revision: state.as_ref().map(|s| s.revision),
                command,
            },
        )
        .await
        .unwrap()
    };
    let state = execute(None, Command::Refresh).await;
    let state = execute(
        Some(state),
        Command::Setup {
            name: "Linked".into(),
            target: "main".into(),
        },
    )
    .await;
    let _ = execute(
        Some(state),
        Command::Apply {
            branch: "topic-two".into(),
        },
    )
    .await;
    assert!(!folder.join("two.txt").exists());
    assert!(!stock(&folder, &["file", "list"]).contains("outside.txt"));
    assert_eq!(stock(&folder, &["file", "show", "two.txt"]), "two\n");
    assert_eq!(
        fs::read(root.join(".jj/jj-store/stacks.toml")).unwrap(),
        before_config
    );
    assert_eq!(
        stock(root, &["log", "-r", "@", "--no-graph", "-T", "commit_id"]),
        before_wc
    );
    let config = fs::read_to_string(folder.join(".jj/jj-store/stacks.toml")).unwrap();
    assert!(config.contains("Linked"));
    assert!(!config.contains("Source"));
}
