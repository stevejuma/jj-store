//! Native readers restricted to a verified adapter snapshot's inventory.
use crate::reads::query::{
    self, ReadAccess,
    content::{ContentReader, Object, TreePage, TreeRequest, VerifiedFile},
};
use jj_store_adapters as a;
use jj_store_model as m;
use jj_store_object::{
    self as storage, ByteRange, Condition, Download, Key, Metadata, ObjectStore, StagedUpload,
    VersionToken, WriteOutcome,
};
use jj_store_wal::inventory::Representation;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

struct Access {
    objects: a::Objects,
    operation: m::OperationId,
    inventory: BTreeSet<m::ObjectKey>,
}
#[async_trait::async_trait]
impl ReadAccess for Access {
    fn repository(&self) -> m::RepositoryId {
        self.objects.repository()
    }
    fn epoch(&self) -> m::EpochId {
        self.objects.epoch()
    }
    fn operation(&self) -> m::OperationId {
        self.operation
    }
    fn inventory(&self) -> &BTreeSet<m::ObjectKey> {
        &self.inventory
    }
    async fn authorize(&self) -> query::Result<()> {
        self.objects.authorize().await.map_err(query_error)
    }
}
fn query_error(error: a::Error) -> query::Error {
    match error {
        a::Error::Denied => query::Error::Denied,
        a::Error::Corrupt => query::Error::Corrupt,
        a::Error::Canceled => query::Error::Canceled,
        a::Error::Limit => query::Error::Limit,
        _ => query::Error::Unavailable,
    }
}
fn storage_error(error: a::Error) -> storage::Error {
    match error {
        a::Error::Limit => storage::Error::Limit,
        _ => storage::Error::Invalid("adapter source read failed"),
    }
}
struct Store {
    objects: a::Objects,
    paths: BTreeMap<Key, m::ObjectKey>,
}
impl Store {
    fn key(&self, key: &Key) -> storage::Result<m::ObjectKey> {
        self.paths
            .get(key)
            .copied()
            .ok_or(storage::Error::Invalid("outside adapter snapshot"))
    }
    fn metadata(&self, key: m::ObjectKey) -> storage::Result<Metadata> {
        let node = self
            .objects
            .entries()
            .get(&key)
            .ok_or(storage::Error::NotFound)?;
        match node.representation {
            Representation::Virtual => Err(storage::Error::NotFound),
            Representation::Stored {
                bytes,
                envelope_digest,
            } => Ok(Metadata {
                len: bytes,
                version: VersionToken::new(envelope_digest.to_string())?,
            }),
        }
    }
}
#[async_trait::async_trait]
impl ObjectStore for Store {
    async fn head(&self, key: &Key) -> storage::Result<Metadata> {
        self.objects.authorize().await.map_err(storage_error)?;
        let metadata = self.metadata(self.key(key)?)?;
        self.objects.authorize().await.map_err(storage_error)?;
        Ok(metadata)
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> storage::Result<Download> {
        self.objects.authorize().await.map_err(storage_error)?;
        let key = self.key(key)?;
        let mut metadata = self.metadata(key)?;
        let bytes = self
            .objects
            .read(key)
            .await
            .map_err(storage_error)?
            .ok_or(storage::Error::NotFound)?;
        let bytes = if let Some(range) = range {
            let start = usize::try_from(range.start()).map_err(|_| storage::Error::Range)?;
            let end = usize::try_from(range.end()).map_err(|_| storage::Error::Range)?;
            bytes.get(start..end).ok_or(storage::Error::Range)?.to_vec()
        } else {
            bytes
        };
        metadata.len = bytes.len() as u64;
        self.objects.authorize().await.map_err(storage_error)?;
        Ok(Download {
            metadata,
            reader: Box::pin(std::io::Cursor::new(bytes)),
        })
    }
    async fn put(&self, _: &Key, _: Condition, _: &StagedUpload) -> storage::Result<WriteOutcome> {
        Err(storage::Error::Invalid("read-only adapter inventory"))
    }
}
/// No session, raw storage credential or publication API escapes this boundary.
/// Clones retain the adapter run context through any native blocking workers.
#[derive(Clone)]
pub struct ProjectionRead {
    access: Arc<Access>,
    store: Arc<Store>,
    reader: ContentReader,
}
impl ProjectionRead {
    pub async fn file_bytes(&self, id: m::FileId) -> query::Result<u64> {
        self.access.authorize().await?;
        let node = self
            .access
            .objects
            .entries()
            .get(&m::ObjectKey::File(id))
            .ok_or(query::Error::NotCommitted)?;
        match node.representation {
            Representation::Stored { bytes, .. } => bytes
                .checked_sub(m::envelope::HEADER_LEN as u64)
                .ok_or(query::Error::Corrupt),
            Representation::Virtual => Err(query::Error::Corrupt),
        }
    }
    pub async fn new(
        snapshot: &a::Snapshot,
        operation: m::OperationId,
        reader: ContentReader,
    ) -> a::Result<Self> {
        snapshot.objects.authorize().await?;
        if snapshot.selected_position().repository != snapshot.objects.repository()
            || snapshot.selected_position().epoch != snapshot.objects.epoch()
            || !snapshot
                .objects
                .entries()
                .contains_key(&m::ObjectKey::Operation(operation))
        {
            return Err(a::Error::Denied);
        }
        let inventory = snapshot.objects.entries().keys().copied().collect();
        let paths = snapshot
            .objects
            .entries()
            .keys()
            .map(|key| {
                Ok((
                    Key::new(key.storage_path(snapshot.objects.repository()))
                        .map_err(|_| a::Error::Corrupt)?,
                    *key,
                ))
            })
            .collect::<a::Result<_>>()?;
        Ok(Self {
            access: Arc::new(Access {
                objects: snapshot.objects.clone(),
                operation,
                inventory,
            }),
            store: Arc::new(Store {
                objects: snapshot.objects.clone(),
                paths,
            }),
            reader,
        })
    }
    pub async fn object(&self, key: m::ObjectKey) -> query::Result<Object> {
        self.reader
            .object(self.store.clone(), self.access.clone(), key)
            .await
    }
    pub async fn tree(&self, request: TreeRequest) -> query::Result<TreePage> {
        self.reader
            .tree(self.store.clone(), self.access.clone(), request)
            .await
    }
    pub async fn file(&self, id: m::FileId) -> query::Result<VerifiedFile> {
        self.reader
            .file(self.store.clone(), self.access.clone(), id, None)
            .await
    }
    pub async fn diff(
        &self,
        request: query::content::diff::Request,
    ) -> query::Result<query::content::diff::Response> {
        self.reader
            .diff(self.store.clone(), self.access.clone(), request)
            .await
    }
}
