//! Native durable conversion acceptance; memory storage is not RustFS evidence.
use gix::objs::Write as _;
use jj_lib::object_id::ObjectId as _;
use jj_lib::{
    backend::{self as b},
    config::StackedConfig,
    settings::UserSettings,
};
use jj_store_jj::{
    git_bridge::{GitConverter, Limits as GitLimits},
    objects::{NativeObjects, UploadStatus},
};
use jj_store_model as m;
use jj_store_object::memory::MemoryStore;
use jj_store_wal::{self as wal, publication::*};
use std::{collections::BTreeSet, path::Path, sync::Arc};

const REPO: m::RepositoryId = m::RepositoryId::new([71; 16]);
const ACTOR: m::PrincipalId = m::PrincipalId::new([72; 16]);
struct Permit;
#[async_trait::async_trait]
impl PublicationGuard for Permit {
    async fn authorize_replication_seed(
        &self,
        _: &ManifestSnapshot,
        _: &SeedReplicationRequestV1,
    ) -> wal::Result<()> {
        Ok(())
    }
    async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
        Ok(())
    }
    async fn validate_transition(
        &self,
        _: &ManifestSnapshot,
        _: &Request,
        _: &Snapshot,
    ) -> wal::Result<()> {
        Ok(())
    }
    async fn authorize_domain(&self, _: &ManifestSnapshot, _: &DomainRequest) -> wal::Result<()> {
        Ok(())
    }
}
fn coordinator(store: Arc<MemoryStore>, native: Arc<NativeObjects>) -> Coordinator {
    Coordinator::new(store, REPO, native, Arc::new(Permit), Limits::default()).unwrap()
}
fn native(store: Arc<MemoryStore>) -> Arc<NativeObjects> {
    Arc::new(NativeObjects::new(store, REPO, 32 * 1024 * 1024).unwrap())
}
fn converter(
    root: &Path,
    native: Arc<NativeObjects>,
    accepted: Option<&GitStoreSnapshot>,
) -> GitConverter {
    std::fs::create_dir_all(root).unwrap();
    let bare = root.join("objects.git");
    gix::init_bare(&bare).unwrap();
    GitConverter::open(
        &UserSettings::from_config(StackedConfig::with_defaults()).unwrap(),
        &root.join("conversion"),
        &bare,
        native,
        GitLimits::default(),
        accepted,
    )
    .unwrap()
}
fn raw_commit(repo: &gix::Repository, parent: Option<&str>, extra: &str, message: &[u8]) -> String {
    let tree = repo
        .write_object(gix::objs::Tree::empty())
        .unwrap()
        .to_string();
    let parent = parent.map(|p| format!("parent {p}\n")).unwrap_or_default();
    let mut bytes = format!("tree {tree}\n{parent}author A <a@example.invalid> 1000 +0130\ncommitter C <c@example.invalid> 1001 -0230\n{extra}\n").into_bytes();
    bytes.extend_from_slice(message);
    repo.objects
        .write_buf(gix::objs::Kind::Commit, &bytes)
        .unwrap()
        .to_string()
}
async fn accept(c: &Coordinator, converter: &GitConverter, byte: u8) -> GitStoreSnapshot {
    let before = c.read().await.unwrap();
    let pinned = match before.manifest().git_root {
        Some(root) => c.git_store(root).await.unwrap(),
        None => c.empty_git_store().await.unwrap(),
    };
    let staged = c
        .stage_git_records(
            &pinned,
            &converter.records.values().cloned().collect::<Vec<_>>(),
        )
        .await
        .unwrap();
    let request = DomainRequest::new(
        ACTOR,
        m::RequestId::new([byte; 16]),
        m::PublicationId::new([byte; 16]),
        Domain::Git,
        DomainState {
            root: pinned.reference,
            version: before.manifest().git_state_version,
        },
        staged.reference,
        Vec::new(),
    )
    .unwrap();
    let accepted = c.publish_domain(&request).await.unwrap();
    assert!(matches!(accepted, PublishOutcome::Committed(_)));
    assert_eq!(c.publish_domain(&request).await.unwrap(), accepted);
    c.git_store(c.recover(4096).await.unwrap().manifest().git_root.unwrap())
        .await
        .unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn git_domain_keeps_workstore_origins_frozen_and_native_signatures_unsupported() {
    let store = Arc::new(MemoryStore::default());
    let objects = native(store.clone());
    let c = coordinator(store, objects.clone());
    c.initialize(m::EpochId::new([84; 16]), Vec::new())
        .await
        .unwrap();
    c.seed_replication(&SeedReplicationRequestV1 {
        schema_version: 1,
        principal: ACTOR,
        request_id: m::RequestId::new([84; 16]),
        publication_id: m::PublicationId::new([84; 16]),
        replica: m::ReplicaId::new([85; 16]),
        writer_epoch: m::WriterEpoch::new([86; 16]),
    })
    .await
    .unwrap();
    let empty = c.empty_git_store().await.unwrap();
    let request = DomainRequest::new(
        ACTOR,
        m::RequestId::new([87; 16]),
        m::PublicationId::new([87; 16]),
        Domain::Git,
        DomainState {
            root: empty.reference,
            version: 0,
        },
        empty.reference,
        Vec::new(),
    )
    .unwrap();
    let before = c.read().await.unwrap();
    assert!(matches!(
        c.publish_domain(&request).await,
        Err(wal::Error::Unsupported(_))
    ));
    assert_eq!(before.digest(), c.read().await.unwrap().digest());
    c.publish_work_command(
        &WorkCommandRequest::new(
            ACTOR,
            m::RequestId::new([88; 16]),
            m::PublicationId::new([88; 16]),
            WorkCommandV1::CreateProject {
                id: ProjectId::new([88; 16]),
                title: "Home".into(),
                body: "".into(),
            },
        )
        .unwrap(),
    )
    .await
    .unwrap();
    let frozen = c.read().await.unwrap().manifest().replication_root;
    let root = tempfile::tempdir().unwrap();
    let mut bridge = converter(root.path(), objects.clone(), None);
    let repo = gix::open(root.path().join("objects.git")).unwrap();
    let original = raw_commit(&repo, None, "", b"base\n");
    let id = bridge.import_commit(original).await.unwrap();
    accept(&c, &bridge, 89).await;
    assert_eq!(c.read().await.unwrap().manifest().replication_root, frozen);
    let mut signed = objects
        .get::<b::Commit>(m::ObjectKey::Commit(id))
        .await
        .unwrap();
    signed.description = "Signed native revision\n".into();
    signed.secure_sig = Some(b::SecureSig {
        data: b"native-domain-bytes".to_vec(),
        sig: b"signature".to_vec(),
    });
    let (m::ObjectKey::Commit(signed_id), _) = objects.put(&signed).await.unwrap() else {
        panic!()
    };
    assert!(
        bridge
            .export_commit(signed_id)
            .await
            .unwrap_err()
            .to_string()
            .contains("signature")
    );
    let accepted = c.read().await.unwrap();
    let tiny = GitLimits {
        object_bytes: 1,
        ..GitLimits::default()
    };
    let mut bounded = GitConverter::open(
        &UserSettings::from_config(StackedConfig::with_defaults()).unwrap(),
        &root.path().join("bounded-state"),
        &root.path().join("objects.git"),
        objects,
        tiny,
        None,
    )
    .unwrap();
    assert!(
        bounded
            .import_commit(
                repo.objects
                    .write_buf(gix::objs::Kind::Commit, b"oversized\n")
                    .unwrap()
                    .to_string()
            )
            .await
            .is_err()
    );
    assert_eq!(c.read().await.unwrap().digest(), accepted.digest());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn exact_signed_git_parent_and_native_child_survive_cache_loss_copy_and_cold_recovery() {
    let store = Arc::new(MemoryStore::default());
    let objects = native(store.clone());
    let c = coordinator(store.clone(), objects.clone());
    c.initialize(m::EpochId::new([73; 16]), Vec::new())
        .await
        .unwrap();
    let project = WorkCommandRequest::new(
        ACTOR,
        m::RequestId::new([81; 16]),
        m::PublicationId::new([81; 16]),
        WorkCommandV1::CreateProject {
            id: ProjectId::new([81; 16]),
            title: "Preserved home".into(),
            body: "Git bridge must retain this".into(),
        },
    )
    .unwrap();
    c.publish_work_command(&project).await.unwrap();
    let work_before = c
        .read()
        .await
        .unwrap()
        .manifest()
        .domain_state(Domain::Work)
        .unwrap();
    let root = tempfile::tempdir().unwrap();
    let first = root.path().join("first");
    let mut bridge = converter(&first, objects.clone(), None);
    let repo = gix::open(first.join("objects.git")).unwrap();
    let parent = raw_commit(
        &repo,
        None,
        "x-extra unusual header\ngpgsig -----BEGIN PGP SIGNATURE-----\n fixture\n -----END PGP SIGNATURE-----\n",
        b"Original parent without a change-id header\n",
    );
    let tag = repo.objects.write_buf(gix::objs::Kind::Tag, format!("object {parent}\ntype commit\ntag v1\ntagger T <t@example.invalid> 1002 +0000\n\nOriginal annotation\n").as_bytes()).unwrap().to_string();
    let imported = bridge.import_commit(parent.clone()).await.unwrap();
    bridge.retain_object(&tag).await.unwrap();
    let mut child = objects
        .get::<b::Commit>(m::ObjectKey::Commit(imported))
        .await
        .unwrap();
    assert!(child.secure_sig.is_some());
    child.secure_sig = None;
    child.description = "Native child\n".into();
    child.parents = vec![b::CommitId::from_bytes(imported.as_bytes())];
    child.change_id = b::ChangeId::from_bytes(&[44; 16]);
    let (key, status) = objects.put(&child).await.unwrap();
    assert_eq!(status, UploadStatus::Stored);
    let m::ObjectKey::Commit(child_id) = key else {
        panic!()
    };
    let accepted = accept(&c, &bridge, 75).await;
    let snapshot = c.read().await.unwrap();
    assert_eq!(snapshot.manifest().format, 4);
    assert_eq!(snapshot.manifest().code_state_version, 0);
    assert_eq!(
        snapshot.manifest().domain_state(Domain::Work),
        Some(work_before)
    );
    drop(bridge);
    drop(repo);
    std::fs::remove_dir_all(&first).unwrap();
    let second = root.path().join("second");
    let mut bridge = converter(&second, objects.clone(), Some(&accepted));
    assert_eq!(bridge.export_commit(imported).await.unwrap(), parent);
    let child_git = bridge.export_commit(child_id).await.unwrap();
    let repo = gix::open(second.join("objects.git")).unwrap();
    let object = repo
        .find_object(gix::ObjectId::from_hex(child_git.as_bytes()).unwrap())
        .unwrap();
    assert_eq!(
        object
            .try_to_commit_ref()
            .unwrap()
            .parents()
            .map(|p| p.to_string())
            .collect::<Vec<_>>(),
        vec![parent.clone()]
    );
    assert_eq!(
        bridge.import_commit(child_git.clone()).await.unwrap(),
        child_id
    );
    accept(&c, &bridge, 76).await;
    c.checkpoint().await.unwrap();
    let destination = Arc::new(MemoryStore::default());
    let restored_objects = native(destination.clone());
    let copy = c
        .copy_native_to(destination.clone(), restored_objects.clone(), 4096)
        .await
        .unwrap();
    assert_eq!(
        copy.snapshot.manifest().git_root,
        c.read().await.unwrap().manifest().git_root
    );
    let restored = coordinator(destination, restored_objects.clone());
    let selected = restored.recover(4096).await.unwrap();
    let accepted = restored
        .git_store(selected.manifest().git_root.unwrap())
        .await
        .unwrap();
    drop(bridge);
    drop(object);
    drop(repo);
    std::fs::remove_dir_all(&second).unwrap();
    let mut bridge = converter(
        &root.path().join("third"),
        restored_objects,
        Some(&accepted),
    );
    assert_eq!(bridge.export_commit(imported).await.unwrap(), parent);
    assert_eq!(bridge.export_commit(child_id).await.unwrap(), child_git);
    assert!(
        accepted
            .values
            .values()
            .any(|v| matches!(v, GitRecordV1::Object(o) if o.oid == tag))
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn lossy_git_metadata_and_incomplete_object_closure_are_rejected() {
    let store = Arc::new(MemoryStore::default());
    let objects = native(store.clone());
    let c = coordinator(store, objects.clone());
    c.initialize(m::EpochId::new([74; 16]), Vec::new())
        .await
        .unwrap();
    let root = tempfile::tempdir().unwrap();
    let mut bridge = converter(root.path(), objects, None);
    let repo = gix::open(root.path().join("objects.git")).unwrap();
    for (header, message) in [
        ("", b"invalid \xff".as_slice()),
        ("encoding iso-8859-1\n", b"valid".as_slice()),
        ("jj:trees malformed\n", b"valid".as_slice()),
    ] {
        let oid = raw_commit(&repo, None, header, message);
        assert!(
            bridge.import_commit(oid).await.is_err(),
            "accepted unsupported header {header:?}"
        );
    }
    let empty = c.empty_git_store().await.unwrap();
    let incomplete = GitRecordV1::Object(GitObjectV1 {
        oid: "a".repeat(40),
        kind: GitObjectKind::Commit,
        body: m::FileId::new([8; 64]),
        bytes: 1,
        dependencies: BTreeSet::from(["b".repeat(40)]),
    });
    assert!(c.stage_git_records(&empty, &[incomplete]).await.is_err());
    assert!(c.read().await.unwrap().manifest().git_root.is_none());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn git_tree_modes_gitlinks_merge_order_and_deterministic_export_survive_rebuild() {
    let store = Arc::new(MemoryStore::default());
    let objects = native(store.clone());
    let c = coordinator(store, objects.clone());
    c.initialize(m::EpochId::new([78; 16]), Vec::new())
        .await
        .unwrap();
    let root = tempfile::tempdir().unwrap();
    let first = root.path().join("first");
    let mut bridge = converter(&first, objects.clone(), None);
    let repo = gix::open(first.join("objects.git")).unwrap();
    let left = raw_commit(&repo, None, "", b"left\n");
    let right = raw_commit(&repo, None, "", b"right\n");
    let blob = repo
        .objects
        .write_buf(gix::objs::Kind::Blob, b"\0exact binary\xff")
        .unwrap();
    let link = repo
        .objects
        .write_buf(gix::objs::Kind::Blob, b"target/path")
        .unwrap();
    let gitlink = gix::ObjectId::from_hex(b"1234567890123456789012345678901234567890").unwrap();
    let tree = gix::objs::Tree {
        entries: vec![
            gix::objs::tree::Entry {
                mode: gix::objs::tree::EntryKind::BlobExecutable.into(),
                filename: "exec".into(),
                oid: blob,
            },
            gix::objs::tree::Entry {
                mode: gix::objs::tree::EntryKind::Link.into(),
                filename: "link".into(),
                oid: link,
            },
            gix::objs::tree::Entry {
                mode: gix::objs::tree::EntryKind::Commit.into(),
                filename: "submodule".into(),
                oid: gitlink,
            },
        ],
    };
    let tree_oid = repo.write_object(tree).unwrap().to_string();
    let raw = format!(
        "tree {tree_oid}\nparent {right}\nparent {left}\nauthor A <a@example.invalid> 1000 +0000\ncommitter C <c@example.invalid> 1001 +0000\n\nmerge\n"
    );
    let git_oid = repo
        .objects
        .write_buf(gix::objs::Kind::Commit, raw.as_bytes())
        .unwrap()
        .to_string();
    let native_id = bridge.import_commit(git_oid.clone()).await.unwrap();
    let commit = objects
        .get::<b::Commit>(m::ObjectKey::Commit(native_id))
        .await
        .unwrap();
    assert_eq!(commit.parents.len(), 2);
    let native_tree = objects
        .get::<b::Tree>(m::ObjectKey::Tree(
            m::TreeId::from_slice(commit.root_tree.as_resolved().unwrap().as_bytes()).unwrap(),
        ))
        .await
        .unwrap();
    let entries = native_tree.entries().collect::<Vec<_>>();
    assert!(matches!(
        entries[0].value(),
        b::TreeValue::File {
            executable: true,
            ..
        }
    ));
    assert!(matches!(entries[1].value(), b::TreeValue::Symlink(_)));
    assert!(
        matches!(entries[2].value(), b::TreeValue::GitSubmodule(id) if id.as_bytes() == gitlink.as_bytes())
    );
    let accepted = accept(&c, &bridge, 79).await;
    drop(bridge);
    drop(repo);
    std::fs::remove_dir_all(&first).unwrap();
    let mut bridge = converter(
        &root.path().join("second"),
        objects.clone(),
        Some(&accepted),
    );
    assert_eq!(bridge.export_commit(native_id).await.unwrap(), git_oid);
    let mut child = commit;
    child.description = "new native merge projection\n".into();
    child.change_id = b::ChangeId::from_bytes(&[80; 16]);
    let (m::ObjectKey::Commit(child_id), _) = objects.put(&child).await.unwrap() else {
        panic!()
    };
    let exported = bridge.export_commit(child_id).await.unwrap();
    let repo = gix::open(root.path().join("second/objects.git")).unwrap();
    let object = repo
        .find_object(gix::ObjectId::from_hex(exported.as_bytes()).unwrap())
        .unwrap();
    let read = object.try_to_commit_ref().unwrap();
    assert_eq!(read.tree().to_string(), tree_oid);
    assert_eq!(
        read.parents().map(|v| v.to_string()).collect::<Vec<_>>(),
        vec![right, left]
    );
    // Re-encode the same native revision without a recorded child mapping.
    let mut fresh = converter(&root.path().join("third"), objects, Some(&accepted));
    assert_eq!(fresh.export_commit(child_id).await.unwrap(), exported);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn git_catalog_rejects_changed_mappings_missing_closure_and_cross_repository_snapshots() {
    let store = Arc::new(MemoryStore::default());
    let objects = native(store.clone());
    let c = coordinator(store, objects.clone());
    c.initialize(m::EpochId::new([101; 16]), Vec::new())
        .await
        .unwrap();
    let cache = tempfile::tempdir().unwrap();
    let mut bridge = converter(cache.path(), objects.clone(), None);
    let bare = cache.path().join("objects.git");
    let repo = gix::open(&bare).unwrap();
    let oid = raw_commit(&repo, None, "", b"accepted\n");
    let id = bridge.import_commit(oid.clone()).await.unwrap();
    let accepted = accept(&c, &bridge, 102).await;
    let mut changed = accepted
        .values
        .values()
        .find_map(|v| match v {
            GitRecordV1::Object(v) if v.oid == oid => Some(v.clone()),
            _ => None,
        })
        .unwrap();
    changed.bytes += 1;
    assert!(
        c.stage_git_records(&accepted, &[GitRecordV1::Object(changed)])
            .await
            .is_err()
    );
    let mut missing = accepted
        .values
        .values()
        .find_map(|v| match v {
            GitRecordV1::Object(v) if v.oid == oid => Some(v.clone()),
            _ => None,
        })
        .unwrap();
    missing.oid = "ab".repeat(20);
    missing.dependencies.insert("cd".repeat(20));
    assert!(
        c.stage_git_records(&accepted, &[GitRecordV1::Object(missing)])
            .await
            .is_err()
    );
    let mut aliases = accepted.clone();
    let alias = GitRecordV1::Representation(GitRepresentationV1 {
        native_commit: m::CommitId::new([103; 64]),
        git_commit: oid.clone(),
        profile: GIT_PROFILE.into(),
        imported_exact: true,
    });
    aliases.values.insert(alias.key().unwrap(), alias);
    let rebuilt = tempfile::tempdir().unwrap();
    let mut ambiguous = converter(rebuilt.path(), objects.clone(), Some(&aliases));
    assert!(ambiguous.import_commit(oid).await.is_err());
    let other = GitRecordV1::Representation(GitRepresentationV1 {
        native_commit: id,
        git_commit: "ef".repeat(20),
        profile: GIT_PROFILE.into(),
        imported_exact: false,
    });
    aliases.values.insert(other.key().unwrap(), other);
    let rebuilt = tempfile::tempdir().unwrap();
    let mut ambiguous = converter(rebuilt.path(), objects.clone(), Some(&aliases));
    assert!(ambiguous.export_commit(id).await.is_err());
    let mut wrong_repo = accepted;
    wrong_repo.repository = m::RepositoryId::new([104; 16]);
    assert!(c.stage_git_records(&wrong_repo, &[]).await.is_err());
    assert!(
        GitConverter::open(
            &UserSettings::from_config(StackedConfig::with_defaults()).unwrap(),
            &cache.path().join("wrong-conversion"),
            &bare,
            objects.clone(),
            GitLimits::default(),
            Some(&wrong_repo),
        )
        .is_err()
    );
    std::fs::write(
        bare.join("config"),
        "[core]\nrepositoryformatversion = 1\nbare = true\n[extensions]\nobjectformat = sha256\n",
    )
    .unwrap();
    assert!(
        GitConverter::open(
            &UserSettings::from_config(StackedConfig::with_defaults()).unwrap(),
            &cache.path().join("sha256-conversion"),
            &bare,
            objects,
            GitLimits::default(),
            None,
        )
        .is_err()
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn git_acceptance_atomically_keeps_code_evidence_siblings_and_replay_after_checkpoint() {
    use jj_store_jj::roots;
    let store = Arc::new(MemoryStore::default());
    let objects = native(store.clone());
    let c = coordinator(store.clone(), objects.clone());
    c.initialize(m::EpochId::new([110; 16]), Vec::new())
        .await
        .unwrap();
    let base = c.read().await.unwrap().manifest().heads.clone();
    let cache = tempfile::tempdir().unwrap();
    let mut bridge = converter(cache.path(), objects.clone(), None);
    let git = gix::open(cache.path().join("objects.git")).unwrap();
    let oid = raw_commit(&git, None, "", b"fetched\n");
    let id = bridge.import_commit(oid.clone()).await.unwrap();
    let mut view = roots::view();
    view.head_ids.insert(b::CommitId::from_bytes(id.as_bytes()));
    let (m::ObjectKey::View(view), _) = objects.put(&view).await.unwrap() else {
        panic!()
    };
    let mut operation = roots::operation();
    operation.view_id = jj_lib::op_store::ViewId::from_bytes(view.as_bytes());
    operation.parents = vec![jj_lib::op_store::OperationId::from_bytes(
        roots::ROOT_OPERATION.as_bytes(),
    )];
    operation.metadata.description = "fetch origin".into();
    let (m::ObjectKey::Operation(op), _) = objects.put(&operation).await.unwrap() else {
        panic!()
    };
    operation.metadata.description = "independent concurrent work".into();
    let (m::ObjectKey::Operation(sibling), _) = objects.put(&operation).await.unwrap() else {
        panic!()
    };
    let unrelated = Request {
        principal: ACTOR,
        request_id: m::RequestId::new([111; 16]),
        publication_id: m::PublicationId::new([111; 16]),
        transition: Some(Transition::Native {
            remove: base.clone(),
            add: sibling,
        }),
        settings: None,
        result: Vec::new(),
        kind: SemanticKind::OperationPublished,
    };
    c.publish(&unrelated).await.unwrap();
    let remote = m::RequestId::new([112; 16]);
    let binding = m::RecordDigest::new([113; 32]);
    let mut records = bridge.records.values().cloned().collect::<Vec<_>>();
    records.push(GitRecordV1::Remote(GitRemoteV1 {
        remote,
        event: remote,
        predecessors: BTreeSet::new(),
        name: "origin".into(),
        active: true,
    }));
    records.push(GitRecordV1::Observation(GitObservationV1 {
        id: m::RequestId::new([114; 16]),
        remote,
        binding,
        predecessors: BTreeSet::new(),
        operation: op,
        complete_heads: true,
        refs: std::collections::BTreeMap::from([(
            "refs/heads/main".into(),
            GitRefV1 {
                oid: Some(oid.clone()),
                commit_oid: Some(oid.clone()),
                native: Some(id),
            },
        )]),
    }));
    let request = Request {
        principal: ACTOR,
        request_id: m::RequestId::new([115; 16]),
        publication_id: m::PublicationId::new([115; 16]),
        transition: Some(Transition::Native {
            remove: base,
            add: op,
        }),
        settings: None,
        result: Vec::new(),
        kind: SemanticKind::GitStateChanged,
    };
    let receipt = c.publish_git(&request, &records).await.unwrap();
    assert!(matches!(receipt, PublishOutcome::Committed(_)));
    let accepted = c.read().await.unwrap();
    assert_eq!(accepted.manifest().heads, BTreeSet::from([sibling, op]));
    assert_eq!(accepted.manifest().code_state_version, 2);
    assert_eq!(accepted.manifest().git_state_version, 1);
    let catalog = c
        .git_store(accepted.manifest().git_root.unwrap())
        .await
        .unwrap();
    assert_eq!(
        catalog.expectation(remote, binding, "refs/heads/main"),
        Some(Some(oid))
    );
    assert_eq!(
        catalog.expectation(remote, binding, "refs/heads/new"),
        Some(None)
    );
    jj_store_jj::git_bridge::verify_delta(objects.clone(), &catalog, None)
        .await
        .unwrap();
    c.checkpoint().await.unwrap();
    let cold = coordinator(store, objects);
    cold.recover(4096).await.unwrap();
    assert_eq!(cold.publish_git(&request, &records).await.unwrap(), receipt);
    let mut changed = records;
    let GitRecordV1::Remote(v) = &mut changed[bridge.records.len()] else {
        panic!()
    };
    v.name = "changed".into();
    assert!(matches!(
        cold.publish_git(&request, &changed).await,
        Err(wal::Error::IntentConflict)
    ));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn git_observations_keep_filtered_history_and_concurrent_destination_expectations() {
    let store = Arc::new(MemoryStore::default());
    let objects = native(store.clone());
    let c = coordinator(store, objects.clone());
    c.initialize(m::EpochId::new([120; 16]), Vec::new())
        .await
        .unwrap();
    let cache = tempfile::tempdir().unwrap();
    let mut converter = converter(cache.path(), objects, None);
    let repo = gix::open(cache.path().join("objects.git")).unwrap();
    let a = raw_commit(&repo, None, "", b"a\n");
    let b = raw_commit(&repo, Some(&a), "", b"b\n");
    let native_a = converter.import_commit(a.clone()).await.unwrap();
    let native_b = converter.import_commit(b.clone()).await.unwrap();
    let remote = m::RequestId::new([121; 16]);
    let binding = m::RecordDigest::new([122; 32]);
    let mut records = converter.records.values().cloned().collect::<Vec<_>>();
    records.push(GitRecordV1::Remote(GitRemoteV1 {
        remote,
        event: remote,
        predecessors: BTreeSet::new(),
        name: "origin".into(),
        active: true,
    }));
    let observed =
        |byte, predecessors, complete, reference: Option<(&str, String, m::CommitId)>| {
            GitRecordV1::Observation(GitObservationV1 {
                id: m::RequestId::new([byte; 16]),
                remote,
                binding,
                predecessors,
                operation: jj_store_jj::roots::ROOT_OPERATION,
                complete_heads: complete,
                refs: reference
                    .map(|(name, oid, native)| {
                        std::collections::BTreeMap::from([(
                            name.into(),
                            GitRefV1 {
                                oid: Some(oid.clone()),
                                commit_oid: Some(oid),
                                native: Some(native),
                            },
                        )])
                    })
                    .unwrap_or_default(),
            })
        };
    records.push(observed(
        123,
        BTreeSet::new(),
        true,
        Some(("refs/heads/main", a.clone(), native_a)),
    ));
    records.push(observed(
        124,
        BTreeSet::from([m::RequestId::new([123; 16])]),
        false,
        Some(("refs/heads/feature", b.clone(), native_b)),
    ));
    let empty = c.empty_git_store().await.unwrap();
    let staged = c.stage_git_records(&empty, &records).await.unwrap();
    assert_eq!(
        staged.expectation(remote, binding, "refs/heads/main"),
        Some(Some(a.clone()))
    );
    assert_eq!(
        staged.expectation(remote, binding, "refs/heads/new"),
        Some(None)
    );
    let incoming = observed(
        125,
        BTreeSet::from([m::RequestId::new([123; 16])]),
        false,
        Some(("refs/heads/main", b.clone(), native_b)),
    );
    let staged = c.stage_git_records(&staged, &[incoming]).await.unwrap();
    assert_eq!(
        staged.expectation(remote, binding, "refs/heads/main"),
        Some(Some(b.clone()))
    );
    let incoming = observed(
        126,
        BTreeSet::from([m::RequestId::new([123; 16])]),
        false,
        Some(("refs/heads/main", a, native_a)),
    );
    let staged = c.stage_git_records(&staged, &[incoming]).await.unwrap();
    assert_eq!(staged.expectation(remote, binding, "refs/heads/main"), None);
    assert_eq!(
        staged.expectation(remote, m::RecordDigest::new([127; 32]), "refs/heads/main"),
        None
    );
    let reconcile = observed(
        128,
        BTreeSet::from([
            m::RequestId::new([124; 16]),
            m::RequestId::new([125; 16]),
            m::RequestId::new([126; 16]),
        ]),
        true,
        Some(("refs/heads/main", b.clone(), native_b)),
    );
    let staged = c.stage_git_records(&staged, &[reconcile]).await.unwrap();
    assert_eq!(
        staged.expectation(remote, binding, "refs/heads/main"),
        Some(Some(b))
    );
    assert_eq!(
        staged.expectation(remote, binding, "refs/heads/feature"),
        Some(None)
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn git_verification_rejects_forged_dependency_and_import_identity_claims() {
    let store = Arc::new(MemoryStore::default());
    let objects = native(store.clone());
    let c = coordinator(store, objects.clone());
    c.initialize(m::EpochId::new([130; 16]), Vec::new())
        .await
        .unwrap();
    let cache = tempfile::tempdir().unwrap();
    let mut converter = converter(cache.path(), objects.clone(), None);
    let repo = gix::open(cache.path().join("objects.git")).unwrap();
    let oid = raw_commit(&repo, None, "", b"verified\n");
    converter.import_commit(oid.clone()).await.unwrap();
    let empty = c.empty_git_store().await.unwrap();
    let mut forged = converter.records.values().cloned().collect::<Vec<_>>();
    for value in &mut forged {
        if let GitRecordV1::Object(v) = value
            && v.oid == oid
        {
            v.dependencies.clear();
        }
    }
    let staged = c.stage_git_records(&empty, &forged).await.unwrap();
    assert!(
        jj_store_jj::git_bridge::verify_delta(objects.clone(), &staged, Some(&empty))
            .await
            .is_err()
    );
    let mut forged = converter.records.values().cloned().collect::<Vec<_>>();
    for value in &mut forged {
        if let GitRecordV1::Representation(v) = value {
            v.native_commit = m::CommitId::new([131; 64]);
        }
    }
    let staged = c.stage_git_records(&empty, &forged).await.unwrap();
    assert!(
        jj_store_jj::git_bridge::verify_delta(objects, &staged, Some(&empty))
            .await
            .is_err()
    );
}
