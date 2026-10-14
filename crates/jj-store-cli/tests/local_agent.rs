//! Cold executable acceptance for ordinary JJ repositories without a server.
use serde_json::Value;
use std::{
    fs,
    io::Write as _,
    path::Path,
    process::{Command, Output, Stdio},
};

fn command(path: &Path, bin: &Path, args: &[&str]) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jj-store-jj"));
    command
        .args([
            "--no-pager",
            "--config",
            "user.name='Local Agent Test'",
            "--config",
            "user.email='agent@example.invalid'",
        ])
        .args(args)
        .current_dir(path)
        .env("JJ_CONFIG", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("PATH", format!("{}:/usr/bin:/bin", bin.display()));
    command
}
fn run(path: &Path, bin: &Path, args: &[&str]) -> Output {
    command(path, bin, args).output().unwrap()
}
fn json(path: &Path, bin: &Path, args: &[&str]) -> Value {
    let result = run(path, bin, args);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    serde_json::from_slice(&result.stdout).unwrap()
}
fn success(path: &Path, bin: &Path, args: &[&str]) {
    let result = run(path, bin, args);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
}
fn init(root: &Path, kind: &str) -> std::path::PathBuf {
    let path = root.join(kind);
    let args = match kind {
        "colocated" => vec!["git", "init", "--colocate", path.to_str().unwrap()],
        "internal" => vec!["git", "init", path.to_str().unwrap()],
        "external" => {
            success(root, root, &["git", "init", "--colocate", "git-source"]);
            vec![
                "git",
                "init",
                "--git-repo",
                "git-source",
                path.to_str().unwrap(),
            ]
        }
        _ => unreachable!(),
    };
    success(root, root, &args);
    path
}
#[test]
fn local_agent_reads_and_initialization_accept_all_git_layouts_and_workspaces() {
    for kind in ["colocated", "internal", "external"] {
        let root = tempfile::tempdir().unwrap();
        let path = init(root.path(), kind);
        let state = root.path().join("device");
        let state = state.to_str().unwrap();
        let status = json(
            &path,
            root.path(),
            &["agent", "--state-dir", state, "--format", "json", "status"],
        );
        assert_eq!(status["profile"], "local_jj_git");
        assert!(status["repository"].is_null());
        assert!(
            json(
                &path,
                root.path(),
                &["agent", "--format", "json", "sessions"]
            )["sessions"]
                .as_array()
                .unwrap()
                .is_empty()
        );
        assert_eq!(
            json(
                &path,
                root.path(),
                &[
                    "agent",
                    "--format",
                    "json",
                    "receipt",
                    "01010101010101010101010101010101"
                ]
            )["status"],
            "not_known"
        );
        assert!(!Path::new(state).exists());
        assert!(!path.join(".jj/repo/lapis").exists());
        assert!(
            !run(
                &path,
                root.path(),
                &["agent", "enable", "--non-interactive", "--agent", "codex"]
            )
            .status
            .success()
        );
        assert!(!path.join(".jj/repo/lapis").exists());
        fs::write(path.join("unsnapshotted.txt"), "User content").unwrap();
        let before = run(
            &path,
            root.path(),
            &[
                "--ignore-working-copy",
                "op",
                "log",
                "--no-graph",
                "--limit",
                "1",
                "--template",
                "id",
            ],
        )
        .stdout;
        let args = [
            "agent",
            "--format",
            "json",
            "initialize",
            "--request-id",
            "01010101010101010101010101010101",
        ];
        let original = json(&path, root.path(), &args);
        assert_eq!(original["status"], "committed");
        success(
            &path,
            root.path(),
            &[
                "project",
                "create",
                "--request-id",
                "02020202020202020202020202020202",
                "--title",
                "Agent home",
            ],
        );
        assert_eq!(json(&path, root.path(), &args), original);
        assert_eq!(
            json(
                &path,
                root.path(),
                &[
                    "agent",
                    "--format",
                    "json",
                    "receipt",
                    "01010101010101010101010101010101"
                ]
            ),
            original
        );
        let after = run(
            &path,
            root.path(),
            &[
                "--ignore-working-copy",
                "op",
                "log",
                "--no-graph",
                "--limit",
                "1",
                "--template",
                "id",
            ],
        )
        .stdout;
        assert_eq!(before, after);
        assert_eq!(
            fs::read(path.join("unsnapshotted.txt")).unwrap(),
            b"User content"
        );
        let sibling = root.path().join("sibling");
        success(
            &path,
            root.path(),
            &[
                "workspace",
                "add",
                sibling.to_str().unwrap(),
                "--name",
                "sibling",
                "-m",
                "Disposable agent workspace",
            ],
        );
        assert_eq!(json(&sibling, root.path(), &args), original);
        let capabilities = json(
            &path,
            root.path(),
            &["store", "--format", "json", "capabilities"],
        );
        assert_eq!(capabilities["collaboration"]["agents"], true);
        assert_eq!(capabilities["collaboration"]["agent_code_anchors"], true);
    }
}
#[cfg(unix)]
#[test]
fn cold_hook_sanitizes_and_publishes_to_local_git_without_native_connection() {
    use std::os::unix::fs::PermissionsExt as _;
    let root = tempfile::tempdir().unwrap();
    let path = init(root.path(), "colocated");
    let bin = root.path().join("bin");
    fs::create_dir(&bin).unwrap();
    fs::write(
        bin.join("codex"),
        "#!/bin/sh\nprintf 'codex-cli 0.156.1\\n'\n",
    )
    .unwrap();
    fs::set_permissions(bin.join("codex"), fs::Permissions::from_mode(0o700)).unwrap();
    let state = root.path().join("device");
    let state_text = state.to_str().unwrap();
    let preview = json(
        &path,
        &bin,
        &[
            "agent",
            "--format",
            "json",
            "--state-dir",
            state_text,
            "enable",
            "--non-interactive",
            "--agent",
            "codex",
            "--dry-run",
        ],
    );
    assert_eq!(preview["dry_run"], true);
    assert_eq!(preview["integrations"]["codex"]["state"], "enabled");
    assert!(!path.join(".jj/repo/lapis").exists());
    assert!(!state.exists());
    success(
        &path,
        &bin,
        &[
            "agent",
            "--state-dir",
            state_text,
            "enable",
            "--non-interactive",
            "--agent",
            "codex",
        ],
    );
    success(
        &path,
        &bin,
        &["agent", "--state-dir", state_text, "setup", "codex"],
    );
    let secret = "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";
    let event = serde_json::json!({"hook_event_name":"UserPromptSubmit","session_id":"local-session","turn_id":"turn-one","cwd":path,"prompt":format!("Explain this token {secret}")});
    let mut hook = command(
        &path,
        &bin,
        &["agent", "--state-dir", state_text, "hook", "codex"],
    )
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .unwrap();
    hook.stdin
        .take()
        .unwrap()
        .write_all(&serde_json::to_vec(&event).unwrap())
        .unwrap();
    let result = hook.wait_with_output().unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(result.stdout.is_empty());
    let drain = json(
        &path,
        &bin,
        &[
            "agent",
            "--format",
            "json",
            "--state-dir",
            state_text,
            "drain",
        ],
    );
    assert_eq!(drain["remaining"], 0);
    let sessions = json(&path, &bin, &["agent", "--format", "json", "sessions"]);
    assert_eq!(sessions["sessions"].as_array().unwrap().len(), 1);
    assert_eq!(sessions["snapshot"]["profile"], "local_jj_git");
    let git = path.join(".git");
    let objects = Command::new("git")
        .args(["--git-dir"])
        .arg(&git)
        .args(["cat-file", "--batch", "--batch-all-objects"])
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .output()
        .unwrap();
    assert!(objects.status.success());
    assert!(
        !objects
            .stdout
            .windows(secret.len())
            .any(|bytes| bytes == secret.as_bytes())
    );
    assert!(!path.join(".jj/repo/store-connection.json").exists());
    success(
        &path,
        &bin,
        &["agent", "--state-dir", state_text, "remove", "codex"],
    );
}
