//! Profile-aware source identities. These carry evidence, never access grants.
use crate::{CodeCapsuleId, CommitId, GitSha1Id, ObjectKey, OperationId, RepositoryId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CodeProfileV1 {
    NativeV1,
    GitSha1Jj045,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(
    tag = "profile",
    content = "id",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum CodeCommitIdV1 {
    NativeV1(CommitId),
    GitSha1Jj045(GitSha1Id),
}
impl CodeCommitIdV1 {
    pub fn profile(self) -> CodeProfileV1 {
        match self {
            Self::NativeV1(_) => CodeProfileV1::NativeV1,
            Self::GitSha1Jj045(_) => CodeProfileV1::GitSha1Jj045,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum GitObjectKindV1 {
    Commit,
    Tree,
    Blob,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "profile", rename_all = "snake_case", deny_unknown_fields)]
pub enum CodeObjectIdV1 {
    NativeV1 {
        object: ObjectKey,
    },
    GitSha1Jj045 {
        kind: GitObjectKindV1,
        id: GitSha1Id,
    },
}
impl CodeObjectIdV1 {
    pub fn profile(self) -> CodeProfileV1 {
        match self {
            Self::NativeV1 { .. } => CodeProfileV1::NativeV1,
            Self::GitSha1Jj045 { .. } => CodeProfileV1::GitSha1Jj045,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CodeEvidenceRefV1 {
    pub repository: RepositoryId,
    pub profile: CodeProfileV1,
    pub operation: OperationId,
    pub capsule: CodeCapsuleId,
}
