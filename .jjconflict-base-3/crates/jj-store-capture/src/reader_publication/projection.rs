use super::*;
use crate::reader_progress::ReaderProgressV1;
use std::collections::BTreeMap;

/// A physical transcript may continue in several numeric source generations.
/// Only the full retained history can establish its contiguous receipt frontier.
pub(super) fn physical_history(
    catalog: &Catalog,
    retained: &[ReaderProgressV1],
    integration: IntegrationId,
    source: SourceInstanceId,
    digest: [u8; 32],
) -> CaptureResult<Option<ReaderPublicationProgress>> {
    let mut channels: Vec<_> = catalog
        .channels
        .iter()
        .filter(|c| {
            c.key.integration == integration && c.key.source == source && c.key.digest == digest
        })
        .collect();
    channels.sort_by_key(|c| c.key.generation);
    let Some(latest) = channels.last() else {
        return Ok(None);
    };
    let mut records = BTreeMap::<u64, &ReaderProgressV1>::new();
    for progress in retained.iter().filter(|p| {
        p.cursor.integration == integration
            && p.cursor.source_instance_id == source
            && p.metadata.generation_digest == digest
    }) {
        let selected = records
            .entry(progress.cursor.generation)
            .or_insert(progress);
        if progress.last_sequence > selected.last_sequence {
            *selected = progress;
        }
    }
    let available = channels.iter().all(|c| c.history_available) && records.len() == channels.len();
    let mut committed = 0;
    let mut blocked = false;
    let mut outstanding = 0;
    let mut receipts: u64 = 0;
    let mut previous: Option<&Range> = None;
    for channel in &channels {
        if channel.latest.snapshot.is_some() {
            return Err("publication byte position unavailable".into());
        }
        let progress = records
            .get(&channel.key.generation)
            .ok_or("publication reader progress unavailable")?;
        if !channel.latest.matches_progress(progress) {
            return Err("publication reader progress differs".into());
        }
        let first = channel
            .frontier
            .as_ref()
            .or_else(|| channel.pending.first())
            .ok_or("publication channel range unavailable")?;
        if previous.is_some_and(|p| !first.range.follows(p)) {
            return Err("publication physical generation order differs".into());
        }
        previous = Some(&channel.latest);
        if !blocked {
            if let Some(frontier) = &channel.frontier {
                committed = frontier.range.offset;
            }
            blocked = !channel.pending.is_empty();
        }
        outstanding += channel.pending.len();
        receipts = receipts
            .checked_add(channel.receipts)
            .ok_or("publication receipt count exhausted")?;
    }
    let current = records
        .last_key_value()
        .map(|(_, p)| *p)
        .ok_or("publication reader progress unavailable")?;
    let jj_agent::SourcePositionV1::JsonLines {
        complete_byte_offset,
        ..
    } = current.cursor.position
    else {
        return Err("publication reader position differs".into());
    };
    Ok(Some(ReaderPublicationProgress {
        generation: latest.key.generation.max(current.cursor.generation),
        history_available: available,
        captured_byte_offset: complete_byte_offset,
        committed_byte_offset: if available { committed } else { 0 },
        outstanding_ranges: outstanding,
        receipts,
    }))
}
