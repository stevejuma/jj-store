//! Complete kind enumeration shares bounded windows but no process-local pin.
use super::*;
use std::collections::BTreeSet;

impl NativeSession {
    pub(crate) async fn inventory_ids(
        &self,
        copies: bool,
        limit: usize,
    ) -> wal::Result<BTreeSet<m::ObjectKey>> {
        let deadline = tokio::time::Instant::now()
            .checked_add(self.limits.timeout)
            .ok_or(wal::Error::Limit("native inventory time"))?;
        let check_time = || {
            if tokio::time::Instant::now() >= deadline {
                Err(wal::Error::Limit("native inventory time"))
            } else {
                Ok(())
            }
        };
        check_time()?;
        let result = tokio::time::timeout_at(deadline, async {
            let selected = self.snapshot().await?;
            let mut ids = BTreeSet::new();
            let mut after = None;
            let mut scanned = 0usize;
            let result_limit = limit.min(self.limits.inventory_nodes);
            loop {
                check_time()?;
                let current = self.snapshot().await?;
                if current.manifest().epoch != selected.manifest().epoch {
                    return Err(wal::Error::Stale);
                }
                let remaining = MAX_SCANNED
                    .checked_sub(scanned)
                    .filter(|count| *count != 0)
                    .ok_or(wal::Error::Limit("native inventory scan"))?;
                let page = selected
                    .inventory_page(
                        self.object_store.clone(),
                        after,
                        MAX_SCAN.min(self.limits.inventory_nodes).min(remaining),
                    )
                    .await?;
                check_time()?;
                scanned = scanned
                    .checked_add(page.keys.len())
                    .ok_or(wal::Error::Limit("native inventory scan"))?;
                for key in page.keys {
                    if !matches!(
                        (copies, key),
                        (true, m::ObjectKey::Copy(_)) | (false, m::ObjectKey::Operation(_))
                    ) {
                        continue;
                    }
                    if ids.contains(&key) {
                        return Err(wal::Error::Corrupt("duplicate native inventory ID"));
                    }
                    if ids.len() == result_limit {
                        return Err(wal::Error::Limit("native inventory result"));
                    }
                    ids.insert(key);
                }
                let current = self.snapshot().await?;
                if current.manifest().epoch != selected.manifest().epoch {
                    return Err(wal::Error::Stale);
                }
                check_time()?;
                if let Some(next) = page.next {
                    if after.is_some_and(|after| next <= after) {
                        return Err(wal::Error::Corrupt("native inventory continuation"));
                    }
                    after = Some(next);
                } else {
                    return Ok(ids);
                }
            }
        })
        .await
        .map_err(|_| wal::Error::Limit("native inventory time"))?;
        check_time()?;
        result
    }
}
