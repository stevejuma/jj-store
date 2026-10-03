//! Opaque query-bound continuations retain audited immutable snapshots only.
use super::*;
use crate::auth::Clock;

use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, Mutex},
};

pub(super) struct Pin {
    pub expires: u64,
    pub snapshot: ContentSnapshot,
    pub viewer: jj_store_model::PrincipalId,
    pub results: Vec<ContentHit>,
}

impl Pin {
    fn bytes(&self) -> usize {
        self.results
            .iter()
            .map(|r| r.title.len() + r.excerpt.len() + r.revision.len() + 512)
            .sum()
    }
}
struct Entry<Q> {
    pin: Arc<Pin>,
    query: Q,
    offset: usize,
    expires: u64,
}
pub(super) struct Registry<Q = ContentQuery> {
    clock: Arc<dyn Clock>,
    entries: Mutex<BTreeMap<String, Entry<Q>>>,
}
pub(super) trait CursorQuery: Clone + PartialEq {
    fn cursor(&self) -> Option<&String>;
    fn clear_cursor(&mut self);
}
impl CursorQuery for ContentQuery {
    fn cursor(&self) -> Option<&String> {
        self.cursor.as_ref()
    }
    fn clear_cursor(&mut self) {
        self.cursor = None;
    }
}
impl<Q: CursorQuery> Registry<Q> {
    pub fn new(clock: Arc<dyn Clock>) -> Self {
        Self {
            clock,
            entries: Mutex::default(),
        }
    }
    fn now(&self) -> Result<u64> {
        self.clock.unix_seconds().map_err(|_| Error::Unavailable)
    }
    pub fn deadline(&self) -> Result<u64> {
        self.now()?.checked_add(300).ok_or(Error::Limit)
    }
    pub fn lookup(
        &self,
        repository: jj_store_model::RepositoryId,
        query: &Q,
    ) -> Result<(Arc<Pin>, usize)> {
        let token = query.cursor().ok_or(Error::InvalidCursor)?;
        if token.len() != 64
            || !token
                .bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
        {
            return Err(Error::InvalidCursor);
        }
        let now = self.now()?;
        let mut entries = self.entries.lock().map_err(|_| Error::Unavailable)?;
        entries.retain(|_, entry| now < entry.expires);
        let entry = entries.get(token).ok_or(Error::CursorReset)?;
        let mut requested = query.clone();
        requested.clear_cursor();
        if requested != entry.query || repository != entry.pin.snapshot.repo_id {
            return Err(Error::InvalidCursor);
        }
        Ok((entry.pin.clone(), entry.offset))
    }
    pub fn retain(&self, pin: Arc<Pin>, mut query: Q, offset: usize) -> Result<String> {
        let now = self.now()?;
        let expires = pin.expires;
        if now >= expires {
            return Err(Error::CursorReset);
        }
        let mut entries = self.entries.lock().map_err(|_| Error::Unavailable)?;
        entries.retain(|_, entry| now < entry.expires);
        let mut seen = BTreeSet::new();
        let mut bytes = 0usize;
        for retained in entries
            .values()
            .map(|e| &e.pin)
            .chain(std::iter::once(&pin))
        {
            if seen.insert(Arc::as_ptr(retained) as usize) {
                bytes = bytes.checked_add(retained.bytes()).ok_or(Error::Limit)?;
            }
        }
        if entries.len() >= 64 || bytes > 64 * 1024 * 1024 {
            return Err(Error::Busy);
        }
        let mut random = [0; 32];
        getrandom::fill(&mut random).map_err(|_| Error::Unavailable)?;
        let token = hex::encode(random);
        if entries.contains_key(&token) {
            return Err(Error::Unavailable);
        }
        query.clear_cursor();
        entries.insert(
            token.clone(),
            Entry {
                pin,
                query,
                offset,
                expires,
            },
        );
        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    struct Time(AtomicU64);
    impl Clock for Time {
        fn unix_seconds(&self) -> crate::auth::Result<u64> {
            Ok(self.0.load(Ordering::SeqCst))
        }
    }
    fn pin() -> Arc<Pin> {
        Arc::new(Pin {
            expires: 400,
            viewer: jj_store_model::PrincipalId::new([3; 16]),
            results: Vec::new(),
            snapshot: ContentSnapshot {
                repo_id: jj_store_model::RepositoryId::new([1; 16]),
                epoch: jj_store_model::EpochId::new([2; 16]),
                manifest_digest: jj_store_model::RecordDigest::new([4; 32]),
                published_seq: jj_store_model::WalPosition(1),
                thread_root: None,
                work_root: None,
            },
        })
    }
    #[test]
    fn capacity_restart_and_query_deadline_fail_closed_without_eviction() {
        let clock = Arc::new(Time(AtomicU64::new(100)));
        let registry = Registry::new(clock.clone());
        let pin = pin();
        let first = registry
            .retain(pin.clone(), ContentQuery::default(), 1)
            .unwrap();
        for _ in 1..64 {
            registry
                .retain(pin.clone(), ContentQuery::default(), 1)
                .unwrap();
        }
        assert_eq!(
            registry
                .retain(pin.clone(), ContentQuery::default(), 1)
                .unwrap_err(),
            Error::Busy
        );
        let query = ContentQuery {
            cursor: Some(first),
            ..Default::default()
        };
        assert_eq!(registry.lookup(pin.snapshot.repo_id, &query).unwrap().1, 1);
        assert!(matches!(
            Registry::new(clock.clone()).lookup(pin.snapshot.repo_id, &query),
            Err(Error::CursorReset)
        ));
        clock.0.store(400, Ordering::SeqCst);
        assert!(matches!(
            registry.lookup(pin.snapshot.repo_id, &query),
            Err(Error::CursorReset)
        ));
        assert_eq!(
            registry
                .retain(pin, ContentQuery::default(), 1)
                .unwrap_err(),
            Error::CursorReset
        );
    }
}
