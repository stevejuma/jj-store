//! Operator configuration, independent of execution and repository authority.
use crate::{agent_execution::Setup, auth::BearerToken};
use jj_store_model::PrincipalId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "ServerAgentDefinition")]
pub struct Definition {
    pub setup: Setup,
    pub principals: BTreeSet<PrincipalId>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "ServerAgentConfiguration")]
pub struct Configuration {
    pub acp_executable: String,
    pub jj_executable: String,
    pub agents: Vec<Definition>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "ServerAgentAdministrationStatus")]
pub struct Status {
    pub revision: String,
    pub desired: Option<Configuration>,
    pub effective: Option<Configuration>,
    pub restart_required: bool,
    pub principals: Vec<PrincipalId>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "ServerAgentConfigurationSave")]
pub struct Save {
    pub expected_revision: String,
    pub configuration: Option<Configuration>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "OperatorAccess")]
pub struct Access {
    pub agents: bool,
    pub identities: bool,
}
#[async_trait::async_trait]
pub trait Administration: Send + Sync {
    async fn status(&self, token: BearerToken) -> Result<Status, crate::server_settings::Error>;
    async fn save(
        &self,
        token: BearerToken,
        request: Save,
    ) -> Result<Status, crate::server_settings::Error>;
}
