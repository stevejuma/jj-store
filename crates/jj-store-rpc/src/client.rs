use crate::{
    CHUNK_BYTES, MAX_IDS, REQUEST_BYTES, RESPONSE_BYTES, administration as admin, agent,
    application, review,
    wire::{self, native_store_client::NativeStoreClient},
    wire_object, wire_record,
};
use jj_store_model as m;
use jj_store_object::{
    self as storage, ByteRange, Condition, Download, Key, Metadata, ObjectStore, StagedUpload,
    VersionToken, WriteOutcome,
};
use jj_store_service::administration::{
    AdministrationSnapshot, TokenListing, TokenReceipt, TokenRequest,
};
use jj_store_service::auth::BearerToken;
use jj_store_service::replication_control as control;
use jj_store_service::transport as replication;
use jj_store_service::transport::RecordRef;
use jj_store_service::transport::{
    AgentDeltaRequest, NormalizerProfileV1, PublishOutcome, SanitizedObservationBatchV1,
};
use sha2::{Digest as _, Sha256};
use std::{collections::BTreeSet, sync::Arc, time::Duration};
use tokio::{io::AsyncReadExt as _, time::Instant};
use tonic::{
    Code, Request, Status,
    transport::{Channel, ClientTlsConfig, Endpoint},
};

#[derive(Clone, Debug)]
pub struct Options {
    pub insecure_loopback: bool,
    /// Optional deployment CA; normal hostname verification remains enabled.
    pub ca_certificate_pem: Option<Vec<u8>>,
    pub timeout: Duration,
    pub object_bytes: u64,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            insecure_loopback: false,
            ca_certificate_pem: None,
            timeout: Duration::from_secs(30),
            object_bytes: 64 * 1024 * 1024,
        }
    }
}
#[derive(Clone)]
pub struct NativeClient {
    channel: Channel,
    token: Option<Arc<BearerToken>>,
    repository: m::RepositoryId,
    options: Options,
}
impl NativeClient {
    pub async fn connect(
        url: &str,
        repository: m::RepositoryId,
        token: BearerToken,
        options: Options,
    ) -> Result<Self, Status> {
        let parsed =
            url::Url::parse(url).map_err(|_| Status::invalid_argument("RPC endpoint URL"))?;
        let loopback = match parsed.host() {
            Some(url::Host::Ipv4(ip)) => ip.is_loopback(),
            Some(url::Host::Ipv6(ip)) => ip.is_loopback(),
            _ => false,
        };
        if !parsed.username().is_empty()
            || parsed.password().is_some()
            || parsed.query().is_some()
            || parsed.fragment().is_some()
            || parsed.path() != "/"
            || !matches!(parsed.scheme(), "http" | "https")
            || (parsed.scheme() != "https" && !(options.insecure_loopback && loopback))
        {
            return Err(Status::invalid_argument(
                "TLS required except explicitly enabled literal loopback",
            ));
        }
        if options.timeout.is_zero()
            || options.timeout > Duration::from_secs(3600)
            || options.object_bytes < m::envelope::HEADER_LEN as u64
            || options.object_bytes > storage::staging::MAX_SINGLE_PUT
        {
            return Err(Status::invalid_argument("native client limits"));
        }
        let mut endpoint = Endpoint::from_shared(url.to_owned())
            .map_err(|_| Status::invalid_argument("RPC endpoint URL"))?
            .connect_timeout(options.timeout)
            .timeout(options.timeout);
        if parsed.scheme() == "https" {
            let mut tls = ClientTlsConfig::new().with_native_roots();
            if let Some(pem) = &options.ca_certificate_pem {
                tls = tls.ca_certificate(tonic::transport::Certificate::from_pem(pem));
            }
            endpoint = endpoint
                .tls_config(tls)
                .map_err(|_| Status::invalid_argument("TLS configuration"))?;
        }
        let channel = endpoint
            .connect()
            .await
            .map_err(|_| Status::unavailable("RPC connection failed"))?;
        Ok(Self {
            channel,
            token: Some(Arc::new(token)),
            repository,
            options,
        })
    }
    pub async fn connect_unix(
        socket: &std::path::Path,
        repository: m::RepositoryId,
        options: Options,
    ) -> Result<Self, Status> {
        if !socket.is_absolute()
            || socket.as_os_str().as_encoded_bytes().len() > 4096
            || options.timeout.is_zero()
            || options.timeout > Duration::from_secs(3600)
            || options.object_bytes < m::envelope::HEADER_LEN as u64
            || options.object_bytes > storage::staging::MAX_SINGLE_PUT
            || options.ca_certificate_pem.is_some()
        {
            return Err(Status::invalid_argument("native Unix client configuration"));
        }
        let endpoint = Endpoint::from_static("http://localhost")
            .connect_timeout(options.timeout)
            .timeout(options.timeout);
        let socket = socket.to_owned();
        let channel = endpoint
            .connect_with_connector(tower::service_fn(move |_| {
                let socket = socket.clone();
                async move {
                    tokio::net::UnixStream::connect(socket)
                        .await
                        .map(hyper_util::rt::TokioIo::new)
                }
            }))
            .await
            .map_err(|_| Status::unavailable("RPC connection failed"))?;
        Ok(Self {
            channel,
            token: None,
            repository,
            options,
        })
    }
    pub fn repository(&self) -> m::RepositoryId {
        self.repository
    }
    /// This client's bearer token must have the separate operator capability.
    /// Retain the exact repository, epoch and admin credential for retries.
    pub async fn create_repository(
        &self,
        epoch: m::EpochId,
        credential: &BearerToken,
    ) -> Result<AdministrationSnapshot, Status> {
        self.create_repository_named(epoch, credential, &self.repository.to_string())
            .await
    }
    pub async fn create_repository_named(
        &self,
        epoch: m::EpochId,
        credential: &BearerToken,
        name: &str,
    ) -> Result<AdministrationSnapshot, Status> {
        self.create_repository_request(epoch, Some(credential), name)
            .await
    }
    pub async fn create_repository_local(
        &self,
        epoch: m::EpochId,
    ) -> Result<AdministrationSnapshot, Status> {
        if self.token.is_some() {
            return Err(Status::invalid_argument(
                "local repository creation requires a Unix connection",
            ));
        }
        self.create_repository_local_named(epoch, &self.repository.to_string())
            .await
    }
    pub async fn create_repository_local_named(
        &self,
        epoch: m::EpochId,
        name: &str,
    ) -> Result<AdministrationSnapshot, Status> {
        if self.token.is_some() {
            return Err(Status::invalid_argument(
                "local repository creation requires a Unix connection",
            ));
        }
        self.create_repository_request(epoch, None, name).await
    }
    async fn create_repository_request(
        &self,
        epoch: m::EpochId,
        credential: Option<&BearerToken>,
        name: &str,
    ) -> Result<AdministrationSnapshot, Status> {
        let response = self
            .wire()
            .create_repository(
                self.request(wire::CreateRepositoryRequest {
                    repository: self.repository.as_bytes().to_vec(),
                    epoch: epoch.as_bytes().to_vec(),
                    admin_credential: credential
                        .map(|value| value.expose().to_string())
                        .unwrap_or_default(),
                    name: name.to_owned(),
                }),
            )
            .await?
            .into_inner();
        let snapshot = admin::snapshot(Some(response)).map_err(admin::bad_response)?;
        if snapshot.epoch != epoch {
            return Err(Status::data_loss("creation epoch mismatch"));
        }
        Ok(snapshot)
    }
    pub async fn tokens(&self) -> Result<TokenListing, Status> {
        let response = self
            .wire()
            .tokens(self.request(wire::Repository {
                repository: self.repository.as_bytes().to_vec(),
            }))
            .await?
            .into_inner();
        admin::listing(response).map_err(admin::bad_response)
    }
    /// None denotes an unknown outcome. Retry this exact request or ask status.
    pub async fn change_token(
        &self,
        request: &TokenRequest,
    ) -> Result<Option<TokenReceipt>, Status> {
        let body = admin::wire_request(self.repository, request)?;
        let response = self
            .wire()
            .change_token(self.request(body))
            .await?
            .into_inner();
        admin::reply(response, request).map_err(admin::bad_response)
    }
    /// None means no committed receipt observed; it does not confirm rejection.
    pub async fn token_status(
        &self,
        request: &TokenRequest,
    ) -> Result<Option<TokenReceipt>, Status> {
        let body = admin::wire_request(self.repository, request)?;
        let response = self
            .wire()
            .token_status(self.request(body))
            .await?
            .into_inner();
        admin::reply(response, request).map_err(admin::bad_response)
    }
    fn wire(&self) -> NativeStoreClient<Channel> {
        NativeStoreClient::new(self.channel.clone())
            .max_decoding_message_size(RESPONSE_BYTES)
            .max_encoding_message_size(REQUEST_BYTES)
    }
    fn request<T>(&self, body: T) -> Request<T> {
        let mut request = Request::new(body);
        if let Some(token) = &self.token {
            let exposed = token.expose();
            let mut authorization = format!("Bearer {}", exposed.as_str())
                .parse::<tonic::metadata::MetadataValue<_>>()
                .expect("canonical ASCII bearer token");
            authorization.set_sensitive(true);
            request
                .metadata_mut()
                .insert("authorization", authorization);
        }
        request.set_timeout(self.options.timeout);
        request
    }
    pub async fn authorize(&self, write: bool) -> Result<(), Status> {
        self.wire()
            .authorize(self.request(wire::AccessRequest {
                repository: self.repository.as_bytes().to_vec(),
                write,
            }))
            .await?;
        Ok(())
    }
    pub async fn heads(&self) -> Result<BTreeSet<m::OperationId>, Status> {
        let response = self
            .wire()
            .heads(self.request(wire::Repository {
                repository: self.repository.as_bytes().to_vec(),
            }))
            .await?
            .into_inner();
        if response.ids.is_empty() || response.ids.len() > 256 {
            return Err(Status::data_loss("invalid head response"));
        }
        let count = response.ids.len();
        let ids: BTreeSet<_> = response
            .ids
            .iter()
            .map(|id| {
                m::OperationId::from_slice(id)
                    .map_err(|_| Status::data_loss("invalid operation ID"))
            })
            .collect::<Result<_, _>>()?;
        if ids.len() != count {
            return Err(Status::data_loss("duplicate heads"));
        }
        Ok(ids)
    }
    pub async fn snapshot(&self) -> Result<jj_store_service::transport::Snapshot, Status> {
        let response = self
            .wire()
            .current_snapshot(self.request(wire::Repository {
                repository: self.repository.as_bytes().to_vec(),
            }))
            .await?
            .into_inner();
        admin::snapshot(Some(response)).map_err(admin::bad_response)
    }
    pub async fn inventory(&self, copies: bool, limit: usize) -> Result<Vec<[u8; 64]>, Status> {
        if limit == 0 || limit > MAX_IDS {
            return Err(Status::resource_exhausted("inventory limit"));
        }
        let response = self
            .wire()
            .inventory(self.request(wire::InventoryRequest {
                repository: self.repository.as_bytes().to_vec(),
                copies,
                limit: limit as u32,
            }))
            .await?
            .into_inner();
        if response.ids.len() > limit {
            return Err(Status::data_loss("inventory response limit"));
        }
        let ids: Vec<[u8; 64]> = response
            .ids
            .into_iter()
            .map(|id| {
                id.try_into()
                    .map_err(|_| Status::data_loss("inventory ID length"))
            })
            .collect::<Result<_, _>>()?;
        if ids.windows(2).any(|pair| pair[0] >= pair[1]) {
            return Err(Status::data_loss("noncanonical inventory"));
        }
        Ok(ids)
    }
    pub async fn publish(
        &self,
        old: BTreeSet<m::OperationId>,
        new: m::OperationId,
    ) -> Result<(), Status> {
        self.wire()
            .publish(self.request(wire::PublishRequest {
                repository: self.repository.as_bytes().to_vec(),
                removed: old.iter().map(|id| id.as_bytes().to_vec()).collect(),
                added: new.as_bytes().to_vec(),
            }))
            .await?;
        Ok(())
    }
    pub async fn apply_mutation(
        &self,
        request: &jj_store_service::mutations::Request,
    ) -> Result<jj_store_service::mutations::Outcome, Status> {
        let request_json = serde_json::to_vec(request)
            .map_err(|_| Status::invalid_argument("application mutation JSON request"))?;
        let reply = self
            .wire()
            .apply_mutation(self.request(wire::MutationRequest {
                repository: self.repository.as_bytes().to_vec(),
                request_json,
            }))
            .await?
            .into_inner();
        application::response(reply)
    }
    pub async fn mutation_status(
        &self,
        request_id: m::RequestId,
    ) -> Result<jj_store_service::mutations::Status, Status> {
        let reply = self
            .wire()
            .mutation_status(self.request(wire::MutationStatusRequest {
                repository: self.repository.as_bytes().to_vec(),
                request_id: request_id.as_bytes().to_vec(),
            }))
            .await?
            .into_inner();
        application::response(reply)
    }
    fn replication_request<T: serde::Serialize>(
        &self,
        request: &T,
    ) -> Result<Request<wire::ReplicationRequest>, Status> {
        let request_json = serde_json::to_vec(request)
            .map_err(|_| Status::invalid_argument("replication request encoding"))?;
        if request_json.len() > REQUEST_BYTES {
            return Err(Status::resource_exhausted("replication request limit"));
        }
        Ok(self.request(wire::ReplicationRequest {
            repository: self.repository.as_bytes().to_vec(),
            request_json,
        }))
    }
    fn replication_response<T: serde::de::DeserializeOwned>(
        &self,
        reply: wire::JsonReply,
    ) -> Result<T, Status> {
        if reply.json.len() > RESPONSE_BYTES {
            return Err(Status::resource_exhausted("replication response limit"));
        }
        serde_json::from_slice(&reply.json)
            .map_err(|_| Status::data_loss("invalid replication response"))
    }
    pub async fn negotiate_replica(
        &self,
        request: &replication::NegotiateReplicaV1,
    ) -> Result<replication::ReplicaNegotiationV1, Status> {
        let body = self.replication_request(request)?;
        let reply = self.wire().negotiate_replica(body).await?.into_inner();
        self.replication_response(reply)
    }
    pub async fn committed_feed(
        &self,
        request: &replication::FeedRequestV1,
    ) -> Result<replication::FeedResponseV1, Status> {
        let body = self.replication_request(request)?;
        let reply = self.wire().get_committed_feed(body).await?.into_inner();
        self.replication_response(reply)
    }
    pub async fn offer_origins(
        &self,
        request: &replication::OfferOriginsRequestV1,
    ) -> Result<replication::OfferOriginsResponseV1, Status> {
        let body = self.replication_request(request)?;
        let reply = self.wire().offer_origins(body).await?.into_inner();
        self.replication_response(reply)
    }
    pub async fn origin_closure(
        &self,
        request: &replication::OriginClosureRequestV1,
    ) -> Result<replication::OriginClosureResponseV1, Status> {
        let body = self.replication_request(request)?;
        let reply = self.wire().get_origin_closure(body).await?.into_inner();
        self.replication_response(reply)
    }
    pub async fn apply_origins(
        &self,
        request: &replication::ApplyOriginsTransportRequestV1,
    ) -> Result<replication::ApplyOriginsTransportResponseV1, Status> {
        let body = self.replication_request(request)?;
        let reply = self.wire().apply_origins(body).await?.into_inner();
        self.replication_response(reply)
    }
    fn repository_request(&self) -> Request<wire::Repository> {
        self.request(wire::Repository {
            repository: self.repository.as_bytes().to_vec(),
        })
    }
    pub async fn replication_status(&self) -> Result<control::StatusV1, Status> {
        let request = self.repository_request();
        let reply = self.wire().replication_status(request).await?.into_inner();
        self.replication_response(reply)
    }
    pub async fn replication_now(&self) -> Result<control::StatusV1, Status> {
        let request = self.repository_request();
        let reply = self.wire().replication_now(request).await?.into_inner();
        self.replication_response(reply)
    }
    pub async fn replication_pause(&self) -> Result<control::StatusV1, Status> {
        let request = self.repository_request();
        let reply = self.wire().replication_pause(request).await?.into_inner();
        self.replication_response(reply)
    }
    pub async fn replication_resume(&self) -> Result<control::StatusV1, Status> {
        let request = self.repository_request();
        let reply = self.wire().replication_resume(request).await?.into_inner();
        self.replication_response(reply)
    }
    pub async fn replication_capture_barrier(&self) -> Result<control::BarrierV1, Status> {
        let request = self.repository_request();
        let reply = self
            .wire()
            .replication_capture_barrier(request)
            .await?
            .into_inner();
        self.replication_response(reply)
    }
    pub async fn replication_wait(
        &self,
        request: &control::WaitRequestV1,
    ) -> Result<control::WaitOutcomeV1, Status> {
        let request_json = serde_json::to_vec(request)
            .map_err(|_| Status::invalid_argument("replication control encoding"))?;
        if request_json.len() > REQUEST_BYTES {
            return Err(Status::resource_exhausted("replication control limit"));
        }
        let request = self.request(wire::ReplicationControlRequest {
            repository: self.repository.as_bytes().to_vec(),
            request_json,
        });
        let reply = self.wire().replication_wait(request).await?.into_inner();
        self.replication_response(reply)
    }
    pub async fn replica_inspect(&self) -> Result<control::StatusV1, Status> {
        let request = self.repository_request();
        let reply = self.wire().replica_inspect(request).await?.into_inner();
        self.replication_response(reply)
    }
    fn control_request(&self, reference: RecordRef) -> Request<wire::ControlRecordRequest> {
        self.request(wire::ControlRecordRequest {
            repository: self.repository.as_bytes().to_vec(),
            record: Some(wire_record(reference)),
        })
    }
    pub async fn read_control_record(&self, reference: RecordRef) -> Result<Vec<u8>, Status> {
        let deadline = Instant::now() + self.options.timeout;
        let mut response = self
            .wire()
            .read_control_record(self.control_request(reference))
            .await?
            .into_inner();
        let first = tokio::time::timeout_at(deadline, response.message())
            .await
            .map_err(|_| Status::deadline_exceeded("control download deadline"))??;
        let Some(wire::ObjectChunk {
            part: Some(wire::object_chunk::Part::Metadata(metadata)),
        }) = first
        else {
            return Err(Status::data_loss("control record metadata required"));
        };
        if metadata.length == 0
            || metadata.length > replication::MAX_RECORD_BYTES
            || metadata.length > self.options.object_bytes
            || metadata.version != reference.digest.to_string()
        {
            return Err(Status::data_loss("control record metadata"));
        }
        let mut bytes = Vec::with_capacity(metadata.length as usize);
        while let Some(part) = tokio::time::timeout_at(deadline, response.message())
            .await
            .map_err(|_| Status::deadline_exceeded("control download deadline"))??
        {
            let Some(wire::object_chunk::Part::Data(chunk)) = part.part else {
                return Err(Status::data_loss("invalid control record chunk"));
            };
            if chunk.is_empty() || chunk.len() > CHUNK_BYTES {
                return Err(Status::data_loss("control record chunk limit"));
            }
            if bytes.len().saturating_add(chunk.len()) > metadata.length as usize {
                return Err(Status::data_loss("control record length mismatch"));
            }
            bytes.extend_from_slice(&chunk);
        }
        if bytes.len() as u64 != metadata.length
            || m::RecordDigest::new(Sha256::digest(&bytes).into()) != reference.digest
        {
            return Err(Status::data_loss("control record digest mismatch"));
        }
        Ok(bytes)
    }
    pub async fn write_control_record(
        &self,
        reference: RecordRef,
        bytes: Vec<u8>,
    ) -> Result<WriteOutcome, Status> {
        if bytes.is_empty()
            || bytes.len() as u64 > replication::MAX_RECORD_BYTES
            || bytes.len() as u64 > self.options.object_bytes
            || m::RecordDigest::new(Sha256::digest(&bytes).into()) != reference.digest
        {
            return Err(Status::invalid_argument("control record bytes"));
        }
        let start = wire::ControlUploadStart {
            repository: self.repository.as_bytes().to_vec(),
            record: Some(wire_record(reference)),
            length: bytes.len() as u64,
        };
        let messages = std::iter::once(wire::ControlUploadChunk {
            part: Some(wire::control_upload_chunk::Part::Start(start)),
        })
        .chain(
            bytes
                .chunks(CHUNK_BYTES)
                .map(|chunk| wire::ControlUploadChunk {
                    part: Some(wire::control_upload_chunk::Part::Data(chunk.to_vec())),
                }),
        )
        .collect::<Vec<_>>();
        let reply = self
            .wire()
            .write_control_record(self.request(tokio_stream::iter(messages)))
            .await?
            .into_inner();
        match wire::upload_result::Outcome::try_from(reply.outcome) {
            Ok(wire::upload_result::Outcome::Stored) => Ok(WriteOutcome::Applied(
                VersionToken::new(reply.version)
                    .map_err(|_| Status::data_loss("control upload version"))?,
            )),
            Ok(wire::upload_result::Outcome::AlreadyPresent) => {
                Ok(WriteOutcome::PreconditionFailed)
            }
            Ok(wire::upload_result::Outcome::Unknown) => Ok(WriteOutcome::Unknown),
            _ => Err(Status::data_loss("invalid control upload outcome")),
        }
    }
    pub async fn prepare_agent_capture(
        &self,
        request_id: m::RequestId,
        publication_id: m::PublicationId,
        profile: &NormalizerProfileV1,
        batch: &SanitizedObservationBatchV1,
        result: Vec<u8>,
    ) -> Result<AgentDeltaRequest, Status> {
        let expected_result = result.clone();
        let body = agent::prepare_request(
            self.repository,
            request_id,
            publication_id,
            profile,
            batch,
            result,
        )?;
        let response = self
            .wire()
            .prepare_agent_capture(self.request(body))
            .await?
            .into_inner();
        agent::prepared(response, request_id, publication_id, &expected_result)
    }
    pub async fn publish_agent_delta(
        &self,
        request: &AgentDeltaRequest,
    ) -> Result<PublishOutcome, Status> {
        let body = agent::publish_request(self.repository, request)?;
        let response = self
            .wire()
            .publish_agent_delta(self.request(body))
            .await?
            .into_inner();
        agent::outcome(response, request)
    }
    pub async fn review_threads(
        &self,
        cursor: Option<&str>,
        limit: usize,
    ) -> Result<jj_store_service::collaboration::ReviewThreadPage, Status> {
        let limit =
            u32::try_from(limit).map_err(|_| Status::invalid_argument("review page limit"))?;
        let reply = self
            .wire()
            .review_threads(self.request(wire::ReviewThreadsRequest {
                repository: self.repository.as_bytes().to_vec(),
                limit,
                cursor: cursor.unwrap_or_default().to_owned(),
            }))
            .await?
            .into_inner();
        review::response(reply)
    }
    pub async fn review_thread(
        &self,
        thread: jj_store_service::collaboration::ReviewThreadIdentity,
    ) -> Result<jj_store_service::collaboration::ReviewThreadDetail, Status> {
        let reply = self
            .wire()
            .review_thread(self.request(wire::ReviewThreadRequest {
                repository: self.repository.as_bytes().to_vec(),
                thread_id: thread.as_bytes().to_vec(),
            }))
            .await?
            .into_inner();
        review::response(reply)
    }
    pub async fn apply_review_command(
        &self,
        request: &jj_store_service::review_mutations::Request,
    ) -> Result<jj_store_service::review_mutations::Outcome, Status> {
        let request_json = serde_json::to_vec(request)
            .map_err(|_| Status::invalid_argument("review JSON request"))?;
        let reply = self
            .wire()
            .apply_review_command(self.request(wire::ReviewCommandRequest {
                repository: self.repository.as_bytes().to_vec(),
                request_json,
            }))
            .await?
            .into_inner();
        review::response(reply)
    }
    pub async fn review_command_status(
        &self,
        request_id: m::RequestId,
    ) -> Result<jj_store_service::review_mutations::Status, Status> {
        let reply = self
            .wire()
            .review_command_status(self.request(wire::ReviewCommandStatusRequest {
                repository: self.repository.as_bytes().to_vec(),
                request_id: request_id.as_bytes().to_vec(),
            }))
            .await?
            .into_inner();
        review::response(reply)
    }
    fn object_request(&self, key: m::ObjectKey) -> Request<wire::ObjectRequest> {
        self.request(wire::ObjectRequest {
            repository: self.repository.as_bytes().to_vec(),
            object: Some(wire_object(key)),
        })
    }
    pub async fn head(&self, key: m::ObjectKey) -> Result<Metadata, Status> {
        let metadata = self
            .wire()
            .head_object(self.object_request(key))
            .await?
            .into_inner();
        self.metadata(metadata)
    }
    fn metadata(&self, metadata: wire::ObjectMetadata) -> Result<Metadata, Status> {
        if metadata.length > self.options.object_bytes
            || metadata.length < m::envelope::HEADER_LEN as u64
        {
            return Err(Status::data_loss("object response length"));
        }
        Ok(Metadata {
            len: metadata.length,
            version: VersionToken::new(metadata.version)
                .map_err(|_| Status::data_loss("object version token"))?,
        })
    }
    pub async fn read(&self, key: m::ObjectKey) -> Result<Download, Status> {
        let deadline = Instant::now() + self.options.timeout;
        let mut response = self
            .wire()
            .read_object(self.object_request(key))
            .await?
            .into_inner();
        let first = tokio::time::timeout_at(deadline, response.message())
            .await
            .map_err(|_| Status::deadline_exceeded("native download deadline"))??;
        let Some(wire::ObjectChunk {
            part: Some(wire::object_chunk::Part::Metadata(metadata)),
        }) = first
        else {
            return Err(Status::data_loss("object metadata required"));
        };
        let metadata = self.metadata(metadata)?;
        let mut remaining = metadata.len;
        let chunks: std::pin::Pin<
            Box<dyn futures::Stream<Item = std::io::Result<bytes::Bytes>> + Send>,
        > = Box::pin(async_stream::try_stream! {
            loop {
                let next = tokio::time::timeout_at(deadline, response.message()).await
                    .map_err(|_| std::io::Error::other("native download deadline"))?
                    .map_err(|_| std::io::Error::other("native download failed"))?;
                let Some(part) = next else { break; };
                let Some(wire::object_chunk::Part::Data(bytes)) = part.part else {
                    Err(std::io::Error::other("invalid download chunk"))?;
                    unreachable!();
                };
                if bytes.is_empty() || bytes.len() > CHUNK_BYTES { Err(std::io::Error::other("download chunk limit"))?; }
                remaining = remaining.checked_sub(bytes.len() as u64).ok_or_else(|| std::io::Error::other("download length mismatch"))?;
                yield bytes::Bytes::from(bytes);
            }
            if remaining != 0 { Err(std::io::Error::other("download truncated"))?; }
        });
        Ok(Download {
            metadata,
            reader: Box::pin(tokio_util::io::StreamReader::new(Box::pin(chunks))),
        })
    }
    pub fn upload(
        &self,
        key: m::ObjectKey,
        upload: &StagedUpload,
    ) -> impl std::future::Future<Output = Result<WriteOutcome, Status>> + Send + 'static {
        let this = self.clone();
        let length = upload.len();
        let reader = upload.reader();
        async move {
            if length > this.options.object_bytes {
                return Err(Status::resource_exhausted("upload limit"));
            }
            let mut reader = reader.map_err(|_| Status::internal("upload spool unavailable"))?;
            let start = wire::UploadStart {
                repository: this.repository.as_bytes().to_vec(),
                object: Some(wire_object(key)),
                length,
            };
            let (tx, rx) = tokio::sync::mpsc::channel(1);
            let timeout = this.options.timeout;
            tokio::spawn(async move {
                let produce = async {
                    if tx
                        .send(wire::UploadChunk {
                            part: Some(wire::upload_chunk::Part::Start(start)),
                        })
                        .await
                        .is_err()
                    {
                        return;
                    }
                    let mut buffer = vec![0; CHUNK_BYTES];
                    loop {
                        match reader.read(&mut buffer).await {
                            Ok(0) | Err(_) => break, // The server rejects truncation before storage.
                            Ok(n) => {
                                if tx
                                    .send(wire::UploadChunk {
                                        part: Some(wire::upload_chunk::Part::Data(
                                            buffer[..n].to_vec(),
                                        )),
                                    })
                                    .await
                                    .is_err()
                                {
                                    break;
                                }
                            }
                        }
                    }
                };
                tokio::select! {
                    _ = tx.closed() => {},
                    _ = tokio::time::timeout(timeout, produce) => {},
                }
            });
            let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
            let request = this.request(stream);
            let mut client = this.wire();
            let result = match client.write_object(request).await {
                Ok(response) => response.into_inner(),
                Err(error)
                    if matches!(
                        error.code(),
                        Code::Unavailable
                            | Code::Unknown
                            | Code::DeadlineExceeded
                            | Code::Cancelled
                            | Code::Internal
                            | Code::DataLoss
                    ) =>
                {
                    return Ok(WriteOutcome::Unknown);
                }
                Err(error) => return Err(error),
            };
            match wire::upload_result::Outcome::try_from(result.outcome) {
                Ok(wire::upload_result::Outcome::Stored) => Ok(WriteOutcome::Applied(
                    VersionToken::new(result.version)
                        .map_err(|_| Status::data_loss("upload version token"))?,
                )),
                Ok(wire::upload_result::Outcome::AlreadyPresent) => {
                    Ok(WriteOutcome::PreconditionFailed)
                }
                Ok(wire::upload_result::Outcome::Unknown) => Ok(WriteOutcome::Unknown),
                _ => Err(Status::data_loss("invalid upload outcome")),
            }
        }
    }
}

/// Only native immutable paths are accepted. This adapter cannot read control
/// records, address other repositories, or issue mutable conditional writes.
#[derive(Clone)]
pub struct RemoteObjects(pub NativeClient);
impl RemoteObjects {
    fn key(&self, path: &Key) -> storage::Result<m::ObjectKey> {
        let prefix = format!("repos/{}/objects/v1/", self.0.repository);
        let parts: Vec<_> = path
            .as_str()
            .strip_prefix(&prefix)
            .ok_or(storage::Error::Invalid("remote object scope"))?
            .split('/')
            .collect();
        if parts.len() != 3 {
            return Err(storage::Error::Invalid("remote object key"));
        }
        let kind = parts[0]
            .parse::<u8>()
            .map_err(|_| storage::Error::Invalid("remote object kind"))?;
        let id: m::CommitId = parts[2]
            .parse()
            .map_err(|_| storage::Error::Invalid("remote object ID"))?;
        let key = m::ObjectKey::from_parts(kind, *id.as_bytes())
            .ok_or(storage::Error::Invalid("remote object kind"))?;
        if key.storage_path(self.0.repository) != path.as_str() {
            return Err(storage::Error::Invalid("noncanonical remote key"));
        }
        Ok(key)
    }
}
fn storage_error(status: Status) -> storage::Error {
    match status.code() {
        Code::NotFound => storage::Error::NotFound,
        Code::ResourceExhausted => storage::Error::Limit,
        Code::InvalidArgument => storage::Error::Invalid("native RPC request"),
        Code::PermissionDenied | Code::Unauthenticated => storage::Error::Rejected(403),
        Code::DataLoss => storage::Error::Response("native RPC integrity"),
        _ => storage::Error::Transport,
    }
}
#[async_trait::async_trait]
impl ObjectStore for RemoteObjects {
    async fn head(&self, key: &Key) -> storage::Result<Metadata> {
        self.0.head(self.key(key)?).await.map_err(storage_error)
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> storage::Result<Download> {
        let mut download = self.0.read(self.key(key)?).await.map_err(storage_error)?;
        if let Some(range) = range {
            if range.end() > download.metadata.len {
                return Err(storage::Error::Range);
            }
            // Range is a transport optimization only. Native readers request and
            // verify complete objects; no partial response claims full integrity.
            let copied = tokio::io::copy(
                &mut download.reader.as_mut().take(range.start()),
                &mut tokio::io::sink(),
            )
            .await?;
            if copied != range.start() {
                return Err(storage::Error::Response("range truncated"));
            }
            download.metadata.len = range.len();
            download.reader = Box::pin(download.reader.take(range.len()));
        }
        Ok(download)
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> storage::Result<WriteOutcome> {
        let key = self.key(key)?;
        if !matches!(condition, Condition::Absent) {
            return Err(storage::Error::Invalid("remote mutable writes disabled"));
        }
        self.0.upload(key, upload).await.map_err(storage_error)
    }
}
