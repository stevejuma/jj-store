//! Trusted conversation dispatch carries stable identities and registered attachments.
use super::*;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentConversationOrigin")]
pub struct Origin {
    pub scope: String,
    pub home: Option<RepositoryId>,
    pub thread: Option<jj_collab::ThreadId>,
    #[serde(default)]
    pub agent_target: Option<jj_collab::ReactionTargetV1>,
    pub context: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentConversationAttachment")]
pub struct Attachment {
    pub repository: RepositoryId,
    pub target_id: Option<String>,
    pub operation: Option<OperationId>,
    pub commit: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentConversation")]
pub struct Conversation {
    pub id: String,
    pub origin: Origin,
    pub home: RepositoryId,
    #[serde(default)]
    pub discussion_thread: Option<jj_collab::ThreadId>,
    pub participants: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentConversationSend")]
pub struct Send {
    pub version: u32,
    pub request_id: RequestId,
    pub conversation_id: String,
    pub origin: Origin,
    pub agent_ids: Vec<String>,
    pub attachments: Vec<Attachment>,
    pub text: String,
    pub max_duration_ms: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentConversationOutcome")]
pub struct SendOutcome {
    pub request_id: RequestId,
    pub conversation_id: String,
    pub agents: Vec<Outcome>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentConversationWorkspace")]
pub struct Binding {
    pub repository: RepositoryId,
    pub target_id: String,
    pub profile: String,
    pub workspace: String,
    pub selected_operation: OperationId,
    pub base_commit: String,
    pub operation: OperationId,
    pub commit: String,
}
