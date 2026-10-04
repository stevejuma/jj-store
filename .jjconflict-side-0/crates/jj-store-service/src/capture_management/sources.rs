use super::*;
use jj_store_model::RepositoryId;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CaptureSourceQuery {
    #[schemars(range(min = 1, max = 100))]
    pub limit: Option<usize>,
    #[schemars(
        length(min = 49, max = 49),
        regex(pattern = "^[0-9a-f]{32}:[0-9a-f]{16}$")
    )]
    pub cursor: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CaptureSourceCoverage {
    #[schemars(
        length(min = 49, max = 49),
        regex(pattern = "^[0-9a-f]{32}:[0-9a-f]{16}$")
    )]
    pub selector: String,
    pub profile: CaptureHarnessProfile,
    #[schemars(length(min = 1, max = 32))]
    pub product_version: String,
    pub phase: CaptureRuntimePhase,
    pub observed: bool,
    pub awaiting_activity: bool,
    pub caught_up: bool,
    pub running: bool,
    pub degraded: bool,
    pub last_bound_repository: Option<RepositoryId>,
    pub ingress_required_capacity_bytes: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_publication: Option<CaptureSnapshotPublication>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CaptureSourcePage {
    pub version: u32,
    pub enabled: bool,
    #[schemars(length(min = 32, max = 32), regex(pattern = "^[0-9a-f]{32}$"))]
    pub runtime_generation: Option<String>,
    pub phase: Option<CaptureRuntimePhase>,
    #[schemars(range(max = 4096))]
    pub total: usize,
    #[schemars(length(max = 100))]
    pub sources: Vec<CaptureSourceCoverage>,
    #[schemars(
        length(min = 49, max = 49),
        regex(pattern = "^[0-9a-f]{32}:[0-9a-f]{16}$")
    )]
    pub next_cursor: Option<String>,
}

/// Source coverage only; no import completion, execution identity or current eligibility.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CaptureSnapshotPublication {
    pub history_available: bool,
    pub captured_sequence: u64,
    pub captured_logical_records: u64,
    pub committed_sequence: u64,
    pub committed_logical_records: Option<u64>,
    #[schemars(range(max = 4096))]
    pub outstanding_ranges: usize,
    pub receipts: u64,
}
