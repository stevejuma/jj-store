use jj_store_jj::{objects::NativeObjects, policy::Protection, roots};
use jj_store_model as m;
use jj_store_object::memory::MemoryStore;
use jj_store_rpc::{
    client::{NativeClient, Options},
    server::{Limits as RpcLimits, NativeRpc, ReplicationControlError, ReplicationControls},
};
use jj_store_service::{
    auth::{BearerToken, Permission, Scope, SystemClock, TokenPolicy},
    native::{SessionLimits, create_repository},
    replication_control as control,
    transport::NativeEndpoint,
};
use jj_store_wal::publication::Limits as PublicationLimits;
use std::{sync::Arc, time::Duration};

const REPOSITORY: m::RepositoryId = m::RepositoryId::new([101; 16]);
const LOCAL: m::ReplicaId = m::ReplicaId::new([102; 16]);
const PEER: m::ReplicaId = m::ReplicaId::new([103; 16]);
const LOCAL_EPOCH: m::EpochId = m::EpochId::new([104; 16]);
const PEER_EPOCH: m::EpochId = m::EpochId::new([105; 16]);

#[derive(Default)]
struct Controls {
    wakes: std::sync::atomic::AtomicUsize,
    paused: std::sync::atomic::AtomicBool,
}

impl Controls {
    fn snapshot_status(&self) -> control::StatusV1 {
        control::StatusV1 {
            version: 1,
            repository: REPOSITORY,
            local_ready: true,
            offline_ready: true,
            phase: if self.paused.load(std::sync::atomic::Ordering::SeqCst) {
                control::Phase::Paused
            } else {
                control::Phase::Running
            },
            local_replica: Some(LOCAL),
            peer_replica: Some(PEER),
            local_frontier: Some(control::FrontierV1 {
                replica: LOCAL,
                log_epoch: LOCAL_EPOCH,
                through: m::WalPosition(7),
                observed_upper_bound: m::WalPosition(7),
            }),
            peer_frontier: Some(control::FrontierV1 {
                replica: PEER,
                log_epoch: PEER_EPOCH,
                through: m::WalPosition(9),
                observed_upper_bound: m::WalPosition(9),
            }),
            pushed: None,
            pulled: None,
            local_pending_positions: Some(7),
            peer_pending_positions: Some(9),
            checkpoint_revision: 0,
            operation_heads: 1,
            reconciliation_required: false,
            pull: control::DirectionStatusV1::default(),
            push: control::DirectionStatusV1::default(),
            search: control::SearchStatusV1 {
                configured: true,
                repository_position: Some(m::WalPosition(7)),
                indexed_position: Some(m::WalPosition(6)),
                degraded: false,
            },
        }
    }

    fn barrier(&self) -> control::BarrierV1 {
        control::BarrierV1 {
            version: 1,
            repository: REPOSITORY,
            local_replica: LOCAL,
            peer_replica: PEER,
            push_epoch: LOCAL_EPOCH,
            push_through: m::WalPosition(7),
            pull_epoch: PEER_EPOCH,
            pull_through: m::WalPosition(9),
        }
    }
}

#[async_trait::async_trait]
impl ReplicationControls for Controls {
    async fn status(
        &self,
        repository: m::RepositoryId,
    ) -> Result<control::StatusV1, ReplicationControlError> {
        (repository == REPOSITORY)
            .then(|| self.snapshot_status())
            .ok_or(ReplicationControlError::NotConfigured)
    }

    async fn wake(
        &self,
        repository: m::RepositoryId,
    ) -> Result<control::StatusV1, ReplicationControlError> {
        self.wakes.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        ReplicationControls::status(self, repository).await
    }

    async fn set_paused(
        &self,
        repository: m::RepositoryId,
        paused: bool,
    ) -> Result<control::StatusV1, ReplicationControlError> {
        self.paused
            .store(paused, std::sync::atomic::Ordering::SeqCst);
        ReplicationControls::status(self, repository).await
    }

    async fn capture_barrier(
        &self,
        repository: m::RepositoryId,
    ) -> Result<control::BarrierV1, ReplicationControlError> {
        (repository == REPOSITORY)
            .then(|| self.barrier())
            .ok_or(ReplicationControlError::NotConfigured)
    }

    async fn wait(
        &self,
        repository: m::RepositoryId,
        request: &control::WaitRequestV1,
    ) -> Result<control::WaitOutcomeV1, ReplicationControlError> {
        if repository != REPOSITORY
            || request.version != 1
            || request.barrier != self.barrier()
            || request.timeout_millis == 0
        {
            return Err(ReplicationControlError::Invalid);
        }
        Ok(control::WaitOutcomeV1 {
            version: 1,
            barrier: request.barrier,
            status: self.snapshot_status(),
        })
    }
}

async fn endpoint() -> (NativeEndpoint, BearerToken) {
    let store = Arc::new(MemoryStore::default());
    let objects = Arc::new(NativeObjects::new(store.clone(), REPOSITORY, 1024 * 1024).unwrap());
    let principal = m::PrincipalId::new([106; 16]);
    let mut policy = TokenPolicy::new(Scope::Repository(REPOSITORY));
    let token = policy.issue(principal, Permission::Admin, None).unwrap();
    let mut operators = TokenPolicy::new(Scope::Operator);
    let operator = operators
        .issue(principal, Permission::CreateRepository, None)
        .unwrap();
    create_repository(
        store.clone(),
        objects,
        &operators,
        &operator,
        &policy,
        LOCAL_EPOCH,
        &SystemClock,
        PublicationLimits::default(),
    )
    .await
    .unwrap();
    let endpoint = NativeEndpoint::new(
        REPOSITORY,
        store,
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
        1024 * 1024,
    )
    .unwrap();
    assert_eq!(
        endpoint.bind(token.clone()).unwrap().heads().await.unwrap(),
        [roots::ROOT_OPERATION].into()
    );
    (endpoint, token)
}

#[tokio::test]
async fn native_control_rpc_preserves_finite_frontiers_and_authorized_controls() {
    let (endpoint, token) = endpoint().await;
    let controls = Arc::new(Controls::default());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let (stop, stopped) = tokio::sync::oneshot::channel();
    let service_controls = controls.clone();
    let task = tokio::spawn(async move {
        NativeRpc::new(vec![endpoint], RpcLimits::default())
            .unwrap()
            .with_replication_controls(service_controls)
            .serve(listener, None, true, async {
                let _ = stopped.await;
            })
            .await
            .unwrap();
    });
    let client = NativeClient::connect(
        &format!("http://{address}"),
        REPOSITORY,
        token,
        Options {
            insecure_loopback: true,
            timeout: Duration::from_secs(5),
            object_bytes: 1024 * 1024,
            ..Options::default()
        },
    )
    .await
    .unwrap();

    let status = client.replication_status().await.unwrap();
    assert_eq!(status.local_pending_positions, Some(7));
    assert_eq!(status.peer_pending_positions, Some(9));
    assert_ne!(
        status.local_frontier.unwrap().replica,
        status.peer_frontier.unwrap().replica
    );
    client.replication_now().await.unwrap();
    assert_eq!(controls.wakes.load(std::sync::atomic::Ordering::SeqCst), 1);
    assert_eq!(
        client.replication_pause().await.unwrap().phase,
        control::Phase::Paused
    );
    assert_eq!(
        client.replication_resume().await.unwrap().phase,
        control::Phase::Running
    );
    let barrier = client.replication_capture_barrier().await.unwrap();
    let outcome = client
        .replication_wait(&control::WaitRequestV1 {
            version: 1,
            barrier,
            timeout_millis: 500,
        })
        .await
        .unwrap();
    assert_eq!(outcome.barrier, barrier);
    assert_eq!(client.replica_inspect().await.unwrap(), outcome.status);

    stop.send(()).unwrap();
    tokio::time::timeout(Duration::from_secs(5), task)
        .await
        .unwrap()
        .unwrap();
}
