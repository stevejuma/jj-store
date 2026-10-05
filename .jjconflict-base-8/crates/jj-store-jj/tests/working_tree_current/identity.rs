use super::*;

#[tokio::test(flavor = "multi_thread")]
async fn stable_identity_matches_observer_and_survives_current_code_loss_without_writes() {
    let (_fixture, root, access, operation) = fixture().await;
    let reader = code::Executor::new(1).unwrap();
    let observed = reader
        .working_tree_current(access.clone(), root.clone(), operation)
        .await
        .unwrap();
    let identity = reader
        .physical_workspace_identity(access.clone(), root.clone())
        .await
        .unwrap();
    assert_eq!(identity, observed.physical_workspace);
    let checkout = root.join(".jj/working_copy/checkout");
    let tree = root.join(".jj/working_copy/tree_state");
    let original = [fs::read(&checkout).unwrap(), fs::read(&tree).unwrap()];
    assert_eq!(
        reader
            .physical_workspace_identity(access.clone(), root.clone())
            .await
            .unwrap(),
        identity
    );
    assert_eq!(
        [fs::read(&checkout).unwrap(), fs::read(&tree).unwrap()],
        original
    );

    // Recovery admission must not hydrate a missing current operation/index,
    // decode damaged checkout bytes or require readable policy/editor files.
    for name in ["op_store", "op_heads", "index"] {
        fs::remove_dir_all(access.0.join(name)).unwrap();
    }
    fs::remove_file(&tree).unwrap();
    fs::write(&checkout, "invalid current checkout bytes").unwrap();
    fs::write(access.0.join("store/git/config"), "invalid config").unwrap();
    fs::remove_file(access.0.join("store/git/info/exclude")).unwrap();
    std::os::unix::fs::symlink("/missing-policy", access.0.join("store/git/info/exclude")).unwrap();
    fs::write(root.join("editor.txt"), "unsnapshotted edits\n").unwrap();
    std::os::unix::fs::symlink("/missing-editor-file", root.join("editor-link")).unwrap();
    assert_eq!(
        reader
            .physical_workspace_identity(access, root.clone())
            .await
            .unwrap(),
        identity
    );
    for name in [
        ".jj/repo/op_store",
        ".jj/repo/op_heads",
        ".jj/repo/index",
        ".jj/working_copy/tree_state",
    ] {
        assert!(
            !root.join(name).exists(),
            "identity reads must not repair {name}"
        );
    }
    assert_eq!(
        fs::read(&checkout).unwrap(),
        b"invalid current checkout bytes"
    );
    assert_eq!(
        fs::read(root.join("editor.txt")).unwrap(),
        b"unsnapshotted edits\n"
    );
    assert!(
        fs::symlink_metadata(root.join("editor-link"))
            .unwrap()
            .file_type()
            .is_symlink()
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn stable_identity_distinguishes_root_and_jj_replacement() {
    for replace in ["root", "jj"] {
        let (_fixture, root, access, _) = fixture().await;
        let reader = code::Executor::new(1).unwrap();
        let before = reader
            .physical_workspace_identity(access.clone(), root.clone())
            .await
            .unwrap();
        if replace == "root" {
            let saved = root.with_extension("saved");
            fs::rename(&root, &saved).unwrap();
            fs::create_dir(&root).unwrap();
            fs::rename(saved.join(".jj"), root.join(".jj")).unwrap();
        } else {
            fs::rename(root.join(".jj"), root.join(".jj-saved")).unwrap();
            fs::create_dir(root.join(".jj")).unwrap();
        }
        let after = reader
            .physical_workspace_identity(access, root)
            .await
            .unwrap();
        assert_ne!(
            before, after,
            "{replace} replacement must not retain admission identity"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn stable_identity_rejects_unsafe_missing_and_unbounded_roots() {
    let (_fixture, root, access, _) = fixture().await;
    let reader = code::Executor::new(1).unwrap();
    let link = root.with_extension("link");
    std::os::unix::fs::symlink(&root, &link).unwrap();
    for path in [
        link,
        PathBuf::from("relative"),
        root.join("../workspace"),
        root.join("missing"),
        PathBuf::from(format!("/{}", "x".repeat(4096))),
    ] {
        assert!(
            reader
                .physical_workspace_identity(access.clone(), path)
                .await
                .is_err()
        );
    }
    fs::rename(root.join(".jj"), root.join(".jj-saved")).unwrap();
    assert!(
        reader
            .physical_workspace_identity(access.clone(), root.clone())
            .await
            .is_err()
    );
    assert!(
        !root.join(".jj").exists(),
        "missing directories must not be initialized"
    );
    std::os::unix::fs::symlink(root.join(".jj-saved"), root.join(".jj")).unwrap();
    assert!(
        reader
            .physical_workspace_identity(access, root)
            .await
            .is_err()
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn stable_identity_rechecks_replacement_and_authorization() {
    for change in [
        "root",
        "jj",
        "denied-before",
        "denied-between",
        "denied-after",
    ] {
        let (_fixture, root, access, _) = fixture().await;
        let changed_root = root.clone();
        let hook = Arc::new(HookAccess {
            metadata: access.0.clone(),
            calls: AtomicUsize::new(0),
            hook: Box::new(move |n| {
                if (change == "denied-before" && n == 1)
                    || (change == "denied-between" && n == 2)
                    || (change == "denied-after" && n == 3)
                {
                    return Err(code::Error::Denied);
                }
                if n == 2 && change == "root" {
                    let saved = changed_root.with_extension("saved");
                    fs::rename(&changed_root, &saved).unwrap();
                    fs::create_dir(&changed_root).unwrap();
                    fs::rename(saved.join(".jj"), changed_root.join(".jj")).unwrap();
                }
                if n == 2 && change == "jj" {
                    fs::rename(changed_root.join(".jj"), changed_root.join(".jj-saved")).unwrap();
                    fs::create_dir(changed_root.join(".jj")).unwrap();
                }
                Ok(())
            }),
        });
        let result = code::Executor::new(1)
            .unwrap()
            .physical_workspace_identity(hook, root)
            .await;
        if change.starts_with("denied") {
            assert!(
                matches!(result, Err(code::Error::Denied)),
                "{change}: {result:?}"
            );
        } else {
            assert!(
                matches!(result, Err(code::Error::EpochChanged)),
                "{change}: {result:?}"
            );
        }
    }
}
