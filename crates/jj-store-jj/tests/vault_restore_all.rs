#![cfg(feature = "vault-backup")]
use jj_store_jj::vault_workspace::{Binding, FileContents, Session, VaultRestoreRequest};
use std::fs;
fn setup() -> (tempfile::TempDir, Binding) {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let binding = Binding {
        vault_id: "whole-vault".into(),
        root: base.join("notes"),
        state: base.join("private/history"),
    };
    fs::create_dir(&binding.root).unwrap();
    (temp, binding)
}
#[tokio::test]
async fn restores_a_whole_vault_and_preserves_original_safety_and_runtime_files() {
    let (_temp, binding) = setup();
    fs::write(binding.root.join("a.md"), "old a").unwrap();
    fs::write(binding.root.join("c.bin"), [0, 255, 1]).unwrap();
    fs::create_dir(binding.root.join(".lapis")).unwrap();
    fs::write(binding.root.join(".lapis/runtime"), "runtime survives").unwrap();
    let mut session = Session::open(binding.clone()).await.unwrap();
    let first = session
        .capture(&[], "First")
        .await
        .unwrap()
        .checkpoint
        .unwrap();
    fs::write(binding.root.join("a.md"), "new a").unwrap();
    fs::write(binding.root.join("b.md"), "new b").unwrap();
    fs::remove_file(binding.root.join("c.bin")).unwrap();
    let preview = session
        .preview_vault_restore(&first.commit, &[])
        .await
        .unwrap();
    assert_eq!(
        (preview.added, preview.modified, preview.deleted),
        (1, 1, 1)
    );
    let request = VaultRestoreRequest {
        request_id: "all-1".into(),
        checkpoint: first.commit.clone(),
        expected: preview.expected,
    };
    let result = session.restore_vault(&request, &[]).await.unwrap();
    assert_eq!(
        fs::read_to_string(binding.root.join("a.md")).unwrap(),
        "old a"
    );
    assert!(!binding.root.join("b.md").exists());
    assert_eq!(fs::read(binding.root.join("c.bin")).unwrap(), [0, 255, 1]);
    assert_eq!(
        fs::read_to_string(binding.root.join(".lapis/runtime")).unwrap(),
        "runtime survives"
    );
    let receipt: serde_json::Value = serde_json::from_slice(
        &fs::read(binding.state.join("vault-restore-receipts/all-1.json")).unwrap(),
    )
    .unwrap();
    let safety = receipt["safety"]["commit"].as_str().unwrap();
    assert_eq!(
        session.file_at(safety, "a.md").await.unwrap(),
        Some(FileContents::Regular {
            data: b"new a".to_vec(),
            executable: false
        })
    );
    assert_eq!(
        session.file_at(safety, "b.md").await.unwrap(),
        Some(FileContents::Regular {
            data: b"new b".to_vec(),
            executable: false
        })
    );
    drop(session);
    fs::write(binding.root.join("a.md"), "later edit").unwrap();
    let mut session = Session::open(binding.clone()).await.unwrap();
    assert_eq!(
        session
            .restore_vault(&request, &[])
            .await
            .unwrap()
            .operation,
        result.operation
    );
    assert_eq!(
        fs::read_to_string(binding.root.join("a.md")).unwrap(),
        "later edit"
    );
    assert!(!binding.root.join(".jj").exists());
}
#[tokio::test]
async fn stale_vault_preview_and_interrupted_restore_preserve_newer_edits() {
    let (_temp, binding) = setup();
    fs::write(binding.root.join("a.md"), "old a").unwrap();
    fs::write(binding.root.join("b.md"), "old b").unwrap();
    let mut session = Session::open(binding.clone()).await.unwrap();
    let first = session
        .capture(&[], "First")
        .await
        .unwrap()
        .checkpoint
        .unwrap();
    fs::write(binding.root.join("a.md"), "new a").unwrap();
    fs::write(binding.root.join("b.md"), "new b").unwrap();
    let preview = session
        .preview_vault_restore(&first.commit, &[])
        .await
        .unwrap();
    let mut request = VaultRestoreRequest {
        request_id: "all-2".into(),
        checkpoint: first.commit.clone(),
        expected: preview.expected,
    };
    fs::write(binding.root.join("a.md"), "another a").unwrap();
    assert!(session.restore_vault(&request, &[]).await.is_err());
    assert_eq!(
        fs::read_to_string(binding.root.join("a.md")).unwrap(),
        "another a"
    );
    request.expected = session
        .preview_vault_restore(&first.commit, &[])
        .await
        .unwrap()
        .expected;
    session.restore_vault(&request, &[]).await.unwrap();
    let path = binding.state.join("vault-restore-receipts/all-2.json");
    let mut receipt: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    receipt["next"] = serde_json::json!(1);
    receipt["result"] = serde_json::Value::Null;
    fs::write(&path, serde_json::to_vec(&receipt).unwrap()).unwrap();
    fs::write(binding.root.join("b.md"), "new b").unwrap();
    drop(session);
    let mut session = Session::open(binding.clone()).await.unwrap();
    // Simulates a process exiting after the first replacement and durable step.
    session.restore_vault(&request, &[]).await.unwrap();
    assert_eq!(
        fs::read_to_string(binding.root.join("b.md")).unwrap(),
        "old b"
    );
    receipt["next"] = serde_json::json!(1);
    fs::write(&path, serde_json::to_vec(&receipt).unwrap()).unwrap();
    fs::write(binding.root.join("b.md"), "later b edit").unwrap();
    assert!(session.restore_vault(&request, &[]).await.is_err());
    assert_eq!(
        fs::read_to_string(binding.root.join("b.md")).unwrap(),
        "later b edit"
    );
}
