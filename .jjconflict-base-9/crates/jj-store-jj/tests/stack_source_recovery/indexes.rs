use super::*;
use jj_lib::{
    config::StackedConfig, default_index::DefaultIndexStore, index::IndexStore as _,
    operation::Operation, settings::UserSettings,
};
use std::sync::Arc;

pub(super) fn index_state(root: &std::path::Path) -> Vec<(String, Vec<u8>)> {
    let mut out = vec![];
    let index = root.join(".jj/repo/index");
    if !index.exists() {
        return out;
    }
    for e in fs::read_dir(&index).unwrap() {
        let e = e.unwrap();
        let name = e.file_name().to_str().unwrap().to_owned();
        if e.file_type().unwrap().is_dir() {
            for f in fs::read_dir(e.path()).unwrap() {
                let f = f.unwrap();
                out.push((
                    format!("{name}/{}", f.file_name().to_str().unwrap()),
                    fs::read(f.path()).unwrap(),
                ));
            }
        } else {
            out.push((name, fs::read(e.path()).unwrap()));
        }
    }
    out.sort();
    out
}

#[tokio::test]
async fn saved_original_and_candidate_indexes_restore_without_future_visibility_or_source_import() {
    let f = Fixture::new().await;
    let (witness, key) = prepare(&f).await;
    let binding = witness.binding();
    let archive = MetadataStore::open(&f.root().join(".git"), Limits::default())
        .unwrap()
        .read_stack_prepared(&key)
        .unwrap()
        .unwrap()
        .history
        .unwrap();
    let prepared = archive.views[&archive.operations[&archive.operation].view_id].wc_commit_ids
        [&binding.workspace];
    assert_ne!(prepared, binding.commit);
    let before = f.state();
    fs::remove_dir_all(f.root().join(".jj/repo/op_store")).unwrap();
    fs::remove_dir_all(f.root().join(".jj/repo/index")).unwrap();
    fs::remove_dir_all(f.root().join(".jj/repo/store/extra")).unwrap();
    let recovery = Coordinator::new(1)
        .unwrap()
        .restore_history(f.access(), f.root(), witness.clone(), key.clone())
        .await
        .unwrap();
    // Index construction is archive-only, even with no original extras present.
    let recovery = recovery.restore_indexes().await.unwrap();
    assert!(!f.root().join(".jj/repo/store/extra").exists());
    let recovery = recovery.restore_extras().await.unwrap();
    let extras = extras::extra_state(&f.root());
    let indexes = index_state(&f.root());
    assert!(!indexes.is_empty());
    let settings = UserSettings::from_config(StackedConfig::with_defaults()).unwrap();
    let loader = jj_lib::repo::RepoLoader::init_from_file_system(
        &settings,
        &f.root().join(".jj/repo"),
        &jj_lib::default_backend_factories::default_backend_factories(),
    )
    .unwrap();
    let source = SimpleOpStore::load(
        &f.root().join(".jj/repo/op_store"),
        jj_lib::op_store::RootOperationData {
            root_commit_id: jj_lib::backend::CommitId::from_bytes(&[0; 20]),
        },
    );
    let store = Arc::new(source);
    for (id, future) in [(binding.operation, false), (archive.operation, true)] {
        let native = jj_lib::op_store::OperationId::from_bytes(id.as_bytes());
        let operation = Operation::new(
            store.clone(),
            native.clone(),
            store.read_operation(&native).await.unwrap(),
        );
        let index = DefaultIndexStore::load(&f.root().join(".jj/repo/index"))
            .get_index_at_op(&operation, loader.store())
            .await
            .unwrap();
        assert!(
            index
                .as_index()
                .has_id(&jj_lib::backend::CommitId::from_bytes(
                    binding.commit.as_bytes()
                ))
                .await
                .unwrap()
        );
        assert_eq!(
            index
                .as_index()
                .has_id(&jj_lib::backend::CommitId::from_bytes(prepared.as_bytes()))
                .await
                .unwrap(),
            future
        );
    }
    assert_eq!(extras::extra_state(&f.root()), extras);
    assert_eq!(index_state(&f.root()), indexes);
    assert_eq!(f.state(), before);
    drop(recovery);
    drop(
        Coordinator::new(1)
            .unwrap()
            .restore_history(f.access(), f.root(), witness, key)
            .await
            .unwrap()
            .restore_indexes()
            .await
            .unwrap(),
    );
    assert_eq!(index_state(&f.root()), indexes);
    assert_eq!(f.state(), before);
}

#[tokio::test]
async fn conflicting_existing_index_association_fails_before_missing_segments_are_selected() {
    let f = Fixture::new().await;
    let (witness, key) = prepare(&f).await;
    let before = f.state();
    fs::remove_dir_all(f.root().join(".jj/repo/index")).unwrap();
    let index = f.root().join(".jj/repo/index");
    fs::create_dir_all(index.join("op_links")).unwrap();
    fs::write(index.join("type"), "default").unwrap();
    fs::write(
        index
            .join("op_links")
            .join(witness.binding().operation.to_string()),
        "conflicting association",
    )
    .unwrap();
    let state = index_state(&f.root());
    let recovery = Coordinator::new(1)
        .unwrap()
        .restore_history(f.access(), f.root(), witness, key)
        .await
        .unwrap();
    assert!(recovery.restore_indexes().await.is_err());
    assert_eq!(index_state(&f.root()), state);
    assert_eq!(f.state(), before);
}

#[tokio::test]
async fn valid_existing_cli_index_bytes_are_preserved_while_missing_candidate_is_added() {
    let f = Fixture::new().await;
    let (witness, key) = prepare(&f).await;
    let physical = f.state();
    let original = index_state(&f.root());
    let recovery = Coordinator::new(1)
        .unwrap()
        .restore_history(f.access(), f.root(), witness, key)
        .await
        .unwrap();
    drop(recovery.restore_indexes().await.unwrap());
    let complete = index_state(&f.root());
    for entry in original {
        assert!(complete.contains(&entry));
    }
    assert_eq!(f.state(), physical);
}

#[tokio::test]
async fn valid_candidate_index_cannot_substitute_for_the_original_operation_graph() {
    let f = Fixture::new().await;
    let (witness, key) = prepare(&f).await;
    let archive = MetadataStore::open(&f.root().join(".git"), Limits::default())
        .unwrap()
        .read_stack_prepared(&key)
        .unwrap()
        .unwrap()
        .history
        .unwrap();
    fs::remove_dir_all(f.root().join(".jj/repo/index")).unwrap();
    drop(
        Coordinator::new(1)
            .unwrap()
            .restore_history(f.access(), f.root(), witness.clone(), key.clone())
            .await
            .unwrap()
            .restore_indexes()
            .await
            .unwrap(),
    );
    let links = f.root().join(".jj/repo/index/op_links");
    let candidate = fs::read(links.join(archive.operation.to_string())).unwrap();
    fs::write(
        links.join(witness.binding().operation.to_string()),
        candidate,
    )
    .unwrap();
    let before = index_state(&f.root());
    let physical = f.state();
    let recovery = Coordinator::new(1)
        .unwrap()
        .restore_history(f.access(), f.root(), witness, key)
        .await
        .unwrap();
    assert!(recovery.restore_indexes().await.is_err());
    assert_eq!(index_state(&f.root()), before);
    assert_eq!(f.state(), physical);
}
