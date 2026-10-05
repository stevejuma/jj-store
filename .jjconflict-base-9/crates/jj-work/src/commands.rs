use crate::*;
use jj_store_model::RepositoryId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "command", rename_all = "snake_case", deny_unknown_fields)]
pub enum WorkCommandV1 {
    CreateProject {
        id: ProjectId,
        title: String,
        body: String,
    },
    UpdateProject {
        id: ProjectId,
        expected_version: u64,
        title: String,
        body: String,
    },
    SetRepository {
        project: ProjectId,
        expected_version: u64,
        repository: RepositoryId,
        active: bool,
    },
    CreateTask {
        id: TaskId,
        project: ProjectId,
        expected_project_version: u64,
        title: String,
        body: String,
        repository: Option<RepositoryId>,
        parent: Option<TaskId>,
    },
    UpdateTask {
        id: TaskId,
        expected_version: u64,
        expected_project_version: u64,
        title: String,
        body: String,
        state: TaskStateV1,
        repository: Option<RepositoryId>,
        parent: Option<TaskId>,
    },
    AddComment {
        id: WorkCommentId,
        task: TaskId,
        expected_task_version: u64,
        reply_to: Option<WorkCommentId>,
        body: String,
    },
    EditComment {
        id: WorkCommentId,
        expected_version: u64,
        expected_task_version: u64,
        body: String,
    },
    TombstoneComment {
        id: WorkCommentId,
        expected_version: u64,
        expected_task_version: u64,
    },
}
impl WorkCommandV1 {
    pub fn validate(&self) -> Result<()> {
        use WorkCommandV1::*;
        match self {
            CreateProject { title, body, .. }
            | UpdateProject { title, body, .. }
            | CreateTask { title, body, .. }
            | UpdateTask { title, body, .. } => {
                crate::validation::title(title)?;
                crate::validation::body(body)?;
            }
            AddComment { body, .. } | EditComment { body, .. } => {
                crate::validation::body(body)?;
                if body.trim().is_empty() {
                    return Err(Error::Invalid("empty comment"));
                }
            }
            _ => {}
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WorkCommandResultV1 {
    pub project: Option<ProjectV1>,
    pub task: Option<TaskV1>,
    pub comment: Option<WorkCommentV1>,
}
