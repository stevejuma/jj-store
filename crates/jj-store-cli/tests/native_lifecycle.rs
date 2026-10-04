//! Real pinned JJ transactions over production authentication/publication. The
//! memory transport is not deployment conformance or executable CLI evidence.
use futures::{AsyncReadExt as _, StreamExt as _, TryStreamExt as _};
use jj_lib::{
    backend as b,
    config::StackedConfig,
    default_backend_factories::{default_backend_factories, default_working_copy_factories},
    gitignore::GitIgnoreFile,
    matchers::{EverythingMatcher, NothingMatcher},
    object_id::ObjectId as _,
    op_store as o,
    repo::{ReadonlyRepo, Repo as _},
    repo_path::RepoPathBuf,
    settings::UserSettings,
    working_copy::SnapshotOptions,
    workspace::Workspace,
};
use jj_store_jj::{
    backend::{BackendAuthority, BackendLimits},
    factories::NativeStores,
    objects::NativeObjects,
    operation_stores::{OperationAuthority, OperationLimits},
    policy::Protection,
    roots,
};
use jj_store_model as m;
use jj_store_object::memory::MemoryStore;
use jj_store_service::{auth::*, native::*};
use jj_store_wal::{self as wal, publication::*};
use std::{collections::BTreeSet, fs, sync::Arc};
const REPO: m::RepositoryId = m::RepositoryId::new([1; 16]);
const ACTOR: m::PrincipalId = m::PrincipalId::new([2; 16]);
struct Fixture {
    store: Arc<MemoryStore>,
    objects: Arc<NativeObjects>,
    policy: TokenPolicy,
    token: BearerToken,
    reader: BearerToken,
    session: Arc<NativeSession>,
    stores: NativeStores,
}
impl Fixture {
    async fn new() -> Self {
        let store = Arc::new(MemoryStore::default());
        let objects = Arc::new(NativeObjects::new(store.clone(), REPO, 4 * 1024 * 1024).unwrap());
        let mut policy = TokenPolicy::new(Scope::Repository(REPO));
        let token = policy.issue(ACTOR, Permission::Admin, None).unwrap();
        let reader = policy
            .issue(m::PrincipalId::new([3; 16]), Permission::Read, None)
            .unwrap();
        let mut operator = TokenPolicy::new(Scope::Operator);
        let op = operator
            .issue(ACTOR, Permission::CreateRepository, None)
            .unwrap();
        create_repository(
            store.clone(),
            objects.clone(),
            &operator,
            &op,
            &policy,
            m::EpochId::new([4; 16]),
            &SystemClock,
            Limits::default(),
        )
        .await
        .unwrap();
        let session = Arc::new(
            NativeSession::new(
                store.clone(),
                objects.clone(),
                token.clone(),
                Arc::new(SystemClock),
                Protection::default(),
                SessionLimits::default(),
            )
            .unwrap(),
        );
        let stores = NativeStores::new(
            objects.clone(),
            session.clone(),
            BackendLimits::default(),
            OperationLimits::default(),
        )
        .unwrap();
        Self {
            store,
            objects,
            policy,
            token,
            reader,
            session,
            stores,
        }
    }
    fn session(&self, token: BearerToken, protection: Protection) -> Arc<NativeSession> {
        Arc::new(
            NativeSession::new(
                self.store.clone(),
                self.objects.clone(),
                token,
                Arc::new(SystemClock),
                protection,
                SessionLimits::default(),
            )
            .unwrap(),
        )
    }
    fn stores(&self, session: Arc<NativeSession>) -> NativeStores {
        NativeStores::new(
            self.objects.clone(),
            session,
            BackendLimits::default(),
            OperationLimits::default(),
        )
        .unwrap()
    }
}
fn settings() -> UserSettings {
    UserSettings::from_config(StackedConfig::with_defaults()).unwrap()
}
fn oid(id: &o::OperationId) -> m::OperationId {
    m::OperationId::from_slice(id.as_bytes()).unwrap()
}
fn cid(id: &b::CommitId) -> m::CommitId {
    m::CommitId::from_slice(id.as_bytes()).unwrap()
}
async fn describe(
    repo: &Arc<ReadonlyRepo>,
    old: &jj_lib::commit::Commit,
    text: &str,
) -> (Arc<ReadonlyRepo>, jj_lib::commit::Commit) {
    let mut tx = repo.start_transaction();
    let new = tx
        .repo_mut()
        .rewrite_commit(old)
        .set_description(text)
        .write()
        .await
        .unwrap();
    tx.repo_mut().rebase_descendants().await.unwrap();
    (tx.commit("describe fixture").await.unwrap(), new)
}
async fn snapshot(
    workspace: &mut Workspace,
    repo: &Arc<ReadonlyRepo>,
) -> (Arc<ReadonlyRepo>, jj_lib::commit::Commit) {
    let name = workspace.workspace_name().to_owned();
    let old = repo
        .store()
        .get_commit(repo.view().get_wc_commit_id(&name).unwrap())
        .unwrap();
    let mut locked = workspace.start_working_copy_mutation().await.unwrap();
    let (tree, _) = locked
        .locked_wc()
        .snapshot(&SnapshotOptions {
            base_ignores: GitIgnoreFile::empty(),
            progress: None,
            start_tracking_matcher: &EverythingMatcher,
            force_tracking_matcher: &NothingMatcher,
            max_new_file_size: 4 * 1024 * 1024,
        })
        .await
        .unwrap();
    let mut tx = repo.start_transaction();
    let new = tx
        .repo_mut()
        .rewrite_commit(&old)
        .set_tree(tree)
        .write()
        .await
        .unwrap();
    tx.repo_mut().rebase_descendants().await.unwrap();
    let repo = tx.commit("snapshot physical working copy").await.unwrap();
    locked.finish(repo.op_id().clone()).await.unwrap();
    (repo, new)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn authenticated_native_workspace_edit_describe_reopen_and_cache_rebuild() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let working = tempfile::tempdir().unwrap();
            let settings = settings();
            f.stores
                .initialize_cache(&settings, cache.path())
                .await
                .unwrap();
            let (mut workspace, repo) = f
                .stores
                .attach_workspace(
                    &settings,
                    working.path(),
                    cache.path(),
                    "device-a-unique".into(),
                )
                .await
                .unwrap();
            fs::write(working.path().join("hello.txt"), b"native content\n").unwrap();
            let (repo, wc) = snapshot(&mut workspace, &repo).await;
            let (repo, described) = describe(&repo, &wc, "hello native storage").await;
            let operation = oid(repo.op_id());
            let commit_id = described.id().clone();
            assert_eq!(repo.store().backend().name(), "jj-store");
            assert!(!working.path().join(".git").exists());
            let mut factories = default_backend_factories();
            factories.merge(f.stores.factories());
            drop(workspace);
            drop(repo);
            drop(described);
            let workspace = Workspace::load(
                &settings,
                working.path(),
                &factories,
                &default_working_copy_factories(),
            )
            .unwrap();
            let reopened = f
                .stores
                .load_at(workspace.repo_loader(), operation)
                .await
                .unwrap();
            assert_eq!(
                reopened
                    .store()
                    .get_commit(&commit_id)
                    .unwrap()
                    .description(),
                "hello native storage"
            );
            let log: Vec<_> = jj_lib::op_walk::walk_ancestors(&[reopened.operation().clone()])
                .take(10)
                .try_collect()
                .await
                .unwrap();
            assert!(log.len() >= 4);
            let evolution: Vec<_> = jj_lib::evolution::walk_predecessors(
                reopened.as_ref(),
                std::slice::from_ref(&commit_id),
            )
            .take(10)
            .try_collect()
            .await
            .unwrap();
            assert!(evolution.len() >= 3);
            // Fresh cache has no derived index or checkout state from the first client.
            let clean = tempfile::tempdir().unwrap();
            let cold_session = f.session(f.token.clone(), Protection::default());
            let cold = f.stores(cold_session);
            let loader = cold
                .initialize_cache(&settings, clean.path())
                .await
                .unwrap();
            let recovered = cold.load_at(&loader, operation).await.unwrap();
            let recovered_commit = recovered.store().get_commit(&commit_id).unwrap();
            let tree = recovered_commit.tree();
            let entry = tree
                .path_value(&RepoPathBuf::from_internal_string("hello.txt").unwrap())
                .await
                .unwrap();
            let b::TreeValue::File { id, .. } = entry.as_resolved().unwrap().as_ref().unwrap()
            else {
                panic!("file")
            };
            let mut reader = recovered
                .store()
                .read_file(&RepoPathBuf::from_internal_string("hello.txt").unwrap(), id)
                .await
                .unwrap();
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await.unwrap();
            assert_eq!(bytes, b"native content\n");
            for name in ["store", "op_store", "op_heads"] {
                let files = fs::read_dir(clean.path().join(name)).unwrap().count();
                assert_eq!(files, 2, "cache contains only type and profile binding");
            }
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn independent_workspaces_preserve_concurrent_operations_and_unsnapshotted_edits() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let settings = settings();
            let ca = tempfile::tempdir().unwrap();
            let cb = tempfile::tempdir().unwrap();
            let wa = tempfile::tempdir().unwrap();
            let wb = tempfile::tempdir().unwrap();
            f.stores
                .initialize_cache(&settings, ca.path())
                .await
                .unwrap();
            let (mut a, ra) = f
                .stores
                .attach_workspace(&settings, wa.path(), ca.path(), "device-a".into())
                .await
                .unwrap();
            f.stores
                .initialize_cache(&settings, cb.path())
                .await
                .unwrap();
            let (mut b, rb) = f
                .stores
                .attach_workspace(&settings, wb.path(), cb.path(), "device-b".into())
                .await
                .unwrap();
            assert!(
                rb.view().get_wc_commit_id(a.workspace_name()).is_some(),
                "attach discarded existing history"
            );
            fs::write(wa.path().join("a.txt"), b"a local edits").unwrap();
            fs::write(wb.path().join("b.txt"), b"b local edits").unwrap();
            let (ra, _) = snapshot(&mut a, &ra).await;
            let (rb, _) = snapshot(&mut b, &rb).await;
            let heads = f.session.heads().await.unwrap();
            assert_eq!(heads.len(), 2);
            assert!(heads.contains(&oid(ra.op_id())));
            assert!(heads.contains(&oid(rb.op_id())));
            let before = f.session.snapshot().await.unwrap().manifest().published;
            f.stores
                .load_at(a.repo_loader(), oid(ra.op_id()))
                .await
                .unwrap();
            assert_eq!(
                f.session.snapshot().await.unwrap().manifest().published,
                before,
                "pinned read published reconciliation"
            );
            fs::write(wa.path().join("uncommitted.txt"), b"must survive").unwrap();
            let merged = f.stores.load_for_mutation(b.repo_loader()).await.unwrap();
            assert_eq!(f.session.heads().await.unwrap().len(), 1);
            assert!(merged.view().get_wc_commit_id(a.workspace_name()).is_some());
            assert!(merged.view().get_wc_commit_id(b.workspace_name()).is_some());
            assert_eq!(
                fs::read(wa.path().join("uncommitted.txt")).unwrap(),
                b"must survive"
            );
            assert_eq!(fs::read(wb.path().join("b.txt")).unwrap(), b"b local edits");
            assert!(
                f.stores
                    .attach_workspace(
                        &settings,
                        tempfile::tempdir().unwrap().path(),
                        ca.path(),
                        "device-a".into()
                    )
                    .await
                    .is_err()
            );
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn transition_identity_survives_session_restart_and_late_duplicates() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let working = tempfile::tempdir().unwrap();
            let settings = settings();
            f.stores
                .initialize_cache(&settings, cache.path())
                .await
                .unwrap();
            let (_, repo) = f
                .stores
                .attach_workspace(&settings, working.path(), cache.path(), "one".into())
                .await
                .unwrap();
            let old = BTreeSet::from([roots::ROOT_OPERATION]);
            let request = f
                .session
                .transition_request(old.clone(), oid(repo.op_id()))
                .await
                .unwrap();
            let recovered = f.session(f.token.clone(), Protection::default());
            assert_eq!(
                request,
                recovered
                    .transition_request(old, oid(repo.op_id()))
                    .await
                    .unwrap()
            );
            let duplicate = recovered.publish_request(&request).await.unwrap();
            let PublishOutcome::Committed(receipt) = duplicate else {
                panic!("receipt")
            };
            let mut tx = repo.start_transaction();
            tx.set_attribute("another".into(), "operation".into());
            let newer = tx
                .commit("another operation in same invocation")
                .await
                .unwrap();
            let second = f
                .session
                .transition_request(BTreeSet::from([oid(repo.op_id())]), oid(newer.op_id()))
                .await
                .unwrap();
            assert_ne!(request.request_id, second.request_id);
            let before = f.session.snapshot().await.unwrap().manifest().snapshot();
            assert_eq!(
                recovered.publish_request(&request).await.unwrap(),
                PublishOutcome::Committed(receipt)
            );
            assert_eq!(
                f.session.snapshot().await.unwrap().manifest().snapshot(),
                before
            );
        })
        .await;
}

async fn stage_view(
    f: &Fixture,
    repo: &Arc<ReadonlyRepo>,
    view: o::View,
    label: &str,
) -> m::OperationId {
    let (key, _) = f.objects.put(&view).await.unwrap();
    let mut op = repo.operation().store_operation().clone();
    op.parents = vec![repo.op_id().clone()];
    op.view_id = o::ViewId::from_bytes(key.as_bytes());
    op.metadata.description = label.into();
    op.commit_predecessors = None;
    let (key, _) = f.objects.put(&op).await.unwrap();
    m::OperationId::from_slice(key.as_bytes()).unwrap()
}
async fn init_working(
    f: &Fixture,
    cache: &std::path::Path,
    working: &std::path::Path,
) -> Arc<ReadonlyRepo> {
    let settings = settings();
    f.stores.initialize_cache(&settings, cache).await.unwrap();
    f.stores
        .attach_workspace(&settings, working, cache, "fixture".into())
        .await
        .unwrap()
        .1
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn protected_history_rejects_hidden_rewrites_without_predecessor_metadata() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let working = tempfile::tempdir().unwrap();
            let repo = init_working(&f, cache.path(), working.path()).await;
            let original = repo
                .store()
                .get_commit(
                    repo.view()
                        .get_wc_commit_id(jj_lib::ref_name::WorkspaceName::new("fixture"))
                        .unwrap(),
                )
                .unwrap();
            let session = f.session(
                f.token.clone(),
                Protection {
                    commits: BTreeSet::from([cid(original.id())]),
                    bookmarks: BTreeSet::new(),
                },
            );
            let mut rewritten = original.store_commit().as_ref().clone();
            rewritten.description = "forged rewrite".into();
            rewritten.predecessors.clear();
            let (key, _) = f.objects.put(&rewritten).await.unwrap();
            let mut view = repo.view().store_view().clone();
            view.head_ids = BTreeSet::from([b::CommitId::from_bytes(key.as_bytes())])
                .into_iter()
                .collect();
            let new = stage_view(&f, &repo, view, "no evolution records supplied").await;
            let before = f.session.snapshot().await.unwrap().manifest().snapshot();
            let request = session
                .transition_request(BTreeSet::from([oid(repo.op_id())]), new)
                .await
                .unwrap();
            assert!(matches!(
                session.publish_request(&request).await,
                Err(wal::Error::Denied)
            ));
            assert_eq!(
                f.session.snapshot().await.unwrap().manifest().snapshot(),
                before
            );
            let hidden = stage_view(&f, &repo, roots::view(), "hide protected commit").await;
            assert!(
                session
                    .publish(BTreeSet::from([oid(repo.op_id())]), hidden)
                    .await
                    .is_err()
            );
            // New descendants with a distinct native change ID remain permitted.
            rewritten.change_id = b::ChangeId::from_bytes(&[8; 16]);
            rewritten.parents = vec![original.id().clone()];
            let (key, _) = f.objects.put(&rewritten).await.unwrap();
            let mut view = repo.view().store_view().clone();
            view.head_ids = [b::CommitId::from_bytes(key.as_bytes())]
                .into_iter()
                .collect();
            let new = stage_view(&f, &repo, view, "allowed new descendant").await;
            session
                .publish(BTreeSet::from([oid(repo.op_id())]), new)
                .await
                .unwrap();
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn protected_bookmark_also_protects_its_commit_identity() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let working = tempfile::tempdir().unwrap();
            let repo = init_working(&f, cache.path(), working.path()).await;
            let original = repo
                .store()
                .get_commit(
                    repo.view()
                        .get_wc_commit_id(jj_lib::ref_name::WorkspaceName::new("fixture"))
                        .unwrap(),
                )
                .unwrap();
            let mut tx = repo.start_transaction();
            tx.repo_mut().set_local_bookmark_target(
                jj_lib::ref_name::RefName::new("stable"),
                o::RefTarget::normal(original.id().clone()),
            );
            let repo = tx.commit("protected bookmark").await.unwrap();
            let session = f.session(
                f.token.clone(),
                Protection {
                    commits: BTreeSet::new(),
                    bookmarks: BTreeSet::from(["stable".into()]),
                },
            );
            let mut changed = repo.view().store_view().clone();
            changed
                .local_bookmarks
                .remove(jj_lib::ref_name::RefName::new("stable"));
            let new = stage_view(&f, &repo, changed, "delete protected bookmark").await;
            assert!(
                session
                    .publish(BTreeSet::from([oid(repo.op_id())]), new)
                    .await
                    .is_err()
            );
            let mut rewritten = original.store_commit().as_ref().clone();
            rewritten.description = "divergent protected change".into();
            rewritten.predecessors.clear();
            let (key, _) = f.objects.put(&rewritten).await.unwrap();
            let mut view = repo.view().store_view().clone();
            view.head_ids
                .insert(b::CommitId::from_bytes(key.as_bytes()));
            let new = stage_view(&f, &repo, view, "retain bookmark but rewrite same change").await;
            assert!(
                session
                    .publish(BTreeSet::from([oid(repo.op_id())]), new)
                    .await
                    .is_err(),
                "protected bookmark's change was rewritten"
            );
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn committed_superseded_operation_cannot_be_resurrected_under_a_new_request() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let working = tempfile::tempdir().unwrap();
            let repo = init_working(&f, cache.path(), working.path()).await;
            let old = BTreeSet::from([roots::ROOT_OPERATION]);
            let mut request = f
                .session
                .transition_request(old, oid(repo.op_id()))
                .await
                .unwrap();
            let mut tx = repo.start_transaction();
            tx.set_attribute("new".into(), "head".into());
            tx.commit("supersede").await.unwrap();
            request.request_id = m::RequestId::new([51; 16]);
            request.publication_id = m::PublicationId::new([52; 16]);
            assert!(
                matches!(
                    f.session.publish_request(&request).await,
                    Err(wal::Error::Denied)
                ),
                "new request resurrected an obsolete operation head"
            );
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn current_permissions_revocation_and_repository_binding_cover_native_access() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let mut f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let working = tempfile::tempdir().unwrap();
            let repo = init_working(&f, cache.path(), working.path()).await;
            let reader = f.session(f.reader.clone(), Protection::default());
            assert!(
                BackendAuthority::authorize_read(reader.as_ref())
                    .await
                    .is_ok()
            );
            assert!(
                BackendAuthority::authorize_write(reader.as_ref())
                    .await
                    .is_err()
            );
            let read_stores = f.stores(reader.clone());
            let loader = read_stores.load_cache(&settings(), cache.path()).unwrap();
            assert!(
                read_stores
                    .load_at(&loader, oid(repo.op_id()))
                    .await
                    .is_ok()
            );
            assert!(read_stores.load_for_mutation(&loader).await.is_err());
            let mut request = f
                .session
                .transition_request(BTreeSet::from([roots::ROOT_OPERATION]), oid(repo.op_id()))
                .await
                .unwrap();
            request.principal = m::PrincipalId::new([99; 16]);
            assert!(matches!(
                f.session.publish_request(&request).await,
                Err(wal::Error::Denied)
            ));
            let foreign_objects = Arc::new(
                NativeObjects::new(f.store.clone(), m::RepositoryId::new([33; 16]), 1024).unwrap(),
            );
            assert!(
                NativeStores::new(
                    foreign_objects,
                    f.session.clone(),
                    BackendLimits::default(),
                    OperationLimits::default()
                )
                .is_err()
            );
            let marker = cache.path().join("store/native-profile");
            let original = fs::read(&marker).unwrap();
            fs::write(&marker, b"wrong profile").unwrap();
            assert!(f.stores.load_cache(&settings(), cache.path()).is_err());
            fs::write(&marker, original).unwrap();
            assert!(
                create_repository(
                    f.store.clone(),
                    f.objects.clone(),
                    &f.policy,
                    &f.token,
                    &f.policy,
                    m::EpochId::new([4; 16]),
                    &SystemClock,
                    Limits::default()
                )
                .await
                .is_err(),
                "repository admin created a repository without operator capability"
            );
            f.policy.revoke(f.token.id());
            let request = Request {
                principal: ACTOR,
                request_id: m::RequestId::new([60; 16]),
                publication_id: m::PublicationId::new([61; 16]),
                transition: None,
                settings: Some(f.policy.encode().unwrap()),
                result: vec![],
                kind: SemanticKind::SettingsChanged,
            };
            assert!(matches!(
                f.session.publish_request(&request).await.unwrap(),
                PublishOutcome::Committed(_)
            ));
            assert!(f.session.heads().await.is_err());
            assert!(f.stores.load_at(&loader, oid(repo.op_id())).await.is_err());
            assert!(f.session.publish_request(&request).await.is_err());
            assert!(reader.heads().await.is_ok());
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn concurrent_workspace_name_claims_cannot_alias_physical_clients() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let loader = f
                .stores
                .initialize_cache(&settings(), cache.path())
                .await
                .unwrap();
            let base = f
                .stores
                .load_at(&loader, roots::ROOT_OPERATION)
                .await
                .unwrap();
            let mut a = base.start_transaction();
            a.repo_mut()
                .check_out("same-name".into(), &base.store().root_commit())
                .await
                .unwrap();
            a.commit("first physical client").await.unwrap();
            let mut b = base.start_transaction();
            b.repo_mut()
                .check_out("same-name".into(), &base.store().root_commit())
                .await
                .unwrap();
            assert!(
                b.commit("second physical client").await.is_err(),
                "concurrent clients claimed the same logical workspace"
            );
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn malformed_native_publications_stale_strict_intents_and_unknown_retries() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let working = tempfile::tempdir().unwrap();
            let repo = init_working(&f, cache.path(), working.path()).await;
            let base = f.session.snapshot().await.unwrap().manifest().snapshot();
            let old = BTreeSet::from([oid(repo.op_id())]);
            let mut invalid = repo.operation().store_operation().clone();
            invalid.parents.clear();
            let (key, _) = f.objects.put(&invalid).await.unwrap();
            let bad = m::OperationId::from_slice(key.as_bytes()).unwrap();
            assert!(f.session.publish(old.clone(), bad).await.is_err());
            assert!(
                f.session
                    .publish(old.clone(), roots::ROOT_OPERATION)
                    .await
                    .is_err()
            );
            let mut root_rewrite = roots::commit();
            root_rewrite.parents = vec![repo.store().root_commit_id().clone()];
            root_rewrite.description = "forged root rewrite without predecessors".into();
            let (key, _) = f.objects.put(&root_rewrite).await.unwrap();
            let mut view = repo.view().store_view().clone();
            view.head_ids = [b::CommitId::from_bytes(key.as_bytes())]
                .into_iter()
                .collect();
            let new = stage_view(&f, &repo, view, "rewrite root").await;
            assert!(f.session.publish(old.clone(), new).await.is_err());
            assert_eq!(
                f.session.snapshot().await.unwrap().manifest().snapshot(),
                base
            );
            let new = stage_view(
                &f,
                &repo,
                repo.view().store_view().clone(),
                "unknown then retry",
            )
            .await;
            let request = f
                .session
                .transition_request(old.clone(), new)
                .await
                .unwrap();
            f.store.inject(jj_store_object::memory::Fault::BeforeWrite);
            assert!(matches!(
                f.session.publish_request(&request).await.unwrap(),
                PublishOutcome::OutcomeUnknown { .. }
            ));
            let cold = f.session(f.token.clone(), Protection::default());
            let retry = cold.transition_request(old, new).await.unwrap();
            assert_eq!(retry, request);
            assert!(matches!(
                cold.publish_request(&retry).await.unwrap(),
                PublishOutcome::Committed(_)
            ));
            let stale = Request {
                principal: ACTOR,
                request_id: m::RequestId::new([80; 16]),
                publication_id: m::PublicationId::new([81; 16]),
                transition: Some(Transition::Strict {
                    expected: base,
                    heads: BTreeSet::from([new]),
                }),
                settings: None,
                result: vec![],
                kind: SemanticKind::OperationPublished,
            };
            assert!(matches!(
                cold.publish_request(&stale).await,
                Err(wal::Error::Stale)
            ));
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn protection_can_freeze_multiple_existing_divergent_incarnations() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let working = tempfile::tempdir().unwrap();
            let repo = init_working(&f, cache.path(), working.path()).await;
            let original = repo
                .store()
                .get_commit(
                    repo.view()
                        .get_wc_commit_id(jj_lib::ref_name::WorkspaceName::new("fixture"))
                        .unwrap(),
                )
                .unwrap();
            let mut divergent = original.store_commit().as_ref().clone();
            divergent.description = "already accepted divergence".into();
            let (key, _) = f.objects.put(&divergent).await.unwrap();
            let divergent_id = b::CommitId::from_bytes(key.as_bytes());
            let mut view = repo.view().store_view().clone();
            view.head_ids.insert(divergent_id.clone());
            let divergence = stage_view(&f, &repo, view, "introduce divergence").await;
            f.session
                .publish(BTreeSet::from([oid(repo.op_id())]), divergence)
                .await
                .unwrap();
            let base = f.stores.load_at(repo.loader(), divergence).await.unwrap();
            let session = f.session(
                f.token.clone(),
                Protection {
                    commits: BTreeSet::from([cid(original.id()), cid(&divergent_id)]),
                    bookmarks: BTreeSet::new(),
                },
            );
            let unchanged = stage_view(
                &f,
                &base,
                base.view().store_view().clone(),
                "retain both frozen incarnations",
            )
            .await;
            assert!(
                session
                    .publish(BTreeSet::from([divergence]), unchanged)
                    .await
                    .is_ok(),
                "unchanged protected divergent history was rejected"
            );
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn read_session_point_lookup_does_not_materialize_inventory() {
    use jj_store_service::reads::{OpenRead, ReadLimits};
    let f = Fixture::new().await;
    let before = f.session.snapshot().await.unwrap();
    assert!(before.inventory(f.store.clone(), 100).await.unwrap().len() > 1);
    let OpenRead::Pinned(read) = f
        .session
        .open_read(
            None,
            ReadLimits {
                max_objects: 1,
                ..ReadLimits::default()
            },
        )
        .await
        .expect("one requested operation must not charge unrelated inventory objects")
    else {
        panic!("sole head must pin")
    };
    let selected = read.snapshot().await.unwrap().operation_id;
    read.authorize_object(m::ObjectKey::Operation(selected))
        .await
        .unwrap();
    read.authorize_object(m::ObjectKey::Operation(selected))
        .await
        .unwrap();
    assert_eq!(
        f.session.snapshot().await.unwrap().manifest(),
        before.manifest()
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn read_session_pins_history_and_reports_divergent_heads_without_publication() {
    use jj_store_service::reads::{OpenRead, ReadLimits};
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let cache = tempfile::tempdir().unwrap();
            let work = tempfile::tempdir().unwrap();
            let repo = init_working(&f, cache.path(), work.path()).await;
            let before = f.session.snapshot().await.unwrap().manifest().clone();
            let OpenRead::Pinned(read) = f
                .session
                .open_read(None, ReadLimits::default())
                .await
                .unwrap()
            else {
                panic!("single head must pin")
            };
            let pinned = read.snapshot().await.unwrap().clone();
            assert_eq!(pinned.operation_id, oid(repo.op_id()));
            assert_eq!(pinned.authority.manifest_revision, before.revision);
            assert_eq!(f.session.snapshot().await.unwrap().manifest(), &before);
            let first = repo
                .start_transaction()
                .commit("first sibling")
                .await
                .unwrap();
            let second = repo
                .start_transaction()
                .commit("second sibling")
                .await
                .unwrap();
            let siblings = f.session.snapshot().await.unwrap().manifest().clone();
            assert_eq!(siblings.heads.len(), 2);
            assert_eq!(read.snapshot().await.unwrap(), &pinned);
            assert!(
                matches!(
                    f.session
                        .open_read(
                            None,
                            ReadLimits {
                                timeout: std::time::Duration::from_nanos(1),
                                ..ReadLimits::default()
                            }
                        )
                        .await,
                    Err(jj_store_service::reads::Error::Limit)
                ),
                "ready multi-head response escaped its elapsed deadline"
            );
            let OpenRead::SelectHead(selection) = f
                .session
                .open_read(None, ReadLimits::default())
                .await
                .unwrap()
            else {
                panic!("concurrent heads must be explicit")
            };
            assert_eq!(
                selection.heads,
                BTreeSet::from([oid(first.op_id()), oid(second.op_id())])
            );
            let OpenRead::Pinned(historical) = f
                .session
                .open_read(Some(oid(repo.op_id())), ReadLimits::default())
                .await
                .unwrap()
            else {
                panic!("explicit historical operation must pin")
            };
            assert_eq!(
                historical.snapshot().await.unwrap().operation_id,
                oid(repo.op_id())
            );
            assert_eq!(f.session.snapshot().await.unwrap().manifest(), &siblings);
            assert!(matches!(
                read.authorize_object(m::ObjectKey::Operation(oid(first.op_id())))
                    .await,
                Err(jj_store_service::reads::Error::NotCommitted)
            ));
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn read_session_excludes_staged_operations_and_rechecks_revocation() {
    use jj_store_service::{
        administration::{TokenChange, TokenRequest},
        reads::{Error, OpenRead, ReadLimits},
    };
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let reader = f.session(f.reader.clone(), Protection::default());
            let OpenRead::Pinned(read) =
                reader.open_read(None, ReadLimits::default()).await.unwrap()
            else {
                panic!("initial head must pin")
            };
            let before = f.session.snapshot().await.unwrap().manifest().clone();
            let mut candidate = roots::operation();
            candidate.parents = vec![o::OperationId::from_bytes(roots::ROOT_OPERATION.as_bytes())];
            candidate.metadata.description = "unpublished candidate".into();
            let (key, _) = f.objects.put(&candidate).await.unwrap();
            let m::ObjectKey::Operation(id) = key else {
                unreachable!()
            };
            assert!(matches!(
                reader.open_read(Some(id), ReadLimits::default()).await,
                Err(Error::NotCommitted)
            ));
            assert_eq!(read.authorize_object(key).await, Err(Error::NotCommitted));
            assert_eq!(f.session.snapshot().await.unwrap().manifest(), &before);
            f.session
                .change_token(&TokenRequest {
                    request_id: m::RequestId::new([211; 16]),
                    expected: before.snapshot(),
                    change: TokenChange::Revoke {
                        token: f.reader.id(),
                    },
                })
                .await
                .unwrap();
            assert_eq!(read.snapshot().await, Err(Error::Denied));
            assert_eq!(read.authorize_object(key).await, Err(Error::Denied));
            assert!(matches!(
                reader.open_read(Some(id), ReadLimits::default()).await,
                Err(Error::Denied)
            ));
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn read_session_enforces_inventory_and_elapsed_budgets() {
    use jj_store_service::reads::{Error, OpenRead, ReadLimits};
    let f = Fixture::new().await;
    let OpenRead::Pinned(tiny) = f
        .session
        .open_read(
            None,
            ReadLimits {
                max_objects: 1,
                ..ReadLimits::default()
            },
        )
        .await
        .unwrap()
    else {
        panic!("sole head")
    };
    // Opening charges only the selected operation. A second distinct object
    // still exceeds this request's object budget; complete enumeration is bounded.
    assert_eq!(
        tiny.authorize_object(m::ObjectKey::View(roots::ROOT_VIEW))
            .await,
        Err(Error::Limit)
    );
    assert_eq!(
        jj_store_jj::query::ReadAccess::inventory(&tiny).await,
        Err(jj_store_jj::query::Error::Limit)
    );
    assert!(matches!(
        f.session
            .open_read(
                None,
                ReadLimits {
                    max_objects: usize::MAX,
                    ..ReadLimits::default()
                }
            )
            .await,
        Err(Error::Limit)
    ));
    let OpenRead::Pinned(read) = f
        .session
        .open_read(
            None,
            ReadLimits {
                timeout: std::time::Duration::from_millis(100),
                ..ReadLimits::default()
            },
        )
        .await
        .unwrap()
    else {
        panic!("initial head must pin")
    };
    tokio::time::sleep(std::time::Duration::from_millis(120)).await;
    assert_eq!(read.snapshot().await, Err(Error::Limit));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn read_session_rejects_replaced_epoch_without_rebinding() {
    use jj_store_object::{Condition, Key, ObjectStore as _, StagedUpload, WriteOutcome};
    use jj_store_service::reads::{Error, OpenRead, ReadLimits};
    let f = Fixture::new().await;
    let OpenRead::Pinned(read) = f
        .session
        .open_read(None, ReadLimits::default())
        .await
        .unwrap()
    else {
        panic!("initial head must pin")
    };
    let original = read.snapshot().await.unwrap().clone();
    // Fault injection into this disposable memory store models replacement by a
    // new authority epoch. This is not a supported mutation or promotion API.
    let key = Key::new(format!("repos/{REPO}/manifest.json")).unwrap();
    let version = f.store.head(&key).await.unwrap().version;
    let mut manifest = f.session.snapshot().await.unwrap().manifest().clone();
    manifest.epoch = m::EpochId::new([212; 16]);
    let bytes = serde_json::to_vec(&manifest).unwrap();
    let upload = StagedUpload::from_bytes(&bytes, 128 * 1024).await.unwrap();
    assert!(matches!(
        f.store
            .put(&key, Condition::Matches(version), &upload)
            .await
            .unwrap(),
        WriteOutcome::Applied(_)
    ));
    assert_eq!(read.snapshot().await, Err(Error::EpochChanged));
    assert_eq!(
        read.authorize_object(m::ObjectKey::Operation(original.operation_id))
            .await,
        Err(Error::EpochChanged)
    );
    let OpenRead::Pinned(new_read) = f
        .session
        .open_read(None, ReadLimits::default())
        .await
        .unwrap()
    else {
        panic!("explicit fresh selection required")
    };
    assert_ne!(
        new_read.snapshot().await.unwrap().authority.epoch,
        original.authority.epoch
    );
}

#[path = "native_lifecycle/graphs.rs"]
mod graph_acceptance;

#[path = "native_lifecycle/pagination.rs"]
mod pagination_acceptance;

#[path = "native_lifecycle/content.rs"]
mod content_acceptance;

#[path = "native_lifecycle/browsing.rs"]
mod browsing_acceptance;

#[path = "native_lifecycle/http_browsing.rs"]
mod http_browsing_acceptance;

#[path = "native_lifecycle/http_collaboration.rs"]
mod http_collaboration_acceptance;

#[path = "native_lifecycle/http_collaboration_checkpoints.rs"]
mod http_collaboration_checkpoint_acceptance;

#[path = "native_lifecycle/http_collaboration_reviews.rs"]
mod http_collaboration_review_acceptance;

#[path = "native_lifecycle/events.rs"]
mod event_acceptance;

#[path = "native_lifecycle/commands.rs"]
mod command_acceptance;

#[path = "native_lifecycle/mutations.rs"]
mod mutation_acceptance;

#[path = "native_lifecycle/http_mutations.rs"]
mod http_mutation_acceptance;

#[path = "native_lifecycle/reconciliation.rs"]
mod reconciliation_acceptance;

#[path = "native_lifecycle/abandon.rs"]
mod abandon_acceptance;

#[path = "native_lifecycle/adapters.rs"]
mod adapter_acceptance;

#[path = "native_lifecycle/http_work.rs"]
mod http_work_acceptance;

#[path = "native_lifecycle/http_threads_reactions.rs"]
mod http_threads_reactions_acceptance;
