//! Selected copy relations, shared independently of each request's authority.
mod cache;
use super::*;
use jj_store_jj::query::{self, copy_index as index};
use std::collections::BTreeSet;
use tokio::time::Instant;
use wal::publication::InventoryProjectionLimits;

fn check(deadline: Instant) -> query::Result<()> {
    if Instant::now() >= deadline {
        Err(query::Error::Limit)
    } else {
        Ok(())
    }
}
fn wal_error(error: wal::Error) -> query::Error {
    crate::reads::kind_error(error)
}
impl NativeSession {
    pub(crate) async fn copy_family_at(
        &self,
        selected: &ManifestSnapshot,
        seeds: &BTreeSet<m::CopyId>,
        required: &BTreeSet<m::CopyId>,
        mut limits: index::LookupLimits,
        deadline: Instant,
    ) -> query::Result<index::Family> {
        if selected.manifest().repository != self.repository {
            return Err(query::Error::Denied);
        }
        limits.max_nodes = limits.max_nodes.min(self.limits.inventory_nodes);
        if limits.max_nodes == 0
            || limits.max_work == 0
            || seeds.len() > limits.max_nodes
            || seeds
                .len()
                .checked_mul(512)
                .is_none_or(|bytes| bytes > limits.max_bytes)
            || !required.is_subset(seeds)
        {
            return Err(query::Error::Limit);
        }
        let deadline = deadline.min(
            Instant::now()
                .checked_add(self.limits.timeout)
                .ok_or(query::Error::Limit)?,
        );
        check(deadline)?;
        let result = tokio::time::timeout_at(deadline, async {
            self.inventory_check(selected, deadline)
                .await
                .map_err(wal_error)?;
            let _permit = super::cache::shared()
                .permits
                .acquire()
                .await
                .map_err(|_| query::Error::IndexBudget)?;
            check(deadline)?;
            if seeds.is_empty() {
                self.inventory_check(selected, deadline)
                    .await
                    .map_err(wal_error)?;
                return Ok(index::Family {
                    ids: BTreeSet::new(),
                    work: 0,
                });
            }
            let cache = cache::shared();
            let key = cache::Key::new(self.inventory_scope, selected);
            let (ready, gate) = cache.entry(key)?;
            let data = if let Some(data) = ready {
                Some(data)
            } else {
                let _guard = gate.lock().await;
                self.inventory_check(selected, deadline)
                    .await
                    .map_err(wal_error)?;
                if let Some(data) = cache.ready(key) {
                    Some(data)
                } else {
                    // If every optional staged seed is absent, no repository
                    // scan is necessary. Required seeds remain strict.
                    let mut found = false;
                    for (position, id) in seeds.iter().enumerate() {
                        if position % 128 == 0 {
                            tokio::task::yield_now().await;
                        }
                        check(deadline)?;
                        let present = selected
                            .inventory_member(self.object_store.clone(), m::ObjectKey::Copy(*id))
                            .await
                            .map_err(wal_error)?
                            .is_some();
                        if !present && required.contains(id) {
                            return Err(query::Error::NotCommitted);
                        }
                        found |= present;
                    }
                    if !found {
                        None
                    } else {
                        let graph = self
                            .build_copy_graph(cache, selected, key, deadline)
                            .await?;
                        self.inventory_check(selected, deadline)
                            .await
                            .map_err(wal_error)?;
                        let lease = cache.reserve(cache::HEADER)?;
                        let data = Arc::new(cache::Data {
                            selected: selected.clone(),
                            graph,
                            _lease: lease,
                        });
                        cache.finish(key, &gate, data.clone());
                        Some(data)
                    }
                }
            };
            let result = if let Some(data) = data {
                let mut known = BTreeSet::new();
                for (position, id) in seeds.iter().enumerate() {
                    if position % 128 == 0 {
                        tokio::task::yield_now().await;
                    }
                    check(deadline)?;
                    if data.graph.index.parents(*id).is_some() {
                        known.insert(*id);
                    } else if required.contains(id) {
                        return Err(query::Error::NotCommitted);
                    }
                }
                let classification = seeds.len();
                let work = limits
                    .max_work
                    .checked_sub(classification)
                    .ok_or(query::Error::Limit)?;
                let mut family = data
                    .graph
                    .index
                    .family_with_work(
                        &known,
                        index::LookupLimits {
                            max_work: work,
                            ..limits
                        },
                        deadline,
                        &Default::default(),
                    )
                    .await?;
                family.work = family
                    .work
                    .checked_add(classification)
                    .ok_or(query::Error::Limit)?;
                family
            } else {
                index::Family {
                    ids: BTreeSet::new(),
                    work: seeds.len(),
                }
            };
            if result.work > limits.max_work {
                return Err(query::Error::Limit);
            }
            self.inventory_check(selected, deadline)
                .await
                .map_err(wal_error)?;
            Ok(result)
        })
        .await
        .map_err(|_| query::Error::Limit)?;
        check(deadline)?;
        result
    }
    async fn build_copy_graph(
        &self,
        cache: &cache::Cache,
        selected: &ManifestSnapshot,
        key: cache::Key,
        deadline: Instant,
    ) -> query::Result<Arc<cache::Graph>> {
        let mut base = cache.base(key, selected.manifest().revision);
        let mut build: Option<(index::Builder, super::cache::Lease)> = None;
        let mut after = None;
        let mut scanned = 0usize;
        loop {
            self.inventory_check(selected, deadline)
                .await
                .map_err(wal_error)?;
            let remaining = MAX_SCANNED
                .checked_sub(scanned)
                .filter(|count| *count != 0)
                .ok_or(query::Error::IndexBudget)?;
            let limits = InventoryProjectionLimits {
                descriptors: MAX_SCAN.min(remaining),
                ..InventoryProjectionLimits::default()
            };
            let page = if let Some(base) = &base {
                selected
                    .inventory_projection_additions_page(
                        &base.selected,
                        self.object_store.clone(),
                        after,
                        limits,
                    )
                    .await
            } else {
                selected
                    .inventory_projection_page(self.object_store.clone(), after, limits)
                    .await
            };
            let page = match page {
                Err(wal::Error::Corrupt("catalog base not retained")) if base.is_some() => {
                    base = None;
                    build = None;
                    after = None;
                    continue; // Keep spent scan work and the original deadline.
                }
                result => result.map_err(wal_error)?,
            };
            check(deadline)?;
            scanned = scanned
                .checked_add(page.scanned)
                .ok_or(query::Error::IndexBudget)?;
            for copy in page.copies {
                check(deadline)?;
                if base
                    .as_ref()
                    .is_some_and(|base| base.graph.index.parents(copy.id).is_some())
                {
                    return Err(query::Error::Corrupt);
                }
                if build.is_none() {
                    let mut lease = cache.reserve(cache::HEADER)?;
                    let builder = if let Some(base) = &base {
                        index::Builder::from_index(
                            &base.graph.index,
                            index::Limits::default(),
                            |bytes| cache.grow(&mut lease, bytes),
                        )?
                    } else {
                        index::Builder::new(index::Limits::default())?
                    };
                    build = Some((builder, lease));
                }
                let (builder, lease) = build.as_mut().unwrap();
                builder.insert_admitted(copy.id, copy.parents, |bytes| cache.grow(lease, bytes))?;
            }
            self.inventory_check(selected, deadline)
                .await
                .map_err(wal_error)?;
            if let Some(next) = page.next {
                if after.is_some_and(|after| next <= after) {
                    return Err(query::Error::Corrupt);
                }
                after = Some(next);
            } else {
                if let Some((builder, lease)) = build {
                    let index = builder.finish(deadline, &Default::default()).await?;
                    check(deadline)?;
                    return Ok(Arc::new(cache::Graph {
                        index,
                        _lease: lease,
                    }));
                }
                if let Some(base) = base {
                    return Ok(base.graph.clone());
                }
                // A checked selection may contain no copies at all.
                let lease = cache.reserve(cache::HEADER)?;
                let index = index::Builder::new(index::Limits::default())?
                    .finish(deadline, &Default::default())
                    .await?;
                return Ok(Arc::new(cache::Graph {
                    index,
                    _lease: lease,
                }));
            }
        }
    }
}
