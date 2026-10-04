use jj_collab::{HumanMessageId, ThreadId};
use jj_store_model as m;
use jj_store_wal::publication::{ProjectId, TaskId, WorkCommentId};
use serde::{Deserialize, Serialize};
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ContentSection {
    #[default]
    All,
    Threads,
    Projects,
    Tasks,
}
fn limit() -> usize {
    50
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ContentQuery {
    #[serde(default)]
    pub q: String,
    #[serde(default)]
    pub section: ContentSection,
    pub thread: Option<ThreadId>,
    pub project: Option<ProjectId>,
    pub task: Option<TaskId>,
    #[serde(default = "limit")]
    pub limit: usize,
    pub cursor: Option<String>,
}
impl Default for ContentQuery {
    fn default() -> Self {
        Self {
            q: String::new(),
            section: ContentSection::All,
            thread: None,
            project: None,
            task: None,
            limit: limit(),
            cursor: None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ContentKind {
    Thread,
    Message,
    Project,
    Task,
    Comment,
}
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ContentHit {
    pub kind: ContentKind,
    pub id: String,
    pub title: String,
    pub excerpt: String,
    pub actor: m::PrincipalId,
    pub at_millis: i64,
    pub revision: String,
    pub thread: Option<ThreadId>,
    pub message: Option<HumanMessageId>,
    pub project: Option<ProjectId>,
    pub task: Option<TaskId>,
    pub comment: Option<WorkCommentId>,
}
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ContentSnapshot {
    pub repo_id: m::RepositoryId,
    pub epoch: m::EpochId,
    pub manifest_digest: m::RecordDigest,
    pub published_seq: m::WalPosition,
    pub thread_root: Option<m::RecordDigest>,
    pub work_root: Option<m::RecordDigest>,
}
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ContentPage {
    pub version: u32,
    pub snapshot: ContentSnapshot,
    pub results: Vec<ContentHit>,
    pub next_cursor: Option<String>,
}
