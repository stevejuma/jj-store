use super::*;
use jj_lib::{merge::Merge, repo_path::RepoPathComponentBuf};
use jj_store_jj::{
    codec::{NativeCodec as _, NativeSymlink},
    native_dto as dto,
    query::{
        self as q,
        content::{self as c, ContentReader, ObjectData},
    },
};
use jj_store_object::{ByteRange, Condition, Key, ObjectStore as _, StagedUpload, WriteOutcome};
use jj_store_service::reads::{CommittedRead, OpenRead, ReadLimits};
use std::{collections::BTreeMap, time::Duration};

struct ContentFixture {
    f: Fixture,
    read: Arc<CommittedRead>,
    commit: b::Commit,
    tree: b::Tree,
    copy: b::CopyHistory,
    view: o::View,
    operation: o::Operation,
    binary: Vec<u8>,
    file: m::FileId,
    empty: m::FileId,
    symlink: m::ObjectKey,
}
impl ContentFixture {
    async fn new() -> Self {
        let f = Fixture::new().await;
        let binary: Vec<u8> = (0..200_000).map(|n| n as u8).collect();
        let file = f.objects.put_file(binary.as_slice()).await.unwrap().0;
        let empty = f.objects.put_file(b"".as_slice()).await.unwrap().0;
        let symlink = f
            .objects
            .put(&NativeSymlink("../../outside/<script>".into()))
            .await
            .unwrap()
            .0;
        let copy = b::CopyHistory {
            current_path: RepoPathBuf::from_internal_string("bin").unwrap(),
            parents: vec![b::CopyId::placeholder()],
            salt: vec![0, 255, 1],
        };
        let copy_key = f.objects.put(&copy).await.unwrap().0;
        let file_value = b::TreeValue::File {
            id: b::FileId::from_bytes(file.as_bytes()),
            executable: false,
            copy_id: b::CopyId::from_bytes(copy_key.as_bytes()),
        };
        let subtree = tree(vec![("nested.bin", file_value.clone())]);
        let subtree_key = f.objects.put(&subtree).await.unwrap().0;
        let tree = tree(vec![
            ("bin", file_value),
            (
                "dir",
                b::TreeValue::Tree(b::TreeId::from_bytes(subtree_key.as_bytes())),
            ),
            (
                "empty",
                b::TreeValue::File {
                    id: b::FileId::from_bytes(empty.as_bytes()),
                    executable: false,
                    copy_id: b::CopyId::placeholder(),
                },
            ),
            (
                "exec",
                b::TreeValue::File {
                    id: b::FileId::from_bytes(file.as_bytes()),
                    executable: true,
                    copy_id: b::CopyId::placeholder(),
                },
            ),
            (
                "link",
                b::TreeValue::Symlink(b::SymlinkId::from_bytes(symlink.as_bytes())),
            ),
            (
                "module",
                b::TreeValue::GitSubmodule(b::CommitId::from_bytes(&[61; 20])),
            ),
        ]);
        let tree_key = f.objects.put(&tree).await.unwrap().0;
        let mut commit = b::make_root_commit(
            b::ChangeId::from_bytes(&[60; 16]),
            b::TreeId::from_bytes(tree_key.as_bytes()),
        );
        commit.parents = vec![b::CommitId::from_bytes(roots::ROOT_COMMIT.as_bytes())];
        let mut predecessor = commit.clone();
        predecessor.description = "invisible predecessor".into();
        let predecessor_key = f.objects.put(&predecessor).await.unwrap().0;
        commit.predecessors = vec![b::CommitId::from_bytes(predecessor_key.as_bytes())];
        commit.description = "binary <script> — 日本語\0\n".into();
        commit.author = b::Signature {
            name: "Native author".into(),
            email: "author@example.invalid".into(),
            timestamp: b::Timestamp {
                timestamp: b::MillisSinceEpoch(-123),
                tz_offset: 345,
            },
        };
        commit.committer = commit.author.clone();
        commit.secure_sig = Some(b::SecureSig {
            data: vec![0, 1, 255],
            sig: vec![255, 2, 0],
        });
        let commit_key = f.objects.put(&commit).await.unwrap().0;
        let native_id = b::CommitId::from_bytes(commit_key.as_bytes());
        let target = o::RefTarget::from_merge(Merge::from_vec(vec![
            Some(native_id.clone()),
            None,
            Some(b::CommitId::from_bytes(roots::ROOT_COMMIT.as_bytes())),
        ]));
        let mut view = o::View::make_root(native_id.clone());
        view.local_bookmarks
            .insert("conflicted".into(), target.clone());
        view.local_tags
            .insert("v1".into(), o::RefTarget::normal(native_id.clone()));
        view.remote_views.insert(
            "origin".into(),
            o::RemoteView {
                bookmarks: BTreeMap::from([(
                    "main".into(),
                    o::RemoteRef {
                        target: target.clone(),
                        state: o::RemoteRefState::Tracked,
                    },
                )]),
                tags: BTreeMap::from([(
                    "release".into(),
                    o::RemoteRef {
                        target: target.clone(),
                        state: o::RemoteRefState::New,
                    },
                )]),
            },
        );
        view.git_refs
            .insert("refs/heads/main".into(), target.clone());
        view.git_heads.insert("laptop".into(), target);
        view.wc_commit_ids
            .insert("laptop".into(), native_id.clone());
        view.wc_commit_ids.insert("desktop".into(), native_id);
        let view_key = f.objects.put(&view).await.unwrap().0;
        let mut operation = o::Operation::make_root(o::ViewId::from_bytes(view_key.as_bytes()));
        operation.parents = vec![o::OperationId::from_bytes(roots::ROOT_OPERATION.as_bytes())];
        operation.metadata.description = "content fixture".into();
        operation.metadata.workspace_name = Some("laptop".into());
        operation.metadata.is_snapshot = true;
        operation
            .metadata
            .attributes
            .insert("test".into(), "all fields".into());
        operation.commit_predecessors = None;
        let operation_key = f.objects.put(&operation).await.unwrap().0;
        f.session
            .publish(
                BTreeSet::from([roots::ROOT_OPERATION]),
                m::OperationId::from_slice(operation_key.as_bytes()).unwrap(),
            )
            .await
            .unwrap();
        let OpenRead::Pinned(read) = f
            .session
            .open_read(None, ReadLimits::default())
            .await
            .unwrap()
        else {
            unreachable!()
        };
        Self {
            f,
            read: Arc::new(read),
            commit,
            tree,
            copy,
            view,
            operation,
            binary,
            file,
            empty,
            symlink,
        }
    }
}
fn tree(entries: Vec<(&str, b::TreeValue)>) -> b::Tree {
    b::Tree::from_sorted_entries(
        entries
            .into_iter()
            .map(|(name, value)| (RepoPathComponentBuf::new(name).unwrap(), value))
            .collect(),
    )
}
fn reader() -> ContentReader {
    ContentReader::new(c::Limits::default(), 4).unwrap()
}
fn request(commit: m::CommitId, path: &str) -> c::TreeRequest {
    c::TreeRequest {
        commit,
        path: path.into(),
        limit: 100,
        after: None,
    }
}
fn commit_id(f: &ContentFixture) -> m::CommitId {
    m::CommitId::from_slice(f.commit.key().as_bytes()).unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn content_read_preserves_every_native_object_field_and_virtual_roots() {
    let f = ContentFixture::new().await;
    let reader = reader();
    let before = f.f.session.snapshot().await.unwrap().manifest().clone();
    macro_rules! check {
        ($native:expr,$variant:ident,$type:ty) => {{
            let value = f.read.object(&reader, $native.key()).await.unwrap();
            assert_eq!(value.key, $native.key());
            assert!(!value.virtual_object);
            let ObjectData::$variant(dto) = value.data else {
                panic!("wrong object kind")
            };
            let restored = <$type>::try_from(dto).unwrap();
            assert_eq!(restored, $native);
            assert_eq!(
                restored.encode_payload().unwrap(),
                $native.encode_payload().unwrap()
            );
        }};
    }
    check!(f.commit, Commit, b::Commit);
    check!(f.tree, Tree, b::Tree);
    check!(f.copy, Copy, b::CopyHistory);
    check!(f.view, View, o::View);
    check!(f.operation, Operation, o::Operation);
    let root = f
        .read
        .object(&reader, m::ObjectKey::Operation(roots::ROOT_OPERATION))
        .await
        .unwrap();
    assert!(root.virtual_object);
    let ObjectData::Operation(root) = root.data else {
        unreachable!()
    };
    assert!(root.commit_predecessors.unwrap().is_empty());
    for key in [
        m::ObjectKey::Commit(roots::ROOT_COMMIT),
        m::ObjectKey::View(roots::ROOT_VIEW),
        b::Tree::default().key(),
    ] {
        assert!(f.read.object(&reader, key).await.unwrap().virtual_object);
    }
    assert_eq!(
        f.read.object(&reader, f.symlink).await.unwrap().data,
        ObjectData::Symlink {
            target: "../../outside/<script>".into()
        }
    );
    assert_eq!(
        f.read
            .object(&reader, m::ObjectKey::File(f.file))
            .await
            .unwrap()
            .data,
        ObjectData::File {
            bytes: f.binary.len() as u64
        }
    );
    assert_eq!(*f.f.session.snapshot().await.unwrap().manifest(), before);
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn content_read_lists_native_directory_entries_with_paths_and_pagination() {
    let f = ContentFixture::new().await;
    let reader = reader();
    let id = commit_id(&f);
    let before = f.f.session.snapshot().await.unwrap().manifest().clone();
    let full = f.read.tree(&reader, request(id, "")).await.unwrap();
    assert_eq!(
        full.entries
            .iter()
            .map(|e| e.name.as_str())
            .collect::<Vec<_>>(),
        vec!["bin", "dir", "empty", "exec", "link", "module"]
    );
    assert!(matches!(
        full.entries[3].terms[0],
        Some(dto::TreeValue::File {
            executable: true,
            copy: m::CopyReference::Placeholder,
            ..
        })
    ));
    assert_eq!(
        full.entries[5].terms[0],
        Some(dto::TreeValue::Submodule { id: vec![61; 20] })
    );
    let mut input = request(id, "");
    input.limit = 2;
    let first = f.read.tree(&reader, input.clone()).await.unwrap();
    input.after = first.next;
    let second = f.read.tree(&reader, input.clone()).await.unwrap();
    input.after = second.next;
    let third = f.read.tree(&reader, input).await.unwrap();
    assert!(third.next.is_none());
    assert_eq!(
        first
            .entries
            .into_iter()
            .chain(second.entries)
            .chain(third.entries)
            .collect::<Vec<_>>(),
        full.entries
    );
    assert_eq!(
        f.read
            .tree(&reader, request(id, "dir"))
            .await
            .unwrap()
            .entries[0]
            .name,
        "nested.bin"
    );
    for path in ["bin", "link", "module"] {
        assert_eq!(
            f.read.tree(&reader, request(id, path)).await.unwrap_err(),
            q::Error::NotDirectory
        );
    }
    assert_eq!(
        f.read
            .tree(&reader, request(id, "missing"))
            .await
            .unwrap_err(),
        q::Error::NotFound
    );
    for path in ["../bin", "/bin", "dir/../bin", "dir//nested.bin"] {
        assert_eq!(
            f.read.tree(&reader, request(id, path)).await.unwrap_err(),
            q::Error::Invalid
        );
    }
    let mut missing = request(id, "");
    missing.after = Some("missing".into());
    assert_eq!(
        f.read.tree(&reader, missing).await.unwrap_err(),
        q::Error::Invalid
    );
    assert_eq!(*f.f.session.snapshot().await.unwrap().manifest(), before);
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn content_read_verifies_binary_ranges_before_streaming_and_retains_permits() {
    let f = ContentFixture::new().await;
    let reader = ContentReader::new(c::Limits::default(), 1).unwrap();
    let mut file = f.read.file(&reader, f.file, None).await.unwrap();
    let info = file.info().await.unwrap();
    assert_eq!(info.full_length, f.binary.len() as u64);
    assert_eq!((info.start, info.end), (0, f.binary.len() as u64));
    assert_eq!(
        f.read.object(&reader, f.tree.key()).await.unwrap_err(),
        q::Error::Busy
    );
    let mut received = vec![];
    while let Some(chunk) = file.next_chunk().await.unwrap() {
        assert!(chunk.len() <= 65536);
        received.extend(chunk);
    }
    assert_eq!(received, f.binary);
    drop(file);
    let mut range = f
        .read
        .file(
            &reader,
            f.file,
            Some(ByteRange::new(65_530, 130_010).unwrap()),
        )
        .await
        .unwrap();
    let mut received = vec![];
    while let Some(chunk) = range.next_chunk().await.unwrap() {
        received.extend(chunk);
    }
    assert_eq!(received, f.binary[65_530..130_010]);
    drop(range);
    let mut empty = f.read.file(&reader, f.empty, None).await.unwrap();
    assert_eq!(empty.info().await.unwrap().full_length, 0);
    assert!(empty.next_chunk().await.unwrap().is_none());
    drop(empty);
    assert_eq!(
        f.read
            .file(&reader, f.empty, Some(ByteRange::new(0, 1).unwrap()))
            .await
            .unwrap_err(),
        q::Error::Range
    );
    assert_eq!(
        f.read
            .file(&reader, f.file, Some(ByteRange::new(1, 500_000).unwrap()))
            .await
            .unwrap_err(),
        q::Error::Range
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn content_read_rechecks_revocation_and_limits_and_rejects_staged_objects() {
    use jj_store_service::administration::{TokenChange, TokenRequest};
    let f = ContentFixture::new().await;
    let reader = reader();
    let staged =
        f.f.objects
            .put_file(b"only staged".as_slice())
            .await
            .unwrap()
            .0;
    assert_eq!(
        f.read.file(&reader, staged, None).await.unwrap_err(),
        q::Error::NotCommitted
    );
    assert_eq!(
        f.read
            .object(&reader, m::ObjectKey::File(staged))
            .await
            .unwrap_err(),
        q::Error::NotCommitted
    );
    for limits in [
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
        assert_eq!(
            f.read
                .object(&ContentReader::new(limits, 1).unwrap(), f.tree.key())
                .await
                .unwrap_err(),
            q::Error::Limit
        );
    }
    assert_eq!(
        f.read
            .file(
                &ContentReader::new(
                    c::Limits {
                        max_blob_bytes: 1,
                        ..c::Limits::default()
                    },
                    1
                )
                .unwrap(),
                f.file,
                None
            )
            .await
            .unwrap_err(),
        q::Error::Limit
    );
    let limited = ContentReader::new(
        c::Limits {
            max_page: 1,
            ..c::Limits::default()
        },
        1,
    )
    .unwrap();
    assert_eq!(
        f.read
            .tree(&limited, request(commit_id(&f), ""))
            .await
            .unwrap_err(),
        q::Error::Limit
    );
    let session = f.f.session(f.f.reader.clone(), Protection::default());
    let OpenRead::Pinned(read) = session
        .open_read(None, ReadLimits::default())
        .await
        .unwrap()
    else {
        unreachable!()
    };
    let read = Arc::new(read);
    let mut stream = read.file(&reader, f.file, None).await.unwrap();
    assert_eq!(
        stream.next_chunk().await.unwrap().unwrap(),
        f.binary[..65536]
    );
    f.f.session
        .change_token(&TokenRequest {
            request_id: m::RequestId::new([240; 16]),
            expected: f.f.session.snapshot().await.unwrap().manifest().snapshot(),
            change: TokenChange::Revoke {
                token: f.f.reader.id(),
            },
        })
        .await
        .unwrap();
    let after = f.f.session.snapshot().await.unwrap().manifest().clone();
    assert_eq!(stream.next_chunk().await.unwrap_err(), q::Error::Denied);
    assert_eq!(stream.info().await.unwrap_err(), q::Error::Denied);
    assert_eq!(
        read.object(&reader, m::ObjectKey::Commit(roots::ROOT_COMMIT))
            .await
            .unwrap_err(),
        q::Error::Denied
    );
    assert_eq!(
        read.tree(&reader, request(commit_id(&f), ""))
            .await
            .unwrap_err(),
        q::Error::Denied
    );
    assert_eq!(*f.f.session.snapshot().await.unwrap().manifest(), after);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn content_read_rejects_corruption_outside_requested_range_and_missing_committed_objects() {
    let f = ContentFixture::new().await;
    let reader = reader();
    let key = Key::new(m::ObjectKey::File(f.file).storage_path(REPO)).unwrap();
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
        f.read
            .file(&reader, f.file, Some(ByteRange::new(0, 10).unwrap()))
            .await
            .unwrap_err(),
        q::Error::Corrupt
    );
    let tree_key = Key::new(f.tree.key().storage_path(REPO)).unwrap();
    let version = f.f.store.head(&tree_key).await.unwrap().version;
    let mut bytes =
        f.f.store
            .get(&tree_key, None)
            .await
            .unwrap()
            .read_bounded(1_000_000)
            .await
            .unwrap();
    *bytes.last_mut().unwrap() ^= 1;
    let upload = StagedUpload::from_bytes(&bytes, 1_000_000).await.unwrap();
    assert!(matches!(
        f.f.store
            .put(&tree_key, Condition::Matches(version), &upload)
            .await
            .unwrap(),
        WriteOutcome::Applied(_)
    ));
    assert_eq!(
        f.read.object(&reader, f.tree.key()).await.unwrap_err(),
        q::Error::Corrupt
    );
    assert_eq!(
        f.read
            .tree(&reader, request(commit_id(&f), ""))
            .await
            .unwrap_err(),
        q::Error::Corrupt
    );
    let store = Arc::new(MissingNative {
        inner: f.f.store.clone(),
        missing: key,
    });
    let native = Arc::new(NativeObjects::new(store.clone(), REPO, 1_000_000).unwrap());
    let session = Arc::new(
        NativeSession::new(
            store,
            native,
            f.f.token.clone(),
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
        )
        .unwrap(),
    );
    let OpenRead::Pinned(read) = session
        .open_read(None, ReadLimits::default())
        .await
        .unwrap()
    else {
        unreachable!()
    };
    assert_eq!(
        Arc::new(read)
            .file(&reader, f.file, None)
            .await
            .unwrap_err(),
        q::Error::Incomplete
    );
}
struct MissingNative {
    inner: Arc<MemoryStore>,
    missing: Key,
}
#[async_trait::async_trait]
impl jj_store_object::ObjectStore for MissingNative {
    async fn head(&self, key: &Key) -> jj_store_object::Result<jj_store_object::Metadata> {
        self.inner.head(key).await
    }
    async fn get(
        &self,
        key: &Key,
        range: Option<ByteRange>,
    ) -> jj_store_object::Result<jj_store_object::Download> {
        if key == &self.missing {
            return Err(jj_store_object::Error::NotFound);
        }
        self.inner.get(key, range).await
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> jj_store_object::Result<WriteOutcome> {
        self.inner.put(key, condition, upload).await
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn content_read_stream_deadline_and_merged_tree_conflicts_preserve_native_terms() {
    let f = ContentFixture::new().await;
    let timed = ContentReader::new(
        c::Limits {
            timeout: Duration::from_millis(300),
            ..c::Limits::default()
        },
        1,
    )
    .unwrap();
    let mut stream = f.read.file(&timed, f.file, None).await.unwrap();
    tokio::time::sleep(Duration::from_millis(310)).await;
    assert_eq!(stream.next_chunk().await.unwrap_err(), q::Error::Limit);
    drop(stream);
    let mut file_values = vec![];
    for text in ["left", "base", "right", "third"] {
        let id = f.f.objects.put_file(text.as_bytes()).await.unwrap().0;
        file_values.push(b::TreeValue::File {
            id: b::FileId::from_bytes(id.as_bytes()),
            executable: false,
            copy_id: b::CopyId::placeholder(),
        });
    }
    let sub = f
        .tree
        .entries()
        .find(|e| e.name().as_internal_str() == "dir")
        .unwrap()
        .value()
        .clone();
    let symlink = b::TreeValue::Symlink(b::SymlinkId::from_bytes(f.symlink.as_bytes()));
    let left = tree(vec![
        ("conflict", file_values[0].clone()),
        ("file-tree", sub.clone()),
        ("trivial", file_values[0].clone()),
    ]);
    let base = tree(vec![
        ("conflict", file_values[1].clone()),
        ("deleted", file_values[1].clone()),
        ("file-tree", file_values[1].clone()),
        ("trivial", file_values[1].clone()),
    ]);
    let right = tree(vec![
        ("conflict", file_values[2].clone()),
        ("deleted", file_values[1].clone()),
        ("file-tree", symlink.clone()),
        ("trivial", file_values[1].clone()),
    ]);
    let third = tree(vec![
        ("conflict", file_values[3].clone()),
        ("deleted", file_values[1].clone()),
        ("file-tree", symlink),
        ("trivial", file_values[1].clone()),
    ]);
    let mut ids = vec![];
    for value in [&left, &base, &right, &base, &third] {
        ids.push(b::TreeId::from_bytes(
            f.f.objects.put(value).await.unwrap().0.as_bytes(),
        ));
    }
    let mut commit = f.commit.clone();
    commit.change_id = b::ChangeId::from_bytes(&[62; 16]);
    commit.parents = vec![b::CommitId::from_bytes(f.commit.key().as_bytes())];
    commit.predecessors = commit.parents.clone();
    commit.root_tree = Merge::from_vec(ids);
    commit.conflict_labels = Merge::from_vec(vec![
        "left".into(),
        "base".into(),
        "right".into(),
        "base".into(),
        "third".into(),
    ]);
    let key = f.f.objects.put(&commit).await.unwrap().0;
    let id = m::CommitId::from_slice(key.as_bytes()).unwrap();
    let mut view = f.view.clone();
    view.head_ids = [b::CommitId::from_bytes(key.as_bytes())].into();
    let view_key = f.f.objects.put(&view).await.unwrap().0;
    let mut op = f.operation.clone();
    op.parents = vec![o::OperationId::from_bytes(f.operation.key().as_bytes())];
    op.view_id = o::ViewId::from_bytes(view_key.as_bytes());
    op.commit_predecessors = Some(BTreeMap::from([(
        b::CommitId::from_bytes(key.as_bytes()),
        vec![b::CommitId::from_bytes(f.commit.key().as_bytes())],
    )]));
    let op_key = f.f.objects.put(&op).await.unwrap().0;
    let reader = reader();
    assert_eq!(
        f.read.object(&reader, key).await.unwrap_err(),
        q::Error::NotCommitted
    );
    f.f.session
        .publish(
            BTreeSet::from([m::OperationId::from_slice(f.operation.key().as_bytes()).unwrap()]),
            m::OperationId::from_slice(op_key.as_bytes()).unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        f.read.object(&reader, key).await.unwrap_err(),
        q::Error::NotCommitted
    );
    let OpenRead::Pinned(read) =
        f.f.session
            .open_read(None, ReadLimits::default())
            .await
            .unwrap()
    else {
        unreachable!()
    };
    let read = Arc::new(read);
    let before = f.f.session.snapshot().await.unwrap().manifest().clone();
    let result = read.tree(&reader, request(id, "")).await.unwrap();
    assert_eq!(
        result
            .entries
            .iter()
            .map(|e| e.name.as_str())
            .collect::<Vec<_>>(),
        vec!["conflict", "file-tree", "trivial"]
    );
    assert_eq!(
        result.entries[0].terms,
        file_values
            .iter()
            .take(3)
            .chain([&file_values[1], &file_values[3]])
            .map(|v| Some(dto::TreeValue::try_from(v).unwrap()))
            .collect::<Vec<_>>()
    );
    assert!(result.entries[0].conflicted);
    assert!(result.entries[1].conflicted);
    assert_eq!(
        read.tree(&reader, request(id, "file-tree"))
            .await
            .unwrap_err(),
        q::Error::NotDirectory
    );
    assert_eq!(
        result.entries[2].terms,
        vec![Some(dto::TreeValue::try_from(&file_values[0]).unwrap())]
    );
    assert!(!result.entries[2].conflicted);
    assert_eq!(*f.f.session.snapshot().await.unwrap().manifest(), before);
}

struct PauseContent {
    inner: Arc<CommittedRead>,
    remaining: std::sync::atomic::AtomicUsize,
    entered: tokio::sync::Notify,
    release: tokio::sync::Notify,
}
#[async_trait::async_trait]
impl q::ReadAccess for PauseContent {
    fn repository(&self) -> m::RepositoryId {
        q::ReadAccess::repository(self.inner.as_ref())
    }
    fn epoch(&self) -> m::EpochId {
        q::ReadAccess::epoch(self.inner.as_ref())
    }
    fn operation(&self) -> m::OperationId {
        q::ReadAccess::operation(self.inner.as_ref())
    }
    async fn contains(&self, key: m::ObjectKey) -> q::Result<bool> {
        q::ReadAccess::contains(self.inner.as_ref(), key).await
    }
    async fn inventory(&self) -> q::Result<Arc<BTreeSet<m::ObjectKey>>> {
        q::ReadAccess::inventory(self.inner.as_ref()).await
    }
    async fn authorize(&self) -> q::Result<()> {
        use std::sync::atomic::Ordering;
        if self
            .remaining
            .try_update(Ordering::SeqCst, Ordering::SeqCst, |n| n.checked_sub(1))
            == Ok(1)
        {
            self.entered.notify_one();
            self.release.notified().await;
        }
        q::ReadAccess::authorize(self.inner.as_ref()).await
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn content_read_canceled_chunk_cannot_resume_with_skipped_bytes() {
    use std::sync::atomic::Ordering;
    let f = ContentFixture::new().await;
    let access = Arc::new(PauseContent {
        inner: f.read.clone(),
        remaining: Default::default(),
        entered: Default::default(),
        release: Default::default(),
    });
    let reader = ContentReader::new(c::Limits::default(), 1).unwrap();
    let mut stream = reader
        .file(f.f.store.clone(), access.clone(), f.file, None)
        .await
        .unwrap();
    // Pause after bytes have been consumed but before they are authorized for
    // return, then cancel the future while retaining the stream handle.
    access.remaining.store(2, Ordering::SeqCst);
    let mut next = Box::pin(stream.next_chunk());
    tokio::select! {
        _=access.entered.notified()=>{},
        result=&mut next=>panic!("did not pause: {result:?}"),
        _=tokio::time::sleep(Duration::from_secs(3))=>panic!("pause timeout"),
    }
    drop(next);
    assert_eq!(
        stream.next_chunk().await.unwrap_err(),
        q::Error::Unavailable
    );
    assert_eq!(
        f.read.object(&reader, f.tree.key()).await.unwrap_err(),
        q::Error::Busy
    );
    drop(stream);
    assert!(f.read.object(&reader, f.tree.key()).await.is_ok());
}

#[path = "directory.rs"]
mod directory_acceptance;

#[path = "operation_diff.rs"]
mod operation_diff_acceptance;

#[path = "diff.rs"]
mod diff_acceptance;

#[path = "content_admission.rs"]
mod admission_acceptance;

#[path = "code_summary.rs"]
mod code_summary_acceptance;

#[path = "query_predicates.rs"]
mod predicate_acceptance;

#[path = "query_workers.rs"]
mod worker_acceptance;

#[path = "index_tasks.rs"]
mod index_task_acceptance;

#[path = "kind_tasks.rs"]
mod kind_task_acceptance;

#[path = "http_reads.rs"]
mod http_acceptance;

#[path = "adapter_projection.rs"]
mod adapter_projection;
