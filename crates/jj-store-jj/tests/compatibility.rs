//! M0 compatibility probes; the filesystem stores and RPC fixture are not
//! production authority, codecs, authentication, or publication implementations.
use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use futures::{StreamExt as _, TryStreamExt as _};
use jj_lib::backend::{Backend as _, CommitId, Tree};
use jj_lib::config::StackedConfig;
use jj_lib::content_hash::blake2b_hash;
use jj_lib::object_id::ObjectId as _;
use jj_lib::op_heads_store::{OpHeadsStore, OpHeadsStoreError, OpHeadsStoreLock};
use jj_lib::op_store::{OpStore as _, Operation, OperationId, RootOperationData, View};
use jj_lib::repo::{Repo as _, StoreFactories};
use jj_lib::settings::UserSettings;
use jj_lib::simple_backend::SimpleBackend;
use jj_lib::simple_op_store::SimpleOpStore;
use jj_lib::workspace::Workspace;
use tokio::sync::Mutex;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::{Request, Response, Status};

mod wire {
    tonic::include_proto!("jj_store.probe");
}
use wire::publication_probe_client::PublicationProbeClient;
use wire::publication_probe_server::{PublicationProbe, PublicationProbeServer};
use wire::{Empty, Heads, Transition};

const EMPTY_TREE: &str = "482ae5a29fbe856c7272f2071b8b0f0359ee2d89ff392b8a900643fbd0836eccd067b8bf41909e206c90d45d6e7d8b6686b93ecaee5fe1a9060d87b672101310";

#[tokio::test]
async fn pinned_identity_roots_and_predecessor_presence() {
    let dir = tempfile::tempdir().unwrap();
    let backend = SimpleBackend::init(dir.path());
    assert_eq!(backend.commit_id_length(), 64);
    assert_eq!(backend.change_id_length(), 16);
    assert_eq!(backend.root_commit_id().as_bytes(), &[0; 64]);
    assert_eq!(backend.root_change_id().as_bytes(), &[0; 16]);
    assert_eq!(backend.empty_tree_id().hex(), EMPTY_TREE);
    assert_eq!(hex(blake2b_hash(&Tree::default()).as_slice()), EMPTY_TREE);
    let ops = tempfile::tempdir().unwrap();
    let store = SimpleOpStore::init(
        ops.path(),
        RootOperationData {
            root_commit_id: backend.root_commit_id().clone(),
        },
    )
    .unwrap();
    assert_eq!(store.root_operation_id().as_bytes(), &[0; 64]);
    let mut view = View::make_root(backend.root_commit_id().clone());
    view.wc_commit_ids
        .insert("laptop".into(), CommitId::from_bytes(&[1; 64]));
    let view_id = store.write_view(&view).await.unwrap();
    assert_eq!(view_id.as_bytes(), blake2b_hash(&view).as_slice());
    assert_eq!(store.read_view(&view_id).await.unwrap(), view);
    let mut operation = Operation::make_root(view_id.clone());
    operation.parents = vec![store.root_operation_id().clone()];
    operation.metadata.description = "native identity fixture".into();
    operation.metadata.time.start.tz_offset = -210;
    operation.metadata.time.end.tz_offset = 345;
    operation.commit_predecessors = None;
    let absent = store.write_operation(&operation).await.unwrap();
    assert_eq!(store.read_operation(&absent).await.unwrap(), operation);
    operation.commit_predecessors = Some(BTreeMap::new());
    let empty = store.write_operation(&operation).await.unwrap();
    assert_ne!(absent, empty);
    assert_eq!(store.read_operation(&empty).await.unwrap(), operation);
    operation.commit_predecessors = Some(BTreeMap::from([
        (
            CommitId::from_bytes(&[2; 64]),
            vec![CommitId::from_bytes(&[1; 64])],
        ),
        (
            CommitId::from_bytes(&[3; 64]),
            vec![CommitId::from_bytes(&[2; 64])],
        ),
    ]));
    let intermediate = store.write_operation(&operation).await.unwrap();
    assert_eq!(
        store.read_operation(&intermediate).await.unwrap(),
        operation
    );
    let observed = serde_json::json!({"view":view_id.hex(),"absent_predecessors":absent.hex(),"empty_predecessors":empty.hex(),"intermediate_predecessors":intermediate.hex()});
    let golden: serde_json::Value = serde_json::from_str(include_str!(
        "../../../fixtures/native-profile/m0-hashes.json"
    ))
    .unwrap();
    assert_eq!(observed, golden);
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[tokio::test]
async fn explicit_bounded_reads_preserve_divergent_heads() {
    let local = tokio::task::LocalSet::new();
    local
        .run_until(async {
            let dir = tempfile::tempdir().unwrap();
            let settings =
                UserSettings::from_config_and_home_dir(StackedConfig::with_defaults(), None)
                    .unwrap();
            let (workspace, repo) = Workspace::init_with_backend(
                &settings,
                dir.path(),
                &|_, path| Ok(Box::new(SimpleBackend::init(path))),
                jj_lib::signing_factory::signer_from_settings(&settings).unwrap(),
            )
            .await
            .unwrap();
            let mut first = repo.start_transaction();
            first.set_attribute("probe".into(), "first".into());
            let first = first.commit("first divergent operation").await.unwrap();
            let mut second = repo.start_transaction();
            second.set_attribute("probe".into(), "second".into());
            second.commit("second divergent operation").await.unwrap();
            let loader = workspace.repo_loader();
            let mut before = loader.op_heads_store().get_op_heads().await.unwrap();
            before.sort();
            assert_eq!(before.len(), 2);
            let query = async {
                let operation = loader.load_operation(first.operation().id()).await.unwrap();
                let pinned = loader.load_at(&operation).await.unwrap();
                assert_eq!(pinned.operation().id(), first.operation().id());
                let heads = vec![operation];
                let page: Vec<_> = jj_lib::op_walk::walk_ancestors(&heads)
                    .take(1)
                    .try_collect()
                    .await
                    .unwrap();
                assert_eq!(page.len(), 1);
                let starts = vec![repo.store().root_commit_id().clone()];
                let evolution: Vec<_> =
                    jj_lib::evolution::walk_predecessors(pinned.as_ref(), &starts)
                        .take(2)
                        .try_collect()
                        .await
                        .unwrap();
                assert!(!evolution.is_empty());
            };
            tokio::time::timeout(Duration::from_secs(5), query)
                .await
                .unwrap();
            let mut after = loader.op_heads_store().get_op_heads().await.unwrap();
            after.sort();
            assert_eq!(before, after, "read session reconciled native heads");
        })
        .await;
}

#[derive(Debug, Clone)]
struct Context {
    // A test identity in a token, never caller-authored operation metadata.
    token: &'static str,
    request: &'static str,
    expected: Vec<u8>,
}

#[derive(Debug)]
struct RemoteHeads {
    client: PublicationProbeClient<tonic::transport::Channel>,
    context: Arc<Context>,
}
struct NoLock;
impl OpHeadsStoreLock for NoLock {}

#[async_trait]
impl OpHeadsStore for RemoteHeads {
    fn name(&self) -> &str {
        "m0-probe-only"
    }
    async fn update_op_heads(
        &self,
        old_ids: &[OperationId],
        new_id: &OperationId,
    ) -> Result<(), OpHeadsStoreError> {
        let mut request = Request::new(Transition {
            old_ids: old_ids.iter().map(|id| id.to_bytes()).collect(),
            new_id: new_id.to_bytes(),
            expected_head: self.context.expected.clone(),
        });
        request
            .metadata_mut()
            .insert("authorization", self.context.token.parse().unwrap());
        // One invocation can publish multiple native operations. Bind each
        // publication to its operation instead of reusing one idempotency key.
        let key = format!("{}:{}", self.context.request, new_id.hex());
        request
            .metadata_mut()
            .insert("x-request-id", key.parse().unwrap());
        self.client
            .clone()
            .update(request)
            .await
            .map_err(|source| OpHeadsStoreError::Write {
                new_op_id: new_id.clone(),
                source: Box::new(source),
            })?;
        Ok(())
    }
    async fn get_op_heads(&self) -> Result<Vec<OperationId>, OpHeadsStoreError> {
        let mut request = Request::new(Empty {});
        request
            .metadata_mut()
            .insert("authorization", self.context.token.parse().unwrap());
        let result = self
            .client
            .clone()
            .read(request)
            .await
            .map_err(|source| OpHeadsStoreError::Read(Box::new(source)))?;
        Ok(result
            .into_inner()
            .ids
            .into_iter()
            .map(OperationId::new)
            .collect())
    }
    async fn lock(&self) -> Result<Box<dyn OpHeadsStoreLock + '_>, OpHeadsStoreError> {
        Ok(Box::new(NoLock))
    }
}

#[derive(Debug, Default)]
struct ProbeState {
    heads: Vec<Vec<u8>>,
    seen: Vec<(String, String, Vec<u8>)>,
}
#[derive(Clone)]
struct Probe(Arc<Mutex<ProbeState>>);

fn principal<T>(request: &Request<T>) -> Result<&'static str, Status> {
    match request
        .metadata()
        .get("authorization")
        .and_then(|s| s.to_str().ok())
    {
        Some("Bearer probe-a") => Ok("actor-a"),
        Some("Bearer probe-b") => Ok("actor-b"),
        _ => Err(Status::unauthenticated("test token required")),
    }
}

#[tonic::async_trait]
impl PublicationProbe for Probe {
    async fn update(&self, request: Request<Transition>) -> Result<Response<Heads>, Status> {
        let actor = principal(&request)?.to_owned();
        let key = request
            .metadata()
            .get("x-request-id")
            .ok_or_else(|| Status::invalid_argument("request ID required"))?
            .to_str()
            .map_err(|_| Status::invalid_argument("request ID encoding"))?
            .to_owned();
        let transition = request.into_inner();
        let mut state = self.0.lock().await;
        state.seen.push((actor, key, transition.expected_head));
        state.heads.retain(|id| !transition.old_ids.contains(id));
        if !state.heads.contains(&transition.new_id) {
            state.heads.push(transition.new_id);
        }
        Ok(Response::new(Heads {
            ids: state.heads.clone(),
        }))
    }
    async fn read(&self, request: Request<Empty>) -> Result<Response<Heads>, Status> {
        principal(&request)?;
        Ok(Response::new(Heads {
            ids: self.0.lock().await.heads.clone(),
        }))
    }
}

#[tokio::test]
async fn native_factory_preserves_request_context_across_rpc() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let state = Arc::new(Mutex::new(ProbeState {
        heads: vec![vec![0; 64]],
        seen: vec![],
    }));
    let service = PublicationProbeServer::new(Probe(state.clone()));
    let (shutdown, stop) = tokio::sync::oneshot::channel();
    let server = tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(service)
            .serve_with_incoming_shutdown(TcpListenerStream::new(listener), async {
                let _ = stop.await;
            })
            .await
            .unwrap();
    });
    let channel = tonic::transport::Channel::from_shared(format!("http://{address}"))
        .unwrap()
        .connect()
        .await
        .unwrap();
    let settings =
        UserSettings::from_config_and_home_dir(StackedConfig::with_defaults(), None).unwrap();
    let mut handles = Vec::new();
    let dirs = [tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap()];
    for (i, token) in ["Bearer probe-a", "Bearer probe-b"].into_iter().enumerate() {
        let context = Arc::new(Context {
            token,
            request: if i == 0 { "request-a" } else { "request-b" },
            expected: vec![i as u8; 64],
        });
        let channel = channel.clone();
        let mut factories = StoreFactories::empty();
        factories.add_op_heads_store(
            "m0-probe-only",
            Box::new(move |_, _| {
                Ok(Box::new(RemoteHeads {
                    client: PublicationProbeClient::new(channel.clone()),
                    context: context.clone(),
                }))
            }),
        );
        std::fs::write(dirs[i].path().join("type"), "m0-probe-only").unwrap();
        handles.push(
            factories
                .load_op_heads_store(&settings, dirs[i].path())
                .unwrap(),
        );
    }
    let old = OperationId::from_bytes(&[0; 64]);
    let a = OperationId::from_bytes(&[1; 64]);
    let b = OperationId::from_bytes(&[2; 64]);
    let old_ids = [old];
    let (left, right) = tokio::join!(
        handles[0].update_op_heads(&old_ids, &a),
        handles[1].update_op_heads(&old_ids, &b)
    );
    left.unwrap();
    right.unwrap();
    let mut heads = handles[0].get_op_heads().await.unwrap();
    heads.sort();
    assert_eq!(heads, vec![a.clone(), b.clone()]);
    let mut seen = state.lock().await.seen.clone();
    seen.sort();
    assert_eq!(
        seen,
        vec![
            (
                "actor-a".into(),
                format!("request-a:{}", a.hex()),
                vec![0; 64]
            ),
            (
                "actor-b".into(),
                format!("request-b:{}", b.hex()),
                vec![1; 64]
            )
        ]
    );
    shutdown.send(()).unwrap();
    server.await.unwrap();
}
