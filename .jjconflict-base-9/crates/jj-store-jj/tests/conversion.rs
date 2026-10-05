//! Real GitBackend and native codec roundtrip, without source head or checkout writes.
use jj_lib::{
    config::StackedConfig, default_backend_factories::default_backend_factories, repo::RepoLoader,
    settings::UserSettings,
};
use jj_store_jj::{conversion::*, objects::NativeObjects};
use jj_store_model as m;
use std::{fs, path::Path, process::Command, sync::Arc};
const SOURCE: m::RepositoryId = m::RepositoryId::new([91; 16]);
const DEST: m::RepositoryId = m::RepositoryId::new([92; 16]);
fn settings() -> UserSettings {
    UserSettings::from_config(StackedConfig::with_defaults()).unwrap()
}
fn stock(path: &Path, args: &[&str]) -> String {
    let out = Command::new("/opt/homebrew/bin/jj")
        .current_dir(path)
        .args([
            "--no-pager",
            "--config",
            "user.name='Fixture'",
            "--config",
            "user.email='fixture@invalid'",
        ])
        .args(args)
        .env("JJ_CONFIG", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}
fn fixture() -> (tempfile::TempDir, m::OperationId) {
    let root = tempfile::tempdir().unwrap();
    stock(root.path(), &["git", "init", "--colocate"]);
    fs::write(root.path().join("file.txt"), "first\n").unwrap();
    stock(root.path(), &["describe", "-m", "First"]);
    stock(root.path(), &["bookmark", "create", "main"]);
    stock(root.path(), &["new", "-m", "Second"]);
    fs::write(root.path().join("file.txt"), "second\n").unwrap();
    stock(root.path(), &["describe", "-m", "Rewritten second"]);
    let op = stock(
        root.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    (root, op)
}
#[tokio::test]
async fn bidirectional_code_conversion_preserves_evolution_view_and_dirty_source() {
    let (source, op) = fixture();
    let path = source.path().join(".jj/repo");
    fs::write(source.path().join("dirty.txt"), "unsnapshotted").unwrap();
    let heads_before = fs::read_dir(path.join("op_heads/heads"))
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect::<Vec<_>>();
    let archive = archive_local(&path, SOURCE, op, None).await.unwrap();
    let CodeArchiveV1::Git { capsule, .. } = &archive else {
        panic!()
    };
    assert!(capsule.commits.len() >= 4);
    let original_predecessors = jj_lib::op_store::Operation::try_from(capsule.operation.clone())
        .unwrap()
        .commit_predecessors;
    assert!(
        original_predecessors
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    );
    let tmp = tempfile::tempdir().unwrap();
    let store = Arc::new(jj_store_object::memory::MemoryStore::default());
    let objects = Arc::new(NativeObjects::new(store.clone(), DEST, MAX_BYTES as u64).unwrap());
    let (native_op, map) = to_native(&archive, &tmp.path().join("read"), objects.clone())
        .await
        .unwrap();
    let native: jj_lib::op_store::Operation = objects
        .get(m::ObjectKey::Operation(native_op))
        .await
        .unwrap();
    assert_eq!(
        native.parents,
        vec![jj_lib::op_store::OperationId::from_bytes(
            jj_store_jj::roots::ROOT_OPERATION.as_bytes()
        )]
    );
    assert_eq!(
        native.commit_predecessors.as_ref().unwrap().len(),
        original_predecessors.as_ref().unwrap().len()
    );
    assert_eq!(map.commits.len(), capsule.commits.len());
    assert_eq!(map.commits[&"00".repeat(20)], "00".repeat(64));
    let verified = jj_store_wal::inventory::extend(
        objects.as_ref(),
        &Default::default(),
        [m::ObjectKey::Operation(native_op)],
        Default::default(),
    )
    .await
    .unwrap();
    let selected = verified
        .complete
        .entries()
        .values()
        .cloned()
        .collect::<Vec<_>>();
    let native_archive = archive_native(store.clone(), DEST, native_op, &selected)
        .await
        .unwrap();
    assert!(
        archive_native(store.clone(), DEST, native_op, &[])
            .await
            .is_err()
    );
    let destination = tmp.path().join("exported");
    let (local_op, back) = to_local(
        &native_archive,
        &tmp.path().join("native-read"),
        &destination,
    )
    .await
    .unwrap();
    assert_eq!(
        fs::read_to_string(destination.join("file.txt")).unwrap(),
        "second\n"
    );
    assert!(!destination.join("dirty.txt").exists());
    assert!(destination.join(".git").is_dir());
    let loader = RepoLoader::init_from_file_system(
        &settings(),
        &destination.join(".jj/repo"),
        &default_backend_factories(),
    )
    .unwrap();
    let selected = loader
        .load_operation(&jj_lib::op_store::OperationId::from_bytes(
            local_op.as_bytes(),
        ))
        .await
        .unwrap();
    let value = selected
        .op_store()
        .read_operation(selected.id())
        .await
        .unwrap();
    assert_eq!(
        value.parents,
        vec![jj_lib::op_store::OperationId::from_bytes(
            jj_store_jj::roots::ROOT_OPERATION.as_bytes()
        )]
    );
    assert_eq!(
        value.commit_predecessors.as_ref().unwrap().len(),
        original_predecessors.as_ref().unwrap().len()
    );
    for (old, new) in &map.commits {
        assert_eq!(&back.commits[new], old);
    }
    assert_eq!(
        fs::read_to_string(source.path().join("dirty.txt")).unwrap(),
        "unsnapshotted"
    );
    let heads_after = fs::read_dir(path.join("op_heads/heads"))
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect::<Vec<_>>();
    assert_eq!(heads_before, heads_after);
}
#[tokio::test]
async fn conversion_rejects_changed_or_uncommitted_source_and_occupied_destination() {
    let (source, op) = fixture();
    let path = source.path().join(".jj/repo");
    assert!(
        archive_local(&path, SOURCE, m::OperationId::new([13; 64]), None)
            .await
            .is_err()
    );
    let archive = archive_local(&path, SOURCE, op, None).await.unwrap();
    let tmp = tempfile::tempdir().unwrap();
    let destination = tmp.path().join("occupied");
    fs::create_dir(&destination).unwrap();
    fs::write(destination.join("kept"), "keep").unwrap();
    assert!(
        to_local(&archive, &tmp.path().join("scratch"), &destination)
            .await
            .is_err()
    );
    assert_eq!(
        fs::read_to_string(destination.join("kept")).unwrap(),
        "keep"
    );
    assert!(!destination.join(".jj").exists());
}
