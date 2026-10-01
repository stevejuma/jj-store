use super::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "LocalGraphRequest")]
#[serde(tag = "graph", rename_all = "snake_case", deny_unknown_fields)]
pub enum GraphRequest {
    Log {
        revset: String,
        workspace: Option<String>,
        user_email: String,
        at_time_millis: i64,
    },
    Operations,
    Evolution {
        commit: m::GitSha1Id,
    },
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "LocalGraphPageRequest")]
#[serde(deny_unknown_fields)]
pub struct GraphWindow {
    pub limit: usize,
    pub after: Option<ObjectKey>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalGraphTimestamp")]
pub struct Timestamp {
    pub millis: i64,
    pub offset_minutes: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalGraphSignature")]
pub struct Signature {
    pub name: String,
    pub email: String,
    pub timestamp: Timestamp,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalGraphBookmark")]
pub struct Bookmark {
    pub name: String,
    pub remote: Option<String>,
    pub added: bool,
    pub removed: bool,
    pub conflicted: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalGraphEdgeKind")]
#[serde(rename_all = "snake_case")]
pub enum EdgeKind {
    Direct,
    Indirect,
    Missing,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalGraphEdge")]
pub struct Edge {
    pub target: m::GitSha1Id,
    pub kind: EdgeKind,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalGraphCommit")]
pub struct Commit {
    pub id: m::GitSha1Id,
    pub change_id: m::ChangeId,
    pub parents: Vec<m::GitSha1Id>,
    pub description: String,
    pub author: Signature,
    pub committer: Signature,
    pub tree_terms: Vec<m::GitSha1Id>,
    pub conflicted: bool,
    pub bookmarks: Vec<Bookmark>,
    pub workspaces: Vec<String>,
    pub visible: bool,
    pub divergent: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalGraphCommitNode")]
pub struct CommitNode {
    pub commit: Commit,
    pub edges: Vec<Edge>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalGraphOperationNode")]
pub struct OperationNode {
    pub id: m::OperationId,
    pub parents: Vec<m::OperationId>,
    pub view: m::ViewId,
    pub description: String,
    pub user: String,
    pub host: String,
    pub start: Timestamp,
    pub end: Timestamp,
    pub is_snapshot: bool,
    pub workspace: Option<String>,
    pub attributes: std::collections::BTreeMap<String, String>,
    pub predecessor_data_present: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalGraphEvolutionNode")]
pub struct EvolutionNode {
    pub commit: Commit,
    pub operation: Option<m::OperationId>,
    pub predecessors: Vec<m::GitSha1Id>,
    pub predecessors_known: bool,
    pub legacy_predecessors: Vec<m::GitSha1Id>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalGraphNodes")]
#[serde(tag = "graph", content = "nodes", rename_all = "snake_case")]
pub enum Nodes {
    Log(Vec<CommitNode>),
    Operations(Vec<OperationNode>),
    Evolution(Vec<EvolutionNode>),
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "LocalGraphStats")]
pub struct Stats {
    pub metadata_nodes: u64,
    pub predicate_file_bytes: u64,
    pub predicate_lines: u64,
    pub object_reads: u64,
    pub object_bytes: u64,
    pub work: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalGraphPage")]
pub struct GraphPage {
    pub version: u32,
    pub operation: m::OperationId,
    pub data: Nodes,
    pub next: Option<ObjectKey>,
    pub stats: Stats,
}
