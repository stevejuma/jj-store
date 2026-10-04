//! Durable staging namespace, invisible to final repository discovery and manifests.
use jj_store_object::*;
use std::sync::Arc;
pub(super) struct Prefix {
    pub store: Arc<dyn ObjectStore>,
    pub prefix: String,
}
impl Prefix {
    fn key(&self, key: &Key) -> Result<Key> {
        Key::new(format!("{}/{}", self.prefix, key.as_str()))
    }
}
#[async_trait::async_trait]
impl ObjectStore for Prefix {
    async fn head(&self, key: &Key) -> Result<Metadata> {
        self.store.head(&self.key(key)?).await
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> Result<Download> {
        self.store.get(&self.key(key)?, range).await
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> Result<WriteOutcome> {
        self.store.put(&self.key(key)?, condition, upload).await
    }
}
