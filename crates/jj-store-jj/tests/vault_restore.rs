#![cfg(feature = "vault-backup")]
use jj_store_jj::vault_workspace::{Binding, FileContents, RestoreRequest, Session};
use std::fs;

#[cfg(unix)]
#[tokio::test]
async fn restore_cannot_follow_an_ancestor_alias_into_private_vault_metadata() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let binding = Binding {
        vault_id: "aliased-restore".into(),
        root: base.join("notes"),
        state: base.join("private/history"),
    };
    fs::create_dir_all(binding.root.join("alias")).unwrap();
    fs::create_dir(binding.root.join(".lapis")).unwrap();
    fs::write(
        binding.root.join("alias/settings.json"),
        "historical public note",
    )
    .unwrap();
    fs::write(
        binding.root.join(".lapis/settings.json"),
        "private runtime state",
    )
    .unwrap();
    let mut session = Session::open(binding.clone()).await.unwrap();
    let checkpoint = session
        .capture(&[], "Public history")
        .await
        .unwrap()
        .checkpoint
        .unwrap();
    fs::write(
        binding.root.join("alias/settings.json"),
        "private runtime state",
    )
    .unwrap();
    let preview = session
        .preview_file_restore(&checkpoint.commit, "alias/settings.json")
        .await
        .unwrap();
    fs::remove_dir_all(binding.root.join("alias")).unwrap();
    std::os::unix::fs::symlink(".lapis", binding.root.join("alias")).unwrap();
    let preview_denied = session
        .preview_file_restore(&checkpoint.commit, "alias/settings.json")
        .await
        .is_err();
    let applied = session
        .restore_file(
            &RestoreRequest {
                request_id: "aliased-command".into(),
                checkpoint: checkpoint.commit,
                path: preview.path,
                expected: preview.expected,
            },
            &[],
        )
        .await;
    assert_eq!(
        fs::read_to_string(binding.root.join(".lapis/settings.json")).unwrap(),
        "private runtime state",
        "Restoration must not write excluded runtime state through an ancestor alias"
    );
    assert!(
        preview_denied,
        "Restore preview must reject symbolic-link ancestors"
    );
    assert!(
        applied.is_err(),
        "Apply must recheck ancestors even after a valid preview"
    );
}

#[tokio::test]
async fn restores_retained_bytes_after_a_safety_checkpoint_and_replays_only_its_receipt() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let binding = Binding {
        vault_id: "restore-vault".into(),
        root: base.join("notes"),
        state: base.join("private/history"),
    };
    fs::create_dir(&binding.root).unwrap();
    fs::write(binding.root.join("Note.md"), "original").unwrap();
    let mut session = Session::open(binding.clone()).await.unwrap();
    let first = session
        .capture(&[], "First")
        .await
        .unwrap()
        .checkpoint
        .unwrap();
    fs::write(binding.root.join("Note.md"), "unsnapshotted new text").unwrap();
    let preview = session
        .preview_file_restore(&first.commit, "Note.md")
        .await
        .unwrap();
    let request = RestoreRequest {
        request_id: "restore-1".into(),
        checkpoint: preview.checkpoint,
        path: preview.path,
        expected: preview.expected,
    };
    let restored = session.restore_file(&request, &[]).await.unwrap();
    assert_eq!(
        fs::read_to_string(binding.root.join("Note.md")).unwrap(),
        "original"
    );
    let receipt_path = binding.state.join("restore-receipts/restore-1.json");
    let receipt: serde_json::Value =
        serde_json::from_slice(&fs::read(&receipt_path).unwrap()).unwrap();
    let safety = receipt["safety"]["commit"].as_str().unwrap();
    assert_eq!(
        session.file_at(safety, "Note.md").await.unwrap(),
        Some(FileContents::Regular {
            data: b"unsnapshotted new text".to_vec(),
            executable: false
        })
    );
    drop(session);
    let mut session = Session::open(binding.clone()).await.unwrap();
    fs::write(binding.root.join("Note.md"), "newer edit after restore").unwrap();
    let replay = session.restore_file(&request, &[]).await.unwrap();
    assert_eq!(replay.operation, restored.operation);
    assert_eq!(
        fs::read_to_string(binding.root.join("Note.md")).unwrap(),
        "newer edit after restore"
    );
    let mut collision = request.clone();
    collision.path = "another.md".into();
    assert!(session.restore_file(&collision, &[]).await.is_err());
    assert!(!binding.root.join(".jj").exists());
}

#[tokio::test]
async fn stale_previews_and_pending_receipts_never_overwrite_later_edits() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let binding = Binding {
        vault_id: "restore-vault".into(),
        root: base.join("notes"),
        state: base.join("private/history"),
    };
    fs::create_dir(&binding.root).unwrap();
    fs::write(binding.root.join("Note.md"), "v1").unwrap();
    let mut session = Session::open(binding.clone()).await.unwrap();
    let first = session
        .capture(&[], "First")
        .await
        .unwrap()
        .checkpoint
        .unwrap();
    fs::write(binding.root.join("Note.md"), "v2").unwrap();
    let preview = session
        .preview_file_restore(&first.commit, "Note.md")
        .await
        .unwrap();
    let mut request = RestoreRequest {
        request_id: "restore-2".into(),
        checkpoint: preview.checkpoint,
        path: preview.path,
        expected: preview.expected,
    };
    fs::write(binding.root.join("Note.md"), "v3").unwrap();
    let before = session.operation();
    assert!(session.restore_file(&request, &[]).await.is_err());
    assert_eq!(session.operation(), before);
    assert_eq!(
        fs::read_to_string(binding.root.join("Note.md")).unwrap(),
        "v3"
    );
    request.expected = session
        .preview_file_restore(&first.commit, "Note.md")
        .await
        .unwrap()
        .expected;
    session.restore_file(&request, &[]).await.unwrap();
    let path = binding.state.join("restore-receipts/restore-2.json");
    let mut receipt: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    receipt["result"] = serde_json::Value::Null;
    fs::write(&path, serde_json::to_vec(&receipt).unwrap()).unwrap();
    drop(session);
    let mut session = Session::open(binding.clone()).await.unwrap();
    // Represents a process exit after file replacement, before receipt completion.
    session.restore_file(&request, &[]).await.unwrap();
    assert_eq!(
        fs::read_to_string(binding.root.join("Note.md")).unwrap(),
        "v1"
    );
    receipt["result"] = serde_json::Value::Null;
    fs::write(&path, serde_json::to_vec(&receipt).unwrap()).unwrap();
    fs::write(binding.root.join("Note.md"), "later external edit").unwrap();
    assert!(session.restore_file(&request, &[]).await.is_err());
    assert_eq!(
        fs::read_to_string(binding.root.join("Note.md")).unwrap(),
        "later external edit"
    );
    assert!(
        session
            .preview_file_restore(&first.commit, "../escape")
            .await
            .is_err()
    );
    assert!(
        session
            .preview_file_restore(&first.commit, ".lapis/config")
            .await
            .is_err()
    );
}

#[tokio::test]
async fn restores_binary_deletion_and_execution_mode_without_touching_excluded_files() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let binding = Binding {
        vault_id: "binary-restore".into(),
        root: base.join("notes"),
        state: base.join("private/history"),
    };
    fs::create_dir(&binding.root).unwrap();
    fs::write(binding.root.join("attachment.bin"), [0, 255, 1]).unwrap();
    let mut session = Session::open(binding.clone()).await.unwrap();
    let first = session
        .capture(&[], "First")
        .await
        .unwrap()
        .checkpoint
        .unwrap();
    fs::remove_file(binding.root.join("attachment.bin")).unwrap();
    let deleted = session
        .capture(&[], "Deleted")
        .await
        .unwrap()
        .checkpoint
        .unwrap();
    let preview = session
        .preview_file_restore(&first.commit, "attachment.bin")
        .await
        .unwrap();
    let request = RestoreRequest {
        request_id: "restore-binary".into(),
        checkpoint: first.commit.clone(),
        path: preview.path,
        expected: preview.expected,
    };
    assert!(
        session
            .restore_file(&request, &["attachment.bin".into()])
            .await
            .is_err()
    );
    assert!(!binding.root.join("attachment.bin").exists());
    session.restore_file(&request, &[]).await.unwrap();
    assert_eq!(
        fs::read(binding.root.join("attachment.bin")).unwrap(),
        [0, 255, 1]
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        fs::set_permissions(
            binding.root.join("attachment.bin"),
            fs::Permissions::from_mode(0o755),
        )
        .unwrap();
        let preview = session
            .preview_file_restore(&first.commit, "attachment.bin")
            .await
            .unwrap();
        session
            .restore_file(
                &RestoreRequest {
                    request_id: "restore-mode".into(),
                    checkpoint: first.commit,
                    path: preview.path,
                    expected: preview.expected,
                },
                &[],
            )
            .await
            .unwrap();
        assert_eq!(
            fs::metadata(binding.root.join("attachment.bin"))
                .unwrap()
                .permissions()
                .mode()
                & 0o111,
            0
        );
    }
    let preview = session
        .preview_file_restore(&deleted.commit, "attachment.bin")
        .await
        .unwrap();
    assert!(preview.deleted);
    session
        .restore_file(
            &RestoreRequest {
                request_id: "restore-deletion".into(),
                checkpoint: deleted.commit,
                path: preview.path,
                expected: preview.expected,
            },
            &[],
        )
        .await
        .unwrap();
    assert!(!binding.root.join("attachment.bin").exists());
}
