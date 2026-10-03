//! Sanitization is the only bridge from memory-only native rows to reader intent.
use crate::{
    CaptureResult, codex_transcript::CodexTranscriptBatch, reader_progress::ReaderCapture,
};
use jj_agent::{
    IntegrationId, NormalizerProfileV1, ObservationId, SanitizedMediaType,
    SanitizedObservationBatchV1, SanitizedObservationV1, SanitizedPayloadV1,
    SanitizedRepresentation, SourceKind, SourceTrust,
};
use jj_store_model::RepositoryId;
use kingfisher_scanner::Scanner;
use serde_json::{Value, json};
use sha2::{Digest as _, Sha256};
use std::path::Path;
pub const TRANSCRIPT_POLICY: &str = "mongodb-kingfisher@2.7.0/builtin-medium/transcript-paths/v1";
pub struct TranscriptSanitizer {
    scanner: Scanner,
    replacements: Vec<(String, &'static str)>,
}
impl TranscriptSanitizer {
    pub fn new(
        workspace: &Path,
        home: &Path,
        temporary: &Path,
        cache: &Path,
    ) -> CaptureResult<Self> {
        let mut replacements = Vec::new();
        for (path, tag) in [
            (workspace, "$WORKSPACE"),
            (home, "$HOME"),
            (temporary, "$TEMP"),
        ] {
            let text = path.to_str().ok_or("invalid transcript privacy root")?;
            if !path.is_absolute()
                || path.parent().is_none()
                || text.contains('\0')
                || path.components().any(|v| {
                    matches!(
                        v,
                        std::path::Component::ParentDir | std::path::Component::CurDir
                    )
                })
            {
                return Err("transcript privacy roots must be absolute non-root paths".into());
            }
            let text = text.trim_end_matches('/');
            if replacements.iter().any(|(old, _)| old == text) {
                return Err("transcript privacy roots must be distinct".into());
            }
            replacements.push((text.to_owned(), tag));
        }
        replacements.sort_by_key(|(path, _)| std::cmp::Reverse(path.len()));
        let cache = crate::path_policy::outside_checkout(cache)?;
        Ok(Self {
            scanner: crate::agent_sanitization::scanner(&cache)?,
            replacements,
        })
    }
    pub(crate) fn sanitize(&self, mut value: Value) -> Option<SanitizedPayloadV1> {
        scrub(&mut value, &self.replacements)?;
        crate::agent_sanitization::scan_json(value, &self.scanner)
    }
}
fn scrub(value: &mut Value, replacements: &[(String, &'static str)]) -> Option<()> {
    match value {
        Value::String(text) => {
            for (path, tag) in replacements {
                if text.contains(path) {
                    *text = text.replace(path, tag);
                }
            }
        }
        Value::Array(values) => {
            for v in values {
                scrub(v, replacements)?;
            }
        }
        Value::Object(values) => {
            let mut replacement = serde_json::Map::new();
            for (mut key, mut value) in std::mem::take(values) {
                for (path, tag) in replacements {
                    if key.contains(path) {
                        key = key.replace(path, tag);
                    }
                }
                scrub(&mut value, replacements)?;
                if replacement.insert(key, value).is_some() {
                    return None;
                }
            }
            *values = replacement;
        }
        _ => {}
    }
    Some(())
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
        profile.validate()?;
        self.cursor.validate()?;
        if profile.integration != IntegrationId::Codex
            || self.cursor.integration != IntegrationId::Codex
            || self.cursor.parser_version != format!("codex-jsonl@{}/v1", profile.product_version)
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
                            sanitizer.sanitize(json!({
                    "capture_format":"codex-jsonl/v1", "product_version":profile.product_version,
                    "session_id":session, "record":native
                }))
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
            integration: IntegrationId::Codex,
            source_instance_id: self.cursor.source_instance_id,
            sanitizer_version: TRANSCRIPT_POLICY.into(),
            representation: SanitizedRepresentation::SanitizedNative,
            observations,
        };
        batch.validate()?;
        Ok(ReaderCapture {
            repository,
            profile: profile.clone(),
            cursor: self.cursor.clone(),
            batch,
            metadata: self.metadata.clone(),
            thread_binding: None,
        })
    }
}
