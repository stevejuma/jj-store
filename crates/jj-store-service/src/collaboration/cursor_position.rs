use super::agent_cursors::AgentPin;
use jj_store_model as m;
use jj_store_wal::publication::{AgentCatalogKey, AgentObjectId, AgentSessionId, SourceInstanceId};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub(super) struct Entry {
    pub pin: AgentPin,
    pub page_size: usize,
    pub issued: u64,
    pub expires: u64,
    pub objects: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum Position {
    Sessions {
        after: AgentCatalogKey,
    },
    Transcript {
        session_id: AgentSessionId,
        transcript_id: AgentObjectId,
        source_instance_id: SourceInstanceId,
        source_generation: u64,
        max_bytes: usize,
        after_sequence: u64,
    },
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Claims {
    version: u32,
    pub pin: [u8; 16],
    pub repository: m::RepositoryId,
    epoch: m::EpochId,
    published_seq: m::WalPosition,
    revision: u64,
    agent_root: m::RecordDigest,
    agent_state_version: u64,
    page_size: usize,
    pub position: Position,
    issued: u64,
    expires: u64,
}

impl Claims {
    pub fn new(id: [u8; 16], entry: &Entry, position: Position) -> Self {
        let snapshot = &entry.pin.snapshot;
        Self {
            version: 1,
            pin: id,
            repository: snapshot.repo_id,
            epoch: snapshot.epoch,
            published_seq: snapshot.published_seq,
            revision: snapshot.manifest_revision,
            agent_root: snapshot.agent_root,
            agent_state_version: snapshot.agent_state_version,
            page_size: entry.page_size,
            position,
            issued: entry.issued,
            expires: entry.expires,
        }
    }

    pub fn matches(&self, entry: &Entry) -> bool {
        let expected = Self::new(self.pin, entry, self.position);
        self.version == expected.version
            && self.repository == expected.repository
            && self.epoch == expected.epoch
            && self.published_seq == expected.published_seq
            && self.revision == expected.revision
            && self.agent_root == expected.agent_root
            && self.agent_state_version == expected.agent_state_version
            && self.page_size == expected.page_size
            && self.issued == expected.issued
            && self.expires == expected.expires
    }

    pub fn valid_version(&self) -> bool {
        self.version == 1
    }

    pub fn issued(&self) -> u64 {
        self.issued
    }

    pub fn expires(&self) -> u64 {
        self.expires
    }

    pub fn page_size(&self) -> usize {
        self.page_size
    }
}
