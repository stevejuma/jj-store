use super::{CandidateV1, Claim, ConfirmationV1, ContextV1, EntryV1, Journal, PreparedV1};
use crate::capture::{CaptureBatch, CaptureError, DurableCapture, Receipt};
use jj_agent::NormalizerProfileV1;
use jj_collab::{CollabObjectV1, ThreadBindingV1, ThreadStateV1};
use jj_store_core::{AgentCommand, RepositoryStores, StoreKind, StoreOutcome, StoreRequest};
use std::sync::Arc;

/// Receipt-aware native capture sink. Construction never selects or initializes
/// a backend. Hosts retain current admission and exclusive journal ownership.
pub struct StoreCapture {
    context: ContextV1,
    stores: Arc<RepositoryStores>,
    journal: Arc<dyn Journal>,
}
impl StoreCapture {
    pub fn new(
        context: ContextV1,
        stores: Arc<RepositoryStores>,
        journal: Arc<dyn Journal>,
    ) -> Result<Self, CaptureError> {
        context.validate()?;
        stores
            .require_shared_publication(&[StoreKind::Agent, StoreKind::Thread])
            .map_err(|_| CaptureError::Rejected)?;
        for scope in [
            stores.agent().unwrap().scope(),
            stores.thread().unwrap().scope(),
        ] {
            if scope.repository != Some(context.repository) {
                return Err(CaptureError::Invalid);
            }
        }
        Ok(Self {
            context,
            stores,
            journal,
        })
    }

    /// Reauthorize and verify the original exact request. This method never
    /// prepares, executes, reconstructs output frames or dispatches a prompt.
    pub async fn recover(&self, ordinal: u64) -> Result<ConfirmationV1, CaptureError> {
        if ordinal == 0 {
            return Err(CaptureError::Invalid);
        }
        let entry = self
            .journal
            .load(self.context.request(ordinal))
            .await?
            .ok_or(CaptureError::Unknown)?;
        if entry.candidate.ordinal != ordinal || entry.candidate.context != self.context {
            return Err(CaptureError::ReceiptMismatch);
        }
        self.replay(entry).await
    }

    async fn replay(&self, entry: EntryV1) -> Result<ConfirmationV1, CaptureError> {
        entry.validate()?;
        let prepared = entry.prepared.as_ref().ok_or(CaptureError::Unknown)?;
        let request = prepared.request(&entry.candidate)?;
        // Bare status and current snapshots do not verify the original intent.
        let outcome = self
            .stores
            .agent()
            .unwrap()
            .replay(&request)
            .await
            .map_err(|_| CaptureError::Unknown)?
            .ok_or(CaptureError::Unknown)?;
        self.confirm(
            &entry.candidate,
            prepared,
            entry.confirmation.as_ref(),
            outcome,
        )
        .await
    }

    async fn confirm(
        &self,
        candidate: &CandidateV1,
        prepared: &PreparedV1,
        prior: Option<&ConfirmationV1>,
        outcome: StoreOutcome<jj_store_core::AgentResult>,
    ) -> Result<ConfirmationV1, CaptureError> {
        let StoreOutcome::Committed(receipt) = outcome else {
            return Err(CaptureError::Unknown);
        };
        let confirmation = ConfirmationV1::from_receipt(candidate, prepared, receipt)?;
        if prior.is_some_and(|prior| prior != &confirmation) {
            return Err(CaptureError::ReceiptMismatch);
        }
        // An uncertain journal ACK cannot authorize output even after commit.
        self.journal
            .confirm(candidate, &confirmation)
            .await
            .map_err(|_| CaptureError::Unknown)?;
        Ok(confirmation)
    }

    async fn publish(&self, candidate: &CandidateV1) -> Result<(), CaptureError> {
        // Never turn a pre-existing provider outcome without retained preparation
        // into permission to prepare or execute it again.
        match self
            .stores
            .agent()
            .unwrap()
            .status(candidate.request())
            .await
            .map_err(|_| CaptureError::Unknown)?
        {
            StoreOutcome::NotKnown { request_id } if request_id == candidate.request() => {}
            _ => return Err(CaptureError::Unknown),
        }
        let pin = self
            .stores
            .thread()
            .unwrap()
            .snapshot()
            .await
            .map_err(|_| CaptureError::Rejected)?;
        if pin.scope.repository != Some(self.context.repository) {
            return Err(CaptureError::ReceiptMismatch);
        }
        let state = ThreadStateV1::read(pin.data.root.clone(), &pin.data.graph)
            .map_err(|_| CaptureError::Rejected)?;
        let session = candidate.session()?;
        let assignment = state.assignments.get(&session).copied();
        if let Some(reference) = assignment {
            let CollabObjectV1::ThreadLink(link) = pin
                .data
                .graph
                .get(reference)
                .map_err(|_| CaptureError::Rejected)?
            else {
                return Err(CaptureError::Rejected);
            };
            if link.thread_id != self.context.thread {
                return Err(CaptureError::Rejected);
            }
        }
        let binding = ThreadBindingV1 {
            thread_id: self.context.thread,
            observed_thread: *state
                .threads
                .get(&self.context.thread)
                .ok_or(CaptureError::Rejected)?,
            observed_assignment: assignment,
        };
        let request = StoreRequest::new(
            candidate.request(),
            AgentCommand::Capture {
                profile: NormalizerProfileV1::native_capture_v1(),
                batch: Box::new(candidate.observation()?),
                binding: Some(binding.clone()),
            },
        );
        let request = self
            .stores
            .prepare_agent(&request)
            .await
            .map_err(|_| CaptureError::Rejected)?;
        let prepared = PreparedV1::from_request(candidate, &binding, request)?;
        self.journal.prepared(candidate, &prepared).await?;
        let request = prepared.request(candidate)?;
        // This is the only execute path. Errors are uncertain; recovery may only read.
        let outcome = self
            .stores
            .execute_agent(&request)
            .await
            .map_err(|_| CaptureError::Unknown)?;
        self.confirm(candidate, &prepared, None, outcome).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl DurableCapture for StoreCapture {
    async fn persist(&mut self, batch: &CaptureBatch) -> Result<Receipt, CaptureError> {
        let candidate = CandidateV1::new(self.context.clone(), batch)?;
        // Must precede all provider IO, including status and preparation reads.
        match self.journal.begin(&candidate).await? {
            Claim::New => self.publish(&candidate).await?,
            Claim::Existing(entry) => {
                if entry.candidate != candidate {
                    return Err(CaptureError::ReceiptMismatch);
                }
                self.replay(*entry).await?;
            }
        }
        Ok(candidate.receipt())
    }
}
