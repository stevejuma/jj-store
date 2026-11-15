use super::*;
use std::fs;
#[tokio::test]
async fn immutable_reader_accepts_ancestral_heads_without_changing_the_published_frontier() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().canonicalize().unwrap();
    let binding = Binding {
        vault_id: "readonly-publication".into(),
        root: root.join("notes"),
        state: root.join("private"),
    };
    fs::create_dir(&binding.root).unwrap();
    fs::write(binding.root.join("Note.md"), "first").unwrap();
    let mut session = Session::open(binding.clone()).await.unwrap();
    session.capture(&[], "first").await.unwrap();
    let loader = RepoLoader::init_from_file_system(
        &settings().unwrap(),
        &binding.metadata(),
        &default_backend_factories(),
    )
    .unwrap();
    let old = loader
        .op_heads_store()
        .get_op_heads()
        .await
        .unwrap()
        .pop()
        .unwrap();
    fs::write(binding.root.join("Note.md"), "second").unwrap();
    let second = session
        .capture(&[], "second")
        .await
        .unwrap()
        .checkpoint
        .unwrap();
    // Hold the valid publication window deterministically: new head exists while
    // the previous ancestral head has not yet been removed.
    loader
        .op_heads_store()
        .update_op_heads(&[], &old)
        .await
        .unwrap();
    let mut before = loader.op_heads_store().get_op_heads().await.unwrap();
    before.sort();
    assert_eq!(before.len(), 2);
    fs::write(binding.root.join("Note.md"), "unsnapshotted draft").unwrap();
    let reader = HistoryReader::open(binding.clone()).await.unwrap();
    assert_eq!(
        reader
            .history_revision(None, 0)
            .await
            .unwrap()
            .revision
            .unwrap()
            .checkpoint
            .commit,
        second.commit
    );
    assert_eq!(
        reader
            .history_files(&second.commit, None, &[])
            .await
            .unwrap()
            .files[0]
            .text
            .as_deref(),
        Some("second")
    );
    let mut after = loader.op_heads_store().get_op_heads().await.unwrap();
    after.sort();
    assert_eq!(
        after, before,
        "a read must not repair or merge operation heads"
    );
    assert_eq!(
        fs::read_to_string(binding.root.join("Note.md")).unwrap(),
        "unsnapshotted draft"
    );
    let ancestor = loader
        .load_at(&loader.load_operation(&old).await.unwrap())
        .await
        .unwrap();
    let fork = ancestor
        .start_transaction()
        .commit("independent divergent operation")
        .await
        .unwrap();
    assert_ne!(fork.op_id(), session.repo.op_id());
    let mut divergent = loader.op_heads_store().get_op_heads().await.unwrap();
    divergent.sort();
    assert_eq!(divergent.len(), 2);
    assert!(
        HistoryReader::open(binding).await.is_err(),
        "genuine divergent operations remain unavailable"
    );
    let mut unchanged = loader.op_heads_store().get_op_heads().await.unwrap();
    unchanged.sort();
    assert_eq!(
        unchanged, divergent,
        "a refused read must not merge divergent operations"
    );
}
