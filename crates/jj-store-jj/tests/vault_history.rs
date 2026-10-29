#![cfg(feature = "vault-backup")]
use jj_store_jj::vault_workspace::{Binding, Session};
use std::fs;
#[tokio::test]
async fn history_reads_pin_saved_versions_and_page_changed_text_without_snapshots() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let binding = Binding {
        vault_id: "history-search".into(),
        root: base.join("notes"),
        state: base.join("private/history"),
    };
    fs::create_dir(&binding.root).unwrap();
    fs::write(
        binding.root.join("Note.md"),
        "lost historical phrase\nsecond line",
    )
    .unwrap();
    for n in 0..12 {
        fs::write(
            binding.root.join(format!("page-{n:02}.md")),
            format!("page {n}"),
        )
        .unwrap();
    }
    fs::write(binding.root.join("binary.bin"), [0, 1, 2]).unwrap();
    fs::write(binding.root.join("large.md"), "x".repeat(1024 * 1024 + 1)).unwrap();
    let mut session = Session::open(binding.clone()).await.unwrap();
    let first = session
        .capture(&[], "First historical checkpoint")
        .await
        .unwrap()
        .checkpoint
        .unwrap();
    fs::write(binding.root.join("Note.md"), "replacement phrase").unwrap();
    let second = session
        .capture(&[], "Second checkpoint")
        .await
        .unwrap()
        .checkpoint
        .unwrap();
    // A saved edit not yet captured must never leak into an immutable query.
    fs::write(binding.root.join("Note.md"), "unsnapshotted edit").unwrap();
    let operation = session.operation();
    let independent = jj_store_jj::vault_workspace::HistoryReader::open(binding.clone())
        .await
        .unwrap();
    assert!(
        independent
            .history_revision(None, 0)
            .await
            .unwrap()
            .revision
            .is_some(),
        "immutable search must not wait for the writer lease"
    );
    let page = session.history_revision(None, 0).await.unwrap();
    assert_eq!(page.anchor.as_deref(), Some(second.commit.as_str()));
    let current = page.revision.unwrap();
    assert_eq!(current.checkpoint.commit, second.commit);
    assert_eq!(current.author, "Lapis Notes");
    let files = session
        .history_files(&second.commit, None, &[])
        .await
        .unwrap();
    assert_eq!(files.files.len(), 1);
    assert_eq!(files.files[0].text.as_deref(), Some("replacement phrase"));
    assert_eq!(
        session
            .history_revision(Some(&second.commit), 1)
            .await
            .unwrap()
            .revision
            .unwrap()
            .checkpoint
            .commit,
        first.commit
    );
    let mut after = None;
    let mut found = vec![];
    loop {
        let page = session
            .history_files(&first.commit, after.as_deref(), &[])
            .await
            .unwrap();
        found.extend(page.files);
        after = page.next;
        if after.is_none() {
            break;
        }
    }
    assert_eq!(found.len(), 15);
    assert_eq!(
        found
            .iter()
            .find(|f| f.path == "Note.md")
            .unwrap()
            .text
            .as_deref(),
        Some("lost historical phrase\nsecond line")
    );
    assert!(
        found
            .iter()
            .find(|f| f.path == "binary.bin")
            .unwrap()
            .unindexed
    );
    assert!(
        found
            .iter()
            .find(|f| f.path == "large.md")
            .unwrap()
            .unindexed
    );
    assert!(
        session
            .history_files(&first.commit, None, &["Note.md".into()])
            .await
            .unwrap()
            .files
            .iter()
            .all(|f| f.path != "Note.md")
    );
    assert!(
        session
            .history_revision(Some(&"f".repeat(40)), 0)
            .await
            .is_err()
    );
    assert_eq!(session.operation(), operation);
    assert_eq!(
        fs::read_to_string(binding.root.join("Note.md")).unwrap(),
        "unsnapshotted edit"
    );
    assert!(!binding.root.join(".jj").exists());
}
