use super::CandidateV1;
use crate::capture::CaptureError;
use jj_agent::{AgentDeltaV1, NormalizerProfileV1};
use jj_collab::ThreadBindingV1;
use jj_store_core::{AgentCommand, PreparedAgentDelta, PreparedThreadBinding, StoreRequest};
use jj_store_model::{PrincipalId, PublicationId, RequestId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum PreparedCommandV1 {
    Capture {
        binding: ThreadBindingV1,
    },
    Native {
        actor: PrincipalId,
        delta: AgentDeltaV1,
        result: Vec<u8>,
        binding: ThreadBindingV1,
        accepted_at_millis: i64,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreparedV1 {
    pub schema: u32,
    pub capture_digest: [u8; 32],
    pub request_id: RequestId,
    pub publication_id: Option<PublicationId>,
    pub command: PreparedCommandV1,
}
impl PreparedV1 {
    pub(crate) fn from_request(
        candidate: &CandidateV1,
        observed: &ThreadBindingV1,
        request: StoreRequest<AgentCommand>,
    ) -> Result<Self, CaptureError> {
        let command = match request.command {
            AgentCommand::Capture {
                profile,
                batch,
                binding: Some(binding),
            } if profile == NormalizerProfileV1::native_capture_v1()
                && *batch == candidate.observation()?
                && &binding == observed =>
            {
                PreparedCommandV1::Capture { binding }
            }
            AgentCommand::Prepared { delta } => {
                let binding = delta.thread_binding.ok_or(CaptureError::ReceiptMismatch)?;
                if &binding.binding != observed {
                    return Err(CaptureError::ReceiptMismatch);
                }
                PreparedCommandV1::Native {
                    actor: delta.actor,
                    delta: delta.delta,
                    result: delta.result,
                    binding: binding.binding,
                    accepted_at_millis: binding.accepted_at_millis,
                }
            }
            _ => return Err(CaptureError::ReceiptMismatch),
        };
        let value = Self {
            schema: 1,
            capture_digest: candidate.capture_digest,
            request_id: request.request_id,
            publication_id: request.publication_id,
            command,
        };
        value.validate(candidate)?;
        Ok(value)
    }
    pub fn validate(&self, candidate: &CandidateV1) -> Result<(), CaptureError> {
        candidate.validate()?;
        if self.schema != 1
            || self.request_id != candidate.request()
            || self.capture_digest != candidate.capture_digest
        {
            return Err(CaptureError::ReceiptMismatch);
        }
        let binding = match &self.command {
            PreparedCommandV1::Capture { binding } => binding,
            PreparedCommandV1::Native {
                delta,
                result,
                binding,
                accepted_at_millis,
                ..
            } => {
                delta.validate().map_err(|_| CaptureError::Invalid)?;
                if self.publication_id.is_none()
                    || *accepted_at_millis < 0
                    || result.len() > 2 * 1024 * 1024
                {
                    return Err(CaptureError::Invalid);
                }
                binding
            }
        };
        binding.validate().map_err(|_| CaptureError::Invalid)?;
        if binding.thread_id != candidate.context.thread {
            return Err(CaptureError::ReceiptMismatch);
        }
        Ok(())
    }
    pub(crate) fn request(
        &self,
        candidate: &CandidateV1,
    ) -> Result<StoreRequest<AgentCommand>, CaptureError> {
        self.validate(candidate)?;
        let command = match &self.command {
            PreparedCommandV1::Capture { binding } => AgentCommand::Capture {
                profile: NormalizerProfileV1::native_capture_v1(),
                batch: Box::new(candidate.observation()?),
                binding: Some(binding.clone()),
            },
            PreparedCommandV1::Native {
                actor,
                delta,
                result,
                binding,
                accepted_at_millis,
            } => AgentCommand::Prepared {
                delta: Box::new(PreparedAgentDelta {
                    actor: *actor,
                    delta: delta.clone(),
                    result: result.clone(),
                    thread_binding: Some(PreparedThreadBinding {
                        binding: binding.clone(),
                        accepted_at_millis: *accepted_at_millis,
                    }),
                }),
            },
        };
        Ok(StoreRequest {
            request_id: self.request_id,
            publication_id: self.publication_id,
            command,
        })
    }
}
