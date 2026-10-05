//! Explicit preparation, independent of ordinary complete-read scan ceilings.
mod state;
use super::*;
use jj_store_jj::query::{self as q, Error};
pub use state::{Limits, Preparation, Progress, Stats};
use std::collections::BTreeSet;
use tokio::time::Instant;
use wal::publication::InventoryProjectionLimits;

const WINDOW_READS: usize = 1024;
const WINDOW_BYTES: u64 = 32 * 1024 * 1024;
const HEADER: usize = 1024 * 1024;
fn allocation(ids: usize) -> q::Result<usize> {
    ids.checked_mul(192)
        .and_then(|bytes| bytes.checked_add(HEADER))
        .ok_or(Error::IndexBudget)
}
fn map(error: wal::Error) -> Error {
    crate::reads::kind_error(error)
}
fn check(deadline: Instant) -> q::Result<()> {
    if Instant::now() >= deadline {
        Err(Error::Limit)
    } else {
        Ok(())
    }
}
impl NativeSession {
    /// Starts a caller-owned, data-only task at the current checked selection.
    /// Use CommittedRead's equivalent to retain an explicit historical pin.
    pub async fn begin_kind_preparation(
        &self,
        copies: bool,
        limits: Limits,
    ) -> q::Result<Preparation> {
        limits.validate()?;
        let deadline = Instant::now()
            .checked_add(self.limits.timeout)
            .ok_or(Error::Limit)?;
        let selected = tokio::time::timeout_at(deadline, self.snapshot())
            .await
            .map_err(|_| Error::Limit)?
            .map_err(map)?;
        check(deadline)?;
        self.begin_kind_preparation_at(&selected, copies, limits, deadline)
            .await
    }
    pub(crate) async fn begin_kind_preparation_at(
        &self,
        selected: &ManifestSnapshot,
        copies: bool,
        mut limits: Limits,
        deadline: Instant,
    ) -> q::Result<Preparation> {
        limits.validate()?;
        limits.max_ids = limits.max_ids.min(self.limits.inventory_nodes);
        if selected.manifest().repository != self.repository {
            return Err(Error::Denied);
        }
        tokio::time::timeout_at(deadline, self.inventory_check(selected, deadline))
            .await
            .map_err(|_| Error::Limit)?
            .map_err(map)?;
        check(deadline)?;
        let cache = cache::shared();
        let key = cache::Key::new(self.inventory_scope, selected, copies);
        let (ready, _) = cache.lookup(key, selected.manifest().revision);
        if ready.as_ref().is_some_and(|data| {
            data.ids.len() > limits.max_ids
                || allocation(data.ids.len()).map_or(true, |bytes| bytes > limits.max_bytes)
        }) {
            return Err(Error::Limit);
        }
        let lease = if ready.is_none() {
            Some(cache.reserve(HEADER).ok_or(Error::IndexBudget)?)
        } else {
            None
        };
        let expires = Instant::now()
            .checked_add(limits.timeout)
            .ok_or(Error::Limit)?;
        Ok(Preparation {
            selected: selected.clone(),
            scope: self.inventory_scope,
            copies,
            limits,
            expires,
            after: None,
            ids: BTreeSet::new(),
            lease,
            ready,
            stats: Stats::default(),
        })
    }
    async fn preparation_check(&self, task: &Preparation, deadline: Instant) -> q::Result<Instant> {
        if task.scope != self.inventory_scope
            || task.selected.manifest().repository != self.repository
        {
            return Err(Error::Denied);
        }
        let deadline = deadline.min(task.expires).min(
            Instant::now()
                .checked_add(self.limits.timeout)
                .ok_or(Error::Limit)?,
        );
        check(deadline)?;
        tokio::time::timeout_at(deadline, self.inventory_check(&task.selected, deadline))
            .await
            .map_err(|_| Error::Limit)?
            .map_err(map)?;
        check(deadline)?;
        Ok(deadline)
    }
    /// One bounded window. Dropping this future retains its admitted work but
    /// commits no cursor/ID progress; callers can resume under the original pin.
    pub async fn advance_kind_preparation(
        &self,
        task: &mut Preparation,
        deadline: Instant,
    ) -> q::Result<Progress> {
        let deadline = self.preparation_check(task, deadline).await?;
        if task.ready.is_some() {
            return Ok(task.progress());
        }
        // Another admitted reader/preparer may have proved this exact root
        // while the task was between windows. Reuse its complete allocation.
        let (ready, _) = cache::shared().lookup(
            cache::Key::new(task.scope, &task.selected, task.copies),
            task.selected.manifest().revision,
        );
        if let Some(data) = ready {
            if data.ids.len() > task.limits.max_ids
                || data.ids.len() > self.limits.inventory_nodes
                || allocation(data.ids.len())? > task.limits.max_bytes
            {
                return Err(Error::Limit);
            }
            task.ids.clear();
            task.lease = None;
            task.ready = Some(data);
            check(deadline)?;
            return Ok(task.progress());
        }
        let _permit = cache::shared()
            .permits
            .try_acquire()
            .map_err(|_| Error::Busy)?;
        let remaining = task
            .limits
            .max_descriptors
            .checked_sub(task.stats.descriptor_work)
            .filter(|n| *n != 0)
            .ok_or(Error::IndexBudget)?;
        let window = MAX_SCAN.min(remaining);
        let previous = task.stats;
        // Admit the whole window before I/O. On an unknown outcome keep these
        // conservative charges; retries cannot renew total work or lifetime.
        task.reserve(window, WINDOW_READS, WINDOW_BYTES)?;
        let page = tokio::time::timeout_at(
            deadline,
            task.selected.inventory_projection_page(
                self.object_store.clone(),
                task.after,
                InventoryProjectionLimits {
                    descriptors: window,
                    ..Default::default()
                },
            ),
        )
        .await
        .map_err(|_| Error::Limit)?
        .map_err(map)?;
        check(deadline)?;
        if page.scanned > window
            || page.record_reads > WINDOW_READS
            || page.record_bytes > WINDOW_BYTES
            || page
                .operations
                .len()
                .checked_add(page.copies.len())
                .is_none_or(|n| n > page.scanned)
            || page.next.is_some_and(|next| {
                task.after.is_some_and(|after| next <= after) || page.scanned == 0
            })
        {
            return Err(Error::Corrupt);
        }
        let keys: BTreeSet<_> = if task.copies {
            page.copies
                .iter()
                .map(|copy| m::ObjectKey::Copy(copy.id))
                .collect()
        } else {
            page.operations
                .iter()
                .map(|id| m::ObjectKey::Operation(*id))
                .collect()
        };
        let count = if task.copies {
            page.copies.len()
        } else {
            page.operations.len()
        };
        if keys.len() != count || keys.iter().any(|key| task.ids.contains(key)) {
            return Err(Error::Corrupt);
        }
        let count = task.ids.len().checked_add(keys.len()).ok_or(Error::Limit)?;
        if count > task.limits.max_ids
            || count > self.limits.inventory_nodes
            || allocation(count)? > task.limits.max_bytes
        {
            return Err(Error::Limit);
        }
        self.preparation_check(task, deadline).await?;
        // All async failure/cancellation points precede cursor/identity commit.
        // Window-local sets fit the fixed header allowance; retained growth is
        // separately admitted before inserting any selected identity.
        task.lease
            .as_mut()
            .ok_or(Error::Corrupt)?
            .resize(allocation(count)?)
            .ok_or(Error::IndexBudget)?;
        task.ids.extend(keys);
        task.after = page.next;
        task.stats = Stats {
            windows: previous.windows + 1,
            scanned: previous
                .scanned
                .checked_add(page.scanned)
                .ok_or(Error::IndexBudget)?,
            descriptor_work: previous.descriptor_work + page.scanned,
            record_reads: previous.record_reads + page.record_reads,
            record_bytes: previous.record_bytes + page.record_bytes,
        };
        if task.after.is_none() {
            task.ready = Some(cache::shared().publish_prepared(
                cache::Key::new(task.scope, &task.selected, task.copies),
                task.selected.clone(),
                std::mem::take(&mut task.ids),
                task.lease.take().ok_or(Error::Corrupt)?,
            ));
        }
        Ok(task.progress())
    }
    /// A complete result requires a fresh source/epoch/permission/deadline check.
    pub async fn finish_kind_preparation(
        &self,
        task: &Preparation,
        deadline: Instant,
    ) -> q::Result<BTreeSet<m::ObjectKey>> {
        let deadline = self.preparation_check(task, deadline).await?;
        let data = task.ready.as_ref().ok_or(Error::IndexRebuildRequired)?;
        if data.ids.len() > self.limits.inventory_nodes || data.ids.len() > task.limits.max_ids {
            return Err(Error::Limit);
        }
        let ids = data.ids.clone();
        self.preparation_check(task, deadline).await?;
        Ok(ids)
    }
}
#[cfg(test)]
mod tests;
