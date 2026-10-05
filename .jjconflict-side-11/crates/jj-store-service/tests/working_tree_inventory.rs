#[path = "working_tree_inventory/adversarial.rs"]
mod adversarial;
#[path = "working_tree_inventory/guard.rs"]
mod guard;
use jj_store_jj::local_execution::{PhysicalDirectoryV1, PhysicalFileV1, PhysicalInventoryV1};
use jj_store_model::RepositoryId;
use jj_store_service::working_tree as tree;
#[test]
fn original_inventory_roundtrips_the_complete_scanner_fingerprint() {
    let root = tempfile::tempdir().unwrap();
    std::fs::create_dir(root.path().join(".jj")).unwrap();
    std::fs::create_dir_all(root.path().join("src/empty")).unwrap();
    std::fs::write(root.path().join("src/file"), "unchanged\n").unwrap();
    let physical = tree::scan(
        root.path(),
        RepositoryId::new([1; 16]),
        7,
        tree::Limits::default(),
        |_| Ok(()),
    )
    .unwrap();
    let inventory = physical.inventory().unwrap();
    assert_eq!(inventory.fingerprint(), physical.fingerprint);
    assert_eq!(inventory.files().count(), 1);
    assert_eq!(inventory.directories().count(), 3);
    let bytes = inventory.encode().unwrap();
    assert_eq!(
        jj_store_jj::local_execution::PhysicalInventoryV1::decode(inventory.id().unwrap(), &bytes)
            .unwrap(),
        inventory
    );
}

fn fixture() -> tempfile::TempDir {
    let root = tempfile::tempdir().unwrap();
    std::fs::create_dir(root.path().join(".jj")).unwrap();
    std::fs::create_dir_all(root.path().join("src/empty")).unwrap();
    std::fs::write(root.path().join("src/file"), "unchanged\n").unwrap();
    root
}
fn scan(root: &std::path::Path) -> tree::WorkingTree {
    tree::scan(
        root,
        RepositoryId::new([1; 16]),
        7,
        tree::Limits::default(),
        |_| Ok(()),
    )
    .unwrap()
}
#[test]
fn inventory_retains_ignored_symlink_binary_and_unicode_rows_without_content() {
    use std::os::unix::fs::{PermissionsExt as _, symlink};
    let root = fixture();
    std::fs::write(root.path().join(".gitignore"), "ignored\n").unwrap();
    std::fs::write(root.path().join("ignored"), [0, 255, 3]).unwrap();
    std::fs::write(root.path().join("src/文件"), "unicode\n").unwrap();
    std::fs::set_permissions(
        root.path().join("ignored"),
        std::fs::Permissions::from_mode(0o755),
    )
    .unwrap();
    symlink("/not-followed", root.path().join("outside-link")).unwrap();
    let physical = scan(root.path());
    let inventory = physical.inventory().unwrap();
    let files: Vec<_> = inventory.files().map(|f| f.path.as_str()).collect();
    assert_eq!(
        files,
        [
            ".gitignore",
            "ignored",
            "outside-link",
            "src/file",
            "src/文件"
        ]
    );
    let value = serde_json::to_value(&inventory).unwrap();
    assert_eq!(value["files"][0].as_object().unwrap().len(), 2);
    assert!(value.get("contents").is_none());
    assert_eq!(inventory, scan(root.path()).inventory().unwrap());
    assert_eq!(
        std::fs::read(root.path().join("ignored")).unwrap(),
        [0, 255, 3]
    );
}
