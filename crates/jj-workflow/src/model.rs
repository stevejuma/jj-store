use crate::*;
use jj_store_model as m;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const MAX_ENTITIES: usize = 10_000;
pub const MAX_TEXT: usize = 64 * 1024;
pub const MAX_EVIDENCE: usize = 128 * 1024;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "WorkflowDefinition")]
pub struct Definition {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub enabled: bool,
    pub repository: m::RepositoryId,
    pub trigger: Trigger,
    #[serde(default, skip_serializing_if = "Conditions::is_empty")]
    pub conditions: Conditions,
    #[serde(default)]
    pub labels: BTreeSet<String>,
    pub actions: Vec<Action>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "on", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(rename = "WorkflowTrigger")]
pub enum Trigger {
    Manual,
    CodePublished,
    MessagePosted,
    ReactionAdded,
    BookmarkChanged { bookmark: String },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "WorkflowAction")]
pub struct Action {
    pub id: String,
    pub executor: Executor,
    #[serde(default)]
    pub inputs: BTreeMap<String, String>,
    #[serde(default = "timeout")]
    pub timeout_seconds: u64,
}
fn timeout() -> u64 {
    300
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(rename = "WorkflowExecutor")]
pub enum Executor {
    Command { name: String },
    Agent { agent: String, prompt: String },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "WorkflowDefinitionVersion")]
pub struct DefinitionVersion {
    pub id: WorkflowId,
    pub version: VersionId,
    pub owner: m::PrincipalId,
    pub created_at: i64,
    pub definition: Definition,
}
/// Source anchors stay distinct from workflow and source-event identities.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "WorkflowSource")]
pub struct Source {
    pub repository: m::RepositoryId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<m::OperationId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<m::CommitId>,
    pub bookmark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation: Option<Conversation>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "WorkflowSourceEvent")]
pub struct SourceEvent {
    pub id: SourceEventId,
    /// Canonical origin key/digest, or issuing repository/epoch/selected receipt.
    pub publication: String,
    pub kind: EventKind,
    pub source: Source,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub filter_results: BTreeMap<FilterId, bool>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(rename = "WorkflowEventKind")]
pub enum EventKind {
    MessagePosted,
    ReactionAdded,
    CodePublished,
    BookmarkChanged,
}
/// Exact conversation anchors, never a message body copied into the workflow home.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "WorkflowConversationSource")]
pub struct Conversation {
    pub thread: jj_collab::ThreadId,
    pub message: Option<jj_collab::HumanMessageId>,
    pub revision: jj_collab::CollabObjectRef,
    pub reaction: Option<jj_collab::CollabObjectRef>,
    pub position: m::WalPosition,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "WorkflowEvaluation")]
pub struct Evaluation {
    pub event: SourceEvent,
    pub evaluated_at: i64,
    pub definitions: BTreeMap<WorkflowId, VersionId>,
    pub runs: Vec<RunId>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(rename = "WorkflowRunStatus")]
pub enum RunStatus {
    Pending,
    Running,
    CancelRequested,
    Completed,
    Failed,
    Cancelled,
    Unknown,
}
impl RunStatus {
    pub fn terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "WorkflowAttempt")]
pub struct Attempt {
    pub id: ExecutionAttemptId,
    pub owner: Option<m::PrincipalId>,
    pub host: Option<String>,
    pub status: RunStatus,
    pub sequence: u64,
    pub started_at: Option<i64>,
    pub updated_at: i64,
    pub completed_at: Option<i64>,
    pub result: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "WorkflowRun")]
pub struct Run {
    pub id: RunId,
    pub workflow: WorkflowId,
    pub version: VersionId,
    pub event: Option<SourceEventId>,
    pub source: Source,
    pub requester: m::PrincipalId,
    pub created_at: i64,
    pub attempts: Vec<Attempt>,
}
impl Run {
    pub fn current(&self) -> &Attempt {
        &self.attempts[self.attempts.len() - 1]
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(rename = "WorkflowEvidenceData")]
pub enum EvidenceData {
    Log {
        stream: String,
        text: String,
    },
    Step {
        step: String,
        status: RunStatus,
        output: BTreeMap<String, String>,
    },
    Artifact {
        name: String,
        media_type: String,
        bytes: Vec<u8>,
    },
    Agent {
        session: String,
        request_id: m::RequestId,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "WorkflowEvidence")]
pub struct Evidence {
    pub id: EvidenceId,
    pub run: RunId,
    pub attempt: ExecutionAttemptId,
    pub sequence: u64,
    pub at: i64,
    pub data: EvidenceData,
}
pub fn text(value: &str, max: usize) -> Result<()> {
    if value.len() > max {
        return Err(Error::Limit);
    }
    if value.contains('\0') {
        return Err(Error::Invalid("NUL in text"));
    }
    Ok(())
}
pub fn name(value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > 128
        || !value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_.-".contains(&b))
    {
        return Err(Error::Invalid("name"));
    }
    Ok(())
}
impl Definition {
    pub fn parse_yaml(yaml: &str) -> Result<Self> {
        text(yaml, MAX_TEXT)?;
        let value: Self =
            serde_yaml::from_str(yaml).map_err(|_| Error::Invalid("YAML definition"))?;
        value.validate()?;
        Ok(value)
    }
    pub fn validate(&self) -> Result<()> {
        text(&self.name, 256)?;
        if self.name.trim().is_empty() {
            return Err(Error::Invalid("empty name"));
        }
        text(&self.description, 4096)?;
        self.conditions.validate()?;
        if matches!(self.trigger, Trigger::ReactionAdded) && self.conditions.text.is_some() {
            return Err(Error::Invalid("reaction text condition"));
        }
        if matches!(self.trigger, Trigger::Manual) && !self.conditions.is_empty()
            || !matches!(
                self.trigger,
                Trigger::CodePublished | Trigger::BookmarkChanged { .. }
            ) && self.conditions.revset.is_some()
            || !matches!(
                self.trigger,
                Trigger::MessagePosted | Trigger::ReactionAdded
            ) && (self.conditions.thread.is_some() || self.conditions.message.is_some())
            || !matches!(self.trigger, Trigger::ReactionAdded)
                && (self.conditions.emoji.is_some() || self.conditions.excluded_emoji.is_some())
        {
            return Err(Error::Invalid("conditions incompatible with trigger"));
        }
        if let Trigger::BookmarkChanged { bookmark } = &self.trigger {
            text(bookmark, 1024)?;
            if bookmark.is_empty() {
                return Err(Error::Invalid("bookmark"));
            }
        }
        if self.labels.len() > 32 || self.actions.is_empty() || self.actions.len() > 32 {
            return Err(Error::Limit);
        }
        for label in &self.labels {
            name(label)?;
        }
        let mut ids = BTreeSet::new();
        for action in &self.actions {
            name(&action.id)?;
            if !ids.insert(&action.id) {
                return Err(Error::Invalid("duplicate action"));
            }
            if action.timeout_seconds == 0
                || action.timeout_seconds > 86400
                || action.inputs.len() > 64
            {
                return Err(Error::Limit);
            }
            for (key, value) in &action.inputs {
                name(key)?;
                text(value, 4096)?;
            }
            match &action.executor {
                Executor::Command { name: command } => name(command)?,
                Executor::Agent { agent, prompt } => {
                    name(agent)?;
                    text(prompt, MAX_TEXT)?;
                    if prompt.trim().is_empty() || action.timeout_seconds > 3600 {
                        return Err(Error::Invalid("agent"));
                    }
                }
            }
        }
        Ok(())
    }
    pub fn accepts_kind(&self, event: &SourceEvent) -> bool {
        self.enabled
            && self.repository == event.source.repository
            && match &self.trigger {
                Trigger::Manual => false,
                Trigger::CodePublished => event.kind == EventKind::CodePublished,
                Trigger::MessagePosted => event.kind == EventKind::MessagePosted,
                Trigger::ReactionAdded => event.kind == EventKind::ReactionAdded,
                Trigger::BookmarkChanged { bookmark } => {
                    event.kind == EventKind::BookmarkChanged
                        && event
                            .source
                            .bookmark
                            .as_deref()
                            .is_some_and(|value| bookmark_matches(bookmark, value))
                }
            }
    }
    pub fn matches(&self, event: &SourceEvent) -> Result<bool> {
        if !self.accepts_kind(event) {
            return Ok(false);
        }
        if self.conditions.is_empty() {
            return Ok(true);
        }
        event
            .filter_results
            .get(&self.conditions.id()?)
            .copied()
            .ok_or(Error::Invalid("source condition result missing"))
    }
}
impl Source {
    pub fn validate(&self) -> Result<()> {
        use jj_collab::CollabObjectKind as K;
        if self.operation.is_some() != self.commit.is_some()
            || self.commit.is_none() && self.conversation.is_none()
        {
            return Err(Error::Invalid("source anchor"));
        }
        if let Some(context) = &self.conversation {
            let kind = if context.message.is_some() {
                K::HumanMessageRevision
            } else {
                K::ThreadVersion
            };
            if context.position.0 == 0
                || context.revision.kind != kind
                || context.reaction.is_some_and(|r| r.kind != K::Reaction)
            {
                return Err(Error::Invalid("conversation anchor"));
            }
        }
        Ok(())
    }
}
impl SourceEvent {
    pub fn validate(&self) -> Result<()> {
        use sha2::{Digest, Sha256};
        self.source.validate()?;
        if self.filter_results.len() > MAX_ENTITIES {
            return Err(Error::Limit);
        }
        if self.source.operation.is_some() != self.source.commit.is_some() {
            return Err(Error::Invalid("partial code anchor"));
        }
        let conversational = matches!(
            self.kind,
            EventKind::MessagePosted | EventKind::ReactionAdded
        );
        if conversational != self.source.conversation.is_some()
            || (!conversational && self.source.commit.is_none())
        {
            return Err(Error::Invalid("event anchor"));
        }
        if let Some(context) = &self.source.conversation
            && (context.position.0 == 0
                || (self.kind == EventKind::ReactionAdded) != context.reaction.is_some()
                || self.kind == EventKind::MessagePosted && context.message.is_none())
        {
            return Err(Error::Invalid("conversation event"));
        }
        text(&self.publication, 2048)?;
        if self.publication.is_empty() {
            return Err(Error::Invalid("source publication"));
        }
        if let Some(bookmark) = &self.source.bookmark {
            text(bookmark, 1024)?;
            if bookmark.is_empty() {
                return Err(Error::Invalid("source bookmark"));
            }
        }
        if (self.kind == EventKind::BookmarkChanged) != self.source.bookmark.is_some() {
            return Err(Error::Invalid("event kind and source"));
        }
        let encoded = serde_json::to_vec(&(&self.publication, &self.kind, &self.source))
            .map_err(|_| Error::Invalid("event encoding"))?;
        if self.id.as_bytes() != &<[u8; 32]>::from(Sha256::digest(encoded)) {
            return Err(Error::Invalid("source event identity"));
        }
        Ok(())
    }
    pub fn new(publication: String, kind: EventKind, source: Source) -> Result<Self> {
        use sha2::{Digest, Sha256};
        let bytes = serde_json::to_vec(&(&publication, &kind, &source))
            .map_err(|_| Error::Invalid("event encoding"))?;
        let event = Self {
            id: SourceEventId::new(Sha256::digest(bytes).into()),
            publication,
            kind,
            source,
            filter_results: Default::default(),
        };
        event.validate()?;
        Ok(event)
    }
}
impl EvidenceData {
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Log {
                stream,
                text: value,
            } => {
                name(stream)?;
                text(value, MAX_TEXT)?;
            }
            Self::Step { step, output, .. } => {
                name(step)?;
                if output.len() > 64 {
                    return Err(Error::Limit);
                }
                for (key, value) in output {
                    name(key)?;
                    text(value, 4096)?;
                }
            }
            Self::Artifact {
                name: path,
                media_type,
                bytes,
            } => {
                text(path, 1024)?;
                text(media_type, 256)?;
                if path.is_empty()
                    || path.starts_with('/')
                    || path.split('/').any(|p| p == ".." || p.is_empty())
                    || path.contains('\\')
                {
                    return Err(Error::Invalid("artifact path"));
                }
                if bytes.len() > MAX_EVIDENCE {
                    return Err(Error::Limit);
                }
            }
            Self::Agent { session, .. } => {
                text(session, 256)?;
                if session.is_empty() {
                    return Err(Error::Invalid("session"));
                }
            }
        }
        Ok(())
    }
}
