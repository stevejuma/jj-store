use super::*;
use jj_store_service::{
    commands::{Command, Executor, Limits as CommandLimits},
    mutations::{CommandService, Outcome, Preview, Request as MutationRequest},
};

fn service() -> CommandService {
    CommandService::new(Executor::new(CommandLimits::default(), 2).unwrap())
}
fn reconcile(expected: Snapshot, base: m::OperationId, id: u8) -> MutationRequest {
    MutationRequest {
        version: 1,
        request_id: m::RequestId::new([id; 16]),
        expected,
        command: Command::ReconcileOperations { base },
    }
}
pub(super) async fn seed(
    f: &Fixture,
    loader: &jj_lib::repo::RepoLoader,
) -> (
    Arc<ReadonlyRepo>,
    Arc<ReadonlyRepo>,
    m::CommitId,
    m::CommitId,
) {
    let root = f
        .stores
        .load_at(loader, roots::ROOT_OPERATION)
        .await
        .unwrap();
    let mut tx = root.start_transaction();
    let original = tx
        .repo_mut()
        .new_commit(
            vec![root.store().root_commit_id().clone()],
            root.store().empty_merged_tree(),
        )
        .set_description("original")
        .write()
        .await
        .unwrap();
    tx.repo_mut().set_local_bookmark_target(
        "topic".as_ref(),
        o::RefTarget::normal(original.id().clone()),
    );
    let base = tx.commit("common ancestor").await.unwrap();
    let mut left = base.start_transaction();
    let rewritten = left
        .repo_mut()
        .rewrite_commit(&original)
        .set_description("left rewrite")
        .write()
        .await
        .unwrap();
    left.repo_mut().rebase_descendants().await.unwrap();
    let left = left.commit("left").await.unwrap();
    let mut right = base.start_transaction();
    let child = right
        .repo_mut()
        .new_commit(vec![original.id().clone()], original.tree())
        .set_description("right child")
        .write()
        .await
        .unwrap();
    right
        .repo_mut()
        .set_wc_commit("right-workspace".into(), child.id().clone())
        .unwrap();
    right
        .repo_mut()
        .set_local_bookmark_target("child".as_ref(), o::RefTarget::normal(child.id().clone()));
    let right = right.commit("right").await.unwrap();
    (left, right, cid(rewritten.id()), cid(child.id()))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn typed_reconciliation_recursive_ancestors_and_conflicted_bookmarks_match_upstream() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let loader = f
                .stores
                .initialize_cache(&settings(), cache.path())
                .await
                .unwrap();
            let root = f
                .stores
                .load_at(&loader, roots::ROOT_OPERATION)
                .await
                .unwrap();
            let mut branches = vec![];
            for description in ["left", "right", "third"] {
                let mut tx = root.start_transaction();
                let new = tx
                    .repo_mut()
                    .new_commit(
                        vec![root.store().root_commit_id().clone()],
                        root.store().empty_merged_tree(),
                    )
                    .set_description(description)
                    .write()
                    .await
                    .unwrap();
                tx.repo_mut().set_local_bookmark_target(
                    "topic".as_ref(),
                    o::RefTarget::normal(new.id().clone()),
                );
                branches.push(tx.commit(description).await.unwrap());
            }
            let (first, _) = loader
                .merge_operations(
                    vec![
                        branches[0].operation().clone(),
                        branches[1].operation().clone(),
                    ],
                    None,
                    Some("first merge"),
                    [],
                )
                .await
                .unwrap();
            let (second, _) = loader
                .merge_operations(
                    vec![
                        branches[1].operation().clone(),
                        branches[0].operation().clone(),
                    ],
                    None,
                    Some("second merge"),
                    [],
                )
                .await
                .unwrap();
            let removed = branches[..2]
                .iter()
                .map(|r| oid(r.op_id()))
                .collect::<BTreeSet<_>>();
            f.session
                .publish(removed.clone(), oid(first.op_id()))
                .await
                .unwrap();
            f.session
                .publish(removed, oid(second.op_id()))
                .await
                .unwrap();
            let expected = f.session.snapshot().await.unwrap().manifest().snapshot();
            assert_eq!(expected.heads.len(), 3);
            let mut others = [second.operation().clone(), branches[2].operation().clone()];
            others.sort_by(|a, b| a.id().cmp(b.id()));
            let ordered = std::iter::once(first.operation().clone())
                .chain(others)
                .collect::<Vec<_>>();
            let parents = ordered.iter().map(|op| op.id().clone()).collect::<Vec<_>>();
            let (upstream, _) = loader
                .merge_operations(ordered, None, Some("reference"), [])
                .await
                .unwrap();
            assert!(
                upstream
                    .view()
                    .get_local_bookmark("topic".as_ref())
                    .has_conflict()
            );
            let input = reconcile(expected.clone(), oid(first.op_id()), 146);
            let Preview::Prepared { preview } =
                service().preview(f.session.clone(), &input).await.unwrap()
            else {
                panic!("preview")
            };
            let candidate = f
                .stores
                .load_at(&loader, preview.result.operation)
                .await
                .unwrap();
            assert_eq!(candidate.view().store_view(), upstream.view().store_view());
            assert_eq!(candidate.operation().parent_ids(), parents);
            assert_eq!(
                f.session.snapshot().await.unwrap().manifest().snapshot(),
                expected
            );
            let Outcome::Committed { receipt } =
                service().apply(f.session.clone(), &input).await.unwrap()
            else {
                panic!("receipt")
            };
            let accepted = f
                .stores
                .load_at(&loader, receipt.result.operation)
                .await
                .unwrap();
            assert_eq!(accepted.view().store_view(), upstream.view().store_view());
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn typed_reconciliation_resource_and_protection_failures_preserve_heads() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let loader = f
                .stores
                .initialize_cache(&settings(), cache.path())
                .await
                .unwrap();
            let (left, _, _, child) = seed(&f, &loader).await;
            let expected = f.session.snapshot().await.unwrap().manifest().snapshot();
            let input = reconcile(expected.clone(), oid(left.op_id()), 147);
            let protected = f.session(
                f.token.clone(),
                Protection {
                    commits: BTreeSet::from([child]),
                    ..Default::default()
                },
            );
            for preview in [true, false] {
                let error = if preview {
                    service()
                        .preview(protected.clone(), &input)
                        .await
                        .unwrap_err()
                } else {
                    service()
                        .apply(protected.clone(), &input)
                        .await
                        .unwrap_err()
                };
                assert_eq!(error, jj_store_service::mutations::Error::Denied);
            }
            let limited = CommandService::new(
                Executor::new(
                    CommandLimits {
                        max_staged_objects: 1,
                        ..Default::default()
                    },
                    1,
                )
                .unwrap(),
            );
            assert_eq!(
                limited
                    .preview(f.session.clone(), &input)
                    .await
                    .unwrap_err(),
                jj_store_service::mutations::Error::Limit
            );
            let limited = CommandService::new(
                Executor::new(
                    CommandLimits {
                        query: jj_store_service::reads::query::Limits {
                            max_work: 1,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    1,
                )
                .unwrap(),
            );
            assert_eq!(
                limited
                    .preview(f.session.clone(), &input)
                    .await
                    .unwrap_err(),
                jj_store_service::mutations::Error::Limit
            );
            assert_eq!(
                f.session.snapshot().await.unwrap().manifest().snapshot(),
                expected
            );
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn typed_reconciliation_merges_native_heads_rebases_descendants_and_replays_receipt() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let loader = f
                .stores
                .initialize_cache(&settings(), cache.path())
                .await
                .unwrap();
            let (left, right, rewritten, child) = seed(&f, &loader).await;
            let expected = f.session.snapshot().await.unwrap().manifest().snapshot();
            assert_eq!(
                expected.heads,
                BTreeSet::from([oid(left.op_id()), oid(right.op_id())])
            );
            let request = reconcile(expected.clone(), oid(left.op_id()), 141);
            let commands = service();
            let Preview::Prepared { preview } =
                commands.preview(f.session.clone(), &request).await.unwrap()
            else {
                panic!("preview expected")
            };
            assert_eq!(
                f.session.snapshot().await.unwrap().manifest().snapshot(),
                expected
            );
            let candidate = f
                .stores
                .load_at(&loader, preview.result.operation)
                .await
                .unwrap();
            assert_eq!(
                candidate.operation().parent_ids(),
                &[left.op_id().clone(), right.op_id().clone()]
            );
            let descendant = candidate
                .store()
                .get_commit(
                    candidate
                        .view()
                        .get_wc_commit_id("right-workspace".as_ref())
                        .unwrap(),
                )
                .unwrap();
            assert_eq!(
                descendant.parent_ids(),
                &[b::CommitId::from_bytes(rewritten.as_bytes())]
            );
            assert_ne!(cid(descendant.id()), child);
            assert_eq!(
                candidate
                    .view()
                    .get_local_bookmark("child".as_ref())
                    .as_normal(),
                Some(descendant.id())
            );
            assert!(
                preview
                    .result
                    .rewrites
                    .iter()
                    .any(|r| r.predecessors.contains(&child))
            );
            assert!(preview.result.workspaces.contains_key("right-workspace"));
            let Outcome::Committed { receipt } =
                commands.apply(f.session.clone(), &request).await.unwrap()
            else {
                panic!("receipt expected")
            };
            assert_eq!(
                receipt.accepted.position,
                m::WalPosition(expected.position.0 + 1)
            );
            assert_eq!(receipt.accepted.heads.len(), 1);
            assert_eq!(receipt.request_digest, preview.request_digest);
            let selected = f.session.snapshot().await.unwrap().manifest().snapshot();
            assert_eq!(
                service().apply(f.session.clone(), &request).await.unwrap(),
                Outcome::Committed { receipt }
            );
            assert_eq!(
                f.session.snapshot().await.unwrap().manifest().snapshot(),
                selected
            );
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn typed_reconciliation_requires_exact_heads_and_explicit_current_base() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let loader = f
                .stores
                .initialize_cache(&settings(), cache.path())
                .await
                .unwrap();
            let (left, right, _, _) = seed(&f, &loader).await;
            let expected = f.session.snapshot().await.unwrap().manifest().snapshot();
            let commands = service();
            assert!(
                commands
                    .preview(
                        f.session.clone(),
                        &reconcile(expected.clone(), roots::ROOT_OPERATION, 142)
                    )
                    .await
                    .is_err()
            );
            let mut subset = expected.clone();
            subset.heads.remove(&oid(right.op_id()));
            assert!(matches!(
                commands
                    .apply(
                        f.session.clone(),
                        &reconcile(subset, oid(left.op_id()), 143)
                    )
                    .await,
                Err(jj_store_service::mutations::Error::Stale)
            ));
            assert_eq!(
                f.session.snapshot().await.unwrap().manifest().snapshot(),
                expected
            );
            let request = reconcile(expected.clone(), oid(left.op_id()), 144);
            let accepted = commands.apply(f.session.clone(), &request).await.unwrap();
            assert!(matches!(accepted, Outcome::Committed { .. }));
            assert!(matches!(
                commands
                    .apply(
                        f.session.clone(),
                        &reconcile(expected, oid(right.op_id()), 144)
                    )
                    .await,
                Err(jj_store_service::mutations::Error::IntentConflict)
            ));
            let single = f.session.snapshot().await.unwrap().manifest().snapshot();
            assert!(
                commands
                    .preview(
                        f.session.clone(),
                        &reconcile(single.clone(), *single.heads.first().unwrap(), 145)
                    )
                    .await
                    .is_err()
            );
            assert_eq!(
                f.session.snapshot().await.unwrap().manifest().snapshot(),
                single
            );
        })
        .await;
}
