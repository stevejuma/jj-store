use super::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackCommandRetirement")]
#[serde(rename_all = "snake_case")]
pub enum Retirement {
    Undone,
    Abandoned,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackCheckoutState")]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum CheckoutState {
    Required,
    Blocked { reason: String },
    Verified { fingerprint: m::RecordDigest },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackReceiptState")]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum ReceiptState {
    Prepared,
    /// Recovery must inspect the original prepared operation. No reexecution.
    PublicationUnknown,
    Published {
        operation: m::OperationId,
        view: m::ViewId,
        checkout: CheckoutState,
    },
    Rejected {
        reason: String,
    },
    Retired {
        operation: m::OperationId,
        reason: Retirement,
    },
}
/// Historical outcome only. Refresh obtains a distinct current workspace view.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackCommandReceipt")]
#[serde(deny_unknown_fields)]
pub struct Receipt {
    pub version: u32,
    pub request_id: m::RequestId,
    pub intent_digest: m::IntentDigest,
    pub selection: Selection,
    pub prepared: Option<PreparedIdentity>,
    pub state: ReceiptState,
}
impl Receipt {
    pub fn validate(&self, retained: &PrepareRequest) -> Result<()> {
        if self.version != VERSION {
            return Err(Error::Invalid);
        }
        retained.validate()?;
        if self.request_id != retained.request_id
            || self.intent_digest != retained.intent_digest
            || self.selection != retained.selection
        {
            return Err(Error::IntentMismatch);
        }
        if !matches!(self.state, ReceiptState::Rejected { .. }) && self.prepared.is_none() {
            return Err(Error::Invalid);
        }
        match &self.state {
            ReceiptState::Rejected { reason }
            | ReceiptState::Published {
                checkout: CheckoutState::Blocked { reason },
                ..
            } => text(reason, 4096)?,
            _ => (),
        }
        Ok(())
    }
    /// A prepared record allows apply of its retained candidates, not another
    /// preparation. Every post-publication/unknown/retired state denies it.
    pub fn may_execute(&self) -> bool {
        self.prepared.is_some() && matches!(self.state, ReceiptState::Prepared)
    }
    pub fn completed(&self) -> bool {
        matches!(
            self.state,
            ReceiptState::Published {
                checkout: CheckoutState::Verified { .. },
                ..
            }
        )
    }
    /// Original preparation/publication/checkout discovery only; terminal
    /// receipts remain available by exact status but cannot occupy this list.
    pub fn pending(&self) -> bool {
        matches!(
            self.state,
            ReceiptState::Prepared
                | ReceiptState::PublicationUnknown
                | ReceiptState::Published {
                    checkout: CheckoutState::Required | CheckoutState::Blocked { .. },
                    ..
                }
        )
    }
}
