use super::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackPrepareRequest")]
#[serde(deny_unknown_fields)]
pub struct PrepareRequest {
    pub version: u32,
    pub request_id: m::RequestId,
    pub selection: Selection,
    pub command: Command,
    pub intent_digest: m::IntentDigest,
}
impl PrepareRequest {
    pub fn new(request_id: m::RequestId, selection: Selection, command: Command) -> Result<Self> {
        let mut value = Self {
            version: VERSION,
            request_id,
            selection,
            command,
            intent_digest: m::IntentDigest::new([0; 32]),
        };
        value.intent_digest = value.intent()?;
        Ok(value)
    }
    pub fn intent(&self) -> Result<m::IntentDigest> {
        if self.version != VERSION {
            return Err(Error::Invalid);
        }
        self.command.validate(&self.selection)?;
        let digest = digest(
            b"jj-store/stack-command-intent/v1\0",
            &(self.version, &self.selection, &self.command),
        )?;
        Ok(m::IntentDigest::new(*digest.as_bytes()))
    }
    pub fn validate(&self) -> Result<()> {
        if self.intent()? != self.intent_digest {
            return Err(Error::IntentMismatch);
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackApplyRequest")]
#[serde(deny_unknown_fields)]
pub struct ApplyRequest {
    pub version: u32,
    pub request_id: m::RequestId,
    pub intent_digest: m::IntentDigest,
    pub prepared: PreparedIdentity,
    pub selection: Selection,
}
impl ApplyRequest {
    /// Validate against retained server preparation. Providers separately
    /// reauthorize, look up original intent and recheck the physical checkout.
    pub fn validate(&self, retained: &PrepareRequest, prepared: PreparedIdentity) -> Result<()> {
        if self.version != VERSION {
            return Err(Error::Invalid);
        }
        retained.validate()?;
        if self.request_id != retained.request_id || self.intent_digest != retained.intent_digest {
            return Err(Error::IntentMismatch);
        }
        self.selection.check_current(&retained.selection)?;
        if self.prepared != prepared {
            return Err(Error::Stale);
        }
        Ok(())
    }
}
