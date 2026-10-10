//! Application-owned execution contracts, independent of ACP implementation.
mod feedback_subject;
pub mod thread_workspace;
pub mod workspace_inspection;
pub use feedback_subject::FeedbackSubject;
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
    #[error("discussion context exceeds the reply or byte budget")]
    FeedbackLimit,
    #[error("discussion context could not be read")]
    FeedbackUnavailable,
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
    #[serde(default)]
    pub options: settings::RuntimeOptions,
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
    #[serde(default)]
    pub providers: Vec<providers::Provider>,
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
    pub conversation_id: Option<String>,
    #[serde(default)]
    pub repository_bindings: Vec<conversations::Binding>,
    #[serde(default)]
    pub setup_id: Option<String>,
    /// Read-only capture-normalizer aliases for this installation-owned session.
    #[serde(default)]
    pub recorded_session_ids: Vec<crate::collaboration::AgentSessionIdentity>,
    /// Read-only name from the retained ready workspace proof, absent after cleanup.
    #[serde(default)]
    pub workspace_name: Option<String>,
    /// Operation from that ready workspace proof, including creation before the first turn.
    #[serde(default)]
    pub workspace_operation: Option<OperationId>,
    /// Read-only live supervisor projection, never journal authority.
    #[serde(default)]
    pub connected: bool,
    pub repository: RepositoryId,
    pub target_id: String,
    pub runner: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discovery_text: Option<String>,
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
    #[serde(default)]
    pub live_agents: Vec<fleet::LiveAgent>,
    #[serde(default)]
    pub conversations: Vec<conversations::Conversation>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(rename = "AgentExecutionAction")]
pub enum Action {
    Configure {
        preferences: Preferences,
    },
    Prompt {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        comments: Option<Box<jj_collab::ConversationSubmissionV1>>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_publication: Option<conversations::CommentPublication>,
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
    fn ignore_inputs(&self, _workspace: &Workspace) -> Result<Vec<Vec<u8>>> {
        Err(Error::Unavailable)
    }
    async fn initialize_personal(&self, _path: PathBuf) -> Result<Workspace> {
        Err(Error::Unavailable)
    }
    async fn materialize(
        &self,
        _repository: RepositoryId,
        _path: PathBuf,
        _socket: PathBuf,
    ) -> Result<Workspace> {
        Err(Error::Unavailable)
    }
    async fn inspect(&self, path: PathBuf) -> Result<Workspace>;
    async fn verify(&self, expected: &Workspace) -> Result<()> {
        let current = self.inspect(expected.path.clone()).await?;
        if current.path != expected.path
            || current.metadata != expected.metadata
            || current.operation != expected.operation
            || current.commit != expected.commit
            || current.name != expected.name
        {
            return Err(Error::Stale);
        }
        Ok(())
    }
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
    async fn thread_workspace(
        &self,
        _actor: PrincipalId,
        _request: thread_workspace::Request,
    ) -> Result<thread_workspace::Outcome> {
        Err(Error::Unavailable)
    }
    async fn chatgpt_status(&self, _actor: PrincipalId) -> Result<chatgpt::Status> {
        Ok(chatgpt::Status {
            supported: false,
            accounts: vec![],
            attempts: vec![],
        })
    }
    async fn chatgpt_command(
        &self,
        _actor: PrincipalId,
        _command: chatgpt::Command,
    ) -> Result<chatgpt::Status> {
        Err(Error::Unavailable)
    }
    async fn inspect_workspace(
        &self,
        _actor: PrincipalId,
        _request: workspace_inspection::Request,
    ) -> Result<workspace_inspection::Inspection> {
        Err(Error::Unavailable)
    }
    async fn catalog_preview(
        &self,
        _actor: PrincipalId,
        _preview: catalog::Preview,
    ) -> Result<catalog::Catalog> {
        Err(Error::Unavailable)
    }
    async fn catalog(&self, _actor: PrincipalId, _runner: String) -> Result<catalog::Catalog> {
        Err(Error::Unavailable)
    }
    async fn runtime_status(&self, _actor: PrincipalId) -> Result<tooling::Status> {
        Err(Error::Unavailable)
    }
    async fn install_runtime(
        &self,
        _actor: PrincipalId,
        _command: tooling::Command,
    ) -> Result<tooling::Outcome> {
        Err(Error::Unavailable)
    }
    async fn runtime_outcome(
        &self,
        _actor: PrincipalId,
        _request: RequestId,
    ) -> Result<tooling::Outcome> {
        Err(Error::Unavailable)
    }
    async fn send_conversation(
        &self,
        _actor: PrincipalId,
        _command: conversations::Send,
    ) -> Result<conversations::SendOutcome> {
        Err(Error::Unavailable)
    }
    async fn control_agent(
        &self,
        _actor: PrincipalId,
        _command: fleet::Control,
    ) -> Result<Outcome> {
        Err(Error::Unavailable)
    }
    async fn catalog_for_setup(
        &self,
        _actor: PrincipalId,
        _runner: String,
        _setup_id: String,
    ) -> Result<catalog::Catalog> {
        Err(Error::Unavailable)
    }
    async fn compute_status(&self, _actor: PrincipalId) -> Result<compute::Status> {
        Err(Error::Unavailable)
    }
    async fn compute_command(
        &self,
        _actor: PrincipalId,
        _command: compute::Command,
    ) -> Result<Outcome> {
        Err(Error::Unavailable)
    }
    async fn settings(&self, _actor: PrincipalId) -> Result<settings::Settings> {
        Err(Error::Unavailable)
    }
    async fn save_settings(
        &self,
        _actor: PrincipalId,
        _command: settings::SettingsCommand,
    ) -> Result<settings::Settings> {
        Err(Error::Unavailable)
    }
    async fn secret(
        &self,
        _actor: PrincipalId,
        _command: settings::SecretCommand,
    ) -> Result<settings::SecretStatus> {
        Err(Error::Unavailable)
    }
    async fn metadata(
        &self,
        _actor: PrincipalId,
        _query: metadata::Query,
    ) -> Result<metadata::View> {
        Err(Error::Unavailable)
    }
    async fn metadata_command(
        &self,
        _actor: PrincipalId,
        _command: metadata::Command,
    ) -> Result<metadata::Job> {
        Err(Error::Unavailable)
    }
    async fn metadata_job(
        &self,
        _actor: PrincipalId,
        _request: RequestId,
    ) -> Result<metadata::Job> {
        Err(Error::Unavailable)
    }
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
    fn freshness(&self) -> Option<tokio::sync::broadcast::Receiver<crate::freshness::Change>> {
        None
    }
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

pub mod chatgpt;
pub mod compute;
pub mod context;
pub mod metadata;
pub mod settings;
pub mod tooling;

/// Repository-selected messages and ordinary discussion revisions, never journal authority.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "AgentExecutionFeedback")]
pub struct Feedback {
    pub messages: Vec<FeedbackMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    /// Current browser feedback write capability; does not suppress readable history.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write_unavailable: Option<String>,
}
impl Feedback {
    pub fn prompt_context(&self) -> Result<String> {
        crate::agent_feedback::context(self)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "AgentExecutionFeedbackMessage")]
pub struct FeedbackMessage {
    pub row_id: String,
    pub target: jj_collab::ReactionTargetV1,
    pub threads: Vec<jj_collab::ThreadId>,
    pub replies: Vec<FeedbackReply>,
    #[serde(default)]
    pub text_parts: Vec<FeedbackTextPart>,
    #[serde(default)]
    pub answers: Vec<jj_collab::CommentReplyLinkV1>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "AgentExecutionFeedbackReply")]
pub struct FeedbackReply {
    #[serde(default)]
    pub historical: bool,
    #[serde(default)]
    pub batches: Vec<RequestId>,
    pub thread_id: jj_collab::ThreadId,
    pub revision: jj_collab::CollabObjectRef,
    pub message: jj_collab::HumanMessageRevisionV1,
    #[serde(default)]
    pub thread_revision: Option<jj_collab::CollabObjectRef>,
    #[serde(default)]
    pub source: Option<jj_collab::ConversationSourceV1>,
    #[serde(default)]
    pub answers: Vec<jj_collab::CommentReplyLinkV1>,
}

pub mod fleet;

pub mod conversations;

pub mod catalog;

pub mod providers;

pub mod context_submission;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "AgentExecutionFeedbackTextPart")]
pub struct FeedbackTextPart {
    pub part: Option<String>,
    pub text: String,
}
