//! Public historical command projections; no private execution or live pin.
use super::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const MAX_PENDING_COMMANDS: usize = 128;
pub const MAX_COMMAND_RESPONSE_BYTES: usize = 8 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackCodeProfile")]
#[serde(rename_all = "snake_case")]
pub enum CodeProfile {
    Native,
    GitSha1Jj045,
}
/// Stable request-discovery scope. Current code/binding/combination generations
/// cannot stand in for the original preparation or prevent historical lookup.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackRecoveryScope")]
#[serde(deny_unknown_fields)]
pub struct RecoveryScope {
    pub repository: m::RepositoryId,
    pub physical_workspace: PhysicalWorkspaceId,
    pub jj_workspace: String,
    pub profile: CodeProfile,
}
impl From<&Selection> for RecoveryScope {
    fn from(value: &Selection) -> Self {
        Self {
            repository: value.repository,
            physical_workspace: value.physical_workspace,
            jj_workspace: value.jj_workspace.clone(),
            profile: match value.code {
                CodeSelection::Native { .. } => CodeProfile::Native,
                CodeSelection::GitSha1Jj045 { .. } => CodeProfile::GitSha1Jj045,
            },
        }
    }
}
impl RecoveryScope {
    pub fn validate(&self) -> Result<()> {
        if self.repository.as_bytes() == &[0; 16]
            || self.physical_workspace.0.as_bytes() == &[0; 32]
            || self.jj_workspace.trim().is_empty()
        {
            return Err(Error::Invalid);
        }
        text(&self.jj_workspace, 256)
    }
    pub fn check(&self, selection: &Selection) -> Result<()> {
        self.validate()?;
        selection.validate()?;
        if *self != Self::from(selection) {
            return Err(Error::IntentMismatch);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackCommandStatusRequest")]
#[serde(deny_unknown_fields)]
pub struct StatusRequest {
    pub version: u32,
    pub scope: RecoveryScope,
    pub request_id: m::RequestId,
    pub intent_digest: m::IntentDigest,
}
impl StatusRequest {
    pub fn validate(&self) -> Result<()> {
        if self.version != VERSION
            || self.request_id.as_bytes() == &[0; 16]
            || self.intent_digest.as_bytes() == &[0; 32]
        {
            return Err(Error::Invalid);
        }
        self.scope.validate()
    }
    pub fn check(&self, original: &PrepareRequest) -> Result<()> {
        self.validate()?;
        original.validate()?;
        self.scope.check(&original.selection)?;
        if self.request_id != original.request_id || self.intent_digest != original.intent_digest {
            return Err(Error::IntentMismatch);
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackPendingCommandsRequest")]
#[serde(deny_unknown_fields)]
pub struct PendingRequest {
    pub version: u32,
    pub scope: RecoveryScope,
}
impl PendingRequest {
    pub fn validate(&self) -> Result<()> {
        if self.version != VERSION {
            return Err(Error::Invalid);
        }
        self.scope.validate()
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackCancelPreparationRequest")]
#[serde(deny_unknown_fields)]
pub struct CancelRequest {
    pub version: u32,
    pub request: StatusRequest,
    pub prepared: PreparedIdentity,
}
impl CancelRequest {
    pub fn validate(&self) -> Result<()> {
        if self.version != VERSION || self.prepared.0.as_bytes() == &[0; 32] {
            return Err(Error::Invalid);
        }
        self.request.validate()
    }
    /// Identity validation only; runtime separately authorizes and proves that
    /// publication has not started before coordinating a durable cancellation.
    pub fn validate_prepared(&self, preview: &Preview) -> Result<()> {
        self.validate()?;
        preview.validate()?;
        self.request.check(&preview.plan.request)?;
        if self.prepared != preview.prepared {
            return Err(Error::IntentMismatch);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackCommandResult")]
#[serde(deny_unknown_fields)]
pub struct CommandResult {
    pub version: u32,
    pub preview: Preview,
    pub receipt: Receipt,
    pub replayed: bool,
}
impl CommandResult {
    pub fn validate(&self) -> Result<()> {
        if self.version != VERSION {
            return Err(Error::Invalid);
        }
        RecoveryScope::from(&self.preview.plan.request.selection).validate()?;
        self.receipt.validate_prepared(&self.preview)?;
        response_limit(self)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackCommandStatus")]
#[serde(deny_unknown_fields)]
pub struct CommandStatus {
    pub version: u32,
    pub request: StatusRequest,
    pub result: Option<CommandResult>,
}
impl CommandStatus {
    pub fn validate(&self) -> Result<()> {
        if self.version != VERSION {
            return Err(Error::Invalid);
        }
        self.request.validate()?;
        if let Some(result) = &self.result {
            result.validate()?;
            self.request.check(&result.preview.plan.request)?;
        }
        response_limit(self)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackPendingCommands")]
#[serde(deny_unknown_fields)]
pub struct PendingCommands {
    pub version: u32,
    pub request: PendingRequest,
    pub commands: Vec<CommandResult>,
}
impl PendingCommands {
    pub fn validate(&self) -> Result<()> {
        if self.version != VERSION {
            return Err(Error::Invalid);
        }
        self.request.validate()?;
        if self.commands.len() > MAX_PENDING_COMMANDS {
            return Err(Error::Limit);
        }
        let mut requests = std::collections::BTreeSet::new();
        for command in &self.commands {
            command.validate()?;
            self.request.scope.check(&command.receipt.selection)?;
            if !command.receipt.pending() || !requests.insert(command.receipt.request_id) {
                return Err(Error::Invalid);
            }
        }
        response_limit(self)
    }
}
fn response_limit(value: &impl Serialize) -> Result<()> {
    if serde_json::to_vec(value).map_err(|_| Error::Invalid)?.len() > MAX_COMMAND_RESPONSE_BYTES {
        return Err(Error::Limit);
    }
    Ok(())
}
