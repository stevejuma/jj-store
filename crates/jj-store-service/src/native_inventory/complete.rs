//! Complete kind lookup; immutable reuse never retains a request capability.
use super::*;
use std::collections::BTreeSet;

fn check_time(deadline: tokio::time::Instant) -> wal::Result<()> {
    if tokio::time::Instant::now() >= deadline {
        Err(wal::Error::Limit("native inventory time"))
    } else {
        Ok(())
    }
}
impl NativeSession {
    async fn inventory_check(
        &self,
        selected: &ManifestSnapshot,
        deadline: tokio::time::Instant,
    ) -> wal::Result<()> {
        check_time(deadline)?;
        let current = self.snapshot().await?;
        check_time(deadline)?;
        if current.manifest().epoch != selected.manifest().epoch {
            return Err(wal::Error::Stale);
        }
        Ok(())
    }
    pub(crate) async fn inventory_ids(
        &self,
        copies: bool,
        limit: usize,
    ) -> wal::Result<BTreeSet<m::ObjectKey>> {
        let deadline = tokio::time::Instant::now()
            .checked_add(self.limits.timeout)
            .ok_or(wal::Error::Limit("native inventory time"))?;
        check_time(deadline)?;
        let result = tokio::time::timeout_at(deadline, async {
            let cache = cache::shared();
            let _permit = cache
                .permits
                .acquire()
                .await
                .map_err(|_| wal::Error::Limit("native inventory concurrency"))?;
            let selected = self.snapshot().await?;
            check_time(deadline)?;
            let key = cache::Key::new(self.inventory_scope, &selected, copies);
            let result_limit = limit.min(self.limits.inventory_nodes);
            let (exact, mut base) = cache.lookup(key, selected.manifest().revision);
            if let Some(exact) = exact {
                if exact.ids.len() > result_limit {
                    return Err(wal::Error::Limit("native inventory result"));
                }
                let ids = exact.ids.clone();
                self.inventory_check(&selected, deadline).await?;
                return Ok(ids);
            }
            if base
                .as_ref()
                .is_some_and(|base| base.ids.len() > result_limit)
            {
                // Retention must first be proved: cold scan may discard a
                // non-retained baseline, so infer no result limit from it.
                base = None;
            }
            let mut build = cache.build(base.as_deref());
            if !build.admitted_base() {
                base = None;
            }
            let mut after = None;
            let mut scanned = 0usize;
            loop {
                self.inventory_check(&selected, deadline).await?;
                let remaining = MAX_SCANNED
                    .checked_sub(scanned)
                    .filter(|count| *count != 0)
                    .ok_or(wal::Error::Limit("native inventory scan"))?;
                let window = MAX_SCAN.min(self.limits.inventory_nodes).min(remaining);
                let page = if let Some(base) = &base {
                    selected
                        .inventory_additions_page(
                            &base.selected,
                            self.object_store.clone(),
                            after,
                            window,
                        )
                        .await
                } else {
                    selected
                        .inventory_page(self.object_store.clone(), after, window)
                        .await
                };
                let page = match page {
                    Err(wal::Error::Corrupt("catalog base not retained")) if base.is_some() => {
                        let prior = base.take().unwrap();
                        cache.discard(cache::Key::new(
                            self.inventory_scope,
                            &prior.selected,
                            copies,
                        ));
                        drop(prior);
                        build = cache.build(None);
                        after = None;
                        // Keep consumed scan allowance and original deadline.
                        continue;
                    }
                    result => result?,
                };
                check_time(deadline)?;
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
                    if build.ids.contains(&key) {
                        return Err(wal::Error::Corrupt("duplicate native inventory ID"));
                    }
                    if build.ids.len() == result_limit {
                        return Err(wal::Error::Limit("native inventory result"));
                    }
                    build.insert(key);
                }
                self.inventory_check(&selected, deadline).await?;
                if let Some(next) = page.next {
                    if after.is_some_and(|after| next <= after) {
                        return Err(wal::Error::Corrupt("native inventory continuation"));
                    }
                    after = Some(next);
                } else {
                    let ids = cache.finish(key, selected.clone(), build);
                    self.inventory_check(&selected, deadline).await?;
                    return Ok(ids);
                }
            }
        })
        .await
        .map_err(|_| wal::Error::Limit("native inventory time"))?;
        check_time(deadline)?;
        result
    }
}
