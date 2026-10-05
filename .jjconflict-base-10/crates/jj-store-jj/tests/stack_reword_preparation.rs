#[path = "stack_foreground/fixture.rs"]
mod fixture;
#[path = "stack_reword_preparation/lifecycle.rs"]
mod lifecycle;
#[path = "stack_reword_preparation/policy.rs"]
mod policy;
use fixture::*;
use jj_store_jj::{
    local_browser::Error,
    local_execution::{Coordinator, Reword},
};
use jj_store_model as m;

#[tokio::test]
async fn exact_reword_preparation_retains_change_tree_and_dirty_checkout_without_publication() {
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
            immutable_heads: vec![],
            principal: m::PrincipalId::new([85; 16]),
            commit: expected.baseline.commit,
            message: "Prepared title\n\nPrepared description".into(),
        })
        .await
        .unwrap();
    assert_eq!(fixture.state(), before);
    assert_ne!(prepared.capsule.operation_id, expected.baseline.operation);
    assert_eq!(
        prepared.capsule.operation.parents,
        vec![expected.baseline.operation]
    );
    let new_id = prepared.capsule.view.wc_commit_ids[&expected.checkout.workspace];
    let old = &prepared.capsule.commits[&expected.baseline.commit];
    let new = &prepared.capsule.commits[&new_id];
    assert_eq!(new.change_id, old.change_id);
    assert_eq!(new.author, old.author);
    assert_eq!(new.root_tree, old.root_tree);
    assert_eq!(new.description, "Prepared title\n\nPrepared description");
    assert_eq!(prepared.rewrites.len(), 1);
    assert_eq!(prepared.rewrites[0].before, vec![expected.baseline.commit]);
    assert_eq!(prepared.rewrites[0].after, vec![new_id]);
    assert_eq!(prepared.checkout_before, prepared.checkout_after);
    prepared.capsule.validate().unwrap();
    lease.validate_metadata().await.unwrap();
}

#[tokio::test]
async fn invalid_reword_root_and_hidden_targets_create_no_published_state() {
    let fixture = Fixture::new().await;
    let coordinator = Coordinator::new(1).unwrap();
    let expected = fixture.witness().await;
    let before = fixture.state();
    for (commit, message) in [
        (m::GitSha1Id::new([0; 20]), "Root".into()),
        (m::GitSha1Id::new([86; 20]), "Hidden".into()),
        (expected.baseline.commit, "bad\0message".into()),
        (expected.baseline.commit, "x".repeat(65537)),
    ] {
        let lease = coordinator
            .acquire(fixture.access(), fixture.root(), expected.clone())
            .await
            .unwrap();
        let result = lease
            .prepare_reword(Reword {
                immutable_heads: vec![],
                principal: m::PrincipalId::new([85; 16]),
                commit,
                message,
            })
            .await;
        assert!(matches!(
            result,
            Err(Error::Invalid | Error::NotCommitted | Error::Limit)
        ));
        assert_eq!(fixture.state(), before);
    }
}

#[tokio::test]
async fn ancestor_reword_reports_all_descendants_and_exact_bookmark_terms() {
    use jj_store_jj::local_browser::Executor;
    use std::fs;
    let fixture = Fixture::new().await;
    stock(&fixture.root(), &["bookmark", "create", "main", "-r", "@"]);
    let first: m::GitSha1Id = stock(
        &fixture.root(),
        &[
            "--ignore-working-copy",
            "log",
            "-r",
            "@",
            "--no-graph",
            "-T",
            "commit_id",
        ],
    )
    .parse()
    .unwrap();
    stock(&fixture.root(), &["new", "-m", "Second"]);
    fs::write(fixture.root().join("file.txt"), "second committed tree\n").unwrap();
    stock(&fixture.root(), &["describe", "-m", "Second"]);
    stock(
        &fixture.root(),
        &["bookmark", "create", "feature", "-r", "@"],
    );
    stock(&fixture.root(), &["new", "-m", "WIP"]);
    let operation = stock(
        &fixture.root(),
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
    fs::write(fixture.root().join("file.txt"), "new unsnapshotted WIP\n").unwrap();
    let expected = Executor::new(1)
        .unwrap()
        .working_tree_current(fixture.access(), fixture.root(), operation)
        .await
        .unwrap();
    let before = fixture.state();
    let lease = Coordinator::new(1)
        .unwrap()
        .acquire(fixture.access(), fixture.root(), expected)
        .await
        .unwrap();
    let (lease, prepared) = lease
        .prepare_reword(Reword {
            immutable_heads: vec![],
            principal: m::PrincipalId::new([85; 16]),
            commit: first,
            message: "Reword ancestor".into(),
        })
        .await
        .unwrap();
    assert_eq!(prepared.rewrites.len(), 3);
    assert_eq!(
        prepared
            .bookmarks
            .iter()
            .map(|b| b.name.as_str())
            .collect::<Vec<_>>(),
        ["feature", "main"]
    );
    for rewrite in &prepared.rewrites {
        assert_eq!(rewrite.before.len(), 1);
        assert_eq!(rewrite.after.len(), 1);
        let old = &prepared.source.commits[&rewrite.before[0]];
        let new = &prepared.capsule.commits[&rewrite.after[0]];
        assert_eq!(old.change_id, new.change_id);
        assert_eq!(old.root_tree, new.root_tree);
        assert_eq!(old.author, new.author);
        let expected_parents: Vec<_> = old
            .parents
            .iter()
            .map(|id| {
                prepared
                    .rewrites
                    .iter()
                    .find(|r| r.before.contains(id))
                    .map_or(*id, |r| r.after[0])
            })
            .collect();
        assert_eq!(new.parents, expected_parents);
    }
    for bookmark in &prepared.bookmarks {
        assert_eq!(
            bookmark.before,
            prepared.source.view.local_bookmarks[&bookmark.name]
        );
        assert_eq!(
            bookmark.after,
            prepared.capsule.view.local_bookmarks[&bookmark.name]
        );
    }
    lease.validate_metadata().await.unwrap();
    assert_eq!(fixture.state(), before);
}

#[tokio::test]
async fn actual_prepared_reword_is_compatible_with_reserved_ref_retention() {
    use jj_store_jj::local_metadata::{
        Limits, MetadataStore, StackPreparationKey, StackRetentionOutcome,
    };
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
            immutable_heads: vec![],
            principal: m::PrincipalId::new([85; 16]),
            commit: expected.baseline.commit,
            message: "Retain exact candidate".into(),
        })
        .await
        .unwrap();
    let store = MetadataStore::open(&fixture.root().join(".git"), Limits::default()).unwrap();
    let key = StackPreparationKey {
        repository: expected.baseline.repository,
        physical_workspace: expected.physical_workspace,
        principal: m::PrincipalId::new([85; 16]),
        request: m::RequestId::new([86; 16]),
        intent: m::IntentDigest::new([87; 32]),
        prepared: m::RecordDigest::new([88; 32]),
    };
    let StackRetentionOutcome::Retained(retained) = store
        .retain_stack_execution(&key, &prepared.history)
        .unwrap()
    else {
        panic!("fixture retention must be acknowledged")
    };
    assert_eq!(retained.capsule, prepared.capsule);
    assert_eq!(retained.history.as_ref(), Some(&prepared.history));
    assert_eq!(prepared.history.capsule().unwrap(), prepared.capsule);
    assert!(
        prepared
            .history
            .operations
            .contains_key(&prepared.source.operation_id)
    );
    assert_eq!(store.read_stack_prepared(&key).unwrap().unwrap(), *retained);
    assert!(store.read().unwrap().is_none());
    lease.validate_metadata().await.unwrap();
    assert_eq!(fixture.state(), before);
}
