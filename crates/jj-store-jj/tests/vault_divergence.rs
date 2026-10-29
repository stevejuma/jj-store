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
async fn immutable_divergence_counts_retained_ancestry_without_snapshotting_or_moving_its_pin() {
    use jj_store_jj::vault_workspace::HistoryReader;
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
    assert!(HistoryReader::open(a.clone()).await.is_err());
    assert!(!a.state.exists());
    fs::write(a.root.join("Base.md"), "Shared base").unwrap();
    sync(&a, &origin, "base-a").await;
    sync(&b, &origin, "base-b").await;
    sync(&a, &origin, "base-a-again").await;
    let mut local = Session::open(a.clone()).await.unwrap();
    fs::write(a.root.join("Local.md"), "Local save").unwrap();
    local.capture(&[], "Local save").await.unwrap();
    drop(local);
    let mut remote = Session::open(b.clone()).await.unwrap();
    for number in 1..=2 {
        fs::write(b.root.join("Remote.md"), format!("Remote save {number}")).unwrap();
        remote.capture(&[], "Remote save").await.unwrap();
    }
    drop(remote);
    sync(&b, &origin, "publish-two").await;
    let transfer = RemoteTransfer::connect_local(a.clone(), &origin, "main")
        .await
        .unwrap();
    let fetched = transfer.fetch().await.unwrap().unwrap();
    let mut local = Session::open(a.clone()).await.unwrap();
    let incoming = local.import_fetched(&fetched).await.unwrap();
    drop(local);
    let reader = HistoryReader::open(a.clone()).await.unwrap();
    let before: Vec<_> = fs::read_dir(a.metadata().join("op_heads/heads"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect();
    let counts = reader.divergence(&incoming.commit).await.unwrap().unwrap();
    assert_eq!((counts.incoming, counts.outgoing), (2, 1));
    assert_eq!(counts.remote_commit, incoming.commit);
    let after: Vec<_> = fs::read_dir(a.metadata().join("op_heads/heads"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect();
    assert_eq!(before, after);
    assert!(!a.root.join("Remote.md").exists());
    assert!(reader.divergence(&"f".repeat(40)).await.is_err());
    let mut local = Session::open(a.clone()).await.unwrap();
    fs::write(a.root.join("Local.md"), "Another local save").unwrap();
    let latest = local
        .capture(&[], "Another local save")
        .await
        .unwrap()
        .checkpoint
        .unwrap();
    drop(local);
    assert_eq!(
        reader.divergence(&incoming.commit).await.unwrap().unwrap(),
        counts
    );
    let fresh = HistoryReader::open(a).await.unwrap();
    let changed = fresh.divergence(&incoming.commit).await.unwrap().unwrap();
    assert_eq!((changed.incoming, changed.outgoing), (2, 2));
    let equal = fresh.divergence(&latest.commit).await.unwrap().unwrap();
    assert_eq!((equal.incoming, equal.outgoing), (0, 0));
}
