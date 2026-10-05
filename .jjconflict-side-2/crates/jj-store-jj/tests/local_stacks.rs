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

#[tokio::test]
async fn workspace_settings_and_removal_preserve_dirty_jj_work_and_reload() {
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
    let first = state.active.clone().unwrap();
    fs::write(root.join("draft.txt"), "unfinished work\n").unwrap();
    let state = refresh(root).await;
    let before_change = stock(root, &["log", "-r", "@", "--no-graph", "-T", "change_id"]);
    let state = run(
        root,
        Some(&state),
        Command::Update {
            workspace: first.clone(),
            name: "Renamed".into(),
            target: "topic-one".into(),
        },
    )
    .await;
    assert_eq!(state.workspaces[0].name, "Renamed");
    assert_eq!(state.workspaces[0].target, "topic-one");
    assert_eq!(
        stock(root, &["log", "-r", "@", "--no-graph", "-T", "change_id"]),
        before_change
    );
    assert!(root.join("one.txt").exists());
    assert_eq!(
        fs::read_to_string(root.join("draft.txt")).unwrap(),
        "unfinished work\n"
    );
    assert_eq!(refresh(root).await.workspaces[0].name, "Renamed");
    let merge = state.workspaces[0].merge_commit.clone();
    let wip = state.workspaces[0].wip_commit.clone();
    let state = run(
        root,
        Some(&state),
        Command::Update {
            workspace: first.clone(),
            name: "Name only".into(),
            target: "topic-one".into(),
        },
    )
    .await;
    assert_eq!(state.workspaces[0].merge_commit, merge);
    assert_eq!(state.workspaces[0].wip_commit, wip);
    let retained_wip = state.workspaces[0].wip_commit.clone();
    let state = run(
        root,
        Some(&state),
        Command::Create {
            name: "Other".into(),
        },
    )
    .await;
    let second = state.active.clone().unwrap();
    let state = run(root, Some(&state), Command::Delete { workspace: first }).await;
    assert_eq!(state.active.as_ref(), Some(&second));
    assert_eq!(state.workspaces.len(), 1);
    assert_eq!(
        stock(root, &["file", "show", "-r", &retained_wip, "draft.txt"]),
        "unfinished work\n"
    );
    fs::write(root.join("second.txt"), "another draft\n").unwrap();
    let state = refresh(root).await;
    let wc = stock(root, &["log", "-r", "@", "--no-graph", "-T", "commit_id"]);
    let bookmarks = stock(root, &["bookmark", "list", "-T", "name"]);
    let state = run(root, Some(&state), Command::Delete { workspace: second }).await;
    assert!(state.active.is_none() && state.workspaces.is_empty());
    assert_eq!(
        stock(root, &["log", "-r", "@", "--no-graph", "-T", "commit_id"]),
        wc
    );
    assert_eq!(stock(root, &["bookmark", "list", "-T", "name"]), bookmarks);
    assert_eq!(
        fs::read_to_string(root.join("second.txt")).unwrap(),
        "another draft\n"
    );
    // Retained app bookmarks keep both WIPs reachable, even after settings removal.
    assert!(stock(root, &["bookmark", "list"]).contains("/wip"));
    assert!(refresh(root).await.workspaces.is_empty());
}

#[tokio::test]
async fn workspace_settings_reject_invalid_and_stale_intent_without_config_changes() {
    let temp = fixture();
    let root = temp.path();
    let state = refresh(root).await;
    let state = run(
        root,
        Some(&state),
        Command::Setup {
            name: "First".into(),
            target: "main".into(),
        },
    )
    .await;
    let inactive = state.active.clone().unwrap();
    let state = run(
        root,
        Some(&state),
        Command::Create {
            name: "Second".into(),
        },
    )
    .await;
    let active = state.active.clone().unwrap();
    let bytes = fs::read(root.join(".jj/jj-store/stacks.toml")).unwrap();
    let wip = state
        .workspaces
        .iter()
        .find(|w| w.id == active)
        .unwrap()
        .wip_commit
        .clone();
    for command in [
        Command::Update {
            workspace: active.clone(),
            name: "First".into(),
            target: "main".into(),
        },
        Command::Update {
            workspace: active.clone(),
            name: "\n".into(),
            target: "main".into(),
        },
        Command::Update {
            workspace: active.clone(),
            name: "Valid".into(),
            target: "missing".into(),
        },
        Command::Update {
            workspace: inactive,
            name: "Valid".into(),
            target: "main".into(),
        },
        Command::Delete {
            workspace: "missing".into(),
        },
    ] {
        let error = local_stacks::execute(
            Arc::new(Access(root.join(".jj/repo"))),
            root.canonicalize().unwrap(),
            Request {
                version: 1,
                binding_generation: 1,
                operation: Some(state.operation),
                revision: Some(state.revision),
                command,
            },
        )
        .await
        .unwrap_err();
        assert!(matches!(error, local_stacks::Error::Invalid));
        assert_eq!(
            fs::read(root.join(".jj/jj-store/stacks.toml")).unwrap(),
            bytes
        );
    }
    let error = local_stacks::execute(
        Arc::new(Access(root.join(".jj/repo"))),
        root.canonicalize().unwrap(),
        Request {
            version: 1,
            binding_generation: 1,
            operation: Some(state.operation),
            revision: Some(state.revision - 1),
            command: Command::Delete { workspace: active },
        },
    )
    .await
    .unwrap_err();
    assert!(matches!(error, local_stacks::Error::Stale));
    assert_eq!(
        stock(root, &["log", "-r", "@", "--no-graph", "-T", "commit_id"]),
        wip
    );
    assert_eq!(
        fs::read(root.join(".jj/jj-store/stacks.toml")).unwrap(),
        bytes
    );
}

#[tokio::test]
async fn selected_file_commit_uses_native_propagation_and_retains_wip() {
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
    let _state = run(
        root,
        Some(&state),
        Command::Apply {
            branch: "topic-one".into(),
        },
    )
    .await;
    fs::write(root.join("selected.txt"), "selected\n").unwrap();
    fs::write(root.join("remaining.txt"), "remaining\n").unwrap();
    let state = refresh(root).await;
    let w = &state.workspaces[0];
    let merge_change = stock(
        root,
        &[
            "log",
            "--no-graph",
            "-r",
            &w.merge_commit,
            "-T",
            "change_id",
        ],
    );
    let wip_change = stock(
        root,
        &["log", "--no-graph", "-r", &w.wip_commit, "-T", "change_id"],
    );
    let request = Request {
        version: 1,
        binding_generation: 1,
        operation: Some(state.operation),
        revision: Some(state.revision),
        command: Command::Commit {
            branch: "topic-one".into(),
            message: "Selected file".into(),
            selections: vec![local_stacks::Selection {
                source: w.wip_commit.clone(),
                path: "selected.txt".into(),
            }],
        },
    };
    let before = stock(root, &["op", "log", "--no-graph", "-n", "1", "-T", "id"]);
    let preview = local_stacks::preview(
        Arc::new(Access(root.join(".jj/repo"))),
        root.canonicalize().unwrap(),
        request.clone(),
    )
    .await
    .unwrap();
    assert_eq!(preview.paths, vec!["selected.txt"]);
    assert_eq!(
        before,
        stock(root, &["op", "log", "--no-graph", "-n", "1", "-T", "id"])
    );
    let next = local_stacks::execute(
        Arc::new(Access(root.join(".jj/repo"))),
        root.canonicalize().unwrap(),
        request,
    )
    .await
    .unwrap();
    assert_eq!(
        stock(
            root,
            &[
                "log",
                "--no-graph",
                "-r",
                &next.workspaces[0].merge_commit,
                "-T",
                "change_id"
            ]
        ),
        merge_change
    );
    assert_eq!(
        stock(
            root,
            &[
                "log",
                "--no-graph",
                "-r",
                &next.workspaces[0].wip_commit,
                "-T",
                "change_id"
            ]
        ),
        wip_change
    );
    assert_eq!(
        stock(root, &["file", "show", "-r", "topic-one", "selected.txt"]),
        "selected\n"
    );
    assert_eq!(
        stock(root, &["diff", "-r", "@", "--name-only"]),
        "remaining.txt\n"
    );
    let next = run(
        root,
        Some(&next),
        Command::Commit {
            branch: "topic-one".into(),
            message: "Last file".into(),
            selections: vec![local_stacks::Selection {
                source: next.workspaces[0].wip_commit.clone(),
                path: "remaining.txt".into(),
            }],
        },
    )
    .await;
    assert_eq!(
        stock(root, &["log", "--no-graph", "-r", "@", "-T", "empty"]),
        "true"
    );
    assert_eq!(
        stock(
            root,
            &[
                "log",
                "--no-graph",
                "-r",
                &next.workspaces[0].wip_commit,
                "-T",
                "change_id"
            ]
        ),
        wip_change
    );
}

#[tokio::test]
async fn selected_file_commands_reject_invalid_stale_and_immutable_sources() {
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
    let _ = run(
        root,
        Some(&state),
        Command::Apply {
            branch: "topic-one".into(),
        },
    )
    .await;
    fs::write(root.join("selected.txt"), "selected\n").unwrap();
    let state = refresh(root).await;
    let config = fs::read(root.join(".jj/jj-store/stacks.toml")).unwrap();
    let request = Request {
        version: 1,
        binding_generation: 1,
        operation: Some(state.operation),
        revision: Some(state.revision),
        command: Command::Commit {
            branch: "topic-one".into(),
            message: "Selection".into(),
            selections: vec![local_stacks::Selection {
                source: state.workspaces[0].wip_commit.clone(),
                path: "selected.txt".into(),
            }],
        },
    };
    let mut invalid = request.clone();
    if let Command::Commit { selections, .. } = &mut invalid.command {
        selections[0].path = "../selected.txt".into();
    }
    let mut stale = request.clone();
    stale.revision = Some(state.revision + 1);
    for invalid in [invalid, stale] {
        assert!(
            local_stacks::preview(
                Arc::new(Access(root.join(".jj/repo"))),
                root.canonicalize().unwrap(),
                invalid.clone()
            )
            .await
            .is_err()
        );
        assert!(
            local_stacks::execute(
                Arc::new(Access(root.join(".jj/repo"))),
                root.canonicalize().unwrap(),
                invalid
            )
            .await
            .is_err()
        );
    }
    fs::write(
        root.join(".jj/repo/config.toml"),
        "[revset-aliases]\n'immutable_heads()' = '@'\n",
    )
    .unwrap();
    assert!(
        local_stacks::preview(
            Arc::new(Access(root.join(".jj/repo"))),
            root.canonicalize().unwrap(),
            request.clone()
        )
        .await
        .is_err()
    );
    assert!(
        local_stacks::execute(
            Arc::new(Access(root.join(".jj/repo"))),
            root.canonicalize().unwrap(),
            request
        )
        .await
        .is_err()
    );
    assert_eq!(
        fs::read(root.join(".jj/jj-store/stacks.toml")).unwrap(),
        config
    );
    assert_eq!(
        stock(root, &["op", "log", "--no-graph", "-n", "1", "-T", "id"]).trim(),
        state.operation.to_string()
    );
    assert_eq!(
        fs::read_to_string(root.join("selected.txt")).unwrap(),
        "selected\n"
    );
}

#[tokio::test]
async fn local_bookmark_and_basic_commit_actions_use_native_rewrites() {
    use local_stacks::Placement;
    let temp = fixture();
    let root = temp.path();
    let state = refresh(root).await;
    let _state = run(
        root,
        Some(&state),
        Command::Setup {
            name: "Work".into(),
            target: "main".into(),
        },
    )
    .await;
    fs::write(root.join("draft.txt"), "unfinished\n").unwrap();
    let state = refresh(root).await;
    let state = run(
        root,
        Some(&state),
        Command::CreateStack {
            name: "independent".into(),
            parent: None,
        },
    )
    .await;
    assert_eq!(
        stock(
            root,
            &["log", "--no-graph", "-r", "independent", "-T", "commit_id"]
        ),
        stock(
            root,
            &["log", "--no-graph", "-r", "main", "-T", "commit_id"]
        )
    );
    let state = run(
        root,
        Some(&state),
        Command::Apply {
            branch: "topic-one".into(),
        },
    )
    .await;
    let state = run(
        root,
        Some(&state),
        Command::CreateStack {
            name: "dependent".into(),
            parent: Some("topic-one".into()),
        },
    )
    .await;
    let state = run(
        root,
        Some(&state),
        Command::RenameBranch {
            branch: "dependent".into(),
            name: "renamed".into(),
        },
    )
    .await;
    assert!(state.workspaces[0].applied.contains(&"renamed".into()));
    assert!(!state.workspaces[0].applied.contains(&"dependent".into()));
    let tip = stock(
        root,
        &["log", "--no-graph", "-r", "topic-one", "-T", "commit_id"],
    );
    let merge = stock(root, &["log", "--no-graph", "-r", "@-", "-T", "change_id"]);
    let wip = stock(root, &["log", "--no-graph", "-r", "@", "-T", "change_id"]);
    let inspected = local_stacks::inspect(
        Arc::new(Access(root.join(".jj/repo"))),
        root.canonicalize().unwrap(),
        local_stacks::InspectionRequest {
            version: 1,
            binding_generation: 1,
            operation: state.operation,
            revision: state.revision,
            commits: vec![tip.clone(), state.workspaces[0].target_commit.clone()],
        },
    )
    .await
    .unwrap();
    assert!(inspected.commits[0].mutable);
    assert!(!inspected.commits[1].mutable);
    let state = run(
        root,
        Some(&state),
        Command::CreateBranch {
            commit: tip.clone(),
            name: "above".into(),
            placement: Placement::Above,
        },
    )
    .await;
    let state = run(
        root,
        Some(&state),
        Command::CreateBranch {
            commit: tip.clone(),
            name: "below".into(),
            placement: Placement::Below,
        },
    )
    .await;
    assert_eq!(
        stock(
            root,
            &["log", "--no-graph", "-r", "below", "-T", "commit_id"]
        ),
        state.workspaces[0].target_commit
    );
    let state = run(
        root,
        Some(&state),
        Command::Reword {
            commit: tip,
            message: "Updated description".into(),
        },
    )
    .await;
    assert_eq!(
        stock(
            root,
            &["log", "--no-graph", "-r", "topic-one", "-T", "description"]
        ),
        "Updated description"
    );
    let tip = stock(
        root,
        &["log", "--no-graph", "-r", "topic-one", "-T", "commit_id"],
    );
    let state = run(
        root,
        Some(&state),
        Command::Empty {
            commit: tip,
            placement: Placement::Below,
            message: "Below".into(),
        },
    )
    .await;
    assert_eq!(
        stock(
            root,
            &["log", "--no-graph", "-r", "topic-one-", "-T", "description"]
        ),
        "Below"
    );
    let tip = stock(
        root,
        &["log", "--no-graph", "-r", "topic-one", "-T", "commit_id"],
    );
    let _state = run(
        root,
        Some(&state),
        Command::Empty {
            commit: tip,
            placement: Placement::Above,
            message: "Above".into(),
        },
    )
    .await;
    assert_eq!(
        stock(
            root,
            &["log", "--no-graph", "-r", "topic-one", "-T", "description"]
        ),
        "Above"
    );
    assert_eq!(
        stock(root, &["log", "--no-graph", "-r", "@-", "-T", "change_id"]),
        merge
    );
    assert_eq!(
        stock(root, &["log", "--no-graph", "-r", "@", "-T", "change_id"]),
        wip
    );
    assert_eq!(
        fs::read_to_string(root.join("draft.txt")).unwrap(),
        "unfinished\n"
    );
}

#[tokio::test]
async fn first_commit_on_empty_stack_forks_target_without_rewriting_other_branches() {
    let temp = fixture();
    let root = temp.path();
    let main = stock(
        root,
        &["log", "--no-graph", "-r", "main", "-T", "commit_id"],
    );
    let unrelated = stock(
        root,
        &["log", "--no-graph", "-r", "topic-one", "-T", "commit_id"],
    );
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
    let _ = run(
        root,
        Some(&state),
        Command::CreateStack {
            name: "new-stack".into(),
            parent: None,
        },
    )
    .await;
    fs::write(root.join("first.txt"), "first\n").unwrap();
    let state = refresh(root).await;
    let merge = stock(root, &["log", "--no-graph", "-r", "@-", "-T", "change_id"]);
    let wip = stock(root, &["log", "--no-graph", "-r", "@", "-T", "change_id"]);
    let state = run(
        root,
        Some(&state),
        Command::Commit {
            branch: "new-stack".into(),
            message: "First".into(),
            selections: vec![local_stacks::Selection {
                source: state.workspaces[0].wip_commit.clone(),
                path: "first.txt".into(),
            }],
        },
    )
    .await;
    assert_eq!(
        stock(
            root,
            &["log", "--no-graph", "-r", "main", "-T", "commit_id"]
        ),
        main
    );
    assert_eq!(
        stock(
            root,
            &["log", "--no-graph", "-r", "topic-one", "-T", "commit_id"]
        ),
        unrelated
    );
    assert_eq!(stock(root, &["diff", "-r", "@", "--name-only"]), "");
    let refreshed = refresh(root).await;
    assert_eq!(state.operation, refreshed.operation);
    assert_eq!(
        stock(root, &["log", "--no-graph", "-r", "@-", "-T", "change_id"]),
        merge
    );
    assert_eq!(
        stock(root, &["log", "--no-graph", "-r", "@", "-T", "change_id"]),
        wip
    );
    let _ = run(
        root,
        Some(&refreshed),
        Command::Commit {
            branch: "new-stack".into(),
            message: "Empty".into(),
            selections: vec![],
        },
    )
    .await;
    assert_eq!(
        stock(
            root,
            &["log", "--no-graph", "-r", "new-stack", "-T", "empty"]
        ),
        "true"
    );
}
