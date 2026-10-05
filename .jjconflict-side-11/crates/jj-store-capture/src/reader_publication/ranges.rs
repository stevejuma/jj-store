use super::*;
use crate::reader_progress::ReaderProgressV1;
use jj_agent::{CaptureBatchId, SourcePositionV1};
use jj_store_model::{PrincipalId, PublicationId, RepositoryId, RequestId};
use jj_store_wal::publication::Receipt;
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Key {
    pub integration: IntegrationId,
    pub source: SourceInstanceId,
    pub generation: u64,
    pub digest: [u8; 32],
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Range {
    pub repository: RepositoryId,
    pub batch: CaptureBatchId,
    pub first: u64,
    pub last: u64,
    pub offset: u64,
    pub prefix: [u8; 32],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<SnapshotPosition>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SnapshotPosition {
    pub digest: [u8; 32],
    pub logical_records: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Evidence {
    pub principal: PrincipalId,
    pub request: RequestId,
    pub publication: PublicationId,
    pub receipt_digest: [u8; 32],
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Required {
    pub range: Range,
    pub evidence: Option<Evidence>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Channel {
    pub key: Key,
    pub history_available: bool,
    pub latest: Range,
    pub frontier: Option<Required>,
    pub receipts: u64,
    pub chain: [u8; 32],
    pub pending: Vec<Required>,
}
impl Channel {
    pub fn new(key: Key, latest: Range, history_available: bool) -> Self {
        Self {
            key,
            history_available,
            latest,
            frontier: None,
            receipts: 0,
            chain: [0; 32],
            pending: vec![],
        }
    }
}
impl Range {
    pub fn matches_progress(&self, p: &ReaderProgressV1) -> bool {
        p.repository == self.repository
            && p.last_batch == self.batch
            && p.last_sequence == self.last
            && p.cursor.position == self.position()
    }
    pub fn validate(&self) -> CaptureResult<()> {
        let valid_position = match self.snapshot {
            Some(snapshot) => {
                self.offset == 0 && self.prefix == [0; 32] && snapshot.digest != [0; 32]
            }
            None => self.offset > 0 && self.prefix != [0; 32],
        };
        if self.first == 0 || self.last < self.first || !valid_position {
            return Err("invalid publication range".into());
        }
        Ok(())
    }
    pub fn follows(&self, previous: &Self) -> bool {
        self.first > previous.last
            && match (self.snapshot, previous.snapshot) {
                (None, None) => self.offset > previous.offset,
                (Some(_), Some(_)) => true,
                _ => false,
            }
    }
    pub fn covered_by(&self, frontier: &Self) -> bool {
        self.last <= frontier.last
            && match (self.snapshot, frontier.snapshot) {
                (None, None) => self.offset <= frontier.offset,
                (Some(_), Some(_)) => true,
                _ => false,
            }
    }
    fn position(&self) -> SourcePositionV1 {
        match self.snapshot {
            Some(snapshot) => SourcePositionV1::Snapshot {
                generation_digest: snapshot.digest,
                logical_records: snapshot.logical_records,
            },
            None => SourcePositionV1::JsonLines {
                complete_byte_offset: self.offset,
                prefix_sha256: self.prefix,
            },
        }
    }
}
pub(super) fn of(entry: &PendingCaptureV1) -> CaptureResult<Option<(Key, Range)>> {
    entry.validate()?;
    let Some(metadata) = &entry.reader_metadata else {
        return Ok(None);
    };
    let (offset, prefix, snapshot) = match entry.cursor.position {
        SourcePositionV1::JsonLines {
            complete_byte_offset,
            prefix_sha256,
        } => (complete_byte_offset, prefix_sha256, None),
        SourcePositionV1::Snapshot {
            generation_digest,
            logical_records,
        } => (
            0,
            [0; 32],
            Some(SnapshotPosition {
                digest: generation_digest,
                logical_records,
            }),
        ),
        _ => return Ok(None),
    };
    Ok(Some((
        Key {
            integration: entry.cursor.integration,
            source: entry.cursor.source_instance_id,
            generation: entry.cursor.generation,
            digest: metadata.generation_digest,
        },
        Range {
            repository: entry.repository,
            batch: entry.batch.batch_id,
            first: entry
                .batch
                .observations
                .first()
                .expect("validated batch")
                .source_sequence,
            last: entry
                .batch
                .observations
                .last()
                .expect("validated batch")
                .source_sequence,
            offset,
            prefix,
            snapshot,
        },
    )))
}
impl Evidence {
    pub fn of(receipt: &Receipt) -> CaptureResult<Self> {
        let mut hash = Sha256::new_with_prefix(b"jj-store/reader-publication-receipt/v1\0");
        hash.update(serde_json::to_vec(receipt)?);
        Ok(Self {
            principal: receipt.principal,
            request: receipt.request_id,
            publication: receipt.publication_id,
            receipt_digest: hash.finalize().into(),
        })
    }
}
