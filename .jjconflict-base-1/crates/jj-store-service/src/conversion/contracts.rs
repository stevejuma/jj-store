//! Versioned installation conversion contracts; source archives remain private.
use jj_store_model::{OperationId, RepositoryId, RequestId};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ConversionProfile {
    Native,
    LocalJjGit,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ConversionIntent {
    pub version: u32,
    pub source: RepositoryId,
    pub source_profile: ConversionProfile,
    pub operation: OperationId,
    pub destination_profile: ConversionProfile,
    pub name: String,
    pub path: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StartConversion {
    pub request_id: RequestId,
    pub intent: ConversionIntent,
    pub prepared_source: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ConversionPreview {
    pub version: u32,
    pub prepared_source: Option<String>,
    pub source_bytes: usize,
    pub history_selections: usize,
    pub blocker: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ConversionReport {
    pub version: u32,
    pub request_id: RequestId,
    pub source: RepositoryId,
    pub destination: RepositoryId,
    pub destination_profile: ConversionProfile,
    pub name: String,
    pub operation: OperationId,
    pub archive_digest: String,
    pub code_mappings: BTreeMap<String, BTreeMap<String, String>>,
    pub collaboration: super::CollaborationMappingsV1,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ConversionOutcome {
    Committed { report: Box<ConversionReport> },
    OutcomeUnknown { request_id: RequestId },
    NotKnown { request_id: RequestId },
}
