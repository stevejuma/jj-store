use super::*;
use jj_store_jj::{local_browser as q, local_execution::RecoveredHistory};

async fn recovered(
    f: &Fixture,
    witness: &RecoveryWitnessV1,
    key: &StackPreparationKey,
) -> RecoveredHistory {
    Coordinator::new(1)
        .unwrap()
        .resume_publication(f.access(), f.root(), witness.clone(), key.clone())
        .await
        .unwrap()
}
#[tokio::test]
async fn retained_source_policy_rejects_protected_hidden_duplicate_and_oversized_inputs_without_writes()
 {
    let f = Fixture::new().await;
    let current = f.witness().await;
    let (witness, key) = prepare(&f).await;
    let before = f.state();
    let root = m::GitSha1Id::new([0; 20]);
    let foreign = m::GitSha1Id::new([99; 20]);
    for (target, heads, expected) in [
        (current.baseline.commit, vec![current.baseline.commit], 0),
        (current.baseline.commit, vec![root, root], 1),
        (current.baseline.commit, vec![foreign], 2),
        (foreign, vec![root], 2),
        (root, vec![root], 1),
        (current.baseline.commit, vec![root; 65], 1),
    ] {
        let result = recovered(&f, &witness, &key)
            .await
            .validate_reword_policy(target, heads)
            .await;
        assert!(match expected {
            0 => matches!(result, Err(q::Error::Denied)),
            1 => matches!(result, Err(q::Error::Invalid)),
            _ => matches!(result, Err(q::Error::NotCommitted)),
        });
        assert_eq!(f.state(), before);
    }
    // The original predecessor is retained for evolution but is not visible.
    let archive = MetadataStore::open(&f.root().join(".git"), Limits::default())
        .unwrap()
        .read_stack_prepared(&key)
        .unwrap()
        .unwrap()
        .history
        .unwrap();
    let hidden = archive.commits[&current.baseline.commit]
        .predecessors
        .first()
        .copied()
        .unwrap();
    assert!(matches!(
        recovered(&f, &witness, &key)
            .await
            .validate_reword_policy(hidden, vec![root])
            .await,
        Err(q::Error::NotCommitted)
    ));
    let lease = recovered(&f, &witness, &key)
        .await
        .validate_reword_policy(current.baseline.commit, vec![root])
        .await
        .unwrap();
    lease.validate_binding().await.unwrap();
    assert_eq!(f.state(), before);
}

#[tokio::test]
async fn current_protected_descendant_blocks_rewriting_its_retained_ancestor() {
    let f = Fixture::new().await;
    stock(&f.root(), &["new", "-m", "Descendant"]);
    let operation = stock(
        &f.root(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    let current = q::Executor::new(1)
        .unwrap()
        .working_tree_current(f.access(), f.root(), operation)
        .await
        .unwrap();
    let ancestor = stock(
        &f.root(),
        &["log", "--no-graph", "-r", "@-", "-T", "commit_id"],
    )
    .parse()
    .unwrap();
    let lease = Coordinator::new(1)
        .unwrap()
        .acquire(f.access(), f.root(), current.clone())
        .await
        .unwrap();
    let (lease, witness) = lease.recovery_witness().await.unwrap();
    let (lease, candidate) = lease
        .prepare_reword(Reword {
            principal: m::PrincipalId::new([85; 16]),
            commit: ancestor,
            message: "Retained ancestor".into(),
            immutable_heads: vec![],
        })
        .await
        .unwrap();
    let key = StackPreparationKey {
        repository: current.baseline.repository,
        physical_workspace: current.physical_workspace,
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
    let before = f.state();
    assert!(matches!(
        recovered(&f, &witness, &key)
            .await
            .validate_reword_policy(ancestor, vec![current.baseline.commit])
            .await,
        Err(q::Error::Denied)
    ));
    assert_eq!(f.state(), before);
}

#[tokio::test]
async fn acknowledgement_only_rechecks_publication_and_cannot_reannounce_a_lost_candidate() {
    let f = Fixture::new().await;
    let (witness, key) = prepare(&f).await;
    fs::write(f.root().join(".jj/repo/op_heads/heads/lock"), []).unwrap();
    let before = f.state();
    let lease = recovered(&f, &witness, &key)
        .await
        .restore_extras()
        .await
        .unwrap()
        .restore_indexes()
        .await
        .unwrap();
    let (lease, selected) = lease.publication_selected().await.unwrap();
    assert!(!selected);
    assert!(matches!(
        lease.acknowledge_publication().await,
        Err(q::Error::EpochChanged)
    ));
    assert_eq!(f.state(), before);
    let (lease, published) = recovered(&f, &witness, &key)
        .await
        .publish_head()
        .await
        .unwrap();
    let (lease, selected) = lease.publication_selected().await.unwrap();
    assert!(selected);
    let (lease, acknowledged) = lease.acknowledge_publication().await.unwrap();
    assert_eq!(acknowledged, published);
    let (lease, selected) = lease.publication_selected().await.unwrap();
    assert!(selected);
    // Selected-head status is only information. Simulate a later external
    // restoration before acknowledgement and verify no candidate is recreated.
    let heads = f.root().join(".jj/repo/op_heads/heads");
    fs::remove_file(heads.join(published.operation.to_string())).unwrap();
    fs::write(heads.join(witness.binding().operation.to_string()), []).unwrap();
    let restored = f.state();
    assert!(matches!(
        lease.acknowledge_publication().await,
        Err(q::Error::EpochChanged)
    ));
    assert_eq!(f.state(), restored);
}
