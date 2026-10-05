#[path = "stack_head_publication/adversarial.rs"]
mod adversarial;
#[path = "stack_foreground/fixture.rs"]
mod fixture;
#[path = "stack_head_publication/lifecycle.rs"]
mod lifecycle;
use fixture::*;
use jj_store_jj::{
    local_execution::{Coordinator, RecoveryWitnessV1, Reword},
    local_metadata::{Limits, MetadataStore, StackPreparationKey},
};
use jj_store_model as m;
use std::fs;

async fn prepare(f: &Fixture) -> (RecoveryWitnessV1, StackPreparationKey, m::OperationId) {
    let state = f.witness().await;
    let lease = Coordinator::new(1)
        .unwrap()
        .acquire(f.access(), f.root(), state.clone())
        .await
        .unwrap();
    let (lease, witness) = lease.recovery_witness().await.unwrap();
    let (lease, candidate) = lease
        .prepare_reword(Reword {
            principal: m::PrincipalId::new([85; 16]),
            immutable_heads: vec![],
            commit: state.baseline.commit,
            message: "Exact head publication".into(),
        })
        .await
        .unwrap();
    let operation = candidate.history.operation;
    let key = StackPreparationKey {
        repository: state.baseline.repository,
        physical_workspace: state.physical_workspace,
        principal: m::PrincipalId::new([85; 16]),
        request: m::RequestId::new([94; 16]),
        intent: m::IntentDigest::new([95; 32]),
        prepared: m::RecordDigest::new([96; 32]),
    };
    MetadataStore::open(&f.root().join(".git"), Limits::default())
        .unwrap()
        .retain_stack_execution(&key, &candidate.history)
        .unwrap();
    drop(lease);
    // Include the native coordination file in fixture snapshots. Publication
    // may create this empty lock during preflight without selecting a head.
    fs::write(f.root().join(".jj/repo/op_heads/heads/lock"), []).unwrap();
    (witness, key, operation)
}
fn heads(f: &Fixture) -> Vec<String> {
    let mut out: Vec<_> = fs::read_dir(f.root().join(".jj/repo/op_heads/heads"))
        .unwrap()
        .map(|e| e.unwrap().file_name().to_str().unwrap().to_owned())
        .filter(|n| n.len() == 128)
        .collect();
    out.sort();
    out
}

#[tokio::test]
async fn exact_publication_replaces_only_original_head_without_checkout_and_retries_after_reopen() {
    let f = Fixture::new().await;
    let (witness, key, candidate) = prepare(&f).await;
    let before = f.state();
    let recovery = Coordinator::new(1)
        .unwrap()
        .resume_publication(f.access(), f.root(), witness.clone(), key.clone())
        .await
        .unwrap()
        .restore_extras()
        .await
        .unwrap()
        .restore_indexes()
        .await
        .unwrap();
    let (recovery, proof) = recovery.publish_head().await.unwrap();
    assert_eq!(proof.operation, candidate);
    assert_eq!(proof.preparation(), &key);
    assert_eq!(proof.witness(), witness.id().unwrap());
    assert_eq!(heads(&f), vec![candidate.to_string()]);
    assert_eq!(&f.state()[..4], &before[..4]);
    recovery.validate_binding().await.unwrap();
    drop(recovery);
    let recovery = Coordinator::new(1)
        .unwrap()
        .resume_publication(f.access(), f.root(), witness, key)
        .await
        .unwrap();
    let (_recovery, same) = recovery.publish_head().await.unwrap();
    assert_eq!(same.operation, proof.operation);
    assert_eq!(same.view, proof.view);
    assert_eq!(heads(&f), vec![candidate.to_string()]);
    assert_eq!(&f.state()[..4], &before[..4]);
}

#[tokio::test]
async fn actual_cli_sibling_head_is_preserved_when_the_original_head_was_already_consumed() {
    let f = Fixture::new().await;
    let (witness, key, candidate) = prepare(&f).await;
    let before = f.state();
    stock(
        &f.root(),
        &[
            "--ignore-working-copy",
            "--at-op",
            &witness.binding().operation.to_string(),
            "describe",
            "-r",
            &witness.binding().commit.to_string(),
            "-m",
            "Concurrent CLI branch",
        ],
    );
    let sibling = heads(&f);
    assert_eq!(sibling.len(), 1);
    assert_ne!(sibling[0], witness.binding().operation.to_string());
    assert_eq!(&f.state()[..4], &before[..4]);
    let recovery = Coordinator::new(1)
        .unwrap()
        .resume_publication(f.access(), f.root(), witness, key)
        .await
        .unwrap()
        .restore_extras()
        .await
        .unwrap()
        .restore_indexes()
        .await
        .unwrap();
    let (_recovery, proof) = recovery.publish_head().await.unwrap();
    let mut expected = sibling;
    expected.push(candidate.to_string());
    expected.sort();
    assert_eq!(heads(&f), expected);
    assert_eq!(proof.operation, candidate);
    assert_eq!(&f.state()[..4], &before[..4]);
}

async fn recover(
    f: &Fixture,
    witness: RecoveryWitnessV1,
    key: StackPreparationKey,
) -> jj_store_jj::local_execution::RecoveredHistory {
    Coordinator::new(1)
        .unwrap()
        .resume_publication(f.access(), f.root(), witness, key)
        .await
        .unwrap()
        .restore_extras()
        .await
        .unwrap()
        .restore_indexes()
        .await
        .unwrap()
}

#[tokio::test]
async fn retry_after_real_cli_descendant_does_not_reannounce_or_rewind_candidate() {
    let f = Fixture::new().await;
    let (witness, key, candidate) = prepare(&f).await;
    let before = f.state();
    let (recovery, proof) = recover(&f, witness.clone(), key.clone())
        .await
        .publish_head()
        .await
        .unwrap();
    drop(recovery);
    stock(
        &f.root(),
        &[
            "--ignore-working-copy",
            "--at-op",
            &candidate.to_string(),
            "describe",
            "-r",
            "@",
            "-m",
            "Later CLI operation",
        ],
    );
    let later = heads(&f);
    assert_eq!(later.len(), 1);
    assert_ne!(later[0], candidate.to_string());
    let (_recovery, same) = recover(&f, witness, key)
        .await
        .publish_head()
        .await
        .unwrap();
    assert_eq!(same, proof);
    assert_eq!(heads(&f), later);
    assert_eq!(&f.state()[..4], &before[..4]);
}

#[tokio::test]
async fn multiple_real_cli_siblings_survive_exact_candidate_selection() {
    let f = Fixture::new().await;
    let (witness, key, candidate) = prepare(&f).await;
    let before = f.state();
    for message in ["Sibling one", "Sibling two"] {
        stock(
            &f.root(),
            &[
                "--ignore-working-copy",
                "--at-op",
                &witness.binding().operation.to_string(),
                "describe",
                "-r",
                &witness.binding().commit.to_string(),
                "-m",
                message,
            ],
        );
    }
    let mut expected = heads(&f);
    assert_eq!(expected.len(), 2);
    expected.push(candidate.to_string());
    expected.sort();
    let (_recovery, proof) = recover(&f, witness, key)
        .await
        .publish_head()
        .await
        .unwrap();
    assert_eq!(proof.operation, candidate);
    assert_eq!(heads(&f), expected);
    assert_eq!(&f.state()[..4], &before[..4]);
}

#[tokio::test]
async fn ordinary_history_restoration_remains_strict_after_publication() {
    let f = Fixture::new().await;
    let (witness, key, _) = prepare(&f).await;
    let (recovery, _) = recover(&f, witness.clone(), key.clone())
        .await
        .publish_head()
        .await
        .unwrap();
    drop(recovery);
    assert!(matches!(
        Coordinator::new(1)
            .unwrap()
            .restore_history(f.access(), f.root(), witness, key)
            .await,
        Err(jj_store_jj::local_browser::Error::EpochChanged)
    ));
}
