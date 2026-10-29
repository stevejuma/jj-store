use super::*;
use jj_store_jj::local_evidence::OperationArchiveV1;
use jj_store_jj::local_metadata::Error;
use std::os::unix::fs::symlink;

pub(super) fn key(fixture: &Fixture) -> StackPreparationKey {
    StackPreparationKey {
        repository: fixture.access().repository(),
        physical_workspace: m::RecordDigest::new([71; 32]),
        principal: m::PrincipalId::new([72; 16]),
        request: m::RequestId::new([73; 16]),
        intent: m::IntentDigest::new([74; 32]),
        prepared: m::RecordDigest::new([75; 32]),
    }
}
pub(super) async fn capture(fixture: &Fixture, operation: m::OperationId) -> OperationArchiveV1 {
    local_evidence::capture_operation_archive(
        &UserSettings::from_config_and_home_dir(StackedConfig::with_defaults(), None).unwrap(),
        &fixture.root().join(".jj/repo"),
        fixture.access().repository(),
        operation,
        OperationArchiveLimits::default(),
    )
    .await
    .unwrap()
}
fn reference(key: &StackPreparationKey) -> String {
    format!(
        "{}{}/{}",
        jj_store_jj::local_metadata::STACK_PREPARED_REF_PREFIX,
        key.physical_workspace,
        key.request
    )
}

#[tokio::test]
async fn complete_slots_replay_exactly_and_legacy_slots_are_immutable() {
    let fixture = Fixture::new().await;
    let operation = fixture.witness().await.baseline.operation;
    let before = fixture.state();
    let archive = capture(&fixture, operation).await;
    let capsule = archive.capsule().unwrap();
    let store = MetadataStore::open(&fixture.root().join(".git"), Limits::default()).unwrap();
    let legacy_key = key(&fixture);
    let StackRetentionOutcome::Retained(legacy) =
        store.retain_stack_prepared(&legacy_key, &capsule).unwrap()
    else {
        panic!()
    };
    let legacy_bytes = fs::read(store.git_dir().join(reference(&legacy_key))).unwrap();
    assert!(legacy.history.is_none());
    assert!(matches!(
        store.retain_stack_execution(&legacy_key, &archive),
        Err(Error::IntentConflict)
    ));
    assert_eq!(
        store.read_stack_prepared(&legacy_key).unwrap().unwrap(),
        *legacy
    );
    assert_eq!(
        fs::read(store.git_dir().join(reference(&legacy_key))).unwrap(),
        legacy_bytes
    );

    let mut complete_key = legacy_key.clone();
    complete_key.request = m::RequestId::new([76; 16]);
    let path = store.git_dir().join(reference(&complete_key));
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    let lock = path.with_extension("lock");
    fs::write(&lock, b"held fixture ref lock").unwrap();
    let StackRetentionOutcome::Unknown { commit } = store
        .retain_stack_execution(&complete_key, &archive)
        .unwrap()
    else {
        panic!()
    };
    assert!(store.read_stack_prepared(&complete_key).unwrap().is_none());
    fs::remove_file(lock).unwrap();
    let StackRetentionOutcome::Retained(selected) = store
        .retain_stack_execution(&complete_key, &archive)
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(selected.commit, commit);
    assert_eq!(selected.history.as_ref(), Some(&archive));
    let StackRetentionOutcome::Retained(replay) = store
        .retain_stack_execution(&complete_key, &archive)
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(replay, selected);
    assert!(matches!(
        store.retain_stack_prepared(&complete_key, &capsule),
        Err(Error::IntentConflict)
    ));
    assert_eq!(fixture.state(), before);
    assert!(store.read().unwrap().is_none());
}

#[tokio::test]
async fn archive_codec_rejects_missing_unrelated_tampered_and_noncanonical_records() {
    let fixture = Fixture::new().await;
    let archive = capture(&fixture, fixture.witness().await.baseline.operation).await;
    let bytes = archive.encode().unwrap();
    assert_eq!(
        OperationArchiveV1::decode(archive.id().unwrap(), &bytes).unwrap(),
        archive
    );
    assert!(OperationArchiveV1::decode(m::RecordDigest::new([99; 32]), &bytes).is_err());
    let mut spaced = bytes.clone();
    spaced.push(b'\n');
    assert!(OperationArchiveV1::decode(archive.id().unwrap(), &spaced).is_err());
    for case in 0..7 {
        let mut broken = archive.clone();
        match case {
            0 => broken.version = 2,
            1 => {
                broken.operations.remove(&m::OperationId::new([0; 64]));
            }
            2 => {
                broken
                    .views
                    .remove(&broken.operations[&broken.operation].view_id);
            }
            3 => broken
                .operations
                .get_mut(&broken.operation)
                .unwrap()
                .metadata
                .description
                .push_str("tampered"),
            4 => {
                broken.commits.remove(&m::GitSha1Id::new([0; 20]));
            }
            5 => {
                broken.operations.insert(
                    m::OperationId::new([90; 64]),
                    broken.operations[&broken.operation].clone(),
                );
            }
            _ => {
                broken.commits.insert(
                    m::GitSha1Id::new([90; 20]),
                    broken.commits[&m::GitSha1Id::new([0; 20])].clone(),
                );
            }
        }
        assert!(broken.encode().is_err(), "case {case}");
        let store = MetadataStore::open(&fixture.root().join(".git"), Limits::default()).unwrap();
        assert!(
            store
                .retain_stack_execution(&key(&fixture), &broken)
                .is_err()
        );
        assert!(!store.git_dir().join("refs/lapis").exists());
    }
    // A valid content hash does not make a parentless non-root operation safe
    // to materialize: the upstream writer asserts its ancestry precondition.
    let mut broken = archive.clone();
    let mut operation = broken.operations[&broken.operation].clone();
    operation.parents.clear();
    let native = jj_lib::op_store::Operation::try_from(operation.clone()).unwrap();
    let id = m::OperationId::new(jj_lib::content_hash::blake2b_hash(&native).into());
    broken.operation = id;
    broken.operations = std::collections::BTreeMap::from([(id, operation)]);
    assert!(broken.verify_materialization().await.is_err());
}

#[tokio::test]
async fn nofollow_operation_and_view_reads_reject_links_and_wrong_source_profile() {
    let fixture = Fixture::new().await;
    let operation = fixture.witness().await.baseline.operation;
    let archive = capture(&fixture, operation).await;
    let before = fixture.state();
    let metadata = fixture.root().join(".jj/repo");
    let settings =
        UserSettings::from_config_and_home_dir(StackedConfig::with_defaults(), None).unwrap();
    let view = archive.operations[&operation].view_id;
    for name in [format!("operations/{operation}"), format!("views/{view}")] {
        let path = metadata.join("op_store").join(name);
        let saved = path.with_extension("saved");
        fs::rename(&path, &saved).unwrap();
        for hard in [false, true] {
            if hard {
                fs::hard_link(&saved, &path).unwrap();
            } else {
                symlink(&saved, &path).unwrap();
            }
            assert!(
                local_evidence::capture_operation_archive(
                    &settings,
                    &metadata,
                    fixture.access().repository(),
                    operation,
                    OperationArchiveLimits::default()
                )
                .await
                .is_err()
            );
            fs::remove_file(&path).unwrap();
        }
        fs::rename(&saved, &path).unwrap();
    }
    fs::write(metadata.join("op_store/type"), "unsupported").unwrap();
    assert!(
        local_evidence::capture_operation_archive(
            &settings,
            &metadata,
            fixture.access().repository(),
            operation,
            OperationArchiveLimits::default()
        )
        .await
        .is_err()
    );
    assert_eq!(fixture.state(), before);
}
