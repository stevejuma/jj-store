//! One server-owned source cache and disposable projection store for native reads.
use crate::{auth::SystemClock, collab_pages::Limits, collab_reads::materialization::Cache};
use jj_store_read_index::{Index, Key, Row, Stats, Store};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Semaphore;

/// Weak source registrations prevent pointer reuse from reusing a prior source's data.
#[derive(Default)]
pub(crate) struct Sources(std::sync::Mutex<Vec<SourceRegistration>>);
type SourceRegistration = (std::sync::Weak<dyn jj_store_object::ObjectStore>, u64);
impl Sources {
    pub fn identify(
        &self,
        source: &Arc<dyn jj_store_object::ObjectStore>,
    ) -> std::result::Result<u64, ()> {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let weak = Arc::downgrade(source);
        let mut sources = self.0.lock().map_err(|_| ())?;
        sources.retain(|(prior, _)| prior.strong_count() != 0);
        if let Some((_, id)) = sources.iter().find(|(prior, _)| prior.ptr_eq(&weak)) {
            return Ok(*id);
        }
        let id = NEXT
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| v.checked_add(1))
            .map_err(|_| ())?;
        sources.push((weak, id));
        Ok(id)
    }
}

pub struct Indexes {
    pub(crate) shared: bool,
    pub(crate) cache: Cache,
    pub(crate) content: crate::content_search::materialization::Cache,
    store: Option<Arc<Store>>,
    namespace: String,
    builder: Arc<Semaphore>,
    failed: std::sync::atomic::AtomicU64,
}
impl Default for Indexes {
    fn default() -> Self {
        Self::memory(false)
    }
}
impl Indexes {
    fn memory(shared: bool) -> Self {
        Self {
            shared,
            cache: Cache::new(Limits::default(), Arc::new(SystemClock)),
            content: crate::content_search::materialization::Cache::new(Arc::new(SystemClock)),
            store: None,
            namespace: String::new(),
            builder: Arc::new(Semaphore::new(1)),
            failed: std::sync::atomic::AtomicU64::new(0),
        }
    }
    /// The owner is bound to one host storage source. Clones share no credentials.
    pub fn persistent(path: PathBuf) -> Self {
        use sha2::{Digest, Sha256};
        let namespace = hex::encode(Sha256::digest(path.as_os_str().as_encoded_bytes()));
        let store = Store::new(path).ok().map(Arc::new);
        let failures = u64::from(store.is_none());
        Self {
            store,
            namespace,
            failed: std::sync::atomic::AtomicU64::new(failures),
            ..Self::memory(true)
        }
    }
    pub fn stats(&self) -> Stats {
        let mut stats = self.store.as_ref().map_or(Stats::default(), |s| s.stats());
        stats.failed += self.failed.load(std::sync::atomic::Ordering::Relaxed);
        stats
    }
    pub(crate) async fn prepare(
        &self,
        repository: String,
        epoch: String,
        root: String,
        projection: &str,
        rows: Vec<Row>,
    ) -> jj_store_read_index::Result<Index> {
        let key = Key {
            namespace: self.namespace.clone(),
            repository,
            epoch,
            root,
            projection: projection.into(),
        };
        let store = self.store.clone();
        let permit = self.builder.clone().try_acquire_owned().map_err(|_| {
            self.failed
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            jj_store_read_index::Error::Unavailable
        })?;
        let result = tokio::task::spawn_blocking(move || {
            let _permit = permit;
            match store {
                Some(store) => store.prepare(&key, rows),
                None => Index::build(rows),
            }
        })
        .await
        .map_err(|_| jj_store_read_index::Error::Unavailable)?;
        if result.is_err() {
            self.failed
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn preparation_pressure_returns_to_authorized_fallback_without_waiting() {
        let indexes = Indexes::default();
        let _busy = indexes.builder.clone().acquire_owned().await.unwrap();
        let result = tokio::time::timeout(
            std::time::Duration::from_millis(100),
            indexes.prepare("repo".into(), "epoch".into(), "root".into(), "test", vec![]),
        )
        .await
        .unwrap();
        assert!(result.is_err());
        assert_eq!(indexes.stats().failed, 1);
    }
}
