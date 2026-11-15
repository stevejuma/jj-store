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
async fn initialized_transfer_does_not_take_the_capture_lease() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let (binding, _) = setup(&base).await;
    let mut capture = Session::open(binding.clone()).await.unwrap();
    let operation = capture.operation();
    let transfer = RemoteTransfer::connect_local(binding.clone(), &base.join("origin.git"), "main")
        .await
        .unwrap();
    let reader = jj_store_jj::vault_workspace::HistoryReader::open(binding.clone())
        .await
        .unwrap();
    let review = reader
        .prepare_push(&transfer, "immutable-review", None)
        .await
        .unwrap();
    assert_eq!(reader.push_divergence(&review).await.unwrap().outgoing, 1);
    assert_eq!(transfer.observe().await.unwrap(), None);
    assert_eq!(capture.operation(), operation);
    fs::write(binding.root.join("Note.md"), "later captured checkpoint").unwrap();
    let later = capture
        .capture(&[], "Later capture")
        .await
        .unwrap()
        .checkpoint
        .unwrap();
    assert_ne!(later.commit, review.commit());
    assert_eq!(reader.push_divergence(&review).await.unwrap().outgoing, 1);
    assert_eq!(
        reader
            .prepare_push(&transfer, "same-pinned-reader", None)
            .await
            .unwrap()
            .commit(),
        review.commit()
    );
    assert_eq!(
        transfer.publish(&review).await.unwrap(),
        PushOutcome::Complete
    );
    assert_eq!(
        transfer.observe().await.unwrap().as_deref(),
        Some(review.commit())
    );
    assert!(
        Session::open(binding).await.is_err(),
        "mutation still requires its exclusive capture lease"
    );
}

#[tokio::test]
async fn retained_publication_does_not_take_the_capture_lease() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let (binding, transfer) = setup(&base).await;
    let capture = Session::open(binding.clone()).await.unwrap();
    let intent = capture
        .prepare_push(&transfer, "capture-independent-push", None)
        .await
        .unwrap();
    let operation = capture.operation();
    fs::write(
        binding.root.join("Unsnapshotted.md"),
        "must remain unpublished",
    )
    .unwrap();
    assert_eq!(
        transfer.publish(&intent).await.unwrap(),
        PushOutcome::Complete
    );
    assert_eq!(
        transfer.observe().await.unwrap().as_deref(),
        Some(intent.commit())
    );
    assert_eq!(capture.operation(), operation);
    assert_eq!(
        fs::read_to_string(binding.root.join("Unsnapshotted.md")).unwrap(),
        "must remain unpublished"
    );
    assert!(
        !Command::new("git")
            .arg("--git-dir")
            .arg(base.join("origin.git"))
            .args([
                "cat-file",
                "-e",
                &format!("{}:Unsnapshotted.md", intent.commit())
            ])
            .output()
            .unwrap()
            .status
            .success()
    );
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
