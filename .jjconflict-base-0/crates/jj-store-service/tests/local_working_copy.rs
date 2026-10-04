use jj_store_model::RepositoryId;
use jj_store_service::working_copy::{self as wc, Error};
use std::{
    fs,
    os::unix::fs::{PermissionsExt, symlink},
};

#[test]
fn physical_save_is_atomic_stale_checked_and_preserves_recorded_state() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path();
    fs::create_dir(path.join(".jj")).unwrap();
    fs::write(path.join(".jj/recorded"), b"unchanged heads and metadata").unwrap();
    fs::write(path.join("example.txt"), b"original\n").unwrap();
    fs::set_permissions(path.join("example.txt"), fs::Permissions::from_mode(0o755)).unwrap();
    let repo = RepositoryId::new([1; 16]);
    let original = wc::read(path, repo, 1, "example.txt").unwrap();
    let saved = wc::save(path, repo, 1, &original.selection, "edited\n").unwrap();
    assert_eq!(saved.contents, "edited\n");
    assert_ne!(saved.selection.fingerprint, original.selection.fingerprint);
    assert_eq!(
        fs::metadata(path.join("example.txt"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o755
    );
    assert_eq!(
        fs::read(path.join(".jj/recorded")).unwrap(),
        b"unchanged heads and metadata"
    );
    assert_eq!(
        wc::save(path, repo, 1, &original.selection, "stale").unwrap_err(),
        Error::Stale
    );
    assert_eq!(
        wc::save(path, repo, 2, &saved.selection, "wrong generation").unwrap_err(),
        Error::Stale
    );
    assert_eq!(
        wc::save(
            path,
            RepositoryId::new([2; 16]),
            1,
            &saved.selection,
            "foreign"
        )
        .unwrap_err(),
        Error::Stale
    );
    fs::write(path.join("example.txt"), b"external edit").unwrap();
    assert_eq!(
        wc::save(path, repo, 1, &saved.selection, "stale external").unwrap_err(),
        Error::Stale
    );
    assert_eq!(
        fs::read_to_string(path.join("example.txt")).unwrap(),
        "external edit"
    );
}

#[test]
fn working_files_reject_unsafe_paths_symlinks_special_and_unbounded_content() {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir(root.path().join(".jj")).unwrap();
    fs::create_dir(root.path().join("nested")).unwrap();
    fs::write(root.path().join("nested/text"), "é\n").unwrap();
    let repo = RepositoryId::new([1; 16]);
    for path in [
        "../outside",
        "/absolute",
        ".jj/config",
        ".JJ/config",
        ".Git/config",
        "nested/.git/config",
        "a\\b",
        "a//b",
        "nested/./text",
        "",
    ] {
        assert_eq!(
            wc::read(root.path(), repo, 1, path).unwrap_err(),
            Error::Invalid,
            "{path}"
        );
    }
    symlink(root.path().join("nested/text"), root.path().join("link")).unwrap();
    symlink(root.path().join("nested"), root.path().join("linked-dir")).unwrap();
    assert!(wc::read(root.path(), repo, 1, "link").is_err());
    assert!(wc::read(root.path(), repo, 1, "linked-dir/text").is_err());
    assert!(wc::read(root.path(), repo, 1, "nested").is_err());
    fs::write(root.path().join("binary"), [0xff, 0]).unwrap();
    assert_eq!(
        wc::read(root.path(), repo, 1, "binary").unwrap_err(),
        Error::Unsupported
    );
    fs::write(root.path().join("large"), vec![b'a'; wc::MAX_BYTES + 1]).unwrap();
    assert_eq!(
        wc::read(root.path(), repo, 1, "large").unwrap_err(),
        Error::Limit
    );
    let before = wc::read(root.path(), repo, 1, "nested/text").unwrap();
    assert_eq!(
        wc::save(
            root.path(),
            repo,
            1,
            &before.selection,
            &"a".repeat(wc::MAX_BYTES + 1)
        )
        .unwrap_err(),
        Error::Limit
    );
    assert_eq!(
        fs::read_to_string(root.path().join("nested/text")).unwrap(),
        "é\n"
    );
}

#[test]
fn workspace_replacement_and_file_replacement_are_stale() {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir(root.path().join(".jj")).unwrap();
    fs::write(root.path().join("text"), "same").unwrap();
    let repo = RepositoryId::new([1; 16]);
    let before = wc::read(root.path(), repo, 1, "text").unwrap();
    fs::rename(root.path().join("text"), root.path().join("old")).unwrap();
    fs::write(root.path().join("text"), "same").unwrap();
    assert_eq!(
        wc::save(root.path(), repo, 1, &before.selection, "draft").unwrap_err(),
        Error::Stale
    );
    let before = wc::read(root.path(), repo, 1, "text").unwrap();
    fs::rename(root.path().join(".jj"), root.path().join("old-jj")).unwrap();
    fs::create_dir(root.path().join(".jj")).unwrap();
    assert_eq!(
        wc::save(root.path(), repo, 1, &before.selection, "draft").unwrap_err(),
        Error::Stale
    );
}

#[test]
fn concurrent_browser_saves_accept_only_one_original_selection() {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir(root.path().join(".jj")).unwrap();
    fs::write(root.path().join("text"), "initial").unwrap();
    let repo = RepositoryId::new([1; 16]);
    let original = wc::read(root.path(), repo, 1, "text").unwrap();
    let barrier = std::sync::Barrier::new(2);
    let results = std::thread::scope(|scope| {
        let a = scope.spawn(|| {
            barrier.wait();
            wc::save(root.path(), repo, 1, &original.selection, "one")
        });
        let b = scope.spawn(|| {
            barrier.wait();
            wc::save(root.path(), repo, 1, &original.selection, "two")
        });
        [a.join().unwrap(), b.join().unwrap()]
    });
    assert_eq!(results.iter().filter(|r| r.is_ok()).count(), 1);
    assert_eq!(
        results
            .iter()
            .filter(|r| matches!(r, Err(Error::Stale)))
            .count(),
        1
    );
    assert!(
        fs::read_dir(root.path().join(".jj"))
            .unwrap()
            .next()
            .is_none()
    );
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 2);
    fs::hard_link(root.path().join("text"), root.path().join("alias")).unwrap();
    assert_eq!(
        wc::read(root.path(), repo, 1, "text").unwrap_err(),
        Error::Unsupported
    );
}
