#[path = "stack_source_recovery/adversarial.rs"]
mod adversarial;
#[path = "stack_source_recovery/extras.rs"]
mod extras;
#[path = "stack_source_recovery/extras_lifecycle.rs"]
mod extras_lifecycle;
#[path = "stack_source_recovery/extras_limits.rs"]
mod extras_limits;
#[path = "stack_foreground/fixture.rs"]
mod fixture;
#[path = "stack_source_recovery/indexes.rs"]
mod indexes;
#[path = "stack_source_recovery/indexes_adversarial.rs"]
mod indexes_adversarial;
#[path = "stack_source_recovery/indexes_lifecycle.rs"]
mod indexes_lifecycle;
#[path = "stack_source_recovery/lifecycle.rs"]
mod lifecycle;
use fixture::*;
use jj_lib::{op_store::OpStore as _, simple_op_store::SimpleOpStore};
use jj_store_jj::{
    local_execution::{Coordinator, RecoveryWitnessV1, Reword},
    local_metadata::{Limits, MetadataStore, StackPreparationKey, StackRetentionOutcome},
};
use jj_store_model as m;
use std::fs;

async fn prepare(f: &Fixture) -> (RecoveryWitnessV1, StackPreparationKey) {
    let expected = f.witness().await;
    let lease = Coordinator::new(1)
        .unwrap()
        .acquire(f.access(), f.root(), expected.clone())
        .await
        .unwrap();
    let (lease, witness) = lease.recovery_witness().await.unwrap();
    let (lease, candidate) = lease
        .prepare_reword(Reword {
            immutable_heads: vec![],
            principal: m::PrincipalId::new([85; 16]),
            commit: expected.baseline.commit,
            message: "Retained source recovery".into(),
        })
        .await
        .unwrap();
    let key = StackPreparationKey {
        repository: expected.baseline.repository,
        physical_workspace: expected.physical_workspace,
        principal: m::PrincipalId::new([85; 16]),
        request: m::RequestId::new([94; 16]),
        intent: m::IntentDigest::new([95; 32]),
        prepared: m::RecordDigest::new([96; 32]),
    };
    let store = MetadataStore::open(&f.root().join(".git"), Limits::default()).unwrap();
    assert!(matches!(
        store
            .retain_stack_execution(&key, &candidate.history)
            .unwrap(),
        StackRetentionOutcome::Retained(_)
    ));
    drop(lease);
    (witness, key)
}

#[tokio::test]
async fn restart_restores_exact_retained_history_without_heads_checkout_or_index_changes() {
    let f = Fixture::new().await;
    let before = f.state();
    let (witness, key) = prepare(&f).await;
    let bytes = witness.encode().unwrap();
    let witness = RecoveryWitnessV1::decode(witness.id().unwrap(), &bytes).unwrap();
    fs::remove_dir_all(f.root().join(".jj/repo/op_store")).unwrap();
    fs::remove_dir_all(f.root().join(".jj/repo/index")).unwrap();
    let recovery = Coordinator::new(1)
        .unwrap()
        .restore_history(f.access(), f.root(), witness.clone(), key.clone())
        .await
        .unwrap();
    recovery.validate_binding().await.unwrap();
    let retained = MetadataStore::open(&f.root().join(".git"), Limits::default())
        .unwrap()
        .read_stack_prepared(&key)
        .unwrap()
        .unwrap();
    let archive = retained.history.unwrap();
    let native = SimpleOpStore::load(
        &f.root().join(".jj/repo/op_store"),
        jj_lib::op_store::RootOperationData {
            root_commit_id: jj_lib::backend::CommitId::from_bytes(&[0; 20]),
        },
    );
    for (id, record) in &archive.operations {
        let actual = native
            .read_operation(&jj_lib::op_store::OperationId::from_bytes(id.as_bytes()))
            .await
            .unwrap();
        assert_eq!(actual, record.clone().try_into().unwrap());
    }
    for (id, record) in &archive.views {
        let actual = native
            .read_view(&jj_lib::op_store::ViewId::from_bytes(id.as_bytes()))
            .await
            .unwrap();
        assert_eq!(actual, record.clone().try_into().unwrap());
    }
    assert_eq!(f.state(), before);
    assert!(!f.root().join(".jj/repo/index").exists());
    assert!(
        MetadataStore::open(&f.root().join(".git"), Limits::default())
            .unwrap()
            .read()
            .unwrap()
            .is_none()
    );
    drop(recovery);
    Coordinator::new(1)
        .unwrap()
        .restore_history(f.access(), f.root(), witness, key)
        .await
        .unwrap();
    assert_eq!(f.state(), before);
}

#[tokio::test]
async fn stale_checkout_or_original_key_cannot_restore_missing_state() {
    let f = Fixture::new().await;
    let (witness, key) = prepare(&f).await;
    fs::remove_dir_all(f.root().join(".jj/repo/op_store")).unwrap();
    let mut wrong = key.clone();
    wrong.intent = m::IntentDigest::new([97; 32]);
    assert!(
        Coordinator::new(1)
            .unwrap()
            .restore_history(f.access(), f.root(), witness.clone(), wrong)
            .await
            .is_err()
    );
    assert!(!f.root().join(".jj/repo/op_store").exists());
    fs::write(
        f.root().join(".jj/working_copy/checkout"),
        "different checkout",
    )
    .unwrap();
    assert!(
        Coordinator::new(1)
            .unwrap()
            .restore_history(f.access(), f.root(), witness, key)
            .await
            .is_err()
    );
    assert!(!f.root().join(".jj/repo/op_store").exists());
}

#[tokio::test]
async fn conflicting_existing_record_fails_before_any_missing_record_is_restored() {
    let f = Fixture::new().await;
    let (witness, key) = prepare(&f).await;
    let store = MetadataStore::open(&f.root().join(".git"), Limits::default()).unwrap();
    let archive = store
        .read_stack_prepared(&key)
        .unwrap()
        .unwrap()
        .history
        .unwrap();
    let op_dir = f.root().join(".jj/repo/op_store/operations");
    let ids: Vec<_> = archive
        .operations
        .keys()
        .filter(|id| id.as_bytes() != &[0; 64])
        .collect();
    assert!(ids.len() >= 2);
    fs::remove_file(op_dir.join(ids[0].to_string())).unwrap();
    fs::write(op_dir.join(ids[1].to_string()), "conflicting record").unwrap();
    let before = f.state();
    assert!(
        Coordinator::new(1)
            .unwrap()
            .restore_history(f.access(), f.root(), witness, key)
            .await
            .is_err()
    );
    assert!(!op_dir.join(ids[0].to_string()).exists());
    assert_eq!(
        fs::read(op_dir.join(ids[1].to_string())).unwrap(),
        b"conflicting record"
    );
    assert_eq!(f.state(), before);
}
