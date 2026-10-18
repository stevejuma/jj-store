use crate::capture::{CaptureBatch, CaptureError, Receipt};
use jj_agent::*;
use jj_collab::ThreadId;
use jj_store_model::{RepositoryId, RequestId};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::result::Result;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextV1 {
    pub repository: RepositoryId,
    pub thread: ThreadId,
    pub binding: NativeCaptureBindingV1,
    pub policy: String,
}
impl ContextV1 {
    pub fn validate(&self) -> Result<(), CaptureError> {
        self.binding.validate().map_err(|_| CaptureError::Invalid)?;
        if self.policy.is_empty()
            || self.policy.len() > 128
            || self.policy.chars().any(char::is_control)
        {
            return Err(CaptureError::Invalid);
        }
        Ok(())
    }
    /// Repository-scoped native source. Thread assignment and policy never rename a session.
    pub fn source(&self) -> SourceInstanceId {
        let mut hash = Sha256::new_with_prefix(b"jj-store/native-capture-source/v1\0");
        hash.update(self.repository.as_bytes());
        for value in [
            &self.binding.scope,
            &self.binding.worker,
            &self.binding.request,
            &self.binding.target,
            &self.binding.window,
            &self.binding.admission,
        ] {
            hash.update((value.len() as u64).to_be_bytes());
            hash.update(value.as_bytes());
        }
        SourceInstanceId::new(hash.finalize()[..16].try_into().expect("SHA-256 prefix"))
    }
    /// One intent per ordinal; changed payloads cannot obtain a new request identity.
    pub fn request(&self, ordinal: u64) -> RequestId {
        RequestId::new(identity(
            b"jj-store/native-capture-request/v1\0",
            self.source().as_bytes(),
            ordinal,
        ))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateV1 {
    pub schema: u32,
    pub context: ContextV1,
    pub ordinal: u64,
    pub capture: Vec<u8>,
    pub capture_digest: [u8; 32],
}
impl CandidateV1 {
    pub(crate) fn new(context: ContextV1, batch: &CaptureBatch) -> Result<Self, CaptureError> {
        let envelope =
            NativeCaptureEnvelopeV1::decode(batch.bytes()).map_err(|_| CaptureError::Invalid)?;
        let value = Self {
            schema: 1,
            context,
            ordinal: envelope.ordinal,
            capture: batch.bytes().to_vec(),
            capture_digest: batch.receipt().0,
        };
        value.validate()?;
        Ok(value)
    }
    pub fn request(&self) -> RequestId {
        self.context.request(self.ordinal)
    }
    pub fn receipt(&self) -> Receipt {
        Receipt(self.capture_digest)
    }
    pub fn validate(&self) -> Result<(), CaptureError> {
        self.context.validate()?;
        let envelope =
            NativeCaptureEnvelopeV1::decode(&self.capture).map_err(|_| CaptureError::Invalid)?;
        if self.schema != 1
            || self.ordinal == 0
            || envelope.ordinal != self.ordinal
            || envelope.binding != self.context.binding
            || envelope.policy != self.context.policy
            || <[u8; 32]>::from(
                Sha256::new_with_prefix(b"jj-store/native-capture/v1\0")
                    .chain_update(&self.capture)
                    .finalize(),
            ) != self.capture_digest
        {
            return Err(CaptureError::ReceiptMismatch);
        }
        Ok(())
    }
    pub fn batch_id(&self) -> CaptureBatchId {
        CaptureBatchId::new(identity(
            b"jj-store/native-capture-batch/v1\0",
            self.context.source().as_bytes(),
            self.ordinal,
        ))
    }
    pub(crate) fn observation(&self) -> Result<SanitizedObservationBatchV1, CaptureError> {
        self.validate()?;
        Ok(SanitizedObservationBatchV1 {
            schema_version: 1,
            batch_id: self.batch_id(),
            integration: IntegrationId::OwnedController,
            source_instance_id: self.context.source(),
            sanitizer_version: self.context.policy.clone(),
            representation: SanitizedRepresentation::SanitizedNative,
            observations: vec![SanitizedObservationV1 {
                observation_id: ObservationId::new(identity(
                    b"jj-store/native-capture-observation/v1\0",
                    self.context.source().as_bytes(),
                    self.ordinal,
                )),
                source_sequence: self.ordinal,
                occurred_at_millis: None,
                source_kind: SourceKind::NativeApi,
                source_trust: SourceTrust::LocallyTrustedPlugin,
                payload: SanitizedPayloadV1::new(SanitizedMediaType::Json, &self.capture)
                    .map_err(|_| CaptureError::Limit)?,
            }],
        })
    }
    pub(crate) fn session(&self) -> Result<AgentSessionId, CaptureError> {
        let normalized = normalize_batch(
            &NormalizerProfileV1::native_capture_v1(),
            &self.observation()?,
        )
        .map_err(|_| CaptureError::Invalid)?;
        let mut sessions = normalized
            .aliases
            .iter()
            .filter_map(|alias| match alias.target {
                AgentEntityRefV1::Session { session_id } => Some(session_id),
                _ => None,
            });
        let value = sessions.next().ok_or(CaptureError::Invalid)?;
        if sessions.any(|session| session != value) {
            return Err(CaptureError::Invalid);
        }
        Ok(value)
    }
}
fn identity(domain: &[u8], source: &[u8], ordinal: u64) -> [u8; 16] {
    Sha256::new_with_prefix(domain)
        .chain_update(source)
        .chain_update(ordinal.to_be_bytes())
        .finalize()[..16]
        .try_into()
        .expect("SHA-256 prefix")
}
