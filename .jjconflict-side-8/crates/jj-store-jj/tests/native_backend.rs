use futures::{AsyncReadExt as _, StreamExt as _, TryStreamExt as _};
use jj_lib::{
    backend::*,
    merge::Merge,
    object_id::ObjectId as _,
    repo_path::{RepoPath, RepoPathBuf, RepoPathComponentBuf},
};
use jj_store_jj::{
    backend::*,
    codec::{NativeCodec as _, NativeFile},
    objects::NativeObjects,
    operation_stores::AuthorityResult,
    roots,
};
use jj_store_model as m;
use jj_store_object::memory::{Fault, MemoryStore};
#[path = "native_backend/families.rs"]
mod families;
use std::{
    collections::BTreeSet,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

struct Authority {
    allowed: AtomicBool,
    slow: AtomicBool,
    committed: Mutex<BTreeSet<m::CopyId>>,
}
#[async_trait::async_trait]
impl BackendAuthority for Authority {
    async fn authorize_read(&self) -> AuthorityResult<()> {
        if self.slow.load(Ordering::SeqCst) {
            std::future::pending::<()>().await;
        }
        if self.allowed.load(Ordering::SeqCst) {
            Ok(())
        } else {
            Err("revoked".into())
        }
    }
    async fn authorize_write(&self) -> AuthorityResult<()> {
        self.authorize_read().await
    }
    async fn copy_ids(&self, _: usize) -> AuthorityResult<BTreeSet<m::CopyId>> {
        Ok(self.committed.lock().unwrap().clone())
    }
}
struct Fixture {
    store: Arc<MemoryStore>,
    objects: Arc<NativeObjects>,
    authority: Arc<Authority>,
    backend: NativeBackend,
}
impl Fixture {
    fn new() -> Self {
        let store = Arc::new(MemoryStore::default());
        let objects = Arc::new(
            NativeObjects::new(
                store.clone(),
                m::RepositoryId::new([1; 16]),
                4 * 1024 * 1024,
            )
            .unwrap(),
        );
        let authority = Arc::new(Authority {
            allowed: AtomicBool::new(true),
            slow: AtomicBool::new(false),
            committed: Mutex::new(BTreeSet::new()),
        });
        let backend =
            NativeBackend::new(objects.clone(), authority.clone(), BackendLimits::default())
                .unwrap();
        Self {
            store,
            objects,
            authority,
            backend,
        }
    }
    fn reload(&self, limits: BackendLimits) -> NativeBackend {
        NativeBackend::new(self.objects.clone(), self.authority.clone(), limits).unwrap()
    }
}
fn path(value: &str) -> RepoPathBuf {
    RepoPathBuf::from_internal_string(value).unwrap()
}
async fn copy(backend: &NativeBackend, name: &str, parents: Vec<CopyId>) -> CopyId {
    backend
        .write_copy(&CopyHistory {
            current_path: path(name),
            parents,
            salt: vec![],
        })
        .await
        .unwrap()
}
async fn file(backend: &NativeBackend, data: &[u8]) -> FileId {
    backend
        .write_file(RepoPath::root(), &mut futures::io::Cursor::new(data))
        .await
        .unwrap()
}
async fn tree(backend: &NativeBackend, entries: Vec<(&str, TreeValue)>) -> TreeId {
    let mut entries: Vec<_> = entries
        .into_iter()
        .map(|(name, value)| (RepoPathComponentBuf::new(name.to_owned()).unwrap(), value))
        .collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    backend
        .write_tree(RepoPath::root(), &Tree::from_sorted_entries(entries))
        .await
        .unwrap()
}
fn value(file: &FileId, copy: &CopyId) -> TreeValue {
    TreeValue::File {
        id: file.clone(),
        executable: false,
        copy_id: copy.clone(),
    }
}
async fn commit(
    backend: &NativeBackend,
    parents: Vec<CommitId>,
    tree: TreeId,
    label: &str,
) -> CommitId {
    let mut c = roots::commit();
    c.parents = parents;
    c.change_id = ChangeId::from_bytes(&[9; 16]);
    c.description = label.to_owned();
    c.root_tree = Merge::resolved(tree);
    backend.write_commit(c, None).await.unwrap().0
}
async fn records(backend: &NativeBackend, root: &CommitId, head: &CommitId) -> Vec<CopyRecord> {
    backend
        .get_copy_records(None, root, head)
        .unwrap()
        .try_collect()
        .await
        .unwrap()
}

#[tokio::test]
async fn native_backend_roundtrips_roots_full_trees_commits_and_signatures() {
    let f = Fixture::new();
    let b = &f.backend;
    assert_eq!(b.commit_id_length(), 64);
    assert_eq!(b.change_id_length(), 16);
    assert_eq!(
        b.read_commit(b.root_commit_id()).await.unwrap(),
        roots::commit()
    );
    assert_eq!(
        b.read_tree(RepoPath::root(), b.empty_tree_id())
            .await
            .unwrap(),
        Tree::default()
    );
    let content = file(b, b"hello\0world").await;
    let history = copy(b, "nested/file", vec![]).await;
    let symlink = b.write_symlink(&path("link"), "../outside").await.unwrap();
    assert_eq!(
        b.read_symlink(&path("any"), &symlink).await.unwrap(),
        "../outside"
    );
    let nested = tree(
        b,
        vec![(
            "file",
            TreeValue::File {
                id: content,
                executable: true,
                copy_id: history.clone(),
            },
        )],
    )
    .await;
    let top = tree(
        b,
        vec![
            ("nested", TreeValue::Tree(nested)),
            ("link", TreeValue::Symlink(symlink)),
            (
                "module",
                TreeValue::GitSubmodule(CommitId::from_bytes(&[4; 20])),
            ),
        ],
    )
    .await;
    let original = b.read_tree(RepoPath::root(), &top).await.unwrap();
    assert!(matches!(
        original
            .entries()
            .find(|e| e.name().as_internal_str() == "module")
            .unwrap()
            .value(),
        TreeValue::GitSubmodule(_)
    ));
    let parent = commit(b, vec![b.root_commit_id().clone()], top.clone(), "parent").await;
    let mut c = roots::commit();
    c.parents = vec![parent.clone(), b.root_commit_id().clone()];
    c.predecessors = vec![parent];
    c.change_id = ChangeId::from_bytes(&[7; 16]);
    c.description = "merge".into();
    c.root_tree = Merge::from_vec(vec![top.clone(), b.empty_tree_id().clone(), top]);
    c.conflict_labels = Merge::from_vec(vec!["left".into(), "base".into(), "right".into()]);
    let unsigned = c.encode_payload().unwrap();
    let mut signer = |bytes: &[u8]| {
        assert_eq!(
            &bytes[b"jj-store/native-commit-signature/v1\0".len()..],
            unsigned
        );
        Ok(vec![42, 99])
    };
    let (id, signed) = b.write_commit(c, Some(&mut signer)).await.unwrap();
    assert_eq!(id.as_bytes(), signed.key().as_bytes());
    assert_eq!(signed.secure_sig.as_ref().unwrap().sig, vec![42, 99]);
    let cold = f.reload(BackendLimits::default());
    assert_eq!(cold.read_commit(&id).await.unwrap(), signed);
    assert_eq!(cold.write_commit(signed.clone(), None).await.unwrap().0, id);
    assert!(cold.write_commit(signed, Some(&mut signer)).await.is_err());
    assert_eq!(
        cold.read_copy(&history).await.unwrap().current_path,
        path("nested/file")
    );
    assert!(cold.write_commit(roots::commit(), None).await.is_err());
}

#[tokio::test]
async fn file_stream_survives_backend_drop_and_rejects_unknown_corrupt_or_revoked_reads() {
    let f = Fixture::new();
    let bytes: Vec<_> = (0..2 * 1024 * 1024).map(|n| n as u8).collect();
    let id = file(&f.backend, &bytes).await;
    assert_eq!(id.as_bytes(), NativeFile(bytes.clone()).key().as_bytes());
    let cold = f.reload(BackendLimits::default());
    let mut reader = cold.read_file(&path("renamed"), &id).await.unwrap();
    drop(cold);
    let mut actual = Vec::new();
    reader.read_to_end(&mut actual).await.unwrap();
    assert_eq!(actual, bytes);
    f.store.inject(Fault::BeforeWrite);
    assert!(
        f.backend
            .write_file(
                RepoPath::root(),
                &mut futures::io::Cursor::new(b"unacknowledged")
            )
            .await
            .is_err()
    );
    f.store.inject(Fault::AfterWrite);
    let ack = file(&f.backend, b"lost response").await;
    assert!(f.backend.read_file(RepoPath::root(), &ack).await.is_ok());
    f.store.inject(Fault::ReadCorrupt);
    assert!(f.backend.read_file(RepoPath::root(), &id).await.is_err());
    f.authority.allowed.store(false, Ordering::SeqCst);
    assert!(f.backend.read_file(RepoPath::root(), &id).await.is_err());
    assert!(
        f.backend
            .read_commit(f.backend.root_commit_id())
            .await
            .is_err()
    );
    assert!(
        f.backend
            .read_tree(RepoPath::root(), f.backend.empty_tree_id())
            .await
            .is_err()
    );
    assert!(
        f.backend
            .write_symlink(RepoPath::root(), "denied")
            .await
            .is_err()
    );
}

#[tokio::test]
async fn copy_inventory_is_session_local_complete_deterministic_and_reverse_topological() {
    let f = Fixture::new();
    let b = &f.backend;
    let a = copy(b, "a", vec![]).await;
    let c = copy(b, "c", vec![a.clone()]).await;
    let d = copy(b, "d", vec![a.clone(), c.clone()]).await;
    let sibling = copy(b, "sibling", vec![a.clone()]).await;
    let first = b.get_related_copies(&c).await.unwrap();
    assert_eq!(first, b.get_related_copies(&c).await.unwrap());
    assert_eq!(first.len(), 4);
    let position = |id: &CopyId| first.iter().position(|copy| copy.id == *id).unwrap();
    assert!(position(&d) < position(&c));
    assert!(position(&c) < position(&a));
    assert!(position(&sibling) < position(&a));
    let cold = f.reload(BackendLimits::default());
    let local = cold.get_related_copies(&c).await.unwrap();
    assert_eq!(local.len(), 2);
    *f.authority.committed.lock().unwrap() = [d.clone(), sibling]
        .into_iter()
        .map(|id| m::CopyId::from_slice(id.as_bytes()).unwrap())
        .collect();
    assert_eq!(cold.get_related_copies(&c).await.unwrap(), first);
}

#[tokio::test]
async fn copy_records_follow_renames_edits_dag_ranges_and_path_filters() {
    let f = Fixture::new();
    let b = &f.backend;
    let root = b.root_commit_id().clone();
    let data = file(b, b"source").await;
    let ca = copy(b, "a", vec![]).await;
    let a = commit(
        b,
        vec![root.clone()],
        tree(b, vec![("a", value(&data, &ca))]).await,
        "a",
    )
    .await;
    let cb = copy(b, "b", vec![ca.clone()]).await;
    let target = commit(
        b,
        vec![a.clone()],
        tree(b, vec![("b", value(&data, &cb))]).await,
        "rename",
    )
    .await;
    let edited = file(b, b"edit").await;
    let edit = commit(
        b,
        vec![target.clone()],
        tree(b, vec![("b", value(&edited, &cb))]).await,
        "edit",
    )
    .await;
    let cc = copy(b, "c", vec![cb.clone()]).await;
    let head = commit(
        b,
        vec![edit.clone()],
        tree(b, vec![("c", value(&edited, &cc))]).await,
        "rename again",
    )
    .await;
    let got = records(b, &root, &head).await;
    assert_eq!(got.len(), 2);
    assert_eq!(
        got[0],
        CopyRecord {
            target: path("c"),
            target_commit: head.clone(),
            source: path("b"),
            source_file: edited,
            source_commit: edit
        }
    );
    assert_eq!(
        got[1],
        CopyRecord {
            target: path("b"),
            target_commit: target.clone(),
            source: path("a"),
            source_file: data,
            source_commit: a.clone()
        }
    );
    assert_eq!(records(b, &target, &head).await, got[..1]);
    let filtered: Vec<_> = b
        .get_copy_records(Some(&[path("b")]), &root, &head)
        .unwrap()
        .try_collect()
        .await
        .unwrap();
    assert_eq!(filtered, got[1..]);
    assert!(records(b, &head, &head).await.is_empty());
    assert!(records(b, &a, &root).await.is_empty());
}

#[tokio::test]
async fn copy_records_cover_merges_multiple_sources_and_intra_commit_chains() {
    let f = Fixture::new();
    let b = &f.backend;
    let root = b.root_commit_id().clone();
    let fa = file(b, b"a").await;
    let fb = file(b, b"b").await;
    let ca = copy(b, "a", vec![]).await;
    let cb = copy(b, "b", vec![]).await;
    let a = commit(
        b,
        vec![root.clone()],
        tree(b, vec![("a", value(&fa, &ca))]).await,
        "left",
    )
    .await;
    let r = commit(
        b,
        vec![root.clone()],
        tree(b, vec![("b", value(&fb, &cb))]).await,
        "right",
    )
    .await;
    let mid = copy(b, "temporary", vec![ca.clone(), cb.clone()]).await;
    let merged = copy(b, "merged", vec![mid]).await;
    let head = commit(
        b,
        vec![a.clone(), r.clone()],
        tree(b, vec![("merged", value(&fa, &merged))]).await,
        "merge files",
    )
    .await;
    let got = records(b, &a, &head).await;
    assert_eq!(got.len(), 2);
    assert!(
        got.iter()
            .any(|record| record.source_commit == a && record.source_file == fa)
    );
    assert!(
        got.iter()
            .any(|record| record.source_commit == r && record.source_file == fb)
    );
    let new = copy(b, "new", vec![]).await;
    let renamed = copy(b, "brand-new", vec![new]).await;
    let brand_new = commit(
        b,
        vec![head.clone()],
        tree(b, vec![("brand-new", value(&fa, &renamed))]).await,
        "new then renamed",
    )
    .await;
    assert!(records(b, &head, &brand_new).await.is_empty());
}

#[tokio::test]
async fn conflicted_copy_targets_preserve_positive_alternatives_and_nested_paths() {
    let f = Fixture::new();
    let b = &f.backend;
    let root = b.root_commit_id().clone();
    let data = file(b, b"bytes").await;
    let ca = copy(b, "a", vec![]).await;
    let a = commit(
        b,
        vec![root.clone()],
        tree(b, vec![("a", value(&data, &ca))]).await,
        "source",
    )
    .await;
    let cb = copy(b, "dir/b", vec![ca.clone()]).await;
    let cc = copy(b, "c", vec![ca]).await;
    let inner = tree(b, vec![("b", value(&data, &cb))]).await;
    let left = tree(b, vec![("dir", TreeValue::Tree(inner))]).await;
    let right = tree(b, vec![("c", value(&data, &cc))]).await;
    let mut c = roots::commit();
    c.parents = vec![a.clone()];
    c.change_id = ChangeId::from_bytes(&[9; 16]);
    c.root_tree = Merge::from_vec(vec![left, b.empty_tree_id().clone(), right]);
    c.conflict_labels = Merge::from_vec(vec!["l".into(), "base".into(), "r".into()]);
    let head = b.write_commit(c, None).await.unwrap().0;
    let got = records(b, &a, &head).await;
    assert_eq!(got.len(), 2);
    assert_eq!(
        got.iter()
            .map(|r| r.target.clone())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([path("c"), path("dir/b")])
    );
}

#[tokio::test]
async fn copy_queries_fail_explicitly_on_limits_deadlines_missing_data_and_revocation() {
    let f = Fixture::new();
    let b = &f.backend;
    let c = copy(b, "a", vec![]).await;
    let d = copy(b, "b", vec![c.clone()]).await;
    let limited = f.reload(BackendLimits {
        max_nodes: 1,
        ..BackendLimits::default()
    });
    assert!(
        limited
            .get_related_copies(&d)
            .await
            .unwrap_err()
            .to_string()
            .contains("budget")
    );
    for limits in [
        BackendLimits {
            max_bytes: 1,
            ..BackendLimits::default()
        },
        BackendLimits {
            max_work: 1,
            ..BackendLimits::default()
        },
    ] {
        assert!(f.reload(limits).get_related_copies(&d).await.is_err());
    }
    assert!(b.get_related_copies(&CopyId::placeholder()).await.is_err());
    assert!(matches!(
        b.read_commit(&CommitId::from_bytes(&[8; 64])).await,
        Err(BackendError::ObjectNotFound { .. })
    ));
    assert!(matches!(
        b.read_tree(RepoPath::root(), &TreeId::from_bytes(&[1; 20]))
            .await,
        Err(BackendError::InvalidHashLength { .. })
    ));
    let data = file(b, b"x").await;
    let a = commit(
        b,
        vec![b.root_commit_id().clone()],
        tree(b, vec![("a", value(&data, &c))]).await,
        "a",
    )
    .await;
    let head = commit(
        b,
        vec![a.clone()],
        tree(b, vec![("b", value(&data, &d))]).await,
        "b",
    )
    .await;
    assert!(
        limited
            .get_copy_records(None, &a, &head)
            .unwrap()
            .try_collect::<Vec<_>>()
            .await
            .is_err()
    );
    let mut lazy = b.get_copy_records(None, &a, &head).unwrap();
    f.authority.allowed.store(false, Ordering::SeqCst);
    assert!(lazy.next().await.unwrap().is_err());
    f.authority.allowed.store(true, Ordering::SeqCst);
    f.authority.slow.store(true, Ordering::SeqCst);
    let timed = f.reload(BackendLimits {
        timeout: Duration::from_millis(20),
        ..BackendLimits::default()
    });
    assert!(
        timed
            .get_related_copies(&c)
            .await
            .unwrap_err()
            .to_string()
            .contains("deadline")
    );
    assert!(
        timed
            .get_copy_records(None, &a, &head)
            .unwrap()
            .try_collect::<Vec<_>>()
            .await
            .unwrap_err()
            .to_string()
            .contains("deadline")
    );
}

#[tokio::test]
async fn staging_limit_allows_identical_copy_retry_without_accepting_new_history() {
    let f = Fixture::new();
    let b = f.reload(BackendLimits {
        max_nodes: 1,
        ..BackendLimits::default()
    });
    let history = CopyHistory {
        current_path: path("a"),
        parents: vec![],
        salt: vec![],
    };
    let id = b.write_copy(&history).await.unwrap();
    assert_eq!(id, b.write_copy(&history).await.unwrap());
    let mut other = history;
    other.salt.push(1);
    assert!(b.write_copy(&other).await.is_err());
    assert!(b.get_related_copies(&id).await.is_ok());
}

#[tokio::test]
async fn signing_failure_and_unacknowledged_metadata_leave_no_successful_object() {
    let f = Fixture::new();
    let mut c = roots::commit();
    c.parents = vec![f.backend.root_commit_id().clone()];
    c.description = "signing failure".into();
    let id = CommitId::from_bytes(c.key().as_bytes());
    let mut signer = |_: &[u8]| {
        Err(jj_lib::signing::SignError::Backend(
            "fixture signer offline".into(),
        ))
    };
    assert!(f.backend.write_commit(c, Some(&mut signer)).await.is_err());
    assert!(matches!(
        f.backend.read_commit(&id).await,
        Err(BackendError::ObjectNotFound { .. })
    ));
    let history = CopyHistory {
        current_path: path("a"),
        parents: vec![],
        salt: vec![],
    };
    f.store.inject(Fault::BeforeWrite);
    assert!(f.backend.write_copy(&history).await.is_err());
    let unrelated = copy(&f.backend, "unrelated", vec![]).await;
    assert_eq!(
        f.backend
            .get_related_copies(&unrelated)
            .await
            .unwrap()
            .len(),
        1
    );
    f.store.inject(Fault::AfterWrite);
    assert!(f.backend.write_copy(&history).await.is_ok());
}

#[tokio::test]
async fn record_stream_rechecks_revocation_between_results_and_rejects_bad_copy_paths() {
    let f = Fixture::new();
    let b = &f.backend;
    let data = file(b, b"x").await;
    let ca = copy(b, "a", vec![]).await;
    let a = commit(
        b,
        vec![b.root_commit_id().clone()],
        tree(b, vec![("a", value(&data, &ca))]).await,
        "a",
    )
    .await;
    let cb = copy(b, "b", vec![ca.clone()]).await;
    let cc = copy(b, "c", vec![ca]).await;
    let head = commit(
        b,
        vec![a.clone()],
        tree(b, vec![("b", value(&data, &cb)), ("c", value(&data, &cc))]).await,
        "two copies",
    )
    .await;
    let mut stream = b.get_copy_records(None, &a, &head).unwrap();
    assert!(stream.next().await.unwrap().is_ok());
    f.authority.allowed.store(false, Ordering::SeqCst);
    assert!(stream.next().await.unwrap().is_err());
    f.authority.allowed.store(true, Ordering::SeqCst);
    let wrong = commit(
        b,
        vec![a.clone()],
        tree(b, vec![("wrong", value(&data, &cb))]).await,
        "invalid path",
    )
    .await;
    assert!(
        b.get_copy_records(None, &a, &wrong)
            .unwrap()
            .try_collect::<Vec<_>>()
            .await
            .unwrap_err()
            .to_string()
            .contains("path")
    );
    let placeholder = commit(
        b,
        vec![a.clone()],
        tree(b, vec![("legacy", value(&data, &CopyId::placeholder()))]).await,
        "placeholder",
    )
    .await;
    assert!(records(b, &a, &placeholder).await.is_empty());
}

#[tokio::test]
async fn canceled_tree_terms_do_not_emit_deleted_copy_records() {
    let f = Fixture::new();
    let b = &f.backend;
    let data = file(b, b"source").await;
    let ca = copy(b, "a", vec![]).await;
    let a = commit(
        b,
        vec![b.root_commit_id().clone()],
        tree(b, vec![("a", value(&data, &ca))]).await,
        "source",
    )
    .await;
    let cb = copy(b, "b", vec![ca]).await;
    let base = tree(b, vec![("b", value(&data, &cb))]).await;
    // The whole trees differ: cancellation must happen per path, not just by
    // comparing the three tree IDs. The unchanged b term cancels with base b.
    let symlink = b.write_symlink(&path("extra"), "target").await.unwrap();
    let left = tree(
        b,
        vec![
            ("b", value(&data, &cb)),
            ("extra", TreeValue::Symlink(symlink)),
        ],
    )
    .await;
    let mut c = roots::commit();
    c.parents = vec![a.clone()];
    c.change_id = ChangeId::from_bytes(&[9; 16]);
    c.root_tree = Merge::from_vec(vec![left, base, b.empty_tree_id().clone()]);
    c.conflict_labels = Merge::from_vec(vec!["left".into(), "base".into(), "right".into()]);
    let head = b.write_commit(c, None).await.unwrap().0;
    assert!(
        records(b, &a, &head).await.is_empty(),
        "b was deleted by cancellation"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn copy_placeholder_parents_are_boundaries_not_stored_history() {
    let f = Fixture::new();
    let b = &f.backend;
    let file = file(b, b"content").await;
    let base = copy(b, "source", vec![CopyId::placeholder()]).await;
    let root = commit(
        b,
        vec![b.root_commit_id().clone()],
        tree(b, vec![("source", value(&file, &base))]).await,
        "source",
    )
    .await;
    let copied = copy(b, "target", vec![CopyId::placeholder(), base.clone()]).await;
    let head = commit(
        b,
        vec![root.clone()],
        tree(
            b,
            vec![
                ("source", value(&file, &base)),
                ("target", value(&file, &copied)),
            ],
        )
        .await,
        "copy",
    )
    .await;
    let related = b.get_related_copies(&copied).await.unwrap();
    assert_eq!(related.len(), 2);
    assert_eq!(
        related[0].history.parents,
        vec![CopyId::placeholder(), base]
    );
    assert!(related.iter().all(|r| !r.id.as_bytes().is_empty()));
    let records: Vec<_> = b
        .get_copy_records(None, &root, &head)
        .unwrap()
        .try_collect()
        .await
        .unwrap();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].source, path("source"));
    assert_eq!(records[0].target, path("target"));
    assert!(b.get_related_copies(&CopyId::placeholder()).await.is_err());
}
