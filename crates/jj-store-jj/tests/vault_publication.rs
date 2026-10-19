#![cfg(feature = "vault-backup")]
use jj_store_jj::vault_workspace::{Binding, PushOutcome, RemoteTransfer, Session};
use std::{fs, path::Path, process::Command};
async fn setup(base: &Path) -> (Binding, RemoteTransfer) {
    let root = base.join("notes");
    fs::create_dir(&root).unwrap();
    let binding = Binding {
        vault_id: "vault".into(),
        root,
        state: base.join("private/history"),
    };
    fs::write(binding.root.join("Note.md"), "retained local data").unwrap();
    let mut session = Session::open(binding.clone()).await.unwrap();
    session.capture(&[], "Original").await.unwrap();
    drop(session);
    let origin = base.join("origin.git");
    assert!(
        Command::new("git")
            .args(["init", "--bare", "--quiet"])
            .arg(&origin)
            .status()
            .unwrap()
            .success()
    );
    let remote = RemoteTransfer::connect_local(binding.clone(), &origin, "main")
        .await
        .unwrap();
    (binding, remote)
}
fn unknown(binding: &Binding, request: &str) {
    let path = binding
        .state
        .join("remote-push-receipts")
        .join(format!("{request}.json"));
    let mut value: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    value["outcome"] = serde_json::json!("unknown");
    fs::write(path, serde_json::to_vec(&value).unwrap()).unwrap();
}
#[tokio::test]
async fn explicit_stop_preserves_the_unknown_receipt_and_never_replays_or_clears_a_newer_push() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let (binding, remote) = setup(&base).await;
    let session = Session::open(binding.clone()).await.unwrap();
    let original = session
        .prepare_push(&remote, "original", None)
        .await
        .unwrap();
    drop(session);
    remote.reserve(&original).unwrap();
    unknown(&binding, "original");
    let pending = binding.pending_publication().unwrap().unwrap();
    assert_eq!(pending.request_id, "original");
    assert_eq!(pending.outcome, PushOutcome::Unknown);
    let text = serde_json::to_string(&pending).unwrap();
    assert!(!text.contains(base.to_str().unwrap()));
    assert!(!text.contains("binding"));
    assert_eq!(
        remote.reconcile("original").await.unwrap(),
        PushOutcome::Unknown
    );
    assert_eq!(remote.observe().await.unwrap(), None);
    assert!(binding.stop_publication("another-request").is_err());
    assert_eq!(
        binding.stop_publication("original").unwrap().outcome,
        PushOutcome::Stopped
    );
    assert!(binding.pending_publication().unwrap().is_none());
    assert_eq!(
        remote.publish(&original).await.unwrap(),
        PushOutcome::Stopped
    );
    assert_eq!(remote.observe().await.unwrap(), None);
    let session = Session::open(binding.clone()).await.unwrap();
    let next = session.prepare_push(&remote, "next", None).await.unwrap();
    drop(session);
    remote.reserve(&next).unwrap();
    assert_eq!(
        binding.stop_publication("original").unwrap().outcome,
        PushOutcome::Stopped
    );
    assert_eq!(
        remote.publish(&original).await.unwrap(),
        PushOutcome::Stopped
    );
    assert_eq!(remote.pending_push().unwrap(), Some(next.clone()));
    assert_eq!(remote.publish(&next).await.unwrap(), PushOutcome::Complete);
    assert_eq!(
        fs::read_to_string(binding.root.join("Note.md")).unwrap(),
        "retained local data"
    );
    let receipt: serde_json::Value = serde_json::from_slice(
        &fs::read(binding.state.join("remote-push-receipts/original.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(receipt["outcome"], "stopped");
}
#[tokio::test]
async fn observation_confirms_acceptance_without_publication_and_pending_recovery_follows_a_root_relink()
 {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let (binding, remote) = setup(&base).await;
    let session = Session::open(binding.clone()).await.unwrap();
    let original = session
        .prepare_push(&remote, "accepted", None)
        .await
        .unwrap();
    drop(session);
    assert_eq!(
        remote.publish(&original).await.unwrap(),
        PushOutcome::Complete
    );
    remote.reserve(&original).unwrap();
    unknown(&binding, "accepted");
    fs::write(
        binding.state.join("remote-push-receipts/active.json"),
        serde_json::to_vec(&original).unwrap(),
    )
    .unwrap();
    let mut moved = binding.clone();
    moved.root = base.join("moved");
    fs::rename(&binding.root, &moved.root).unwrap();
    moved.relink().unwrap();
    assert_eq!(
        moved.pending_publication().unwrap().unwrap().request_id,
        "accepted"
    );
    let current = RemoteTransfer::connect_local(moved.clone(), &base.join("origin.git"), "main")
        .await
        .unwrap();
    let before = current.observe().await.unwrap();
    assert_eq!(
        current.reconcile("accepted").await.unwrap(),
        PushOutcome::Complete
    );
    assert_eq!(current.observe().await.unwrap(), before);
    assert!(moved.pending_publication().unwrap().is_none());
    assert_eq!(
        moved.stop_publication("accepted").unwrap().outcome,
        PushOutcome::Complete
    );
}
