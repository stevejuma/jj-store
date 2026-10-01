use crate::{
    CHUNK_BYTES, MAX_IDS, REQUEST_BYTES, RESPONSE_BYTES, administration as admin, agent,
    application, collab, object, record, review, service_error,
    wire::{
        self,
        native_store_server::{NativeStore, NativeStoreServer},
    },
    work,
};
use futures::{Stream, StreamExt as _};
use jj_store_model as m;
use jj_store_object::{StagedUpload, WriteOutcome};
use jj_store_service::transport as replication;
use jj_store_service::{
    administration::RepositoryHost,
    auth::{BearerToken, Permission, SystemClock},
    collaboration::ReviewThreadPager,
    mutations::CommandService,
    pagination,
    registry::{self, RepositoryRegistry},
    replication_control as control,
    transport::{NativeEndpoint, NativeRequest},
};
use std::{
    collections::{BTreeMap, BTreeSet},
    future::Future,
    pin::Pin,
    sync::Arc,
    time::Duration,
};
use tokio::{
    io::AsyncReadExt as _,
    sync::{OwnedSemaphorePermit, Semaphore},
    time::Instant,
};
use tonic::{Request, Response, Status, metadata::MetadataMap};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReplicationControlError {
    NotConfigured,
    Invalid,
    Unavailable,
    Timeout,
}

#[tonic::async_trait]
pub trait ReplicationControls: Send + Sync {
    async fn status(
        &self,
        repository: m::RepositoryId,
    ) -> Result<control::StatusV1, ReplicationControlError>;
    async fn wake(
        &self,
        repository: m::RepositoryId,
    ) -> Result<control::StatusV1, ReplicationControlError>;
    async fn set_paused(
        &self,
        repository: m::RepositoryId,
        paused: bool,
    ) -> Result<control::StatusV1, ReplicationControlError>;
    async fn capture_barrier(
        &self,
        repository: m::RepositoryId,
    ) -> Result<control::BarrierV1, ReplicationControlError>;
    async fn wait(
        &self,
        repository: m::RepositoryId,
        request: &control::WaitRequestV1,
    ) -> Result<control::WaitOutcomeV1, ReplicationControlError>;
}

fn replication_control_error(error: ReplicationControlError) -> Status {
    match error {
        ReplicationControlError::NotConfigured => {
            Status::failed_precondition("replication is not configured for this repository")
        }
        ReplicationControlError::Invalid => {
            Status::invalid_argument("invalid replication control request")
        }
        ReplicationControlError::Unavailable => {
            Status::unavailable("replication worker is unavailable")
        }
        ReplicationControlError::Timeout => {
            Status::deadline_exceeded("replication barrier deadline exceeded")
        }
    }
}

fn replication_request<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T, Status> {
    if bytes.len() > REQUEST_BYTES {
        return Err(Status::resource_exhausted("replication request limit"));
    }
    serde_json::from_slice(bytes)
        .map_err(|_| Status::invalid_argument("invalid replication request"))
}

fn replication_reply<T: serde::Serialize>(value: &T) -> Result<wire::JsonReply, Status> {
    let json = serde_json::to_vec(value)
        .map_err(|_| Status::internal("replication response encoding failed"))?;
    if json.len() > RESPONSE_BYTES {
        return Err(Status::resource_exhausted("replication response limit"));
    }
    Ok(wire::JsonReply { json })
}

#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub concurrent: usize,
    pub timeout: Duration,
    pub object_bytes: u64,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            concurrent: 8,
            timeout: Duration::from_secs(30),
            object_bytes: 64 * 1024 * 1024,
        }
    }
}
#[derive(Clone)]
pub struct NativeRpc {
    repositories: Arc<BTreeMap<m::RepositoryId, NativeEndpoint>>,
    host: Option<RepositoryHost>,
    slots: Arc<Semaphore>,
    limits: Limits,
    access: RpcAccess,
    registry: Option<Arc<dyn RepositoryRegistry>>,
    review_threads: Arc<ReviewThreadPager>,
    collab: Arc<jj_store_service::collab_pages::CollabPager>,
    work: Arc<jj_store_service::work::WorkPager>,
    commands: CommandService,
    replication_controls: Option<Arc<dyn ReplicationControls>>,
}
#[derive(Clone)]
enum RpcAccess {
    Authenticated,
    LocalTrusted {
        principal: m::PrincipalId,
        permission: Permission,
        owner_uid: u32,
    },
}
enum RequestAccess {
    Authenticated(BearerToken),
    LocalTrusted {
        principal: m::PrincipalId,
        permission: Permission,
    },
}
impl NativeRpc {
    pub fn new(repositories: Vec<NativeEndpoint>, limits: Limits) -> Result<Self, Status> {
        if limits.concurrent == 0
            || limits.concurrent > 1024
            || limits.timeout.is_zero()
            || limits.timeout > Duration::from_secs(3600)
            || limits.object_bytes < m::envelope::HEADER_LEN as u64
            || limits.object_bytes > jj_store_object::staging::MAX_SINGLE_PUT
        {
            return Err(Status::invalid_argument("native RPC limits"));
        }
        let mut registry = BTreeMap::new();
        for endpoint in repositories {
            if registry.insert(endpoint.repository(), endpoint).is_some() {
                return Err(Status::invalid_argument("duplicate repository"));
            }
        }
        let executor = jj_store_service::commands::Executor::new(
            jj_store_service::commands::Limits::default(),
            limits.concurrent,
        )
        .map_err(|_| Status::invalid_argument("native command limits"))?;
        Ok(Self {
            repositories: Arc::new(registry),
            host: None,
            slots: Arc::new(Semaphore::new(limits.concurrent)),
            limits,
            access: RpcAccess::Authenticated,
            registry: None,
            review_threads: Arc::new(
                ReviewThreadPager::new(pagination::Limits::default(), Arc::new(SystemClock))
                    .map_err(review::read_error)?,
            ),
            collab: Arc::new(
                jj_store_service::collab_pages::CollabPager::new(
                    jj_store_service::collab_pages::Limits {
                        concurrency: limits.concurrent.min(64),
                        timeout: limits.timeout.min(Duration::from_secs(300)),
                        ..Default::default()
                    },
                    Arc::new(SystemClock),
                    Arc::new(jj_store_service::collab_reads::RepositoryMemberReactionActors),
                )
                .map_err(collab::read_error)?,
            ),
            commands: CommandService::new(executor),
            work: Arc::new(jj_store_service::work::WorkPager::new(Arc::new(
                SystemClock,
            ))),
            replication_controls: None,
        })
    }
    pub fn hosted(host: RepositoryHost, limits: Limits) -> Result<Self, Status> {
        let mut rpc = Self::new(Vec::new(), limits)?;
        rpc.host = Some(host);
        Ok(rpc)
    }
    pub fn with_registry(mut self, registry: Arc<dyn RepositoryRegistry>) -> Self {
        self.registry = Some(registry);
        self
    }
    pub fn with_replication_controls(mut self, controls: Arc<dyn ReplicationControls>) -> Self {
        self.replication_controls = Some(controls);
        self
    }
    pub fn hosted_local_trusted(
        host: RepositoryHost,
        limits: Limits,
        principal: m::PrincipalId,
        permission: Permission,
        owner_uid: u32,
    ) -> Result<Self, Status> {
        let mut rpc = Self::hosted(host, limits)?;
        if permission == Permission::CreateRepository {
            return Err(Status::invalid_argument("local repository permission"));
        }
        rpc.access = RpcAccess::LocalTrusted {
            principal,
            permission,
            owner_uid,
        };
        Ok(rpc)
    }
    pub fn service(self) -> NativeStoreServer<Self> {
        NativeStoreServer::new(self)
            .max_decoding_message_size(REQUEST_BYTES)
            .max_encoding_message_size(RESPONSE_BYTES)
    }
    /// TLS is mandatory unless a literal loopback listener is explicitly enabled.
    pub async fn serve(
        self,
        listener: tokio::net::TcpListener,
        identity: Option<tonic::transport::Identity>,
        insecure_loopback: bool,
        shutdown: impl Future<Output = ()> + Send + 'static,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let address = listener.local_addr()?;
        if identity.is_none() && !(insecure_loopback && address.ip().is_loopback()) {
            return Err("TLS required except explicitly enabled loopback".into());
        }
        let mut server = tonic::transport::Server::builder();
        if let Some(identity) = identity {
            server =
                server.tls_config(tonic::transport::ServerTlsConfig::new().identity(identity))?;
        }
        server
            .add_service(self.service())
            .serve_with_incoming_shutdown(
                tokio_stream::wrappers::TcpListenerStream::new(listener),
                shutdown,
            )
            .await?;
        Ok(())
    }
    fn permit(&self) -> Result<OwnedSemaphorePermit, Status> {
        self.slots
            .clone()
            .try_acquire_owned()
            .map_err(|_| Status::resource_exhausted("native request concurrency"))
    }
    fn registry_error(error: registry::Error) -> Status {
        match error {
            registry::Error::Conflict => Status::already_exists("repository alias already exists"),
            registry::Error::Invalid => Status::invalid_argument("invalid repository alias"),
            registry::Error::Unavailable => Status::unavailable("repository registry unavailable"),
        }
    }
    fn credential(metadata: &MetadataMap) -> Result<BearerToken, Status> {
        if metadata.get_all("authorization").iter().count() != 1 {
            return Err(Status::unauthenticated("bearer credential required"));
        }
        let value = metadata
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| Status::unauthenticated("bearer credential required"))?;
        BearerToken::parse(value).map_err(|_| Status::unauthenticated("invalid bearer credential"))
    }
    fn request_access<T>(&self, request: &Request<T>) -> Result<RequestAccess, Status> {
        match self.access {
            RpcAccess::Authenticated => {
                Self::credential(request.metadata()).map(RequestAccess::Authenticated)
            }
            RpcAccess::LocalTrusted {
                principal,
                permission,
                owner_uid,
            } => {
                if request.metadata().contains_key("authorization") {
                    return Err(Status::unauthenticated(
                        "bearer credentials are disabled for local trusted RPC",
                    ));
                }
                let uid = request
                    .extensions()
                    .get::<tonic::transport::server::UdsConnectInfo>()
                    .and_then(|info| info.peer_cred)
                    .map(|credential| credential.uid());
                if uid != Some(owner_uid) {
                    return Err(Status::permission_denied("local peer access denied"));
                }
                Ok(RequestAccess::LocalTrusted {
                    principal,
                    permission,
                })
            }
        }
    }
    fn bind(&self, repository: &[u8], access: RequestAccess) -> Result<NativeRequest, Status> {
        let repository = m::RepositoryId::from_slice(repository)
            .map_err(|_| Status::invalid_argument("repository ID length"))?;
        let endpoint = match &self.host {
            Some(host) => host.endpoint(repository).map_err(service_error)?,
            None => self
                .repositories
                .get(&repository)
                .cloned()
                .ok_or_else(|| Status::permission_denied("repository access denied"))?,
        };
        match access {
            RequestAccess::Authenticated(credential) => {
                endpoint.bind(credential).map_err(service_error)
            }
            RequestAccess::LocalTrusted {
                principal,
                permission,
            } => endpoint
                .bind_local_trusted(principal, permission)
                .map_err(service_error),
        }
    }
    async fn bounded<T>(
        &self,
        work: impl Future<Output = Result<T, Status>>,
    ) -> Result<Response<T>, Status> {
        let _permit = self.permit()?;
        tokio::time::timeout(self.limits.timeout, work)
            .await
            .map_err(|_| {
                Status::deadline_exceeded(
                    "native request deadline; publication outcome may be unknown",
                )
            })?
            .map(Response::new)
    }

    fn controls(&self) -> Result<&Arc<dyn ReplicationControls>, Status> {
        self.replication_controls
            .as_ref()
            .ok_or_else(|| Status::failed_precondition("replication controls are not configured"))
    }

    async fn replication_action(
        &self,
        request: Request<wire::Repository>,
        paused: Option<bool>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let repository = m::RepositoryId::from_slice(&body.repository)
            .map_err(|_| Status::invalid_argument("repository ID length"))?;
        let session = self.bind(&body.repository, access)?;
        let controls = self.controls()?.clone();
        self.bounded(async move {
            session.authorize(true).await.map_err(service_error)?;
            let status = match paused {
                Some(paused) => controls.set_paused(repository, paused).await,
                None => controls.wake(repository).await,
            }
            .map_err(replication_control_error)?;
            replication_reply(&status)
        })
        .await
    }
}
#[tonic::async_trait]
impl NativeStore for NativeRpc {
    async fn create_repository(
        &self,
        request: Request<wire::CreateRepositoryRequest>,
    ) -> Result<Response<wire::Snapshot>, Status> {
        let access = self.request_access(&request)?;
        let mut request = request.into_inner();
        let repository = m::RepositoryId::from_slice(&request.repository)
            .map_err(|_| Status::invalid_argument("repository ID length"))?;
        let epoch = m::EpochId::from_slice(&request.epoch)
            .map_err(|_| Status::invalid_argument("epoch ID length"))?;
        let name = if request.name.is_empty() {
            repository.to_string()
        } else {
            std::mem::take(&mut request.name)
        };
        let host = self
            .host
            .as_ref()
            .ok_or_else(|| Status::unimplemented("repository creation unavailable"))?;
        self.bounded(async {
            if let Some(registry) = &self.registry {
                registry
                    .reserve(repository, name.clone())
                    .await
                    .map_err(Self::registry_error)?;
            }
            let (snapshot, initial_token) = match access {
                RequestAccess::Authenticated(token) => {
                    let credential = admin::parse_secret(&mut request.admin_credential)?;
                    let token_id = credential.id();
                    (
                        host.create(&token, repository, epoch, &credential).await,
                        Some(token_id),
                    )
                }
                RequestAccess::LocalTrusted { permission, .. } => {
                    if !request.admin_credential.is_empty() {
                        return Err(Status::invalid_argument(
                            "local trusted creation accepts no bearer credential",
                        ));
                    }
                    (
                        host.create_local_trusted(repository, epoch, permission)
                            .await,
                        None,
                    )
                }
            };
            let snapshot = snapshot.map_err(|error| service_error(error.into()))?;
            if let Some(registry) = &self.registry {
                registry
                    .register(repository, name, initial_token)
                    .await
                    .map_err(Self::registry_error)?;
            }
            Ok(admin::wire_snapshot(snapshot))
        })
        .await
    }
    async fn tokens(
        &self,
        request: Request<wire::Repository>,
    ) -> Result<Response<wire::TokenListing>, Status> {
        let repository = m::RepositoryId::from_slice(&request.get_ref().repository)
            .map_err(|_| Status::invalid_argument("repository ID length"))?;
        let session = self.bind(repository.as_bytes(), self.request_access(&request)?)?;
        self.bounded(async { admin::wire_listing(session.tokens().await.map_err(service_error)?) })
            .await
    }
    async fn change_token(
        &self,
        request: Request<wire::TokenRequest>,
    ) -> Result<Response<wire::TokenReply>, Status> {
        let repository = m::RepositoryId::from_slice(&request.get_ref().repository)
            .map_err(|_| Status::invalid_argument("repository ID length"))?;
        let session = self.bind(repository.as_bytes(), self.request_access(&request)?)?;
        let request = admin::request(request.into_inner())?;
        self.bounded(async {
            let receipt = session
                .change_token(&request)
                .await
                .map_err(service_error)?;
            if let (Some(registry), Some(receipt)) = (&self.registry, &receipt) {
                let (token, granted) = match &receipt.result {
                    jj_store_service::administration::TokenResult::Granted { token } => {
                        (token.id, true)
                    }
                    jj_store_service::administration::TokenResult::Revoked { token } => {
                        (*token, false)
                    }
                };
                registry
                    .token_candidate(repository, token, granted)
                    .await
                    .map_err(Self::registry_error)?;
            }
            admin::wire_reply(receipt)
        })
        .await
    }
    async fn token_status(
        &self,
        request: Request<wire::TokenRequest>,
    ) -> Result<Response<wire::TokenReply>, Status> {
        let repository = m::RepositoryId::from_slice(&request.get_ref().repository)
            .map_err(|_| Status::invalid_argument("repository ID length"))?;
        let session = self.bind(repository.as_bytes(), self.request_access(&request)?)?;
        let request = admin::request(request.into_inner())?;
        self.bounded(async {
            let receipt = session
                .token_status(&request)
                .await
                .map_err(service_error)?;
            if let (Some(registry), Some(receipt)) = (&self.registry, &receipt) {
                let (token, granted) = match &receipt.result {
                    jj_store_service::administration::TokenResult::Granted { token } => {
                        (token.id, true)
                    }
                    jj_store_service::administration::TokenResult::Revoked { token } => {
                        (*token, false)
                    }
                };
                registry
                    .token_candidate(repository, token, granted)
                    .await
                    .map_err(Self::registry_error)?;
            }
            admin::wire_reply(receipt)
        })
        .await
    }
    async fn authorize(
        &self,
        request: Request<wire::AccessRequest>,
    ) -> Result<Response<wire::Empty>, Status> {
        let access = self.request_access(&request)?;
        let r = request.into_inner();
        let session = self.bind(&r.repository, access)?;
        self.bounded(async {
            session.authorize(r.write).await.map_err(service_error)?;
            Ok(wire::Empty {})
        })
        .await
    }
    async fn heads(
        &self,
        request: Request<wire::Repository>,
    ) -> Result<Response<wire::Ids>, Status> {
        let session = self.bind(
            &request.get_ref().repository,
            self.request_access(&request)?,
        )?;
        self.bounded(async {
            let ids = session.heads().await.map_err(service_error)?;
            Ok(wire::Ids {
                ids: ids.into_iter().map(|id| id.as_bytes().to_vec()).collect(),
            })
        })
        .await
    }
    async fn current_snapshot(
        &self,
        request: Request<wire::Repository>,
    ) -> Result<Response<wire::Snapshot>, Status> {
        let session = self
            .bind(
                &request.get_ref().repository,
                self.request_access(&request)?,
            )?
            .into_session();
        self.bounded(async move {
            let snapshot = session
                .snapshot()
                .await
                .map_err(|error| service_error(error.into()))?
                .manifest()
                .snapshot();
            Ok(admin::wire_snapshot(snapshot))
        })
        .await
    }
    async fn inventory(
        &self,
        request: Request<wire::InventoryRequest>,
    ) -> Result<Response<wire::Ids>, Status> {
        let session = self.bind(
            &request.get_ref().repository,
            self.request_access(&request)?,
        )?;
        let r = request.into_inner();
        if r.limit == 0 || r.limit as usize > MAX_IDS {
            return Err(Status::resource_exhausted("inventory limit"));
        }
        self.bounded(async {
            Ok(wire::Ids {
                ids: session
                    .inventory(r.copies, r.limit as usize)
                    .await
                    .map_err(service_error)?,
            })
        })
        .await
    }
    async fn publish(
        &self,
        request: Request<wire::PublishRequest>,
    ) -> Result<Response<wire::Empty>, Status> {
        let session = self.bind(
            &request.get_ref().repository,
            self.request_access(&request)?,
        )?;
        let r = request.into_inner();
        if r.removed.is_empty() || r.removed.len() > 256 {
            return Err(Status::invalid_argument("native head count"));
        }
        let old: BTreeSet<_> = r
            .removed
            .iter()
            .map(|id| {
                m::OperationId::from_slice(id)
                    .map_err(|_| Status::invalid_argument("operation ID length"))
            })
            .collect::<Result<_, _>>()?;
        if old.len() != r.removed.len() {
            return Err(Status::invalid_argument("duplicate removed head"));
        }
        let new = m::OperationId::from_slice(&r.added)
            .map_err(|_| Status::invalid_argument("operation ID length"))?;
        self.bounded(async {
            session.publish(old, new).await.map_err(service_error)?;
            Ok(wire::Empty {})
        })
        .await
    }
    async fn apply_mutation(
        &self,
        request: Request<wire::MutationRequest>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let command = application::request(&body.request_json)?;
        let session = self.bind(&body.repository, access)?.into_session();
        let commands = self.commands.clone();
        self.bounded(async move {
            let outcome = commands
                .apply(session, &command)
                .await
                .map_err(application::error)?;
            application::reply(&outcome)
        })
        .await
    }
    async fn mutation_status(
        &self,
        request: Request<wire::MutationStatusRequest>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let request_id = m::RequestId::from_slice(&body.request_id)
            .map_err(|_| Status::invalid_argument("application request ID"))?;
        let session = self.bind(&body.repository, access)?.into_session();
        let commands = self.commands.clone();
        self.bounded(async move {
            let status = commands
                .status(session, request_id)
                .await
                .map_err(application::error)?;
            application::reply(&status)
        })
        .await
    }
    async fn prepare_agent_capture(
        &self,
        request: Request<wire::AgentCapturePrepareRequest>,
    ) -> Result<Response<wire::AgentCapturePrepared>, Status> {
        let access = self.request_access(&request)?;
        let prepared = agent::prepared_capture(request.into_inner())?;
        let session = self.bind(&prepared.repository, access)?;
        self.bounded(async {
            let request = session
                .prepare_agent_capture_with_binding(
                    prepared.request_id,
                    prepared.publication_id,
                    &prepared.profile,
                    &prepared.batch,
                    prepared.result,
                    prepared.binding.as_ref(),
                )
                .await
                .map_err(service_error)?;
            agent::wire_prepared(&request)
        })
        .await
    }
    async fn publish_agent_delta(
        &self,
        request: Request<wire::AgentDeltaPublishRequest>,
    ) -> Result<Response<wire::AgentPublicationReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let session = self.bind(&body.repository, access)?;
        let request = agent::delta_request(&body.request_json)?;
        self.bounded(async {
            let outcome = session
                .publish_agent_delta(&request)
                .await
                .map_err(service_error)?;
            Ok(agent::wire_outcome(outcome))
        })
        .await
    }
    async fn review_threads(
        &self,
        request: Request<wire::ReviewThreadsRequest>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let session = self.bind(&body.repository, access)?.into_session();
        let limit = usize::try_from(body.limit)
            .map_err(|_| Status::invalid_argument("review page limit"))?;
        let pager = self.review_threads.clone();
        self.bounded(async move {
            let page = if body.cursor.is_empty() {
                pager.begin(session, limit).await
            } else {
                pager.resume(session, &body.cursor, limit).await
            }
            .map_err(review::read_error)?;
            review::reply(&page)
        })
        .await
    }
    async fn review_thread(
        &self,
        request: Request<wire::ReviewThreadRequest>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let thread =
            jj_store_service::collaboration::ReviewThreadIdentity::from_slice(&body.thread_id)
                .map_err(|_| Status::invalid_argument("review thread ID"))?;
        let session = self.bind(&body.repository, access)?.into_session();
        let pager = self.review_threads.clone();
        self.bounded(async move {
            review::reply(
                &pager
                    .detail(session, thread)
                    .await
                    .map_err(review::read_error)?,
            )
        })
        .await
    }
    async fn apply_review_command(
        &self,
        request: Request<wire::ReviewCommandRequest>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let command = review::request(&body.request_json)?;
        let session = self.bind(&body.repository, access)?.into_session();
        self.bounded(async move {
            let outcome = jj_store_service::review_mutations::ReviewCommandService
                .apply(session, &command)
                .await
                .map_err(review::command_error)?;
            review::reply(&outcome)
        })
        .await
    }
    async fn review_command_status(
        &self,
        request: Request<wire::ReviewCommandStatusRequest>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let request_id = m::RequestId::from_slice(&body.request_id)
            .map_err(|_| Status::invalid_argument("review request ID"))?;
        let session = self.bind(&body.repository, access)?.into_session();
        self.bounded(async move {
            let status = jj_store_service::review_mutations::ReviewCommandService
                .status(session, request_id)
                .await
                .map_err(review::command_error)?;
            review::reply(&status)
        })
        .await
    }
    async fn collab_page(
        &self,
        request: Request<wire::CollabPageRequest>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let query = collab::decode(&body.query_json)?;
        let session = self.bind(&body.repository, access)?.into_session();
        let pager = self.collab.clone();
        self.bounded(async move {
            let cursor = (!body.cursor.is_empty()).then_some(body.cursor.as_str());
            let page = pager
                .page(session, query, cursor)
                .await
                .map_err(collab::read_error)?;
            collab::reply(&page)
        })
        .await
    }
    async fn collab_thread(
        &self,
        request: Request<wire::CollabThreadRequest>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let id = jj_store_service::collab_model::ThreadId::from_slice(&body.thread_id)
            .map_err(|_| Status::invalid_argument("Thread ID"))?;
        let session = self.bind(&body.repository, access)?.into_session();
        self.bounded(async move {
            let pin = jj_store_service::collab_reads::PinnedCollabRead::open(
                session.clone(),
                Arc::new(jj_store_service::collab_reads::RepositoryMemberReactionActors),
            )
            .await
            .map_err(collab::command_error)?;
            let thread = pin
                .thread(session, id)
                .await
                .map_err(collab::command_error)?;
            collab::reply(&jj_store_service::collab_reads::ThreadDetailV1 {
                snapshot: pin.snapshot().clone(),
                thread,
            })
        })
        .await
    }
    async fn collab_message(
        &self,
        request: Request<wire::CollabMessageRequest>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let target = collab::decode(&body.target_json)?;
        let session = self.bind(&body.repository, access)?.into_session();
        self.bounded(async move {
            let pin = jj_store_service::collab_reads::PinnedCollabRead::open(
                session.clone(),
                Arc::new(jj_store_service::collab_reads::RepositoryMemberReactionActors),
            )
            .await
            .map_err(collab::command_error)?;
            let message = pin
                .message(session, &target)
                .await
                .map_err(collab::command_error)?;
            collab::reply(&jj_store_service::collab_reads::HumanMessageDetailV1 {
                snapshot: pin.snapshot().clone(),
                target,
                message,
            })
        })
        .await
    }
    async fn collab_asset(
        &self,
        request: Request<wire::CollabAssetRequest>,
    ) -> Result<Response<wire::CollabAssetReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let id = jj_store_service::collab_model::CollabObjectId::from_slice(&body.object_id)
            .map_err(|_| Status::invalid_argument("collaboration asset ID"))?;
        let reference = jj_store_service::collab_model::CollabObjectRef {
            kind: jj_store_service::collab_model::CollabObjectKind::ImageAsset,
            id,
        };
        let session = self.bind(&body.repository, access)?.into_session();
        self.bounded(async move {
            let pin = jj_store_service::collab_reads::PinnedCollabRead::open(
                session.clone(),
                Arc::new(jj_store_service::collab_reads::RepositoryMemberReactionActors),
            )
            .await
            .map_err(collab::command_error)?;
            let asset = pin
                .image(session, reference)
                .await
                .map_err(collab::command_error)?;
            collab::asset_reply(&pin, asset)
        })
        .await
    }
    async fn apply_collab_command(
        &self,
        request: Request<wire::CollabCommandRequest>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let command = collab::command(&body)?;
        let session = self.bind(&body.repository, access)?.into_session();
        self.bounded(async move {
            let outcome = jj_store_service::collab_mutations::CollabCommandService
                .apply(session, &command)
                .await
                .map_err(collab::command_error)?;
            collab::reply(&outcome)
        })
        .await
    }
    async fn collab_command_status(
        &self,
        request: Request<wire::CollabCommandStatusRequest>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let id = m::RequestId::from_slice(&body.request_id)
            .map_err(|_| Status::invalid_argument("collaboration request ID"))?;
        let session = self.bind(&body.repository, access)?.into_session();
        self.bounded(async move {
            let status = jj_store_service::collab_mutations::CollabCommandService
                .status(session, id)
                .await
                .map_err(collab::command_error)?;
            collab::reply(&status)
        })
        .await
    }
    async fn git_bridge_state(
        &self,
        request: Request<wire::Repository>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        let access = self.request_access(&request)?;
        let session = self
            .bind(&request.into_inner().repository, access)?
            .into_session();
        self.bounded(async move {
            work::reply(
                &jj_store_service::git_bridge::read(session)
                    .await
                    .map_err(|e| service_error(e.into()))?,
            )
        })
        .await
    }
    async fn accept_git_bridge(
        &self,
        request: Request<wire::GitBridgeRequest>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let command = work::request(&body.request_json)?;
        let session = self.bind(&body.repository, access)?.into_session();
        self.bounded(async move {
            work::reply(
                &jj_store_service::git_bridge::accept(session, &command)
                    .await
                    .map_err(|e| service_error(e.into()))?,
            )
        })
        .await
    }
    async fn git_bridge_status(
        &self,
        request: Request<wire::GitBridgeStatusRequest>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let id = m::RequestId::from_slice(&body.request_id)
            .map_err(|_| Status::invalid_argument("Git request ID"))?;
        let session = self.bind(&body.repository, access)?.into_session();
        self.bounded(async move {
            work::reply(
                &jj_store_service::git_bridge::status(session, id)
                    .await
                    .map_err(|e| service_error(e.into()))?,
            )
        })
        .await
    }
    async fn work_page(
        &self,
        request: Request<wire::WorkRequest>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let query = work::request(&body.request_json)?;
        let session = self.bind(&body.repository, access)?.into_session();
        let pager = self.work.clone();
        self.bounded(
            async move { work::reply(&pager.page(session, query).await.map_err(work::error)?) },
        )
        .await
    }
    async fn apply_work_command(
        &self,
        request: Request<wire::WorkRequest>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let command = work::request(&body.request_json)?;
        let session = self.bind(&body.repository, access)?.into_session();
        self.bounded(async move {
            work::reply(
                &jj_store_service::work::WorkCommandService
                    .apply(session, &command)
                    .await
                    .map_err(work::error)?,
            )
        })
        .await
    }
    async fn work_command_status(
        &self,
        request: Request<wire::WorkCommandStatusRequest>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let id = m::RequestId::from_slice(&body.request_id)
            .map_err(|_| Status::invalid_argument("work request ID"))?;
        let session = self.bind(&body.repository, access)?.into_session();
        self.bounded(async move {
            work::reply(
                &jj_store_service::work::WorkCommandService
                    .status(session, id)
                    .await
                    .map_err(work::error)?,
            )
        })
        .await
    }
    async fn negotiate_replica(
        &self,
        request: Request<wire::ReplicationRequest>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let session = self.bind(&body.repository, access)?;
        let request: replication::NegotiateReplicaV1 = replication_request(&body.request_json)?;
        self.bounded(async {
            replication_reply(
                &session
                    .negotiate_replica(&request)
                    .await
                    .map_err(service_error)?,
            )
        })
        .await
    }
    async fn get_committed_feed(
        &self,
        request: Request<wire::ReplicationRequest>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let session = self.bind(&body.repository, access)?;
        let request: replication::FeedRequestV1 = replication_request(&body.request_json)?;
        self.bounded(async {
            replication_reply(
                &session
                    .replication_feed(&request)
                    .await
                    .map_err(service_error)?,
            )
        })
        .await
    }
    async fn offer_origins(
        &self,
        request: Request<wire::ReplicationRequest>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let session = self.bind(&body.repository, access)?;
        let request: replication::OfferOriginsRequestV1 = replication_request(&body.request_json)?;
        self.bounded(async {
            replication_reply(
                &session
                    .offer_origins(&request)
                    .await
                    .map_err(service_error)?,
            )
        })
        .await
    }
    async fn get_origin_closure(
        &self,
        request: Request<wire::ReplicationRequest>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let session = self.bind(&body.repository, access)?;
        let request: replication::OriginClosureRequestV1 = replication_request(&body.request_json)?;
        self.bounded(async {
            replication_reply(
                &session
                    .origin_closure(&request)
                    .await
                    .map_err(service_error)?,
            )
        })
        .await
    }
    async fn apply_origins(
        &self,
        request: Request<wire::ReplicationRequest>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let session = self.bind(&body.repository, access)?;
        let request: replication::ApplyOriginsTransportRequestV1 =
            replication_request(&body.request_json)?;
        self.bounded(async {
            replication_reply(
                &session
                    .apply_origins(&request)
                    .await
                    .map_err(service_error)?,
            )
        })
        .await
    }
    async fn replication_status(
        &self,
        request: Request<wire::Repository>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let repository = m::RepositoryId::from_slice(&body.repository)
            .map_err(|_| Status::invalid_argument("repository ID length"))?;
        let session = self.bind(&body.repository, access)?;
        let controls = self.controls()?.clone();
        self.bounded(async move {
            session.authorize(false).await.map_err(service_error)?;
            replication_reply(
                &controls
                    .status(repository)
                    .await
                    .map_err(replication_control_error)?,
            )
        })
        .await
    }
    async fn replication_now(
        &self,
        request: Request<wire::Repository>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        self.replication_action(request, None).await
    }
    async fn replication_pause(
        &self,
        request: Request<wire::Repository>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        self.replication_action(request, Some(true)).await
    }
    async fn replication_resume(
        &self,
        request: Request<wire::Repository>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        self.replication_action(request, Some(false)).await
    }
    async fn replication_capture_barrier(
        &self,
        request: Request<wire::Repository>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let repository = m::RepositoryId::from_slice(&body.repository)
            .map_err(|_| Status::invalid_argument("repository ID length"))?;
        let session = self.bind(&body.repository, access)?;
        let controls = self.controls()?.clone();
        self.bounded(async move {
            session.authorize(true).await.map_err(service_error)?;
            replication_reply(
                &controls
                    .capture_barrier(repository)
                    .await
                    .map_err(replication_control_error)?,
            )
        })
        .await
    }
    async fn replication_wait(
        &self,
        request: Request<wire::ReplicationControlRequest>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        let access = self.request_access(&request)?;
        let body = request.into_inner();
        let repository = m::RepositoryId::from_slice(&body.repository)
            .map_err(|_| Status::invalid_argument("repository ID length"))?;
        let session = self.bind(&body.repository, access)?;
        let request: control::WaitRequestV1 = replication_request(&body.request_json)?;
        let controls = self.controls()?.clone();
        self.bounded(async move {
            session.authorize(true).await.map_err(service_error)?;
            replication_reply(
                &controls
                    .wait(repository, &request)
                    .await
                    .map_err(replication_control_error)?,
            )
        })
        .await
    }
    async fn replica_inspect(
        &self,
        request: Request<wire::Repository>,
    ) -> Result<Response<wire::JsonReply>, Status> {
        self.replication_status(request).await
    }
    type ReadControlRecordStream =
        Pin<Box<dyn Stream<Item = Result<wire::ObjectChunk, Status>> + Send>>;
    async fn read_control_record(
        &self,
        request: Request<wire::ControlRecordRequest>,
    ) -> Result<Response<Self::ReadControlRecordStream>, Status> {
        let permit = self.permit()?;
        let deadline = Instant::now() + self.limits.timeout;
        let session = self.bind(
            &request.get_ref().repository,
            self.request_access(&request)?,
        )?;
        let reference = record(request.into_inner().record)?;
        let bytes = tokio::time::timeout_at(deadline, session.read_replication_record(reference))
            .await
            .map_err(|_| Status::deadline_exceeded("control record download deadline"))?
            .map_err(service_error)?;
        if bytes.len() as u64 > replication::MAX_RECORD_BYTES
            || bytes.len() as u64 > self.limits.object_bytes
        {
            return Err(Status::resource_exhausted("control record byte limit"));
        }
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let (finished_tx, finished_rx) = tokio::sync::oneshot::channel();
        tokio::spawn(async move {
            let _permit = permit;
            let produce = async {
                tx.send(Ok(wire::ObjectChunk {
                    part: Some(wire::object_chunk::Part::Metadata(wire::ObjectMetadata {
                        length: bytes.len() as u64,
                        version: reference.digest.to_string(),
                    })),
                }))
                .await
                .map_err(|_| Status::cancelled("control record download canceled"))?;
                for chunk in bytes.chunks(CHUNK_BYTES) {
                    session.authorize(true).await.map_err(service_error)?;
                    tx.send(Ok(wire::ObjectChunk {
                        part: Some(wire::object_chunk::Part::Data(chunk.to_vec())),
                    }))
                    .await
                    .map_err(|_| Status::cancelled("control record download canceled"))?;
                }
                Ok(())
            };
            let result = tokio::select! {
                _ = tx.closed() => return,
                result = tokio::time::timeout_at(deadline, produce) => result.unwrap_or_else(|_| Err(Status::deadline_exceeded("control record download deadline"))),
            };
            let _ = finished_tx.send(result);
        });
        let stream = async_stream::stream! {
            while let Some(part) = rx.recv().await { yield part; }
            match finished_rx.await {
                Ok(Ok(())) => {},
                Ok(Err(error)) => yield Err(error),
                Err(_) => yield Err(Status::unavailable("control record download interrupted")),
            }
        };
        Ok(Response::new(Box::pin(stream)))
    }
    async fn write_control_record(
        &self,
        request: Request<tonic::Streaming<wire::ControlUploadChunk>>,
    ) -> Result<Response<wire::UploadResult>, Status> {
        let access = self.request_access(&request)?;
        let mut stream = request.into_inner();
        self.bounded(async {
            let first = stream
                .message()
                .await?
                .ok_or_else(|| Status::invalid_argument("control upload header required"))?;
            let Some(wire::control_upload_chunk::Part::Start(start)) = first.part else {
                return Err(Status::invalid_argument("control upload header required"));
            };
            let session = self.bind(&start.repository, access)?;
            session.authorize(true).await.map_err(service_error)?;
            let reference = record(start.record)?;
            if start.length == 0
                || start.length > replication::MAX_RECORD_BYTES
                || start.length > self.limits.object_bytes
            {
                return Err(Status::resource_exhausted("control upload byte limit"));
            }
            let chunks = stream.map(|part| match part {
                Ok(wire::ControlUploadChunk {
                    part: Some(wire::control_upload_chunk::Part::Data(bytes)),
                }) if !bytes.is_empty() && bytes.len() <= CHUNK_BYTES => {
                    Ok(bytes::Bytes::from(bytes))
                }
                _ => Err(std::io::Error::other("invalid control upload stream")),
            });
            let upload =
                StagedUpload::from_reader(tokio_util::io::StreamReader::new(chunks), start.length)
                    .await
                    .map_err(|_| Status::invalid_argument("invalid control upload stream"))?;
            if upload.len() != start.length {
                return Err(Status::invalid_argument("control upload length mismatch"));
            }
            let mut bytes = Vec::with_capacity(start.length as usize);
            upload
                .reader()
                .map_err(|_| Status::internal("control upload spool unavailable"))?
                .read_to_end(&mut bytes)
                .await
                .map_err(|_| Status::unavailable("control upload spool read failed"))?;
            let outcome = session
                .write_replication_record(reference, &bytes)
                .await
                .map_err(service_error)?;
            let (outcome, version) = match outcome {
                WriteOutcome::Applied(version) => (
                    wire::upload_result::Outcome::Stored,
                    version.as_str().to_owned(),
                ),
                WriteOutcome::PreconditionFailed => {
                    (wire::upload_result::Outcome::AlreadyPresent, String::new())
                }
                WriteOutcome::Unknown => (wire::upload_result::Outcome::Unknown, String::new()),
            };
            Ok(wire::UploadResult {
                outcome: outcome as i32,
                version,
            })
        })
        .await
    }
    async fn head_object(
        &self,
        request: Request<wire::ObjectRequest>,
    ) -> Result<Response<wire::ObjectMetadata>, Status> {
        let session = self.bind(
            &request.get_ref().repository,
            self.request_access(&request)?,
        )?;
        let key = object(request.into_inner().object)?;
        self.bounded(async {
            let metadata = session.head(key).await.map_err(service_error)?;
            if metadata.len > self.limits.object_bytes {
                return Err(Status::resource_exhausted("object byte limit"));
            }
            Ok(wire::ObjectMetadata {
                length: metadata.len,
                version: metadata.version.as_str().into(),
            })
        })
        .await
    }
    type ReadObjectStream = Pin<Box<dyn Stream<Item = Result<wire::ObjectChunk, Status>> + Send>>;
    async fn read_object(
        &self,
        request: Request<wire::ObjectRequest>,
    ) -> Result<Response<Self::ReadObjectStream>, Status> {
        let permit = self.permit()?;
        let deadline = Instant::now() + self.limits.timeout;
        let session = self.bind(
            &request.get_ref().repository,
            self.request_access(&request)?,
        )?;
        let key = object(request.into_inner().object)?;
        let mut download = tokio::time::timeout_at(deadline, session.read(key))
            .await
            .map_err(|_| Status::deadline_exceeded("native download deadline"))?
            .map_err(service_error)?;
        if download.metadata.len > self.limits.object_bytes {
            return Err(Status::resource_exhausted("object byte limit"));
        }
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let (finished_tx, finished_rx) = tokio::sync::oneshot::channel();
        tokio::spawn(async move {
            let _permit = permit;
            let produce = async {
                tx.send(Ok(wire::ObjectChunk {
                    part: Some(wire::object_chunk::Part::Metadata(wire::ObjectMetadata {
                        length: download.metadata.len,
                        version: download.metadata.version.as_str().into(),
                    })),
                }))
                .await
                .map_err(|_| Status::cancelled("download canceled"))?;
                let mut remaining = download.metadata.len;
                let mut buffer = vec![0; CHUNK_BYTES];
                loop {
                    let n = download
                        .reader
                        .read(&mut buffer)
                        .await
                        .map_err(|_| Status::unavailable("native download failed"))?;
                    if n == 0 {
                        break;
                    }
                    remaining = remaining
                        .checked_sub(n as u64)
                        .ok_or_else(|| Status::data_loss("download length mismatch"))?;
                    session.authorize(false).await.map_err(service_error)?;
                    tx.send(Ok(wire::ObjectChunk {
                        part: Some(wire::object_chunk::Part::Data(buffer[..n].to_vec())),
                    }))
                    .await
                    .map_err(|_| Status::cancelled("download canceled"))?;
                }
                if remaining != 0 {
                    return Err(Status::data_loss("download truncated"));
                }
                session.authorize(false).await.map_err(service_error)
            };
            let result = tokio::select! {
                _ = tx.closed() => return,
                result = tokio::time::timeout_at(deadline, produce) => result.unwrap_or_else(|_| Err(Status::deadline_exceeded("native download deadline"))),
            };
            let _ = finished_tx.send(result);
        });
        // The terminal status cannot be lost when a slow client fills the data
        // channel. Producers release permits independently of draining clients.
        let stream = async_stream::stream! {
            while let Some(part) = rx.recv().await { yield part; }
            match finished_rx.await {
                Ok(Ok(())) => {},
                Ok(Err(error)) => yield Err(error),
                Err(_) => yield Err(Status::unavailable("native download interrupted")),
            }
        };
        Ok(Response::new(Box::pin(stream)))
    }
    async fn write_object(
        &self,
        request: Request<tonic::Streaming<wire::UploadChunk>>,
    ) -> Result<Response<wire::UploadResult>, Status> {
        let access = self.request_access(&request)?;
        let mut stream = request.into_inner();
        self.bounded(async {
            let first = stream
                .message()
                .await?
                .ok_or_else(|| Status::invalid_argument("upload header required"))?;
            let Some(wire::upload_chunk::Part::Start(start)) = first.part else {
                return Err(Status::invalid_argument("upload header required"));
            };
            let session = self.bind(&start.repository, access)?;
            session.authorize(true).await.map_err(service_error)?;
            let key = object(start.object)?;
            if start.length < m::envelope::HEADER_LEN as u64
                || start.length > self.limits.object_bytes
            {
                return Err(Status::resource_exhausted("upload byte limit"));
            }
            let chunks = stream.map(|part| match part {
                Ok(wire::UploadChunk {
                    part: Some(wire::upload_chunk::Part::Data(bytes)),
                }) if !bytes.is_empty() && bytes.len() <= CHUNK_BYTES => {
                    Ok(bytes::Bytes::from(bytes))
                }
                _ => Err(std::io::Error::other("invalid upload stream")),
            });
            let upload =
                StagedUpload::from_reader(tokio_util::io::StreamReader::new(chunks), start.length)
                    .await
                    .map_err(|_| Status::invalid_argument("invalid or oversized upload stream"))?;
            if upload.len() != start.length {
                return Err(Status::invalid_argument("upload length mismatch"));
            }
            let outcome = session.upload(key, &upload).await.map_err(service_error)?;
            let (outcome, version) = match outcome {
                WriteOutcome::Applied(version) => (
                    wire::upload_result::Outcome::Stored,
                    version.as_str().to_owned(),
                ),
                WriteOutcome::PreconditionFailed => {
                    (wire::upload_result::Outcome::AlreadyPresent, String::new())
                }
                WriteOutcome::Unknown => (wire::upload_result::Outcome::Unknown, String::new()),
            };
            Ok(wire::UploadResult {
                outcome: outcome as i32,
                version,
            })
        })
        .await
    }
}
