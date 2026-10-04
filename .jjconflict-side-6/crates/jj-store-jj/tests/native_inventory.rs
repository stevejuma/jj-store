//! Complete native closure, rather than process-local upload bookkeeping.
use jj_lib::merge::Merge;
use jj_lib::repo_path::{RepoPathBuf, RepoPathComponentBuf};
use jj_lib::{backend as b, op_store as o};
use jj_store_jj::{
    codec::{NativeCodec, NativeFile, NativeSymlink},
    dependencies::NativeDependencies as _,
    objects::NativeObjects,
    roots,
};
use jj_store_model::{self as m, ObjectKey};
use jj_store_object::memory::{Fault, MemoryStore};
use jj_store_wal::inventory::{
    self, Budget, DependencySource as _, Representation, VerifiedInventory,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

fn cid(key: ObjectKey) -> b::CommitId {
    b::CommitId::from_bytes(key.as_bytes())
}
fn tid(key: ObjectKey) -> b::TreeId {
    b::TreeId::from_bytes(key.as_bytes())
}
async fn stored<T: NativeCodec>(objects: &NativeObjects, value: &T) -> ObjectKey {
    objects.put(value).await.unwrap().0
}
fn commit(n: usize) -> b::Commit {
    let mut c = roots::commit();
    c.parents = vec![b::CommitId::from_bytes(roots::ROOT_COMMIT.as_bytes())];
    c.description = format!("commit {n}");
    c
}
#[tokio::test]
async fn inventory_includes_invisible_evolution_copies_and_every_reference_term() {
    let store = Arc::new(MemoryStore::default());
    let objects = NativeObjects::new(store, m::RepositoryId::new([1; 16]), 1024 * 1024).unwrap();
    let mut required = BTreeSet::new();
    let mut commits = Vec::new();
    for i in 0..12 {
        let key = stored(&objects, &commit(i)).await;
        required.insert(key);
        commits.push(cid(key));
    }
    let file = stored(&objects, &NativeFile(vec![0, 255, 1])).await;
    required.insert(file);
    let symlink = stored(&objects, &NativeSymlink("../source".into())).await;
    required.insert(symlink);
    let parent = b::CopyHistory {
        current_path: RepoPathBuf::from_internal_string("source").unwrap(),
        parents: vec![],
        salt: vec![1],
    };
    let copy_parent = stored(&objects, &parent).await;
    required.insert(copy_parent);
    let copy = b::CopyHistory {
        current_path: RepoPathBuf::from_internal_string("target").unwrap(),
        parents: vec![
            b::CopyId::from_bytes(copy_parent.as_bytes()),
            b::CopyId::placeholder(),
        ],
        salt: vec![2],
    };
    let copy = stored(&objects, &copy).await;
    required.insert(copy);
    let tree = b::Tree::from_sorted_entries(vec![
        (
            RepoPathComponentBuf::new("file").unwrap(),
            b::TreeValue::File {
                id: b::FileId::from_bytes(file.as_bytes()),
                executable: true,
                copy_id: b::CopyId::from_bytes(copy.as_bytes()),
            },
        ),
        (
            RepoPathComponentBuf::new("link").unwrap(),
            b::TreeValue::Symlink(b::SymlinkId::from_bytes(symlink.as_bytes())),
        ),
        (
            RepoPathComponentBuf::new("module").unwrap(),
            b::TreeValue::GitSubmodule(b::CommitId::from_bytes(&[7; 20])),
        ),
        (
            RepoPathComponentBuf::new("subtree").unwrap(),
            b::TreeValue::Tree(roots::empty_tree_id()),
        ),
    ]);
    let tree = stored(&objects, &tree).await;
    required.insert(tree);
    let mut current = commit(100);
    current.root_tree = Merge::from_vec(vec![tid(tree), roots::empty_tree_id(), tid(tree)]);
    current.conflict_labels = Merge::from_vec(vec!["left".into(), "base".into(), "right".into()]);
    current.predecessors = vec![commits[0].clone()];
    let current = stored(&objects, &current).await;
    required.insert(current);
    let mut view = roots::view();
    view.head_ids = BTreeSet::from([cid(current)]).into_iter().collect();
    view.local_bookmarks.insert(
        "conflicted".into(),
        o::RefTarget::from_merge(Merge::from_vec(vec![
            Some(commits[1].clone()),
            Some(commits[2].clone()),
            None,
        ])),
    );
    view.local_tags
        .insert("tag".into(), o::RefTarget::normal(commits[3].clone()));
    view.remote_views.insert(
        "remote".into(),
        o::RemoteView {
            bookmarks: BTreeMap::from([(
                "bookmark".into(),
                o::RemoteRef {
                    target: o::RefTarget::normal(commits[4].clone()),
                    state: o::RemoteRefState::New,
                },
            )]),
            tags: BTreeMap::from([(
                "tag".into(),
                o::RemoteRef {
                    target: o::RefTarget::normal(commits[5].clone()),
                    state: o::RemoteRefState::Tracked,
                },
            )]),
        },
    );
    view.git_refs.insert(
        "refs/heads/old".into(),
        o::RefTarget::normal(commits[6].clone()),
    );
    view.git_heads
        .insert("laptop".into(), o::RefTarget::normal(commits[7].clone()));
    view.wc_commit_ids
        .insert("desktop".into(), commits[8].clone());
    let view = stored(&objects, &view).await;
    required.insert(view);
    let mut op = roots::operation();
    op.view_id = o::ViewId::from_bytes(view.as_bytes());
    op.parents = vec![o::OperationId::from_bytes(roots::ROOT_OPERATION.as_bytes())];
    op.commit_predecessors = Some(BTreeMap::from([
        (commits[9].clone(), vec![commits[10].clone()]),
        (commits[10].clone(), vec![commits[11].clone()]),
    ]));
    let op = stored(&objects, &op).await;
    required.insert(op);
    let verified = inventory::extend(
        &objects,
        &VerifiedInventory::default(),
        [op],
        Budget::default(),
    )
    .await
    .unwrap();
    let persisted: BTreeSet<_> = verified
        .complete
        .entries()
        .iter()
        .filter(|(_, n)| matches!(n.representation, Representation::Stored { .. }))
        .map(|(key, _)| *key)
        .collect();
    assert_eq!(
        persisted, required,
        "includes objects staged independently and hidden from visible heads"
    );
    assert_eq!(
        verified
            .complete
            .entries()
            .values()
            .filter(|n| n.representation == Representation::Virtual)
            .count(),
        4
    );
    // A file has no graph edges, and a real copy history follows its parents.
    assert!(
        objects
            .load_verified(file, u64::MAX)
            .await
            .unwrap()
            .dependencies
            .is_empty()
    );
    assert_eq!(
        objects
            .load_verified(copy, u64::MAX)
            .await
            .unwrap()
            .dependencies,
        BTreeSet::from([copy_parent])
    );
}
#[tokio::test]
async fn missing_or_corrupt_native_dependencies_never_certify_inventory() {
    let store = Arc::new(MemoryStore::default());
    let objects = NativeObjects::new(store.clone(), m::RepositoryId::new([1; 16]), 1024).unwrap();
    let mut c = commit(0);
    c.predecessors = vec![b::CommitId::from_bytes(&[99; 64])];
    let key = stored(&objects, &c).await;
    assert!(
        inventory::extend(
            &objects,
            &VerifiedInventory::default(),
            [key],
            Budget::default()
        )
        .await
        .is_err()
    );
    store.inject(Fault::ReadCorrupt);
    assert!(
        inventory::extend(
            &objects,
            &VerifiedInventory::default(),
            [key],
            Budget::default()
        )
        .await
        .is_err()
    );
    assert!(objects.load_verified(key, 1).await.is_err());
}
#[test]
fn submodule_and_placeholder_ids_are_external_to_repository_closure() {
    let tree = b::Tree::from_sorted_entries(vec![
        (
            RepoPathComponentBuf::new("file").unwrap(),
            b::TreeValue::File {
                id: b::FileId::from_bytes(&[1; 64]),
                executable: false,
                copy_id: b::CopyId::placeholder(),
            },
        ),
        (
            RepoPathComponentBuf::new("module").unwrap(),
            b::TreeValue::GitSubmodule(b::CommitId::from_bytes(&[2; 20])),
        ),
    ]);
    assert_eq!(
        tree.dependencies().unwrap(),
        BTreeSet::from([ObjectKey::File(m::FileId::new([1; 64]))])
    );
}
