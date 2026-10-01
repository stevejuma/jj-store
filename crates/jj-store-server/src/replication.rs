//! Background writable-replica composition and native control plane.
use crate::{
    Error, Result,
    config::{Replication, ReplicationLimits},
    policy,
};
use async_trait::async_trait;
use jj_store_adapters::Health as SearchHealth;
use jj_store_model as m;
use jj_store_object::{Download, ObjectStore, StagedUpload, WriteOutcome};
use jj_store_replication::{
    DirectionStatus, EndpointFailure, EndpointResult, FaultKind, FeedCursorV1, PeerCheckpointStore,
    ReplicaCapabilitiesV1, ReplicationBarrier, ReplicationEndpoint, ReplicationWorker,
    WorkerHandle, WorkerLimits,
};
use jj_store_rpc::{
    client::{NativeClient, Options},
    server::{ReplicationControlError, ReplicationControls},
};
use jj_store_service::{
    native::ReplicaDescriptor,
    replication_control as control,
    transport::{self, NativeRequest},
};
use std::{
    collections::BTreeMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use tokio::sync::{Notify, watch};
use tonic::Code;
use zeroize::Zeroizing;

#[derive(Clone)]
struct LocalEndpoint {
    repository: m::RepositoryId,
    request: Arc<NativeRequest>,
}

impl LocalEndpoint {
    async fn descriptor(&self) -> EndpointResult<ReplicaDescriptor> {
        self.request
            .replica_descriptor()
            .await
            .map_err(|error| local_error("local descriptor", error))
    }

    async fn heads(&self) -> EndpointResult<std::collections::BTreeSet<m::OperationId>> {
        self.request
            .heads()
            .await
            .map_err(|error| local_error("local heads", error))
    }
}

fn local_error(operation: &'static str, error: transport::Error) -> EndpointFailure {
    match error {
        transport::Error::Unavailable
        | transport::Error::Unknown
        | transport::Error::Conflict
        | transport::Error::Limit => EndpointFailure::transient(operation),
        transport::Error::Unsupported
        | transport::Error::Denied
        | transport::Error::Invalid
        | transport::Error::NotFound => EndpointFailure::permanent(operation),
    }
}

#[async_trait]
impl ReplicationEndpoint for LocalEndpoint {
    fn repository(&self) -> m::RepositoryId {
        self.repository
    }

    async fn negotiate(
        &self,
        request: &jj_store_replication::NegotiateReplicaV1,
    ) -> EndpointResult<jj_store_replication::ReplicaNegotiationV1> {
        self.request
            .negotiate_replica(request)
            .await
            .map_err(|error| local_error("local negotiate", error))
    }

    async fn feed(
        &self,
        request: &jj_store_replication::FeedRequestV1,
    ) -> EndpointResult<jj_store_replication::FeedResponseV1> {
        self.request
            .replication_feed(request)
            .await
            .map_err(|error| local_error("local feed", error))
    }

    async fn offer(
        &self,
        request: &jj_store_replication::OfferOriginsRequestV1,
    ) -> EndpointResult<jj_store_replication::OfferOriginsResponseV1> {
        self.request
            .offer_origins(request)
            .await
            .map_err(|error| local_error("local offer", error))
    }

    async fn closure(
        &self,
        request: &jj_store_replication::OriginClosureRequestV1,
    ) -> EndpointResult<jj_store_replication::OriginClosureResponseV1> {
        self.request
            .origin_closure(request)
            .await
            .map_err(|error| local_error("local closure", error))
    }

    async fn read_native(&self, key: m::ObjectKey) -> EndpointResult<Download> {
        self.request
            .read(key)
            .await
            .map_err(|error| local_error("local native read", error))
    }

    async fn write_native(
        &self,
        key: m::ObjectKey,
        upload: &StagedUpload,
    ) -> EndpointResult<WriteOutcome> {
        self.request
            .upload(key, upload)
            .await
            .map_err(|error| local_error("local native write", error))
    }

    async fn read_control(&self, reference: transport::RecordRef) -> EndpointResult<Vec<u8>> {
        self.request
            .read_replication_record(reference)
            .await
            .map_err(|error| local_error("local control read", error))
    }

    async fn write_control(
        &self,
        reference: transport::RecordRef,
        bytes: Vec<u8>,
    ) -> EndpointResult<WriteOutcome> {
        self.request
            .write_replication_record(reference, &bytes)
            .await
            .map_err(|error| local_error("local control write", error))
    }

    async fn apply(
        &self,
        request: &jj_store_replication::ApplyOriginsTransportRequestV1,
    ) -> EndpointResult<jj_store_replication::ApplyOriginsTransportResponseV1> {
        self.request
            .apply_origins(request)
            .await
            .map_err(|error| local_error("local apply", error))
    }
}

#[derive(Clone)]
struct RemoteEndpoint {
    repository: m::RepositoryId,
    upstream: String,
    token_environment: String,
    options: Options,
    token_override: Option<jj_store_service::auth::BearerToken>,
    client: Arc<tokio::sync::Mutex<Option<NativeClient>>>,
}

impl RemoteEndpoint {
    fn new(config: &Replication, ca_certificate_pem: Option<Vec<u8>>) -> Self {
        Self {
            repository: config.repository,
            upstream: config.upstream.clone(),
            token_environment: config.token_environment.clone(),
            options: Options {
                insecure_loopback: config.insecure_loopback,
                ca_certificate_pem,
                timeout: Duration::from_secs(config.limits.timeout_seconds),
                object_bytes: config.limits.max_native_object_bytes,
            },
            token_override: None,
            client: Arc::new(tokio::sync::Mutex::new(None)),
        }
    }

    async fn client(&self) -> EndpointResult<NativeClient> {
        let mut slot = self.client.lock().await;
        if let Some(client) = slot.as_ref() {
            return Ok(client.clone());
        }
        let token = match &self.token_override {
            Some(token) => token.clone(),
            None => {
                let credential = Zeroizing::new(
                    std::env::var(&self.token_environment)
                        .map_err(|_| EndpointFailure::permanent("upstream credential"))?,
                );
                jj_store_service::auth::BearerToken::parse(&credential)
                    .map_err(|_| EndpointFailure::permanent("upstream credential"))?
            }
        };
        let client =
            NativeClient::connect(&self.upstream, self.repository, token, self.options.clone())
                .await
                .map_err(|status| remote_error("upstream connect", &status))?;
        *slot = Some(client.clone());
        Ok(client)
    }

    async fn failure(&self, operation: &'static str, status: tonic::Status) -> EndpointFailure {
        *self.client.lock().await = None;
        remote_error(operation, &status)
    }
}

fn remote_error(operation: &'static str, status: &tonic::Status) -> EndpointFailure {
    if matches!(
        status.code(),
        Code::Cancelled
            | Code::Unknown
            | Code::DeadlineExceeded
            | Code::ResourceExhausted
            | Code::Aborted
            | Code::Internal
            | Code::Unavailable
    ) {
        EndpointFailure::transient(operation)
    } else {
        EndpointFailure::permanent(operation)
    }
}

#[async_trait]
impl ReplicationEndpoint for RemoteEndpoint {
    fn repository(&self) -> m::RepositoryId {
        self.repository
    }

    async fn negotiate(
        &self,
        request: &jj_store_replication::NegotiateReplicaV1,
    ) -> EndpointResult<jj_store_replication::ReplicaNegotiationV1> {
        let client = self.client().await?;
        match client.negotiate_replica(request).await {
            Ok(value) => Ok(value),
            Err(status) => Err(self.failure("upstream negotiate", status).await),
        }
    }

    async fn feed(
        &self,
        request: &jj_store_replication::FeedRequestV1,
    ) -> EndpointResult<jj_store_replication::FeedResponseV1> {
        let client = self.client().await?;
        match client.committed_feed(request).await {
            Ok(value) => Ok(value),
            Err(status) => Err(self.failure("upstream feed", status).await),
        }
    }

    async fn offer(
        &self,
        request: &jj_store_replication::OfferOriginsRequestV1,
    ) -> EndpointResult<jj_store_replication::OfferOriginsResponseV1> {
        let client = self.client().await?;
        match client.offer_origins(request).await {
            Ok(value) => Ok(value),
            Err(status) => Err(self.failure("upstream offer", status).await),
        }
    }

    async fn closure(
        &self,
        request: &jj_store_replication::OriginClosureRequestV1,
    ) -> EndpointResult<jj_store_replication::OriginClosureResponseV1> {
        let client = self.client().await?;
        match client.origin_closure(request).await {
            Ok(value) => Ok(value),
            Err(status) => Err(self.failure("upstream closure", status).await),
        }
    }

    async fn read_native(&self, key: m::ObjectKey) -> EndpointResult<Download> {
        let client = self.client().await?;
        match client.read(key).await {
            Ok(value) => Ok(value),
            Err(status) => Err(self.failure("upstream native read", status).await),
        }
    }

    async fn write_native(
        &self,
        key: m::ObjectKey,
        upload: &StagedUpload,
    ) -> EndpointResult<WriteOutcome> {
        let client = self.client().await?;
        match client.upload(key, upload).await {
            Ok(value) => Ok(value),
            Err(status) => Err(self.failure("upstream native write", status).await),
        }
    }

    async fn read_control(&self, reference: transport::RecordRef) -> EndpointResult<Vec<u8>> {
        let client = self.client().await?;
        match client.read_control_record(reference).await {
            Ok(value) => Ok(value),
            Err(status) => Err(self.failure("upstream control read", status).await),
        }
    }

    async fn write_control(
        &self,
        reference: transport::RecordRef,
        bytes: Vec<u8>,
    ) -> EndpointResult<WriteOutcome> {
        let client = self.client().await?;
        match client.write_control_record(reference, bytes).await {
            Ok(value) => Ok(value),
            Err(status) => Err(self.failure("upstream control write", status).await),
        }
    }

    async fn apply(
        &self,
        request: &jj_store_replication::ApplyOriginsTransportRequestV1,
    ) -> EndpointResult<jj_store_replication::ApplyOriginsTransportResponseV1> {
        let client = self.client().await?;
        match client.apply_origins(request).await {
            Ok(value) => Ok(value),
            Err(status) => Err(self.failure("upstream apply", status).await),
        }
    }
}

#[derive(Clone)]
struct RuntimeState {
    phase: control::Phase,
    handle: Option<WorkerHandle>,
    local: Option<ReplicaDescriptor>,
    peer: Option<jj_store_replication::ReplicaNegotiationV1>,
}

struct Entry {
    repository: m::RepositoryId,
    local: LocalEndpoint,
    remote: RemoteEndpoint,
    store: Arc<dyn ObjectStore>,
    limits: WorkerLimits,
    wait_limit: Duration,
    paused: AtomicBool,
    wake: Notify,
    state: Mutex<RuntimeState>,
    search: Mutex<Option<watch::Receiver<SearchHealth>>>,
}

pub struct PreparedReplication {
    controls: Arc<ControlPlane>,
}

impl PreparedReplication {
    pub fn new(
        host: &jj_store_service::administration::RepositoryHost,
        store: Arc<dyn ObjectStore>,
        configs: &[Replication],
        principal: m::PrincipalId,
        search: BTreeMap<m::RepositoryId, watch::Receiver<SearchHealth>>,
    ) -> Result<Self> {
        let mut entries = BTreeMap::new();
        for config in configs {
            let ca_certificate_pem = config
                .ca_certificate
                .as_deref()
                .map(|path| policy::read_regular(path, 1024 * 1024))
                .transpose()?;
            let request = host
                .endpoint(config.repository)
                .and_then(|endpoint| {
                    endpoint
                        .bind_local_trusted(principal, jj_store_service::auth::Permission::Write)
                })
                .map_err(|_| Error::Config("replication local endpoint"))?;
            let entry = Arc::new(Entry {
                repository: config.repository,
                local: LocalEndpoint {
                    repository: config.repository,
                    request: Arc::new(request),
                },
                remote: RemoteEndpoint::new(config, ca_certificate_pem),
                store: store.clone(),
                limits: worker_limits(config.limits),
                wait_limit: Duration::from_secs(config.limits.timeout_seconds),
                paused: AtomicBool::new(false),
                wake: Notify::new(),
                state: Mutex::new(RuntimeState {
                    phase: control::Phase::Stopped,
                    handle: None,
                    local: None,
                    peer: None,
                }),
                search: Mutex::new(search.get(&config.repository).cloned()),
            });
            if entries.insert(config.repository, entry).is_some() {
                return Err(Error::Config("duplicate replication repository"));
            }
        }
        Ok(Self {
            controls: Arc::new(ControlPlane { entries }),
        })
    }

    pub(crate) fn search_observer(&self) -> crate::search_management::Observer {
        let controls = self.controls.clone();
        Arc::new(move |repo, health| {
            if let Some(entry) = controls.entries.get(&repo) {
                *entry.search.lock().unwrap() = health;
            }
        })
    }
    pub fn controls(&self) -> Arc<dyn ReplicationControls> {
        self.controls.clone()
    }

    pub fn start(&self) -> RunningReplication {
        let (shutdown, receiver) = watch::channel(false);
        let tasks = self
            .controls
            .entries
            .values()
            .cloned()
            .map(|entry| {
                let receiver = receiver.clone();
                tokio::spawn(async move { run_entry(entry, receiver).await })
            })
            .collect();
        RunningReplication { shutdown, tasks }
    }
}

pub struct RunningReplication {
    shutdown: watch::Sender<bool>,
    tasks: Vec<tokio::task::JoinHandle<()>>,
}

impl RunningReplication {
    pub async fn shutdown(mut self) -> Result<()> {
        self.shutdown.send_replace(true);
        let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
        for task in &mut self.tasks {
            if tokio::time::timeout_at(deadline, &mut *task).await.is_err() {
                task.abort();
                return Err(Error::Service("replication shutdown exceeded five seconds"));
            }
        }
        self.tasks.clear();
        Ok(())
    }
}

impl Drop for RunningReplication {
    fn drop(&mut self) {
        self.shutdown.send_replace(true);
        for task in &self.tasks {
            task.abort();
        }
    }
}

struct ControlPlane {
    entries: BTreeMap<m::RepositoryId, Arc<Entry>>,
}

impl ControlPlane {
    fn entry(
        &self,
        repository: m::RepositoryId,
    ) -> std::result::Result<Arc<Entry>, ReplicationControlError> {
        self.entries
            .get(&repository)
            .cloned()
            .ok_or(ReplicationControlError::NotConfigured)
    }
}

#[async_trait]
impl ReplicationControls for ControlPlane {
    async fn status(
        &self,
        repository: m::RepositoryId,
    ) -> std::result::Result<control::StatusV1, ReplicationControlError> {
        status(&self.entry(repository)?).await
    }

    async fn wake(
        &self,
        repository: m::RepositoryId,
    ) -> std::result::Result<control::StatusV1, ReplicationControlError> {
        let entry = self.entry(repository)?;
        let handle = entry
            .state
            .lock()
            .expect("replication state")
            .handle
            .clone();
        if let Some(handle) = handle {
            handle.wake();
        }
        entry.wake.notify_one();
        status(&entry).await
    }

    async fn set_paused(
        &self,
        repository: m::RepositoryId,
        paused: bool,
    ) -> std::result::Result<control::StatusV1, ReplicationControlError> {
        let entry = self.entry(repository)?;
        entry.paused.store(paused, Ordering::SeqCst);
        let handle = entry
            .state
            .lock()
            .expect("replication state")
            .handle
            .clone();
        if let Some(handle) = handle {
            handle.set_paused(paused);
        }
        entry.wake.notify_one();
        status(&entry).await
    }

    async fn capture_barrier(
        &self,
        repository: m::RepositoryId,
    ) -> std::result::Result<control::BarrierV1, ReplicationControlError> {
        let entry = self.entry(repository)?;
        let local = entry
            .local
            .descriptor()
            .await
            .map_err(|_| ReplicationControlError::Unavailable)?;
        let peer = entry
            .remote
            .negotiate(&jj_store_replication::NegotiateReplicaV1 {
                schema_version: 1,
                repository,
                peer: local.replica,
                capabilities: ReplicaCapabilitiesV1::default(),
            })
            .await
            .map_err(|_| ReplicationControlError::Unavailable)?;
        Ok(control::BarrierV1 {
            version: 1,
            repository,
            local_replica: local.replica,
            peer_replica: peer.replica,
            push_epoch: local.log_epoch,
            push_through: local.feed_upper_bound,
            pull_epoch: peer.log_epoch,
            pull_through: peer.feed_upper_bound,
        })
    }

    async fn wait(
        &self,
        repository: m::RepositoryId,
        request: &control::WaitRequestV1,
    ) -> std::result::Result<control::WaitOutcomeV1, ReplicationControlError> {
        let entry = self.entry(repository)?;
        if request.version != 1
            || request.barrier.version != 1
            || request.barrier.repository != repository
            || request.timeout_millis == 0
            || Duration::from_millis(request.timeout_millis) > entry.wait_limit
        {
            return Err(ReplicationControlError::Invalid);
        }
        let (handle, local, peer) = {
            let state = entry.state.lock().expect("replication state");
            (state.handle.clone(), state.local, state.peer.clone())
        };
        let handle = handle.ok_or(ReplicationControlError::Unavailable)?;
        if local.map(|value| value.replica) != Some(request.barrier.local_replica)
            || peer.map(|value| value.replica) != Some(request.barrier.peer_replica)
        {
            return Err(ReplicationControlError::Invalid);
        }
        handle
            .wait_for_barrier(
                ReplicationBarrier {
                    repository,
                    local_replica: request.barrier.local_replica,
                    peer_replica: request.barrier.peer_replica,
                    push_epoch: request.barrier.push_epoch,
                    push_through: request.barrier.push_through,
                    pull_epoch: request.barrier.pull_epoch,
                    pull_through: request.barrier.pull_through,
                },
                Duration::from_millis(request.timeout_millis),
            )
            .await
            .map_err(|error| match error {
                jj_store_replication::Error::Limit("replication barrier timeout") => {
                    ReplicationControlError::Timeout
                }
                _ => ReplicationControlError::Unavailable,
            })?;
        Ok(control::WaitOutcomeV1 {
            version: 1,
            barrier: request.barrier,
            status: status(&entry).await?,
        })
    }
}

async fn status(
    entry: &Arc<Entry>,
) -> std::result::Result<control::StatusV1, ReplicationControlError> {
    let descriptor = entry.local.descriptor().await.ok();
    let heads = entry.local.heads().await.ok();
    let state = entry.state.lock().expect("replication state").clone();
    let worker = state
        .handle
        .as_ref()
        .map(WorkerHandle::status)
        .unwrap_or_default();
    let paused = entry.paused.load(Ordering::SeqCst);
    let phase = if paused {
        control::Phase::Paused
    } else if state.handle.is_some()
        && (worker.pull.last_fault.is_some() || worker.push.last_fault.is_some())
    {
        control::Phase::Degraded
    } else if state.handle.is_some() {
        control::Phase::Running
    } else {
        state.phase
    };
    let local_frontier = descriptor.map(|value| control::FrontierV1 {
        replica: value.replica,
        log_epoch: value.log_epoch,
        through: value.feed_upper_bound,
        observed_upper_bound: value.feed_upper_bound,
    });
    let pushed = worker.pushed.map(frontier);
    let pulled = worker.pulled.map(frontier);
    let peer_frontier = worker
        .pulled
        .map(|value| control::FrontierV1 {
            replica: value.issuer,
            log_epoch: value.log_epoch,
            through: value.observed_upper_bound,
            observed_upper_bound: value.observed_upper_bound,
        })
        .or_else(|| {
            state.peer.as_ref().map(|value| control::FrontierV1 {
                replica: value.replica,
                log_epoch: value.log_epoch,
                through: value.feed_upper_bound,
                observed_upper_bound: value.feed_upper_bound,
            })
        });
    let local_pending_positions = local_frontier.map(|current| {
        pushed
            .filter(|value| {
                value.replica == current.replica && value.log_epoch == current.log_epoch
            })
            .map_or(current.through.0, |value| {
                current.through.0.saturating_sub(value.through.0)
            })
    });
    let peer_pending_positions =
        pulled.map(|value| value.observed_upper_bound.0.saturating_sub(value.through.0));
    let search = entry.search.lock().unwrap().as_ref().map_or(
        control::SearchStatusV1 {
            configured: false,
            repository_position: None,
            indexed_position: None,
            degraded: false,
        },
        |receiver| {
            let health = receiver.borrow().clone();
            control::SearchStatusV1 {
                configured: true,
                repository_position: health.repository_position.map(|value| value.position),
                indexed_position: health.applied.map(|value| value.source.position),
                degraded: health.degraded,
            }
        },
    );
    Ok(control::StatusV1 {
        version: 1,
        repository: entry.repository,
        local_ready: descriptor.is_some(),
        offline_ready: descriptor.is_some(),
        phase,
        local_replica: descriptor.map(|value| value.replica),
        peer_replica: state.peer.map(|value| value.replica),
        local_frontier,
        peer_frontier,
        pushed,
        pulled,
        local_pending_positions,
        peer_pending_positions,
        checkpoint_revision: worker.checkpoint_revision,
        operation_heads: heads.as_ref().map_or(0, std::collections::BTreeSet::len),
        reconciliation_required: heads.as_ref().is_some_and(|values| values.len() > 1),
        pull: direction(worker.pull),
        push: direction(worker.push),
        search,
    })
}

fn frontier(value: FeedCursorV1) -> control::FrontierV1 {
    control::FrontierV1 {
        replica: value.issuer,
        log_epoch: value.log_epoch,
        through: value.through,
        observed_upper_bound: value.observed_upper_bound,
    }
}

fn direction(value: DirectionStatus) -> control::DirectionStatusV1 {
    control::DirectionStatusV1 {
        consecutive_failures: value.consecutive_failures,
        circuit_open: value.circuit_open,
        last_fault: value.last_fault.map(|fault| match fault {
            FaultKind::TransientEndpoint => control::Fault::TransientEndpoint,
            FaultKind::PermanentEndpoint => control::Fault::PermanentEndpoint,
            FaultKind::Integrity => control::Fault::Integrity,
            FaultKind::Limit => control::Fault::Limit,
            FaultKind::CursorConflict => control::Fault::CursorConflict,
            FaultKind::OutcomeUnknown => control::Fault::OutcomeUnknown,
        }),
    }
}

async fn run_entry(entry: Arc<Entry>, mut shutdown: watch::Receiver<bool>) {
    loop {
        if *shutdown.borrow() {
            set_stopped(&entry);
            return;
        }
        if entry.paused.load(Ordering::SeqCst) {
            entry.state.lock().expect("replication state").phase = control::Phase::Paused;
            tokio::select! {
                changed=shutdown.changed()=>if changed.is_err(){set_stopped(&entry);return;},
                _=entry.wake.notified()=>{},
            }
            continue;
        }
        entry.state.lock().expect("replication state").phase = control::Phase::Connecting;
        let local = match entry.local.descriptor().await {
            Ok(value) => value,
            Err(_) => {
                entry.state.lock().expect("replication state").phase = control::Phase::Degraded;
                if wait_retry(&entry, &mut shutdown).await {
                    return;
                }
                continue;
            }
        };
        let peer = match entry
            .remote
            .negotiate(&jj_store_replication::NegotiateReplicaV1 {
                schema_version: 1,
                repository: entry.repository,
                peer: local.replica,
                capabilities: ReplicaCapabilitiesV1::default(),
            })
            .await
        {
            Ok(value)
                if value.repository == entry.repository
                    && value.replica != local.replica
                    && value.feed_lower_bound <= value.feed_upper_bound
                    && ReplicaCapabilitiesV1::default().compatible_with(&value.capabilities) =>
            {
                value
            }
            _ => {
                entry.state.lock().expect("replication state").phase = control::Phase::Degraded;
                if wait_retry(&entry, &mut shutdown).await {
                    return;
                }
                continue;
            }
        };
        let checkpoints = match PeerCheckpointStore::new(
            entry.store.clone(),
            entry.repository,
            local.replica,
            peer.replica,
        ) {
            Ok(value) => value,
            Err(_) => {
                entry.state.lock().expect("replication state").phase = control::Phase::Degraded;
                if wait_retry(&entry, &mut shutdown).await {
                    return;
                }
                continue;
            }
        };
        let worker = match ReplicationWorker::new(
            entry.repository,
            local.replica,
            peer.replica,
            entry.local.clone(),
            entry.remote.clone(),
            checkpoints,
            entry.limits,
        ) {
            Ok(value) => value,
            Err(_) => {
                entry.state.lock().expect("replication state").phase = control::Phase::Degraded;
                if wait_retry(&entry, &mut shutdown).await {
                    return;
                }
                continue;
            }
        };
        let handle = worker.handle();
        if entry.paused.load(Ordering::SeqCst) {
            handle.set_paused(true);
        }
        {
            let mut state = entry.state.lock().expect("replication state");
            state.phase = control::Phase::Running;
            state.handle = Some(handle);
            state.local = Some(local);
            state.peer = Some(peer);
        }
        let _ = worker.run(shutdown.clone()).await;
        {
            let mut state = entry.state.lock().expect("replication state");
            state.handle = None;
            state.phase = if *shutdown.borrow() {
                control::Phase::Stopped
            } else {
                control::Phase::Degraded
            };
        }
        if *shutdown.borrow() {
            return;
        }
        if wait_retry(&entry, &mut shutdown).await {
            return;
        }
    }
}

async fn wait_retry(entry: &Entry, shutdown: &mut watch::Receiver<bool>) -> bool {
    tokio::select! {
        changed=shutdown.changed()=>changed.is_err() || *shutdown.borrow(),
        _=entry.wake.notified()=>false,
        _=tokio::time::sleep(entry.limits.retry_base)=>false,
    }
}

fn set_stopped(entry: &Entry) {
    let mut state = entry.state.lock().expect("replication state");
    state.handle = None;
    state.phase = control::Phase::Stopped;
}

fn worker_limits(limits: ReplicationLimits) -> WorkerLimits {
    WorkerLimits {
        feed_entries: limits.max_batch_origins,
        closure_entries: limits.max_closure_entries,
        max_objects_per_step: limits.max_objects_per_step,
        max_bytes_per_step: limits.max_bytes_per_step,
        max_native_object_bytes: limits.max_native_object_bytes,
        poll_interval: Duration::from_millis(limits.poll_millis),
        retry_base: Duration::from_millis(limits.retry_initial_millis),
        retry_max: Duration::from_secs(limits.retry_max_seconds),
        circuit_failures: limits.circuit_failures,
        circuit_cooldown: Duration::from_secs(limits.circuit_cooldown_seconds),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jj_store_jj::{objects::NativeObjects, policy::Protection};
    use jj_store_object::memory::MemoryStore;
    use jj_store_service::{
        auth::{BearerToken, Permission, SystemClock},
        native::{SessionLimits, create_repository_local_trusted},
        transport::NativeEndpoint,
    };

    #[tokio::test]
    async fn upstream_outage_keeps_local_authority_ready_and_controls_responsive() {
        let repository = m::RepositoryId::new([111; 16]);
        let store = Arc::new(MemoryStore::default());
        let object_store: Arc<dyn ObjectStore> = store.clone();
        let objects =
            Arc::new(NativeObjects::new(object_store.clone(), repository, 1024 * 1024).unwrap());
        create_repository_local_trusted(
            object_store.clone(),
            objects,
            Permission::Admin,
            m::EpochId::new([112; 16]),
            Default::default(),
        )
        .await
        .unwrap();
        let request = NativeEndpoint::new(
            repository,
            object_store.clone(),
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
            1024 * 1024,
        )
        .unwrap()
        .bind_local_trusted(m::PrincipalId::new([113; 16]), Permission::Write)
        .unwrap();
        let local = LocalEndpoint {
            repository,
            request: Arc::new(request),
        };
        let entry = Arc::new(Entry {
            repository,
            local: local.clone(),
            remote: RemoteEndpoint {
                repository,
                upstream: "http://127.0.0.1:9/".into(),
                token_environment: "JJ_STORE_REPLICATION_TOKEN_OUTAGE_FIXTURE".into(),
                options: Options {
                    insecure_loopback: true,
                    timeout: Duration::from_millis(20),
                    object_bytes: 1024 * 1024,
                    ..Options::default()
                },
                token_override: Some(BearerToken::generate().unwrap()),
                client: Arc::new(tokio::sync::Mutex::new(None)),
            },
            store: object_store,
            limits: WorkerLimits {
                poll_interval: Duration::from_millis(10),
                retry_base: Duration::from_millis(10),
                retry_max: Duration::from_millis(20),
                circuit_cooldown: Duration::from_millis(20),
                ..WorkerLimits::default()
            },
            wait_limit: Duration::from_secs(1),
            paused: AtomicBool::new(false),
            wake: Notify::new(),
            state: Mutex::new(RuntimeState {
                phase: control::Phase::Stopped,
                handle: None,
                local: None,
                peer: None,
            }),
            search: Mutex::new(None),
        });
        let controls = ControlPlane {
            entries: BTreeMap::from([(repository, entry.clone())]),
        };
        let (shutdown, receiver) = watch::channel(false);
        let task = tokio::spawn(run_entry(entry, receiver));

        tokio::time::sleep(Duration::from_millis(80)).await;
        assert_eq!(local.heads().await.unwrap().len(), 1);
        let available = controls.status(repository).await.unwrap();
        assert!(available.local_ready && available.offline_ready);
        assert!(matches!(
            available.phase,
            control::Phase::Connecting | control::Phase::Degraded
        ));
        assert_eq!(
            controls.set_paused(repository, true).await.unwrap().phase,
            control::Phase::Paused
        );
        assert!(controls.wake(repository).await.unwrap().local_ready);
        controls.set_paused(repository, false).await.unwrap();

        shutdown.send_replace(true);
        // Wake both discovery and any future worker wait without contacting the peer.
        controls.wake(repository).await.unwrap();
        tokio::time::timeout(Duration::from_secs(1), task)
            .await
            .unwrap()
            .unwrap();
    }
}
