//! Server-free executable acceptance against disposable ordinary JJ repositories.
use serde_json::Value;
use std::{
    fs,
    path::Path,
    process::{Command, Output},
};

fn run(path: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jj-store-jj"))
        .args([
            "--no-pager",
            "--config",
            "user.name='Local Work Test'",
            "--config",
            "user.email='work@example.invalid'",
        ])
        .args(args)
        .current_dir(path)
        .env("JJ_CONFIG", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap()
}
fn success(path: &Path, args: &[&str]) -> String {
    let output = run(path, args);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}
fn json(path: &Path, args: &[&str]) -> Value {
    serde_json::from_str(&success(path, args)).unwrap()
}
fn init(root: &Path, kind: &str) -> std::path::PathBuf {
    let path = root.join(kind);
    let path_text = path.to_str().unwrap();
    match kind {
        "colocated" => {
            success(root, &["git", "init", "--colocate", path_text]);
        }
        "internal" => {
            success(root, &["git", "init", path_text]);
        }
        "external" => {
            let git_path = root.join("git-source");
            success(
                root,
                &["git", "init", "--colocate", git_path.to_str().unwrap()],
            );
            success(
                root,
                &[
                    "git",
                    "init",
                    "--git-repo",
                    git_path.to_str().unwrap(),
                    path_text,
                ],
            );
        }
        _ => unreachable!(),
    }
    path
}

#[test]
fn project_task_comment_commands_work_without_native_connection_or_server() {
    for kind in ["colocated", "internal", "external"] {
        let root = tempfile::tempdir().unwrap();
        let path = init(root.path(), kind);
        let first = json(&path, &["project", "--format", "json", "list"]);
        let capabilities = json(&path, &["store", "--format", "json", "capabilities"]);
        assert_eq!(capabilities["profile"], "local_jj_git");
        assert_eq!(capabilities["server_required"], false);
        assert_eq!(capabilities["metadata_initialized"], false);
        assert_eq!(capabilities["collaboration"]["tasks"], true);
        assert_eq!(first["version"], 2);
        assert!(first["snapshot"]["repo_id"].is_null());
        assert!(!path.join(".jj/repo/lapis").exists());
        assert!(!path.join(".jj/repo/store-connection.json").exists());
        fs::write(path.join("unsnapshotted.txt"), b"local user edit").unwrap();
        let request = "01010101010101010101010101010101";
        let args = [
            "project",
            "--format",
            "json",
            "create",
            "--request-id",
            request,
            "--title",
            "Local home",
            "--body",
            "Project body",
        ];
        let created = json(&path, &args);
        assert_eq!(created["status"], "committed");
        assert_eq!(created["receipt"]["profile"], "local_jj_git");
        assert_eq!(
            created["receipt"]["work_object"].as_str().unwrap().len(),
            40
        );
        assert_eq!(json(&path, &args), created);
        assert_eq!(
            json(&path, &["project", "--format", "json", "status", request]),
            created
        );
        let project = created["receipt"]["result"]["project"]["id"]
            .as_str()
            .unwrap();
        let task = json(
            &path,
            &[
                "task",
                "--format",
                "json",
                "create",
                "--request-id",
                "02020202020202020202020202020202",
                "--project",
                project,
                "--expected-project-version",
                "1",
                "--title",
                "Local task",
            ],
        );
        let task_id = task["receipt"]["result"]["task"]["id"].as_str().unwrap();
        let comment = json(
            &path,
            &[
                "task",
                "--format",
                "json",
                "comment",
                task_id,
                "--request-id",
                "03030303030303030303030303030303",
                "--expected-task-version",
                "1",
                "--body",
                "Original comment",
            ],
        );
        let comment_id = comment["receipt"]["result"]["comment"]["id"]
            .as_str()
            .unwrap();
        json(
            &path,
            &[
                "task",
                "--format",
                "json",
                "tombstone-comment",
                comment_id,
                "--request-id",
                "04040404040404040404040404040404",
                "--expected-version",
                "1",
                "--expected-task-version",
                "2",
            ],
        );
        let selected = json(&path, &["task", "--format", "json", "show", task_id]);
        assert!(selected["comments"][0]["body"].is_null());
        assert_eq!(selected["tasks"][0]["version"], 3);
        assert_eq!(
            fs::read(path.join("unsnapshotted.txt")).unwrap(),
            b"local user edit"
        );
        let stale = run(
            &path,
            &[
                "project",
                "edit",
                project,
                "--request-id",
                "05050505050505050505050505050505",
                "--expected-version",
                "99",
                "--title",
                "Stale",
                "--body",
                "Rejected",
            ],
        );
        assert!(!stale.status.success());
        assert!(String::from_utf8_lossy(&stale.stderr).contains("stale"));
        let read = json(&path, &["project", "--format", "json", "show", project]);
        assert_eq!(read["projects"][0]["title"], "Local home");
    }
}

#[test]
fn jj_workspaces_share_local_actor_metadata_and_pinned_continuations() {
    let root = tempfile::tempdir().unwrap();
    let path = init(root.path(), "colocated");
    let created = json(
        &path,
        &[
            "project",
            "--format",
            "json",
            "create",
            "--request-id",
            "11111111111111111111111111111111",
            "--title",
            "First",
        ],
    );
    json(
        &path,
        &[
            "project",
            "--format",
            "json",
            "create",
            "--request-id",
            "22222222222222222222222222222222",
            "--title",
            "Second",
        ],
    );
    let sibling = root.path().join("sibling");
    success(
        &path,
        &[
            "workspace",
            "add",
            sibling.to_str().unwrap(),
            "--name",
            "sibling",
            "-m",
            "test sibling",
        ],
    );
    let first = json(
        &sibling,
        &["project", "--format", "json", "list", "--limit", "1"],
    );
    let cursor = first["next_cursor"].as_str().unwrap();
    assert_eq!(
        json(
            &sibling,
            &[
                "project",
                "--format",
                "json",
                "status",
                "11111111111111111111111111111111"
            ]
        ),
        created
    );
    json(
        &path,
        &[
            "project",
            "--format",
            "json",
            "create",
            "--request-id",
            "33333333333333333333333333333333",
            "--title",
            "Third",
        ],
    );
    let second = json(
        &path,
        &[
            "project", "--format", "json", "list", "--limit", "1", "--cursor", cursor,
        ],
    );
    assert_eq!(first["snapshot"], second["snapshot"]);
    assert!(second["next_cursor"].is_null());
    assert_ne!(first["projects"][0]["id"], second["projects"][0]["id"]);
}
