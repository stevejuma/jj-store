use crate::transfer::verify_native_upload;
use crate::{
    Error, FeedCursorV1, FeedRequestV1, OfferOriginsRequestV1, OriginClosureRequestV1,
    PeerCheckpointStore, ReplicaCapabilitiesV1, ReplicaNegotiationV1, Result, VersionedCheckpoint,
};
use async_trait::async_trait;
use jj_store_model::{
    EnvelopeDigest, EpochId, ObjectKey, PublicationId, RecordDigest, ReplicaId, RepositoryId,
    RequestId, WalPosition,
};
use jj_store_object::{Download, StagedUpload, WriteOutcome};
use jj_store_wal::{
    inventory::Representation,
    publication::{
        ApplyOriginsOutcomeV1, ApplyOriginsTransportRequestV1, ApplyOriginsTransportResponseV1,
        BootstrapCheckpointV1, FeedResponseV1, NegotiateReplicaV1, OfferOriginsResponseV1,
        OriginClosureResponseV1, OriginEnvelopeV1, OriginOfferResultV1, OriginOfferV1,
    },
    records::{MAX_RECORD_BYTES, RecordRef},
};
use sha2::{Digest as _, Sha256};
use std::{collections::BTreeSet, sync::Arc, time::Duration};
use tokio::{
    sync::{Notify, watch},
    time::Instant,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetryClass {
    Transient,
    Permanent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("{operation} ({class:?})")]
pub struct EndpointFailure {
    pub operation: &'static str,
    pub class: RetryClass,
}

impl EndpointFailure {
    pub const fn transient(operation: &'static str) -> Self {
        Self {
            operation,
            class: RetryClass::Transient,
        }
    }

    pub const fn permanent(operation: &'static str) -> Self {
        Self {
            operation,
            class: RetryClass::Permanent,
        }
    }
}

pub type EndpointResult<T> = std::result::Result<T, EndpointFailure>;

/// One authenticated repository endpoint. Implementations retain credentials
/// in process memory and map transport failures to retryable or permanent
/// classes without exposing credential or response text in worker status.
#[async_trait]
pub trait ReplicationEndpoint: Send + Sync {
    fn repository(&self) -> RepositoryId;

    async fn negotiate(&self, request: &NegotiateReplicaV1)
    -> EndpointResult<ReplicaNegotiationV1>;
    async fn feed(&self, request: &FeedRequestV1) -> EndpointResult<FeedResponseV1>;
    async fn offer(
        &self,
        request: &OfferOriginsRequestV1,
    ) -> EndpointResult<OfferOriginsResponseV1>;
    async fn closure(
        &self,
        request: &OriginClosureRequestV1,
    ) -> EndpointResult<OriginClosureResponseV1>;
    async fn read_native(&self, key: ObjectKey) -> EndpointResult<Download>;
    async fn write_native(
        &self,
        key: ObjectKey,
        upload: &StagedUpload,
    ) -> EndpointResult<WriteOutcome>;
    async fn read_control(&self, reference: RecordRef) -> EndpointResult<Vec<u8>>;
    async fn write_control(
        &self,
        reference: RecordRef,
        bytes: Vec<u8>,
    ) -> EndpointResult<WriteOutcome>;
    async fn apply(
        &self,
        request: &ApplyOriginsTransportRequestV1,
    ) -> EndpointResult<ApplyOriginsTransportResponseV1>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Pull,
    Push,
}

#[derive(Clone, Copy, Debug)]
pub struct WorkerLimits {
    pub feed_entries: usize,
    pub closure_entries: usize,
    pub max_objects_per_step: usize,
    pub max_bytes_per_step: u64,
    pub max_native_object_bytes: u64,
    pub poll_interval: Duration,
    pub retry_base: Duration,
    pub retry_max: Duration,
    pub circuit_failures: u32,
    pub circuit_cooldown: Duration,
}

impl Default for WorkerLimits {
    fn default() -> Self {
        Self {
            feed_entries: 64,
            closure_entries: 100_000,
            max_objects_per_step: 100_000,
            max_bytes_per_step: 1024 * 1024 * 1024,
            max_native_object_bytes: 64 * 1024 * 1024,
            poll_interval: Duration::from_secs(1),
            retry_base: Duration::from_millis(250),
            retry_max: Duration::from_secs(30),
            circuit_failures: 8,
            circuit_cooldown: Duration::from_secs(60),
        }
    }
}

impl WorkerLimits {
    fn validate(self) -> Result<()> {
        if !(1..=128).contains(&self.feed_entries)
            || self.closure_entries == 0
            || self.closure_entries > 1_000_000
            || self.max_objects_per_step == 0
            || self.max_bytes_per_step == 0
            || self.max_native_object_bytes == 0
            || self.max_native_object_bytes > jj_store_object::staging::MAX_SINGLE_PUT
            || !(Duration::from_millis(10)..=Duration::from_secs(60)).contains(&self.poll_interval)
            || self.retry_base.is_zero()
            || self.retry_base > self.retry_max
            || self.retry_max > Duration::from_secs(3600)
            || self.circuit_failures == 0
            || self.circuit_cooldown.is_zero()
            || self.circuit_cooldown > Duration::from_secs(3600)
        {
            return Err(Error::Limit("worker limits"));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaultKind {
    TransientEndpoint,
    PermanentEndpoint,
    Integrity,
    Limit,
    CursorConflict,
    OutcomeUnknown,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DirectionStatus {
    pub consecutive_failures: u32,
    pub circuit_open: bool,
    pub last_fault: Option<FaultKind>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkerStatus {
    pub paused: bool,
    pub active: Option<Direction>,
    pub next_direction: Direction,
    pub checkpoint_revision: u64,
    pub pulled: Option<FeedCursorV1>,
    pub pushed: Option<FeedCursorV1>,
    pub pull: DirectionStatus,
    pub push: DirectionStatus,
}

impl Default for WorkerStatus {
    fn default() -> Self {
        Self {
            paused: false,
            active: None,
            next_direction: Direction::Pull,
            checkpoint_revision: 0,
            pulled: None,
            pushed: None,
            pull: DirectionStatus::default(),
            push: DirectionStatus::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReplicationBarrier {
    pub repository: RepositoryId,
    pub local_replica: ReplicaId,
    pub peer_replica: ReplicaId,
    pub push_epoch: EpochId,
    pub push_through: WalPosition,
    pub pull_epoch: EpochId,
    pub pull_through: WalPosition,
}

impl ReplicationBarrier {
    pub fn satisfied_by(&self, status: &WorkerStatus) -> bool {
        cursor_reaches(
            status.pushed,
            self.local_replica,
            self.push_epoch,
            self.push_through,
        ) && cursor_reaches(
            status.pulled,
            self.peer_replica,
            self.pull_epoch,
            self.pull_through,
        )
    }
}

fn cursor_reaches(
    cursor: Option<FeedCursorV1>,
    issuer: ReplicaId,
    epoch: EpochId,
    through: WalPosition,
) -> bool {
    cursor.is_some_and(|cursor| {
        cursor.issuer == issuer && cursor.log_epoch == epoch && cursor.through >= through
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StepOutcome {
    pub direction: Direction,
    pub observed_upper_bound: WalPosition,
    pub through: WalPosition,
    pub offered_origins: usize,
    pub transferred_origins: usize,
    pub transferred_objects: usize,
    pub transferred_bytes: u64,
    pub caught_up: bool,
}

#[derive(Clone)]
pub struct WorkerHandle {
    wake: Arc<Notify>,
    paused: watch::Sender<bool>,
    status: watch::Receiver<WorkerStatus>,
}

impl WorkerHandle {
    pub fn wake(&self) {
        self.wake.notify_one();
    }

    pub fn set_paused(&self, paused: bool) {
        self.paused.send_replace(paused);
        self.wake();
    }

    pub fn status(&self) -> WorkerStatus {
        self.status.borrow().clone()
    }

    pub fn subscribe(&self) -> watch::Receiver<WorkerStatus> {
        self.status.clone()
    }

    pub async fn wait_for_barrier(
        &self,
        barrier: ReplicationBarrier,
        timeout: Duration,
    ) -> Result<()> {
        let deadline = Instant::now() + timeout;
        let mut status = self.status.clone();
        loop {
            if barrier.satisfied_by(&status.borrow()) {
                return Ok(());
            }
            tokio::time::timeout_at(deadline, status.changed())
                .await
                .map_err(|_| Error::Limit("replication barrier timeout"))?
                .map_err(|_| Error::Integrity("replication worker stopped"))?;
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Circuit {
    failures: u32,
    open_until: Option<Instant>,
    last_fault: Option<FaultKind>,
}

#[derive(Clone, Copy, Debug, Default)]
struct TransferStats {
    offered: usize,
    transferred: usize,
    objects: usize,
    bytes: u64,
}

impl Circuit {
    fn status(self) -> DirectionStatus {
        DirectionStatus {
            consecutive_failures: self.failures,
            circuit_open: self.open_until.is_some_and(|until| until > Instant::now()),
            last_fault: self.last_fault,
        }
    }
}

pub struct ReplicationWorker<L, P> {
    repository: RepositoryId,
    local_replica: ReplicaId,
    peer_replica: ReplicaId,
    local: L,
    peer: P,
    checkpoints: PeerCheckpointStore,
    limits: WorkerLimits,
    next: Direction,
    pull: Circuit,
    push: Circuit,
    wake: Arc<Notify>,
    paused: watch::Sender<bool>,
    status: watch::Sender<WorkerStatus>,
}

impl<L: ReplicationEndpoint, P: ReplicationEndpoint> ReplicationWorker<L, P> {
    pub fn new(
        repository: RepositoryId,
        local_replica: ReplicaId,
        peer_replica: ReplicaId,
        local: L,
        peer: P,
        checkpoints: PeerCheckpointStore,
        limits: WorkerLimits,
    ) -> Result<Self> {
        limits.validate()?;
        if local_replica == peer_replica
            || local.repository() != repository
            || peer.repository() != repository
        {
            return Err(Error::Integrity("replication worker identity"));
        }
        let wake = Arc::new(Notify::new());
        let paused = watch::channel(false).0;
        let status = watch::channel(WorkerStatus::default()).0;
        Ok(Self {
            repository,
            local_replica,
            peer_replica,
            local,
            peer,
            checkpoints,
            limits,
            next: Direction::Pull,
            pull: Circuit::default(),
            push: Circuit::default(),
            wake,
            paused,
            status,
        })
    }

    pub fn handle(&self) -> WorkerHandle {
        WorkerHandle {
            wake: self.wake.clone(),
            paused: self.paused.clone(),
            status: self.status.subscribe(),
        }
    }

    pub async fn capture_barrier(&self) -> Result<ReplicationBarrier> {
        let local = self
            .negotiate(&self.local, self.peer_replica, self.local_replica)
            .await?;
        let peer = self
            .negotiate(&self.peer, self.local_replica, self.peer_replica)
            .await?;
        Ok(ReplicationBarrier {
            repository: self.repository,
            local_replica: self.local_replica,
            peer_replica: self.peer_replica,
            push_epoch: local.log_epoch,
            push_through: local.feed_upper_bound,
            pull_epoch: peer.log_epoch,
            pull_through: peer.feed_upper_bound,
        })
    }

    pub async fn step(&mut self) -> Result<StepOutcome> {
        let direction = self.select_direction()?;
        self.step_direction(direction).await
    }

    /// Run one explicitly selected direction. Bootstrap and migration use this
    /// to hydrate a destination without publishing its new local seed back to
    /// the source before the workspace handoff is complete.
    pub async fn step_direction(&mut self, direction: Direction) -> Result<StepOutcome> {
        if *self.paused.borrow() {
            self.publish_status(None, self.checkpoints.load().await?.as_ref());
            return Err(Error::CircuitOpen(direction));
        }
        if self.circuit_is_open(direction) {
            return Err(Error::CircuitOpen(direction));
        }
        self.next = opposite(direction);
        self.publish_status(Some(direction), self.checkpoints.load().await?.as_ref());
        let result = match direction {
            Direction::Pull => {
                self.sync_direction(
                    direction,
                    &self.peer,
                    self.peer_replica,
                    &self.local,
                    self.local_replica,
                )
                .await
            }
            Direction::Push => {
                self.sync_direction(
                    direction,
                    &self.local,
                    self.local_replica,
                    &self.peer,
                    self.peer_replica,
                )
                .await
            }
        };
        match &result {
            Ok(_) => self.record_success(direction),
            Err(error) => self.record_failure(direction, error),
        }
        let checkpoint = self.checkpoints.load().await?;
        self.publish_status(None, checkpoint.as_ref());
        result
    }

    /// Run until shutdown. Push and pull retain independent circuit state, so
    /// malformed or unavailable data in one direction does not stop the other.
    pub async fn run(mut self, mut shutdown: watch::Receiver<bool>) -> Result<()> {
        let mut paused = self.paused.subscribe();
        loop {
            if *shutdown.borrow() {
                return Ok(());
            }
            if *paused.borrow() {
                self.publish_status(None, self.checkpoints.load().await?.as_ref());
                tokio::select! {
                    changed = shutdown.changed() => if changed.is_err() { return Ok(()); },
                    changed = paused.changed() => if changed.is_err() { return Ok(()); },
                }
                continue;
            }
            let result = self.step().await;
            let delay = match &result {
                Ok(outcome) if !outcome.caught_up => Duration::ZERO,
                Ok(_) => self.limits.poll_interval,
                Err(Error::CircuitOpen(direction)) => self.circuit_delay(*direction),
                Err(_) => self.retry_delay(opposite(self.next)),
            };
            tokio::select! {
                changed = shutdown.changed() => if changed.is_err() { return Ok(()); },
                changed = paused.changed() => if changed.is_err() { return Ok(()); },
                _ = self.wake.notified() => {},
                _ = tokio::time::sleep(delay) => {},
            }
        }
    }

    fn select_direction(&self) -> Result<Direction> {
        let first = self.next;
        if !self.circuit_is_open(first) {
            return Ok(first);
        }
        let second = opposite(first);
        if !self.circuit_is_open(second) {
            return Ok(second);
        }
        Err(Error::CircuitOpen(first))
    }

    fn circuit_is_open(&self, direction: Direction) -> bool {
        self.circuit(direction)
            .open_until
            .is_some_and(|until| until > Instant::now())
    }

    fn circuit_delay(&self, direction: Direction) -> Duration {
        self.circuit(direction)
            .open_until
            .map_or(self.limits.retry_base, |until| {
                until.saturating_duration_since(Instant::now())
            })
    }

    fn retry_delay(&self, direction: Direction) -> Duration {
        let failures = self.circuit(direction).failures.max(1);
        let shift = failures.saturating_sub(1).min(20);
        let scaled = self
            .limits
            .retry_base
            .saturating_mul(1u32.checked_shl(shift).unwrap_or(u32::MAX))
            .min(self.limits.retry_max);
        let mut hash = Sha256::new();
        hash.update(b"jj-store/replication-retry-jitter/v1");
        hash.update(self.repository.as_bytes());
        hash.update(self.local_replica.as_bytes());
        hash.update(self.peer_replica.as_bytes());
        hash.update([direction_tag(direction)]);
        hash.update(failures.to_be_bytes());
        let value = u16::from_be_bytes(hash.finalize()[..2].try_into().expect("hash prefix"));
        let percent = 75u128 + u128::from(value % 51);
        Duration::from_nanos(
            u64::try_from(scaled.as_nanos().saturating_mul(percent) / 100).unwrap_or(u64::MAX),
        )
    }

    fn circuit(&self, direction: Direction) -> &Circuit {
        match direction {
            Direction::Pull => &self.pull,
            Direction::Push => &self.push,
        }
    }

    fn circuit_mut(&mut self, direction: Direction) -> &mut Circuit {
        match direction {
            Direction::Pull => &mut self.pull,
            Direction::Push => &mut self.push,
        }
    }

    fn record_success(&mut self, direction: Direction) {
        *self.circuit_mut(direction) = Circuit::default();
    }

    fn record_failure(&mut self, direction: Direction, error: &Error) {
        let fault = fault_kind(error);
        let permanent = matches!(error, Error::Endpoint(value) if value.class == RetryClass::Permanent)
            || matches!(error, Error::Integrity(_));
        let threshold = self.limits.circuit_failures;
        let cooldown = self.limits.circuit_cooldown;
        let circuit = self.circuit_mut(direction);
        circuit.failures = circuit.failures.saturating_add(1);
        circuit.last_fault = Some(fault);
        if permanent || circuit.failures >= threshold {
            circuit.open_until = Some(Instant::now() + cooldown);
        }
    }

    fn publish_status(&self, active: Option<Direction>, checkpoint: Option<&VersionedCheckpoint>) {
        let status = WorkerStatus {
            paused: *self.paused.borrow(),
            active,
            next_direction: self.next,
            checkpoint_revision: checkpoint.map_or(0, |value| value.checkpoint.revision),
            pulled: checkpoint.and_then(|value| value.checkpoint.pulled),
            pushed: checkpoint.and_then(|value| value.checkpoint.pushed),
            pull: self.pull.status(),
            push: self.push.status(),
        };
        self.status.send_replace(status);
    }

    async fn negotiate<E: ReplicationEndpoint>(
        &self,
        endpoint: &E,
        peer: ReplicaId,
        expected_replica: ReplicaId,
    ) -> Result<ReplicaNegotiationV1> {
        let capabilities = ReplicaCapabilitiesV1::default();
        let response = endpoint
            .negotiate(&NegotiateReplicaV1 {
                schema_version: 1,
                repository: self.repository,
                peer,
                capabilities: capabilities.clone(),
            })
            .await?;
        if response.schema_version != 1
            || response.repository != self.repository
            || response.replica != expected_replica
            || response.feed_lower_bound > response.feed_upper_bound
            || !capabilities.compatible_with(&response.capabilities)
        {
            return Err(Error::Integrity("replication negotiation"));
        }
        Ok(response)
    }

    async fn sync_direction<S: ReplicationEndpoint, D: ReplicationEndpoint>(
        &self,
        direction: Direction,
        source: &S,
        source_replica: ReplicaId,
        destination: &D,
        destination_replica: ReplicaId,
    ) -> Result<StepOutcome> {
        let negotiation = self
            .negotiate(source, destination_replica, source_replica)
            .await?;
        let mut previous = self.checkpoints.load().await?;
        let cursor = previous.as_ref().and_then(|value| match direction {
            Direction::Pull => value.checkpoint.pulled,
            Direction::Push => value.checkpoint.pushed,
        });
        let mut after = match cursor {
            Some(cursor)
                if cursor.issuer == source_replica && cursor.log_epoch == negotiation.log_epoch =>
            {
                cursor.through
            }
            Some(_) => return Err(Error::Integrity("replication feed changed identity")),
            None => WalPosition(0),
        };
        if after > negotiation.feed_upper_bound {
            return Err(Error::Integrity("replication feed moved backwards"));
        }
        let mut bootstrap_stats = TransferStats::default();
        if after < negotiation.feed_lower_bound {
            let checkpoint = negotiation
                .bootstrap
                .as_ref()
                .ok_or(Error::Integrity("replication bootstrap unavailable"))?;
            if checkpoint.schema_version != 1
                || checkpoint.source_replica != source_replica
                || checkpoint.log_epoch != negotiation.log_epoch
                || checkpoint.source_manifest != negotiation.manifest_digest
                || checkpoint.through < negotiation.feed_lower_bound
                || checkpoint.through > negotiation.feed_upper_bound
                || checkpoint.code_heads.is_empty()
                || checkpoint.seed.origin_key.stream.repository != self.repository
                || checkpoint.seed.origin_key.stream.replica != source_replica
                || checkpoint.seed.origin_key.sequence != 1
            {
                return Err(Error::Integrity("replication bootstrap checkpoint"));
            }
            let seed_closure = source
                .closure(&OriginClosureRequestV1 {
                    schema_version: 1,
                    origin_key: checkpoint.seed.origin_key,
                    max_entries: self.limits.closure_entries,
                })
                .await?
                .closure;
            let seed = &seed_closure.envelope.publication;
            if seed_closure.envelope.origin_digest != checkpoint.seed.origin_digest
                || seed.digest()? != checkpoint.seed.origin_digest
                || !seed.is_seed_baseline()
                || seed.published_operations != checkpoint.code_heads
                || seed.agent_root != Some(checkpoint.agent_root)
                || seed.review_root != Some(checkpoint.review_root)
                || seed.git_root != checkpoint.git_root
            {
                return Err(Error::Integrity("replication bootstrap closure"));
            }
            bootstrap_stats = self
                .transfer_offers(
                    direction,
                    source,
                    source_replica,
                    destination,
                    &[checkpoint.seed],
                    negotiation.log_epoch,
                    Some(checkpoint),
                )
                .await?;
            let cursor = FeedCursorV1 {
                issuer: source_replica,
                log_epoch: negotiation.log_epoch,
                through: checkpoint.through,
                observed_upper_bound: negotiation.feed_upper_bound,
            };
            let mut next = previous.as_ref().map_or_else(
                || self.checkpoints.empty(),
                |value| value.checkpoint.clone(),
            );
            next.revision = next
                .revision
                .checked_add(1)
                .ok_or(Error::Limit("peer checkpoint revision"))?;
            match direction {
                Direction::Pull => next.pulled = Some(cursor),
                Direction::Push => next.pushed = Some(cursor),
            }
            previous = Some(self.checkpoints.save(previous.as_ref(), next).await?);
            after = checkpoint.through;
        }
        let page = source
            .feed(&FeedRequestV1 {
                schema_version: 1,
                source_replica,
                log_epoch: negotiation.log_epoch,
                after,
                upper_bound: negotiation.feed_upper_bound,
                limit: self.limits.feed_entries,
            })
            .await?
            .page;
        if page.schema_version != 1
            || page.repository != self.repository
            || page.source_replica != source_replica
            || page.log_epoch != negotiation.log_epoch
            || page.after != after
            || page.upper_bound != negotiation.feed_upper_bound
            || page.through < after
            || page.through > page.upper_bound
            || page.origins.len() > self.limits.feed_entries
            || page.origins.iter().any(|item| {
                item.local_position <= after
                    || item.local_position > page.through
                    || item.envelope.publication.origin_key.stream.repository != self.repository
                    || item.envelope.publication.digest().ok() != Some(item.envelope.origin_digest)
            })
        {
            return Err(Error::Integrity("replication feed page"));
        }
        let offers = page
            .origins
            .iter()
            .map(|item| OriginOfferV1 {
                origin_key: item.envelope.publication.origin_key,
                origin_digest: item.envelope.origin_digest,
            })
            .collect::<Vec<_>>();
        let transfer = self
            .transfer_offers(
                direction,
                source,
                source_replica,
                destination,
                &offers,
                negotiation.log_epoch,
                None,
            )
            .await?;
        let cursor = FeedCursorV1 {
            issuer: source_replica,
            log_epoch: negotiation.log_epoch,
            through: page.through,
            observed_upper_bound: page.upper_bound,
        };
        if match direction {
            Direction::Pull => previous.as_ref().and_then(|value| value.checkpoint.pulled),
            Direction::Push => previous.as_ref().and_then(|value| value.checkpoint.pushed),
        } == Some(cursor)
        {
            return Ok(StepOutcome {
                direction,
                observed_upper_bound: page.upper_bound,
                through: page.through,
                offered_origins: bootstrap_stats.offered + transfer.offered,
                transferred_origins: bootstrap_stats.transferred + transfer.transferred,
                transferred_objects: bootstrap_stats.objects + transfer.objects,
                transferred_bytes: bootstrap_stats.bytes + transfer.bytes,
                caught_up: page.through == page.upper_bound,
            });
        }
        let mut next = previous.as_ref().map_or_else(
            || self.checkpoints.empty(),
            |value| value.checkpoint.clone(),
        );
        next.revision = next
            .revision
            .checked_add(1)
            .ok_or(Error::Limit("peer checkpoint revision"))?;
        match direction {
            Direction::Pull => next.pulled = Some(cursor),
            Direction::Push => next.pushed = Some(cursor),
        }
        self.checkpoints.save(previous.as_ref(), next).await?;
        Ok(StepOutcome {
            direction,
            observed_upper_bound: page.upper_bound,
            through: page.through,
            offered_origins: bootstrap_stats.offered + transfer.offered,
            transferred_origins: bootstrap_stats.transferred + transfer.transferred,
            transferred_objects: bootstrap_stats.objects + transfer.objects,
            transferred_bytes: bootstrap_stats.bytes + transfer.bytes,
            caught_up: page.through == page.upper_bound,
        })
    }

    #[allow(clippy::too_many_arguments)]
    async fn transfer_offers<S: ReplicationEndpoint, D: ReplicationEndpoint>(
        &self,
        direction: Direction,
        source: &S,
        source_replica: ReplicaId,
        destination: &D,
        offers: &[OriginOfferV1],
        log_epoch: EpochId,
        bootstrap: Option<&BootstrapCheckpointV1>,
    ) -> Result<TransferStats> {
        let response = destination
            .offer(&OfferOriginsRequestV1 {
                schema_version: 1,
                offers: offers.to_vec(),
            })
            .await?;
        if response.schema_version != 1 || response.results.len() != offers.len() {
            return Err(Error::Integrity("replication offer response"));
        }
        let mut missing = Vec::new();
        for (expected, result) in offers.iter().zip(response.results) {
            match result {
                OriginOfferResultV1::Missing { offer } if offer == *expected => missing.push(offer),
                OriginOfferResultV1::Known { receipt }
                    if receipt.origin_key == expected.origin_key
                        && receipt.origin_digest == expected.origin_digest => {}
                _ => return Err(Error::Integrity("replication offer result")),
            }
        }
        let mut envelopes = Vec::with_capacity(missing.len());
        let mut transferred_native = BTreeSet::new();
        let mut transferred_control = BTreeSet::new();
        let mut transferred_bytes = 0u64;
        for offer in &missing {
            let closure = source
                .closure(&OriginClosureRequestV1 {
                    schema_version: 1,
                    origin_key: offer.origin_key,
                    max_entries: self.limits.closure_entries,
                })
                .await?
                .closure;
            if closure.envelope.publication.origin_key != offer.origin_key
                || closure.envelope.origin_digest != offer.origin_digest
                || closure.envelope.publication.digest()? != closure.envelope.origin_digest
            {
                return Err(Error::Integrity("replication origin closure"));
            }
            if let Some(checkpoint) = bootstrap {
                let publication = &closure.envelope.publication;
                if offers.len() != 1
                    || !publication.is_seed_baseline()
                    || publication.published_operations != checkpoint.code_heads
                    || publication.agent_root != Some(checkpoint.agent_root)
                    || publication.review_root != Some(checkpoint.review_root)
                    || publication.git_root != checkpoint.git_root
                {
                    return Err(Error::Integrity("replication bootstrap closure"));
                }
            }
            for node in &closure.native {
                let Representation::Stored {
                    envelope_digest,
                    bytes,
                } = node.representation
                else {
                    continue;
                };
                if !transferred_native.insert(node.key) {
                    continue;
                }
                if transferred_native
                    .len()
                    .saturating_add(transferred_control.len())
                    > self.limits.max_objects_per_step
                {
                    return Err(Error::Limit("worker transfer objects"));
                }
                if bytes > self.limits.max_native_object_bytes {
                    return Err(Error::Limit("worker native object bytes"));
                }
                let download = source.read_native(node.key).await?;
                if download.metadata.len != bytes {
                    return Err(Error::Integrity("worker native object length"));
                }
                let upload = StagedUpload::from_reader(download.reader, bytes).await?;
                verify_native_upload(node.key, &upload, envelope_digest, bytes).await?;
                let outcome = destination.write_native(node.key, &upload).await?;
                if !matches!(outcome, WriteOutcome::Applied(_)) {
                    verify_native(destination, node.key, envelope_digest, bytes).await?;
                }
                transferred_bytes =
                    add_transfer_bytes(transferred_bytes, bytes, self.limits.max_bytes_per_step)?;
            }
            for reference in &closure.control {
                if !transferred_control.insert(*reference) {
                    continue;
                }
                if transferred_native
                    .len()
                    .saturating_add(transferred_control.len())
                    > self.limits.max_objects_per_step
                {
                    return Err(Error::Limit("worker transfer objects"));
                }
                let bytes = source.read_control(*reference).await?;
                if bytes.is_empty()
                    || bytes.len() as u64 > MAX_RECORD_BYTES
                    || RecordDigest::new(Sha256::digest(&bytes).into()) != reference.digest
                {
                    return Err(Error::Integrity("worker control record"));
                }
                let outcome = destination.write_control(*reference, bytes.clone()).await?;
                if !matches!(outcome, WriteOutcome::Applied(_)) {
                    let existing = destination.read_control(*reference).await?;
                    if existing != bytes {
                        return Err(Error::Integrity("worker control record collision"));
                    }
                }
                transferred_bytes = add_transfer_bytes(
                    transferred_bytes,
                    bytes.len() as u64,
                    self.limits.max_bytes_per_step,
                )?;
            }
            envelopes.push(closure.envelope);
        }
        if !envelopes.is_empty() {
            let (request_id, publication_id) = stable_apply_ids(
                self.repository,
                direction,
                source_replica,
                log_epoch,
                &envelopes,
            );
            let response = destination
                .apply(&ApplyOriginsTransportRequestV1 {
                    schema_version: 1,
                    request_id,
                    publication_id,
                    origins: envelopes.clone(),
                })
                .await?;
            if response.schema_version != 1 {
                return Err(Error::Integrity("replication apply response"));
            }
            match response.outcome {
                ApplyOriginsOutcomeV1::Applied(result) => {
                    let expected = envelopes
                        .iter()
                        .map(|envelope| (envelope.publication.origin_key, envelope.origin_digest))
                        .collect::<BTreeSet<_>>();
                    let actual = result
                        .receipts
                        .iter()
                        .map(|receipt| (receipt.origin_key, receipt.origin_digest))
                        .collect::<BTreeSet<_>>();
                    if result.schema_version != 1
                        || actual != expected
                        || result.receipts.len() != expected.len()
                    {
                        return Err(Error::Integrity("replication apply receipts"));
                    }
                }
                ApplyOriginsOutcomeV1::OutcomeUnknown {
                    request_id: returned_request,
                    publication_id: returned_publication,
                } if returned_request == request_id && returned_publication == publication_id => {
                    return Err(Error::OutcomeUnknown);
                }
                ApplyOriginsOutcomeV1::OutcomeUnknown { .. } => {
                    return Err(Error::Integrity("replication apply identity"));
                }
            }
        }
        Ok(TransferStats {
            offered: offers.len(),
            transferred: missing.len(),
            objects: transferred_native.len() + transferred_control.len(),
            bytes: transferred_bytes,
        })
    }
}

async fn verify_native<E: ReplicationEndpoint>(
    endpoint: &E,
    key: ObjectKey,
    expected_digest: EnvelopeDigest,
    expected_bytes: u64,
) -> Result<()> {
    let download = endpoint.read_native(key).await?;
    if download.metadata.len != expected_bytes {
        return Err(Error::Integrity("worker native object collision"));
    }
    let upload = StagedUpload::from_reader(download.reader, expected_bytes).await?;
    verify_native_upload(key, &upload, expected_digest, expected_bytes).await?;
    Ok(())
}

fn add_transfer_bytes(current: u64, added: u64, maximum: u64) -> Result<u64> {
    let total = current
        .checked_add(added)
        .ok_or(Error::Limit("worker transfer bytes"))?;
    if total > maximum {
        return Err(Error::Limit("worker transfer bytes"));
    }
    Ok(total)
}

fn stable_apply_ids(
    repository: RepositoryId,
    direction: Direction,
    source_replica: ReplicaId,
    epoch: EpochId,
    origins: &[OriginEnvelopeV1],
) -> (RequestId, PublicationId) {
    let digest = |domain: &[u8]| {
        let mut hash = Sha256::new();
        hash.update(domain);
        hash.update(repository.as_bytes());
        hash.update([direction_tag(direction)]);
        hash.update(source_replica.as_bytes());
        hash.update(epoch.as_bytes());
        for envelope in origins {
            let key = envelope.publication.origin_key;
            hash.update(key.stream.replica.as_bytes());
            hash.update(key.stream.writer_epoch.as_bytes());
            hash.update(key.sequence.to_be_bytes());
            hash.update(envelope.origin_digest.as_bytes());
        }
        let result = hash.finalize();
        <[u8; 16]>::try_from(&result[..16]).expect("SHA-256 prefix")
    };
    (
        RequestId::new(digest(b"jj-store/replication-apply-request/v1")),
        PublicationId::new(digest(b"jj-store/replication-apply-publication/v1")),
    )
}

fn opposite(direction: Direction) -> Direction {
    match direction {
        Direction::Pull => Direction::Push,
        Direction::Push => Direction::Pull,
    }
}

fn direction_tag(direction: Direction) -> u8 {
    match direction {
        Direction::Pull => 0,
        Direction::Push => 1,
    }
}

fn fault_kind(error: &Error) -> FaultKind {
    match error {
        Error::Endpoint(value) if value.class == RetryClass::Transient => {
            FaultKind::TransientEndpoint
        }
        Error::Endpoint(_) => FaultKind::PermanentEndpoint,
        Error::Integrity(_) | Error::Publication(_) | Error::Storage(_) => FaultKind::Integrity,
        Error::Limit(_) | Error::CircuitOpen(_) => FaultKind::Limit,
        Error::CursorConflict => FaultKind::CursorConflict,
        Error::OutcomeUnknown => FaultKind::OutcomeUnknown,
    }
}
