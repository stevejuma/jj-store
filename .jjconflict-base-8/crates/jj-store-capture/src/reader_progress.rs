//! Device-only acknowledged reader progress; sanitized pending evidence comes first.
use crate::CaptureResult;
mod history;
mod source;
mod spool;
use jj_agent::{CaptureBatchId, CaptureCursorV1, NormalizerProfileV1, SanitizedObservationBatchV1};
use jj_store_model::RepositoryId;
use serde::{Deserialize, Serialize};
pub use source::ReaderSourceProgress;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReaderCaptureMetadataV1 {
    pub schema_version: u32,
    pub generation_digest: [u8; 32],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opencode_snapshot: Option<Box<crate::opencode_checkpoint::OpenCodeSnapshotStateV1>>,
}
pub struct ReaderCapture {
    pub repository: RepositoryId,
    pub profile: NormalizerProfileV1,
    pub cursor: CaptureCursorV1,
    pub batch: SanitizedObservationBatchV1,
    pub metadata: ReaderCaptureMetadataV1,
    pub thread_binding: Option<jj_collab::ThreadBindingV1>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReaderProgressV1 {
    pub schema_version: u32,
    pub repository: RepositoryId,
    pub cursor: CaptureCursorV1,
    pub metadata: ReaderCaptureMetadataV1,
    pub last_sequence: u64,
    pub last_batch: CaptureBatchId,
}
impl ReaderCaptureMetadataV1 {
    pub(crate) fn validate(&self) -> CaptureResult<()> {
        if self.schema_version != 1 || self.generation_digest == [0; 32] {
            return Err("invalid reader generation metadata".into());
        }
        if let Some(state) = &self.opencode_snapshot {
            state.validate(self.generation_digest)?;
        }
        Ok(())
    }
}
impl ReaderProgressV1 {
    pub(crate) fn validate(&self) -> CaptureResult<()> {
        self.cursor.validate()?;
        self.metadata.validate()?;
        if self.schema_version != 1 || self.last_sequence == 0 {
            return Err("invalid reader progress".into());
        }
        match self.cursor.position {
            jj_agent::SourcePositionV1::JsonLines {
                complete_byte_offset,
                prefix_sha256,
            } if complete_byte_offset == 0 || prefix_sha256 == [0; 32] => {
                return Err("invalid reader byte position".into());
            }
            jj_agent::SourcePositionV1::NativeSequence { sequence }
                if sequence != self.last_sequence =>
            {
                return Err("invalid reader native position".into());
            }
            jj_agent::SourcePositionV1::Snapshot {
                generation_digest,
                logical_records,
            } if generation_digest == [0; 32]
                || (logical_records == 0 && self.metadata.opencode_snapshot.is_none()) =>
            {
                return Err("invalid reader snapshot position".into());
            }
            _ => {}
        }
        if let Some(state) = &self.metadata.opencode_snapshot {
            state.validate_progress(self)?;
        }
        Ok(())
    }
    pub(super) fn from_pending(
        entry: &crate::agent_capture::PendingCaptureV1,
    ) -> CaptureResult<Option<Self>> {
        let Some(metadata) = &entry.reader_metadata else {
            return Ok(None);
        };
        if metadata.opencode_snapshot.is_some()
            && entry.batch.observations.iter().any(|o| {
                o.source_kind != jj_agent::SourceKind::NativeApi
                    || o.source_trust != jj_agent::SourceTrust::ApprovedHarnessApi
            })
        {
            return Err("OpenCode lifecycle checkpoint requires approved API evidence".into());
        }
        let progress = Self {
            schema_version: 1,
            repository: entry.repository,
            cursor: entry.cursor.clone(),
            metadata: metadata.clone(),
            last_sequence: entry
                .batch
                .observations
                .last()
                .ok_or("reader capture observations missing")?
                .source_sequence,
            last_batch: entry.batch.batch_id,
        };
        progress.validate()?;
        Ok(Some(progress))
    }
    pub(super) fn validate_binding(&self, previous: &Self) -> CaptureResult<()> {
        if self.metadata.schema_version != previous.metadata.schema_version
            || self.metadata.generation_digest != previous.metadata.generation_digest
            || self.cursor.integration != previous.cursor.integration
            || self.cursor.source_instance_id != previous.cursor.source_instance_id
            || self.cursor.generation != previous.cursor.generation
            || self.cursor.parser_version != previous.cursor.parser_version
            || self.cursor.native_session_key != previous.cursor.native_session_key
        {
            return Err("reader generation or parser changed".into());
        }
        Ok(())
    }
    pub(super) fn validate_forward(&self, previous: &Self) -> CaptureResult<()> {
        self.validate_binding(previous)?;
        if self.last_sequence <= previous.last_sequence {
            if self == previous {
                return Ok(());
            }
            return Err("reader progress must advance monotonically".into());
        }
        use jj_agent::SourcePositionV1;
        let advances = match (&previous.cursor.position, &self.cursor.position) {
            (
                SourcePositionV1::JsonLines {
                    complete_byte_offset: old,
                    ..
                },
                SourcePositionV1::JsonLines {
                    complete_byte_offset: new,
                    ..
                },
            ) => new > old,
            (
                SourcePositionV1::NativeSequence { sequence: old },
                SourcePositionV1::NativeSequence { sequence: new },
            ) => new > old,
            (
                SourcePositionV1::Snapshot {
                    generation_digest: old,
                    ..
                },
                SourcePositionV1::Snapshot {
                    generation_digest: new,
                    ..
                },
            ) => new != old,
            _ => false,
        };
        if let Some(state) = &self.metadata.opencode_snapshot {
            state.validate_forward(previous)?;
        } else if previous.metadata.opencode_snapshot.is_some() {
            return Err("OpenCode lifecycle checkpoint dropped".into());
        }
        if !advances {
            return Err("reader source position did not advance".into());
        }
        Ok(())
    }
}
