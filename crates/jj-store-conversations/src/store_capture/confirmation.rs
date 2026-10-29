use super::{CandidateV1, PreparedCommandV1, PreparedV1};
use crate::capture::CaptureError;
use jj_agent::CaptureBatchId;
use jj_store_core::{AgentResult, StoreReceipt};
use jj_store_model::{PrincipalId, RecordDigest, RepositoryId, RequestId};
use serde::{Deserialize, Serialize};

/// Original provider proof, retained verbatim in the private host journal.
/// Loading this value never grants authority or proves that replay succeeded.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfirmationV1 {
    pub schema: u32,
    pub request_id: RequestId,
    pub actor: PrincipalId,
    pub repository: RepositoryId,
    pub version: u64,
    pub selection: Vec<u8>,
    pub provider_receipt: Vec<u8>,
    pub root: RecordDigest,
    pub batch: Option<CaptureBatchId>,
}
impl ConfirmationV1 {
    pub(crate) fn from_receipt(
        candidate: &CandidateV1,
        prepared: &PreparedV1,
        receipt: StoreReceipt<AgentResult>,
    ) -> Result<Self, CaptureError> {
        let value = Self {
            schema: 1,
            request_id: receipt.request_id,
            actor: receipt.actor,
            repository: receipt.repository,
            version: receipt.version,
            selection: receipt.selection,
            provider_receipt: receipt.provider_receipt,
            root: receipt.result.root,
            batch: receipt.result.batch,
        };
        value.validate(candidate, prepared)?;
        Ok(value)
    }
    pub fn validate(
        &self,
        candidate: &CandidateV1,
        prepared: &PreparedV1,
    ) -> Result<(), CaptureError> {
        prepared.validate(candidate)?;
        if self.schema != 1
            || self.request_id != candidate.request()
            || self.repository != candidate.context.repository
            || self.version == 0
            || self.selection.is_empty()
            || self.selection.len() > 1024 * 1024
            || self.provider_receipt.is_empty()
            || self.provider_receipt.len() > 2 * 1024 * 1024
            || self.batch.is_some_and(|id| id != candidate.batch_id())
            || matches!(prepared.command, PreparedCommandV1::Capture { .. })
                && self.batch != Some(candidate.batch_id())
            || matches!(&prepared.command, PreparedCommandV1::Native { actor, .. } if *actor != self.actor)
        {
            return Err(CaptureError::ReceiptMismatch);
        }
        Ok(())
    }
}
