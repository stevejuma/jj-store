#![cfg(feature = "vault-backup")]
use jj_store_jj::vault_workspace::{
    Binding, PushOutcome, RemoteTransfer, Session, VaultRestoreRequest,
};
use std::{fs, path::Path, process::Command};
fn vault(base: &Path, id: &str) -> Binding {
    let root = base.join(id);
    fs::create_dir(&root).unwrap();
    Binding {
        vault_id: id.into(),
        root,
        state: base.join("private").join(id),
    }
}
async fn sync(binding: &Binding, origin: &Path, request: &str) {
    let transfer = RemoteTransfer::connect_local(binding.clone(), origin, "main")
        .await
        .unwrap();
    let fetched = transfer.fetch().await.unwrap();
    let before = fetched.as_ref().map(|h| h.commit().to_owned());
    let mut session = Session::open(binding.clone()).await.unwrap();
    session.capture(&[], "Local save").await.unwrap();
    if let Some(fetched) = fetched {
        let incoming = session.import_fetched(&fetched).await.unwrap();
        let proposal = session.prepare_sync(&incoming.commit, &[]).await.unwrap();
        assert!(proposal.conflicts.is_empty());
        if let Some(candidate) = proposal.candidate {
            let preview = session.preview_sync(&candidate.commit, &[]).await.unwrap();
            session
                .apply_sync(
                    &VaultRestoreRequest {
                        request_id: request.into(),
                        checkpoint: candidate.commit,
                        expected: preview.expected,
                    },
                    &[],
                )
                .await
                .unwrap();
        }
    }
    let intent = session
        .prepare_push(&transfer, request, before)
        .await
        .unwrap();
    drop(session);
    assert_eq!(
        transfer.publish(&intent).await.unwrap(),
        PushOutcome::Complete
    );
}
#[tokio::test]
async fn two_devices_merge_offline_edits_and_preserve_git_ancestry() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let origin = base.join("origin.git");
    assert!(
        Command::new("git")
            .args(["init", "--bare", "--quiet"])
            .arg(&origin)
            .status()
            .unwrap()
            .success()
    );
    let a = vault(&base, "a");
    let b = vault(&base, "b");
    fs::write(a.root.join("Note.md"), "heading\nbase\nfooter\n").unwrap();
    sync(&a, &origin, "a-initial").await;
    sync(&b, &origin, "b-initial").await;
    fs::write(a.root.join("Note.md"), "new heading\nbase\nfooter\n").unwrap();
    fs::write(b.root.join("Other.md"), "offline device b\n").unwrap();
    sync(&a, &origin, "a-later").await;
    sync(&b, &origin, "b-later").await;
    let observer = RemoteTransfer::connect_local(a.clone(), &origin, "main")
        .await
        .unwrap();
    let converged = observer.observe().await.unwrap();
    sync(&a, &origin, "a-pull").await;
    assert_eq!(
        observer.observe().await.unwrap(),
        converged,
        "pulling a descendant must not generate another merge and bounce between devices"
    );
    assert_eq!(
        fs::read_to_string(a.root.join("Other.md")).unwrap(),
        "offline device b\n"
    );
    assert_eq!(
        fs::read_to_string(b.root.join("Note.md")).unwrap(),
        "new heading\nbase\nfooter\n"
    );
    let ra = RemoteTransfer::connect_local(a.clone(), &origin, "main")
        .await
        .unwrap();
    let mut session = Session::open(a.clone()).await.unwrap();
    let current = session.checkpoint().unwrap().unwrap();
    let fetched = ra.fetch().await.unwrap().unwrap();
    let incoming = session.import_fetched(&fetched).await.unwrap();
    assert!(
        session
            .prepare_sync(&incoming.commit, &[])
            .await
            .unwrap()
            .candidate
            .is_none()
    );
    assert_eq!(
        session.checkpoint().unwrap().unwrap().commit,
        current.commit
    );
    assert!(!a.root.join(".jj").exists());
}
#[tokio::test]
async fn conflicts_are_retained_without_touching_files_and_stale_sync_cannot_overwrite_edits() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let origin = base.join("origin.git");
    assert!(
        Command::new("git")
            .args(["init", "--bare", "--quiet"])
            .arg(&origin)
            .status()
            .unwrap()
            .success()
    );
    let a = vault(&base, "a");
    let b = vault(&base, "b");
    fs::write(a.root.join("Note.md"), "base\n").unwrap();
    sync(&a, &origin, "seed").await;
    sync(&b, &origin, "restore").await;
    fs::write(a.root.join("Note.md"), "device a\n").unwrap();
    fs::write(b.root.join("Note.md"), "device b\n").unwrap();
    sync(&a, &origin, "a-edit").await;
    let remote = RemoteTransfer::connect_local(b.clone(), &origin, "main")
        .await
        .unwrap();
    let fetched = remote.fetch().await.unwrap().unwrap();
    let mut session = Session::open(b.clone()).await.unwrap();
    let incoming = session.import_fetched(&fetched).await.unwrap();
    let proposal = session.prepare_sync(&incoming.commit, &[]).await.unwrap();
    assert_eq!(proposal.conflicts, vec!["Note.md"]);
    let candidate = proposal.candidate.unwrap();
    assert!(session.preview_sync(&candidate.commit, &[]).await.is_err());
    assert_eq!(
        fs::read_to_string(b.root.join("Note.md")).unwrap(),
        "device b\n"
    );
    let details = session
        .sync_conflict(&candidate.commit, "Note.md")
        .await
        .unwrap();
    assert_eq!(details.terms.len(), 3);
    let resolved = session
        .resolve_sync(
            &candidate.commit,
            "Note.md",
            jj_store_jj::vault_workspace::SyncResolution::Text {
                text: "merged a and b\n".into(),
                executable: false,
            },
            &[],
        )
        .await
        .unwrap();
    let preview = session.preview_sync(&resolved.commit, &[]).await.unwrap();
    let request = VaultRestoreRequest {
        request_id: "resolve".into(),
        checkpoint: resolved.commit.clone(),
        expected: preview.expected,
    };
    fs::write(b.root.join("Note.md"), "later local edit\n").unwrap();
    assert!(session.apply_sync(&request, &[]).await.is_err());
    assert_eq!(
        fs::read_to_string(b.root.join("Note.md")).unwrap(),
        "later local edit\n"
    );
    fs::write(b.root.join("Note.md"), "device b\n").unwrap();
    session.apply_sync(&request, &[]).await.unwrap();
    fs::write(b.root.join("Note.md"), "after known result\n").unwrap();
    session.apply_sync(&request, &[]).await.unwrap();
    assert_eq!(
        fs::read_to_string(b.root.join("Note.md")).unwrap(),
        "after known result\n"
    );
    let intent = session
        .prepare_push(&remote, "merged-push", Some(incoming.commit))
        .await
        .unwrap();
    drop(session);
    remote.reserve(&intent).unwrap();
    assert_eq!(remote.pending_push().unwrap(), Some(intent.clone()));
    assert_eq!(
        remote.publish(&intent).await.unwrap(),
        PushOutcome::Complete
    );
}
#[tokio::test]
async fn unknown_push_is_confirmed_when_the_remote_has_advanced_to_its_descendant() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let origin = base.join("origin.git");
    assert!(
        Command::new("git")
            .args(["init", "--bare", "--quiet"])
            .arg(&origin)
            .status()
            .unwrap()
            .success()
    );
    let a = vault(&base, "a");
    let b = vault(&base, "b");
    fs::write(a.root.join("Note.md"), "base").unwrap();
    let mut session = Session::open(a.clone()).await.unwrap();
    session.capture(&[], "seed").await.unwrap();
    drop(session);
    let remote = RemoteTransfer::connect_local(a.clone(), &origin, "main")
        .await
        .unwrap();
    let session = Session::open(a.clone()).await.unwrap();
    let intent = session
        .prepare_push(&remote, "original", None)
        .await
        .unwrap();
    drop(session);
    remote.reserve(&intent).unwrap();
    assert_eq!(remote.pending_push().unwrap(), Some(intent.clone()));
    assert_eq!(
        remote.publish(&intent).await.unwrap(),
        PushOutcome::Complete
    );
    sync(&b, &origin, "b-descendant").await;
    let path = a.state.join("remote-push-receipts/original.json");
    let mut receipt: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    receipt["outcome"] = serde_json::json!("unknown");
    fs::write(&path, serde_json::to_vec(&receipt).unwrap()).unwrap();
    fs::write(
        a.state.join("remote-push-receipts/active.json"),
        serde_json::to_vec(&intent).unwrap(),
    )
    .unwrap();
    let before = remote.observe().await.unwrap();
    assert_eq!(
        remote.publish(&intent).await.unwrap(),
        PushOutcome::Complete
    );
    assert_eq!(remote.observe().await.unwrap(), before);
    assert!(remote.pending_push().unwrap().is_none());
}
