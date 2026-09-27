//! Complete owned native V1 metadata. The serialized field order is also the
//! frozen persistence payload: format changes require explicit version review.
use jj_store_model as m;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Timestamp {
    pub millis: i64,
    pub offset_minutes: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Signature {
    pub name: String,
    pub email: String,
    pub timestamp: Timestamp,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecureSig {
    pub data: Vec<u8>,
    pub sig: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Commit {
    pub parents: Vec<m::CommitId>,
    pub predecessors: Vec<m::CommitId>,
    pub root_tree: Vec<m::TreeId>,
    pub conflict_labels: Vec<String>,
    pub change_id: m::ChangeId,
    pub description: String,
    pub author: Signature,
    pub committer: Signature,
    pub secure_sig: Option<SecureSig>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum TreeValue {
    File {
        id: m::FileId,
        executable: bool,
        copy: m::CopyReference,
    },
    Symlink {
        id: m::SymlinkId,
    },
    Tree {
        id: m::TreeId,
    },
    // External IDs (e.g. Git SHA-1) do not have the repository's native width.
    Submodule {
        id: Vec<u8>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tree {
    pub entries: Vec<(String, TreeValue)>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CopyHistory {
    pub current_path: String,
    pub parents: Vec<m::CopyReference>,
    pub salt: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RefTarget(pub Vec<Option<m::CommitId>>);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemoteState {
    New,
    Tracked,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RemoteRef {
    pub target: RefTarget,
    pub state: RemoteState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RemoteView {
    pub bookmarks: BTreeMap<String, RemoteRef>,
    pub tags: BTreeMap<String, RemoteRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct View {
    pub head_ids: Vec<m::CommitId>,
    pub local_bookmarks: BTreeMap<String, RefTarget>,
    pub local_tags: BTreeMap<String, RefTarget>,
    pub remote_views: BTreeMap<String, RemoteView>,
    pub git_refs: BTreeMap<String, RefTarget>,
    pub git_heads: BTreeMap<String, RefTarget>,
    pub wc_commit_ids: BTreeMap<String, m::CommitId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    pub start: Timestamp,
    pub end: Timestamp,
    pub description: String,
    pub hostname: String,
    pub username: String,
    pub is_snapshot: bool,
    pub workspace_name: Option<String>,
    pub attributes: BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Operation {
    pub view_id: m::ViewId,
    pub parents: Vec<m::OperationId>,
    pub metadata: Metadata,
    pub commit_predecessors: Option<BTreeMap<m::CommitId, Vec<m::CommitId>>>,
}
