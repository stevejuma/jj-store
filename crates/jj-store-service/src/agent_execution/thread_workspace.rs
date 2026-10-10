//! Explicit installation-local Thread creation and Codebase configuration.
use super::*;
use jj_collab::{CollabObjectRef, ThreadId};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "ThreadWorkspaceRequest")]
pub struct Request {
    pub version: u32,
    pub request_id: RequestId,
    pub command: Command,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(rename = "ThreadWorkspaceCommand")]
pub enum Command {
    Create {
        title: String,
        repositories: Vec<RepositoryId>,
    },
    Codebase {
        home: RepositoryId,
        thread: ThreadId,
        expected: CollabObjectRef,
        repositories: Vec<RepositoryId>,
    },
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "ThreadWorkspaceOutcome")]
pub struct Outcome {
    pub request_id: RequestId,
    pub home: RepositoryId,
    pub thread: ThreadId,
    pub revision: CollabObjectRef,
}
