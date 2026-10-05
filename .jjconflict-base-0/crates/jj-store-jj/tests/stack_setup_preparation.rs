#[path = "stack_foreground/fixture.rs"]
mod fixture;
#[path = "stack_setup_preparation/lifecycle.rs"]
mod lifecycle;
#[path = "stack_setup_preparation/policy.rs"]
mod policy;
use fixture::*;
use jj_store_jj::{
    local_browser::{self as q, working_tree::CurrentState},
    local_execution::{Coordinator, PhysicalGuard, RecoveryBinding, Setup, SetupBookmark},
};
use jj_store_model as m;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

// Provider boundary fixture only. Registered CLI tests use the real scanner.
struct Guard {
    expected: m::RecordDigest,
    reject_on: usize,
    calls: AtomicUsize,
}
#[async_trait::async_trait]
impl PhysicalGuard for Guard {
    async fn validate(&self, _: &RecoveryBinding, expected: m::RecordDigest) -> q::Result<()> {
        let call = self.calls.fetch_add(1, Ordering::SeqCst) + 1;
        if expected != self.expected || call == self.reject_on {
            Err(q::Error::EpochChanged)
        } else {
            Ok(())
        }
    }
}
fn guard(reject_on: usize) -> Arc<Guard> {
    Arc::new(Guard {
        expected: m::RecordDigest::new([92; 32]),
        reject_on,
        calls: AtomicUsize::new(0),
    })
}
async fn target_fixture() -> (Fixture, CurrentState, SetupBookmark) {
    let f = Fixture::new().await;
    f.witness().await;
    std::fs::write(f.root().join("file.txt"), "original\n").unwrap();
    stock(&f.root(), &["bookmark", "create", "main", "-r", "@"]);
    std::fs::write(f.root().join("file.txt"), "dirty external edit\n").unwrap();
    let op = stock(
        &f.root(),
        &[
            "--ignore-working-copy",
            "op",
            "log",
            "--no-graph",
            "-n",
            "1",
            "-T",
            "id",
        ],
    )
    .parse()
    .unwrap();
    let current = q::Executor::new(1)
        .unwrap()
        .working_tree_current(f.access(), f.root(), op)
        .await
        .unwrap();
    let target = SetupBookmark {
        name: "main".into(),
        remote: None,
        head: current.baseline.commit,
    };
    (f, current, target)
}
#[tokio::test]
async fn actual_setup_retains_original_history_refs_and_dirty_bytes_with_unpublished_merge_wip() {
    let (f, current, target) = target_fixture().await;
    let before = f.state();
    let physical = guard(0);
    let lease = Coordinator::new(1)
        .unwrap()
        .acquire(f.access(), f.root(), current.clone())
        .await
        .unwrap();
    let (lease, witness) = lease.recovery_witness().await.unwrap();
    let (lease, prepared) = lease
        .prepare_setup(
            Setup {
                principal: m::PrincipalId::new([91; 16]),
                target: target.clone(),
            },
            physical.clone(),
            physical.expected,
        )
        .await
        .unwrap();
    assert_eq!(f.state(), before);
    assert_eq!(physical.calls.load(Ordering::SeqCst), 2);
    assert_eq!(prepared.target, target);
    assert_eq!(
        prepared.capsule.operation.parents,
        [current.baseline.operation]
    );
    assert_eq!(
        prepared.capsule.view.wc_commit_ids[&current.checkout.workspace],
        prepared.wip
    );
    let merge = &prepared.capsule.commits[&prepared.merge];
    let wip = &prepared.capsule.commits[&prepared.wip];
    assert_eq!(merge.parents, [target.head]);
    assert_eq!(
        merge.root_tree,
        prepared.source.commits[&target.head].root_tree
    );
    assert_eq!(wip.parents, [prepared.merge]);
    assert_eq!(wip.root_tree, current.checkout.trees);
    assert_eq!(
        prepared.capsule.view.local_bookmarks,
        prepared.source.view.local_bookmarks
    );
    for (id, commit) in &prepared.source.commits {
        assert_eq!(prepared.history.commits.get(id), Some(commit));
    }
    use jj_store_jj::local_metadata::{StackPreparationKey, StackRetentionOutcome};
    let key = StackPreparationKey {
        repository: current.baseline.repository,
        physical_workspace: current.physical_workspace,
        principal: m::PrincipalId::new([91; 16]),
        request: m::RequestId::new([92; 16]),
        intent: m::IntentDigest::new([93; 32]),
        prepared: m::RecordDigest::new([94; 32]),
    };
    let (lease, retained) = lease
        .retain_history(witness, key, prepared.history.clone())
        .await
        .unwrap();
    let StackRetentionOutcome::Retained(retained) = retained else {
        panic!("retention must acknowledge");
    };
    assert_eq!(retained.capsule, prepared.capsule);
    assert_eq!(retained.history.as_ref(), Some(&prepared.history));
    assert_eq!(f.state(), before);
    lease.validate_metadata().await.unwrap();
}
#[tokio::test]
async fn setup_requires_exact_existing_target_and_complete_physical_guard() {
    let (f, current, target) = target_fixture().await;
    let before = f.state();
    let coordinator = Coordinator::new(1).unwrap();
    for (target, reject_on) in [
        (
            SetupBookmark {
                name: "missing".into(),
                ..target.clone()
            },
            0,
        ),
        (
            SetupBookmark {
                head: m::GitSha1Id::new([90; 20]),
                ..target.clone()
            },
            0,
        ),
        (target.clone(), 1),
        (target, 2),
    ] {
        let physical = guard(reject_on);
        let lease = coordinator
            .acquire(f.access(), f.root(), current.clone())
            .await
            .unwrap();
        assert!(matches!(
            lease
                .prepare_setup(
                    Setup {
                        principal: m::PrincipalId::new([91; 16]),
                        target
                    },
                    physical.clone(),
                    physical.expected
                )
                .await,
            Err(q::Error::EpochChanged)
        ));
        assert_eq!(f.state(), before);
    }
}
