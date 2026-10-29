//! JJ payload adapter for the independently owned native privacy library.
use jj_agent::{SanitizedMediaType, SanitizedPayloadV1};
pub use lapis_agent_redaction::{KINGFISHER_POLICY, MAX_HOOK_BYTES, kingfisher_cache_ready};

pub fn warm_kingfisher_cache(cache: &Path) -> crate::CaptureResult<()> {
    Ok(lapis_agent_redaction::warm_kingfisher_cache(cache)?)
}
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SanitizedHookPayload {
    Sanitized {
        payload: SanitizedPayloadV1,
        policy: &'static str,
    },
    Omitted,
}
pub fn sanitize_hook_payload(raw: &[u8], workspace: &Path, cache: &Path) -> SanitizedHookPayload {
    match lapis_agent_redaction::sanitize_hook_payload(raw, workspace, cache) {
        lapis_agent_redaction::SanitizedHookPayload::Sanitized { payload, policy } => {
            match SanitizedPayloadV1::new(SanitizedMediaType::Json, payload.bytes()) {
                Ok(payload) => SanitizedHookPayload::Sanitized { payload, policy },
                Err(_) => SanitizedHookPayload::Omitted,
            }
        }
        lapis_agent_redaction::SanitizedHookPayload::Omitted => SanitizedHookPayload::Omitted,
    }
}
