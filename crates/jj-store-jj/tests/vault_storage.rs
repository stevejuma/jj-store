#![cfg(feature = "vault-backup")]
use jj_store_jj::vault_workspace::{
    Binding, FileContents, PushOutcome, RemoteTransfer, Session, Storage, StorageLocation,
};
use std::{fs, path::Path};
async fn fixture(base: &Path) -> (Binding, Storage, String) {
    let binding = Binding {
        vault_id: "layout-vault".into(),
        root: base.join("notes"),
        state: base.join("private/history"),
    };
    fs::create_dir(&binding.root).unwrap();
    fs::write(binding.root.join("Note.md"), "first version").unwrap();
    let mut session = Session::open(binding.clone()).await.unwrap();
    let first = session
        .capture(&[], "Original retained checkpoint")
        .await
        .unwrap()
        .checkpoint
        .unwrap()
        .commit;
    drop(session);
    let storage = Storage::new(binding.clone(), base.join("private/layout"));
    (binding, storage, first)
}
#[tokio::test]
async fn relocation_preserves_native_history_receipts_and_unsnapshotted_files_across_both_layouts()
{
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let (binding, storage, first) = fixture(&base).await;
    let origin = base.join("origin.git");
    assert!(
        std::process::Command::new("git")
            .args(["init", "--bare", "--quiet"])
            .arg(&origin)
            .status()
            .unwrap()
            .success()
    );
    let remote = RemoteTransfer::connect_local(binding.clone(), &origin, "main")
        .await
        .unwrap();
    let session = Session::open(binding.clone()).await.unwrap();
    let intent = session
        .prepare_push(&remote, "pending-original", None)
        .await
        .unwrap();
    drop(session);
    remote.reserve(&intent).unwrap();
    let receipt = binding
        .state
        .join("remote-push-receipts/pending-original.json");
    let mut value: serde_json::Value =
        serde_json::from_slice(&fs::read(&receipt).unwrap()).unwrap();
    value["outcome"] = serde_json::json!("unknown");
    fs::write(&receipt, serde_json::to_vec(&value).unwrap()).unwrap();
    let bytes = fs::read(&receipt).unwrap();
    fs::write(binding.root.join("Note.md"), "saved but not captured").unwrap();
    let original = Session::open(binding.clone()).await.unwrap().operation();
    let moved = storage
        .change(0, StorageLocation::Colocated, "to-root")
        .await
        .unwrap();
    assert_eq!(moved.revision, 1);
    let root = storage.binding().unwrap();
    assert_eq!(root.state, binding.root.join(".jj"));
    assert!(!binding.state.exists());
    assert_eq!(
        fs::read(
            root.state
                .join("remote-push-receipts/pending-original.json")
        )
        .unwrap(),
        bytes
    );
    let mut session = Session::open(root.clone()).await.unwrap();
    assert_eq!(session.operation(), original);
    assert_eq!(
        session.file_at(&first, "Note.md").await.unwrap(),
        Some(FileContents::Regular {
            data: b"first version".to_vec(),
            executable: false
        })
    );
    session
        .capture(&[], "Saved edit after relocation")
        .await
        .unwrap();
    let last = session.checkpoint().unwrap().unwrap().commit;
    assert!(
        !session
            .capture(&[], "Only metadata locks changed")
            .await
            .unwrap()
            .changed
    );
    drop(session);
    assert_eq!(
        root.pending_publication().unwrap().unwrap().request_id,
        "pending-original"
    );
    assert_eq!(
        root.stop_publication("pending-original").unwrap().outcome,
        PushOutcome::Stopped
    );
    let returned = storage
        .change(1, StorageLocation::External, "to-external")
        .await
        .unwrap();
    assert_eq!(returned.revision, 2);
    let current = storage.binding().unwrap();
    assert_eq!(current.state, binding.state);
    assert_eq!(
        Session::open(current.clone())
            .await
            .unwrap()
            .checkpoint()
            .unwrap()
            .unwrap()
            .commit,
        last
    );
    let remote = RemoteTransfer::connect_local(current.clone(), &origin, "main")
        .await
        .unwrap();
    assert_eq!(remote.publish(&intent).await.unwrap(), PushOutcome::Stopped);
    assert_eq!(
        storage
            .change(0, StorageLocation::Colocated, "to-root")
            .await
            .unwrap()
            .location,
        StorageLocation::External,
        "a completed old request cannot move storage again"
    );
    assert_eq!(
        fs::read_to_string(current.root.join("Note.md")).unwrap(),
        "saved but not captured"
    );
    assert!(!current.root.join(".jj").exists());
}
#[tokio::test]
async fn interrupted_relocation_recovers_the_original_archive_and_never_initializes_an_empty_replacement()
 {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let (binding, storage, first) = fixture(&base).await;
    assert!(
        storage
            .change_with(0, StorageLocation::Colocated, "interrupted", |progress| {
                if progress.phase == "archived" {
                    Err(jj_store_jj::vault_workspace::Error::Busy)
                } else {
                    Ok(())
                }
            })
            .await
            .is_err()
    );
    assert!(storage.status().unwrap().pending.is_some());
    assert!(Session::open(binding.clone()).await.is_err());
    assert!(!binding.state.exists());
    let recovered = storage.recover().await.unwrap();
    assert_eq!(recovered.location, StorageLocation::Colocated);
    assert_eq!(
        Session::open(storage.binding().unwrap())
            .await
            .unwrap()
            .checkpoint()
            .unwrap()
            .unwrap()
            .commit,
        first
    );
    assert!(storage.status().unwrap().pending.is_none());
}
#[tokio::test]
async fn existing_destination_and_symlinked_private_directories_are_never_overwritten() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let (binding, storage, first) = fixture(&base).await;
    fs::create_dir(binding.root.join(".jj")).unwrap();
    fs::write(binding.root.join(".jj/user-owned"), "preserve").unwrap();
    assert!(
        storage
            .change(0, StorageLocation::Colocated, "blocked")
            .await
            .is_err()
    );
    assert_eq!(
        fs::read_to_string(binding.root.join(".jj/user-owned")).unwrap(),
        "preserve"
    );
    assert_eq!(
        Session::open(binding.clone())
            .await
            .unwrap()
            .checkpoint()
            .unwrap()
            .unwrap()
            .commit,
        first
    );
    fs::remove_dir_all(binding.root.join(".jj")).unwrap();
    #[cfg(unix)]
    {
        let outside = base.join("outside");
        fs::create_dir(&outside).unwrap();
        std::os::unix::fs::symlink(&outside, binding.root.join(".lapis")).unwrap();
        assert!(
            storage
                .change(0, StorageLocation::Colocated, "symlink")
                .await
                .is_err()
        );
        assert_eq!(fs::read_dir(&outside).unwrap().count(), 0);
    }
}

#[tokio::test]
async fn root_initialization_and_root_relink_keep_metadata_out_of_the_captured_tree() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    fs::create_dir(base.join("notes")).unwrap();
    fs::write(base.join("notes/Note.md"), "saved note").unwrap();
    let binding = Binding {
        vault_id: "root-init".into(),
        root: base.join("notes"),
        state: base.join("private/history"),
    };
    let storage = Storage::new(binding.clone(), base.join("private/layout"));
    storage
        .change(0, StorageLocation::Colocated, "initial-root")
        .await
        .unwrap();
    let root = storage.binding().unwrap();
    let mut session = Session::open(root.clone()).await.unwrap();
    assert!(session.checkpoint().unwrap().is_none());
    let checkpoint = session
        .capture(&[], "First explicit capture")
        .await
        .unwrap()
        .checkpoint
        .unwrap();
    assert!(
        !session
            .capture(&[], "Unchanged native controls")
            .await
            .unwrap()
            .changed
    );
    drop(session);
    let new_root = base.join("moved-notes");
    fs::rename(&root.root, &new_root).unwrap();
    let moved = Binding {
        root: new_root.clone(),
        state: new_root.join(".jj"),
        ..root
    };
    moved.relink().unwrap();
    let session = Session::open(moved.clone()).await.unwrap();
    assert_eq!(
        session.checkpoint().unwrap().unwrap().commit,
        checkpoint.commit
    );
    let files = session
        .history_files(&checkpoint.commit, None, &[])
        .await
        .unwrap();
    assert_eq!(files.files.len(), 1);
    assert_eq!(files.files[0].path, "Note.md");
    assert!(
        session
            .file_at(&checkpoint.commit, ".root-init-backup.lock")
            .await
            .unwrap()
            .is_none()
    );
}

#[tokio::test]
async fn copied_and_installed_moves_recover_and_a_blocked_copy_can_keep_its_original_layout() {
    for phase in ["copied", "installed"] {
        let temp = tempfile::tempdir().unwrap();
        let base = temp.path().canonicalize().unwrap();
        let (_, storage, first) = fixture(&base).await;
        assert!(
            storage
                .change_with(0, StorageLocation::Colocated, "original", |progress| {
                    if progress.phase == phase {
                        Err(jj_store_jj::vault_workspace::Error::Busy)
                    } else {
                        Ok(())
                    }
                })
                .await
                .is_err()
        );
        if phase == "installed" {
            assert!(
                storage.rollback("original").await.is_err(),
                "an installed target must finish rather than rewind"
            );
        }
        storage.recover().await.unwrap();
        assert_eq!(
            Session::open(storage.binding().unwrap())
                .await
                .unwrap()
                .checkpoint()
                .unwrap()
                .unwrap()
                .commit,
            first
        );
    }
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let (binding, storage, first) = fixture(&base).await;
    assert!(
        storage
            .change_with(0, StorageLocation::Colocated, "keep-old", |progress| {
                if progress.phase == "copied" {
                    Err(jj_store_jj::vault_workspace::Error::Busy)
                } else {
                    Ok(())
                }
            })
            .await
            .is_err()
    );
    fs::create_dir(binding.root.join(".jj")).unwrap();
    fs::write(binding.root.join(".jj/user-owned"), "do not overwrite").unwrap();
    assert!(storage.recover().await.is_err());
    let status = storage.rollback("keep-old").await.unwrap();
    assert_eq!(status.location, StorageLocation::External);
    assert!(status.pending.is_none());
    assert_eq!(
        Session::open(binding.clone())
            .await
            .unwrap()
            .checkpoint()
            .unwrap()
            .unwrap()
            .commit,
        first
    );
    assert_eq!(
        fs::read_to_string(binding.root.join(".jj/user-owned")).unwrap(),
        "do not overwrite"
    );
    assert_eq!(
        storage
            .change(0, StorageLocation::Colocated, "keep-old")
            .await
            .unwrap()
            .location,
        StorageLocation::External
    );
}

#[cfg(unix)]
#[tokio::test]
async fn interrupted_copy_replaces_owned_readonly_stage_files_on_recovery() {
    use std::os::unix::fs::PermissionsExt as _;
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let (binding, storage, first) = fixture(&base).await;
    let readonly = binding.state.join("000-readonly");
    fs::write(&readonly, "retained native file").unwrap();
    fs::set_permissions(&readonly, fs::Permissions::from_mode(0o444)).unwrap();
    assert!(
        storage
            .change_with(
                0,
                StorageLocation::Colocated,
                "copy-interrupted",
                |progress| {
                    if progress.phase == "copying" && progress.files == 1 {
                        Err(jj_store_jj::vault_workspace::Error::Busy)
                    } else {
                        Ok(())
                    }
                }
            )
            .await
            .is_err()
    );
    storage.recover().await.unwrap();
    let binding = storage.binding().unwrap();
    assert_eq!(
        fs::read_to_string(binding.state.join("000-readonly")).unwrap(),
        "retained native file"
    );
    assert_eq!(
        Session::open(binding)
            .await
            .unwrap()
            .checkpoint()
            .unwrap()
            .unwrap()
            .commit,
        first
    );
}
