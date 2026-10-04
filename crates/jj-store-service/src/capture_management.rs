//! Installation-local device policy contracts; actor is supplied by admitted transport.
use jj_store_model::{PrincipalId, RequestId};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
mod sources;
pub use sources::*;
mod imports;
pub use imports::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CaptureControlAction {
    Pause,
    Resume,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CaptureControlCommand {
    pub version: u32,
    pub request_id: RequestId,
    #[schemars(length(min = 32, max = 32), regex(pattern = "^[0-9a-f]{32}$"))]
    pub generation: String,
    pub expected_revision: u64,
    pub action: CaptureControlAction,
}

/// Transient conditional discovery hint; no durable request identity or actor.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CaptureRescanCommand {
    pub version: u32,
    #[schemars(length(min = 32, max = 32), regex(pattern = "^[0-9a-f]{32}$"))]
    pub generation: String,
    pub expected_revision: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CapturePolicy {
    #[schemars(length(min = 32, max = 32), regex(pattern = "^[0-9a-f]{32}$"))]
    pub generation: String,
    pub revision: u64,
    pub paused: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CaptureRuntimePhase {
    Ready,
    Running,
    Overdue,
    Draining,
    Paused,
    Stopped,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CaptureDiscoveryStatus {
    pub phase: CaptureRuntimePhase,
    pub roots: usize,
    pub unchecked_roots: usize,
    pub available_roots: usize,
    pub unavailable_roots: usize,
    pub rescan_pending: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CaptureRuntimeStatus {
    pub phase: CaptureRuntimePhase,
    pub discovery: CaptureDiscoveryStatus,
    pub degraded_sources: usize,
    #[schemars(length(max = 2))]
    pub harnesses: Vec<CaptureHarnessCoverage>,
    pub pending_capacity_bytes: u64,
    pub ingress_capacity_refusals: usize,
    pub ingress_required_capacity_bytes: Option<u64>,
    pub delivery_required_capacity_bytes: Option<u64>,
    pub tracked: usize,
    pub observed_sources: usize,
    pub awaiting_activity: usize,
    pub caught_up: usize,
    pub running_sources: usize,
    pub resident_source_frames: usize,
    pub resident_delivery_frames: usize,
    pub pending_batches: usize,
    pub unavailable: usize,
    pub source_limit_reached: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum CaptureHarnessProfile {
    CodexCliJsonlV1,
    CodexDesktopJsonlV1,
    CursorDesktopTranscriptV1,
    OpencodeReadOnlyApiV1,
    OpencodeExportJsonV1,
}
/// Startup-selected directory metadata, not a discovered transcript file.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct CaptureRootPath(#[schemars(length(min = 1, max = 4096))] pub String);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CaptureHarnessCoverage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(length(max = 16))]
    pub configured_roots: Option<Vec<CaptureRootPath>>,
    pub profile: CaptureHarnessProfile,
    #[schemars(length(min = 1, max = 32))]
    pub product_version: String,
    pub roots: usize,
    pub unchecked_roots: usize,
    pub available_roots: usize,
    pub unavailable_roots: usize,
    pub tracked: usize,
    pub observed_sources: usize,
    pub awaiting_activity: usize,
    pub caught_up: usize,
    pub running_sources: usize,
    pub degraded_sources: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CaptureStatus {
    pub version: u32,
    pub enabled: bool,
    pub policy: Option<CapturePolicy>,
    pub runtime: Option<CaptureRuntimeStatus>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CapturePolicyReceipt {
    pub generation: String,
    pub request_id: RequestId,
    pub expected_revision: u64,
    pub revision: u64,
    pub paused: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CaptureControlOutcome {
    pub version: u32,
    pub receipt: CapturePolicyReceipt,
    pub replayed: bool,
    pub runtime_requested: bool,
    pub status: CaptureStatus,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CaptureRescanOutcome {
    pub version: u32,
    pub accepted: bool,
    pub status: CaptureStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("capture management denied")]
    Denied,
    #[error("capture is disabled")]
    Disabled,
    #[error("capture is paused")]
    Paused,
    #[error("invalid capture control")]
    Invalid,
    #[error("capture control intent conflicts")]
    Conflict,
    #[error("capture control policy changed")]
    Stale,
    #[error("capture source selection changed")]
    SourceStale,
    #[error("capture management busy")]
    Busy,
    #[error("capture control capacity exhausted")]
    Capacity,
    #[error("capture management unavailable")]
    Unavailable,
    #[error("capture control outcome unknown; retain the original request")]
    Unknown,
}

#[async_trait::async_trait]
pub trait CaptureManager: Send + Sync {
    async fn import(
        &self,
        actor: PrincipalId,
        request: CaptureImportCommand,
    ) -> Result<CaptureImportOutcome, Error>;
    async fn import_status(
        &self,
        actor: PrincipalId,
        query: CaptureImportQuery,
    ) -> Result<CaptureImportStatus, Error>;
    async fn status(&self, actor: PrincipalId) -> Result<CaptureStatus, Error>;
    async fn sources(
        &self,
        actor: PrincipalId,
        query: CaptureSourceQuery,
    ) -> Result<CaptureSourcePage, Error>;
    async fn control(
        &self,
        actor: PrincipalId,
        request: CaptureControlCommand,
    ) -> Result<CaptureControlOutcome, Error>;
    async fn rescan(
        &self,
        actor: PrincipalId,
        request: CaptureRescanCommand,
    ) -> Result<CaptureRescanOutcome, Error>;
}
