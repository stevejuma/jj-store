use super::{CandidateV1, ConfirmationV1, PreparedV1};
use crate::capture::CaptureError;
use jj_store_model::RequestId;
use serde::{Deserialize, Serialize};

/// Private recovery record. It never substitutes for provider authorization or
/// receipt replay. Hosts must bind their journal to one selected backend.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntryV1 {
    pub schema: u32,
    pub candidate: CandidateV1,
    pub prepared: Option<PreparedV1>,
    pub confirmation: Option<ConfirmationV1>,
}
impl EntryV1 {
    pub fn new(candidate: CandidateV1) -> Result<Self, CaptureError> {
        candidate.validate()?;
        Ok(Self {
            schema: 1,
            candidate,
            prepared: None,
            confirmation: None,
        })
    }
    pub fn validate(&self) -> Result<(), CaptureError> {
        self.candidate.validate()?;
        if self.schema != 1 {
            return Err(CaptureError::Invalid);
        }
        if let Some(prepared) = &self.prepared {
            prepared.validate(&self.candidate)?;
        }
        if let Some(confirmation) = &self.confirmation {
            confirmation.validate(
                &self.candidate,
                self.prepared
                    .as_ref()
                    .ok_or(CaptureError::ReceiptMismatch)?,
            )?;
        }
        Ok(())
    }
}

pub enum Claim {
    /// Exclusively claimed and durable before any provider read/preparation.
    New,
    /// Never authorizes another preparation or execute, even if incomplete.
    Existing(Box<EntryV1>),
}

/// Host-owned durable journal with an exclusive backend/scope lease. Each
/// transition must be atomic and durable before returning, retaining exact
/// candidates, preparation and original receipts across crash/reopen. Conflicting
/// bytes under one request must fail. No method may detach its writes; the host
/// drains admitted calls before releasing its lease or native authority.
#[async_trait::async_trait]
pub trait Journal: Send + Sync {
    async fn load(&self, request: RequestId) -> Result<Option<EntryV1>, CaptureError>;
    async fn begin(&self, candidate: &CandidateV1) -> Result<Claim, CaptureError>;
    async fn prepared(
        &self,
        candidate: &CandidateV1,
        prepared: &PreparedV1,
    ) -> Result<(), CaptureError>;
    async fn confirm(
        &self,
        candidate: &CandidateV1,
        confirmation: &ConfirmationV1,
    ) -> Result<(), CaptureError>;
}
