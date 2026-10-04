use async_trait::async_trait;
use jj_store_model::{EpochId, ReplicaId, RepositoryId, WalPosition};
use jj_store_object::{
    ByteRange, Condition, Download, Key, Metadata, ObjectStore, StagedUpload, WriteOutcome,
    memory::MemoryStore,
};
use jj_store_replication::{FeedCursorV1, PeerCheckpointStore};
use std::sync::{Arc, Mutex};

struct LostAckStore {
    inner: MemoryStore,
    lose_next: Mutex<bool>,
}

impl LostAckStore {
    fn new() -> Self {
        Self {
            inner: MemoryStore::default(),
            lose_next: Mutex::new(false),
        }
    }

    fn lose_next_ack(&self) {
        *self.lose_next.lock().unwrap() = true;
    }
}

#[async_trait]
impl ObjectStore for LostAckStore {
    async fn head(&self, key: &Key) -> jj_store_object::Result<Metadata> {
        self.inner.head(key).await
    }

    async fn get(&self, key: &Key, range: Option<ByteRange>) -> jj_store_object::Result<Download> {
        self.inner.get(key, range).await
    }

    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> jj_store_object::Result<WriteOutcome> {
        let result = self.inner.put(key, condition, upload).await?;
        if matches!(result, WriteOutcome::Applied(_))
            && std::mem::take(&mut *self.lose_next.lock().unwrap())
        {
            Ok(WriteOutcome::Unknown)
        } else {
            Ok(result)
        }
    }
}

#[tokio::test]
async fn checkpoint_is_issuer_bound_monotonic_and_recovers_a_lost_ack() {
    let repository = RepositoryId::new([1; 16]);
    let local = ReplicaId::new([2; 16]);
    let peer = ReplicaId::new([3; 16]);
    let store = Arc::new(LostAckStore::new());
    let checkpoints = PeerCheckpointStore::new(store.clone(), repository, local, peer).unwrap();
    assert!(checkpoints.load().await.unwrap().is_none());

    let mut first = checkpoints.empty();
    first.revision = 1;
    first.pulled = Some(FeedCursorV1 {
        issuer: peer,
        log_epoch: EpochId::new([4; 16]),
        through: WalPosition(7),
        observed_upper_bound: WalPosition(9),
    });
    store.lose_next_ack();
    let saved = checkpoints.save(None, first.clone()).await.unwrap();
    assert_eq!(saved.checkpoint, first);
    assert_eq!(checkpoints.load().await.unwrap().unwrap().checkpoint, first);

    let mut backwards = first.clone();
    backwards.revision = 2;
    backwards.pulled.as_mut().unwrap().through = WalPosition(6);
    assert!(checkpoints.save(Some(&saved), backwards).await.is_err());

    let mut advanced = first;
    advanced.revision = 2;
    advanced.pulled.as_mut().unwrap().through = WalPosition(9);
    let advanced = checkpoints.save(Some(&saved), advanced).await.unwrap();
    assert_eq!(advanced.checkpoint.pulled.unwrap().through, WalPosition(9));
}
