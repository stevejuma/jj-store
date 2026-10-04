use super::*;
use crate::capture_supervisor::CodexSourceCatalog;

impl Manager {
    pub(super) async fn read_sources(
        &self,
        actor: PrincipalId,
        query: CaptureSourceQuery,
    ) -> Result<CaptureSourcePage, Error> {
        validate(&query)?;
        let this = self.clone();
        let permit = self.observation_permit().await?;
        tokio::task::spawn_blocking(move || {
            let _permit = permit;
            if !this.0.authority.admit(actor)? {
                return Ok(disabled());
            }
            let binding = this.binding()?;
            binding
                .journal
                .read_then(|selected, check| {
                    if selected.generation != binding.generation {
                        return Err(Error::Unavailable);
                    }
                    #[cfg(test)]
                    let hook = this.0.after_selection.lock().unwrap().take();
                    #[cfg(test)]
                    if let Some(hook) = hook {
                        hook();
                    }
                    if !this.0.authority.admit(actor)? {
                        return Err(Error::Disabled);
                    }
                    check().map_err(report::failure)?;
                    let snapshot = binding
                        .runtime
                        .get()
                        .ok_or(Error::Unavailable)?
                        .source_snapshot()
                        .map_err(|_| Error::Unavailable)?;
                    let page = page(&snapshot, query, binding.imports.as_deref())?;
                    if !this.0.authority.admit(actor)? {
                        return Err(Error::Disabled);
                    }
                    check().map_err(report::failure)?;
                    Ok(page)
                })
                .map_err(report::failure)?
        })
        .await
        .map_err(|_| Error::Unavailable)?
    }
}
fn disabled() -> CaptureSourcePage {
    CaptureSourcePage {
        version: 1,
        enabled: false,
        runtime_generation: None,
        phase: None,
        total: 0,
        sources: vec![],
        next_cursor: None,
    }
}
fn validate(query: &CaptureSourceQuery) -> Result<(), Error> {
    if !(1..=100).contains(&query.limit.unwrap_or(50)) {
        return Err(Error::Invalid);
    }
    if let Some(cursor) = &query.cursor
        && (cursor.len() != 49
            || cursor.as_bytes()[32] != b':'
            || cursor
                .bytes()
                .enumerate()
                .any(|(i, b)| i != 32 && !b.is_ascii_digit() && !(b'a'..=b'f').contains(&b))
            || u64::from_str_radix(&cursor[33..], 16).map_err(|_| Error::Invalid)? == 0)
    {
        return Err(Error::Invalid);
    }
    Ok(())
}
fn page(
    snapshot: &CodexSourceCatalog,
    query: CaptureSourceQuery,
    resources: Option<&CaptureImportManagement>,
) -> Result<CaptureSourcePage, Error> {
    let generation = hex::encode(snapshot.generation);
    let after = if let Some(cursor) = &query.cursor {
        if cursor[..32] != generation {
            return Err(Error::SourceStale);
        }
        let after = u64::from_str_radix(&cursor[33..], 16).map_err(|_| Error::Invalid)?;
        if after > snapshot.high_water {
            return Err(Error::SourceStale);
        }
        after
    } else {
        0
    };
    let mut available = snapshot.sources.iter().filter(|row| row.selector > after);
    let selected: Vec<_> = available.by_ref().take(query.limit.unwrap_or(50)).collect();
    let next_cursor = if available.next().is_some() {
        selected
            .last()
            .map(|row| format!("{generation}:{:016x}", row.selector))
    } else {
        None
    };
    let publication = snapshot_coverage(&selected, resources);
    Ok(CaptureSourcePage {
        version: 1,
        enabled: true,
        runtime_generation: Some(generation.clone()),
        phase: Some(report::phase(snapshot.phase)),
        total: snapshot.sources.len(),
        next_cursor,
        sources: selected
            .into_iter()
            .zip(publication)
            .map(|(row, snapshot_publication)| CaptureSourceCoverage {
                snapshot_publication,
                selector: format!("{generation}:{:016x}", row.selector),
                profile: report::profile(row.profile),
                product_version: row.profile.product_version().into(),
                phase: report::phase(row.phase),
                observed: row.observed,
                awaiting_activity: row.awaiting_activity,
                caught_up: row.caught_up,
                running: row.running,
                degraded: row.degraded,
                last_bound_repository: row.last_bound_repository,
                ingress_required_capacity_bytes: row.ingress_required_capacity_bytes,
            })
            .collect(),
    })
}

fn snapshot_coverage(
    rows: &[&crate::capture_supervisor::CodexSourceObservation],
    resources: Option<&CaptureImportManagement>,
) -> Vec<Option<CaptureSnapshotPublication>> {
    let queries: Vec<_> = rows.iter().filter_map(|r| r.snapshot_publication).collect();
    let mut coverage = resources
        .filter(|r| !queries.is_empty() && r.check().is_ok())
        .and_then(|r| {
            r.spool
                .inspect_snapshot_publication_progress_batch(&queries)
                .ok()
        })
        .unwrap_or_else(|| vec![None; queries.len()])
        .into_iter();
    rows.iter()
        .map(|row| {
            row.snapshot_publication
                .and_then(|_| coverage.next().flatten())
                .map(|p| CaptureSnapshotPublication {
                    history_available: p.history_available,
                    captured_sequence: p.captured_sequence,
                    captured_logical_records: p.captured_logical_records,
                    committed_sequence: p.committed_sequence,
                    committed_logical_records: p.committed_logical_records,
                    outstanding_ranges: p.outstanding_ranges,
                    receipts: p.receipts,
                })
        })
        .collect()
}
