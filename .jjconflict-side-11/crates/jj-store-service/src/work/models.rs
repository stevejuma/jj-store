use super::*;
use jj_store_model as m;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct WorkReadSnapshot {
    pub repo_id: m::RepositoryId,
    pub epoch: m::EpochId,
    pub published_seq: m::WalPosition,
    pub manifest_revision: u64,
    pub work_root: Option<m::RecordDigest>,
    pub work_state_version: u64,
}
fn default_limit() -> usize {
    50
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WorkQuery {
    pub project: Option<ProjectId>,
    pub task: Option<TaskId>,
    #[serde(default = "default_limit")]
    pub limit: usize,
    pub cursor: Option<String>,
}
impl Default for WorkQuery {
    fn default() -> Self {
        Self {
            project: None,
            task: None,
            limit: default_limit(),
            cursor: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct WorkPage {
    pub version: u32,
    pub snapshot: WorkReadSnapshot,
    pub projects: Vec<ProjectV1>,
    pub tasks: Vec<TaskV1>,
    pub comments: Vec<WorkCommentV1>,
    pub next_cursor: Option<String>,
}
