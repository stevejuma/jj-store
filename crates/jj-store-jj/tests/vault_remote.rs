#![cfg(feature = "vault-backup")]
use jj_store_jj::vault_workspace::{
    Binding, GitCredentials, PushOutcome, RemoteTarget, RemoteTransfer, Session,
    VaultRestoreRequest,
};
use std::{fs, path::Path, process::Command};
fn git(args: &[&str]) {
    let result = Command::new("git")
        .args(args)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_TERMINAL_PROMPT", "0")
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
}
fn vault(base: &Path, id: &str) -> Binding {
    let root = base.join(id);
    fs::create_dir(&root).unwrap();
    Binding {
        vault_id: id.into(),
        root,
        state: base.join("private").join(id),
    }
}
async fn publish(
    binding: &Binding,
    remote: &RemoteTransfer,
    id: &str,
    before: Option<String>,
) -> PushOutcome {
    let session = Session::open(binding.clone()).await.unwrap();
    let intent = session.prepare_push(remote, id, before).await.unwrap();
    drop(session);
    remote.publish(&intent).await.unwrap()
}
async fn restore(binding: &Binding, remote: &RemoteTransfer, id: &str) -> VaultRestoreRequest {
    let fetched = remote.fetch().await.unwrap().unwrap();
    let mut session = Session::open(binding.clone()).await.unwrap();
    let checkpoint = session.import_fetched(&fetched).await.unwrap();
    assert_eq!(checkpoint.commit, fetched.commit());
    let preview = session
        .preview_vault_restore(&checkpoint.commit, &[])
        .await
        .unwrap();
    let request = VaultRestoreRequest {
        request_id: id.into(),
        checkpoint: checkpoint.commit,
        expected: preview.expected,
    };
    session.restore_imported(&request, &[]).await.unwrap();
    request
}
#[tokio::test]
async fn ordinary_git_backup_restores_files_and_native_history_on_a_fresh_device() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let origin = base.join("origin.git");
    git(&[
        "init",
        "--quiet",
        "--bare",
        "--initial-branch=main",
        origin.to_str().unwrap(),
    ]);
    let a = vault(&base, "a");
    fs::write(a.root.join("Note.md"), "first version").unwrap();
    let mut session = Session::open(a.clone()).await.unwrap();
    let first = session
        .capture(&[], "First save")
        .await
        .unwrap()
        .checkpoint
        .unwrap();
    fs::write(a.root.join("Note.md"), "second version").unwrap();
    let second = session
        .capture(&[], "Second save")
        .await
        .unwrap()
        .checkpoint
        .unwrap();
    drop(session);
    let remote_a = RemoteTransfer::connect_local(a.clone(), &origin, "main")
        .await
        .unwrap();
    assert_eq!(remote_a.observe().await.unwrap(), None);
    assert_eq!(
        publish(&a, &remote_a, "initial-push", None).await,
        PushOutcome::Complete
    );
    assert_eq!(
        remote_a.observe().await.unwrap().as_deref(),
        Some(second.commit.as_str())
    );
    let clone = base.join("ordinary-git");
    git(&[
        "clone",
        "--quiet",
        origin.to_str().unwrap(),
        clone.to_str().unwrap(),
    ]);
    assert_eq!(
        fs::read_to_string(clone.join("Note.md")).unwrap(),
        "second version"
    );
    git(&[
        "-C",
        clone.to_str().unwrap(),
        "cat-file",
        "-e",
        &format!("{}:Note.md", first.commit),
    ]);
    let b = vault(&base, "b");
    let mut session = Session::open(b.clone()).await.unwrap();
    session.capture(&[], "Empty vault").await.unwrap();
    drop(session);
    let remote_b = RemoteTransfer::connect_local(b.clone(), &origin, "main")
        .await
        .unwrap();
    let fetched = remote_b.fetch().await.unwrap().unwrap();
    // A transfer handle does not keep the native capture command lease.
    let mut session = Session::open(b.clone()).await.unwrap();
    assert!(!session.capture(&[], "Still empty").await.unwrap().changed);
    let imported = session.import_fetched(&fetched).await.unwrap();
    assert_eq!(imported.commit, second.commit);
    assert_eq!(imported.change_id, second.change_id);
    assert!(
        !b.root.join("Note.md").exists(),
        "fetch/import cannot change files"
    );
    assert!(
        session
            .file_at(&first.commit, "Note.md")
            .await
            .unwrap()
            .is_some()
    );
    let preview = session
        .preview_vault_restore(&imported.commit, &[])
        .await
        .unwrap();
    let request = VaultRestoreRequest {
        request_id: "fresh-restore".into(),
        checkpoint: imported.commit,
        expected: preview.expected,
    };
    let restored = session.restore_imported(&request, &[]).await.unwrap();
    assert_eq!(
        fs::read_to_string(b.root.join("Note.md")).unwrap(),
        "second version"
    );
    let receipt_path = b.state.join("remote-restore-receipts/fresh-restore.json");
    let mut receipt: serde_json::Value =
        serde_json::from_slice(&fs::read(&receipt_path).unwrap()).unwrap();
    receipt["result"] = serde_json::Value::Null;
    fs::write(&receipt_path, serde_json::to_vec(&receipt).unwrap()).unwrap();
    drop(session);
    let mut session = Session::open(b.clone()).await.unwrap();
    assert_eq!(
        session
            .restore_imported(&request, &[])
            .await
            .unwrap()
            .checkpoint
            .unwrap()
            .commit,
        restored.checkpoint.unwrap().commit
    );
    fs::write(b.root.join("Note.md"), "third version").unwrap();
    session.capture(&[], "Third save").await.unwrap();
    session.restore_imported(&request, &[]).await.unwrap();
    assert_eq!(
        fs::read_to_string(b.root.join("Note.md")).unwrap(),
        "third version",
        "known restore replay preserves later edits"
    );
    let intent = session
        .prepare_push(&remote_b, "second-device-push", Some(second.commit))
        .await
        .unwrap();
    drop(session);
    assert_eq!(
        remote_b.publish(&intent).await.unwrap(),
        PushOutcome::Complete
    );
    assert!(remote_b.pending_push().unwrap().is_none());
    assert_eq!(
        remote_b.publish(&intent).await.unwrap(),
        PushOutcome::Complete
    );
    assert!(!a.root.join(".jj").exists());
    assert!(!b.root.join(".git").exists());
    let fresh = base.join("second-clone");
    git(&[
        "clone",
        "--quiet",
        origin.to_str().unwrap(),
        fresh.to_str().unwrap(),
    ]);
    assert_eq!(
        fs::read_to_string(fresh.join("Note.md")).unwrap(),
        "third version"
    );
}
#[tokio::test]
async fn remote_drift_rejects_publication_and_unknown_intents_are_observed_without_replay() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let origin = base.join("origin.git");
    git(&[
        "init",
        "--quiet",
        "--bare",
        "--initial-branch=main",
        origin.to_str().unwrap(),
    ]);
    let a = vault(&base, "a");
    fs::write(a.root.join("Note.md"), "base").unwrap();
    let mut session = Session::open(a.clone()).await.unwrap();
    let common = session
        .capture(&[], "Base")
        .await
        .unwrap()
        .checkpoint
        .unwrap()
        .commit;
    drop(session);
    let ra = RemoteTransfer::connect_local(a.clone(), &origin, "main")
        .await
        .unwrap();
    assert_eq!(publish(&a, &ra, "seed", None).await, PushOutcome::Complete);
    let b = vault(&base, "b");
    let mut session = Session::open(b.clone()).await.unwrap();
    session.capture(&[], "Empty").await.unwrap();
    drop(session);
    let rb = RemoteTransfer::connect_local(b.clone(), &origin, "main")
        .await
        .unwrap();
    restore(&b, &rb, "restore").await;
    fs::write(a.root.join("Note.md"), "device a").unwrap();
    let mut session = Session::open(a.clone()).await.unwrap();
    let a_head = session
        .capture(&[], "A save")
        .await
        .unwrap()
        .checkpoint
        .unwrap()
        .commit;
    drop(session);
    fs::write(b.root.join("Note.md"), "device b").unwrap();
    let mut session = Session::open(b.clone()).await.unwrap();
    let b_head = session
        .capture(&[], "B save")
        .await
        .unwrap()
        .checkpoint
        .unwrap()
        .commit;
    let stale = session
        .prepare_push(&rb, "stale", Some(common.clone()))
        .await
        .unwrap();
    let following = session
        .prepare_push(&rb, "following", Some(common.clone()))
        .await
        .unwrap();
    let mut fabricated = serde_json::to_value(&following).unwrap();
    fabricated["after"] = serde_json::json!(common);
    fabricated["before"] = serde_json::json!(b_head);
    let fabricated = serde_json::from_value(fabricated).unwrap();
    drop(session);
    assert!(
        rb.publish(&fabricated).await.is_err(),
        "a persisted intent cannot authorize an ancestry rewind"
    );
    assert_eq!(
        publish(&a, &ra, "a-push", Some(common)).await,
        PushOutcome::Complete
    );
    assert_eq!(rb.publish(&stale).await.unwrap(), PushOutcome::Stale);
    assert_eq!(rb.observe().await.unwrap(), Some(a_head.clone()));
    let receipt_path = b.state.join("remote-push-receipts/stale.json");
    let mut receipt: serde_json::Value =
        serde_json::from_slice(&fs::read(&receipt_path).unwrap()).unwrap();
    receipt["outcome"] = serde_json::json!("unknown");
    fs::write(&receipt_path, serde_json::to_vec(&receipt).unwrap()).unwrap();
    fs::write(
        b.state.join("remote-push-receipts/active.json"),
        serde_json::to_vec(&stale).unwrap(),
    )
    .unwrap();
    assert_eq!(rb.publish(&stale).await.unwrap(), PushOutcome::Unknown);
    assert_eq!(rb.publish(&following).await.unwrap(), PushOutcome::Unknown);
    assert!(!b.state.join("remote-push-receipts/following.json").exists());
    assert_eq!(rb.observe().await.unwrap(), Some(a_head));
    assert_eq!(
        fs::read_to_string(b.root.join("Note.md")).unwrap(),
        "device b"
    );
}
#[test]
fn remote_targets_reject_embedded_secrets_and_ref_injection() {
    for url in [
        "http://github.com/a/b",
        "https://token@github.com/a/b",
        "https://github.com/a/b?token=private",
        "https://github.com/a/b#secret",
        "file:///tmp/repo",
    ] {
        assert!(
            RemoteTarget {
                url: url.into(),
                branch: "main".into()
            }
            .validate()
            .is_err()
        );
    }
    for branch in [
        "-main",
        "../main",
        "main:evil",
        "heads//main",
        "main.lock",
        "main\n",
    ] {
        assert!(
            RemoteTarget {
                url: "https://github.com/a/b.git".into(),
                branch: branch.into()
            }
            .validate()
            .is_err()
        );
    }
    assert!(GitCredentials::token("token\nsecret").is_err());
    assert!(
        RemoteTarget {
            url: "https://github.com/a/b.git".into(),
            branch: "backup/notes".into()
        }
        .validate()
        .is_ok()
    );
}

#[tokio::test]
async fn a_stalled_https_child_does_not_hold_the_native_capture_lease() {
    use std::time::Duration;
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let binding = vault(&base, "notes");
    let mut session = Session::open(binding.clone()).await.unwrap();
    session.capture(&[], "Initial").await.unwrap();
    drop(session);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("https://{}/backup.git", listener.local_addr().unwrap());
    let remote = RemoteTransfer::connect(
        binding.clone(),
        RemoteTarget {
            url,
            branch: "main".into(),
        },
        GitCredentials::token("synthetic-token").unwrap(),
    )
    .await
    .unwrap();
    let network = tokio::spawn(async move { remote.observe().await });
    // Accept TCP without completing TLS, keeping the actual Git child pending.
    let (socket, _) = tokio::time::timeout(Duration::from_secs(5), listener.accept())
        .await
        .unwrap()
        .unwrap();
    fs::write(binding.root.join("Note.md"), "saved while remote waits").unwrap();
    let captured = tokio::time::timeout(Duration::from_secs(3), async {
        let mut session = Session::open(binding.clone()).await.unwrap();
        session.capture(&[], "Offline capture").await.unwrap()
    })
    .await
    .unwrap();
    assert!(captured.changed);
    assert!(!network.is_finished());
    drop(socket);
    assert!(
        tokio::time::timeout(Duration::from_secs(5), network)
            .await
            .unwrap()
            .unwrap()
            .is_err()
    );
}
