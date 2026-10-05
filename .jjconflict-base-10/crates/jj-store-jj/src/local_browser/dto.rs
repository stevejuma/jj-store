//! Git-profile content values. These are transport DTOs, never native codecs.
use super::*;
pub use crate::local_evidence::{
    GitCommitV1 as Commit, GitOperationV1 as Operation, GitRefTargetV1 as RefTarget,
    GitRemoteViewV1 as RemoteView, GitViewV1 as View,
};
pub use crate::native_dto::Signature;
use jj_lib::object_id::ObjectId as _;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalCopyReference")]
#[serde(tag = "kind", content = "id", rename_all = "snake_case")]
pub enum CopyReference {
    Placeholder,
    Object(String),
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalTreeValue")]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TreeValue {
    File {
        id: m::GitSha1Id,
        executable: bool,
        copy: CopyReference,
    },
    Symlink {
        id: m::GitSha1Id,
    },
    Tree {
        id: m::GitSha1Id,
    },
    Submodule {
        id: Vec<u8>,
    },
}
impl TryFrom<&b::TreeValue> for TreeValue {
    type Error = Error;
    fn try_from(value: &b::TreeValue) -> Result<Self> {
        Ok(match value {
            b::TreeValue::File {
                id,
                executable,
                copy_id,
            } => Self::File {
                id: git_id(id)?,
                executable: *executable,
                copy: if copy_id.as_bytes().is_empty() {
                    CopyReference::Placeholder
                } else {
                    CopyReference::Object(copy_id.hex())
                },
            },
            b::TreeValue::Symlink(id) => Self::Symlink { id: git_id(id)? },
            b::TreeValue::Tree(id) => Self::Tree { id: git_id(id)? },
            b::TreeValue::GitSubmodule(id) => Self::Submodule {
                id: id.as_bytes().to_vec(),
            },
        })
    }
}
