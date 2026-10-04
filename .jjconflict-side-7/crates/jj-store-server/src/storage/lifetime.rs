//! Resource retention only; no capture admission or publication permission.
use super::*;
use tokio::sync::Notify;

struct Resources {
    store: Arc<S3Store>,
    embedded: Option<EmbeddedStorageRuntime>,
    _lock: Option<File>,
    released: Arc<Notify>,
}

pub(super) struct StorageOwner {
    resources: Arc<Resources>,
}

/// Opaque memory-only retention of storage and its installation lock.
/// Only server-owned capture composition may fork it to its blocking owners.
pub struct CaptureStorageLifetime {
    resources: Option<Arc<Resources>>,
}
impl CaptureStorageLifetime {
    pub(crate) fn fork(&self) -> Self {
        Self {
            resources: self.resources.clone(),
        }
    }
}
impl Drop for CaptureStorageLifetime {
    fn drop(&mut self) {
        if let Some(resources) = self.resources.take() {
            let released = resources.released.clone();
            drop(resources);
            // Update the strong count before waking a shutdown waiter.
            released.notify_waiters();
        }
    }
}
impl StorageOwner {
    pub(super) fn new(
        store: Arc<S3Store>,
        embedded: Option<EmbeddedStorageRuntime>,
        lock: Option<File>,
    ) -> Self {
        Self {
            resources: Arc::new(Resources {
                store,
                embedded,
                _lock: lock,
                released: Arc::new(Notify::new()),
            }),
        }
    }
    pub(super) fn adopt_lock(&mut self, lock: Option<File>) -> Result<()> {
        let Some(lock) = lock else {
            return Ok(());
        };
        let resources = Arc::get_mut(&mut self.resources).ok_or(Error::Service(
            "state lock must be attached before capture lifetimes",
        ))?;
        if resources._lock.is_some() {
            return Err(Error::Config(
                "authoritative storage already owns a state lock",
            ));
        }
        resources._lock = Some(lock);
        Ok(())
    }
    pub(super) fn store(&self) -> Arc<S3Store> {
        self.resources.store.clone()
    }
    pub(super) fn capture_lifetime(&self) -> CaptureStorageLifetime {
        CaptureStorageLifetime {
            resources: Some(self.resources.clone()),
        }
    }
    pub(super) async fn shutdown(self) -> Result<()> {
        let mut resources = self.resources;
        loop {
            let released = resources.released.clone();
            let notified = released.notified();
            tokio::pin!(notified);
            // Register before inspecting ownership; a final release cannot be lost.
            notified.as_mut().enable();
            match Arc::try_unwrap(resources) {
                Ok(mut resources) => {
                    if let Some(runtime) = resources.embedded.take() {
                        runtime.shutdown().await;
                    }
                    // The installation lock stays held through embedded shutdown.
                    return Ok(());
                }
                Err(retained) => resources = retained,
            }
            notified.await;
        }
    }
}
#[cfg(all(test, unix))]
mod tests;
