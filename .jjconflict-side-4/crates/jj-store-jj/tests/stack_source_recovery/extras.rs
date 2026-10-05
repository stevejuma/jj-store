use super::*;
use jj_lib::{
    backend::Backend as _, config::StackedConfig, git_backend::GitBackend, settings::UserSettings,
};

pub(super) fn extra_state(root: &std::path::Path) -> Vec<(String, Vec<u8>)> {
    let mut files = vec![];
    let path = root.join(".jj/repo/store/extra");
    if !path.exists() {
        return files;
    }
    for e in fs::read_dir(&path).unwrap() {
        let e = e.unwrap();
        let name = e.file_name().to_str().unwrap().to_owned();
        if name == "lock" {
            continue;
        }
        if name == "heads" {
            for h in fs::read_dir(e.path()).unwrap() {
                let h = h.unwrap();
                files.push((
                    format!("heads/{}", h.file_name().to_str().unwrap()),
                    fs::read(h.path()).unwrap(),
                ));
            }
        } else {
            files.push((name, fs::read(e.path()).unwrap()));
        }
    }
    files.sort();
    files
}

#[tokio::test]
async fn held_upstream_extra_lock_rejects_without_waiting_or_selecting() {
    use jj_store_jj::local_browser::Error;
    use rustix::fs::{FlockOperation, flock};
    let f = Fixture::new().await;
    let (witness, key) = prepare(&f).await;
    let recovered = Coordinator::new(1)
        .unwrap()
        .restore_history(f.access(), f.root(), witness.clone(), key.clone())
        .await
        .unwrap();
    let before = extra_state(&f.root());
    let physical = f.state();
    let lock = fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(f.root().join(".jj/repo/store/extra/lock"))
        .unwrap();
    flock(&lock, FlockOperation::NonBlockingLockExclusive).unwrap();
    let result = tokio::time::timeout(
        std::time::Duration::from_secs(3),
        recovered.restore_extras(),
    )
    .await
    .unwrap();
    assert!(matches!(result, Err(Error::Busy)));
    assert_eq!(extra_state(&f.root()), before);
    assert_eq!(f.state(), physical);
    drop(lock);
    drop(
        Coordinator::new(1)
            .unwrap()
            .restore_history(f.access(), f.root(), witness, key)
            .await
            .unwrap()
            .restore_extras()
            .await
            .unwrap(),
    );
    // Candidate records may have been missing; every original byte remains.
    let after = extra_state(&f.root());
    for entry in before {
        assert!(after.contains(&entry));
    }
}

#[tokio::test]
async fn unsafe_extra_directories_segments_heads_and_locks_never_touch_targets() {
    let f = Fixture::new().await;
    let (witness, key) = prepare(&f).await;
    let extra = f.root().join(".jj/repo/store/extra");
    let before = extra_state(&f.root());
    let physical = f.state();
    let outside = tempfile::tempdir().unwrap();
    let target = outside.path().join("target");
    fs::write(&target, "untouched outside bytes").unwrap();
    for path in [extra.join("f".repeat(128)), extra.join("lock")] {
        let _ = fs::remove_file(extra.join("lock"));
        for hard in [false, true] {
            if hard {
                fs::hard_link(&target, &path).unwrap();
            } else {
                std::os::unix::fs::symlink(&target, &path).unwrap();
            }
            let recovered = Coordinator::new(1)
                .unwrap()
                .restore_history(f.access(), f.root(), witness.clone(), key.clone())
                .await
                .unwrap();
            assert!(recovered.restore_extras().await.is_err());
            assert_eq!(fs::read(&target).unwrap(), b"untouched outside bytes");
            assert_eq!(f.state(), physical);
            fs::remove_file(&path).unwrap();
        }
    }
    let head = extra.join("heads").join("e".repeat(128));
    std::os::unix::fs::symlink(&target, &head).unwrap();
    let recovered = Coordinator::new(1)
        .unwrap()
        .restore_history(f.access(), f.root(), witness.clone(), key.clone())
        .await
        .unwrap();
    assert!(recovered.restore_extras().await.is_err());
    fs::remove_file(&head).unwrap();
    for path in [extra.join("heads"), extra.clone()] {
        let original = outside.path().join("saved");
        fs::rename(&path, &original).unwrap();
        std::os::unix::fs::symlink(outside.path(), &path).unwrap();
        let recovered = Coordinator::new(1)
            .unwrap()
            .restore_history(f.access(), f.root(), witness.clone(), key.clone())
            .await
            .unwrap();
        assert!(recovered.restore_extras().await.is_err());
        fs::remove_file(&path).unwrap();
        fs::rename(original, path).unwrap();
        assert_eq!(fs::read(&target).unwrap(), b"untouched outside bytes");
    }
    assert_eq!(extra_state(&f.root()), before);
    assert_eq!(f.state(), physical);
}
#[tokio::test]
async fn exact_missing_extras_restore_after_source_loss_without_import_heads_or_checkout() {
    let f = Fixture::new().await;
    let physical = f.state();
    let (witness, key) = prepare(&f).await;
    let retained = MetadataStore::open(&f.root().join(".git"), Limits::default())
        .unwrap()
        .read_stack_prepared(&key)
        .unwrap()
        .unwrap();
    fs::remove_dir_all(f.root().join(".jj/repo/op_store")).unwrap();
    fs::remove_dir_all(f.root().join(".jj/repo/index")).unwrap();
    fs::remove_dir_all(f.root().join(".jj/repo/store/extra")).unwrap();
    let recovered = Coordinator::new(1)
        .unwrap()
        .restore_history(f.access(), f.root(), witness.clone(), key.clone())
        .await
        .unwrap();
    let recovered = recovered.restore_extras().await.unwrap();
    recovered.validate_binding().await.unwrap();
    drop(recovered);
    let before = extra_state(&f.root());
    assert!(!before.is_empty());
    let settings = UserSettings::from_config(StackedConfig::with_defaults()).unwrap();
    let backend = GitBackend::load(&settings, &f.root().join(".jj/repo/store")).unwrap();
    for (id, record) in &retained.history.as_ref().unwrap().commits {
        let actual = backend
            .read_commit(&jj_lib::backend::CommitId::from_bytes(id.as_bytes()))
            .await
            .unwrap();
        assert_eq!(actual, record.clone().try_into().unwrap());
    }
    assert_eq!(extra_state(&f.root()), before);
    assert_eq!(f.state(), physical);
    assert!(!f.root().join(".jj/repo/index").exists());
    let recovered = Coordinator::new(1)
        .unwrap()
        .restore_history(f.access(), f.root(), witness, key)
        .await
        .unwrap();
    drop(recovered.restore_extras().await.unwrap());
    assert_eq!(extra_state(&f.root()), before);
}

#[tokio::test]
async fn conflicting_original_extra_record_fails_complete_preflight_without_table_selection() {
    use jj_lib::stacked_table::TableStore;
    use prost::Message as _;
    let f = Fixture::new().await;
    let (witness, key) = prepare(&f).await;
    let retained = MetadataStore::open(&f.root().join(".git"), Limits::default())
        .unwrap()
        .read_stack_prepared(&key)
        .unwrap()
        .unwrap();
    let id = retained
        .history
        .as_ref()
        .unwrap()
        .commits
        .keys()
        .find(|id| id.as_bytes() != &[0; 20])
        .unwrap();
    let tables = TableStore::load(f.root().join(".jj/repo/store/extra"), 20);
    let head = tables.get_head().unwrap();
    let mut table = head.start_mutation();
    let record = jj_lib::protos::git_store::Commit {
        change_id: vec![99; 16],
        uses_tree_conflict_format: true,
        ..Default::default()
    };
    table.add_entry(id.as_bytes().to_vec(), record.encode_to_vec());
    tables.save_table(table).unwrap();
    let before = extra_state(&f.root());
    let physical = f.state();
    let recovered = Coordinator::new(1)
        .unwrap()
        .restore_history(f.access(), f.root(), witness, key)
        .await
        .unwrap();
    assert!(recovered.restore_extras().await.is_err());
    assert_eq!(extra_state(&f.root()), before);
    assert_eq!(f.state(), physical);
}
