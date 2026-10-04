//! Typed local configuration; saved intent does not mutate a running worker.
use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CaptureHarnessSetting {
    pub profile: CaptureHarnessProfile,
    #[schemars(length(min = 1, max = 32))]
    pub product_version: String,
    #[schemars(length(max = 16))]
    pub roots: Vec<CaptureRootPath>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CaptureConfiguration {
    pub enabled: bool,
    #[schemars(range(min = 1, max = 4))]
    pub max_readers: usize,
    #[schemars(range(min = 1, max = 30))]
    pub rescan_seconds: u64,
    #[schemars(range(min = 1, max = 4096))]
    pub max_sources: usize,
    #[schemars(range(min = 1, max = 67108864))]
    pub max_pending_bytes: u64,
    #[schemars(length(max = 2))]
    pub harnesses: Vec<CaptureHarnessSetting>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LocalCaptureSettings {
    pub version: u32,
    #[schemars(length(min = 64, max = 64), regex(pattern = "^[0-9a-f]{64}$"))]
    pub revision: String,
    pub desired: CaptureConfiguration,
    pub effective: CaptureConfiguration,
    pub restart_required: bool,
    /// Resolved process defaults, also available before capture is enabled.
    #[schemars(length(max = 2))]
    pub defaults: Vec<CaptureHarnessSetting>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SaveCaptureSettings {
    pub version: u32,
    #[schemars(length(min = 64, max = 64), regex(pattern = "^[0-9a-f]{64}$"))]
    pub expected_revision: String,
    pub configuration: CaptureConfiguration,
}
