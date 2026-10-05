//! Explicit Git-backed source records; native V1 records are separate.
use crate::native_dto::{Metadata, RemoteState, SecureSig, Signature};
use jj_store_model as m;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "GitSourceCommitV1")]
#[serde(deny_unknown_fields)]
pub struct GitCommitV1 {
    pub parents: Vec<m::GitSha1Id>,
    pub predecessors: Vec<m::GitSha1Id>,
    pub root_tree: Vec<m::GitSha1Id>,
    pub conflict_labels: Vec<String>,
    pub change_id: m::ChangeId,
    pub description: String,
    pub author: Signature,
    pub committer: Signature,
    pub secure_sig: Option<SecureSig>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "GitSourceRefTargetV1")]
#[serde(transparent)]
pub struct GitRefTargetV1(pub Vec<Option<m::GitSha1Id>>);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "GitSourceRemoteRefV1")]
#[serde(deny_unknown_fields)]
pub struct GitRemoteRefV1 {
    pub target: GitRefTargetV1,
    pub state: RemoteState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "GitSourceRemoteViewV1")]
#[serde(deny_unknown_fields)]
pub struct GitRemoteViewV1 {
    pub bookmarks: BTreeMap<String, GitRemoteRefV1>,
    pub tags: BTreeMap<String, GitRemoteRefV1>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "GitSourceViewV1")]
#[serde(deny_unknown_fields)]
pub struct GitViewV1 {
    pub head_ids: Vec<m::GitSha1Id>,
    pub local_bookmarks: BTreeMap<String, GitRefTargetV1>,
    pub local_tags: BTreeMap<String, GitRefTargetV1>,
    pub remote_views: BTreeMap<String, GitRemoteViewV1>,
    pub git_refs: BTreeMap<String, GitRefTargetV1>,
    pub git_heads: BTreeMap<String, GitRefTargetV1>,
    pub wc_commit_ids: BTreeMap<String, m::GitSha1Id>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "GitSourceOperationV1")]
#[serde(deny_unknown_fields)]
pub struct GitOperationV1 {
    pub view_id: m::ViewId,
    pub parents: Vec<m::OperationId>,
    pub metadata: Metadata,
    pub commit_predecessors: Option<BTreeMap<m::GitSha1Id, Vec<m::GitSha1Id>>>,
}
