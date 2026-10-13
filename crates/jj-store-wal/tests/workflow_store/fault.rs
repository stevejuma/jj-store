use jj_store_object::{self as o, ObjectStore, memory::MemoryStore};
use std::sync::Mutex;
#[derive(Default)]
pub(super) struct FaultStore {
    inner: MemoryStore,
    armed: Mutex<Option<bool>>,
}
impl FaultStore {
    pub(super) fn arm(&self, after: bool) {
        *self.armed.lock().unwrap() = Some(after);
    }
}
#[async_trait::async_trait]
impl ObjectStore for FaultStore {
    async fn head(&self, key: &o::Key) -> o::Result<o::Metadata> {
        self.inner.head(key).await
    }
    async fn get(&self, key: &o::Key, range: Option<o::ByteRange>) -> o::Result<o::Download> {
        self.inner.get(key, range).await
    }
    async fn put(
        &self,
        key: &o::Key,
        condition: o::Condition,
        upload: &o::StagedUpload,
    ) -> o::Result<o::WriteOutcome> {
        let fault = if key.as_str().ends_with("/manifest.json") {
            self.armed.lock().unwrap().take()
        } else {
            None
        };
        if fault == Some(false) {
            return Ok(o::WriteOutcome::Unknown);
        }
        let result = self.inner.put(key, condition, upload).await?;
        if fault == Some(true) {
            std::future::pending().await
        } else {
            Ok(result)
        }
    }
}
