use jj_store_service::agent_execution::workspace_inspection::{Request, files};
use std::{collections::BTreeSet, fs};

#[test]
fn listing_includes_unchanged_and_untracked_files_but_not_ignored_or_metadata() {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir(root.path().join(".jj")).unwrap();
    fs::write(root.path().join(".jj/secret"), "private").unwrap();
    fs::write(root.path().join(".gitignore"), "ignored/\n*.tmp\n").unwrap();
    fs::create_dir(root.path().join("ignored")).unwrap();
    fs::write(root.path().join("ignored/private"), "excluded").unwrap();
    fs::write(root.path().join("tracked.tmp"), "tracked").unwrap();
    fs::write(root.path().join("new.tmp"), "ignored").unwrap();
    fs::write(root.path().join("hello.txt"), "🌍").unwrap();
    fs::write(root.path().join("global.txt"), "ignored by global rules").unwrap();
    std::os::unix::fs::symlink("/", root.path().join("outside")).unwrap();
    let entries = files::list(
        root.path(),
        &BTreeSet::from(["tracked.tmp".into()]),
        vec![b"global.txt\n".to_vec()],
    )
    .unwrap();
    assert_eq!(
        entries.iter().map(|e| e.path.as_str()).collect::<Vec<_>>(),
        vec![".gitignore", "hello.txt", "outside", "tracked.tmp"]
    );
    assert_eq!(
        entries.iter().find(|e| e.path == "outside").unwrap().kind,
        "symlink"
    );
    assert_eq!(
        fs::read_to_string(root.path().join("hello.txt")).unwrap(),
        "🌍"
    );
}

#[test]
fn inspection_request_rejects_arbitrary_roots_and_revision_expressions() {
    let value = serde_json::json!({"version":1,"session":"00000000000000000000000000000001","repository":"00000000000000000000000000000001","query":{"view":"files"},"path":null,"cursor":null});
    assert!(
        serde_json::from_value::<Request>(value.clone())
            .unwrap()
            .validate()
            .is_ok()
    );
    for path in ["../secret", "/etc/passwd", "a/.git/config", "a\\b"] {
        let mut bad = value.clone();
        bad["path"] = path.into();
        assert!(
            serde_json::from_value::<Request>(bad)
                .unwrap()
                .validate()
                .is_err()
        );
    }
    let mut bad = value;
    bad["query"] = serde_json::json!({"view":"changes","range":"branch","base":"all()"});
    assert!(
        serde_json::from_value::<Request>(bad)
            .unwrap()
            .validate()
            .is_err()
    );
}
