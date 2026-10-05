//! Credential-free replication control and status contracts.
//!
//! These DTOs describe local worker state. They deliberately keep local and
//! peer WAL frontiers separate: equal sequence numbers from different issuers
//! never imply equivalent repository state.
use jj_store_model as m;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(rename = "SyncPhase")]
pub enum Phase {
    Connecting,
    Running,
    Paused,
    Degraded,
    Stopped,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(rename = "SyncFault")]
pub enum Fault {
    TransientEndpoint,
    PermanentEndpoint,
    Integrity,
    Limit,
    CursorConflict,
    OutcomeUnknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "SyncFrontier")]
pub struct FrontierV1 {
    pub replica: m::ReplicaId,
    pub log_epoch: m::EpochId,
    pub through: m::WalPosition,
    pub observed_upper_bound: m::WalPosition,
}

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema,
)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "SyncDirectionStatus")]
pub struct DirectionStatusV1 {
    pub consecutive_failures: u32,
    pub circuit_open: bool,
    pub last_fault: Option<Fault>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "SyncSearchStatus")]
pub struct SearchStatusV1 {
    pub configured: bool,
    pub repository_position: Option<m::WalPosition>,
    pub indexed_position: Option<m::WalPosition>,
    pub degraded: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "SyncStatus")]
pub struct StatusV1 {
    pub version: u32,
    pub repository: m::RepositoryId,
    pub local_ready: bool,
    pub offline_ready: bool,
    pub phase: Phase,
    pub local_replica: Option<m::ReplicaId>,
    pub peer_replica: Option<m::ReplicaId>,
    pub local_frontier: Option<FrontierV1>,
    pub peer_frontier: Option<FrontierV1>,
    pub pushed: Option<FrontierV1>,
    pub pulled: Option<FrontierV1>,
    pub local_pending_positions: Option<u64>,
    pub peer_pending_positions: Option<u64>,
    pub checkpoint_revision: u64,
    pub operation_heads: usize,
    pub reconciliation_required: bool,
    pub pull: DirectionStatusV1,
    pub push: DirectionStatusV1,
    pub search: SearchStatusV1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "SyncBarrier")]
pub struct BarrierV1 {
    pub version: u32,
    pub repository: m::RepositoryId,
    pub local_replica: m::ReplicaId,
    pub peer_replica: m::ReplicaId,
    pub push_epoch: m::EpochId,
    pub push_through: m::WalPosition,
    pub pull_epoch: m::EpochId,
    pub pull_through: m::WalPosition,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "SyncWaitRequest")]
pub struct WaitRequestV1 {
    pub version: u32,
    pub barrier: BarrierV1,
    pub timeout_millis: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "SyncWaitOutcome")]
pub struct WaitOutcomeV1 {
    pub version: u32,
    pub barrier: BarrierV1,
    pub status: StatusV1,
}
