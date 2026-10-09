//! Typed metadata commands, independent of model execution and repository publication.
use super::*;
use jj_collab::*;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "ConversationMetadataConfig")]
pub struct Config {
    pub model: String,
    pub options: settings::RuntimeOptions,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "ConversationMetadataQuery")]
pub struct Query {
    pub session: String,
    pub repository: Option<RepositoryId>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "ConversationMetadataView")]
pub struct View {
    pub version: u32,
    pub repository: RepositoryId,
    pub target: ConversationMetadataTargetV1,
    pub thread: Option<ThreadId>,
    pub revision: Option<CollabObjectRef>,
    pub value: ConversationMetadataValueV1,
    pub sources: Vec<ConversationMetadataSourceV1>,
    pub title: Option<String>,
    pub fresh: bool,
    pub writable: bool,
    pub unavailable: Option<String>,
    pub details: serde_json::Value,
    pub job: Option<Job>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub enum Action {
    Rename { title: String },
    ResetTitle,
    CaptureTitles,
    Refresh,
    RegenerateTitle,
    Cancel { job: RequestId },
    Publish { job: RequestId },
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "ConversationMetadataCommand")]
pub struct Command {
    pub version: u32,
    pub request_id: RequestId,
    pub query: Query,
    pub expected: Option<CollabObjectRef>,
    pub action: Action,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "ConversationMetadataJob")]
pub struct Job {
    pub request_id: RequestId,
    pub state: String,
    pub error: Option<String>,
    pub result: Option<GeneratedConversationMetadataV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_command: Option<Box<Command>>,
}
