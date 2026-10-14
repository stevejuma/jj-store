use jj_store_read_index::{Key, Row, Store};
fn key(root: &str) -> Key {
    Key {
        namespace: "installation-A".into(),
        repository: "repo".into(),
        epoch: "epoch".into(),
        root: root.into(),
        projection: "tasks-v1".into(),
    }
}
fn rows() -> Vec<Row> {
    vec![
        Row::new("task".into(), "version-A".into(), vec![])
            .facet("state", "open")
            .text("Original body"),
    ]
}
#[test]
fn reopen_uses_exact_admitted_facts_and_changed_facts_rebuild() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::new(dir.path().into()).unwrap();
    store.prepare(&key("root-A"), rows()).unwrap();
    assert_eq!(store.stats().built, 1);
    drop(store);
    let store = Store::new(dir.path().into()).unwrap();
    assert_eq!(
        store
            .prepare(&key("root-A"), rows())
            .unwrap()
            .select(&[], "original")
            .ids,
        ["task"]
    );
    assert_eq!(store.stats().restored, 1);
    let changed = vec![
        Row::new("task".into(), "version-B".into(), vec![])
            .facet("state", "done")
            .text("Replacement body"),
    ];
    let index = store.prepare(&key("root-A"), changed).unwrap();
    assert!(index.select(&[], "original").ids.is_empty());
    assert_eq!(
        index.select(&[("state", "done".into())], "replacement").ids,
        ["task"]
    );
    assert_eq!(store.stats().built, 1);
}
#[test]
fn damaged_cache_and_failed_destination_do_not_replace_source_results() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::new(dir.path().into()).unwrap();
    store.prepare(&key("root"), rows()).unwrap();
    for file in std::fs::read_dir(dir.path()).unwrap() {
        std::fs::write(file.unwrap().path(), b"damaged").unwrap();
    }
    assert_eq!(
        store
            .prepare(&key("root"), rows())
            .unwrap()
            .select(&[], "original")
            .ids,
        ["task"]
    );
    std::fs::remove_dir_all(dir.path()).unwrap();
    assert_eq!(
        store
            .prepare(&key("next"), rows())
            .unwrap()
            .select(&[], "original")
            .ids,
        ["task"]
    );
    assert!(store.stats().failed > 0);
}
#[test]
fn namespaces_epochs_and_roots_are_independent() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::new(dir.path().into()).unwrap();
    let first = key("root-A");
    store.prepare(&first, rows()).unwrap();
    let mut other = key("root-A");
    other.epoch = "new-epoch".into();
    store.prepare(&other, vec![]).unwrap();
    other.namespace = "installation-B".into();
    store.prepare(&other, rows()).unwrap();
    assert_eq!(store.stats().built, 3);
    assert_eq!(
        store.prepare(&first, rows()).unwrap().select(&[], "").ids,
        ["task"]
    );
}
#[cfg(unix)]
#[test]
fn symlink_destination_and_unknown_files_are_not_followed_or_removed() {
    let dir = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    std::os::unix::fs::symlink(outside.path(), dir.path().join("linked")).unwrap();
    assert!(Store::new(dir.path().join("linked")).is_err());
    let file = dir.path().join("operator-data");
    std::fs::write(&file, b"retained").unwrap();
    let store = Store::new(dir.path().into()).unwrap();
    store.prepare(&key("root"), rows()).unwrap();
    assert_eq!(std::fs::read(file).unwrap(), b"retained");
}
#[test]
fn generation_retention_is_bounded_without_touching_unowned_files() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("sentinel"), b"retain").unwrap();
    let store = Store::new(dir.path().into()).unwrap();
    for root in 0..80 {
        store.prepare(&key(&root.to_string()), rows()).unwrap();
    }
    let files: Vec<_> = std::fs::read_dir(dir.path())
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "redb"))
        .collect();
    assert!(files.len() <= 64);
    assert_eq!(
        std::fs::read(dir.path().join("sentinel")).unwrap(),
        b"retain"
    );
    assert_eq!(
        store
            .prepare(&key("0"), rows())
            .unwrap()
            .select(&[], "")
            .ids,
        ["task"]
    );
}
