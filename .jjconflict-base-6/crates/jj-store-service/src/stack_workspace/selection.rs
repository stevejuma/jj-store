use super::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

macro_rules! identity {
    ($name:ident, $inner:ty) => {
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            Serialize,
            Deserialize,
            JsonSchema,
        )]
        #[serde(transparent)]
        pub struct $name(pub $inner);
    };
}
identity!(PhysicalWorkspaceId, m::RecordDigest);
identity!(CombinationId, m::RequestId);
identity!(StackId, m::RequestId);
identity!(SelectionToken, m::RecordDigest);
identity!(PreparedIdentity, m::RecordDigest);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackCommitIdentity")]
#[serde(
    tag = "profile",
    content = "id",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum CommitIdentity {
    Native(m::CommitId),
    GitSha1(m::GitSha1Id),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackTreeIdentity")]
#[serde(
    tag = "profile",
    content = "id",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum TreeIdentity {
    Native(m::TreeId),
    GitSha1(m::GitSha1Id),
}
/// Ordered merged-tree terms; a normal tree has one term. Native and Git
/// conflicts retain all remove/add terms instead of an invented single tree.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackTreeSelection")]
#[serde(deny_unknown_fields)]
pub struct TreeSelection {
    pub terms: Vec<TreeIdentity>,
}
impl From<TreeIdentity> for TreeSelection {
    fn from(tree: TreeIdentity) -> Self {
        Self { terms: vec![tree] }
    }
}
impl TreeSelection {
    pub fn validate(&self, code: &CodeSelection) -> Result<()> {
        if self.terms.is_empty() || self.terms.len() > 65 || self.terms.len().is_multiple_of(2) {
            return Err(Error::Invalid);
        }
        for term in &self.terms {
            code.tree(*term)?;
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackCodeSelection")]
#[serde(tag = "profile", rename_all = "snake_case", deny_unknown_fields)]
pub enum CodeSelection {
    Native {
        operation: m::OperationId,
        view: m::ViewId,
        working_copy: m::CommitId,
    },
    GitSha1Jj045 {
        operation: m::OperationId,
        view: m::ViewId,
        working_copy: m::GitSha1Id,
    },
}
impl CodeSelection {
    pub fn commit(&self, identity: CommitIdentity) -> Result<()> {
        match (self, identity) {
            (Self::Native { .. }, CommitIdentity::Native(_))
            | (Self::GitSha1Jj045 { .. }, CommitIdentity::GitSha1(_)) => Ok(()),
            _ => Err(Error::UnsupportedProfile),
        }
    }
    pub fn tree(&self, identity: TreeIdentity) -> Result<()> {
        match (self, identity) {
            (Self::Native { .. }, TreeIdentity::Native(_))
            | (Self::GitSha1Jj045 { .. }, TreeIdentity::GitSha1(_)) => Ok(()),
            _ => Err(Error::UnsupportedProfile),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackCombinationSelection")]
#[serde(deny_unknown_fields)]
pub struct CombinationSelection {
    pub id: CombinationId,
    pub generation: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackWorkspaceSelection")]
#[serde(deny_unknown_fields)]
pub struct Selection {
    pub version: u32,
    pub repository: m::RepositoryId,
    pub binding_generation: u64,
    pub physical_workspace: PhysicalWorkspaceId,
    pub jj_workspace: String,
    pub code: CodeSelection,
    /// None before explicit first-use setup. Reads never create a combination.
    pub combination: Option<CombinationSelection>,
    pub working_tree: m::RecordDigest,
}
impl Selection {
    pub fn validate(&self) -> Result<()> {
        if self.version != VERSION {
            return Err(Error::Invalid);
        }
        text(&self.jj_workspace, 256)
    }
    pub fn check_current(&self, current: &Self) -> Result<()> {
        self.validate()?;
        current.validate()?;
        if self != current {
            return Err(Error::Stale);
        }
        Ok(())
    }
}
