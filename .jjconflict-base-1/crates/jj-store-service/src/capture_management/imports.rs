use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CaptureImportCommand {
    pub version: u32,
    pub request_id: RequestId,
    #[schemars(length(min = 32, max = 32), regex(pattern = "^[0-9a-f]{32}$"))]
    pub generation: String,
    pub expected_revision: u64,
    #[schemars(
        length(min = 49, max = 49),
        regex(pattern = "^[0-9a-f]{32}:[0-9a-f]{16}$")
    )]
    pub selector: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CaptureImportQuery {
    pub request_id: RequestId,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CaptureImportPhase {
    Selected,
    Pending,
    Running,
    Withheld,
    Completed,
    Unavailable,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CaptureImportAcceptance {
    pub command: CaptureImportCommand,
    pub ordinal: u64,
    pub profile: CaptureHarnessProfile,
    #[schemars(length(min = 1, max = 32))]
    pub product_version: String,
    pub complete_byte_offset: u64,
    pub complete_records: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CaptureImportTail {
    pub history_available: bool,
    pub captured_byte_offset: u64,
    pub committed_byte_offset: u64,
    pub outstanding_ranges: usize,
    pub receipts: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CaptureImportProgress {
    pub phase: CaptureImportPhase,
    pub boundary_available: bool,
    pub captured_byte_offset: u64,
    pub committed_byte_offset: u64,
    pub tail_phase: CaptureImportPhase,
    pub tail: Option<CaptureImportTail>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CaptureImportStatus {
    pub version: u32,
    pub request_id: RequestId,
    pub receipt: Option<CaptureImportAcceptance>,
    pub progress: CaptureImportProgress,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CaptureImportOutcome {
    pub version: u32,
    pub replayed: bool,
    pub receipt: CaptureImportAcceptance,
    pub progress: CaptureImportProgress,
}
