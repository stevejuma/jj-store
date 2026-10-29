use jj_store_jj::local_browser::working_tree as baseline;
use jj_store_model as m;
use jj_store_service::working_tree::{self as tree, ChangeKind, Error, Limits};
use std::{
    fs,
    os::unix::fs::{PermissionsExt, symlink},
};

fn fixture() -> tempfile::TempDir {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir(root.path().join(".jj")).unwrap();
    root
}
fn scan(root: &std::path::Path) -> tree::WorkingTree {
    tree::scan(
        root,
        m::RepositoryId::new([1; 16]),
        7,
        Limits::default(),
        |_| Ok(()),
    )
    .unwrap()
}
fn baseline(physical: &tree::WorkingTree) -> baseline::Baseline {
    baseline::Baseline {
        version: 1,
        repository: physical.repository,
        generation: physical.generation,
        operation: m::OperationId::new([1; 64]),
        view: m::ViewId::new([2; 64]),
        workspace: "default".into(),
        commit: m::GitSha1Id::new([3; 20]),
        trees: vec![m::GitSha1Id::new([4; 20])],
        labels: vec![],
        entries: physical
            .entries
            .iter()
            .map(|e| baseline::Entry {
                path: e.path.clone(),
                bytes: e.bytes,
                content: e.content,
                data: e.data.clone(),
                kind: match e.kind {
                    tree::Kind::File { executable } => baseline::Kind::File { executable },
                    tree::Kind::Symlink => baseline::Kind::Symlink,
                },
            })
            .collect(),
    }
}
#[test]
fn projection_preserves_tracked_ignored_changes_and_all_untracked_obstacles() {
    let root = fixture();
    fs::write(root.path().join("tracked.tmp"), "before").unwrap();
    fs::write(root.path().join("deleted"), "before").unwrap();
    fs::write(root.path().join("executable"), [0, 0xff]).unwrap();
    symlink("before", root.path().join("link")).unwrap();
    let before = baseline(&scan(root.path()));
    fs::write(root.path().join("tracked.tmp"), "after!").unwrap();
    fs::remove_file(root.path().join("deleted")).unwrap();
    fs::set_permissions(
        root.path().join("executable"),
        fs::Permissions::from_mode(0o755),
    )
    .unwrap();
    fs::remove_file(root.path().join("link")).unwrap();
    symlink("after", root.path().join("link")).unwrap();
    fs::write(root.path().join("new"), "untracked").unwrap();
    fs::write(root.path().join(".gitignore"), "*.tmp\nbuild/\n").unwrap();
    fs::create_dir(root.path().join("build")).unwrap();
    fs::write(root.path().join("build/.gitignore"), "!keep").unwrap();
    fs::write(root.path().join("build/keep"), "obstacle").unwrap();
    fs::write(root.path().join("other.tmp"), "ignored").unwrap();
    let physical = scan(root.path());
    let projection = tree::project(physical.clone(), before, vec![], || Ok(())).unwrap();
    assert_eq!(projection.physical, physical);
    assert_eq!(
        projection
            .changes
            .iter()
            .map(|c| (c.path.as_str(), c.kind))
            .collect::<Vec<_>>(),
        [
            (".gitignore", ChangeKind::Added),
            ("deleted", ChangeKind::Deleted),
            ("executable", ChangeKind::Modified),
            ("link", ChangeKind::Modified),
            ("new", ChangeKind::Added),
            ("tracked.tmp", ChangeKind::Modified),
        ]
    );
    assert_eq!(
        projection
            .obstacles
            .iter()
            .map(|o| (o.entry.path.as_str(), o.ignored))
            .collect::<Vec<_>>(),
        [
            (".gitignore", false),
            ("build/.gitignore", true),
            ("build/keep", true),
            ("new", false),
            ("other.tmp", true),
        ]
    );
    assert_eq!(physical, scan(root.path()));
}
#[test]
fn projection_uses_content_and_mode_not_physical_timestamps_for_dirty_state() {
    let root = fixture();
    fs::write(root.path().join("file"), "bytes").unwrap();
    let initial = scan(root.path());
    let before = baseline(&initial);
    fs::write(root.path().join("replacement"), "bytes").unwrap();
    fs::rename(root.path().join("replacement"), root.path().join("file")).unwrap();
    let physical = scan(root.path());
    assert_ne!(physical.fingerprint, initial.fingerprint);
    assert!(
        tree::project(physical, before, vec![], || Ok(()))
            .unwrap()
            .changes
            .is_empty()
    );
}
#[test]
fn projection_fails_closed_on_stale_binding_captured_rules_and_cancellation() {
    let root = fixture();
    fs::write(root.path().join(".gitignore"), "ignored").unwrap();
    let physical = scan(root.path());
    let mut before = baseline(&physical);
    before.generation += 1;
    assert_eq!(
        tree::project(physical.clone(), before, vec![], || Ok(())).unwrap_err(),
        Error::Stale
    );
    let before = baseline(&physical);
    let mut changed = physical.clone();
    changed.entries[0].ignore_bytes = Some(b"tampered".to_vec());
    assert_eq!(
        tree::project(changed, before.clone(), vec![], || Ok(())).unwrap_err(),
        Error::Stale
    );
    let mut calls = 0;
    assert_eq!(
        tree::project(physical.clone(), before.clone(), vec![], || {
            calls += 1;
            if calls > 1 {
                Err(Error::Cancelled)
            } else {
                Ok(())
            }
        })
        .unwrap_err(),
        Error::Cancelled
    );
    let mut duplicate = physical.clone();
    duplicate.entries.push(duplicate.entries[0].clone());
    assert_eq!(
        tree::project(duplicate, before, vec![], || Ok(())).unwrap_err(),
        Error::Invalid
    );
    fs::remove_file(root.path().join(".gitignore")).unwrap();
    symlink("outside", root.path().join(".gitignore")).unwrap();
    let linked = scan(root.path());
    assert_eq!(
        tree::project(linked.clone(), baseline(&linked), vec![], || Ok(())).unwrap_err(),
        Error::Unsupported
    );
}

#[test]
fn sparse_projection_keeps_excluded_paths_as_obstacles_without_false_changes() {
    let root = fixture();
    fs::create_dir(root.path().join("src")).unwrap();
    fs::write(root.path().join("src/tracked"), "before").unwrap();
    fs::write(root.path().join("excluded"), "tracked outside sparse").unwrap();
    fs::write(root.path().join("missing"), "tracked outside sparse").unwrap();
    let before = baseline(&scan(root.path()));
    fs::write(root.path().join("src/tracked"), "after").unwrap();
    fs::write(root.path().join("src/new"), "candidate").unwrap();
    fs::write(root.path().join("outside"), "obstacle").unwrap();
    fs::remove_file(root.path().join("missing")).unwrap();
    let projection = tree::project_sparse(
        scan(root.path()),
        before,
        vec![],
        vec!["src".into()],
        || Ok(()),
    )
    .unwrap();
    assert_eq!(
        projection
            .changes
            .iter()
            .map(|c| (c.path.as_str(), c.kind))
            .collect::<Vec<_>>(),
        [
            ("src/new", ChangeKind::Added),
            ("src/tracked", ChangeKind::Modified),
        ]
    );
    assert_eq!(
        projection
            .obstacles
            .iter()
            .map(|o| (o.entry.path.as_str(), o.outside_sparse, o.tracked))
            .collect::<Vec<_>>(),
        [
            ("excluded", true, true),
            ("outside", true, false),
            ("src/new", false, false),
        ]
    );
}
