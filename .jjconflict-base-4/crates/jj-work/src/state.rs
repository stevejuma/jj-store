use crate::*;
use jj_store_model::{PrincipalId, RepositoryId};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const MAX_ENTITIES: usize = 4096;
pub const MAX_BODY_BYTES: usize = 64 * 1024;
pub const MAX_TOTAL_BODY_BYTES: usize = 8 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskStateV1 {
    Open,
    InProgress,
    Blocked,
    Done,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WorkActorV1 {
    pub created_by: PrincipalId,
    pub updated_by: PrincipalId,
    pub created_at_millis: i64,
    pub updated_at_millis: i64,
}
impl WorkActorV1 {
    pub(crate) fn new(actor: PrincipalId, at: i64) -> Self {
        Self {
            created_by: actor,
            updated_by: actor,
            created_at_millis: at,
            updated_at_millis: at,
        }
    }
    pub(crate) fn update(&mut self, actor: PrincipalId, at: i64) {
        self.updated_by = actor;
        self.updated_at_millis = at.max(self.created_at_millis);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProjectV1 {
    pub schema_version: u32,
    pub id: ProjectId,
    pub title: String,
    pub body: String,
    pub version: u64,
    /// False preserves a retired membership and its historical task targets.
    pub repositories: BTreeMap<RepositoryId, bool>,
    pub actor: WorkActorV1,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TaskV1 {
    pub schema_version: u32,
    pub id: TaskId,
    pub project: ProjectId,
    pub title: String,
    pub body: String,
    pub state: TaskStateV1,
    pub repository: Option<RepositoryId>,
    pub parent: Option<TaskId>,
    pub version: u64,
    pub actor: WorkActorV1,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WorkCommentV1 {
    pub schema_version: u32,
    pub id: WorkCommentId,
    pub task: TaskId,
    pub reply_to: Option<WorkCommentId>,
    /// None is an explicit tombstone; historical bodies stay in selected WAL roots.
    pub body: Option<String>,
    pub version: u64,
    pub actor: WorkActorV1,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WorkStateV1 {
    pub projects: BTreeMap<ProjectId, ProjectV1>,
    pub tasks: BTreeMap<TaskId, TaskV1>,
    pub comments: BTreeMap<WorkCommentId, WorkCommentV1>,
}
