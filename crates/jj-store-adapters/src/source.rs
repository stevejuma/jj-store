use crate::*;
use async_trait::async_trait;
use jj_store_object::{
    self as storage, ByteRange, Condition, Download, Key, Metadata, ObjectStore, StagedUpload,
    WriteOutcome,
};
use jj_store_wal::{
    inventory::{Representation, VerifiedNode},
    publication::SelectedRecord,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, Mutex},
};

/// Current access comes from the serving service, never from a saved cursor.
#[async_trait]
pub trait Authorizer: Send + Sync {
    fn repository(&self) -> m::RepositoryId;
    async fn authorize(&self) -> Result<m::EpochId>;
}
#[derive(Clone)]
pub struct Source {
    pub(crate) store: Arc<dyn ObjectStore>,
    pub(crate) access: Arc<dyn Authorizer>,
}
impl Source {
    pub fn new(store: Arc<dyn ObjectStore>, access: Arc<dyn Authorizer>) -> Self {
        Self { store, access }
    }
    pub fn repository(&self) -> m::RepositoryId {
        self.access.repository()
    }
    pub(crate) async fn capture(&self, context: &Context) -> Result<Captured> {
        let epoch = context.until(self.access.authorize()).await?;
        let store = Arc::new(CheckedStore {
            inner: self.store.clone(),
            access: self.access.clone(),
            epoch,
            context: context.clone(),
            failure: Mutex::new(None),
        });
        let manifest = ManifestSnapshot::load(store.clone(), self.repository())
            .await
            .map_err(|e| store.failure(e.into()))?;
        store.authorize().await?;
        if manifest.manifest().epoch != epoch {
            return Err(Error::Denied);
        }
        let captured = Captured { manifest, store };
        let position = captured.position();
        if position.position.0 == 0 {
            captured.records(m::WalPosition(0), 0).await?;
        } else {
            // Bootstrap must validate the selected tail/receipt too, even when
            // there is no prior adapter cursor and no tail batch to deliver.
            captured.validate_position(&position).await?;
        }
        Ok(captured)
    }
}
pub(crate) struct Captured {
    pub(crate) manifest: ManifestSnapshot,
    pub(crate) store: Arc<CheckedStore>,
}
impl Captured {
    pub(crate) fn position(&self) -> Position {
        Position::current(&self.manifest)
    }
    pub(crate) async fn validate_position(&self, position: &Position) -> Result<()> {
        position.validate()?;
        let current = self.position();
        if position.repository != current.repository
            || position.epoch != current.epoch
            || position.position.0 > current.position.0
        {
            return Err(Error::Corrupt);
        }
        if position.position.0 > 0 {
            let records = self
                .records(m::WalPosition(position.position.0 - 1), 1)
                .await?;
            if records.first().map(SelectedRecord::reference) != position.segment {
                return Err(Error::Corrupt);
            }
        }
        Ok(())
    }
    async fn records(&self, after: m::WalPosition, limit: usize) -> Result<Vec<SelectedRecord>> {
        let records = self
            .manifest
            .selected_records(self.store.clone(), after, limit)
            .await
            .map_err(|e| self.store.failure(e.into()))?;
        self.store.authorize().await?;
        Ok(records)
    }
    pub(crate) async fn bootstrap(&self) -> Result<Snapshot> {
        let nodes = self
            .manifest
            .inventory(
                self.store.clone(),
                self.store.context.limits().inventory_nodes,
            )
            .await
            .map_err(|e| self.store.failure(e.into()))?;
        self.store.authorize().await?;
        Ok(self.snapshot(
            self.position(),
            self.manifest.manifest().heads.clone(),
            nodes,
        ))
    }
    pub(crate) async fn batch(&self, before: &Position) -> Result<Option<Batch>> {
        self.validate_position(before).await?;
        let records = self
            .records(before.position, self.store.context.limits().page)
            .await?;
        let Some(last) = records.last() else {
            return Ok(None);
        };
        if records[0].transaction().previous != before.segment {
            return Err(Error::Corrupt);
        }
        let nodes = last
            .inventory(
                self.store.clone(),
                self.store.context.limits().inventory_nodes,
            )
            .await
            .map_err(|e| self.store.failure(e.into()))?;
        self.store.authorize().await?;
        let tx = last.transaction();
        let after = Position {
            repository: tx.repository,
            epoch: tx.after.epoch,
            position: tx.after.position,
            segment: Some(last.reference()),
        };
        let snapshot = self.snapshot(after, tx.after.heads.clone(), nodes);
        Ok(Some(Batch {
            before: before.clone(),
            records,
            snapshot,
        }))
    }
    fn snapshot(
        &self,
        position: Position,
        heads: BTreeSet<m::OperationId>,
        nodes: Vec<VerifiedNode>,
    ) -> Snapshot {
        Snapshot {
            position,
            heads,
            objects: Objects {
                store: self.store.clone(),
                nodes: Arc::new(nodes.into_iter().map(|node| (node.key, node)).collect()),
            },
        }
    }
}
pub struct Snapshot {
    pub position: Position,
    pub heads: BTreeSet<m::OperationId>,
    pub objects: Objects,
}
pub struct Batch {
    pub before: Position,
    pub records: Vec<SelectedRecord>,
    pub snapshot: Snapshot,
}
#[derive(Clone)]
pub struct Objects {
    store: Arc<CheckedStore>,
    nodes: Arc<BTreeMap<m::ObjectKey, VerifiedNode>>,
}
impl Objects {
    pub fn entries(&self) -> &BTreeMap<m::ObjectKey, VerifiedNode> {
        &self.nodes
    }
    /// Verified framing from this snapshot's complete native inventory. None
    /// denotes an explicit virtual native root; unknown keys are denied.
    pub async fn read(&self, key: m::ObjectKey) -> Result<Option<Vec<u8>>> {
        self.store.authorize().await?;
        let node = self.nodes.get(&key).ok_or(Error::Denied)?;
        let Representation::Stored {
            envelope_digest,
            bytes,
        } = node.representation
        else {
            return Ok(None);
        };
        let limit = self.store.context.limits().object_bytes;
        if bytes > limit {
            return Err(Error::Limit);
        }
        let download = self
            .store
            .get(
                &Key::new(key.storage_path(self.store.access.repository()))?,
                None,
            )
            .await
            .map_err(|e| self.store.failure(e.into()))?;
        if download.metadata.len != bytes {
            return Err(Error::Corrupt);
        }
        let data = self
            .store
            .context
            .until(async { Ok(download.read_bounded(limit).await?) })
            .await?;
        let header = data.get(..m::envelope::HEADER_LEN).ok_or(Error::Corrupt)?;
        if m::envelope::Header::decode(header, key, limit)
            .map_err(|_| Error::Corrupt)?
            .digest
            != envelope_digest
        {
            return Err(Error::Corrupt);
        }
        m::envelope::decode(&data, key, limit).map_err(|_| Error::Corrupt)?;
        self.store.authorize().await?;
        Ok(Some(data))
    }
}
/// All source/control reads consume one run's budget. No caller receives this
/// raw store; even internal misuse cannot write through it.
pub(crate) struct CheckedStore {
    inner: Arc<dyn ObjectStore>,
    access: Arc<dyn Authorizer>,
    epoch: m::EpochId,
    context: Context,
    failure: Mutex<Option<Error>>,
}
impl CheckedStore {
    pub(crate) async fn authorize(&self) -> Result<()> {
        let epoch = self.context.until(self.access.authorize()).await?;
        if epoch != self.epoch {
            return Err(Error::Denied);
        }
        Ok(())
    }
    pub(crate) fn failure(&self, fallback: Error) -> Error {
        self.failure.lock().unwrap().clone().unwrap_or(fallback)
    }
    fn record(&self, e: Error) -> storage::Error {
        self.failure.lock().unwrap().get_or_insert(e);
        storage::Error::Invalid("adapter read failed")
    }
}
#[async_trait]
impl ObjectStore for CheckedStore {
    async fn head(&self, key: &Key) -> storage::Result<Metadata> {
        self.authorize().await.map_err(|e| self.record(e))?;
        self.context.request().map_err(|e| self.record(e))?;
        let result = self
            .context
            .until(async { Ok(self.inner.head(key).await?) })
            .await
            .map_err(|e| self.record(e))?;
        self.authorize().await.map_err(|e| self.record(e))?;
        Ok(result)
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> storage::Result<Download> {
        if range.is_some() {
            return Err(storage::Error::Range);
        }
        self.authorize().await.map_err(|e| self.record(e))?;
        self.context.request().map_err(|e| self.record(e))?;
        let result = self
            .context
            .until(async { Ok(self.inner.get(key, None).await?) })
            .await
            .map_err(|e| self.record(e))?;
        self.context
            .bytes(result.metadata.len)
            .map_err(|e| self.record(e))?;
        self.authorize().await.map_err(|e| self.record(e))?;
        Ok(result)
    }
    async fn put(&self, _: &Key, _: Condition, _: &StagedUpload) -> storage::Result<WriteOutcome> {
        Err(self.record(Error::Denied))
    }
}
