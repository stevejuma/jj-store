//! Data-only activity reuse after fresh provider admission.
use super::*;
use sha2::{Digest as _, Sha256};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex, OnceLock},
    time::{Duration, Instant},
};
use tokio::sync::OnceCell;

const MAX_ENTRIES: usize = 128;
const TTL: Duration = Duration::from_secs(300);
// Keys, cells and values contain fixed-size fields only; no source object or
// body is held. Entry count therefore also bounds retained memory.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Authority {
    Native(m::EpochId),
    Local,
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Key {
    repository: m::RepositoryId,
    authority: Authority,
    root: Option<m::RecordDigest>,
    limit: usize,
    window: [u8; 32],
}
struct Entry {
    value: Arc<OnceCell<ReviewActivity>>,
    created: Instant,
    touched: Instant,
}
pub(crate) struct Cache {
    entries: Mutex<BTreeMap<Key, Entry>>,
    capacity: usize,
    ttl: Duration,
}
impl Default for Cache {
    fn default() -> Self {
        Self {
            entries: Mutex::default(),
            capacity: MAX_ENTRIES,
            ttl: TTL,
        }
    }
}
pub(crate) fn shared() -> &'static Cache {
    static CACHE: OnceLock<Cache> = OnceLock::new();
    CACHE.get_or_init(Cache::default)
}
impl Cache {
    fn slot(&self, key: Key, now: Instant) -> Result<Option<Arc<OnceCell<ReviewActivity>>>> {
        let mut entries = self.entries.lock().map_err(|_| Error::Unavailable)?;
        // An active owner keeps its slot. Canceled/failed idle initializers are
        // removed immediately; successful idle values obey fixed expiry.
        entries.retain(|_, entry| {
            Arc::strong_count(&entry.value) > 1
                || (entry.value.initialized()
                    && now.saturating_duration_since(entry.created) < self.ttl)
        });
        if let Some(entry) = entries.get_mut(&key) {
            if now.saturating_duration_since(entry.created) >= self.ttl {
                return Ok(None);
            }
            entry.touched = now;
            return Ok(Some(entry.value.clone()));
        }
        if entries.len() >= self.capacity {
            let idle = entries
                .iter()
                .filter(|(_, entry)| Arc::strong_count(&entry.value) == 1)
                .min_by_key(|(_, entry)| entry.touched)
                .map(|(key, _)| *key);
            let Some(idle) = idle else { return Ok(None) };
            entries.remove(&idle);
        }
        if self.capacity == 0 {
            return Ok(None);
        }
        let value = Arc::new(OnceCell::new());
        entries.insert(
            key,
            Entry {
                value: value.clone(),
                created: now,
                touched: now,
            },
        );
        Ok(Some(value))
    }
    pub async fn read(
        &self,
        pin: &ReviewSelection<'_>,
        authority: Authority,
        root: Option<m::RecordDigest>,
        limit: usize,
        selections: Vec<(ReviewCatalogKey, ReviewObjectRef)>,
        partial: bool,
    ) -> Result<ReviewActivity> {
        valid_limit(limit)?;
        if pin.store.repository() != pin.repository
            || selections.len() > limit
            || (root.is_none() && !selections.is_empty())
            || selections.windows(2).any(|rows| rows[0].0 >= rows[1].0)
        {
            return Err(Error::Corrupt);
        }
        let mut digest = Sha256::new();
        digest.update(b"jj-store/review-activity-window/v1\0");
        digest.update((selections.len() as u64).to_be_bytes());
        digest.update([u8::from(partial)]);
        for (key, reference) in &selections {
            digest.update(key.as_bytes());
            digest.update([reference.kind as u8]);
            digest.update(reference.id.as_bytes());
        }
        let key = Key {
            repository: pin.repository,
            authority,
            root,
            limit,
            window: digest.finalize().into(),
        };
        let Some(slot) = self.slot(key, Instant::now())? else {
            // Retention pressure cannot narrow valid repository capacity.
            return resolve(pin, selections, partial).await;
        };
        // The initializer is owned by this request, not stored in the cache.
        // Dropping it releases OnceCell admission so a waiter can retry.
        slot.get_or_try_init(|| resolve(pin, selections, partial))
            .await
            .cloned()
    }
}

#[cfg(test)]
#[path = "cache_tests.rs"]
mod tests;
