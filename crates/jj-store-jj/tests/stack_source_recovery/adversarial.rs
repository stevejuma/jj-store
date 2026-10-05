use super::*;
use jj_store_jj::local_browser::Error;

#[tokio::test]
async fn recovery_codec_and_complete_key_reject_substitution() {
    let f = Fixture::new().await;
    let (witness, key) = prepare(&f).await;
    let bytes = witness.encode().unwrap();
    assert!(RecoveryWitnessV1::decode(m::RecordDigest::new([1; 32]), &bytes).is_err());
    let mut extra = bytes.clone();
    extra.push(b' ');
    assert!(RecoveryWitnessV1::decode(witness.id().unwrap(), &extra).is_err());
    let mut value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    value["version"] = 2.into();
    assert!(
        RecoveryWitnessV1::decode(witness.id().unwrap(), &serde_json::to_vec(&value).unwrap())
            .is_err()
    );
    fs::remove_dir_all(f.root().join(".jj/repo/op_store")).unwrap();
    for i in 0..5 {
        let mut wrong = key.clone();
        match i {
            0 => wrong.principal = m::PrincipalId::new([41; 16]),
            1 => wrong.request = m::RequestId::new([42; 16]),
            2 => wrong.prepared = m::RecordDigest::new([43; 32]),
            3 => wrong.repository = m::RepositoryId::new([44; 16]),
            _ => wrong.physical_workspace = m::RecordDigest::new([45; 32]),
        }
        assert!(
            Coordinator::new(1)
                .unwrap()
                .restore_history(f.access(), f.root(), witness.clone(), wrong)
                .await
                .is_err()
        );
        assert!(!f.root().join(".jj/repo/op_store").exists());
    }
}

#[tokio::test]
async fn unsafe_record_and_directory_links_never_touch_targets() {
    let f = Fixture::new().await;
    let (witness, key) = prepare(&f).await;
    let archive = MetadataStore::open(&f.root().join(".git"), Limits::default())
        .unwrap()
        .read_stack_prepared(&key)
        .unwrap()
        .unwrap()
        .history
        .unwrap();
    let id = archive
        .operations
        .keys()
        .find(|id| id.as_bytes() != &[0; 64])
        .unwrap();
    let path = f
        .root()
        .join(".jj/repo/op_store/operations")
        .join(id.to_string());
    let target = f.root().join("ignored.txt");
    let before = fs::read(&target).unwrap();
    fs::remove_file(&path).unwrap();
    for hard in [false, true] {
        if hard {
            fs::hard_link(&target, &path).unwrap();
        } else {
            std::os::unix::fs::symlink(&target, &path).unwrap();
        }
        assert!(
            Coordinator::new(1)
                .unwrap()
                .restore_history(f.access(), f.root(), witness.clone(), key.clone())
                .await
                .is_err()
        );
        assert_eq!(fs::read(&target).unwrap(), before);
        fs::remove_file(&path).unwrap();
    }
    let views = f.root().join(".jj/repo/op_store/views");
    fs::remove_dir_all(&views).unwrap();
    let outside = tempfile::tempdir().unwrap();
    std::os::unix::fs::symlink(outside.path(), &views).unwrap();
    assert!(
        Coordinator::new(1)
            .unwrap()
            .restore_history(f.access(), f.root(), witness, key)
            .await
            .is_err()
    );
    assert!(fs::read_dir(outside.path()).unwrap().next().is_none());
    assert_eq!(fs::read(target).unwrap(), before);
}

#[tokio::test]
async fn changed_heads_ignore_policy_and_source_binding_fail_without_initialization() {
    let f = Fixture::new().await;
    let (witness, key) = prepare(&f).await;
    fs::remove_dir_all(f.root().join(".jj/repo/op_store")).unwrap();
    let heads = f
        .root()
        .join(".jj/repo/op_heads/heads")
        .join(m::OperationId::new([46; 64]).to_string());
    fs::write(&heads, []).unwrap();
    assert!(matches!(
        Coordinator::new(1)
            .unwrap()
            .restore_history(f.access(), f.root(), witness.clone(), key.clone())
            .await,
        Err(Error::EpochChanged)
    ));
    fs::remove_file(heads).unwrap();
    let exclude = f.root().join(".git/info/exclude");
    fs::write(&exclude, "different-ignore-policy\n").unwrap();
    assert!(matches!(
        Coordinator::new(1)
            .unwrap()
            .restore_history(f.access(), f.root(), witness.clone(), key.clone())
            .await,
        Err(Error::EpochChanged)
    ));
    assert!(!f.root().join(".jj/repo/op_store").exists());
    // Even copied directory contents cannot substitute physical device identity.
    let jj = f.root().join(".jj");
    let moved = f.root().join("original-jj");
    fs::rename(&jj, &moved).unwrap();
    fs::create_dir(&jj).unwrap();
    assert!(
        Coordinator::new(1)
            .unwrap()
            .restore_history(f.access(), f.root(), witness, key)
            .await
            .is_err()
    );
    assert!(fs::read_dir(jj).unwrap().next().is_none());
}

#[tokio::test]
async fn recovered_lease_keeps_foreground_exclusive_and_preserves_editor_bytes() {
    let f = Fixture::new().await;
    let (witness, key) = prepare(&f).await;
    fs::remove_dir_all(f.root().join(".jj/repo/op_store")).unwrap();
    let lease = Coordinator::new(1)
        .unwrap()
        .restore_history(f.access(), f.root(), witness.clone(), key.clone())
        .await
        .unwrap();
    assert!(matches!(
        Coordinator::new(1)
            .unwrap()
            .restore_history(f.access(), f.root(), witness.clone(), key.clone())
            .await,
        Err(Error::Busy)
    ));
    fs::write(f.root().join("file.txt"), "editor bytes after recovery\n").unwrap();
    lease.validate_binding().await.unwrap();
    drop(lease);
    Coordinator::new(1)
        .unwrap()
        .restore_history(f.access(), f.root(), witness, key)
        .await
        .unwrap();
    assert_eq!(
        fs::read(f.root().join("file.txt")).unwrap(),
        b"editor bytes after recovery\n"
    );
}
