//! Cold executable regressions against disposable ordinary JJ repositories.
use serde_json::{Value, json};
use std::{
    fs,
    path::Path,
    process::{Command, Output},
};
fn stock(root: &Path, args: &[&str]) -> String {
    let out = Command::new("/opt/homebrew/bin/jj")
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
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}
fn cli(root: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jj-store"))
        .current_dir(root)
        .args(["--no-pager", "--color=never", "stack"])
        .args(args)
        .env("JJ_CONFIG", "/dev/null")
        .env("JJ_USER", "Fixture")
        .env("JJ_EMAIL", "fixture@invalid")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap()
}
fn ok(root: &Path, args: &[&str]) -> String {
    let out = cli(root, args);
    assert!(
        out.status.success(),
        "{args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}
fn data(root: &Path, args: &[&str]) -> Value {
    let mut prefixed = vec!["--format=json"];
    prefixed.extend_from_slice(args);
    let args = prefixed;
    serde_json::from_str(&ok(root, &args)).unwrap()
}
fn op(root: &Path) -> String {
    stock(
        root,
        &[
            "--ignore-working-copy",
            "op",
            "log",
            "--no-graph",
            "-n",
            "1",
            "-T",
            "id",
        ],
    )
}
fn fixture() -> tempfile::TempDir {
    let root = tempfile::tempdir().unwrap();
    let path = root.path();
    stock(path, &["git", "init", "--colocate"]);
    fs::write(
        path.join("file.txt"),
        (1..=40).map(|i| format!("line {i}\n")).collect::<String>(),
    )
    .unwrap();
    fs::write(path.join("base.txt"), "base\n").unwrap();
    stock(path, &["commit", "-m", "base"]);
    stock(path, &["bookmark", "create", "main", "-r", "@-"]);
    stock(path, &["new", "main", "-m", "lower change"]);
    fs::write(path.join("lower.txt"), "lower\n").unwrap();
    stock(path, &["bookmark", "create", "lower"]);
    stock(path, &["bookmark", "create", "lower-alias"]);
    stock(path, &["new", "lower", "-m", "upper change"]);
    fs::write(path.join("upper.txt"), "upper\n").unwrap();
    stock(path, &["bookmark", "create", "upper"]);
    stock(path, &["new", "main", "-m", "sibling change"]);
    fs::write(path.join("sibling.txt"), "sibling\n").unwrap();
    stock(path, &["bookmark", "create", "sibling"]);
    stock(path, &["new", "main"]);
    root
}
fn setup(root: &Path) -> Value {
    data(root, &["init", "--name", "Primary", "--target", "main"])
}

#[test]
fn stack_help_and_completions_are_discoverable_without_a_repository() {
    use clap::CommandFactory as _;
    jj_store_cli::server_commands::ServerCli::command().debug_assert();
    let root = tempfile::tempdir().unwrap();
    for args in [
        &[][..],
        &["--help"],
        &["workspace", "--help"],
        &["commit", "--help"],
        &["branch", "--help"],
        &["resolve", "--help"],
    ] {
        let text = ok(root.path(), args);
        assert!(!text.is_empty());
        assert!(!text.contains('\u{1b}'));
    }
    let out = Command::new(env!("CARGO_BIN_EXE_jj-store"))
        .args(["completion", "zsh"])
        .output()
        .unwrap();
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("stack"));
}
#[test]
fn inspection_dry_run_and_invalid_arguments_preserve_native_state() {
    let root = fixture();
    let p = root.path();
    fs::write(p.join("dirty.txt"), "unrecorded\n").unwrap();
    let before = op(p);
    let status = data(p, &["status", "--no-snapshot"]);
    assert!(status["state"]["workspaces"].as_array().unwrap().is_empty());
    assert!(
        !status["paths"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v == "dirty.txt")
    );
    let list = data(p, &["list", "--target", "main", "--no-snapshot"]);
    assert_eq!(list["data"]["topology"]["complete"], true);
    let shown = data(p, &["show", "upper", "--target", "main", "--no-snapshot"]);
    assert_eq!(shown["data"]["pages"].as_array().unwrap().len(), 2);
    let preview = data(
        p,
        &["init", "--name", "Primary", "--target", "main", "--dry-run"],
    );
    assert_eq!(preview["dry_run"], true);
    for args in [
        &["discard", "--all"][..],
        &["commit", "--branch", "upper", "-m", "missing selection"],
        &["workspace", "update"],
        &["empty", "-m", "bad"],
    ] {
        let result = cli(p, args);
        assert_eq!(
            result.status.code(),
            Some(2),
            "{args:?}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
    assert_eq!(op(p), before);
    assert!(!p.join(".jj/jj-store").exists());
    assert_eq!(
        fs::read_to_string(p.join("dirty.txt")).unwrap(),
        "unrecorded\n"
    );
}
#[test]
fn workspace_membership_and_settings_keep_native_merge_wip_and_checkout() {
    let root = fixture();
    let p = root.path();
    fs::write(p.join("draft.txt"), "draft\n").unwrap();
    let first = setup(p);
    let merge = first["workspaces"][0]["merge_commit"].as_str().unwrap();
    let merge_change = stock(p, &["log", "-r", merge, "--no-graph", "-T", "change_id"]);
    let applied = data(p, &["apply", "upper"]);
    let current = &applied["workspaces"][0];
    assert_eq!(
        stock(
            p,
            &[
                "log",
                "-r",
                current["merge_commit"].as_str().unwrap(),
                "--no-graph",
                "-T",
                "change_id"
            ]
        ),
        merge_change
    );
    assert_eq!(
        stock(p, &["log", "-r", "@-", "--no-graph", "-T", "parents.len()"]),
        "2"
    );
    assert!(p.join("upper.txt").exists());
    assert_eq!(fs::read_to_string(p.join("draft.txt")).unwrap(), "draft\n");
    let updated = data(p, &["workspace", "update", "--name", "Renamed"]);
    assert_eq!(updated["workspaces"][0]["name"], "Renamed");
    data(p, &["workspace", "create", "Second"]);
    assert!(!p.join("draft.txt").exists());
    data(p, &["workspace", "switch", "Renamed"]);
    assert!(p.join("draft.txt").exists());
    let out = cli(p, &["workspace", "delete", "Second"]);
    assert_eq!(out.status.code(), Some(2));
    let deleted = data(p, &["workspace", "delete", "Second", "--yes"]);
    assert_eq!(deleted["workspaces"].as_array().unwrap().len(), 1);
    data(p, &["unapply", "upper"]);
    assert!(!p.join("upper.txt").exists());
    assert!(p.join("draft.txt").exists());
    assert!(
        stock(
            p,
            &["op", "log", "-n", "3", "--no-graph", "-T", "description"]
        )
        .contains("Stacks: Unapply upper")
    );
}
#[test]
fn topology_cards_aliases_partial_application_and_native_history_agree() {
    let root = fixture();
    let p = root.path();
    setup(p);
    data(p, &["apply", "lower"]);
    assert!(ok(p, &["list"]).contains("Partially applied"));
    let list = data(p, &["list"]);
    let lanes = list["data"]["topology"]["inventory"].as_array().unwrap();
    assert_eq!(lanes.len(), 2);
    let stack = lanes
        .iter()
        .find(|v| {
            v["heads"][0]["names"]
                .as_array()
                .unwrap()
                .iter()
                .any(|n| n == "upper")
        })
        .unwrap();
    assert_eq!(stack["heads"].as_array().unwrap().len(), 2);
    assert!(
        stack["heads"][1]["names"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v == "lower-alias")
    );
    data(p, &["apply", "upper"]);
    let shown = data(p, &["show", "upper"]);
    assert_eq!(shown["data"]["pages"].as_array().unwrap().len(), 2);
    let serialized = serde_json::to_string(&shown).unwrap();
    assert!(serialized.contains("upper change"));
    assert!(serialized.contains("lower change"));
    let text = ok(p, &["show", "upper"]);
    assert!(text.contains("upper change"));
    assert!(text.contains("lower change"));
    let result = data(p, &["unapply", "upper"]);
    assert!(
        result["workspaces"][0]["applied"]
            .as_array()
            .unwrap()
            .is_empty()
    );
}
#[test]
fn selected_files_and_hunks_preserve_unselected_work_and_reject_stale_input() {
    let root = fixture();
    let p = root.path();
    setup(p);
    data(p, &["apply", "upper"]);
    let content = fs::read_to_string(p.join("file.txt"))
        .unwrap()
        .replace("line 2\n", "edited 2\n")
        .replace("line 35\n", "edited 35\n");
    fs::write(p.join("file.txt"), content).unwrap();
    fs::write(p.join("keep.txt"), "keep\n").unwrap();
    let hunks = data(p, &["hunks", "file.txt"]);
    assert_eq!(hunks["data"]["hunks"].as_array().unwrap().len(), 2);
    let h = hunks["data"]["hunks"][0]["id"].as_str().unwrap();
    let operation = hunks["operation"].as_str().unwrap();
    let revision = hunks["revision"].to_string();
    data(
        p,
        &[
            "commit",
            "--branch",
            "upper",
            "-m",
            "one hunk",
            "--expect-operation",
            operation,
            "--expect-revision",
            &revision,
            "--hunk",
            h,
            "--",
            "file.txt",
        ],
    );
    let status = data(p, &["status"]);
    assert!(
        status["paths"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v == "file.txt")
    );
    assert!(
        status["paths"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v == "keep.txt")
    );
    let stale = cli(
        p,
        &[
            "commit",
            "--branch",
            "upper",
            "-m",
            "stale",
            "--expect-operation",
            operation,
            "--expect-revision",
            &revision,
            "--hunk",
            h,
            "--",
            "file.txt",
        ],
    );
    assert_eq!(stale.status.code(), Some(1));
    data(
        p,
        &[
            "commit", "--branch", "upper", "-m", "rest", "--", "file.txt",
        ],
    );
    assert!(
        !data(p, &["status"])["paths"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v == "file.txt")
    );
    let pin = data(p, &["status"]);
    let manifest = json!({"version":1,"operation":pin["operation"],"revision":pin["revision"],"selections":[{"source":pin["state"]["workspaces"][0]["wip_commit"],"path":"keep.txt"}]});
    fs::write(
        p.join(".jj/selection.json"),
        serde_json::to_vec(&manifest).unwrap(),
    )
    .unwrap();
    data(
        p,
        &[
            "transfer",
            "--to",
            "upper",
            "--selection-file",
            ".jj/selection.json",
        ],
    );
    assert!(data(p, &["status"])["paths"].as_array().unwrap().is_empty());
}
#[test]
fn graph_branch_and_content_actions_use_native_rewrites() {
    let root = fixture();
    let p = root.path();
    setup(p);
    data(p, &["apply", "upper"]);
    data(p, &["create", "new-stack", "--parent", "upper"]);
    data(p, &["branch", "rename", "new-stack", "renamed"]);
    data(
        p,
        &[
            "commit",
            "--branch",
            "renamed",
            "-m",
            "empty checkpoint",
            "--empty",
        ],
    );
    data(p, &["reword", "renamed", "-m", "checkpoint renamed"]);
    assert_eq!(
        stock(
            p,
            &["log", "-r", "renamed", "--no-graph", "-T", "description"]
        ),
        "checkpoint renamed"
    );
    data(p, &["empty", "--above", "renamed", "-m", "inserted"]);
    data(p, &["branch", "create", "label", "--above", "renamed"]);
    data(p, &["branch", "delete", "label", "--yes"]);
    data(p, &["squash", "renamed", "--into", "renamed-"]);
    data(
        p,
        &["empty", "--below", "renamed", "-m", "lower checkpoint"],
    );
    data(p, &["squash-branch", "renamed"]);
    data(p, &["rebase", "renamed"]);
    fs::write(p.join("split.txt"), "split\n").unwrap();
    data(
        p,
        &[
            "commit",
            "--branch",
            "renamed",
            "-m",
            "split source",
            "--",
            "split.txt",
        ],
    );
    data(p, &["split", "renamed", "-m", "extracted", "--all"]);
    data(p, &["edit", "renamed", "-m", "edited"]);
    data(p, &["uncommit", "renamed"]);
    data(p, &["discard", "--all", "--yes"]);
}
#[test]
fn linked_workspace_and_relative_paths_use_its_own_local_configuration() {
    let root = fixture();
    let linked = tempfile::tempdir().unwrap();
    let p = linked.path().join("linked");
    stock(
        root.path(),
        &["workspace", "add", p.to_str().unwrap(), "--name", "linked"],
    );
    setup(&p);
    data(&p, &["apply", "upper"]);
    fs::create_dir(p.join("src")).unwrap();
    fs::write(p.join("src/new.txt"), "new\n").unwrap();
    data(
        &p,
        &[
            "-C", "src", "commit", "--branch", "upper", "-m", "relative", "--", "new.txt",
        ],
    );
    assert!(p.join(".jj/jj-store/stacks.toml").exists());
    assert!(!root.path().join(".jj/jj-store/stacks.toml").exists());
}

#[test]
fn grouped_unapply_preserves_shared_explicit_members() {
    let root = fixture();
    let p = root.path();
    stock(p, &["new", "lower", "-m", "another upper"]);
    fs::write(p.join("another.txt"), "another\n").unwrap();
    stock(p, &["bookmark", "create", "another"]);
    stock(p, &["new", "main"]);
    setup(p);
    for branch in ["lower", "upper", "another"] {
        data(p, &["apply", branch]);
    }
    let result = data(p, &["unapply", "upper"]);
    let members = result["workspaces"][0]["applied"].as_array().unwrap();
    assert!(members.iter().any(|v| v == "lower"));
    assert!(members.iter().any(|v| v == "another"));
    assert!(!members.iter().any(|v| v == "upper"));
    assert!(p.join("lower.txt").exists());
    assert!(p.join("another.txt").exists());
    assert!(!p.join("upper.txt").exists());
    data(p, &["unapply", "lower", "--branch-only"]);
}

#[test]
fn text_conflict_resolution_and_native_undo_redo_retain_work() {
    let root = fixture();
    let p = root.path();
    for (name, content) in [("left", "left\n"), ("right", "right\n")] {
        stock(p, &["new", "main", "-m", name]);
        fs::write(p.join("base.txt"), content).unwrap();
        stock(p, &["bookmark", "create", name]);
    }
    stock(p, &["new", "main"]);
    setup(p);
    data(p, &["apply", "left"]);
    data(p, &["apply", "right"]);
    let conflict = data(p, &["conflicts", "base.txt"]);
    assert!(conflict["data"]["terms"].as_array().unwrap().len() >= 3);
    let preview = cli(
        p,
        &["resolve", "base.txt", "--file", "missing", "--dry-run"],
    );
    assert!(!preview.status.success());
    fs::write(p.join(".jj/resolution.txt"), "reviewed\n").unwrap();
    let result = data(p, &["resolve", "base.txt", "--file", ".jj/resolution.txt"]);
    assert!(result["conflicts"].as_array().unwrap().is_empty());
    assert_eq!(
        fs::read_to_string(p.join("base.txt")).unwrap(),
        "reviewed\n"
    );
    let undone = data(p, &["undo"]);
    assert!(
        undone["conflicts"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v == "base.txt")
    );
    data(p, &["redo"]);
    assert_eq!(
        fs::read_to_string(p.join("base.txt")).unwrap(),
        "reviewed\n"
    );
}

#[test]
fn concurrent_heads_and_ambiguous_selectors_do_not_reconcile_during_inspection() {
    let root = fixture();
    let p = root.path();
    let before = op(p);
    stock(p, &["bookmark", "create", "first-concurrent", "-r", "main"]);
    stock(
        p,
        &[
            "--at-op",
            &before,
            "bookmark",
            "create",
            "second-concurrent",
            "-r",
            "main",
        ],
    );
    let heads = || {
        let mut names = fs::read_dir(p.join(".jj/repo/op_heads"))
            .unwrap()
            .map(|e| e.unwrap().file_name())
            .collect::<Vec<_>>();
        names.sort();
        names
    };
    let initial = heads();
    let out = cli(p, &["status", "--no-snapshot"]);
    assert!(!out.status.success());
    assert_eq!(heads(), initial);
    let out = cli(
        p,
        &[
            "init",
            "--name",
            "No mutation",
            "--target",
            "main",
            "--dry-run",
        ],
    );
    assert!(!out.status.success());
    assert_eq!(heads(), initial);
    assert!(!p.join(".jj/jj-store").exists());
    stock(p, &["status"]);
    setup(p);
    let state = data(p, &["status"]);
    let op = state["operation"].as_str().unwrap();
    let invalid = cli(p, &["reword", "all()", "-m", "must not apply", "--dry-run"]);
    assert!(!invalid.status.success());
    assert_eq!(data(p, &["status", "--no-snapshot"])["operation"], op);
}

#[test]
fn movement_and_abandonment_preserve_other_applied_branches() {
    let root = fixture();
    let p = root.path();
    setup(p);
    data(p, &["apply", "upper"]);
    data(p, &["apply", "sibling"]);
    data(p, &["move", "upper", "--above", "sibling"]);
    assert_eq!(
        stock(p, &["log", "-r", "upper-", "--no-graph", "-T", "commit_id"]),
        stock(
            p,
            &["log", "-r", "sibling", "--no-graph", "-T", "commit_id"]
        )
    );
    data(p, &["move", "upper", "--below", "sibling"]);
    assert_eq!(
        stock(
            p,
            &["log", "-r", "sibling-", "--no-graph", "-T", "commit_id"]
        ),
        stock(p, &["log", "-r", "upper", "--no-graph", "-T", "commit_id"])
    );
    data(p, &["empty", "--below", "sibling", "-m", "abandonable"]);
    data(
        p,
        &["abandon", "description(exact:\"abandonable\")", "--yes"],
    );
    assert!(p.join("sibling.txt").exists());
    assert!(p.join("upper.txt").exists());
}

#[test]
fn diff_continuations_integrated_choices_and_reconciliation_are_exact() {
    let root = fixture();
    let p = root.path();
    stock(p, &["bookmark", "create", "integrated", "-r", "main-"]);
    stock(p, &["bookmark", "create", "empty", "-r", "main"]);
    stock(p, &["new", "main"]);
    for index in 0..4 {
        stock(p, &["describe", "-m", &format!("many {index}")]);
        fs::write(p.join(format!("many-{index}.txt")), "page\n").unwrap();
        stock(p, &["commit", "-m", &format!("many {index}")]);
    }
    stock(p, &["bookmark", "create", "many", "-r", "@-"]);
    stock(p, &["new", "main"]);
    setup(p);
    let all = data(p, &["list", "--all"]);
    assert!(
        all["data"]["topology"]["integrated"]
            .as_array()
            .unwrap()
            .iter()
            .any(|n| n == "integrated")
    );
    assert!(ok(p, &["list"]).contains("empty"));
    data(p, &["apply", "many"]);
    let page = data(p, &["show", "many", "--limit", "1"]);
    let next = page["data"]["pages"][0]["page"]["data"]["next"]
        .as_str()
        .unwrap();
    let continued = data(
        p,
        &[
            "show",
            "many",
            "--limit",
            "1",
            "--card",
            "many",
            "--after",
            next,
            "--expect-operation",
            page["operation"].as_str().unwrap(),
            "--expect-revision",
            &page["revision"].to_string(),
        ],
    );
    assert_ne!(
        continued["data"]["pages"][0]["page"]["data"]["commits"],
        page["data"]["pages"][0]["page"]["data"]["commits"]
    );
    fs::write(p.join("a.txt"), "a\n").unwrap();
    fs::write(p.join("b.txt"), "b\n").unwrap();
    let diff = data(p, &["diff", "--limit", "1"]);
    assert_eq!(diff["data"]["files"].as_array().unwrap().len(), 1);
    assert!(
        !diff["data"]["files"][0]["hunks"]["hunks"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    let anchor = diff["data"]["page"]["next"].as_str().unwrap();
    let continued = data(
        p,
        &["diff", "--limit", "1", "--after", anchor, "--no-snapshot"],
    );
    assert_eq!(continued["data"]["files"][0]["change"]["path"], "b.txt");
    let net = data(p, &["diff", "--branch", "many"]);
    assert_eq!(net["data"]["net_comparison"], true);
    let before = op(p);
    data(p, &["reconcile", "--dry-run"]);
    assert_eq!(op(p), before);
    data(p, &["reconcile", "--yes"]);
    data(p, &["workspace", "update", "--target", "sibling"]);
    let result = data(p, &["refresh"]);
    assert_eq!(result["workspaces"][0]["target"], "sibling");
    assert_eq!(fs::read_to_string(p.join("a.txt")).unwrap(), "a\n");
}
