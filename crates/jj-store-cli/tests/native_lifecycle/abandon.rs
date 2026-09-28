use super::*;
use jj_lib::{merge::Merge, repo_path::RepoPathComponentBuf};
use jj_store_service::{
    commands::{Command, Executor, Limits as CommandLimits},
    mutations::{CommandService, Outcome, Preview, Request as MutationRequest},
};

fn service() -> CommandService {
    CommandService::new(Executor::new(CommandLimits::default(), 2).unwrap())
}
pub(super) async fn seed(
    f: &Fixture,
    loader: &jj_lib::repo::RepoLoader,
) -> (Arc<ReadonlyRepo>, Vec<m::CommitId>) {
    let root = f
        .stores
        .load_at(loader, roots::ROOT_OPERATION)
        .await
        .unwrap();
    let mut ids = vec![];
    let link = f
        .objects
        .put(&jj_store_jj::codec::NativeSymlink("../file0".into()))
        .await
        .unwrap()
        .0;
    let mut entries = vec![
        (
            RepoPathComponentBuf::new("link").unwrap(),
            b::TreeValue::Symlink(b::SymlinkId::from_bytes(link.as_bytes())),
        ),
        (
            RepoPathComponentBuf::new("module").unwrap(),
            b::TreeValue::GitSubmodule(b::CommitId::from_bytes(&[61; 20])),
        ),
    ];
    for n in 0..3u8 {
        let file = f.objects.put_file([n, 0, 255].as_slice()).await.unwrap().0;
        let copy_id = if n == 2 {
            let key = f
                .objects
                .put(&b::CopyHistory {
                    current_path: RepoPathBuf::from_internal_string("file2").unwrap(),
                    parents: vec![b::CopyId::placeholder()],
                    salt: vec![42],
                })
                .await
                .unwrap()
                .0;
            b::CopyId::from_bytes(key.as_bytes())
        } else {
            b::CopyId::placeholder()
        };
        entries.push((
            RepoPathComponentBuf::new(format!("file{n}")).unwrap(),
            b::TreeValue::File {
                id: b::FileId::from_bytes(file.as_bytes()),
                executable: n == 0,
                copy_id,
            },
        ));
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        let tree = f
            .objects
            .put(&b::Tree::from_sorted_entries(entries.clone()))
            .await
            .unwrap()
            .0;
        let mut commit = roots::commit();
        commit.parents = vec![
            ids.last()
                .copied()
                .map(|id: m::CommitId| b::CommitId::from_bytes(id.as_bytes()))
                .unwrap_or_else(|| root.store().root_commit_id().clone()),
        ];
        commit.change_id = b::ChangeId::from_bytes(&[n + 41; 16]);
        commit.description = format!("level {n}");
        commit.root_tree = Merge::resolved(b::TreeId::from_bytes(tree.as_bytes()));
        let id = f.objects.put(&commit).await.unwrap().0;
        ids.push(m::CommitId::from_slice(id.as_bytes()).unwrap());
    }
    let native = |id: m::CommitId| b::CommitId::from_bytes(id.as_bytes());
    let mut view = root.view().store_view().clone();
    view.head_ids = std::collections::HashSet::from([native(ids[2])]);
    for (name, id) in [
        ("abandoned", ids[1]),
        ("also-abandoned", ids[1]),
        ("descendant", ids[2]),
    ] {
        view.wc_commit_ids.insert(name.into(), native(id));
    }
    view.local_bookmarks
        .insert("topic".into(), o::RefTarget::normal(native(ids[1])));
    view.local_bookmarks
        .insert("child".into(), o::RefTarget::normal(native(ids[2])));
    view.local_bookmarks.insert(
        "conflicted".into(),
        o::RefTarget::from_merge(Merge::from_vec(vec![
            Some(native(ids[1])),
            None,
            Some(native(ids[2])),
        ])),
    );
    let op = stage_view(f, &root, view, "abandon seed").await;
    f.session
        .publish(BTreeSet::from([roots::ROOT_OPERATION]), op)
        .await
        .unwrap();
    (f.stores.load_at(loader, op).await.unwrap(), ids)
}
fn input(
    expected: Snapshot,
    commits: Vec<m::CommitId>,
    retain_bookmarks: bool,
    restore_descendants: bool,
    n: u8,
) -> MutationRequest {
    MutationRequest {
        version: 1,
        request_id: m::RequestId::new([n; 16]),
        expected,
        command: Command::Abandon {
            commits,
            retain_bookmarks,
            restore_descendants,
        },
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn typed_abandon_preserves_native_descendant_bookmark_workspace_and_history_semantics() {
    tokio::task::LocalSet::new()
        .run_until(async {
            for retain in [false, true] {
                for restore in [false, true] {
                    let f = Fixture::new().await;
                    let cache = tempfile::tempdir().unwrap();
                    let loader = f
                        .stores
                        .initialize_cache(&settings(), cache.path())
                        .await
                        .unwrap();
                    let (before, ids) = seed(&f, &loader).await;
                    let expected = f.session.snapshot().await.unwrap().manifest().snapshot();
                    let request = input(expected.clone(), vec![ids[1]], retain, restore, 151);
                    let commands = service();
                    let Preview::Prepared { preview } =
                        commands.preview(f.session.clone(), &request).await.unwrap()
                    else {
                        panic!("preview")
                    };
                    assert_eq!(preview.result.abandoned, vec![ids[1]]);
                    assert!(preview.result.skipped_hidden.is_empty());
                    assert_eq!(
                        f.session.snapshot().await.unwrap().manifest().snapshot(),
                        expected
                    );
                    let Outcome::Committed { receipt } =
                        commands.apply(f.session.clone(), &request).await.unwrap()
                    else {
                        panic!("receipt")
                    };
                    let after = f
                        .stores
                        .load_at(&loader, receipt.result.operation)
                        .await
                        .unwrap();
                    let child = after
                        .store()
                        .get_commit(
                            after
                                .view()
                                .get_wc_commit_id("descendant".as_ref())
                                .unwrap(),
                        )
                        .unwrap();
                    assert_eq!(
                        child.parent_ids(),
                        &[b::CommitId::from_bytes(ids[0].as_bytes())]
                    );
                    assert_eq!(child.change_id(), &b::ChangeId::from_bytes(&[43; 16]));
                    let names = child
                        .tree()
                        .entries()
                        .map(|(path, value)| {
                            value.unwrap();
                            path.as_internal_file_string().to_owned()
                        })
                        .collect::<Vec<_>>();
                    assert_eq!(
                        names,
                        if restore {
                            vec!["file0", "file1", "file2", "link", "module"]
                        } else {
                            vec!["file0", "file2", "link", "module"]
                        }
                    );
                    let original = before
                        .store()
                        .get_commit(&b::CommitId::from_bytes(ids[2].as_bytes()))
                        .unwrap();
                    let original_entries = original
                        .tree()
                        .entries()
                        .map(|(p, v)| (p, v.unwrap()))
                        .collect::<std::collections::BTreeMap<_, _>>();
                    for (path, value) in child.tree().entries() {
                        assert_eq!(
                            value.unwrap(),
                            original_entries[&path],
                            "preserve native file/mode/copy/symlink/submodule value"
                        );
                    }
                    let topic = after.view().get_local_bookmark("topic".as_ref());
                    if retain {
                        assert_eq!(
                            topic.as_normal(),
                            Some(&b::CommitId::from_bytes(ids[0].as_bytes()))
                        );
                    } else {
                        assert!(topic.is_absent());
                    }
                    let replacement = after
                        .store()
                        .get_commit(after.view().get_wc_commit_id("abandoned".as_ref()).unwrap())
                        .unwrap();
                    assert_eq!(
                        after.view().get_wc_commit_id("also-abandoned".as_ref()),
                        Some(replacement.id())
                    );
                    assert_eq!(
                        replacement.parent_ids(),
                        &[b::CommitId::from_bytes(ids[0].as_bytes())]
                    );
                    assert_ne!(replacement.change_id(), &b::ChangeId::from_bytes(&[42; 16]));
                    assert!(replacement.description().is_empty());
                    assert_eq!(
                        replacement.tree_ids(),
                        before
                            .store()
                            .get_commit(&b::CommitId::from_bytes(ids[0].as_bytes()))
                            .unwrap()
                            .tree_ids()
                    );
                    assert!(
                        receipt
                            .result
                            .rewrites
                            .iter()
                            .any(|r| r.predecessors == [ids[2]])
                    );
                    assert!(receipt.result.created.contains(&cid(replacement.id())));
                    let _: b::Commit = f.objects.get(m::ObjectKey::Commit(ids[1])).await.unwrap();
                    let historical = f
                        .stores
                        .load_at(&loader, oid(before.op_id()))
                        .await
                        .unwrap();
                    assert_eq!(historical.view().store_view(), before.view().store_view());
                    assert_eq!(
                        commands.apply(f.session.clone(), &request).await.unwrap(),
                        Outcome::Committed {
                            receipt: receipt.clone()
                        }
                    );
                    assert_eq!(
                        f.session.snapshot().await.unwrap().manifest().snapshot(),
                        receipt.accepted
                    );
                }
            }
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn typed_abandon_adjacent_targets_and_hidden_noop_keep_complete_native_history() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let loader = f
                .stores
                .initialize_cache(&settings(), cache.path())
                .await
                .unwrap();
            let (_, ids) = seed(&f, &loader).await;
            let expected = f.session.snapshot().await.unwrap().manifest().snapshot();
            let first = input(expected, vec![ids[0], ids[1]], false, false, 152);
            let Outcome::Committed { receipt } =
                service().apply(f.session.clone(), &first).await.unwrap()
            else {
                panic!("receipt")
            };
            let after = f
                .stores
                .load_at(&loader, receipt.result.operation)
                .await
                .unwrap();
            let child = after
                .store()
                .get_commit(
                    after
                        .view()
                        .get_wc_commit_id("descendant".as_ref())
                        .unwrap(),
                )
                .unwrap();
            assert_eq!(
                child.parent_ids(),
                &[after.store().root_commit_id().clone()]
            );
            assert_eq!(
                child
                    .tree()
                    .entries()
                    .map(|(p, v)| {
                        v.unwrap();
                        p.as_internal_file_string().to_owned()
                    })
                    .collect::<Vec<_>>(),
                vec!["file2"]
            );
            let unchanged = f.session.snapshot().await.unwrap().manifest().clone();
            let hidden = input(
                unchanged.snapshot(),
                vec![ids[0], ids[1]],
                false,
                false,
                153,
            );
            assert_eq!(
                service()
                    .preview(f.session.clone(), &hidden)
                    .await
                    .unwrap_err(),
                jj_store_service::mutations::Error::Preparation(
                    jj_store_service::commands::Error::NoChanges
                )
            );
            assert_eq!(*f.session.snapshot().await.unwrap().manifest(), unchanged);
            let mixed = input(
                unchanged.snapshot(),
                vec![ids[1], cid(child.id())],
                false,
                false,
                154,
            );
            let Outcome::Committed { receipt: mixed } =
                service().apply(f.session.clone(), &mixed).await.unwrap()
            else {
                panic!("receipt")
            };
            assert_eq!(mixed.result.abandoned, vec![cid(child.id())]);
            assert_eq!(mixed.result.skipped_hidden, vec![ids[1]]);
            assert_eq!(
                service().apply(f.session.clone(), &first).await.unwrap(),
                Outcome::Committed { receipt }
            );
            let current = f.session.snapshot().await.unwrap().manifest().snapshot();
            assert_eq!(current, mixed.accepted);
            let jj_store_service::reads::OpenRead::Pinned(read) =
                f.session.open_read(None, Default::default()).await.unwrap()
            else {
                panic!("read")
            };
            let graphs = tempfile::tempdir().unwrap();
            let exec = jj_store_service::reads::query::GraphExecutor::new(
                graphs.path().to_owned(),
                Default::default(),
                1,
            )
            .unwrap();
            let page = Arc::new(read)
                .graph(
                    &exec,
                    jj_store_service::reads::query::Request::Evolution {
                        commit: cid(child.id()),
                    },
                    jj_store_service::reads::query::Page {
                        limit: 100,
                        after: None,
                    },
                )
                .await
                .unwrap();
            let jj_store_service::reads::query::Nodes::Evolution(nodes) = page.data else {
                panic!("evolution")
            };
            assert!(nodes.iter().any(|node| node.commit.id == ids[2]));
            assert!(nodes.iter().any(|node| node.commit.id == cid(child.id())));
            for id in ids {
                let _: b::Commit = f.objects.get(m::ObjectKey::Commit(id)).await.unwrap();
            }
            assert_eq!(
                f.session.snapshot().await.unwrap().manifest().snapshot(),
                current
            );
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn typed_abandon_merge_retains_ordered_parents_and_native_conflicted_bookmark_terms() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let loader = f
                .stores
                .initialize_cache(&settings(), cache.path())
                .await
                .unwrap();
            let (base, ids) = seed(&f, &loader).await;
            let left = base
                .store()
                .get_commit(&b::CommitId::from_bytes(ids[2].as_bytes()))
                .unwrap();
            let mut tx = base.start_transaction();
            let right = tx
                .repo_mut()
                .new_commit(
                    vec![base.store().root_commit_id().clone()],
                    base.store().empty_merged_tree(),
                )
                .set_description("right parent")
                .write()
                .await
                .unwrap();
            let tree =
                jj_lib::rewrite::merge_commit_trees(tx.repo(), &[left.clone(), right.clone()])
                    .await
                    .unwrap();
            let merge = tx
                .repo_mut()
                .new_commit(vec![left.id().clone(), right.id().clone()], tree)
                .set_description("merge target")
                .write()
                .await
                .unwrap();
            let child = tx
                .repo_mut()
                .new_commit(vec![merge.id().clone()], merge.tree())
                .set_description("merge child")
                .write()
                .await
                .unwrap();
            tx.repo_mut()
                .set_wc_commit("merge-workspace".into(), child.id().clone())
                .unwrap();
            tx.repo_mut().set_local_bookmark_target(
                "merge".as_ref(),
                o::RefTarget::normal(merge.id().clone()),
            );
            tx.commit("merge fixture").await.unwrap();
            let request = input(
                f.session.snapshot().await.unwrap().manifest().snapshot(),
                vec![cid(merge.id())],
                true,
                true,
                155,
            );
            let Outcome::Committed { receipt } =
                service().apply(f.session.clone(), &request).await.unwrap()
            else {
                panic!("receipt")
            };
            let after = f
                .stores
                .load_at(&loader, receipt.result.operation)
                .await
                .unwrap();
            let rewritten = after
                .store()
                .get_commit(
                    after
                        .view()
                        .get_wc_commit_id("merge-workspace".as_ref())
                        .unwrap(),
                )
                .unwrap();
            assert_eq!(
                rewritten.parent_ids(),
                &[left.id().clone(), right.id().clone()]
            );
            assert_eq!(rewritten.tree_ids(), child.tree_ids());
            let bookmark = after.view().get_local_bookmark("merge".as_ref());
            assert!(bookmark.has_conflict());
            assert_eq!(
                bookmark.added_ids().cloned().collect::<BTreeSet<_>>(),
                BTreeSet::from([left.id().clone(), right.id().clone()])
            );
            assert_eq!(receipt.result.abandoned, vec![cid(merge.id())]);
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn typed_abandon_rejects_root_protected_unknown_and_invalid_batches_atomically() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let loader = f
                .stores
                .initialize_cache(&settings(), cache.path())
                .await
                .unwrap();
            let (_, ids) = seed(&f, &loader).await;
            let expected = f.session.snapshot().await.unwrap().manifest().snapshot();
            for commits in [
                vec![],
                vec![ids[1], ids[1]],
                vec![ids[1], roots::ROOT_COMMIT],
                vec![ids[1], m::CommitId::new([252; 64])],
            ] {
                let request = input(expected.clone(), commits, false, false, 156);
                assert!(
                    service()
                        .preview(f.session.clone(), &request)
                        .await
                        .is_err()
                );
                assert!(service().apply(f.session.clone(), &request).await.is_err());
                assert_eq!(
                    f.session.snapshot().await.unwrap().manifest().snapshot(),
                    expected
                );
            }
            for protection in [
                Protection {
                    commits: BTreeSet::from([ids[1]]),
                    ..Default::default()
                },
                Protection {
                    bookmarks: BTreeSet::from(["topic".into()]),
                    ..Default::default()
                },
            ] {
                let session = f.session(f.token.clone(), protection);
                let request = input(expected.clone(), vec![ids[1]], false, false, 157);
                assert_eq!(
                    service()
                        .preview(session.clone(), &request)
                        .await
                        .unwrap_err(),
                    jj_store_service::mutations::Error::Protected
                );
                assert_eq!(
                    service().apply(session, &request).await.unwrap_err(),
                    jj_store_service::mutations::Error::Protected
                );
                assert_eq!(
                    f.session.snapshot().await.unwrap().manifest().snapshot(),
                    expected
                );
            }
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn typed_abandon_report_extension_preserves_legacy_v1_canonical_receipt_bytes() {
    let f = Fixture::new().await;
    let request = MutationRequest {
        version: 1,
        request_id: m::RequestId::new([158; 16]),
        expected: f.session.snapshot().await.unwrap().manifest().snapshot(),
        command: Command::New {
            parents: vec![roots::ROOT_COMMIT],
            description: "legacy report".into(),
            workspace: None,
        },
    };
    let Outcome::Committed { receipt } =
        service().apply(f.session.clone(), &request).await.unwrap()
    else {
        panic!("receipt")
    };
    let bytes = serde_json::to_vec(&receipt.result).unwrap();
    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert!(value.get("abandoned").is_none());
    assert!(value.get("skipped_hidden").is_none());
    let decoded: jj_store_service::commands::Prepared = serde_json::from_slice(&bytes).unwrap();
    assert!(decoded.abandoned.is_empty() && decoded.skipped_hidden.is_empty());
    assert_eq!(serde_json::to_vec(&decoded).unwrap(), bytes);
    assert_eq!(
        service().apply(f.session.clone(), &request).await.unwrap(),
        Outcome::Committed { receipt }
    );
}
