//! Native privacy adaptation and JJ observation identities.
use crate::{
    CaptureResult, codex_transcript::CodexTranscriptBatch, reader_progress::ReaderCapture,
};
use jj_agent::{
    IntegrationId, NormalizerProfileV1, ObservationId, SanitizedMediaType,
    SanitizedObservationBatchV1, SanitizedObservationV1, SanitizedPayloadV1,
    SanitizedRepresentation, SourceKind, SourceTrust,
};
use jj_store_model::RepositoryId;
pub use lapis_agent_redaction::TRANSCRIPT_POLICY;
use serde_json::{Value, json};
use sha2::{Digest as _, Sha256};
use std::path::Path;
#[derive(Clone)]
pub struct TranscriptSanitizer(lapis_agent_redaction::TranscriptSanitizer);
impl TranscriptSanitizer {
    pub fn new(
        workspace: &Path,
        home: &Path,
        temporary: &Path,
        cache: &Path,
    ) -> CaptureResult<Self> {
        let cache = crate::path_policy::outside_checkout(cache)?;
        Ok(Self(lapis_agent_redaction::TranscriptSanitizer::new(
            workspace, home, temporary, &cache,
        )?))
    }
    pub fn for_workspace(&self, workspace: &Path) -> CaptureResult<Self> {
        Ok(Self(self.0.for_workspace(workspace)?))
    }
    pub fn for_secrets(&self, secrets: &[String]) -> CaptureResult<Self> {
        Ok(Self(self.0.for_secrets(secrets)?))
    }
    pub fn for_oauth_secrets(&self, secrets: &[String]) -> CaptureResult<Self> {
        Ok(Self(self.0.for_oauth_secrets(secrets)?))
    }
    pub fn sanitize(&self, value: Value) -> Option<SanitizedPayloadV1> {
        let payload = self.0.sanitize(value)?;
        SanitizedPayloadV1::new(SanitizedMediaType::Json, payload.bytes()).ok()
    }
    /// Intermediate exact replacement; callers must scan the returned record.
    pub fn replace_stream_text(
        &self,
        pending: &mut String,
        chunk: &str,
        complete: bool,
    ) -> CaptureResult<String> {
        Ok(self.0.replace_stream_text(pending, chunk, complete)?)
    }
}
impl CodexTranscriptBatch {
    pub fn sanitize(
        &self,
        repository: RepositoryId,
        profile: &NormalizerProfileV1,
        sanitizer: &TranscriptSanitizer,
    ) -> CaptureResult<ReaderCapture> {
        if self.directory().is_none() {
            return Err("transcript directory context unavailable".into());
        }
        self.convert(repository, profile, Some(sanitizer))
    }
    /// Retain a safe gap and source progress without inspecting/copying source bodies.
    /// The caller still needs current write authority for the chosen anchor repository.
    pub fn sanitize_omitted(
        &self,
        repository: RepositoryId,
        profile: &NormalizerProfileV1,
    ) -> CaptureResult<ReaderCapture> {
        self.convert(repository, profile, None)
    }
    fn convert(
        &self,
        repository: RepositoryId,
        profile: &NormalizerProfileV1,
        sanitizer: Option<&TranscriptSanitizer>,
    ) -> CaptureResult<ReaderCapture> {
        Ok(ReaderCapture {
            repository,
            profile: profile.clone(),
            cursor: self.cursor.clone(),
            batch: self.observations(profile, sanitizer)?,
            metadata: self.metadata.clone(),
            thread_binding: None,
        })
    }
    /// Native-only privacy projection; grants no durable progress or repository authority.
    pub fn sanitized_observations(
        &self,
        profile: &NormalizerProfileV1,
        sanitizer: &TranscriptSanitizer,
    ) -> CaptureResult<SanitizedObservationBatchV1> {
        if self.directory().is_none() {
            return Err("transcript directory context unavailable".into());
        }
        self.observations(profile, Some(sanitizer))
    }
    fn observations(
        &self,
        profile: &NormalizerProfileV1,
        sanitizer: Option<&TranscriptSanitizer>,
    ) -> CaptureResult<SanitizedObservationBatchV1> {
        profile.validate()?;
        self.cursor.validate()?;
        if !matches!(
            profile.integration,
            IntegrationId::Codex | IntegrationId::Cursor
        ) || self.cursor.integration != profile.integration
            || self.cursor.parser_version
                != format!(
                    "{}@{}/v1",
                    if profile.integration == IntegrationId::Cursor {
                        "cursor-jsonl"
                    } else {
                        "codex-jsonl"
                    },
                    profile.product_version
                )
        {
            return Err("transcript sanitization profile mismatch".into());
        }
        let session = self
            .cursor
            .native_session_key
            .as_ref()
            .ok_or("transcript sanitization session missing")?;
        if !crate::codex_transcript::native_session_key_valid(session)
            || self.records.is_empty()
            || self.records.len() > crate::codex_transcript::MAX_BATCH_RECORDS
        {
            return Err("invalid transcript source identity or count".into());
        }
        self.metadata.validate()?;
        let mut observations = Vec::with_capacity(self.records.len());
        for record in self.records.iter() {
            let payload = sanitizer
                .and_then(|sanitizer| {
                    record
                        .bytes
                        .as_deref()
                        .filter(|bytes| bytes.len() <= crate::codex_transcript::MAX_RECORD_BYTES)
                        .and_then(|bytes| serde_json::from_slice::<Value>(bytes).ok())
                        .filter(|v| v.is_object())
                        .and_then(|native| {
                            let mut envelope=json!({"capture_format":if profile.integration==IntegrationId::Cursor {"cursor-jsonl/v1"} else {"codex-jsonl/v1"}, "product_version":profile.product_version,"session_id":session,"record":native});
                            if profile.integration==IntegrationId::Cursor {
                                envelope["record_key"]=json!(record.record_key.map(|k| k.iter().map(|b|format!("{b:02x}")).collect::<String>()));
                            }
                            sanitizer.sanitize(envelope)
                        })
                })
                .unwrap_or_else(|| {
                    SanitizedPayloadV1::new(
                        SanitizedMediaType::Json,
                        if sanitizer.is_none() {
                            br#"{"capture_omitted":true,"reason":"unverified_directory_context"}"#
                        } else {
                            br#"{"capture_omitted":true,"reason":"unavailable_native_record"}"#
                        },
                    )
                    .expect("constant safe omission")
                });
            let mut id = Sha256::new_with_prefix(b"jj-store/codex-transcript-observation/v1\0");
            id.update(self.batch_id.as_bytes());
            id.update(record.sequence.to_be_bytes());
            let digest = id.finalize();
            observations.push(SanitizedObservationV1 {
                observation_id: ObservationId::new(digest[..16].try_into().expect("digest length")),
                source_sequence: record.sequence,
                occurred_at_millis: None,
                source_kind: SourceKind::Transcript,
                source_trust: SourceTrust::LocalTranscript,
                payload,
            });
        }
        let batch = SanitizedObservationBatchV1 {
            schema_version: 1,
            batch_id: self.batch_id,
            integration: profile.integration,
            source_instance_id: self.cursor.source_instance_id,
            sanitizer_version: TRANSCRIPT_POLICY.into(),
            representation: SanitizedRepresentation::SanitizedNative,
            observations,
        };
        batch.validate()?;
        Ok(batch)
    }
}
