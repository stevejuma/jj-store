//! Real loopback gRPC and native factories. This uses fault-injecting memory
//! storage; it does not replace the pinned RustFS deployment conformance lane.
use futures::{AsyncReadExt as _, StreamExt as _, TryStreamExt as _};
use jj_lib::{backend as b, object_id::ObjectId as _, op_store as o, repo::Repo as _};
use jj_store_cli::remote;
use jj_store_jj::{
    backend::BackendLimits,
    codec::{self, NativeCodec as _, NativeFile},
    objects::NativeObjects,
    operation_stores::OperationLimits,
    policy::Protection,
    roots,
};
use jj_store_model as m;
use jj_store_object::{
    Condition, Key, ObjectStore as _, StagedUpload, WriteOutcome,
    memory::{Fault, MemoryStore},
};
use jj_store_rpc::{
    client::{NativeClient, Options, RemoteObjects},
    server::{Limits as RpcLimits, NativeRpc},
    wire::{self, native_store_client::NativeStoreClient},
};
use jj_store_service::{auth::*, native::*, transport::NativeEndpoint};
use jj_store_wal::publication::*;
use std::{collections::BTreeSet, fs, sync::Arc, time::Duration};
use tonic::{Code, Request};
const REPO: m::RepositoryId = m::RepositoryId::new([1; 16]);
const ACTOR: m::PrincipalId = m::PrincipalId::new([2; 16]);
const MAX_BLOB: u64 = 8 * 1024 * 1024;

#[path = "native_rpc/inventory.rs"]
mod inventory;

struct Harness {
    store: Arc<MemoryStore>,
    objects: Arc<NativeObjects>,
    policy: TokenPolicy,
    admin: BearerToken,
    reader: BearerToken,
    address: String,
    server: Option<tokio::task::JoinHandle<()>>,
    shutdown: Option<tokio::sync::oneshot::Sender<()>>,
}
impl Drop for Harness {
    fn drop(&mut self) {
        if let Some(server) = &self.server {
            server.abort();
        }
    }
}
impl Harness {
    async fn new() -> Self {
        let store = Arc::new(MemoryStore::default());
        let objects = Arc::new(NativeObjects::new(store.clone(), REPO, MAX_BLOB).unwrap());
        let mut policy = TokenPolicy::new(Scope::Repository(REPO));
        let admin = policy.issue(ACTOR, Permission::Admin, None).unwrap();
        let reader = policy
            .issue(m::PrincipalId::new([3; 16]), Permission::Read, None)
            .unwrap();
        let mut operator = TokenPolicy::new(Scope::Operator);
        let token = operator
            .issue(ACTOR, Permission::CreateRepository, None)
            .unwrap();
        create_repository(
            store.clone(),
            objects.clone(),
            &operator,
            &token,
            &policy,
            m::EpochId::new([4; 16]),
            &SystemClock,
            Limits::default(),
        )
        .await
        .unwrap();
        let mut fixture = Self {
            store,
            objects,
            policy,
            admin,
            reader,
            address: String::new(),
            server: None,
            shutdown: None,
        };
        fixture
            .start(Protection::default(), RpcLimits::default())
            .await;
        fixture
    }
    async fn stop(&mut self) {
        if let Some(shutdown) = self.shutdown.take() {
            let _ = shutdown.send(());
        }
        if let Some(server) = self.server.take() {
            tokio::time::timeout(Duration::from_secs(5), server)
                .await
                .unwrap()
                .unwrap();
        }
    }
    async fn start(&mut self, protection: Protection, limits: RpcLimits) {
        self.start_on_store(self.store.clone(), protection, limits)
            .await;
    }
    async fn start_on_store(
        &mut self,
        store: Arc<dyn jj_store_object::ObjectStore>,
        protection: Protection,
        limits: RpcLimits,
    ) {
        self.start_with_session_limits(store, protection, limits, SessionLimits::default())
            .await;
    }
    async fn start_with_session_limits(
        &mut self,
        store: Arc<dyn jj_store_object::ObjectStore>,
        protection: Protection,
        limits: RpcLimits,
        session_limits: SessionLimits,
    ) {
        let endpoint = NativeEndpoint::new(
            REPO,
            store,
            Arc::new(SystemClock),
            protection,
            session_limits,
            MAX_BLOB,
        )
        .unwrap();
        let server = NativeRpc::new(vec![endpoint], limits).unwrap();
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        self.address = format!("http://{}", listener.local_addr().unwrap());
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.shutdown = Some(tx);
        self.server = Some(tokio::spawn(async move {
            server
                .serve(listener, None, true, async {
                    let _ = rx.await;
                })
                .await
                .unwrap();
        }));
    }
    async fn client(&self, token: &BearerToken) -> NativeClient {
        NativeClient::connect(
            &self.address,
            REPO,
            token.clone(),
            Options {
                insecure_loopback: true,
                ..Options::default()
            },
        )
        .await
        .unwrap()
    }
    fn local(&self) -> NativeSession {
        NativeSession::new(
            self.store.clone(),
            self.objects.clone(),
            self.admin.clone(),
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
        )
        .unwrap()
    }
    async fn raw(&self) -> NativeStoreClient<tonic::transport::Channel> {
        NativeStoreClient::connect(self.address.clone())
            .await
            .unwrap()
    }
    async fn revoke_reader(&mut self) {
        self.policy.revoke(self.reader.id());
        let session = self.local();
        let base = session.snapshot().await.unwrap().manifest().snapshot();
        session
            .publish_request(&RequestPolicy::request(base, self.policy.encode().unwrap()))
            .await
            .unwrap();
    }
}
struct RequestPolicy;
impl RequestPolicy {
    fn request(base: Snapshot, policy: Vec<u8>) -> jj_store_wal::publication::Request {
        jj_store_wal::publication::Request {
            principal: ACTOR,
            request_id: m::RequestId::new([11; 16]),
            publication_id: m::PublicationId::new([12; 16]),
            transition: Some(Transition::Strict {
                expected: base.clone(),
                heads: base.heads.clone(),
            }),
            settings: Some(policy),
            result: vec![],
            kind: SemanticKind::SettingsChanged,
        }
    }
}
fn req<T>(token: &BearerToken, value: T) -> Request<T> {
    let mut request = Request::new(value);
    let mut auth = format!("Bearer {}", token.expose().as_str())
        .parse::<tonic::metadata::MetadataValue<_>>()
        .unwrap();
    auth.set_sensitive(true);
    request.metadata_mut().insert("authorization", auth);
    request
}
fn oid(id: &o::OperationId) -> m::OperationId {
    m::OperationId::from_slice(id.as_bytes()).unwrap()
}
fn key(value: m::ObjectKey) -> wire::ObjectId {
    wire::ObjectId {
        kind: value.kind() as u32,
        id: value.as_bytes().to_vec(),
    }
}
fn settings() -> jj_lib::settings::UserSettings {
    jj_lib::settings::UserSettings::from_config(jj_lib::config::StackedConfig::with_defaults())
        .unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn remote_native_factories_preserve_workspace_and_history_after_reconnect() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let mut f = Harness::new().await;
            let client = f.client(&f.admin).await;
            let stores = remote::stores(
                client,
                MAX_BLOB,
                BackendLimits::default(),
                OperationLimits::default(),
            )
            .unwrap();
            let cache = tempfile::tempdir().unwrap();
            let working = tempfile::tempdir().unwrap();
            let settings = settings();
            stores
                .initialize_cache(&settings, cache.path())
                .await
                .unwrap();
            let (mut workspace, repo) = stores
                .attach_workspace(
                    &settings,
                    working.path(),
                    cache.path(),
                    "remote-client-a".into(),
                )
                .await
                .unwrap();
            let bytes: Vec<u8> = (0..2 * 1024 * 1024).map(|i| i as u8).collect();
            fs::write(working.path().join("binary.dat"), &bytes).unwrap();
            let name = workspace.workspace_name().to_owned();
            let old = repo
                .store()
                .get_commit(repo.view().get_wc_commit_id(&name).unwrap())
                .unwrap();
            let mut locked = workspace.start_working_copy_mutation().await.unwrap();
            let (tree, _) = locked
                .locked_wc()
                .snapshot(&jj_lib::working_copy::SnapshotOptions {
                    base_ignores: jj_lib::gitignore::GitIgnoreFile::empty(),
                    progress: None,
                    start_tracking_matcher: &jj_lib::matchers::EverythingMatcher,
                    force_tracking_matcher: &jj_lib::matchers::NothingMatcher,
                    max_new_file_size: MAX_BLOB,
                })
                .await
                .unwrap();
            let mut tx = repo.start_transaction();
            let captured = tx
                .repo_mut()
                .rewrite_commit(&old)
                .set_tree(tree)
                .write()
                .await
                .unwrap();
            tx.repo_mut().rebase_descendants().await.unwrap();
            let repo = tx.commit("snapshot over RPC").await.unwrap();
            locked.finish(repo.op_id().clone()).await.unwrap();
            let mut tx = repo.start_transaction();
            let described = tx
                .repo_mut()
                .rewrite_commit(&captured)
                .set_description("native RPC lifecycle")
                .write()
                .await
                .unwrap();
            tx.repo_mut().rebase_descendants().await.unwrap();
            let repo = tx.commit("describe over RPC").await.unwrap();
            let operation = oid(repo.op_id());
            let commit = described.id().clone();
            assert!(!working.path().join(".git").exists());
            drop(workspace);
            drop(repo);
            drop(stores);
            f.stop().await;
            f.start(Protection::default(), RpcLimits::default()).await;
            let cold = remote::stores(
                f.client(&f.admin).await,
                MAX_BLOB,
                BackendLimits::default(),
                OperationLimits::default(),
            )
            .unwrap();
            let clean = tempfile::tempdir().unwrap();
            let loader = cold
                .initialize_cache(&settings, clean.path())
                .await
                .unwrap();
            let repo = cold.load_at(&loader, operation).await.unwrap();
            let recovered = repo.store().get_commit(&commit).unwrap();
            assert_eq!(recovered.description(), "native RPC lifecycle");
            let path = jj_lib::repo_path::RepoPathBuf::from_internal_string("binary.dat").unwrap();
            let value = recovered.tree().path_value(&path).await.unwrap();
            let b::TreeValue::File { id, .. } = value.as_resolved().unwrap().as_ref().unwrap()
            else {
                panic!("file");
            };
            let mut reader = repo.store().read_file(&path, id).await.unwrap();
            let mut actual = Vec::new();
            reader.read_to_end(&mut actual).await.unwrap();
            assert_eq!(actual, bytes);
            let history: Vec<_> = jj_lib::op_walk::walk_ancestors(&[repo.operation().clone()])
                .take(10)
                .try_collect()
                .await
                .unwrap();
            assert!(history.len() >= 4);
            let evolution: Vec<_> =
                jj_lib::evolution::walk_predecessors(repo.as_ref(), std::slice::from_ref(&commit))
                    .take(10)
                    .try_collect()
                    .await
                    .unwrap();
            assert!(evolution.len() >= 3);
            assert_eq!(
                f.client(&f.admin).await.heads().await.unwrap(),
                BTreeSet::from([operation])
            );
            f.stop().await;
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn rpc_authorization_and_native_validation_precede_immutable_storage() {
    let mut f = Harness::new().await;
    let client = f.client(&f.admin).await;
    let reader = f.client(&f.reader).await;
    let value = NativeFile((0..300_000).map(|i| i as u8).collect());
    let (object, bytes) = codec::encode(&value, MAX_BLOB).unwrap();
    let spool = StagedUpload::from_bytes(&bytes, MAX_BLOB).await.unwrap();
    assert_eq!(
        reader.upload(object, &spool).await.unwrap_err().code(),
        Code::PermissionDenied
    );
    assert_eq!(
        client.head(object).await.unwrap_err().code(),
        Code::NotFound
    );
    let mut corrupted = bytes.clone();
    *corrupted.last_mut().unwrap() ^= 1;
    assert_eq!(
        client
            .upload(
                object,
                &StagedUpload::from_bytes(&corrupted, MAX_BLOB)
                    .await
                    .unwrap()
            )
            .await
            .unwrap_err()
            .code(),
        Code::InvalidArgument
    );
    let forged = m::envelope::encode(object, b"different native identity", MAX_BLOB).unwrap();
    assert_eq!(
        client
            .upload(
                object,
                &StagedUpload::from_bytes(&forged, MAX_BLOB).await.unwrap()
            )
            .await
            .unwrap_err()
            .code(),
        Code::InvalidArgument
    );
    assert_eq!(
        client.head(object).await.unwrap_err().code(),
        Code::NotFound
    );
    assert!(matches!(
        client.upload(object, &spool).await.unwrap(),
        WriteOutcome::Applied(_)
    ));
    assert_eq!(
        client.upload(object, &spool).await.unwrap(),
        WriteOutcome::PreconditionFailed
    );
    let downloaded = reader
        .read(object)
        .await
        .unwrap()
        .read_bounded(MAX_BLOB)
        .await
        .unwrap();
    assert_eq!(downloaded, bytes);
    let transport = RemoteObjects(client.clone());
    let path = Key::new(object.storage_path(REPO)).unwrap();
    let range = transport
        .get(&path, Some(jj_store_object::ByteRange::new(3, 40).unwrap()))
        .await
        .unwrap()
        .read_bounded(37)
        .await
        .unwrap();
    assert_eq!(range, bytes[3..40]);
    assert!(
        transport
            .put(
                &path,
                Condition::Matches(jj_store_object::VersionToken::new("fake").unwrap()),
                &spool
            )
            .await
            .is_err()
    );
    assert!(
        transport
            .head(&Key::new(format!("repos/{REPO}/manifest.json")).unwrap())
            .await
            .is_err()
    );
    assert!(
        transport
            .head(&Key::new(object.storage_path(m::RepositoryId::new([99; 16]))).unwrap())
            .await
            .is_err()
    );
    let other = NativeClient::connect(
        &f.address,
        m::RepositoryId::new([99; 16]),
        f.admin.clone(),
        Options {
            insecure_loopback: true,
            ..Options::default()
        },
    )
    .await
    .unwrap();
    assert_eq!(
        other.read(object).await.unwrap_err().code(),
        Code::PermissionDenied
    );
    let mut wrong_policy = TokenPolicy::new(Scope::Repository(REPO));
    let wrong = wrong_policy.issue(ACTOR, Permission::Admin, None).unwrap();
    assert_eq!(
        f.client(&wrong)
            .await
            .authorize(false)
            .await
            .unwrap_err()
            .code(),
        Code::PermissionDenied
    );
    f.revoke_reader().await;
    assert_eq!(
        reader.read(object).await.unwrap_err().code(),
        Code::PermissionDenied
    );
    assert_eq!(
        reader.inventory(false, 100).await.unwrap_err().code(),
        Code::PermissionDenied
    );
    assert_eq!(
        reader.heads().await.unwrap_err().code(),
        Code::PermissionDenied
    );
    f.stop().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn rpc_guarded_publication_preserves_receipts_siblings_and_protection() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let mut f = Harness::new().await;
            let client = f.client(&f.admin).await;
            let stores = remote::stores(
                client.clone(),
                MAX_BLOB,
                BackendLimits::default(),
                OperationLimits::default(),
            )
            .unwrap();
            let cache = tempfile::tempdir().unwrap();
            let working = tempfile::tempdir().unwrap();
            stores
                .initialize_cache(&settings(), cache.path())
                .await
                .unwrap();
            let (_, repo) = stores
                .attach_workspace(
                    &settings(),
                    working.path(),
                    cache.path(),
                    "rpc-policy".into(),
                )
                .await
                .unwrap();
            let op = repo.operation().store_operation();
            let old: BTreeSet<_> = op.parents.iter().map(oid).collect();
            let new = oid(repo.op_id());
            let before = f.local().snapshot().await.unwrap().manifest().snapshot();
            client.publish(old.clone(), new).await.unwrap();
            assert_eq!(
                f.local().snapshot().await.unwrap().manifest().snapshot(),
                before
            );
            let protected = repo
                .view()
                .get_wc_commit_id(jj_lib::ref_name::WorkspaceName::new("rpc-policy"))
                .unwrap();
            let commit = repo.store().get_commit(protected).unwrap();
            f.stop().await;
            f.start(
                Protection {
                    commits: BTreeSet::from([
                        m::CommitId::from_slice(protected.as_bytes()).unwrap()
                    ]),
                    bookmarks: BTreeSet::new(),
                },
                RpcLimits::default(),
            )
            .await;
            let client = f.client(&f.admin).await;
            client.publish(old, new).await.unwrap();
            let mut forged = commit.store_commit().as_ref().clone();
            forged.description = "attempt unrecorded protected rewrite".into();
            forged.predecessors.clear();
            let (commit_key, _) = f.objects.put(&forged).await.unwrap();
            let mut view = repo.view().store_view().clone();
            view.head_ids
                .insert(b::CommitId::from_bytes(commit_key.as_bytes()));
            let (view_key, _) = f.objects.put(&view).await.unwrap();
            let mut malicious = op.clone();
            malicious.parents = vec![repo.op_id().clone()];
            malicious.view_id = o::ViewId::from_bytes(view_key.as_bytes());
            malicious.commit_predecessors = None;
            let (operation_key, _) = f.objects.put(&malicious).await.unwrap();
            assert_eq!(
                client
                    .publish(
                        BTreeSet::from([new]),
                        m::OperationId::from_slice(operation_key.as_bytes()).unwrap()
                    )
                    .await
                    .unwrap_err()
                    .code(),
                Code::PermissionDenied
            );
            assert_eq!(
                f.local().snapshot().await.unwrap().manifest().snapshot(),
                before
            );
            // Two concurrent operations based on the same parent both remain heads.
            let mut siblings = Vec::new();
            for label in ["first sibling", "second sibling"] {
                let mut sibling = op.clone();
                sibling.parents = vec![repo.op_id().clone()];
                sibling.metadata.description = label.into();
                let (key, bytes) = codec::encode(&sibling, MAX_BLOB).unwrap();
                client
                    .upload(
                        key,
                        &StagedUpload::from_bytes(&bytes, MAX_BLOB).await.unwrap(),
                    )
                    .await
                    .unwrap();
                siblings.push(m::OperationId::from_slice(key.as_bytes()).unwrap());
            }
            let (left, right) = tokio::join!(
                client.publish(BTreeSet::from([new]), siblings[0]),
                client.publish(BTreeSet::from([new]), siblings[1])
            );
            left.unwrap();
            right.unwrap();
            assert_eq!(
                client.heads().await.unwrap(),
                siblings.into_iter().collect()
            );
            f.stop().await;
        })
        .await;
}

struct UploadAdmission {
    inner: Arc<MemoryStore>,
    armed: std::sync::atomic::AtomicBool,
    admitted: tokio::sync::Notify,
}
#[async_trait::async_trait]
impl jj_store_object::ObjectStore for UploadAdmission {
    async fn head(&self, key: &Key) -> jj_store_object::Result<jj_store_object::Metadata> {
        self.inner.head(key).await
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> jj_store_object::Result<WriteOutcome> {
        self.inner.put(key, condition, upload).await
    }
    async fn get(
        &self,
        key: &Key,
        range: Option<jj_store_object::ByteRange>,
    ) -> jj_store_object::Result<jj_store_object::Download> {
        let result = self.inner.get(key, range).await?;
        if key.as_str().ends_with("/manifest.json")
            && self.armed.swap(false, std::sync::atomic::Ordering::SeqCst)
        {
            self.admitted.notify_one();
        }
        Ok(result)
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn rpc_stream_protocol_deadlines_and_permits_are_bounded() {
    let mut f = Harness::new().await;
    f.stop().await;
    let admission = Arc::new(UploadAdmission {
        inner: f.store.clone(),
        armed: Default::default(),
        admitted: Default::default(),
    });
    f.start_on_store(
        admission.clone(),
        Protection::default(),
        RpcLimits {
            concurrent: 1,
            timeout: Duration::from_millis(400),
            object_bytes: 1024,
        },
    )
    .await;
    let object = NativeFile(vec![3; 4]).key();
    let start = || wire::UploadChunk {
        part: Some(wire::upload_chunk::Part::Start(wire::UploadStart {
            repository: REPO.as_bytes().to_vec(),
            object: Some(key(object)),
            length: 200,
        })),
    };
    let mut raw = f.raw().await;
    for messages in [
        vec![],
        vec![start()],
        vec![start(), start()],
        vec![
            start(),
            wire::UploadChunk {
                part: Some(wire::upload_chunk::Part::Data(vec![0; 201])),
            },
        ],
    ] {
        let result = raw
            .write_object(req(&f.admin, tokio_stream::iter(messages)))
            .await;
        assert_eq!(result.unwrap_err().code(), Code::InvalidArgument);
    }
    let (tx, rx) = tokio::sync::mpsc::channel(1);
    tx.send(start()).await.unwrap();
    let request = req(&f.admin, tokio_stream::wrappers::ReceiverStream::new(rx));
    let client = f.client(&f.admin).await;
    admission
        .armed
        .store(true, std::sync::atomic::Ordering::SeqCst);
    let task = tokio::spawn(async move { raw.write_object(request).await });
    // Wait for a server-side authorization read under the upload's permit.
    // Polling heads first can win that one permit and reject the upload itself.
    tokio::time::timeout(Duration::from_secs(2), admission.admitted.notified())
        .await
        .unwrap();
    assert_eq!(
        client.heads().await.unwrap_err().code(),
        Code::ResourceExhausted
    );
    assert_eq!(
        task.await.unwrap().unwrap_err().code(),
        Code::DeadlineExceeded
    );
    drop(tx);
    assert_eq!(
        client.heads().await.unwrap(),
        BTreeSet::from([roots::ROOT_OPERATION])
    );
    assert_eq!(
        client.head(object).await.unwrap_err().code(),
        Code::NotFound
    );
    assert_eq!(
        client.inventory(false, 0).await.unwrap_err().code(),
        Code::ResourceExhausted
    );
    f.stop().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn rpc_requires_credentials_and_explicit_secure_transport_configuration() {
    let mut f = Harness::new().await;
    assert!(
        NativeClient::connect(&f.address, REPO, f.admin.clone(), Options::default())
            .await
            .is_err()
    );
    assert!(
        NativeClient::connect(
            "http://example.invalid",
            REPO,
            f.admin.clone(),
            Options {
                insecure_loopback: true,
                ..Options::default()
            }
        )
        .await
        .is_err()
    );
    let mut raw = f.raw().await;
    let body = wire::Repository {
        repository: REPO.as_bytes().to_vec(),
    };
    assert_eq!(
        raw.heads(body.clone()).await.unwrap_err().code(),
        Code::Unauthenticated
    );
    let mut duplicate = req(&f.admin, body);
    duplicate
        .metadata_mut()
        .append("authorization", "Bearer invalid".parse().unwrap());
    assert_eq!(
        raw.heads(duplicate).await.unwrap_err().code(),
        Code::Unauthenticated
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    assert!(
        NativeRpc::new(vec![], RpcLimits::default())
            .unwrap()
            .serve(listener, None, false, async {})
            .await
            .is_err()
    );
    // Unknown immutable upload acknowledgements stay unknown or are resolved by
    // the native accessor's subsequent identical-content check.
    let client = f.client(&f.admin).await;
    let native = NativeObjects::new(Arc::new(RemoteObjects(client)), REPO, MAX_BLOB).unwrap();
    f.store.inject(Fault::AfterWrite);
    assert_eq!(
        native
            .put_file(b"lost acknowledgement".as_slice())
            .await
            .unwrap()
            .1,
        jj_store_jj::objects::UploadStatus::AlreadyPresent
    );
    f.store.inject(Fault::BeforeWrite);
    assert_eq!(
        native.put_file(b"before apply".as_slice()).await.unwrap().1,
        jj_store_jj::objects::UploadStatus::Unknown
    );
    f.stop().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn rpc_tls_checks_trust_and_server_hostname() {
    let mut f = Harness::new().await;
    f.stop().await;
    let certificate = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let pem = certificate.cert.pem();
    let identity =
        tonic::transport::Identity::from_pem(&pem, certificate.signing_key.serialize_pem());
    let endpoint = NativeEndpoint::new(
        REPO,
        f.store.clone(),
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
        MAX_BLOB,
    )
    .unwrap();
    let server = NativeRpc::new(vec![endpoint], RpcLimits::default()).unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let (tx, rx) = tokio::sync::oneshot::channel();
    f.shutdown = Some(tx);
    f.server = Some(tokio::spawn(async move {
        server
            .serve(listener, Some(identity), false, async {
                let _ = rx.await;
            })
            .await
            .unwrap();
    }));
    let trusted = Options {
        ca_certificate_pem: Some(pem.into_bytes()),
        ..Options::default()
    };
    let url = format!("https://localhost:{port}");
    assert!(
        NativeClient::connect(&url, REPO, f.admin.clone(), Options::default())
            .await
            .is_err()
    );
    assert!(
        NativeClient::connect(
            &format!("https://127.0.0.1:{port}"),
            REPO,
            f.admin.clone(),
            trusted.clone()
        )
        .await
        .is_err()
    );
    let client = NativeClient::connect(&url, REPO, f.admin.clone(), trusted)
        .await
        .unwrap();
    assert_eq!(
        client.heads().await.unwrap(),
        BTreeSet::from([roots::ROOT_OPERATION])
    );
    f.stop().await;
}

struct PausedObjectRead {
    inner: Arc<MemoryStore>,
    key: Key,
    release: Arc<tokio::sync::Notify>,
}
#[async_trait::async_trait]
impl jj_store_object::ObjectStore for PausedObjectRead {
    async fn head(&self, key: &Key) -> jj_store_object::Result<jj_store_object::Metadata> {
        self.inner.head(key).await
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> jj_store_object::Result<WriteOutcome> {
        self.inner.put(key, condition, upload).await
    }
    async fn get(
        &self,
        key: &Key,
        range: Option<jj_store_object::ByteRange>,
    ) -> jj_store_object::Result<jj_store_object::Download> {
        let download = self.inner.get(key, range).await?;
        if key != &self.key {
            return Ok(download);
        }
        let release = self.release.clone();
        let (reader, mut writer) = tokio::io::duplex(64 * 1024);
        tokio::spawn(async move {
            let _ = tokio::time::timeout(Duration::from_secs(5), async {
                release.notified().await;
                let mut source = download.reader;
                let _ = tokio::io::copy(&mut source, &mut writer).await;
            })
            .await;
        });
        Ok(jj_store_object::Download {
            metadata: download.metadata,
            reader: Box::pin(reader),
        })
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn rpc_download_rechecks_revocation_and_releases_permits_on_cancellation() {
    let mut f = Harness::new().await;
    let (id, _) = f
        .objects
        .put_file(b"revoked before first content chunk".as_slice())
        .await
        .unwrap();
    let object = m::ObjectKey::File(id);
    let release = Arc::new(tokio::sync::Notify::new());
    f.stop().await;
    f.start_on_store(
        Arc::new(PausedObjectRead {
            inner: f.store.clone(),
            key: Key::new(object.storage_path(REPO)).unwrap(),
            release: release.clone(),
        }),
        Protection::default(),
        RpcLimits {
            concurrent: 1,
            ..RpcLimits::default()
        },
    )
    .await;
    let mut raw = f.raw().await;
    let mut stream = raw
        .read_object(req(
            &f.reader,
            wire::ObjectRequest {
                repository: REPO.as_bytes().to_vec(),
                object: Some(key(object)),
            },
        ))
        .await
        .unwrap()
        .into_inner();
    assert!(matches!(
        stream.message().await.unwrap().unwrap().part,
        Some(wire::object_chunk::Part::Metadata(_))
    ));
    f.revoke_reader().await;
    release.notify_one();
    assert_eq!(
        stream.message().await.unwrap_err().code(),
        Code::PermissionDenied
    );
    drop(stream);
    // A second reader is canceled while its storage read is paused. The permit
    // must be released without waiting for the storage deadline.
    let mut stream = raw
        .read_object(req(
            &f.admin,
            wire::ObjectRequest {
                repository: REPO.as_bytes().to_vec(),
                object: Some(key(object)),
            },
        ))
        .await
        .unwrap()
        .into_inner();
    stream.message().await.unwrap().unwrap();
    drop(stream);
    let client = f.client(&f.admin).await;
    tokio::time::timeout(Duration::from_secs(2), async {
        loop {
            match client.heads().await {
                Ok(heads) => {
                    assert_eq!(heads, BTreeSet::from([roots::ROOT_OPERATION]));
                    break;
                }
                Err(error) => assert_eq!(error.code(), Code::ResourceExhausted),
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    release.notify_one();
    f.stop().await;
}
