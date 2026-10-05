//! Typed body-free API state; authority and native reconstruction remain with the reader.
use crate::{
    CaptureResult,
    reader_progress::{ReaderCapture, ReaderProgressV1},
};
use jj_agent::{IntegrationId, SourceKind, SourcePositionV1, SourceTrust};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpenCodeLifecycleV1 {
    Busy,
    Retry,
    Idle,
}
impl OpenCodeLifecycleV1 {
    pub fn native_status(self) -> &'static str {
        match self {
            Self::Busy => "busy",
            Self::Retry => "retry",
            Self::Idle => "idle",
        }
    }
    pub fn activity(self) -> crate::opencode_snapshot::OpenCodeActivity {
        match self {
            Self::Busy => crate::opencode_snapshot::OpenCodeActivity::Busy,
            Self::Retry => crate::opencode_snapshot::OpenCodeActivity::Retry,
            Self::Idle => crate::opencode_snapshot::OpenCodeActivity::Idle,
        }
    }
    pub fn from_activity(activity: crate::opencode_snapshot::OpenCodeActivity) -> Option<Self> {
        match activity {
            crate::opencode_snapshot::OpenCodeActivity::Busy => Some(Self::Busy),
            crate::opencode_snapshot::OpenCodeActivity::Retry => Some(Self::Retry),
            crate::opencode_snapshot::OpenCodeActivity::Idle => Some(Self::Idle),
            _ => None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpenCodeSnapshotStateV1 {
    pub message_records: u64,
    pub message_prefix: [u8; 32],
    pub lifecycle: OpenCodeLifecycleV1,
}
pub fn initial_message_prefix(witness: [u8; 32]) -> [u8; 32] {
    let mut hash = Sha256::new_with_prefix(b"jj-store/opencode-snapshot-prefix/v1\0");
    hash.update(witness);
    hash.finalize().into()
}
/// Exact native snapshot chain shared by acquisition, import and reader recovery.
pub fn next_message_prefix(
    previous: [u8; 32],
    record: &crate::opencode_snapshot::OpenCodeMessageSnapshot,
) -> CaptureResult<[u8; 32]> {
    let mut hash = Sha256::new_with_prefix(b"jj-store/opencode-snapshot-record/v1\0");
    hash.update(previous);
    let bytes = serde_json::to_vec(record.envelope())?;
    hash.update((bytes.len() as u64).to_be_bytes());
    hash.update(bytes);
    Ok(hash.finalize().into())
}
impl OpenCodeSnapshotStateV1 {
    pub fn initial(witness: [u8; 32], lifecycle: OpenCodeLifecycleV1) -> Self {
        Self {
            message_records: 0,
            message_prefix: initial_message_prefix(witness),
            lifecycle,
        }
    }
    pub fn digest(&self, witness: [u8; 32]) -> [u8; 32] {
        let mut hash = Sha256::new_with_prefix(b"jj-store/opencode-reader-state/v1\0");
        hash.update(witness);
        hash.update(self.message_records.to_be_bytes());
        hash.update(self.message_prefix);
        hash.update([match self.lifecycle {
            OpenCodeLifecycleV1::Busy => 0,
            OpenCodeLifecycleV1::Retry => 1,
            OpenCodeLifecycleV1::Idle => 2,
        }]);
        hash.finalize().into()
    }
    pub(crate) fn validate(&self, witness: [u8; 32]) -> CaptureResult<()> {
        if witness == [0; 32]
            || self.message_prefix == [0; 32]
            || self.message_records > 131072
            || (self.message_records == 0 && self.message_prefix != initial_message_prefix(witness))
        {
            return Err("invalid OpenCode snapshot checkpoint".into());
        }
        Ok(())
    }
    pub(crate) fn validate_progress(&self, progress: &ReaderProgressV1) -> CaptureResult<()> {
        self.validate(progress.metadata.generation_digest)?;
        if progress.cursor.integration != IntegrationId::OpenCode
            || progress.cursor.parser_version != "opencode-snapshot@1.18.30/v1"
            || !progress
                .cursor
                .native_session_key
                .as_deref()
                .is_some_and(|id| crate::opencode_snapshot::native_id_valid(id, "ses"))
            || progress.cursor.position
                != (SourcePositionV1::Snapshot {
                    generation_digest: self.digest(progress.metadata.generation_digest),
                    logical_records: self.message_records,
                })
        {
            return Err("OpenCode checkpoint cursor differs".into());
        }
        Ok(())
    }
    pub(crate) fn validate_forward(&self, previous: &ReaderProgressV1) -> CaptureResult<()> {
        let (count, prefix) = if let Some(old) = &previous.metadata.opencode_snapshot {
            (old.message_records, old.message_prefix)
        } else {
            match previous.cursor.position {
                SourcePositionV1::Snapshot {
                    generation_digest,
                    logical_records,
                } => (logical_records, generation_digest),
                _ => return Err("OpenCode checkpoint position changed".into()),
            }
        };
        if self.message_records < count
            || (self.message_records == count && self.message_prefix != prefix)
            || (self.message_records > count && self.message_prefix == prefix)
        {
            return Err("OpenCode message checkpoint regressed".into());
        }
        Ok(())
    }
}
impl ReaderCapture {
    pub fn with_opencode_state(mut self, state: OpenCodeSnapshotStateV1) -> CaptureResult<Self> {
        state.validate(self.metadata.generation_digest)?;
        let SourcePositionV1::Snapshot {
            generation_digest,
            logical_records,
        } = self.cursor.position
        else {
            return Err("OpenCode checkpoint position unavailable".into());
        };
        if self.cursor.integration != IntegrationId::OpenCode
            || self.cursor.parser_version != "opencode-snapshot@1.18.30/v1"
            || logical_records != state.message_records
            || (generation_digest != state.message_prefix
                && generation_digest != state.digest(self.metadata.generation_digest))
            || self.batch.observations.len() != 1
            || self.batch.observations[0].source_kind != SourceKind::NativeApi
            || self.batch.observations[0].source_trust != SourceTrust::ApprovedHarnessApi
        {
            return Err("OpenCode checkpoint evidence differs".into());
        }
        self.cursor.position = SourcePositionV1::Snapshot {
            generation_digest: state.digest(self.metadata.generation_digest),
            logical_records: state.message_records,
        };
        self.metadata.opencode_snapshot = Some(Box::new(state));
        Ok(self)
    }
}
