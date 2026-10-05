//! Bounded scans of one immutable manifest selection, without request retention.
mod cache;
mod complete;
mod cursors;
use crate::{auth::Clock, native::NativeSession};
use jj_store_model as m;
use jj_store_wal::{
    self as wal,
    publication::{ManifestSnapshot, Snapshot},
};
use std::{sync::Arc, time::Duration};

pub const MAX_SCAN: usize = 256;
/// Independent ceiling for a complete enumeration, including unrelated kinds.
pub const MAX_SCANNED: usize = 2_000_000;
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("inventory access denied")]
    Denied,
    #[error("inventory resource limit exceeded")]
    Limit,
    #[error("inventory pin capacity exhausted")]
    Busy,
    #[error("invalid inventory cursor or changed query")]
    InvalidCursor,
    #[error("inventory cursor expired or serving instance changed; reselect")]
    CursorReset,
    #[error("inventory epoch changed; reselect")]
    EpochChanged,
    #[error("inventory unavailable")]
    Unavailable,
}
impl From<wal::Error> for Error {
    fn from(value: wal::Error) -> Self {
        match value {
            wal::Error::Denied => Self::Denied,
            wal::Error::Limit(_) | wal::Error::Storage(jj_store_object::Error::Limit) => {
                Self::Limit
            }
            _ => Self::Unavailable,
        }
    }
}
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Clone, Copy)]
pub struct Limits {
    pub snapshots: usize,
    pub concurrency: usize,
    pub ttl: Duration,
    pub timeout: Duration,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            snapshots: 64,
            concurrency: 8,
            ttl: Duration::from_secs(300),
            timeout: Duration::from_secs(30),
        }
    }
}
#[derive(Debug)]
pub struct Page {
    pub snapshot: Snapshot,
    pub ids: Vec<[u8; 64]>,
    pub scanned: usize,
    pub next_cursor: Option<String>,
}
pub struct Pager {
    cursors: cursors::Cursors,
    permits: tokio::sync::Semaphore,
    limits: Limits,
}
impl Pager {
    pub fn new(limits: Limits, clock: Arc<dyn Clock>) -> Result<Self> {
        if !(1..=64).contains(&limits.snapshots)
            || !(1..=64).contains(&limits.concurrency)
            || limits.ttl < Duration::from_secs(1)
            || limits.ttl > Duration::from_secs(300)
            || limits.timeout.is_zero()
            || limits.timeout > Duration::from_secs(300)
        {
            return Err(Error::Limit);
        }
        Ok(Self {
            cursors: cursors::Cursors::new(limits, clock)?,
            permits: tokio::sync::Semaphore::new(limits.concurrency),
            limits,
        })
    }
    pub async fn page(
        &self,
        session: Arc<NativeSession>,
        copies: bool,
        scan_limit: usize,
        cursor: Option<&str>,
    ) -> Result<Page> {
        if scan_limit == 0 || scan_limit > MAX_SCAN {
            return Err(Error::Limit);
        }
        let _permit = self.permits.try_acquire().map_err(|_| Error::Busy)?;
        let deadline = tokio::time::Instant::now()
            .checked_add(self.limits.timeout.min(session.limits.timeout))
            .ok_or(Error::Limit)?;
        let check_time = || {
            if tokio::time::Instant::now() >= deadline {
                Err(Error::Limit)
            } else {
                Ok(())
            }
        };
        check_time()?;
        let result = tokio::time::timeout_at(deadline, async {
            // Authenticate before cursor lookup or any selected catalog read.
            let current = session.snapshot().await?;
            check_time()?;
            let (entry, claims) = match cursor {
                Some(cursor) => {
                    let (entry, claims) =
                        self.cursors
                            .lookup(session.repository, copies, scan_limit, cursor)?;
                    (entry, Some(claims))
                }
                None => (
                    cursors::Entry::new(Arc::new(current.clone()), copies, scan_limit),
                    None,
                ),
            };
            Self::epoch(&current, &entry.base)?;
            let page = entry
                .base
                .inventory_page(
                    session.object_store.clone(),
                    claims.as_ref().map(|claims| claims.after),
                    scan_limit.min(session.limits.inventory_nodes),
                )
                .await?;
            check_time()?;
            let scanned = page.keys.len();
            let mut ids = page
                .keys
                .into_iter()
                .filter(|key| {
                    matches!(
                        (copies, key),
                        (true, m::ObjectKey::Copy(_)) | (false, m::ObjectKey::Operation(_))
                    )
                })
                .map(|key| *key.as_bytes())
                .collect::<Vec<_>>();
            ids.sort_unstable();
            if ids.windows(2).any(|pair| pair[0] >= pair[1]) {
                return Err(Error::Unavailable);
            }
            // Do not expose data after revocation or a changed authority epoch.
            Self::epoch(&session.snapshot().await?, &entry.base)?;
            check_time()?;
            let snapshot = entry.base.manifest().snapshot();
            let next_cursor = self.cursors.finish(entry, claims.as_ref(), page.next)?;
            Ok(Page {
                snapshot,
                ids,
                scanned,
                next_cursor,
            })
        })
        .await
        .map_err(|_| Error::Limit)?;
        // Ready I/O and synchronous decoding can finish without polling the timer.
        check_time()?;
        result
    }
    fn epoch(current: &ManifestSnapshot, selected: &ManifestSnapshot) -> Result<()> {
        if current.manifest().epoch != selected.manifest().epoch {
            Err(Error::EpochChanged)
        } else {
            Ok(())
        }
    }
}
