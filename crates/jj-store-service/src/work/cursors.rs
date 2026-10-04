//! Opaque query-bound continuations retain audited immutable snapshots only.
use super::*;
use crate::auth::Clock;
use crate::collab_reads::WorkData;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, Mutex},
};

pub(super) struct Pin {
    pub snapshot: WorkReadSnapshot,
    pub data: Arc<WorkData>,
    pub selection: Option<super::projection::Selection>,
}
struct Entry<Q> {
    pin: Arc<Pin>,
    query: Q,
    offset: usize,
    expires: u64,
}
pub(super) struct Registry<Q = WorkQuery> {
    clock: Arc<dyn Clock>,
    entries: Mutex<BTreeMap<String, Entry<Q>>>,
}
pub(super) trait CursorQuery: Clone + PartialEq {
    fn cursor(&self) -> Option<&String>;
    fn clear_cursor(&mut self);
}
impl CursorQuery for WorkQuery {
    fn cursor(&self) -> Option<&String> {
        self.cursor.as_ref()
    }
    fn clear_cursor(&mut self) {
        self.cursor = None;
    }
}
#[derive(Clone, PartialEq)]
pub(super) enum WorkCursorQuery {
    Page(WorkQuery),
    Browse(WorkBrowseQuery),
}
impl CursorQuery for WorkCursorQuery {
    fn cursor(&self) -> Option<&String> {
        match self {
            Self::Page(q) => q.cursor(),
            Self::Browse(q) => q.cursor(),
        }
    }
    fn clear_cursor(&mut self) {
        match self {
            Self::Page(q) => q.clear_cursor(),
            Self::Browse(q) => q.clear_cursor(),
        }
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
        let expires = now.checked_add(300).ok_or(Error::Limit)?;
        let mut entries = self.entries.lock().map_err(|_| Error::Unavailable)?;
        entries.retain(|_, entry| now < entry.expires);
        let mut seen_pins = BTreeSet::new();
        let mut seen_data = BTreeSet::new();
        let mut bytes = 0usize;
        for retained in entries
            .values()
            .map(|e| &e.pin)
            .chain(std::iter::once(&pin))
        {
            if seen_data.insert(Arc::as_ptr(&retained.data) as usize) {
                bytes = bytes.checked_add(retained.data.bytes).ok_or(Error::Limit)?;
            }
            if seen_pins.insert(Arc::as_ptr(retained) as usize) {
                bytes = bytes
                    .checked_add(retained.selection.as_ref().map_or(0, |keys| keys.bytes()))
                    .ok_or(Error::Limit)?;
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
