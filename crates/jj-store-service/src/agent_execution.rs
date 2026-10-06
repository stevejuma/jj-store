//! Application-owned execution contracts, independent of ACP implementation.
use jj_store_model::{CommitId, OperationId, PrincipalId, RepositoryId, RequestId};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum Error {
    #[error("agent execution unavailable")]
    Unavailable,
    #[error("agent execution denied")]
    Denied,
    #[error("invalid agent execution request")]
    Invalid,
    #[error("agent execution selection changed")]
    Stale,
    #[error("agent execution resource limit")]
    Limit,
    #[error("agent request identity conflict")]
    Conflict,
    #[error("agent execution busy")]
    Busy,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentExecutionPreferences")]
pub struct Preferences {
    pub model: Option<String>,
    pub effort: Option<String>,
    pub mode: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentExecutionSetup")]
pub struct Setup {
    pub id: String,
    pub name: String,
    pub avatar: String,
    #[serde(default)]
    pub description: String,
    pub instructions: String,
    pub runner: String,
    pub preferences: Preferences,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentExecutionSetupCommand")]
pub struct SetupCommand {
    pub version: u32,
    pub request_id: RequestId,
    pub expected_revision: u64,
    pub setup: Option<Setup>,
    pub remove: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentExecutionRunner")]
pub struct Runner {
    pub id: String,
    pub label: String,
    pub available: bool,
    pub qualified: bool,
    pub install_hint: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentExecutionTarget")]
pub struct Target {
    pub id: String,
    pub repository: RepositoryId,
    pub name: String,
    pub folder: String,
    pub profile: String,
    pub operation: OperationId,
    pub commit: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentExecutionLaunch")]
pub struct Launch {
    pub version: u32,
    pub request_id: RequestId,
    pub target_id: String,
    pub repository: RepositoryId,
    pub operation: OperationId,
    pub commit: String,
    pub runner: String,
    pub setup_id: Option<String>,
    pub context: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentExecutionSession")]
pub struct Session {
    pub id: String,
    #[serde(default)]
    pub setup_id: Option<String>,
    /// Read-only live supervisor projection, never journal authority.
    #[serde(default)]
    pub connected: bool,
    pub repository: RepositoryId,
    pub target_id: String,
    pub runner: String,
    pub title: String,
    pub avatar: String,
    pub workspace: String,
    pub base_commit: String,
    pub review_commit: String,
    pub operation: OperationId,
    pub state: String,
    pub adapter_session_id: Option<String>,
    pub capabilities: serde_json::Value,
    pub preferences: Preferences,
    pub last_sequence: u64,
    /// Device observation time of the latest retained execution event, if known.
    #[serde(default)]
    pub last_activity_at_millis: Option<u64>,
    pub published_sequence: u64,
    pub last_error: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentExecutionEvent")]
pub struct Event {
    pub sequence: u64,
    /// Device observation time captured when this event was retained, if known.
    #[serde(default)]
    pub observed_at_millis: Option<u64>,
    pub payload: serde_json::Value,
    pub committed: bool,
}
#[derive(Clone, Debug, Serialize, JsonSchema)]
#[schemars(rename = "AgentExecutionEventPage")]
pub struct EventPage {
    pub version: u32,
    pub events: Vec<Event>,
    pub next: Option<u64>,
}
#[derive(Clone, Debug, Serialize, JsonSchema)]
#[schemars(rename = "AgentExecutionStatus")]
pub struct Status {
    pub version: u32,
    pub runtime: String,
    pub revision: u64,
    pub setups: Vec<Setup>,
    pub sessions: Vec<Session>,
    pub runners: Vec<Runner>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(rename = "AgentExecutionAction")]
pub enum Action {
    Configure {
        preferences: Preferences,
    },
    Prompt {
        text: String,
        max_duration_ms: u64,
    },
    Permission {
        permission_id: String,
        option_id: Option<String>,
    },
    Cancel {
        target: String,
    },
    End,
    Resume,
    ContinueWithContext,
    Cleanup,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentExecutionCommand")]
pub struct Command {
    pub version: u32,
    pub request_id: RequestId,
    pub session_id: String,
    pub action: Action,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentExecutionOutcome")]
pub struct Outcome {
    pub version: u32,
    pub request_id: RequestId,
    pub session_id: Option<String>,
    pub state: String,
    pub error: Option<String>,
}
/// Internal workspace proof; it never comes from a browser.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub source: PathBuf,
    pub path: PathBuf,
    pub metadata: PathBuf,
    pub operation: OperationId,
    pub commit: String,
    pub name: String,
}
/// JJ composition owns executable selection and physical workspace operations.
#[async_trait::async_trait]
pub trait WorkspaceDriver: Send + Sync {
    async fn inspect(&self, path: PathBuf) -> Result<Workspace>;
    async fn create(
        &self,
        source: Workspace,
        destination: PathBuf,
        operation: OperationId,
        commit: String,
        name: String,
    ) -> Result<Workspace>;
    async fn snapshot(&self, workspace: &Workspace) -> Result<Workspace>;
    async fn cleanup(&self, workspace: &Workspace) -> Result<()>;
}
/// Optional LocalTrusted runtime; repository and actor checks remain host-owned.
#[async_trait::async_trait]
pub trait Manager: Send + Sync {
    async fn status(&self, actor: PrincipalId) -> Result<Status>;
    async fn targets(
        &self,
        actor: PrincipalId,
        repository: Option<RepositoryId>,
    ) -> Result<Vec<Target>>;
    async fn save_setup(&self, actor: PrincipalId, command: SetupCommand) -> Result<Status>;
    async fn launch(&self, actor: PrincipalId, command: Launch) -> Result<Outcome>;
    async fn command(&self, actor: PrincipalId, command: Command) -> Result<Outcome>;
    async fn outcome(&self, actor: PrincipalId, request: RequestId) -> Result<Outcome>;
    async fn events(&self, actor: PrincipalId, session: String, after: u64) -> Result<EventPage>;
    async fn feedback(&self, _actor: PrincipalId, _session: String) -> Result<Feedback> {
        Err(Error::Unavailable)
    }
    fn subscribe(&self) -> tokio::sync::broadcast::Receiver<u64>;
    async fn shutdown(&self) -> Result<()>;
}
/// Compose providers without exposing their concrete process implementation.
pub type AgentManager = Arc<dyn Manager>;
/// Exact native revision shape; local Git SHA-1 is a separately admitted shape.
pub fn valid_commit(commit: &str) -> bool {
    matches!(commit.len(), 40 | 128)
        && commit
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}
/// Display-only conversion cannot invent or remap a native identity.
pub fn native_commit(commit: &str) -> Result<CommitId> {
    commit.parse().map_err(|_| Error::Invalid)
}

/// Repository-selected messages and ordinary discussion revisions, never journal authority.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "AgentExecutionFeedback")]
pub struct Feedback {
    pub messages: Vec<FeedbackMessage>,
    /// Current browser feedback write capability; does not suppress readable history.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write_unavailable: Option<String>,
}
impl Feedback {
    pub fn prompt_context(&self) -> Result<String> {
        crate::agent_feedback::context(self)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "AgentExecutionFeedbackMessage")]
pub struct FeedbackMessage {
    pub row_id: String,
    pub target: jj_collab::ReactionTargetV1,
    pub threads: Vec<jj_collab::ThreadId>,
    pub replies: Vec<FeedbackReply>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "AgentExecutionFeedbackReply")]
pub struct FeedbackReply {
    pub thread_id: jj_collab::ThreadId,
    pub revision: jj_collab::CollabObjectRef,
    pub message: jj_collab::HumanMessageRevisionV1,
}
