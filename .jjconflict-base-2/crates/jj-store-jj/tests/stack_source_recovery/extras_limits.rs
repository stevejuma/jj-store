use super::*;
use jj_store_jj::local_browser::Error;

#[tokio::test]
async fn orphan_segment_conflict_fails_before_head_selection_and_exact_bytes_allow_retry() {
    let f = Fixture::new().await;
    let (witness, key) = prepare(&f).await;
    let before = f.state();
    fs::remove_dir_all(f.root().join(".jj/repo/store/extra")).unwrap();
    let coordinator = Coordinator::new(1).unwrap();
    drop(
        coordinator
            .restore_history(f.access(), f.root(), witness.clone(), key.clone())
            .await
            .unwrap()
            .restore_extras()
            .await
            .unwrap(),
    );
    let complete = extras::extra_state(&f.root());
    let heads: Vec<_> = complete
        .iter()
        .filter(|(n, _)| n.starts_with("heads/"))
        .collect();
    assert_eq!(heads.len(), 1);
    let extra = f.root().join(".jj/repo/store/extra");
    fs::remove_file(extra.join(&heads[0].0)).unwrap();
    let (name, bytes) = complete
        .iter()
        .find(|(n, _)| !n.starts_with("heads/"))
        .unwrap();
    fs::write(extra.join(name), "conflicting interrupted segment").unwrap();
    let partial = extras::extra_state(&f.root());
    let recovery = coordinator
        .restore_history(f.access(), f.root(), witness.clone(), key.clone())
        .await
        .unwrap();
    assert!(recovery.restore_extras().await.is_err());
    assert_eq!(extras::extra_state(&f.root()), partial);
    assert_eq!(f.state(), before);
    fs::write(extra.join(name), bytes).unwrap();
    drop(
        coordinator
            .restore_history(f.access(), f.root(), witness, key)
            .await
            .unwrap()
            .restore_extras()
            .await
            .unwrap(),
    );
    assert_eq!(extras::extra_state(&f.root()), complete);
    assert_eq!(f.state(), before);
}

#[tokio::test]
async fn oversized_segment_fails_without_selecting_or_truncating_source_bytes() {
    let f = Fixture::new().await;
    let (witness, key) = prepare(&f).await;
    let before = f.state();
    let extra = f.root().join(".jj/repo/store/extra");
    let selected = extras::extra_state(&f.root());
    let path = extra.join("f".repeat(128));
    let file = fs::File::create(&path).unwrap();
    file.set_len(4 * 1024 * 1024 + 1).unwrap();
    let recovery = Coordinator::new(1)
        .unwrap()
        .restore_history(f.access(), f.root(), witness, key)
        .await
        .unwrap();
    assert!(matches!(recovery.restore_extras().await, Err(Error::Limit)));
    assert_eq!(file.metadata().unwrap().len(), 4 * 1024 * 1024 + 1);
    fs::remove_file(path).unwrap();
    assert_eq!(extras::extra_state(&f.root()), selected);
    assert_eq!(f.state(), before);
}
