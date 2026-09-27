use crate::{
    CHUNK_BYTES, MAX_IDS, REQUEST_BYTES, RESPONSE_BYTES, object, service_error,
    wire::{
        self,
        native_store_server::{NativeStore, NativeStoreServer},
    },
};
use futures::{Stream, StreamExt as _};
use jj_store_model as m;
use jj_store_object::{StagedUpload, WriteOutcome};
use jj_store_service::{
    auth::BearerToken,
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
    slots: Arc<Semaphore>,
    limits: Limits,
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
        Ok(Self {
            repositories: Arc::new(registry),
            slots: Arc::new(Semaphore::new(limits.concurrent)),
            limits,
        })
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
    fn bind(&self, repository: &[u8], credential: BearerToken) -> Result<NativeRequest, Status> {
        let repository = m::RepositoryId::from_slice(repository)
            .map_err(|_| Status::invalid_argument("repository ID length"))?;
        self.repositories
            .get(&repository)
            .ok_or_else(|| Status::permission_denied("repository access denied"))?
            .bind(credential)
            .map_err(service_error)
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
}
#[tonic::async_trait]
impl NativeStore for NativeRpc {
    async fn authorize(
        &self,
        request: Request<wire::AccessRequest>,
    ) -> Result<Response<wire::Empty>, Status> {
        let token = Self::credential(request.metadata())?;
        let r = request.into_inner();
        let session = self.bind(&r.repository, token)?;
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
            Self::credential(request.metadata())?,
        )?;
        self.bounded(async {
            let ids = session.heads().await.map_err(service_error)?;
            Ok(wire::Ids {
                ids: ids.into_iter().map(|id| id.as_bytes().to_vec()).collect(),
            })
        })
        .await
    }
    async fn inventory(
        &self,
        request: Request<wire::InventoryRequest>,
    ) -> Result<Response<wire::Ids>, Status> {
        let session = self.bind(
            &request.get_ref().repository,
            Self::credential(request.metadata())?,
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
            Self::credential(request.metadata())?,
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
    async fn head_object(
        &self,
        request: Request<wire::ObjectRequest>,
    ) -> Result<Response<wire::ObjectMetadata>, Status> {
        let session = self.bind(
            &request.get_ref().repository,
            Self::credential(request.metadata())?,
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
            Self::credential(request.metadata())?,
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
        let token = Self::credential(request.metadata())?;
        let mut stream = request.into_inner();
        self.bounded(async {
            let first = stream
                .message()
                .await?
                .ok_or_else(|| Status::invalid_argument("upload header required"))?;
            let Some(wire::upload_chunk::Part::Start(start)) = first.part else {
                return Err(Status::invalid_argument("upload header required"));
            };
            let session = self.bind(&start.repository, token)?;
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
