//! Fresh executable processes over disposable Git-backed JJ repositories.
use jj_collab::{CollabObjectRef, HumanMessageId, ThreadActionV1, ThreadCommandV1, ThreadId};
use jj_store_model as m;
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

fn run(path: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jj"))
        .args([
            "--no-pager",
            "--config",
            "user.name='Collaboration Test'",
            "--config",
            "user.email='collab@example.invalid'",
        ])
        .args(args)
        .current_dir(path)
        .env("JJ_CONFIG", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("PATH", "/usr/bin:/bin")
        .output()
        .unwrap()
}
fn success(path: &Path, args: &[&str]) -> String {
    let out = run(path, args);
    assert!(
        out.status.success(),
        "{args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}
fn cli(path: &Path, device: &Path, args: &[&str]) -> Output {
    let mut all = vec![
        "agent",
        "--format",
        "json",
        "--state-dir",
        device.to_str().unwrap(),
    ];
    all.extend_from_slice(args);
    run(path, &all)
}
fn output(path: &Path, device: &Path, args: &[&str]) -> Value {
    let out = cli(path, device, args);
    assert!(
        out.status.success(),
        "{args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).unwrap()
}
fn reject(path: &Path, device: &Path, args: &[&str]) {
    let out = cli(path, device, args);
    assert!(
        !out.status.success(),
        "accepted {args:?}: {}",
        String::from_utf8_lossy(&out.stdout)
    );
}
fn init(root: &Path, layout: &str) -> PathBuf {
    let path = root.join(layout);
    match layout {
        "colocated" => {
            success(root, &["git", "init", "--colocate", path.to_str().unwrap()]);
        }
        "internal" => {
            success(root, &["git", "init", path.to_str().unwrap()]);
        }
        "external" => {
            success(root, &["git", "init", "--colocate", "source"]);
            success(
                root,
                &[
                    "git",
                    "init",
                    "--git-repo",
                    "source",
                    path.to_str().unwrap(),
                ],
            );
        }
        _ => unreachable!(),
    }
    path
}
fn id(n: u8) -> String {
    m::RequestId::new([n; 16]).to_string()
}
fn document(device: &Path, name: &str, value: &impl serde::Serialize) -> PathBuf {
    let path = device.join(name);
    fs::create_dir_all(device).unwrap();
    fs::write(&path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
    path
}
fn create(path: &Path, device: &Path, n: u8) -> Value {
    output(
        path,
        device,
        &[
            "thread",
            "create",
            "--request-id",
            &id(n),
            "--thread-id",
            &id(n),
            "--title",
            &format!("Thread {n}"),
        ],
    )
}
fn apply(path: &Path, device: &Path, n: u8, intent: &ThreadCommandV1) -> Value {
    let doc = document(device, "intent.json", intent);
    output(
        path,
        device,
        &[
            "thread",
            "apply",
            "--request-id",
            &id(n),
            "--intent",
            doc.to_str().unwrap(),
        ],
    )
}
fn reference(value: &Value, name: &str) -> CollabObjectRef {
    serde_json::from_value(value["receipt"]["result"]["result"][name].clone()).unwrap()
}
fn heads(path: &Path) -> Vec<String> {
    let mut values = fs::read_dir(path.join(".jj/repo/op_heads"))
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    values.sort();
    values
}
fn query(path: &Path, device: &Path, target: &Path) -> Value {
    output(
        path,
        device,
        &["reaction", "query", "--target", target.to_str().unwrap()],
    )
}
fn set(
    path: &Path,
    device: &Path,
    n: u8,
    target: &Path,
    value: &Path,
    observed: Option<&Path>,
) -> Value {
    let request = id(n);
    let mut args = vec![
        "reaction",
        "set",
        "--request-id",
        &request,
        "--target",
        target.to_str().unwrap(),
        "--value",
        value.to_str().unwrap(),
    ];
    if let Some(observed) = observed {
        args.extend(["--observed", observed.to_str().unwrap()]);
    }
    output(path, device, &args)
}

#[test]
fn cold_reads_and_receipts_support_all_git_layouts_without_agent_setup_or_code_changes() {
    for layout in ["colocated", "internal", "external"] {
        let root = tempfile::tempdir().unwrap();
        let path = init(root.path(), layout);
        let device = root.path().join("device");
        let initial_heads = heads(&path);
        let list = output(&path, &device, &["thread", "list"]);
        assert!(list["snapshot"]["metadata_commit"].is_null());
        assert!(list["threads"].as_array().unwrap().is_empty());
        assert!(
            output(&path, &device, &["reaction", "emojis"])["definitions"]
                .as_array()
                .unwrap()
                .is_empty()
        );
        assert_eq!(
            output(&path, &device, &["thread", "status", &id(1)])["status"],
            "not_known"
        );
        assert_eq!(
            output(&path, &device, &["reaction", "status", &id(1)])["status"],
            "not_known"
        );
        assert!(!path.join(".jj/repo/lapis").exists());
        assert!(!device.exists());
        fs::write(path.join("unsnapshotted.txt"), b"user edit").unwrap();
        let created = create(&path, &device, 1);
        assert_eq!(created["status"], "committed");
        assert_eq!(create(&path, &device, 1), created);
        assert_eq!(
            output(&path, &device, &["thread", "status", &id(1)]),
            created
        );
        assert_eq!(
            output(&path, &device, &["thread", "show", &id(1)])["reference"],
            created["receipt"]["result"]["result"]["version"]
        );
        reject(
            &path,
            &device,
            &[
                "thread",
                "create",
                "--request-id",
                &id(1),
                "--thread-id",
                &id(1),
                "--title",
                "Altered intent",
            ],
        );
        assert_eq!(heads(&path), initial_heads);
        assert_eq!(
            fs::read(path.join("unsnapshotted.txt")).unwrap(),
            b"user edit"
        );
        assert!(!device.exists());
        let sibling = root.path().join("sibling");
        success(
            &path,
            &[
                "workspace",
                "add",
                "--name",
                "sibling",
                sibling.to_str().unwrap(),
            ],
        );
        assert_eq!(
            output(&sibling, &device, &["thread", "status", &id(1)]),
            created
        );
        assert_eq!(
            output(&sibling, &device, &["thread", "show", &id(1)])["value"]["title"],
            "Thread 1"
        );
    }
}

#[test]
fn cold_message_revisions_keep_feedback_exact_and_replacements_require_observed_records() {
    let root = tempfile::tempdir().unwrap();
    let path = init(root.path(), "colocated");
    let device = root.path().join("device");
    let created = create(&path, &device, 1);
    let intent = ThreadCommandV1::Message {
        thread_id: ThreadId::new([1; 16]),
        expected_thread: reference(&created, "version"),
        message_id: HumanMessageId::new([2; 16]),
        expected_revision: None,
        body: Some("before".into()),
    };
    let input = document(&device, "message.json", &intent);
    let first = output(
        &path,
        &device,
        &[
            "thread",
            "message",
            "--request-id",
            &id(2),
            "--intent",
            input.to_str().unwrap(),
        ],
    );
    let target = document(
        &device,
        "target.json",
        &json!({"type":"human_message", "thread_id":id(1), "message_id":id(2), "revision":reference(&first, "message")}),
    );
    let value = document(
        &device,
        "value.json",
        &json!({"type":"emoji", "sequence":"👩🏽‍💻"}),
    );
    let reacted = set(&path, &device, 3, &target, &value, None);
    let page = query(&path, &device, &target);
    assert_eq!(page["rows"][0]["actor"], reacted["receipt"]["actor"]);
    assert_eq!(page["rows"][0]["value"]["sequence"], "👩🏽‍💻");
    let observed = document(&device, "observed.json", &page["rows"][0]["active"]);
    let edit = apply(
        &path,
        &device,
        4,
        &ThreadCommandV1::Message {
            thread_id: ThreadId::new([1; 16]),
            expected_thread: reference(&first, "version"),
            message_id: HumanMessageId::new([2; 16]),
            expected_revision: Some(reference(&first, "message")),
            body: Some("after".into()),
        },
    );
    let current = document(
        &device,
        "current.json",
        &json!({"type":"human_message", "thread_id":id(1), "message_id":id(2), "revision":reference(&edit,"message")}),
    );
    assert!(
        query(&path, &device, &current)["rows"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert_eq!(query(&path, &device, &target)["revision"], "earlier");
    let like = document(&device, "like.json", &json!({"type":"like"}));
    reject(
        &path,
        &device,
        &[
            "reaction",
            "set",
            "--request-id",
            &id(5),
            "--target",
            target.to_str().unwrap(),
            "--value",
            like.to_str().unwrap(),
        ],
    );
    set(&path, &device, 6, &target, &like, Some(&observed));
    reject(
        &path,
        &device,
        &[
            "reaction",
            "clear",
            "--request-id",
            &id(7),
            "--target",
            target.to_str().unwrap(),
            "--observed",
            observed.to_str().unwrap(),
        ],
    );
    let next = query(&path, &device, &target);
    document(&device, "observed.json", &next["rows"][0]["active"]);
    output(
        &path,
        &device,
        &[
            "reaction",
            "clear",
            "--request-id",
            &id(8),
            "--target",
            target.to_str().unwrap(),
            "--observed",
            observed.to_str().unwrap(),
        ],
    );
    assert!(
        query(&path, &device, &target)["rows"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert_eq!(set(&path, &device, 3, &target, &value, None), reacted);
    assert_eq!(
        output(&path, &device, &["reaction", "status", &id(3)]),
        reacted
    );
}

#[test]
fn lifecycle_and_thread_continuations_fail_on_stale_snapshots_and_hide_tombstones() {
    let root = tempfile::tempdir().unwrap();
    let path = init(root.path(), "colocated");
    let device = root.path().join("device");
    let created = create(&path, &device, 1);
    create(&path, &device, 2);
    let page = output(&path, &device, &["thread", "list", "--limit", "1"]);
    let snapshot = page["snapshot"]["metadata_commit"].as_str().unwrap();
    let after = page["next_id"].as_str().unwrap();
    reject(
        &path,
        &device,
        &["thread", "list", "--limit", "1", "--after", after],
    );
    let next = output(
        &path,
        &device,
        &[
            "thread",
            "list",
            "--limit",
            "1",
            "--after",
            after,
            "--at-metadata",
            snapshot,
        ],
    );
    assert_eq!(next["threads"].as_array().unwrap().len(), 1);
    assert_eq!(next["snapshot"], page["snapshot"]);
    let mut version = reference(&created, "version");
    for (n, action) in [
        (3, ThreadActionV1::Activate),
        (4, ThreadActionV1::Archive),
        (5, ThreadActionV1::Restore),
        (6, ThreadActionV1::Tombstone),
    ] {
        let result = apply(
            &path,
            &device,
            n,
            &ThreadCommandV1::Lifecycle {
                thread_id: ThreadId::new([1; 16]),
                expected: version,
                action,
            },
        );
        version = reference(&result, "version");
    }
    reject(
        &path,
        &device,
        &[
            "thread",
            "list",
            "--limit",
            "1",
            "--after",
            after,
            "--at-metadata",
            snapshot,
        ],
    );
    reject(&path, &device, &["thread", "show", &id(1)]);
    assert_eq!(
        output(&path, &device, &["thread", "list"])["threads"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    apply(
        &path,
        &device,
        7,
        &ThreadCommandV1::Lifecycle {
            thread_id: ThreadId::new([1; 16]),
            expected: version,
            action: ThreadActionV1::Restore,
        },
    );
    assert_eq!(
        output(&path, &device, &["thread", "show", &id(1)])["value"]["status"],
        "idle"
    );
    assert_eq!(create(&path, &device, 1), created);
}

#[test]
fn custom_emoji_retirement_keeps_exact_old_feedback_and_bounded_library_pages() {
    let root = tempfile::tempdir().unwrap();
    let path = init(root.path(), "colocated");
    let device = root.path().join("device");
    create(&path, &device, 1);
    fs::create_dir_all(&device).unwrap();
    let asset = device.join("emoji.png");
    // One retained 1x1 RGBA PNG; no image or network dependency in this executable fixture.
    fs::write(&asset,hex::decode("89504e470d0a1a0a0000000d49484452000000010000000108060000001f15c4890000000d49444154789c63f8cfc0f01f00050001ff89993d1d0000000049454e44ae426082").unwrap()).unwrap();
    let define = |n: u8| {
        output(
            &path,
            &device,
            &[
                "reaction",
                "define-emoji",
                "--request-id",
                &id(n),
                "--emoji-id",
                &id(n),
                "--shortcode",
                &format!("emoji_{n}"),
                "--label",
                "Ship it",
                "--asset",
                asset.to_str().unwrap(),
            ],
        )
    };
    let first = define(2);
    define(3);
    let definition = &first["receipt"]["result"]["version"];
    let target = document(
        &device,
        "target.json",
        &json!({"type":"thread","thread_id":id(1)}),
    );
    let value = document(
        &device,
        "custom.json",
        &json!({"type":"custom","emoji_id":id(2),"version":definition}),
    );
    let reacted = set(&path, &device, 4, &target, &value, None);
    let page = output(&path, &device, &["reaction", "emojis", "--limit", "1"]);
    let after = page["next_id"].as_str().unwrap();
    let snapshot = page["snapshot"]["metadata_commit"].as_str().unwrap();
    reject(
        &path,
        &device,
        &["reaction", "emojis", "--limit", "1", "--after", after],
    );
    let next = output(
        &path,
        &device,
        &[
            "reaction",
            "emojis",
            "--limit",
            "1",
            "--after",
            after,
            "--at-metadata",
            snapshot,
        ],
    );
    assert_eq!(page["snapshot"], next["snapshot"]);
    assert!(next["next_id"].is_null());
    output(
        &path,
        &device,
        &[
            "reaction",
            "retire-emoji",
            "--request-id",
            &id(5),
            "--emoji-id",
            &id(2),
            "--expected",
            definition["id"].as_str().unwrap(),
        ],
    );
    assert_eq!(
        query(&path, &device, &target)["rows"][0]["value"]["version"],
        *definition
    );
    let observed = document(
        &device,
        "observed.json",
        &query(&path, &device, &target)["rows"][0]["active"],
    );
    reject(
        &path,
        &device,
        &[
            "reaction",
            "set",
            "--request-id",
            &id(6),
            "--target",
            target.to_str().unwrap(),
            "--value",
            value.to_str().unwrap(),
            "--observed",
            observed.to_str().unwrap(),
        ],
    );
    assert_eq!(set(&path, &device, 4, &target, &value, None), reacted);
    reject(
        &path,
        &device,
        &[
            "reaction",
            "emojis",
            "--limit",
            "1",
            "--after",
            after,
            "--at-metadata",
            snapshot,
        ],
    );
    let retired = output(&path, &device, &["reaction", "emojis"]);
    assert_eq!(retired["definitions"][0]["retired"], true);
    assert_eq!(
        retired["definitions"][0]["preview"],
        page["definitions"][0]["preview"]
    );
}

#[test]
fn unknown_publication_keeps_original_identity_and_retries_in_a_new_process() {
    let root = tempfile::tempdir().unwrap();
    let path = init(root.path(), "colocated");
    let device = root.path().join("device");
    create(&path, &device, 1);
    let before = output(&path, &device, &["thread", "list"])["snapshot"].clone();
    let lock = path.join(".git/refs/lapis/state/v1.lock");
    fs::write(&lock, b"fixture lock").unwrap();
    let unknown = create(&path, &device, 2);
    assert_eq!(unknown["status"], "outcome_unknown");
    assert_eq!(unknown["request_id"], id(2));
    assert_eq!(
        output(&path, &device, &["thread", "list"])["snapshot"],
        before
    );
    assert_eq!(
        output(&path, &device, &["thread", "status", &id(2)])["status"],
        "not_known"
    );
    fs::remove_file(lock).unwrap();
    let committed = create(&path, &device, 2);
    assert_eq!(create(&path, &device, 2), committed);
    assert_eq!(
        output(&path, &device, &["thread", "list"])["threads"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
}

#[test]
fn malformed_actor_fields_targets_and_oversized_documents_fail_without_publication() {
    let root = tempfile::tempdir().unwrap();
    let path = init(root.path(), "colocated");
    let device = root.path().join("device");
    let created = create(&path, &device, 1);
    let before = output(&path, &device, &["thread", "list"])["snapshot"].clone();
    let target = document(
        &device,
        "target.json",
        &json!({"type":"thread","thread_id":id(1),"actor":id(9)}),
    );
    let value = document(&device, "value.json", &json!({"type":"like"}));
    reject(
        &path,
        &device,
        &[
            "reaction",
            "set",
            "--request-id",
            &id(2),
            "--target",
            target.to_str().unwrap(),
            "--value",
            value.to_str().unwrap(),
        ],
    );
    let malformed = document(
        &device,
        "intent.json",
        &json!({"type":"rename","thread_id":id(1),"expected":reference(&created,"version"),"title":"New", "actor":id(9)}),
    );
    reject(
        &path,
        &device,
        &[
            "thread",
            "apply",
            "--request-id",
            &id(3),
            "--intent",
            malformed.to_str().unwrap(),
        ],
    );
    document(
        &device,
        "intent.json",
        &ThreadCommandV1::Rename {
            thread_id: ThreadId::new([1; 16]),
            expected: reference(&created, "version"),
            title: "New".into(),
        },
    );
    reject(
        &path,
        &device,
        &[
            "thread",
            "message",
            "--request-id",
            &id(3),
            "--intent",
            malformed.to_str().unwrap(),
        ],
    );
    let file = fs::File::create(&malformed).unwrap();
    file.set_len(16 * 1024 * 1024 + 1).unwrap();
    let out = cli(
        &path,
        &device,
        &[
            "thread",
            "apply",
            "--request-id",
            &id(4),
            "--intent",
            malformed.to_str().unwrap(),
        ],
    );
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("byte limit"));
    document(
        &device,
        "target.json",
        &json!({"type":"thread","thread_id":id(1)}),
    );
    reject(
        &path,
        &device,
        &[
            "reaction",
            "query",
            "--target",
            target.to_str().unwrap(),
            "--after",
            &id(9),
        ],
    );
    reject(&path, &device, &["thread", "list", "--limit", "257"]);
    reject(
        &path,
        &device,
        &["thread", "list", "--cursor", "native-token"],
    );
    reject(
        &path,
        &device,
        &["reaction", "emojis", "--cursor", "native-token"],
    );
    reject(&path, &device, &["reaction", "emojis", "--limit", "0"]);
    assert_eq!(
        output(&path, &device, &["thread", "list"])["snapshot"],
        before
    );
    assert!(!device.join("registry.json").exists());
}
