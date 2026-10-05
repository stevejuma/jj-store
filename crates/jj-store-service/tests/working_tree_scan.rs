use jj_store_model::RepositoryId;
use jj_store_service::working_tree::{self as tree, Error, Limits};
use std::{
    fs,
    os::unix::fs::{PermissionsExt, symlink},
};
fn fixture() -> tempfile::TempDir {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir(root.path().join(".jj")).unwrap();
    fs::create_dir(root.path().join(".git")).unwrap();
    fs::write(root.path().join(".jj/heads"), "recorded heads").unwrap();
    fs::write(root.path().join(".git/companion"), "selected metadata").unwrap();
    root
}
fn scan(root: &std::path::Path) -> tree::WorkingTree {
    tree::scan(
        root,
        RepositoryId::new([1; 16]),
        7,
        Limits::default(),
        |_| Ok(()),
    )
    .unwrap()
}
#[test]
fn physical_scan_is_read_only_sorted_and_binds_exact_binary_modes_and_names() {
    let root = fixture();
    fs::create_dir(root.path().join("src")).unwrap();
    fs::write(root.path().join("src/binary"), [0, 0xff, 2]).unwrap();
    fs::write(root.path().join("z.txt"), "unsnapshotted\n").unwrap();
    let before = scan(root.path());
    assert_eq!(
        before
            .entries
            .iter()
            .map(|e| e.path.as_str())
            .collect::<Vec<_>>(),
        ["src/binary", "z.txt"]
    );
    assert_eq!(before, scan(root.path()));
    assert_eq!(
        fs::read(root.path().join(".jj/heads")).unwrap(),
        b"recorded heads"
    );
    assert_eq!(
        fs::read(root.path().join(".git/companion")).unwrap(),
        b"selected metadata"
    );
    assert_eq!(
        fs::read(root.path().join("src/binary")).unwrap(),
        [0, 0xff, 2]
    );
    fs::set_permissions(root.path().join("z.txt"), fs::Permissions::from_mode(0o755)).unwrap();
    assert_ne!(before.fingerprint, scan(root.path()).fingerprint);
    fs::rename(root.path().join("z.txt"), root.path().join("a.txt")).unwrap();
    assert_ne!(before.fingerprint, scan(root.path()).fingerprint);
}
#[test]
fn scan_includes_untracked_obstacles_but_never_follows_links_or_metadata() {
    let root = fixture();
    let external = tempfile::tempdir().unwrap();
    fs::write(external.path().join("secret"), "outside").unwrap();
    symlink(external.path(), root.path().join("outside-link")).unwrap();
    fs::write(root.path().join(".gitignore"), "ignored\n").unwrap();
    fs::write(root.path().join("ignored"), "checkout obstacle").unwrap();
    let before = scan(root.path());
    assert!(before.entries.iter().any(|e| e.path == "ignored"));
    assert!(
        before
            .entries
            .iter()
            .any(|e| e.path == "outside-link" && e.kind == tree::Kind::Symlink)
    );
    assert!(!before.entries.iter().any(|e| e.path.contains("secret")
        || e.path.starts_with(".jj/")
        || e.path.starts_with(".git/")));
    fs::write(external.path().join("secret"), "changed outside").unwrap();
    fs::write(root.path().join(".jj/heads"), "changed recorded heads").unwrap();
    assert_eq!(before.fingerprint, scan(root.path()).fingerprint);
    fs::remove_file(root.path().join("ignored")).unwrap();
    assert_ne!(before.fingerprint, scan(root.path()).fingerprint);
}
#[test]
fn physical_scan_limits_and_cancellation_return_no_partial_selection() {
    let root = fixture();
    fs::write(root.path().join("one"), "1234").unwrap();
    let repo = RepositoryId::new([1; 16]);
    for limits in [
        Limits {
            max_entries: 1,
            ..Limits::default()
        },
        Limits {
            max_bytes: 3,
            ..Limits::default()
        },
        Limits {
            max_file_bytes: 3,
            ..Limits::default()
        },
    ] {
        assert_eq!(
            tree::scan(root.path(), repo, 7, limits, |_| Ok(())).unwrap_err(),
            Error::Limit
        );
    }
    assert_eq!(
        tree::scan(root.path(), repo, 7, Limits::default(), |_| Err(
            Error::Cancelled
        ))
        .unwrap_err(),
        Error::Cancelled
    );
}
#[test]
fn scan_rejects_hardlinks_special_files_unsafe_names_and_workspace_replacement() {
    let root = fixture();
    fs::write(root.path().join("one"), "same").unwrap();
    fs::hard_link(root.path().join("one"), root.path().join("alias")).unwrap();
    let repo = RepositoryId::new([1; 16]);
    assert_eq!(
        tree::scan(root.path(), repo, 7, Limits::default(), |_| Ok(())).unwrap_err(),
        Error::Unsupported
    );
    fs::remove_file(root.path().join("alias")).unwrap();
    let socket = std::os::unix::net::UnixListener::bind(root.path().join("socket")).unwrap();
    assert_eq!(
        tree::scan(root.path(), repo, 7, Limits::default(), |_| Ok(())).unwrap_err(),
        Error::Unsupported
    );
    drop(socket);
    fs::remove_file(root.path().join("socket")).unwrap();
    let before = scan(root.path());
    fs::rename(root.path().join(".jj"), root.path().join("old-jj")).unwrap();
    fs::create_dir(root.path().join(".jj")).unwrap();
    assert_ne!(before.workspace, scan(root.path()).workspace);
    fs::write(root.path().join("bad\\name"), "unsafe").unwrap();
    assert_eq!(
        tree::scan(root.path(), repo, 7, Limits::default(), |_| Ok(())).unwrap_err(),
        Error::Invalid
    );
}

#[test]
fn physical_scan_enforces_depth_deadline_and_verify_cancellation() {
    let root = fixture();
    fs::create_dir_all(root.path().join("one/two")).unwrap();
    fs::write(root.path().join("one/two/text"), "content").unwrap();
    let repo = RepositoryId::new([1; 16]);
    assert_eq!(
        tree::scan(
            root.path(),
            repo,
            7,
            Limits {
                max_depth: 1,
                ..Limits::default()
            },
            |_| Ok(())
        )
        .unwrap_err(),
        Error::Limit
    );
    assert_eq!(
        tree::scan(
            root.path(),
            repo,
            7,
            Limits {
                timeout: std::time::Duration::from_millis(1),
                ..Limits::default()
            },
            |_| {
                std::thread::sleep(std::time::Duration::from_millis(2));
                Ok(())
            }
        )
        .unwrap_err(),
        Error::Limit
    );
    assert_eq!(
        tree::scan(root.path(), repo, 7, Limits::default(), |phase| {
            if phase == tree::Phase::Verify {
                Err(Error::Cancelled)
            } else {
                Ok(())
            }
        })
        .unwrap_err(),
        Error::Cancelled
    );
    let selection = scan(root.path());
    let other_binding = tree::scan(root.path(), repo, 8, Limits::default(), |_| Ok(())).unwrap();
    assert_ne!(selection.fingerprint, other_binding.fingerprint);
    assert_eq!(selection.workspace, other_binding.workspace);
}
#[test]
fn later_file_edits_are_detected_by_the_second_bounded_pass() {
    let root = fixture();
    fs::write(root.path().join("one"), "before").unwrap();
    let mut changed = false;
    let result = tree::scan(
        root.path(),
        RepositoryId::new([1; 16]),
        7,
        Limits::default(),
        |phase| {
            if phase == tree::Phase::Verify && !changed {
                fs::write(root.path().join("one"), "external edit").unwrap();
                changed = true;
            }
            Ok(())
        },
    );
    assert_eq!(result.unwrap_err(), Error::Stale);
    assert_eq!(fs::read(root.path().join("one")).unwrap(), b"external edit");
}
