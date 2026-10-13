//! Poll pinned definitions and selected source publications before durable evaluation.
use super::*;
use std::collections::BTreeMap;
impl Host {
    pub(super) async fn observe_sources(&mut self) -> Result<()> {
        let mut definitions = BTreeMap::new();
        let mut versions = Vec::new();
        let mut at = None;
        let mut after = None;
        loop {
            let page = self
                .client
                .workflow_page(&w::Query {
                    at,
                    after,
                    limit: Some(256),
                    ..Default::default()
                })
                .await?;
            at = Some(page.position);
            for version in page.workflows {
                definitions.insert(version.id, version.version);
                if version.definition.enabled && !version.definition.conditions.is_empty() {
                    versions.push(version);
                }
            }
            after = page.next;
            if after.is_none() {
                break;
            }
        }
        for source in self.config.sources.clone() {
            let mut filters = BTreeMap::new();
            for version in &versions {
                if version.definition.repository == source.repository {
                    filters.insert(
                        version.definition.conditions.id()?,
                        version.definition.conditions.clone(),
                    );
                }
            }
            let mut batches = vec![Vec::new()];
            for filter in filters.into_values() {
                let batch = batches.last().ok_or("Workflow filter batch absent")?;
                let size = serde_json::to_vec(&filter)?.len();
                if batch.len() == 32 || serde_json::to_vec(batch)?.len() + size + 1 > 24 * 1024 {
                    batches.push(Vec::new());
                }
                batches
                    .last_mut()
                    .ok_or("Workflow filter batch absent")?
                    .push(filter);
            }
            let client = config::connection(&source.workspace)?.connect().await?;
            let cursor = self.journal.cursors.get(&source.repository).cloned();
            let mut unavailable = cursor
                .as_ref()
                .map(|c| c.unavailable.clone())
                .unwrap_or_default();
            let mut events: BTreeMap<w::SourceEventId, w::SourceEvent> = BTreeMap::new();
            let mut until = None;
            let mut selected = None;
            for batch in &batches {
                let page = client
                    .workflow_source_events(&w::EventQuery {
                        epoch: cursor.as_ref().map(|c| c.epoch),
                        after: cursor.as_ref().map(|c| c.after),
                        limit: Some(16),
                        until,
                        filters: Some(serde_json::to_string(batch)?),
                    })
                    .await?;
                unavailable.extend(page.unavailable.iter().cloned());
                until = Some(page.upper);
                if let Some((epoch, after)) = selected
                    && (epoch != page.epoch || after != page.after)
                {
                    return Err("Workflow source page changed between filter batches".into());
                }
                selected = Some((page.epoch, page.after));
                for event in &page.events {
                    if let Some(existing) = events.get_mut(&event.id) {
                        existing.filter_results.extend(event.filter_results.clone());
                    } else {
                        events.insert(event.id, event.clone());
                    }
                }
            }
            // Startup begins at current publication; historical backfill is deliberate.
            for event in events.into_values() {
                self.submit(w::Command::Evaluate {
                    event,
                    definitions: Some(definitions.clone()),
                })
                .await?;
            }
            let (epoch, after) = selected.ok_or("Workflow source cursor absent")?;
            if unavailable.len() > 64 {
                unavailable.drain(..unavailable.len() - 64);
            }
            if !unavailable.is_empty() {
                eprintln!("Workflow source diagnostics retained in the host journal");
            }
            self.journal.cursors.insert(
                source.repository,
                journal::Cursor {
                    epoch,
                    after,
                    unavailable,
                },
            );
            self.save()?;
        }
        Ok(())
    }
}
