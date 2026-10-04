use super::*;
use jj_store_jj::query::content::operations as d;

fn op_id(operation: &o::Operation) -> m::OperationId {
    m::OperationId::from_slice(operation.key().as_bytes()).unwrap()
}
fn difference(response: d::Response) -> d::Difference {
    let d::Data::Difference(value) = response.data else {
        panic!("expected difference")
    };
    *value
}
async fn pinned(f: &Fixture, operation: m::OperationId) -> Arc<CommittedRead> {
    let OpenRead::Pinned(read) = f
        .session
        .open_read(Some(operation), ReadLimits::default())
        .await
        .unwrap()
    else {
        unreachable!()
    };
    Arc::new(read)
}
async fn publish(f: &Fixture, operation: &o::Operation) -> m::OperationId {
    let key = f.objects.put(operation).await.unwrap().0;
    let id = m::OperationId::from_slice(key.as_bytes()).unwrap();
    f.session
        .publish(operation.parents.iter().map(oid).collect(), id)
        .await
        .unwrap();
    id
}
fn apply_map<T: Clone + std::fmt::Debug + PartialEq>(
    map: &mut BTreeMap<String, T>,
    changes: &[d::NamedChange<T>],
) {
    for change in changes {
        assert_eq!(map.get(&change.name), change.before.as_ref());
        if let Some(after) = &change.after {
            map.insert(change.name.clone(), after.clone());
        } else {
            map.remove(&change.name);
        }
    }
}
fn apply_view(mut before: dto::View, changes: &d::ViewDelta) -> dto::View {
    let mut heads: BTreeSet<_> = before.head_ids.into_iter().collect();
    for removed in &changes.heads.removed {
        assert!(heads.remove(removed));
    }
    for added in &changes.heads.added {
        assert!(heads.insert(*added));
    }
    before.head_ids = heads.into_iter().collect();
    apply_map(&mut before.local_bookmarks, &changes.local_bookmarks);
    apply_map(&mut before.local_tags, &changes.local_tags);
    apply_map(&mut before.remote_views, &changes.remote_views);
    apply_map(&mut before.git_refs, &changes.git_refs);
    apply_map(&mut before.git_heads, &changes.git_heads);
    apply_map(&mut before.wc_commit_ids, &changes.workspaces);
    before
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn operation_diff_preserves_every_view_field_and_excludes_hidden_predecessors() {
    let f = ContentFixture::new().await;
    let initial = difference(
        f.read
            .operation_diff(
                &reader(),
                d::Request {
                    operation: op_id(&f.operation),
                    parent: None,
                },
            )
            .await
            .unwrap(),
    );
    assert_eq!(initial.visible_commits.added, vec![commit_id(&f)]);
    assert!(initial.visible_commits.removed.is_empty());
    assert_eq!(initial.commit_predecessors, None);
    let native_root = dto::View::try_from(&roots::view()).unwrap();
    assert_eq!(
        apply_view(native_root, &initial.view),
        dto::View::try_from(&f.view).unwrap()
    );
    let root = b::CommitId::from_bytes(roots::ROOT_COMMIT.as_bytes());
    let mut view = f.view.clone();
    view.head_ids = [root.clone()].into_iter().collect();
    view.local_bookmarks.clear();
    view.local_bookmarks
        .insert("new".into(), o::RefTarget::normal(root.clone()));
    view.local_tags.insert(
        "v1".into(),
        f.view.local_bookmarks.values().next().unwrap().clone(),
    );
    let origin = view
        .remote_views
        .get_mut(jj_lib::ref_name::RemoteName::new("origin"))
        .unwrap();
    origin.bookmarks.clear();
    for target in origin.tags.values_mut() {
        target.state = o::RemoteRefState::Tracked;
    }
    view.remote_views
        .insert("empty".into(), o::RemoteView::default());
    view.git_refs.clear();
    view.git_heads
        .insert("laptop".into(), o::RefTarget::normal(root.clone()));
    view.git_heads
        .insert("server".into(), o::RefTarget::normal(root.clone()));
    view.wc_commit_ids.clear();
    view.wc_commit_ids.insert("laptop".into(), root.clone());
    view.wc_commit_ids.insert("server".into(), root);
    let view_key = f.f.objects.put(&view).await.unwrap().0;
    let mut operation = f.operation.clone();
    operation.parents = vec![o::OperationId::from_bytes(f.operation.key().as_bytes())];
    operation.view_id = o::ViewId::from_bytes(view_key.as_bytes());
    operation.commit_predecessors = Some(BTreeMap::new());
    let id = publish(&f.f, &operation).await;
    let read = pinned(&f.f, id).await;
    let before = f.f.session.snapshot().await.unwrap().manifest().clone();
    let result = difference(
        read.operation_diff(
            &reader(),
            d::Request {
                operation: id,
                parent: None,
            },
        )
        .await
        .unwrap(),
    );
    assert_eq!(
        apply_view(dto::View::try_from(&f.view).unwrap(), &result.view),
        dto::View::try_from(&view).unwrap()
    );
    assert_eq!(result.visible_commits.removed, vec![commit_id(&f)]);
    assert!(result.visible_commits.added.is_empty());
    assert_eq!(result.commit_predecessors, Some(BTreeMap::new()));
    assert_eq!(
        result.before_view,
        m::ViewId::from_slice(f.view.key().as_bytes()).unwrap()
    );
    assert_eq!(
        result.after_view,
        m::ViewId::from_slice(view_key.as_bytes()).unwrap()
    );
    assert_eq!(*f.f.session.snapshot().await.unwrap().manifest(), before);
}

async fn visible(repo: &ReadonlyRepo) -> BTreeSet<m::CommitId> {
    jj_lib::revset::ResolvedRevsetExpression::all()
        .evaluate(repo)
        .unwrap()
        .stream()
        .map_ok(|id| cid(&id))
        .try_collect()
        .await
        .unwrap()
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn operation_diff_matches_native_revsets_after_rewrite_and_abandon() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let original = init_working(&f, cache.path(), work.path()).await;
            let old = original
                .store()
                .get_commit(
                    original
                        .view()
                        .get_wc_commit_id(jj_lib::ref_name::WorkspaceName::new("fixture"))
                        .unwrap(),
                )
                .unwrap();
            let (rewritten, commit) = describe(&original, &old, "operation diff rewrite").await;
            let mut tx = rewritten.start_transaction();
            let descendant = tx
                .repo_mut()
                .new_commit(vec![commit.id().clone()], commit.tree())
                .set_description("descendant retains parent visibility")
                .write()
                .await
                .unwrap();
            tx.repo_mut()
                .set_wc_commit("fixture".into(), descendant.id().clone())
                .unwrap();
            tx.repo_mut().set_local_bookmark_target(
                jj_lib::ref_name::RefName::new("parent"),
                o::RefTarget::normal(commit.id().clone()),
            );
            tx.repo_mut().set_local_bookmark_target(
                jj_lib::ref_name::RefName::new("child"),
                o::RefTarget::normal(descendant.id().clone()),
            );
            let child = tx.commit("native child").await.unwrap();
            let mut tx = child.start_transaction();
            tx.repo_mut().record_abandoned_commit(&commit);
            tx.repo_mut().rebase_descendants().await.unwrap();
            let abandoned = tx.commit("operation diff abandon").await.unwrap();
            for (before, after) in [
                (&original, &rewritten),
                (&rewritten, &child),
                (&child, &abandoned),
            ] {
                let id = oid(after.op_id());
                let read = pinned(&f, id).await;
                let result = difference(
                    read.operation_diff(
                        &reader(),
                        d::Request {
                            operation: id,
                            parent: Some(oid(before.op_id())),
                        },
                    )
                    .await
                    .unwrap(),
                );
                let old = visible(before).await;
                let new = visible(after).await;
                assert_eq!(
                    result.visible_commits.added,
                    new.difference(&old).copied().collect::<Vec<_>>()
                );
                assert_eq!(
                    result.visible_commits.removed,
                    old.difference(&new).copied().collect::<Vec<_>>()
                );
                assert_eq!(
                    apply_view(
                        dto::View::try_from(before.view().store_view()).unwrap(),
                        &result.view
                    ),
                    dto::View::try_from(after.view().store_view()).unwrap()
                );
                assert_eq!(
                    result.commit_predecessors,
                    dto::Operation::try_from(after.operation().store_operation())
                        .unwrap()
                        .commit_predecessors
                );
            }
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn operation_diff_merge_parent_selection_and_root_are_explicit() {
    let f = ContentFixture::new().await;
    let mut left = f.operation.clone();
    left.parents = vec![o::OperationId::from_bytes(f.operation.key().as_bytes())];
    left.metadata.description = "left".into();
    let left_id = publish(&f.f, &left).await;
    let mut right = left.clone();
    right.metadata.description = "right".into();
    right.view_id = o::ViewId::from_bytes(roots::ROOT_VIEW.as_bytes());
    let right_id = publish(&f.f, &right).await;
    let mut merge = left.clone();
    merge.parents = vec![
        o::OperationId::from_bytes(left_id.as_bytes()),
        o::OperationId::from_bytes(right_id.as_bytes()),
    ];
    let merged_id = publish(&f.f, &merge).await;
    let read = pinned(&f.f, merged_id).await;
    let before = f.f.session.snapshot().await.unwrap().manifest().clone();
    assert_eq!(
        read.operation_diff(
            &reader(),
            d::Request {
                operation: merged_id,
                parent: None
            }
        )
        .await
        .unwrap()
        .data,
        d::Data::SelectParent {
            operation: merged_id,
            parents: vec![left_id, right_id]
        }
    );
    for parent in [left_id, right_id] {
        let result = difference(
            read.operation_diff(
                &reader(),
                d::Request {
                    operation: merged_id,
                    parent: Some(parent),
                },
            )
            .await
            .unwrap(),
        );
        assert_eq!(result.parent, parent);
        if parent == left_id {
            assert!(result.visible_commits.added.is_empty());
            assert!(result.view.workspaces.is_empty());
        } else {
            assert_eq!(result.visible_commits.added, vec![commit_id(&f)]);
            assert_eq!(result.view.workspaces.len(), 2);
        }
        assert!(result.visible_commits.removed.is_empty());
    }
    for parent in [roots::ROOT_OPERATION, op_id(&f.operation), merged_id] {
        assert_eq!(
            read.operation_diff(
                &reader(),
                d::Request {
                    operation: merged_id,
                    parent: Some(parent)
                }
            )
            .await
            .unwrap_err(),
            q::Error::Invalid
        );
    }
    assert_eq!(
        read.operation_diff(
            &reader(),
            d::Request {
                operation: roots::ROOT_OPERATION,
                parent: None
            }
        )
        .await
        .unwrap()
        .data,
        d::Data::Root {
            operation: roots::ROOT_OPERATION,
            view: roots::ROOT_VIEW
        }
    );
    assert_eq!(
        read.operation_diff(
            &reader(),
            d::Request {
                operation: roots::ROOT_OPERATION,
                parent: Some(roots::ROOT_OPERATION)
            }
        )
        .await
        .unwrap_err(),
        q::Error::Invalid
    );
    assert_eq!(*f.f.session.snapshot().await.unwrap().manifest(), before);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn operation_diff_rejects_limits_staged_later_and_revoked_reads() {
    use jj_store_service::administration::{TokenChange, TokenRequest};
    let f = ContentFixture::new().await;
    let request = d::Request {
        operation: op_id(&f.operation),
        parent: None,
    };
    for limits in [
        c::Limits {
            max_history_visits: 1,
            ..c::Limits::default()
        },
        c::Limits {
            max_read_bytes: 1,
            ..c::Limits::default()
        },
        c::Limits {
            max_output_bytes: 1,
            ..c::Limits::default()
        },
        c::Limits {
            max_work: 1,
            ..c::Limits::default()
        },
    ] {
        let reader = ContentReader::new(limits, 1).unwrap();
        assert_eq!(
            f.read
                .operation_diff(&reader, request.clone())
                .await
                .unwrap_err(),
            q::Error::Limit
        );
    }
    let mut later = f.operation.clone();
    later.parents = vec![o::OperationId::from_bytes(f.operation.key().as_bytes())];
    later.metadata.description = "later".into();
    let key = f.f.objects.put(&later).await.unwrap().0;
    let later_request = d::Request {
        operation: m::OperationId::from_slice(key.as_bytes()).unwrap(),
        parent: None,
    };
    assert_eq!(
        f.read
            .operation_diff(&reader(), later_request.clone())
            .await
            .unwrap_err(),
        q::Error::NotCommitted
    );
    let id = publish(&f.f, &later).await;
    assert_eq!(
        f.read
            .operation_diff(&reader(), later_request)
            .await
            .unwrap_err(),
        q::Error::NotCommitted
    );
    let session = f.f.session(f.f.reader.clone(), Protection::default());
    let OpenRead::Pinned(read) = session
        .open_read(Some(id), ReadLimits::default())
        .await
        .unwrap()
    else {
        unreachable!()
    };
    let read = Arc::new(read);
    f.f.session
        .change_token(&TokenRequest {
            request_id: m::RequestId::new([236; 16]),
            expected: f.f.session.snapshot().await.unwrap().manifest().snapshot(),
            change: TokenChange::Revoke {
                token: f.f.reader.id(),
            },
        })
        .await
        .unwrap();
    assert_eq!(
        read.operation_diff(&reader(), request).await.unwrap_err(),
        q::Error::Denied
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn operation_diff_rejects_missing_and_corrupt_visible_history() {
    let f = ContentFixture::new().await;
    let request = d::Request {
        operation: op_id(&f.operation),
        parent: None,
    };
    let key = Key::new(f.commit.key().storage_path(REPO)).unwrap();
    let store = Arc::new(MissingNative {
        inner: f.f.store.clone(),
        missing: key.clone(),
    });
    assert_eq!(
        reader()
            .operation_diff(store, f.read.clone(), request.clone())
            .await
            .unwrap_err(),
        q::Error::Incomplete
    );
    let version = f.f.store.head(&key).await.unwrap().version;
    let mut bytes =
        f.f.store
            .get(&key, None)
            .await
            .unwrap()
            .read_bounded(1_000_000)
            .await
            .unwrap();
    *bytes.last_mut().unwrap() ^= 1;
    let upload = StagedUpload::from_bytes(&bytes, 1_000_000).await.unwrap();
    assert!(matches!(
        f.f.store
            .put(&key, Condition::Matches(version), &upload)
            .await
            .unwrap(),
        WriteOutcome::Applied(_)
    ));
    assert_eq!(
        f.read.operation_diff(&reader(), request).await.unwrap_err(),
        q::Error::Corrupt
    );
}
