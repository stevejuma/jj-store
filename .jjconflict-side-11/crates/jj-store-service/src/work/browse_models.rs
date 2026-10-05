use super::*;
use jj_store_model::RepositoryId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum WorkBrowseKind {
    #[default]
    Projects,
    Tasks,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum WorkStateFilter {
    Active,
    Open,
    InProgress,
    Blocked,
    Done,
}
impl WorkStateFilter {
    pub(super) fn matches(self, state: TaskStateV1) -> bool {
        match self {
            Self::Active => state != TaskStateV1::Done,
            Self::Open => state == TaskStateV1::Open,
            Self::InProgress => state == TaskStateV1::InProgress,
            Self::Blocked => state == TaskStateV1::Blocked,
            Self::Done => state == TaskStateV1::Done,
        }
    }
}
fn default_limit() -> usize {
    50
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WorkBrowseQuery {
    #[serde(default)]
    pub kind: WorkBrowseKind,
    pub project: Option<ProjectId>,
    pub repository: Option<RepositoryId>,
    pub parent: Option<TaskId>,
    pub state: Option<WorkStateFilter>,
    #[serde(default)]
    pub q: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
    pub cursor: Option<String>,
}
impl super::cursors::CursorQuery for WorkBrowseQuery {
    fn cursor(&self) -> Option<&String> {
        self.cursor.as_ref()
    }
    fn clear_cursor(&mut self) {
        self.cursor = None;
    }
}
#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
pub struct WorkSummaryActor {
    pub created_by: jj_store_model::PrincipalId,
    pub updated_by: jj_store_model::PrincipalId,
    pub created_at_millis: i64,
    pub updated_at_millis: i64,
}
#[derive(Clone, Debug, Default, Serialize, schemars::JsonSchema)]
pub struct WorkTaskCounts {
    pub active: usize,
    pub done: usize,
    pub total: usize,
}
#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
pub struct WorkProjectSummary {
    pub id: ProjectId,
    pub title: String,
    pub excerpt: String,
    pub version: u64,
    pub repositories: BTreeMap<RepositoryId, bool>,
    pub task_counts: WorkTaskCounts,
    pub actor: WorkSummaryActor,
}
#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
pub struct WorkTaskSummary {
    pub id: TaskId,
    pub project: ProjectId,
    pub project_title: String,
    pub title: String,
    pub state: TaskStateV1,
    pub repository: Option<RepositoryId>,
    pub parent: Option<TaskId>,
    pub parent_title: Option<String>,
    pub comment_count: usize,
    pub subtask_count: usize,
    pub version: u64,
    pub actor: WorkSummaryActor,
}
#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
pub struct WorkBrowsePage {
    pub version: u32,
    pub snapshot: WorkReadSnapshot,
    pub projects: Vec<WorkProjectSummary>,
    pub tasks: Vec<WorkTaskSummary>,
    pub total_matches: usize,
    pub next_cursor: Option<String>,
}

impl Default for WorkBrowseQuery {
    fn default() -> Self {
        Self {
            kind: WorkBrowseKind::Projects,
            project: None,
            repository: None,
            parent: None,
            state: None,
            q: String::new(),
            limit: 50,
            cursor: None,
        }
    }
}
