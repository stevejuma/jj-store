//! Durable peer progress. These CAS pointers are transport state and never
//! participate in repository publication authority or an origin feed.
use crate::{Error, Result};
use jj_store_model::{EpochId, ReplicaId, RepositoryId, WalPosition};
use jj_store_object::{Condition, Key, ObjectStore, StagedUpload, VersionToken, WriteOutcome};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

const CHECKPOINT_BYTES: u64 = 64 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FeedCursorV1 {
    pub issuer: ReplicaId,
    pub log_epoch: EpochId,
    pub through: WalPosition,
    pub observed_upper_bound: WalPosition,
}

impl FeedCursorV1 {
    fn validate(self, issuer: ReplicaId) -> Result<()> {
        if self.issuer != issuer || self.through > self.observed_upper_bound {
            return Err(Error::Integrity("peer feed cursor"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PeerCheckpointV1 {
    pub schema_version: u32,
    pub repository: RepositoryId,
    pub local_replica: ReplicaId,
    pub peer_replica: ReplicaId,
    pub revision: u64,
    /// Remote feed entries durably applied or already known locally.
    pub pulled: Option<FeedCursorV1>,
    /// Local feed entries durably acknowledged or already known by the peer.
    pub pushed: Option<FeedCursorV1>,
}

impl PeerCheckpointV1 {
    pub fn empty(
        repository: RepositoryId,
        local_replica: ReplicaId,
        peer_replica: ReplicaId,
    ) -> Self {
        Self {
            schema_version: 1,
            repository,
            local_replica,
            peer_replica,
            revision: 0,
            pulled: None,
            pushed: None,
        }
    }

    fn validate(&self) -> Result<()> {
        if self.schema_version != 1 || self.local_replica == self.peer_replica {
            return Err(Error::Integrity("peer checkpoint identity"));
        }
        if let Some(cursor) = self.pulled {
            cursor.validate(self.peer_replica)?;
        }
        if let Some(cursor) = self.pushed {
            cursor.validate(self.local_replica)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VersionedCheckpoint {
    pub checkpoint: PeerCheckpointV1,
    pub version: VersionToken,
}

#[derive(Clone)]
pub struct PeerCheckpointStore {
    store: Arc<dyn ObjectStore>,
    repository: RepositoryId,
    local_replica: ReplicaId,
    peer_replica: ReplicaId,
    key: Key,
}

impl PeerCheckpointStore {
    pub fn new(
        store: Arc<dyn ObjectStore>,
        repository: RepositoryId,
        local_replica: ReplicaId,
        peer_replica: ReplicaId,
    ) -> Result<Self> {
        if local_replica == peer_replica {
            return Err(Error::Integrity("peer checkpoint identity"));
        }
        let key = Key::new(format!(
            "repos/{repository}/replication/peers/{local_replica}/{peer_replica}.json"
        ))?;
        Ok(Self {
            store,
            repository,
            local_replica,
            peer_replica,
            key,
        })
    }

    pub fn empty(&self) -> PeerCheckpointV1 {
        PeerCheckpointV1::empty(self.repository, self.local_replica, self.peer_replica)
    }

    fn validate_scope(&self, checkpoint: &PeerCheckpointV1) -> Result<()> {
        checkpoint.validate()?;
        if checkpoint.repository != self.repository
            || checkpoint.local_replica != self.local_replica
            || checkpoint.peer_replica != self.peer_replica
        {
            return Err(Error::Integrity("peer checkpoint scope"));
        }
        Ok(())
    }

    pub async fn load(&self) -> Result<Option<VersionedCheckpoint>> {
        let download = match self.store.get(&self.key, None).await {
            Ok(download) => download,
            Err(jj_store_object::Error::NotFound) => return Ok(None),
            Err(error) => return Err(error.into()),
        };
        let version = download.metadata.version.clone();
        let bytes = download.read_bounded(CHECKPOINT_BYTES).await?;
        let checkpoint: PeerCheckpointV1 =
            serde_json::from_slice(&bytes).map_err(|_| Error::Integrity("peer checkpoint JSON"))?;
        self.validate_scope(&checkpoint)?;
        Ok(Some(VersionedCheckpoint {
            checkpoint,
            version,
        }))
    }

    /// CAS a monotonic checkpoint. A returned value is proof that the pointer
    /// is durable, including when the object store lost the original response.
    pub async fn save(
        &self,
        previous: Option<&VersionedCheckpoint>,
        next: PeerCheckpointV1,
    ) -> Result<VersionedCheckpoint> {
        self.validate_scope(&next)?;
        match previous {
            None if next.revision != 1 => return Err(Error::CursorConflict),
            Some(previous) => {
                self.validate_scope(&previous.checkpoint)?;
                if next.revision != previous.checkpoint.revision.saturating_add(1)
                    || !cursor_advances(previous.checkpoint.pulled, next.pulled)
                    || !cursor_advances(previous.checkpoint.pushed, next.pushed)
                {
                    return Err(Error::CursorConflict);
                }
            }
            None => {}
        }
        let bytes =
            serde_json::to_vec(&next).map_err(|_| Error::Integrity("peer checkpoint JSON"))?;
        let upload = StagedUpload::from_bytes(&bytes, CHECKPOINT_BYTES).await?;
        let condition = previous.map_or(Condition::Absent, |value| {
            Condition::Matches(value.version.clone())
        });
        match self.store.put(&self.key, condition, &upload).await? {
            WriteOutcome::Applied(version) => Ok(VersionedCheckpoint {
                checkpoint: next,
                version,
            }),
            WriteOutcome::PreconditionFailed => Err(Error::CursorConflict),
            WriteOutcome::Unknown => match self.load().await? {
                Some(current) if current.checkpoint == next => Ok(current),
                Some(_) => Err(Error::CursorConflict),
                None => Err(Error::OutcomeUnknown),
            },
        }
    }
}

fn cursor_advances(before: Option<FeedCursorV1>, after: Option<FeedCursorV1>) -> bool {
    match (before, after) {
        (None, _) => true,
        (Some(_), None) => false,
        (Some(before), Some(after)) => {
            before.issuer == after.issuer
                && before.log_epoch == after.log_epoch
                && after.through >= before.through
                && after.observed_upper_bound >= before.observed_upper_bound
        }
    }
}
