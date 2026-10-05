use super::*;
use jj_lib::{
    copies::{CopyOperation, CopyRecords},
    diff::{ContentDiff, DiffHunkKind},
    matchers::EverythingMatcher,
    signing::Signer,
    store::Store,
    tree_merge::MergeOptions,
};
use jj_store_jj::{
    backend::NativeBackend,
    query::content::{conflicts as cf, diff as d},
};

struct History {
    f: Fixture,
    commit: m::CommitId,
    operation: m::OperationId,
    sequence: u8,
}
impl History {
    async fn new() -> Self {
        Self {
            f: Fixture::new().await,
            commit: roots::ROOT_COMMIT,
            operation: roots::ROOT_OPERATION,
            sequence: 0,
        }
    }
    async fn file(&self, bytes: &[u8]) -> b::TreeValue {
        let (id, _) = self.f.objects.put_file(bytes).await.unwrap();
        b::TreeValue::File {
            id: b::FileId::from_bytes(id.as_bytes()),
            executable: false,
            copy_id: b::CopyId::placeholder(),
        }
    }
    async fn tree(&self, entries: Vec<(&str, b::TreeValue)>) -> b::TreeId {
        let mut entries = entries;
        entries.sort_by_key(|(name, _)| *name);
        let key = self.f.objects.put(&tree(entries)).await.unwrap().0;
        b::TreeId::from_bytes(key.as_bytes())
    }
    async fn publish(&mut self, trees: Vec<b::TreeId>, labels: Vec<&str>) -> Arc<CommittedRead> {
        self.sequence += 1;
        let mut commit = b::make_root_commit(
            b::ChangeId::from_bytes(&[self.sequence; 16]),
            trees[0].clone(),
        );
        commit.parents = vec![b::CommitId::from_bytes(self.commit.as_bytes())];
        commit.root_tree = Merge::from_vec(trees);
        commit.conflict_labels =
            Merge::from_vec(labels.into_iter().map(str::to_owned).collect::<Vec<_>>());
        let key = self.f.objects.put(&commit).await.unwrap().0;
        let view = o::View::make_root(b::CommitId::from_bytes(key.as_bytes()));
        let viewkey = self.f.objects.put(&view).await.unwrap().0;
        let mut op = o::Operation::make_root(o::ViewId::from_bytes(viewkey.as_bytes()));
        op.parents = vec![o::OperationId::from_bytes(self.operation.as_bytes())];
        op.metadata.description = format!("content {}", self.sequence);
        let opkey = self.f.objects.put(&op).await.unwrap().0;
        let operation = m::OperationId::from_slice(opkey.as_bytes()).unwrap();
        self.f
            .session
            .publish(BTreeSet::from([self.operation]), operation)
            .await
            .unwrap();
        self.commit = m::CommitId::from_slice(key.as_bytes()).unwrap();
        self.operation = operation;
        let OpenRead::Pinned(read) = self
            .f
            .session
            .open_read(None, ReadLimits::default())
            .await
            .unwrap()
        else {
            unreachable!()
        };
        Arc::new(read)
    }
    fn native_store(&self) -> Arc<Store> {
        Store::new(
            Box::new(
                NativeBackend::new(
                    self.f.objects.clone(),
                    self.f.session.clone(),
                    BackendLimits::default(),
                )
                .unwrap(),
            ),
            Signer::new(None, vec![]),
            MergeOptions::from_settings(&settings()).unwrap(),
        )
    }
}
fn unavailable(change: &d::Change, expected: d::Unavailable) {
    assert_eq!(change.content, d::Content::Unavailable { reason: expected });
}
fn text(change: &d::Change) -> &[d::Hunk] {
    let d::Content::Text { hunks } = &change.content else {
        panic!("expected text: {change:?}")
    };
    hunks
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn content_diff_long_file_sparse_edits_remain_text() {
    let mut h = History::new().await;
    let before = (0..2300)
        .map(|n| format!("line {n}\r\n"))
        .collect::<String>();
    let after = before
        .replace("line 20\r\n", "changed 20\r\ninserted\r\n")
        .replace("line 1100\r\n", "changed 1100\r\n")
        .replace("line 2200\r\n", "changed 2200\r\n");
    let value = h.file(before.as_bytes()).await;
    let tree = h.tree(vec![("long.c", value)]).await;
    h.publish(vec![tree], vec![""]).await;
    let from = h.commit;
    let value = h.file(after.as_bytes()).await;
    let tree = h.tree(vec![("long.c", value)]).await;
    let read = h.publish(vec![tree], vec![""]).await;
    let response = read
        .diff(
            &reader(),
            d::Request {
                from,
                to: h.commit,
                path: "long.c".into(),
                content: true,
            },
        )
        .await
        .unwrap();
    let hunks = text(&response.changes[0]);
    assert_eq!(
        hunks.iter().map(|v| v.before.as_str()).collect::<String>(),
        before
    );
    assert_eq!(
        hunks.iter().map(|v| v.after.as_str()).collect::<String>(),
        after
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn content_diff_native_lines_modes_binary_symlinks_submodules_and_prefixes() {
    let mut h = History::new().await;
    let a = "alpha\r\nβeta\nno final newline";
    let b = "alpha\r\nchanged 日本語\nno final newline";
    let first = h.file(a.as_bytes()).await;
    let second = h.file(b.as_bytes()).await;
    let same = h.file(b"same\n").await;
    let mut executable = same.clone();
    let b::TreeValue::File {
        executable: mode, ..
    } = &mut executable
    else {
        unreachable!()
    };
    *mode = true;
    let bin = h.file(&[0, 255, 2]).await;
    let nonutf8 = h.file(&[255, 128]).await;
    let empty = h.file(b"").await;
    let symlink =
        h.f.objects
            .put(&NativeSymlink("../../<script>".into()))
            .await
            .unwrap()
            .0;
    let t1 = h
        .tree(vec![
            ("edit", first),
            ("mode", same.clone()),
            ("remove", same.clone()),
        ])
        .await;
    h.publish(vec![t1], vec![""]).await;
    let from = h.commit;
    let sub = h.tree(vec![("nested", same.clone())]).await;
    let t2 = h
        .tree(vec![
            ("add", same),
            ("bin", bin),
            ("dir", b::TreeValue::Tree(sub)),
            ("edit", second),
            ("empty", empty),
            ("nonutf8", nonutf8),
            (
                "link",
                b::TreeValue::Symlink(b::SymlinkId::from_bytes(symlink.as_bytes())),
            ),
            ("mode", executable),
            (
                "module",
                b::TreeValue::GitSubmodule(b::CommitId::from_bytes(&[7; 20])),
            ),
        ])
        .await;
    let read = h.publish(vec![t2], vec![""]).await;
    let to = h.commit;
    let before = h.f.session.snapshot().await.unwrap().manifest().clone();
    let request = d::Request {
        from,
        to,
        path: String::new(),
        content: true,
    };
    let result = read.diff(&reader(), request.clone()).await.unwrap();
    let paths = result
        .changes
        .iter()
        .map(|c| c.path.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        paths,
        vec![
            "add",
            "bin",
            "dir/nested",
            "edit",
            "empty",
            "link",
            "mode",
            "module",
            "nonutf8",
            "remove"
        ]
    );
    let change = |path: &str| result.changes.iter().find(|c| c.path == path).unwrap();
    unavailable(change("bin"), d::Unavailable::Binary);
    unavailable(change("nonutf8"), d::Unavailable::Binary);
    assert!(
        text(change("empty"))
            .iter()
            .all(|h| h.before.is_empty() && h.after.is_empty())
    );
    unavailable(change("link"), d::Unavailable::Structural);
    unavailable(change("module"), d::Unavailable::Structural);
    let hunks = text(change("edit"));
    assert_eq!(
        hunks.iter().map(|h| h.before.as_str()).collect::<String>(),
        a
    );
    assert_eq!(
        hunks.iter().map(|h| h.after.as_str()).collect::<String>(),
        b
    );
    let native = ContentDiff::by_line([a.as_bytes(), b.as_bytes()]);
    for (owned, native) in hunks.iter().zip(native.hunks()) {
        let native_before: &[u8] = native.contents[0].as_ref();
        assert_eq!(owned.before.as_bytes(), native_before);
        let native_after: &[u8] = native.contents[1].as_ref();
        assert_eq!(owned.after.as_bytes(), native_after);
        assert_eq!(
            matches!(owned.kind, d::HunkKind::Different),
            native.kind == DiffHunkKind::Different
        );
    }
    assert_eq!(hunks.len(), native.hunks().count());
    assert!(
        text(change("mode"))
            .iter()
            .all(|h| h.kind == d::HunkKind::Matching)
    );
    assert!(text(change("add")).iter().all(|h| h.before.is_empty()));
    assert!(text(change("remove")).iter().all(|h| h.after.is_empty()));
    let filtered = read
        .diff(
            &reader(),
            d::Request {
                path: "dir".into(),
                content: false,
                ..request.clone()
            },
        )
        .await
        .unwrap();
    assert_eq!(filtered.changes.len(), 1);
    assert_eq!(filtered.changes[0].path, "dir/nested");
    unavailable(&filtered.changes[0], d::Unavailable::NotRequested);
    assert!(
        read.diff(
            &reader(),
            d::Request {
                from: to,
                ..request
            }
        )
        .await
        .unwrap()
        .changes
        .is_empty()
    );
    assert_eq!(*h.f.session.snapshot().await.unwrap().manifest(), before);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn content_diff_copy_and_rename_sources_match_pinned_native_stream() {
    let mut h = History::new().await;
    let mut file = h.file(b"original\n").await;
    let base = b::CopyHistory {
        current_path: RepoPathBuf::from_internal_string("source").unwrap(),
        parents: vec![b::CopyId::placeholder()],
        salt: vec![21],
    };
    let base_key = h.f.objects.put(&base).await.unwrap().0;
    let b::TreeValue::File { copy_id, .. } = &mut file else {
        unreachable!()
    };
    *copy_id = b::CopyId::from_bytes(base_key.as_bytes());
    let tree = h.tree(vec![("source", file.clone())]).await;
    h.publish(vec![tree], vec![""]).await;
    let from = h.commit;
    let history = b::CopyHistory {
        current_path: RepoPathBuf::from_internal_string("target").unwrap(),
        parents: vec![b::CopyId::from_bytes(base_key.as_bytes())],
        salt: vec![22],
    };
    let key = h.f.objects.put(&history).await.unwrap().0;
    let mut target = h.file(b"changed\n").await;
    let b::TreeValue::File { copy_id, .. } = &mut target else {
        unreachable!()
    };
    *copy_id = b::CopyId::from_bytes(key.as_bytes());
    for keep_source in [true, false] {
        let entries = if keep_source {
            vec![("source", file.clone()), ("target", target.clone())]
        } else {
            vec![("target", target.clone())]
        };
        let tree = h.tree(entries).await;
        let read = h.publish(vec![tree], vec![""]).await;
        let response = read
            .diff(
                &reader(),
                d::Request {
                    from,
                    to: h.commit,
                    path: String::new(),
                    content: true,
                },
            )
            .await
            .unwrap();
        let store = h.native_store();
        let old = store
            .get_commit_async(&b::CommitId::from_bytes(from.as_bytes()))
            .await
            .unwrap()
            .tree();
        let new = store
            .get_commit_async(&b::CommitId::from_bytes(h.commit.as_bytes()))
            .await
            .unwrap()
            .tree();
        let mut records = CopyRecords::default();
        records.add_records(
            store
                .get_copy_records(
                    None,
                    &b::CommitId::from_bytes(from.as_bytes()),
                    &b::CommitId::from_bytes(h.commit.as_bytes()),
                )
                .unwrap()
                .try_collect::<Vec<_>>()
                .await
                .unwrap(),
        );
        let native: Vec<_> = old
            .diff_stream_with_copies(&new, &EverythingMatcher, &records)
            .collect()
            .await;
        for native in native {
            let owned = response
                .changes
                .iter()
                .find(|c| c.path == native.path.target.as_internal_file_string())
                .unwrap();
            let values = native.values.unwrap();
            let convert = |v: &b::MergedTreeValue| {
                v.iter()
                    .map(|v| v.as_ref().map(|v| dto::TreeValue::try_from(v).unwrap()))
                    .collect::<Vec<_>>()
            };
            assert_eq!(owned.after, convert(&values.after));
            let original = old.path_value(&native.path.target).await.unwrap();
            assert_eq!(owned.before, convert(&original));
            if let Some((path, kind)) = native.path.source {
                assert_eq!(owned.sources.len(), 1);
                assert_eq!(owned.sources[0].path, path.as_internal_file_string());
                assert_eq!(
                    owned.sources[0].kind,
                    match kind {
                        CopyOperation::Copy => d::SourceKind::Copy,
                        CopyOperation::Rename => d::SourceKind::Rename,
                    }
                );
                assert_eq!(owned.sources[0].terms, convert(&values.before));
            }
        }
        let copied = response
            .changes
            .iter()
            .find(|c| c.path == "target")
            .unwrap();
        assert_eq!(
            copied.sources[0].kind,
            if keep_source {
                d::SourceKind::Copy
            } else {
                d::SourceKind::Rename
            }
        );
        assert_eq!(copied.sources[0].path, "source");
        assert_eq!(
            text(copied)
                .iter()
                .map(|h| h.before.as_str())
                .collect::<String>(),
            "original\n"
        );
        if !keep_source {
            let reverse = read
                .diff(
                    &reader(),
                    d::Request {
                        from: h.commit,
                        to: from,
                        path: "source".into(),
                        content: true,
                    },
                )
                .await
                .unwrap();
            assert_eq!(reverse.changes[0].sources[0].path, "target");
            assert_eq!(reverse.changes[0].sources[0].kind, d::SourceKind::Rename);
            assert_eq!(
                text(&reverse.changes[0])
                    .iter()
                    .map(|h| h.before.as_str())
                    .collect::<String>(),
                "changed\n"
            );
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn content_diff_bounds_stale_membership_corruption_and_revocation() {
    use jj_store_service::administration::{TokenChange, TokenRequest};
    let mut h = History::new().await;
    let a = h.file(b"a\nb\nc\n").await;
    let b = h.file(b"b\nc\nd\n").await;
    let tree = h.tree(vec![("file", a)]).await;
    let first = h.publish(vec![tree], vec![""]).await;
    let from = h.commit;
    let tree = h.tree(vec![("file", b.clone()), ("new", b)]).await;
    let read = h.publish(vec![tree], vec![""]).await;
    let request = d::Request {
        from,
        to: h.commit,
        path: String::new(),
        content: true,
    };
    assert_eq!(
        first.diff(&reader(), request.clone()).await.unwrap_err(),
        q::Error::NotCommitted
    );
    for limits in [
        c::Limits {
            max_changes: 1,
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
    ] {
        assert_eq!(
            read.diff(&ContentReader::new(limits, 1).unwrap(), request.clone())
                .await
                .unwrap_err(),
            q::Error::Limit
        );
    }
    for (limits, reason) in [
        (
            c::Limits {
                max_text_bytes: 1,
                ..c::Limits::default()
            },
            d::Unavailable::TooLarge,
        ),
        (
            c::Limits {
                max_diff_cells: 1,
                ..c::Limits::default()
            },
            d::Unavailable::Complexity,
        ),
    ] {
        let response = read
            .diff(&ContentReader::new(limits, 1).unwrap(), request.clone())
            .await
            .unwrap();
        unavailable(&response.changes[0], reason);
    }
    let session = h.f.session(h.f.reader.clone(), Protection::default());
    let OpenRead::Pinned(readonly) = session
        .open_read(None, ReadLimits::default())
        .await
        .unwrap()
    else {
        unreachable!()
    };
    let readonly = Arc::new(readonly);
    h.f.session
        .change_token(&TokenRequest {
            request_id: m::RequestId::new([213; 16]),
            expected: h.f.session.snapshot().await.unwrap().manifest().snapshot(),
            change: TokenChange::Revoke {
                token: h.f.reader.id(),
            },
        })
        .await
        .unwrap();
    assert_eq!(
        readonly.diff(&reader(), request.clone()).await.unwrap_err(),
        q::Error::Denied
    );
    let key = Key::new(m::ObjectKey::Commit(h.commit).storage_path(REPO)).unwrap();
    let store = Arc::new(MissingNative {
        inner: h.f.store.clone(),
        missing: key.clone(),
    });
    assert_eq!(
        reader()
            .diff(store, read.clone(), request.clone())
            .await
            .unwrap_err(),
        q::Error::Incomplete
    );
    let old = h.f.store.get(&key, None).await.unwrap();
    let version = old.metadata.version.clone();
    let mut bytes = old.read_bounded(1_000_000).await.unwrap();
    *bytes.last_mut().unwrap() ^= 1;
    let upload = StagedUpload::from_bytes(&bytes, 1_000_000).await.unwrap();
    h.f.store
        .put(&key, Condition::Matches(version), &upload)
        .await
        .unwrap();
    assert_eq!(
        read.diff(&reader(), request).await.unwrap_err(),
        q::Error::Corrupt
    );
}

#[path = "conflicts.rs"]
mod conflict_acceptance;

async fn recorded_file(
    h: &History,
    name: &str,
    parents: Vec<b::CopyId>,
    bytes: &[u8],
) -> (b::CopyId, b::TreeValue) {
    let history = b::CopyHistory {
        current_path: RepoPathBuf::from_internal_string(name).unwrap(),
        parents,
        salt: bytes.to_vec(),
    };
    let key = h.f.objects.put(&history).await.unwrap().0;
    let copy = b::CopyId::from_bytes(key.as_bytes());
    let mut value = h.file(bytes).await;
    let b::TreeValue::File { copy_id, .. } = &mut value else {
        unreachable!()
    };
    *copy_id = copy.clone();
    (copy, value)
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn content_diff_multiple_copy_lineages_are_explicit_without_guessing_one_source() {
    let mut h = History::new().await;
    let (a, fa) = recorded_file(&h, "one", vec![b::CopyId::placeholder()], b"one").await;
    let (b, fb) = recorded_file(&h, "two", vec![b::CopyId::placeholder()], b"two").await;
    let root = h.tree(vec![("one", fa), ("two", fb)]).await;
    h.publish(vec![root], vec![""]).await;
    let from = h.commit;
    let (_, combined) = recorded_file(&h, "combined", vec![a, b], b"one and two").await;
    let root = h.tree(vec![("combined", combined)]).await;
    let read = h.publish(vec![root], vec![""]).await;
    let response = read
        .diff(
            &reader(),
            d::Request {
                from,
                to: h.commit,
                path: "combined".into(),
                content: true,
            },
        )
        .await
        .unwrap();
    let change = &response.changes[0];
    assert_eq!(
        change
            .sources
            .iter()
            .map(|s| s.path.as_str())
            .collect::<Vec<_>>(),
        vec!["one", "two"]
    );
    assert!(
        change
            .sources
            .iter()
            .all(|s| s.kind == d::SourceKind::Rename)
    );
    unavailable(change, d::Unavailable::MultipleSources);
    assert_eq!(change.before, vec![None]);
    assert_eq!(change.after.len(), 1);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn content_diff_canceled_copy_terms_cannot_reappear_as_sources() {
    let mut h = History::new().await;
    let mut roots = vec![];
    let mut candidate = None;
    for bytes in [
        b"left".as_slice(),
        b"canceled",
        b"canceled",
        b"base",
        b"right",
    ] {
        let (id, value) = recorded_file(&h, "source", vec![b::CopyId::placeholder()], bytes).await;
        if bytes == b"canceled" {
            candidate = Some(id);
        }
        roots.push(h.tree(vec![("source", value)]).await);
    }
    h.publish(roots, vec!["left", "cancel", "cancel", "base", "right"])
        .await;
    let from = h.commit;
    let (_, target) = recorded_file(&h, "target", vec![candidate.unwrap()], b"target").await;
    let root = h.tree(vec![("target", target)]).await;
    let read = h.publish(vec![root], vec![""]).await;
    let response = read
        .diff(
            &reader(),
            d::Request {
                from,
                to: h.commit,
                path: "target".into(),
                content: true,
            },
        )
        .await
        .unwrap();
    assert!(response.changes[0].sources.is_empty());
}
