//! Current discovery is separate from original historical command outcomes.
use super::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const MAX_COMBINATIONS: usize = 64;
pub const MAX_COMBINATION_STACKS: usize = 4096;
pub const MAX_WORKSPACE_RESPONSE_BYTES: usize = 8 * 1024 * 1024;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackWorkspaceRequest")]
#[serde(deny_unknown_fields)]
pub struct WorkspaceRequest {
    pub version: u32,
    pub repository: m::RepositoryId,
    pub binding_generation: u64,
    pub operation: m::OperationId,
}
impl WorkspaceRequest {
    pub fn validate(&self) -> Result<()> {
        if self.version != VERSION
            || self.repository.as_bytes() == &[0; 16]
            || self.binding_generation == 0
            || self.operation.as_bytes() == &[0; 64]
        {
            return Err(Error::Invalid);
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackAppliedStackSummary")]
#[serde(deny_unknown_fields)]
pub struct AppliedStackSummary {
    pub id: StackId,
    pub bookmark: String,
    pub head: CommitIdentity,
    pub base: CommitIdentity,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackCombinationSummary")]
#[serde(deny_unknown_fields)]
pub struct CombinationSummary {
    pub selection: CombinationSelection,
    pub name: String,
    pub target: BookmarkTarget,
    pub merge: CommitIdentity,
    pub code: CodeSelection,
    #[schemars(length(max = 4096))]
    pub stacks: Vec<AppliedStackSummary>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackCombinationCatalog")]
#[serde(deny_unknown_fields)]
pub struct CombinationCatalog {
    pub version: u32,
    pub scope: RecoveryScope,
    pub revision: u64,
    pub active: Option<CombinationId>,
    #[schemars(length(max = 64))]
    pub combinations: Vec<CombinationSummary>,
    /// Selected state remains readable while private coordination is pending.
    pub pending: bool,
}
impl CombinationCatalog {
    pub fn active(&self) -> Option<&CombinationSummary> {
        self.combinations
            .iter()
            .find(|c| Some(c.selection.id) == self.active)
    }
    pub fn validate(&self) -> Result<()> {
        self.scope.validate()?;
        if self.version != VERSION
            || self.active.is_some() == self.combinations.is_empty()
            || self.revision == 0 && !self.combinations.is_empty()
            || self.active.is_some() && self.active().is_none()
        {
            return Err(Error::Invalid);
        }
        if self.combinations.len() > MAX_COMBINATIONS {
            return Err(Error::Limit);
        }
        let mut ids = std::collections::BTreeSet::new();
        let mut names = std::collections::BTreeSet::new();
        for combination in &self.combinations {
            name(&combination.name, 256)?;
            name(&combination.target.name, 1024)?;
            if let Some(remote) = &combination.target.remote {
                name(remote, 256)?;
            }
            if combination.selection.id.0.as_bytes() == &[0; 16]
                || combination.selection.generation == 0
                || !ids.insert(combination.selection.id)
                || !names.insert(&combination.name)
            {
                return Err(Error::Invalid);
            }
            if !matches!(
                (self.scope.profile, &combination.code),
                (CodeProfile::Native, CodeSelection::Native { .. })
                    | (
                        CodeProfile::GitSha1Jj045,
                        CodeSelection::GitSha1Jj045 { .. }
                    )
            ) {
                return Err(Error::UnsupportedProfile);
            }
            combination.code.commit(combination.target.head)?;
            combination.code.commit(combination.merge)?;
            if combination.stacks.len() > MAX_COMBINATION_STACKS {
                return Err(Error::Limit);
            }
            let mut stacks = std::collections::BTreeSet::new();
            for stack in &combination.stacks {
                name(&stack.bookmark, 1024)?;
                if stack.id.0.as_bytes() == &[0; 16] || !stacks.insert(stack.id) {
                    return Err(Error::Invalid);
                }
                combination.code.commit(stack.head)?;
                combination.code.commit(stack.base)?;
            }
        }
        response_limit(self)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackWorkspaceReadiness")]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceReadiness {
    Absent,
    Ready,
    Stale,
    Pending,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackWorkspaceState")]
#[serde(deny_unknown_fields)]
pub struct WorkspaceState {
    pub version: u32,
    pub selection: Selection,
    pub state: WorkspaceReadiness,
    /// None means private state is absent; unreadable state must return an error.
    pub catalog: Option<CombinationCatalog>,
}
impl WorkspaceState {
    pub fn validate(&self) -> Result<()> {
        self.selection.validate()?;
        RecoveryScope::from(&self.selection).validate()?;
        if self.version != VERSION || self.selection.binding_generation == 0 {
            return Err(Error::Invalid);
        }
        let active = if let Some(catalog) = &self.catalog {
            catalog.validate()?;
            catalog.scope.check(&self.selection)?;
            catalog.active()
        } else {
            None
        };
        if self.selection.combination.as_ref() != active.map(|c| &c.selection) {
            return Err(Error::Invalid);
        }
        let pending = self.catalog.as_ref().is_some_and(|c| c.pending);
        let valid = match self.state {
            WorkspaceReadiness::Absent => active.is_none() && !pending,
            WorkspaceReadiness::Ready => {
                !pending && active.is_some_and(|c| c.code == self.selection.code)
            }
            WorkspaceReadiness::Stale => {
                !pending && active.is_some_and(|c| c.code != self.selection.code)
            }
            // An original journal preparation can predate combination staging.
            // Runtime must prove the pending request independently in that case.
            WorkspaceReadiness::Pending => true,
        };
        if !valid {
            return Err(Error::Invalid);
        }
        response_limit(self)
    }
    pub fn validate_request(&self, request: &WorkspaceRequest) -> Result<()> {
        request.validate()?;
        self.validate()?;
        let operation = match &self.selection.code {
            CodeSelection::Native { operation, .. }
            | CodeSelection::GitSha1Jj045 { operation, .. } => *operation,
        };
        if self.selection.repository != request.repository
            || self.selection.binding_generation != request.binding_generation
            || operation != request.operation
        {
            return Err(Error::Stale);
        }
        Ok(())
    }
}
fn name(value: &str, max: usize) -> Result<()> {
    if value.trim().is_empty() {
        return Err(Error::Invalid);
    }
    text(value, max)
}
fn response_limit(value: &impl Serialize) -> Result<()> {
    if serde_json::to_vec(value).map_err(|_| Error::Invalid)?.len() > MAX_WORKSPACE_RESPONSE_BYTES {
        return Err(Error::Limit);
    }
    Ok(())
}
