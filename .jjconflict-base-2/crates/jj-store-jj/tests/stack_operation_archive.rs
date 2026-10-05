#[path = "stack_operation_archive/adversarial.rs"]
mod adversarial;
#[path = "stack_foreground/fixture.rs"]
mod fixture;
#[path = "stack_operation_archive/history.rs"]
mod history;
use fixture::*;
use jj_lib::{config::StackedConfig, settings::UserSettings};
use jj_store_jj::{
    local_evidence::{self, OperationArchiveLimits},
    local_execution::{Coordinator, Reword},
    local_metadata::{Limits, MetadataStore, StackPreparationKey, StackRetentionOutcome},
};
use jj_store_model as m;
use std::{fs, process::Command};

#[tokio::test]
async fn retained_operation_ancestry_reconstructs_exactly_after_source_loss_and_gc() {
    let fixture = Fixture::new().await;
    let expected = fixture.witness().await;
    let before = fixture.state();
    let lease = Coordinator::new(1)
        .unwrap()
        .acquire(fixture.access(), fixture.root(), expected.clone())
        .await
        .unwrap();
    let (lease, prepared) = lease
        .prepare_reword(Reword {
            principal: m::PrincipalId::new([85; 16]),
            commit: expected.baseline.commit,
            message: "Exact archived operation".into(),
        })
        .await
        .unwrap();
    let settings = UserSettings::from_config(StackedConfig::with_defaults()).unwrap();
    let archive = local_evidence::capture_operation_archive(
        &settings,
        &fixture.root().join(".jj/repo"),
        expected.baseline.repository,
        prepared.capsule.operation_id,
        OperationArchiveLimits::default(),
    )
    .await
    .unwrap();
    assert!(archive.operations.len() >= 3);
    assert!(
        archive
            .operations
            .contains_key(&expected.baseline.operation)
    );
    assert_eq!(archive.capsule().unwrap(), prepared.capsule);
    assert_eq!(archive, prepared.history);
    archive.verify_materialization().await.unwrap();
    let store = MetadataStore::open(&fixture.root().join(".git"), Limits::default()).unwrap();
    let key = StackPreparationKey {
        repository: expected.baseline.repository,
        physical_workspace: expected.physical_workspace,
        principal: m::PrincipalId::new([85; 16]),
        request: m::RequestId::new([91; 16]),
        intent: m::IntentDigest::new([92; 32]),
        prepared: m::RecordDigest::new([93; 32]),
    };
    let StackRetentionOutcome::Retained(retained) =
        store.retain_stack_execution(&key, &archive).unwrap()
    else {
        panic!("retention must acknowledge")
    };
    assert_eq!(retained.history.as_ref(), Some(&archive));
    lease.validate_metadata().await.unwrap();
    assert_eq!(fixture.state(), before);
    drop(lease);
    fs::remove_dir_all(fixture.root().join(".jj/repo/op_store")).unwrap();
    fs::remove_dir_all(fixture.root().join(".jj/repo/index")).unwrap();
    let output = Command::new("git")
        .arg("--git-dir")
        .arg(fixture.root().join(".git"))
        .args(["gc", "--prune=now"])
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let selected = store.read_stack_prepared(&key).unwrap().unwrap();
    assert_eq!(selected, *retained);
    selected
        .history
        .as_ref()
        .unwrap()
        .verify_materialization()
        .await
        .unwrap();
    assert!(
        !fixture.root().join(".jj/repo/op_store").exists(),
        "reconstruction is private, never implicit source repair"
    );
    assert!(store.read().unwrap().is_none());
}

#[tokio::test]
async fn archive_limits_and_missing_source_ancestry_fail_without_publication() {
    let fixture = Fixture::new().await;
    let expected = fixture.witness().await;
    let before = fixture.state();
    let settings = UserSettings::from_config(StackedConfig::with_defaults()).unwrap();
    let metadata = fixture.root().join(".jj/repo");
    for limits in [
        OperationArchiveLimits {
            max_operations: 1,
            ..Default::default()
        },
        OperationArchiveLimits {
            max_commits: 1,
            ..Default::default()
        },
        OperationArchiveLimits {
            max_bytes: 8,
            ..Default::default()
        },
        OperationArchiveLimits {
            timeout: std::time::Duration::ZERO,
            ..Default::default()
        },
    ] {
        assert!(matches!(
            local_evidence::capture_operation_archive(
                &settings,
                &metadata,
                expected.baseline.repository,
                expected.baseline.operation,
                limits
            )
            .await,
            Err(jj_store_jj::local_metadata::Error::Limit)
        ));
    }
    let archive = local_evidence::capture_operation_archive(
        &settings,
        &metadata,
        expected.baseline.repository,
        expected.baseline.operation,
        OperationArchiveLimits::default(),
    )
    .await
    .unwrap();
    let parent = archive.operations[&expected.baseline.operation]
        .parents
        .iter()
        .find(|id| **id != m::OperationId::new([0; 64]))
        .unwrap();
    fs::remove_file(
        metadata
            .join("op_store/operations")
            .join(parent.to_string()),
    )
    .unwrap();
    assert!(
        local_evidence::capture_operation_archive(
            &settings,
            &metadata,
            expected.baseline.repository,
            expected.baseline.operation,
            OperationArchiveLimits::default()
        )
        .await
        .is_err()
    );
    assert_eq!(fixture.state(), before);
}
