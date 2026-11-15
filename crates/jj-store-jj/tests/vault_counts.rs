#![cfg(feature = "vault-backup")]
use jj_store_jj::vault_workspace::{Binding, HistoryReader, RemoteTarget, RemoteTransfer, Session};
use std::fs;

#[tokio::test]
async fn empty_remote_counts_exclude_the_native_root_and_never_capture_unsaved_files() {
    let temporary = tempfile::tempdir().unwrap();
    let base = temporary.path().canonicalize().unwrap();
    let root = base.join("notes");
    fs::create_dir(&root).unwrap();
    let binding = Binding {
        vault_id: "counts".into(),
        root: root.clone(),
        state: base.join("private/counts"),
    };
    drop(Session::open(binding.clone()).await.unwrap());
    assert!(
        HistoryReader::open(binding.clone())
            .await
            .unwrap()
            .checkpoint_divergence(None)
            .await
            .unwrap()
            .is_none()
    );
    let mut session = Session::open(binding.clone()).await.unwrap();
    fs::write(root.join("Note.md"), "recorded bytes").unwrap();
    let checkpoint = session
        .capture(&[], "First retained version")
        .await
        .unwrap()
        .checkpoint
        .unwrap();
    drop(session);
    fs::write(root.join("Unsaved.md"), "never captured by a count").unwrap();
    let reader = HistoryReader::open(binding.clone()).await.unwrap();
    let counts = reader.checkpoint_divergence(None).await.unwrap().unwrap();
    assert_eq!(counts.local_commit, checkpoint.commit);
    assert_eq!(counts.remote_commit, None);
    assert_eq!((counts.incoming, counts.outgoing), (0, 1));
    assert_eq!(
        fs::read_to_string(root.join("Unsaved.md")).unwrap(),
        "never captured by a count"
    );
    assert_eq!(
        reader.checkpoint_divergence(None).await.unwrap().unwrap(),
        counts
    );
    assert!(
        reader
            .checkpoint_divergence(Some(&"f".repeat(40)))
            .await
            .is_err()
    );
}

#[tokio::test]
async fn push_counts_keep_the_reviewed_commit_when_a_later_capture_advances() {
    let temporary = tempfile::tempdir().unwrap();
    let base = temporary.path().canonicalize().unwrap();
    let root = base.join("notes");
    fs::create_dir(&root).unwrap();
    let binding = Binding {
        vault_id: "review-counts".into(),
        root: root.clone(),
        state: base.join("private/counts"),
    };
    let mut session = Session::open(binding.clone()).await.unwrap();
    fs::write(root.join("Note.md"), "first recorded bytes").unwrap();
    let checkpoint = session
        .capture(&[], "First retained version")
        .await
        .unwrap()
        .checkpoint
        .unwrap();
    drop(session);
    let transfer = RemoteTransfer::connect_anonymous(
        binding.clone(),
        RemoteTarget {
            url: "https://unreachable.invalid/review.git".into(),
            branch: "main".into(),
        },
    )
    .await
    .unwrap();
    let session = Session::open(binding.clone()).await.unwrap();
    let intent = session
        .prepare_push(&transfer, "immutable-counts", None)
        .await
        .unwrap();
    let reviewed = session.push_divergence(&intent).await.unwrap();
    assert_eq!(intent.commit(), checkpoint.commit);
    assert_eq!(intent.request_id(), "immutable-counts");
    assert_eq!(intent.remote_commit(), None);
    assert_eq!((reviewed.incoming, reviewed.outgoing), (0, 1));
    drop(session);
    let mut session = Session::open(binding.clone()).await.unwrap();
    fs::write(root.join("Note.md"), "a later recorded version").unwrap();
    session
        .capture(&[], "Later retained version")
        .await
        .unwrap();
    assert_eq!(session.push_divergence(&intent).await.unwrap(), reviewed);
    drop(session);
    let current = HistoryReader::open(binding)
        .await
        .unwrap()
        .checkpoint_divergence(None)
        .await
        .unwrap()
        .unwrap();
    assert_eq!((current.incoming, current.outgoing), (0, 2));
    assert_ne!(current.local_commit, intent.commit());
}
