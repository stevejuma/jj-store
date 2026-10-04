//! One context-partitioned snapshot observation; progress follows sanitized durability.
use crate::{
    CaptureResult,
    opencode_snapshot::{OpenCodeMessageSnapshot, native_id_valid},
    reader_progress::{ReaderCapture, ReaderCaptureMetadataV1},
    transcript_sanitization::{TRANSCRIPT_POLICY, TranscriptSanitizer},
};
pub use jj_agent::{CaptureBatchId, IntegrationId, SourcePositionV1};
use jj_agent::{
    CaptureCursorV1, NormalizerProfileV1, ObservationId, SanitizedMediaType,
    SanitizedObservationBatchV1, SanitizedObservationV1, SanitizedPayloadV1,
    SanitizedRepresentation, SourceInstanceId, SourceKind, SourceTrust,
};
use jj_store_model::RepositoryId;
use sha2::{Digest as _, Sha256};
use std::path::Path;

#[derive(Clone, Copy)]
pub enum OpenCodeEvidence {
    Api,
    Export,
}
/// Allocation comes from the exclusively owned source and verified source checkpoint.
/// The adapter does not allocate sequences, acknowledge pages or authorize repositories.
pub struct OpenCodeRecordPosition {
    pub source: SourceInstanceId,
    pub generation: u64,
    pub session_witness: [u8; 32],
    pub sequence: u64,
    pub logical_records: u64,
    pub prefix_digest: [u8; 32],
    pub evidence: OpenCodeEvidence,
}
impl OpenCodeRecordPosition {
    fn validate(&self, status: bool) -> CaptureResult<()> {
        if self.source.as_bytes() == &[0; 16]
            || self.generation == 0
            || self.session_witness == [0; 32]
            || self.prefix_digest == [0; 32]
            || self.sequence == 0
            || (self.logical_records == 0 && !status)
            || self.logical_records > 4096 * 32
            || self.logical_records > self.sequence
        {
            return Err("OpenCode source allocation unavailable".into());
        }
        Ok(())
    }
}
impl OpenCodeMessageSnapshot {
    /// The directory is the exact per-record context used by current admission.
    pub fn sanitize(
        &self,
        repository: RepositoryId,
        position: &OpenCodeRecordPosition,
        directory: &Path,
        sanitizer: &TranscriptSanitizer,
    ) -> CaptureResult<ReaderCapture> {
        if self.directory() != Some(directory) {
            return Err("OpenCode record directory context unavailable".into());
        }
        self.convert(repository, position, Some(sanitizer))
    }
    /// Safe omission still needs a separately authorized anchor; no body is inspected.
    pub fn sanitize_omitted(
        &self,
        repository: RepositoryId,
        position: &OpenCodeRecordPosition,
    ) -> CaptureResult<ReaderCapture> {
        self.convert(repository, position, None)
    }
    fn convert(
        &self,
        repository: RepositoryId,
        position: &OpenCodeRecordPosition,
        sanitizer: Option<&TranscriptSanitizer>,
    ) -> CaptureResult<ReaderCapture> {
        let status = self.envelope()["record"]["type"].as_str() == Some("session_status");
        if status && !matches!(position.evidence, OpenCodeEvidence::Api) {
            return Err("OpenCode export status is not lifecycle evidence".into());
        }
        position.validate(status)?;
        let native_session = self.envelope()["session_id"]
            .as_str()
            .filter(|id| native_id_valid(id, "ses"))
            .ok_or("OpenCode source session unavailable")?;
        let payload = sanitizer
            .and_then(|s| s.sanitize(self.envelope().clone()))
            .unwrap_or_else(|| {
                SanitizedPayloadV1::new(
                    SanitizedMediaType::Json,
                    if sanitizer.is_some() {
                        br#"{"capture_omitted":true,"reason":"unavailable_native_record"}"#
                    } else {
                        br#"{"capture_omitted":true,"reason":"unverified_directory_context"}"#
                    },
                )
                .expect("constant omission")
            });
        let (source_kind, source_trust) = match position.evidence {
            OpenCodeEvidence::Api => (SourceKind::NativeApi, SourceTrust::ApprovedHarnessApi),
            OpenCodeEvidence::Export => (SourceKind::SessionExport, SourceTrust::LocalTranscript),
        };
        let mut hash = Sha256::new_with_prefix(b"jj-store/opencode-record-intent/v1\0");
        hash.update(position.source.as_bytes());
        hash.update(repository.as_bytes());
        hash.update(position.generation.to_be_bytes());
        hash.update(position.session_witness);
        hash.update(position.sequence.to_be_bytes());
        hash.update(position.logical_records.to_be_bytes());
        hash.update(position.prefix_digest);
        hash.update([source_kind as u8, source_trust as u8]);
        let digest = hash.finalize();
        let batch_id = CaptureBatchId::new(digest[..16].try_into().expect("digest length"));
        let observation_id = ObservationId::new(digest[16..].try_into().expect("digest length"));
        let cursor = CaptureCursorV1 {
            schema_version: 1,
            integration: IntegrationId::OpenCode,
            source_instance_id: position.source,
            generation: position.generation,
            parser_version: "opencode-snapshot@1.18.30/v1".into(),
            native_session_key: Some(native_session.into()),
            position: SourcePositionV1::Snapshot {
                generation_digest: position.prefix_digest,
                logical_records: position.logical_records,
            },
        };
        cursor.validate()?;
        let batch = SanitizedObservationBatchV1 {
            schema_version: 1,
            batch_id,
            integration: IntegrationId::OpenCode,
            source_instance_id: position.source,
            sanitizer_version: TRANSCRIPT_POLICY.into(),
            representation: SanitizedRepresentation::SanitizedNative,
            observations: vec![SanitizedObservationV1 {
                observation_id,
                source_sequence: position.sequence,
                occurred_at_millis: None,
                source_kind,
                source_trust,
                payload,
            }],
        };
        batch.validate()?;
        Ok(ReaderCapture {
            repository,
            profile: NormalizerProfileV1::opencode_1_18_30(),
            cursor,
            batch,
            metadata: ReaderCaptureMetadataV1 {
                schema_version: 1,
                generation_digest: position.session_witness,
                opencode_snapshot: None,
            },
            thread_binding: None,
        })
    }
}
